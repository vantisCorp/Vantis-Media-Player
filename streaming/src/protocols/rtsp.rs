//! RTSP (Real-Time Streaming Protocol) Handler
//! 
//! This module provides comprehensive RTSP support including:
//! - RTSP protocol implementation (RFC 2326/RFC 7826)
//! - RTP/RTCP packet handling
//! - SDP parsing and generation
//! - Authentication support (Basic, Digest)
//! - Session management
//! - Interleaved TCP and UDP transport
//! - Recording and playback control

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// ============================================================================
// RTSP Constants
// ============================================================================

/// RTSP default port
pub const RTSP_DEFAULT_PORT: u16 = 554;

/// RTSP version
pub const RTSP_VERSION: &str = "RTSP/1.0";

/// Maximum RTSP message size
pub const MAX_RTSP_MESSAGE_SIZE: usize = 65536;

/// RTP default port base
pub const RTP_DEFAULT_PORT: u16 = 6970;

/// RTCP port offset from RTP port
pub const RTCP_PORT_OFFSET: u16 = 1;

// ============================================================================
// RTSP Method Types
// ============================================================================

/// RTSP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RTSPMethod {
    DESCRIBE,
    ANNOUNCE,
    GET_PARAMETER,
    OPTIONS,
    PAUSE,
    PLAY,
    RECORD,
    REDIRECT,
    SETUP,
    SET_PARAMETER,
    TEARDOWN,
}

impl RTSPMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            RTSPMethod::DESCRIBE => "DESCRIBE",
            RTSPMethod::ANNOUNCE => "ANNOUNCE",
            RTSPMethod::GET_PARAMETER => "GET_PARAMETER",
            RTSPMethod::OPTIONS => "OPTIONS",
            RTSPMethod::PAUSE => "PAUSE",
            RTSPMethod::PLAY => "PLAY",
            RTSPMethod::RECORD => "RECORD",
            RTSPMethod::REDIRECT => "REDIRECT",
            RTSPMethod::SETUP => "SETUP",
            RTSPMethod::SET_PARAMETER => "SET_PARAMETER",
            RTSPMethod::TEARDOWN => "TEARDOWN",
        }
    }
}

impl std::fmt::Display for RTSPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RTSPMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DESCRIBE" => Ok(RTSPMethod::DESCRIBE),
            "ANNOUNCE" => Ok(RTSPMethod::ANNOUNCE),
            "GET_PARAMETER" => Ok(RTSPMethod::GET_PARAMETER),
            "OPTIONS" => Ok(RTSPMethod::OPTIONS),
            "PAUSE" => Ok(RTSPMethod::PAUSE),
            "PLAY" => Ok(RTSPMethod::PLAY),
            "RECORD" => Ok(RTSPMethod::RECORD),
            "REDIRECT" => Ok(RTSPMethod::REDIRECT),
            "SETUP" => Ok(RTSPMethod::SETUP),
            "SET_PARAMETER" => Ok(RTSPMethod::SET_PARAMETER),
            "TEARDOWN" => Ok(RTSPMethod::TEARDOWN),
            _ => Err(format!("Unknown RTSP method: {}", s)),
        }
    }
}

// ============================================================================
// RTSP Transport Types
// ============================================================================

/// Transport protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportProtocol {
    UDP,
    TCP,
}

/// RTSP transport specification
#[derive(Debug, Clone)]
pub struct Transport {
    /// Transport protocol
    pub protocol: TransportProtocol,
    /// RTP port (client)
    pub client_rtp_port: Option<u16>,
    /// RTCP port (client)
    pub client_rtcp_port: Option<u16>,
    /// RTP port (server)
    pub server_rtp_port: Option<u16>,
    /// RTCP port (server)
    pub server_rtcp_port: Option<u16>,
    /// Interleaved channels (for TCP)
    pub interleaved: Option<(u8, u8)>,
    /// Multicast settings
    pub multicast: bool,
    /// TTL for multicast
    pub ttl: Option<u8>,
    /// Destination address
    pub destination: Option<String>,
    /// Mode (play or record)
    pub mode: String,
    /// SSRC
    pub ssrc: Option<u32>,
}

impl Transport {
    /// Create UDP transport
    pub fn udp(rtp_port: u16, rtcp_port: u16) -> Self {
        Self {
            protocol: TransportProtocol::UDP,
            client_rtp_port: Some(rtp_port),
            client_rtcp_port: Some(rtcp_port),
            server_rtp_port: None,
            server_rtcp_port: None,
            interleaved: None,
            multicast: false,
            ttl: None,
            destination: None,
            mode: "play".to_string(),
            ssrc: None,
        }
    }

    /// Create TCP interleaved transport
    pub fn tcp_interleaved(rtp_channel: u8, rtcp_channel: u8) -> Self {
        Self {
            protocol: TransportProtocol::TCP,
            client_rtp_port: None,
            client_rtcp_port: None,
            server_rtp_port: None,
            server_rtcp_port: None,
            interleaved: Some((rtp_channel, rtcp_channel)),
            multicast: false,
            ttl: None,
            destination: None,
            mode: "play".to_string(),
            ssrc: None,
        }
    }

    /// Parse transport string
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.is_empty() {
            return Err("Empty transport specification".to_string());
        }

        let protocol = if parts[0].to_lowercase().starts_with("rtp/avp/tcp") {
            TransportProtocol::TCP
        } else {
            TransportProtocol::UDP
        };

        let mut transport = Self {
            protocol,
            client_rtp_port: None,
            client_rtcp_port: None,
            server_rtp_port: None,
            server_rtcp_port: None,
            interleaved: None,
            multicast: parts.iter().any(|p| p.to_lowercase() == "multicast"),
            ttl: None,
            destination: None,
            mode: "play".to_string(),
            ssrc: None,
        };

        for part in &parts[1..] {
            let lower = part.to_lowercase();
            
            if lower.starts_with("client_port=") {
                let ports: Vec<&str> = part.split('=').nth(1).unwrap_or("").split('-').collect();
                if ports.len() >= 1 {
                    transport.client_rtp_port = ports[0].parse().ok();
                }
                if ports.len() >= 2 {
                    transport.client_rtcp_port = ports[1].parse().ok();
                }
            } else if lower.starts_with("server_port=") {
                let ports: Vec<&str> = part.split('=').nth(1).unwrap_or("").split('-').collect();
                if ports.len() >= 1 {
                    transport.server_rtp_port = ports[0].parse().ok();
                }
                if ports.len() >= 2 {
                    transport.server_rtcp_port = ports[1].parse().ok();
                }
            } else if lower.starts_with("interleaved=") {
                let channels: Vec<&str> = part.split('=').nth(1).unwrap_or("").split('-').collect();
                if channels.len() >= 2 {
                    let rtp: u8 = channels[0].parse().unwrap_or(0);
                    let rtcp: u8 = channels[1].parse().unwrap_or(1);
                    transport.interleaved = Some((rtp, rtcp));
                }
            } else if lower.starts_with("ttl=") {
                transport.ttl = part.split('=').nth(1).and_then(|s| s.parse().ok());
            } else if lower.starts_with("destination=") {
                transport.destination = part.split('=').nth(1).map(|s| s.to_string());
            } else if lower.starts_with("mode=") {
                transport.mode = part.split('=').nth(1).unwrap_or("play").to_string();
            } else if lower.starts_with("ssrc=") {
                transport.ssrc = part.split('=').nth(1).and_then(|s| u32::from_str_radix(s, 16).ok());
            }
        }

        Ok(transport)
    }

    /// Convert to transport string for request
    pub fn to_request_string(&self) -> String {
        let mut parts = match self.protocol {
            TransportProtocol::UDP => "RTP/AVP".to_string(),
            TransportProtocol::TCP => "RTP/AVP/TCP".to_string(),
        };

        if self.multicast {
            parts.push_str(";multicast");
        }

        if let Some((rtp, rtcp)) = self.interleaved {
            parts.push_str(&format!(";interleaved={}-{}", rtp, rtcp));
        }

        if let Some(rtp) = self.client_rtp_port {
            parts.push_str(&format!(";client_port={}", rtp));
            if let Some(rtcp) = self.client_rtcp_port {
                parts.push_str(&format!("-{}", rtcp));
            }
        }

        if let Some(ref dest) = self.destination {
            parts.push_str(&format!(";destination={}", dest));
        }

        parts.push_str(&format!(";mode={}", self.mode));

        parts
    }
}

impl std::fmt::Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_request_string())
    }
}

// ============================================================================
// RTSP Message Types
// ============================================================================

/// RTSP request
#[derive(Debug, Clone)]
pub struct RTSPRequest {
    /// Method
    pub method: RTSPMethod,
    /// URI
    pub uri: String,
    /// CSeq (sequence number)
    pub cseq: u32,
    /// Session ID
    pub session: Option<String>,
    /// Transport
    pub transport: Option<Transport>,
    /// Authorization header
    pub authorization: Option<String>,
    /// Range for PLAY
    pub range: Option<String>,
    /// Accept header
    pub accept: Option<String>,
    /// User agent
    pub user_agent: String,
    /// Additional headers
    pub headers: HashMap<String, String>,
    /// Content
    pub content: Option<String>,
}

impl RTSPRequest {
    /// Create new request
    pub fn new(method: RTSPMethod, uri: &str, cseq: u32) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            cseq,
            session: None,
            transport: None,
            authorization: None,
            range: None,
            accept: Some("application/sdp".to_string()),
            user_agent: "VantisMedia/1.0".to_string(),
            headers: HashMap::new(),
            content: None,
        }
    }

    /// Convert to string for sending
    pub fn to_string(&self) -> String {
        let mut msg = format!("{} {} {}\r\n", self.method, self.uri, RTSP_VERSION);
        msg.push_str(&format!("CSeq: {}\r\n", self.cseq));
        
        if let Some(ref session) = self.session {
            msg.push_str(&format!("Session: {}\r\n", session));
        }
        
        if let Some(ref transport) = self.transport {
            msg.push_str(&format!("Transport: {}\r\n", transport));
        }
        
        if let Some(ref auth) = self.authorization {
            msg.push_str(&format!("Authorization: {}\r\n", auth));
        }
        
        if let Some(ref range) = self.range {
            msg.push_str(&format!("Range: {}\r\n", range));
        }
        
        if let Some(ref accept) = self.accept {
            msg.push_str(&format!("Accept: {}\r\n", accept));
        }
        
        msg.push_str(&format!("User-Agent: {}\r\n", self.user_agent));
        
        for (key, value) in &self.headers {
            msg.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        if let Some(ref content) = self.content {
            msg.push_str(&format!("Content-Length: {}\r\n", content.len()));
            msg.push_str(&format!("Content-Type: application/sdp\r\n"));
        }
        
        msg.push_str("\r\n");
        
        if let Some(ref content) = self.content {
            msg.push_str(content);
        }
        
        msg
    }
}

/// RTSP response
#[derive(Debug, Clone)]
pub struct RTSPResponse {
    /// Status code
    pub status_code: u16,
    /// Reason phrase
    pub reason: String,
    /// CSeq
    pub cseq: u32,
    /// Session ID
    pub session: Option<String>,
    /// Transport
    pub transport: Option<Transport>,
    /// WWW-Authenticate header
    pub www_authenticate: Option<String>,
    /// SDP content
    pub content: Option<String>,
    /// Additional headers
    pub headers: HashMap<String, String>,
}

impl RTSPResponse {
    /// Parse response from string
    pub fn parse(data: &str) -> Result<Self, String> {
        let lines: Vec<&str> = data.split("\r\n").collect();
        
        if lines.is_empty() {
            return Err("Empty response".to_string());
        }
        
        // Parse status line
        let status_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if status_parts.len() < 3 {
            return Err("Invalid status line".to_string());
        }
        
        let status_code: u16 = status_parts[1].parse()
            .map_err(|_| "Invalid status code".to_string())?;
        
        let reason = status_parts[2..].join(" ");
        
        let mut response = Self {
            status_code,
            reason,
            cseq: 0,
            session: None,
            transport: None,
            www_authenticate: None,
            content: None,
            headers: HashMap::new(),
        };
        
        // Parse headers
        let mut i = 1;
        let mut content_length: usize = 0;
        
        while i < lines.len() && !lines[i].is_empty() {
            if let Some(colon_pos) = lines[i].find(':') {
                let key = lines[i][..colon_pos].trim().to_string();
                let value = lines[i][colon_pos + 1..].trim().to_string();
                
                match key.to_lowercase().as_str() {
                    "cseq" => response.cseq = value.parse().unwrap_or(0),
                    "session" => response.session = Some(value),
                    "transport" => response.transport = Transport::parse(&value).ok(),
                    "www-authenticate" => response.www_authenticate = Some(value),
                    "content-length" => content_length = value.parse().unwrap_or(0),
                    _ => {}
                }
                
                response.headers.insert(key, value);
            }
            i += 1;
        }
        
        // Extract content body
        if content_length > 0 {
            i += 1; // Skip empty line
            if i < lines.len() {
                let content_start = data.find("\r\n\r\n").map(|pos| pos + 4).unwrap_or(0);
                response.content = Some(data[content_start..].to_string());
            }
        }
        
        Ok(response)
    }
    
    /// Check if response indicates success
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}

// ============================================================================
// Authentication
// ============================================================================

/// Authentication credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Authentication type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthType {
    None,
    Basic,
    Digest,
}

/// Digest authentication parameters
#[derive(Debug, Clone)]
pub struct DigestAuth {
    pub realm: String,
    pub nonce: String,
    pub opaque: Option<String>,
    pub stale: bool,
    pub algorithm: String,
    pub qop: Option<String>,
}

impl DigestAuth {
    /// Parse from WWW-Authenticate header
    pub fn parse(www_authenticate: &str) -> Option<Self> {
        if !www_authenticate.to_lowercase().starts_with("digest") {
            return None;
        }
        
        let mut auth = Self {
            realm: String::new(),
            nonce: String::new(),
            opaque: None,
            stale: false,
            algorithm: "MD5".to_string(),
            qop: None,
        };
        
        let params = www_authenticate.trim_start_matches("Digest").trim();
        for part in params.split(',') {
            let part = part.trim();
            if let Some(eq_pos) = part.find('=') {
                let key = part[..eq_pos].trim();
                let value = part[eq_pos + 1..].trim().trim_matches('"');
                
                match key.to_lowercase().as_str() {
                    "realm" => auth.realm = value.to_string(),
                    "nonce" => auth.nonce = value.to_string(),
                    "opaque" => auth.opaque = Some(value.to_string()),
                    "stale" => auth.stale = value.to_lowercase() == "true",
                    "algorithm" => auth.algorithm = value.to_string(),
                    "qop" => auth.qop = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        
        if auth.realm.is_empty() || auth.nonce.is_empty() {
            return None;
        }
        
        Some(auth)
    }
    
    /// Generate Authorization header value
    pub fn authorize(&self, credentials: &Credentials, method: RTSPMethod, uri: &str) -> String {
        use md5::{Md5, Digest as Md5Digest};
        
        // Calculate HA1 = MD5(username:realm:password)
        let mut hasher = Md5::new();
        hasher.update(format!("{}:{}:{}", credentials.username, self.realm, credentials.password));
        let ha1 = format!("{:x}", hasher.finalize());
        
        // Calculate HA2 = MD5(method:uri)
        let mut hasher = Md5::new();
        hasher.update(format!("{}:{}", method, uri));
        let ha2 = format!("{:x}", hasher.finalize());
        
        // Calculate response
        let response = if let Some(ref qop) = self.qop {
            let cnonce = format!("{:x}", rand::random::<u64>());
            let nc = "00000001";
            
            let mut hasher = Md5::new();
            hasher.update(format!("{}:{}:{}:{}:{}:{}", ha1, self.nonce, nc, cnonce, qop, ha2));
            let response = format!("{:x}", hasher.finalize());
            
            let mut auth_str = format!(
                "Digest username=&quot;{}&quot;, realm=&quot;{}&quot;, nonce=&quot;{}&quot;, uri=&quot;{}&quot;, cnonce=&quot;{}&quot;, nc={}, qop={}, response=&quot;{}&quot;",
                credentials.username, self.realm, self.nonce, uri, cnonce, nc, qop, response
            );
            
            if let Some(ref opaque) = self.opaque {
                auth_str.push_str(&format!(", opaque=&quot;{}&quot;", opaque));
            }
            
            auth_str
        } else {
            let mut hasher = Md5::new();
            hasher.update(format!("{}:{}:{}", ha1, self.nonce, ha2));
            let response = format!("{:x}", hasher.finalize());
            
            format!(
                "Digest username=&quot;{}&quot;, realm=&quot;{}&quot;, nonce=&quot;{}&quot;, uri=&quot;{}&quot;, response=&quot;{}&quot;",
                credentials.username, self.realm, self.nonce, uri, response
            )
        };
        
        response
    }
}

// ============================================================================
// RTP Packet
// ============================================================================

/// RTP packet header
#[derive(Debug, Clone)]
pub struct RTPHeader {
    pub version: u8,
    pub padding: bool,
    pub extension: bool,
    pub csrc_count: u8,
    pub marker: bool,
    pub payload_type: u8,
    pub sequence_number: u16,
    pub timestamp: u32,
    pub ssrc: u32,
    pub csrc_list: Vec<u32>,
}

impl RTPHeader {
    /// Parse RTP header from bytes
    pub fn parse(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 12 {
            return Err("RTP packet too short".to_string());
        }
        
        let version = (data[0] >> 6) & 0x03;
        let padding = (data[0] & 0x20) != 0;
        let extension = (data[0] & 0x10) != 0;
        let csrc_count = data[0] & 0x0F;
        
        let marker = (data[1] & 0x80) != 0;
        let payload_type = data[1] & 0x7F;
        
        let sequence_number = ((data[2] as u16) << 8) | (data[3] as u16);
        let timestamp = ((data[4] as u32) << 24) | ((data[5] as u32) << 16) | 
                        ((data[6] as u32) << 8) | (data[7] as u32);
        let ssrc = ((data[8] as u32) << 24) | ((data[9] as u32) << 16) | 
                   ((data[10] as u32) << 8) | (data[11] as u32);
        
        let header_size = 12 + (csrc_count as usize * 4);
        if data.len() < header_size {
            return Err("RTP packet truncated".to_string());
        }
        
        let mut csrc_list = Vec::new();
        for i in 0..csrc_count as usize {
            let offset = 12 + (i * 4);
            let csrc = ((data[offset] as u32) << 24) | ((data[offset + 1] as u32) << 16) |
                       ((data[offset + 2] as u32) << 8) | (data[offset + 3] as u32);
            csrc_list.push(csrc);
        }
        
        Ok((Self {
            version,
            padding,
            extension,
            csrc_count,
            marker,
            payload_type,
            sequence_number,
            timestamp,
            ssrc,
            csrc_list,
        }, header_size))
    }
}

/// RTP packet
#[derive(Debug, Clone)]
pub struct RTPPacket {
    pub header: RTPHeader,
    pub payload: Vec<u8>,
}

impl RTPPacket {
    /// Parse RTP packet from bytes
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let (header, header_size) = RTPHeader::parse(data)?;
        
        let payload = if header.padding {
            if data.is_empty() {
                return Err("Empty packet with padding bit".to_string());
            }
            let padding_len = *data.last().unwrap() as usize;
            if padding_len > data.len() - header_size {
                return Err("Invalid padding length".to_string());
            }
            data[header_size..data.len() - padding_len].to_vec()
        } else {
            data[header_size..].to_vec()
        };
        
        Ok(Self { header, payload })
    }
}

// ============================================================================
// SDP Parser
// ============================================================================

/// SDP session description
#[derive(Debug, Clone, Default)]
pub struct SessionDescription {
    pub version: u32,
    pub origin: Option<Origin>,
    pub session_name: String,
    pub session_info: Option<String>,
    pub uri: Option<String>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub connection: Option<Connection>,
    pub bandwidth: Vec<Bandwidth>,
    pub timing: Option<Timing>,
    pub media: Vec<MediaDescription>,
    pub attributes: Vec<Attribute>,
}

/// SDP origin field
#[derive(Debug, Clone)]
pub struct Origin {
    pub username: String,
    pub session_id: u64,
    pub session_version: u64,
    pub nettype: String,
    pub addrtype: String,
    pub unicast_address: String,
}

/// SDP connection field
#[derive(Debug, Clone)]
pub struct Connection {
    pub nettype: String,
    pub addrtype: String,
    pub connection_address: String,
    pub ttl: Option<u8>,
    pub num_addresses: Option<u32>,
}

/// SDP bandwidth field
#[derive(Debug, Clone)]
pub struct Bandwidth {
    pub bwtype: String,
    pub bandwidth: u64,
}

/// SDP timing field
#[derive(Debug, Clone)]
pub struct Timing {
    pub start_time: u64,
    pub stop_time: u64,
    pub repeats: Vec<Repeat>,
}

/// SDP repeat field
#[derive(Debug, Clone)]
pub struct Repeat {
    pub interval: u64,
    pub duration: u64,
    pub offsets: Vec<u64>,
}

/// SDP media description
#[derive(Debug, Clone)]
pub struct MediaDescription {
    pub media: String,
    pub port: u16,
    pub num_ports: Option<u16>,
    pub proto: String,
    pub fmt: Vec<String>,
    pub connection: Option<Connection>,
    pub bandwidth: Vec<Bandwidth>,
    pub attributes: Vec<Attribute>,
}

/// SDP attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

/// SDP parser
pub struct SDPParser;

impl SDPParser {
    /// Parse SDP string
    pub fn parse(sdp: &str) -> Result<SessionDescription, String> {
        let mut session = SessionDescription::default();
        let mut current_media: Option<MediaDescription> = None;
        
        for line in sdp.lines() {
            if line.len() < 2 {
                continue;
            }
            
            let key = &line[0..1];
            let value = &line[2..];
            
            match key {
                "v" => session.version = value.parse().unwrap_or(0),
                "o" => session.origin = Self::parse_origin(value),
                "s" => session.session_name = value.to_string(),
                "i" => {
                    if let Some(ref mut media) = current_media {
                        media.attributes.push(Attribute {
                            name: "info".to_string(),
                            value: Some(value.to_string()),
                        });
                    } else {
                        session.session_info = Some(value.to_string());
                    }
                }
                "u" => session.uri = Some(value.to_string()),
                "e" => session.emails.push(value.to_string()),
                "p" => session.phones.push(value.to_string()),
                "c" => {
                    let conn = Self::parse_connection(value);
                    if let Some(ref mut media) = current_media {
                        media.connection = conn.clone();
                    } else {
                        session.connection = conn;
                    }
                }
                "b" => {
                    let bw = Self::parse_bandwidth(value);
                    if let Some(ref mut media) = current_media {
                        media.bandwidth.push(bw);
                    } else {
                        session.bandwidth.push(bw);
                    }
                }
                "t" => session.timing = Self::parse_timing(value),
                "m" => {
                    if let Some(media) = current_media.take() {
                        session.media.push(media);
                    }
                    current_media = Some(Self::parse_media(value));
                }
                "a" => {
                    let attr = Self::parse_attribute(value);
                    if let Some(ref mut media) = current_media {
                        media.attributes.push(attr);
                    } else {
                        session.attributes.push(attr);
                    }
                }
                _ => {}
            }
        }
        
        if let Some(media) = current_media {
            session.media.push(media);
        }
        
        Ok(session)
    }
    
    fn parse_origin(value: &str) -> Option<Origin> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }
        
        Some(Origin {
            username: parts[0].to_string(),
            session_id: parts[1].parse().ok()?,
            session_version: parts[2].parse().ok()?,
            nettype: parts[3].to_string(),
            addrtype: parts[4].to_string(),
            unicast_address: parts[5].to_string(),
        })
    }
    
    fn parse_connection(value: &str) -> Option<Connection> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        
        let mut ttl = None;
        let mut num_addresses = None;
        
        if parts[2].contains('/') {
            let addr_parts: Vec<&str> = parts[2].split('/').collect();
            if addr_parts.len() >= 2 {
                ttl = addr_parts[1].parse().ok();
            }
            if addr_parts.len() >= 3 {
                num_addresses = addr_parts[2].parse().ok();
            }
        }
        
        Some(Connection {
            nettype: parts[0].to_string(),
            addrtype: parts[1].to_string(),
            connection_address: parts[2].split('/').next().unwrap_or("").to_string(),
            ttl,
            num_addresses,
        })
    }
    
    fn parse_bandwidth(value: &str) -> Bandwidth {
        let parts: Vec<&str> = value.split(':').collect();
        Bandwidth {
            bwtype: parts.get(0).unwrap_or(&"").to_string(),
            bandwidth: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
        }
    }
    
    fn parse_timing(value: &str) -> Option<Timing> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        
        Some(Timing {
            start_time: parts[0].parse().ok()?,
            stop_time: parts[1].parse().ok()?,
            repeats: Vec::new(),
        })
    }
    
    fn parse_media(value: &str) -> MediaDescription {
        let parts: Vec<&str> = value.split_whitespace().collect();
        
        let media = parts.get(0).unwrap_or(&"").to_string();
        let port_str = parts.get(1).unwrap_or(&"0");
        
        let (port, num_ports) = if port_str.contains('/') {
            let port_parts: Vec<&str> = port_str.split('/').collect();
            (port_parts[0].parse().unwrap_or(0), port_parts.get(1).and_then(|s| s.parse().ok()))
        } else {
            (port_str.parse().unwrap_or(0), None)
        };
        
        let proto = parts.get(2).unwrap_or(&"").to_string();
        let fmt: Vec<String> = parts[3..].iter().map(|s| s.to_string()).collect();
        
        MediaDescription {
            media,
            port,
            num_ports,
            proto,
            fmt,
            connection: None,
            bandwidth: Vec::new(),
            attributes: Vec::new(),
        }
    }
    
    fn parse_attribute(value: &str) -> Attribute {
        if value.contains(':') {
            let parts: Vec<&str> = value.splitn(2, ':').collect();
            Attribute {
                name: parts[0].to_string(),
                value: Some(parts[1].to_string()),
            }
        } else {
            Attribute {
                name: value.to_string(),
                value: None,
            }
        }
    }
}

// ============================================================================
// RTSP Session
// ============================================================================

/// RTSP session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Init,
    Ready,
    Playing,
    Recording,
}

/// RTSP session info
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    /// Session timeout
    pub timeout: Duration,
    /// Tracks in this session
    pub tracks: HashMap<String, TrackInfo>,
    /// Current state
    pub state: SessionState,
}

/// Track information
#[derive(Debug, Clone)]
pub struct TrackInfo {
    /// Track URL
    pub url: String,
    /// Control URL
    pub control: Option<String>,
    /// RTP payload type
    pub payload_type: u8,
    /// Clock rate
    pub clock_rate: u32,
    /// Encoding name
    pub encoding: String,
    /// Transport
    pub transport: Option<Transport>,
}

// ============================================================================
// RTSP Client
// ============================================================================

/// RTSP client
pub struct RTSPClient {
    /// Server URL
    url: String,
    /// TCP stream
    stream: Option<TcpStream>,
    /// CSeq counter
    cseq: u32,
    /// Current session
    session: Option<SessionInfo>,
    /// Credentials
    credentials: Option<Credentials>,
    /// Authentication type
    auth_type: AuthType,
    /// Digest auth params
    digest_auth: Option<DigestAuth>,
    /// User agent
    user_agent: String,
    /// Connection timeout
    timeout: Duration,
    /// SDP
    sdp: Option<SessionDescription>,
    /// RTP receiver thread
    rtp_thread: Option<JoinHandle<()>>,
    /// Received packets (shared)
    received_packets: Arc<Mutex<Vec<RTPPacket>>>,
}

impl RTSPClient {
    /// Create new RTSP client
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            stream: None,
            cseq: 0,
            session: None,
            credentials: None,
            auth_type: AuthType::None,
            digest_auth: None,
            user_agent: "VantisMedia/1.0".to_string(),
            timeout: Duration::from_secs(10),
            sdp: None,
            rtp_thread: None,
            received_packets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Set credentials
    pub fn with_credentials(mut self, username: &str, password: &str) -> Self {
        self.credentials = Some(Credentials {
            username: username.to_string(),
            password: password.to_string(),
        });
        self
    }

    /// Set user agent
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.to_string();
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Connect to server
    pub fn connect(&mut self) -> Result<(), String> {
        let url = url::Url::parse(&self.url)
            .map_err(|e| format!("Invalid URL: {}", e))?;
        
        let host = url.host_str().ok_or("Missing host")?;
        let port = url.port().unwrap_or(RTSP_DEFAULT_PORT);
        
        let addr = format!("{}:{}", host, port);
        
        let stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| format!("Invalid address: {}", e))?,
            self.timeout,
        ).map_err(|e| format!("Connection failed: {}", e))?;
        
        stream.set_read_timeout(Some(self.timeout)).ok();
        stream.set_write_timeout(Some(self.timeout)).ok();
        
        self.stream = Some(stream);
        Ok(())
    }

    /// Send OPTIONS request
    pub fn options(&mut self) -> Result<Vec<RTSPMethod>, String> {
        let request = RTSPRequest::new(RTSPMethod::OPTIONS, &self.url, self.next_cseq());
        let response = self.send_request(&request)?;
        
        if response.is_success() {
            if let Some(public) = response.headers.get("Public") {
                return Ok(public.split(',')
                    .filter_map(|m| m.trim().parse().ok())
                    .collect());
            }
        }
        
        Err(format!("OPTIONS failed: {}", response.status_code))
    }

    /// Send DESCRIBE request
    pub fn describe(&mut self) -> Result<SessionDescription, String> {
        let mut request = RTSPRequest::new(RTSPMethod::DESCRIBE, &self.url, self.next_cseq());
        request.accept = Some("application/sdp".to_string());
        
        let response = self.send_request(&request)?;
        
        if !response.is_success() {
            if response.status_code == 401 {
                self.handle_auth_challenge(&response)?;
                return self.describe();
            }
            return Err(format!("DESCRIBE failed: {}", response.status_code));
        }
        
        let sdp_content = response.content.ok_or("No SDP content in response")?;
        let sdp = SDPParser::parse(&sdp_content)?;
        self.sdp = Some(sdp.clone());
        
        Ok(sdp)
    }

    /// Setup a track
    pub fn setup(&mut self, track_url: &str, transport: Transport) -> Result<Transport, String> {
        let mut request = RTSPRequest::new(RTSPMethod::SETUP, track_url, self.next_cseq());
        request.transport = Some(transport.clone());
        
        if let Some(ref session) = self.session {
            request.session = Some(session.id.clone());
        }
        
        let response = self.send_request(&request)?;
        
        if !response.is_success() {
            return Err(format!("SETUP failed: {}", response.status_code));
        }
        
        // Create or update session
        if self.session.is_none() {
            if let Some(session_id) = response.session.clone() {
                self.session = Some(SessionInfo {
                    id: session_id,
                    timeout: Duration::from_secs(60),
                    tracks: HashMap::new(),
                    state: SessionState::Ready,
                });
            }
        }
        
        response.transport.ok_or("No transport in response".to_string())
    }

    /// Start playback
    pub fn play(&mut self, range: Option<&str>) -> Result<(), String> {
        let session = self.session.as_ref()
            .ok_or("No active session")?;
        
        let mut request = RTSPRequest::new(RTSPMethod::PLAY, &self.url, self.next_cseq());
        request.session = Some(session.id.clone());
        request.range = range.map(|s| s.to_string());
        
        let response = self.send_request(&request)?;
        
        if !response.is_success() {
            return Err(format!("PLAY failed: {}", response.status_code));
        }
        
        if let Some(ref mut session) = self.session {
            session.state = SessionState::Playing;
        }
        
        Ok(())
    }

    /// Pause playback
    pub fn pause(&mut self) -> Result<(), String> {
        let session = self.session.as_ref()
            .ok_or("No active session")?;
        
        let mut request = RTSPRequest::new(RTSPMethod::PAUSE, &self.url, self.next_cseq());
        request.session = Some(session.id.clone());
        
        let response = self.send_request(&request)?;
        
        if !response.is_success() {
            return Err(format!("PAUSE failed: {}", response.status_code));
        }
        
        if let Some(ref mut session) = self.session {
            session.state = SessionState::Ready;
        }
        
        Ok(())
    }

    /// Teardown session
    pub fn teardown(&mut self) -> Result<(), String> {
        let session = self.session.as_ref()
            .ok_or("No active session")?;
        
        let mut request = RTSPRequest::new(RTSPMethod::TEARDOWN, &self.url, self.next_cseq());
        request.session = Some(session.id.clone());
        
        let response = self.send_request(&request)?;
        
        self.session = None;
        self.sdp = None;
        
        Ok(())
    }

    /// Close connection
    pub fn close(&mut self) {
        if let Err(_) = self.teardown() {
            // Ignore teardown errors
        }
        self.stream = None;
    }

    /// Get session description
    pub fn sdp(&self) -> Option<&SessionDescription> {
        self.sdp.as_ref()
    }

    /// Get session info
    pub fn session_info(&self) -> Option<&SessionInfo> {
        self.session.as_ref()
    }

    /// Get received packets
    pub fn received_packets(&self) -> Vec<RTPPacket> {
        self.received_packets.lock().unwrap().clone()
    }

    /// Clear received packets
    pub fn clear_packets(&mut self) {
        self.received_packets.lock().unwrap().clear();
    }

    // Private methods

    fn next_cseq(&mut self) -> u32 {
        self.cseq += 1;
        self.cseq
    }

    fn send_request(&mut self, request: &RTSPRequest) -> Result<RTSPResponse, String> {
        let stream = self.stream.as_mut()
            .ok_or("Not connected")?;
        
        let request_str = request.to_string();
        
        stream.write_all(request_str.as_bytes())
            .map_err(|e| format!("Send failed: {}", e))?;
        
        let mut buffer = [0u8; MAX_RTSP_MESSAGE_SIZE];
        let n = stream.read(&mut buffer)
            .map_err(|e| format!("Receive failed: {}", e))?;
        
        let response_str = String::from_utf8_lossy(&buffer[..n]);
        RTSPResponse::parse(&response_str)
    }

    fn handle_auth_challenge(&mut self, response: &RTSPResponse) -> Result<(), String> {
        let www_auth = response.www_authenticate.as_ref()
            .ok_or("No WWW-Authenticate header")?;
        
        if www_auth.to_lowercase().starts_with("digest") {
            let digest = DigestAuth::parse(www_auth)
                .ok_or("Invalid digest auth parameters")?;
            self.auth_type = AuthType::Digest;
            self.digest_auth = Some(digest);
        } else if www_auth.to_lowercase().starts_with("basic") {
            self.auth_type = AuthType::Basic;
        }
        
        Ok(())
    }
}

impl Drop for RTSPClient {
    fn drop(&mut self) {
        self.close();
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if URL is RTSP
pub fn is_rtsp_url(url: &str) -> bool {
    url.to_lowercase().starts_with("rtsp://") || url.to_lowercase().starts_with("rtsps://")
}

/// Extract track URLs from SDP
pub fn get_track_urls(sdp: &SessionDescription, base_url: &str) -> Vec<String> {
    let mut tracks = Vec::new();
    
    for media in &sdp.media {
        for attr in &media.attributes {
            if attr.name == "control" {
                if let Some(ref control) = attr.value {
                    if control.starts_with("rtsp://") {
                        tracks.push(control.clone());
                    } else {
                        tracks.push(format!("{}/{}", base_url.trim_end_matches('/'), control));
                    }
                }
            }
        }
    }
    
    tracks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtsp_method_parsing() {
        assert_eq!(RTSPMethod::PLAY, "PLAY".parse().unwrap());
        assert_eq!(RTSPMethod::OPTIONS, "OPTIONS".parse().unwrap());
    }

    #[test]
    fn test_transport_parsing() {
        let transport = Transport::parse("RTP/AVP/UDP;unicast;client_port=8000-8001").unwrap();
        assert_eq!(transport.protocol, TransportProtocol::UDP);
        assert_eq!(transport.client_rtp_port, Some(8000));
        assert_eq!(transport.client_rtcp_port, Some(8001));
    }

    #[test]
    fn test_rtp_header_parsing() {
        let data = [0x80, 0x60, 0x12, 0x34, 0x00, 0x00, 0x00, 0x01, 0xAB, 0xCD, 0xEF, 0x12];
        let (header, _) = RTPHeader::parse(&data).unwrap();
        assert_eq!(header.version, 2);
        assert_eq!(header.marker, false);
        assert_eq!(header.payload_type, 96);
        assert_eq!(header.sequence_number, 0x1234);
        assert_eq!(header.timestamp, 1);
        assert_eq!(header.ssrc, 0xABCDEF12);
    }

    #[test]
    fn test_sdp_parsing() {
        let sdp = "v=0\r\no=- 123456 123456 IN IP4 192.168.1.1\r\ns=Test Session\r\nt=0 0\r\nm=video 0 RTP/AVP 96\r\na=rtpmap:96 H264/90000\r\n";
        let parsed = SDPParser::parse(sdp).unwrap();
        assert_eq!(parsed.version, 0);
        assert_eq!(parsed.session_name, "Test Session");
        assert_eq!(parsed.media.len(), 1);
    }

    #[test]
    fn test_is_rtsp_url() {
        assert!(is_rtsp_url("rtsp://example.com/stream"));
        assert!(!is_rtsp_url("http://example.com/stream"));
    }
}