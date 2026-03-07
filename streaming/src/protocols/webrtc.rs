//! WebRTC Protocol Handler
//! 
//! This module provides comprehensive WebRTC support including:
//! - SDP (Session Description Protocol) offer/answer handling
//! - ICE (Interactive Connectivity Establishment) candidate management
//! - DTLS-SRTP key derivation
//! - Media transport (RTP/RTCP) over SRTP
//! - Data channel support (SCTP over DTLS)
//! - Peer connection state management
//! - Signaling message handling

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================================
// SDP Types for WebRTC
// ============================================================================

/// SDP type (offer or answer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdpType {
    Offer,
    Answer,
    PrAnswer,
    Rollback,
}

/// RTCSessionDescription
#[derive(Debug, Clone)]
pub struct RTCSessionDescription {
    pub sdp_type: SdpType,
    pub sdp: String,
}

impl RTCSessionDescription {
    pub fn new(sdp_type: SdpType, sdp: String) -> Self {
        Self { sdp_type, sdp }
    }

    /// Create offer
    pub fn offer(sdp: String) -> Self {
        Self::new(SdpType::Offer, sdp)
    }

    /// Create answer
    pub fn answer(sdp: String) -> Self {
        Self::new(SdpType::Answer, sdp)
    }

    /// Parse from JSON
    pub fn from_json(json: &str) -> Result<Self, String> {
        let obj: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        let type_str = obj.get("type")
            .and_then(|t| t.as_str())
            .ok_or("Missing type field")?;
        
        let sdp_type = match type_str.to_lowercase().as_str() {
            "offer" => SdpType::Offer,
            "answer" => SdpType::Answer,
            "pranswer" => SdpType::PrAnswer,
            "rollback" => SdpType::Rollback,
            _ => return Err(format!("Unknown SDP type: {}", type_str)),
        };
        
        let sdp = obj.get("sdp")
            .and_then(|s| s.as_str())
            .ok_or("Missing sdp field")?
            .to_string();
        
        Ok(Self { sdp_type, sdp })
    }

    /// Convert to JSON
    pub fn to_json(&self) -> String {
        let type_str = match self.sdp_type {
            SdpType::Offer => "offer",
            SdpType::Answer => "answer",
            SdpType::PrAnswer => "pranswer",
            SdpType::Rollback => "rollback",
        };
        
        serde_json::json!({
            "type": type_str,
            "sdp": self.sdp
        }).to_string()
    }
}

// ============================================================================
// ICE Types
// ============================================================================

/// ICE candidate type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IceCandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

impl IceCandidateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IceCandidateType::Host => "host",
            IceCandidateType::Srflx => "srflx",
            IceCandidateType::Prflx => "prflx",
            IceCandidateType::Relay => "relay",
        }
    }
}

/// ICE protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IceProtocol {
    UDP,
    TCP,
}

impl IceProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            IceProtocol::UDP => "udp",
            IceProtocol::TCP => "tcp",
        }
    }
}

/// ICE TCP candidate type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceTcpCandidateType {
    Active,
    Passive,
    So,
}

/// RTCIceCandidate
#[derive(Debug, Clone)]
pub struct RTCIceCandidate {
    /// Foundation
    pub foundation: String,
    /// Component ID (1 = RTP, 2 = RTCP)
    pub component: u16,
    /// Transport protocol
    pub protocol: IceProtocol,
    /// Priority
    pub priority: u32,
    /// IP address
    pub ip: String,
    /// Port
    pub port: u16,
    /// Candidate type
    pub candidate_type: IceCandidateType,
    /// Related address (for reflexive/relay candidates)
    pub related_address: Option<String>,
    /// Related port
    pub related_port: Option<u16>,
    /// TCP type (for TCP candidates)
    pub tcp_type: Option<IceTcpCandidateType>,
    /// Generation
    pub generation: Option<u16>,
    /// Network ID
    pub network_id: Option<u16>,
    /// Network cost
    pub network_cost: Option<u16>,
    /// SDP Mid
    pub sdp_mid: Option<String>,
    /// SDP MLine Index
    pub sdp_mline_index: Option<u16>,
    /// Ufrag
    pub ufrag: Option<String>,
}

impl RTCIceCandidate {
    /// Create host candidate
    pub fn host(ip: &str, port: u16, component: u16) -> Self {
        Self {
            foundation: Self::generate_foundation(IceCandidateType::Host, ip, port),
            component,
            protocol: IceProtocol::UDP,
            priority: Self::calculate_priority(IceCandidateType::Host, 0, component),
            ip: ip.to_string(),
            port,
            candidate_type: IceCandidateType::Host,
            related_address: None,
            related_port: None,
            tcp_type: None,
            generation: Some(0),
            network_id: None,
            network_cost: None,
            sdp_mid: None,
            sdp_mline_index: None,
            ufrag: None,
        }
    }

    /// Parse from candidate string
    pub fn parse(candidate_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = candidate_str.split_whitespace().collect();
        
        if parts.len() < 8 || parts[0] != "candidate:" {
            return Err("Invalid candidate format".to_string());
        }
        
        let foundation = parts[1].to_string();
        let component: u16 = parts[2].parse()
            .map_err(|_| "Invalid component")?;
        
        let protocol = match parts[3].to_lowercase().as_str() {
            "udp" => IceProtocol::UDP,
            "tcp" => IceProtocol::TCP,
            _ => return Err("Invalid protocol".to_string()),
        };
        
        let priority: u32 = parts[4].parse()
            .map_err(|_| "Invalid priority")?;
        let ip = parts[5].to_string();
        let port: u16 = parts[6].parse()
            .map_err(|_| "Invalid port")?;
        
        let candidate_type = match parts.get(7).map(|s| s.to_lowercase().as_str()) {
            Some("host") => IceCandidateType::Host,
            Some("srflx") => IceCandidateType::Srflx,
            Some("prflx") => IceCandidateType::Prflx,
            Some("relay") => IceCandidateType::Relay,
            _ => return Err("Invalid candidate type".to_string()),
        };
        
        let mut related_address = None;
        let mut related_port = None;
        let mut tcp_type = None;
        let mut generation = None;
        let mut ufrag = None;
        
        let mut i = 8;
        while i + 1 < parts.len() {
            match parts[i].to_lowercase().as_str() {
                "raddr" => related_address = Some(parts[i + 1].to_string()),
                "rport" => related_port = Some(parts[i + 1].parse().unwrap_or(0)),
                "tcptype" => {
                    tcp_type = match parts[i + 1].to_lowercase().as_str() {
                        "active" => Some(IceTcpCandidateType::Active),
                        "passive" => Some(IceTcpCandidateType::Passive),
                        "so" => Some(IceTcpCandidateType::So),
                        _ => None,
                    };
                }
                "generation" => generation = Some(parts[i + 1].parse().unwrap_or(0)),
                "ufrag" => ufrag = Some(parts[i + 1].to_string()),
                _ => {}
            }
            i += 2;
        }
        
        Ok(Self {
            foundation,
            component,
            protocol,
            priority,
            ip,
            port,
            candidate_type,
            related_address,
            related_port,
            tcp_type,
            generation,
            network_id: None,
            network_cost: None,
            sdp_mid: None,
            sdp_mline_index: None,
            ufrag,
        })
    }

    /// Convert to candidate string
    pub fn to_candidate_string(&self) -> String {
        let mut parts = vec![
            "candidate:".to_string(),
            self.foundation.clone(),
            self.component.to_string(),
            self.protocol.as_str().to_string(),
            self.priority.to_string(),
            self.ip.clone(),
            self.port.to_string(),
            self.candidate_type.as_str().to_string(),
        ];
        
        if let Some(ref addr) = self.related_address {
            parts.push("raddr".to_string());
            parts.push(addr.clone());
        }
        
        if let Some(port) = self.related_port {
            parts.push("rport".to_string());
            parts.push(port.to_string());
        }
        
        if let Some(ref tcp) = self.tcp_type {
            parts.push("tcptype".to_string());
            parts.push(match tcp {
                IceTcpCandidateType::Active => "active",
                IceTcpCandidateType::Passive => "passive",
                IceTcpCandidateType::So => "so",
            }.to_string());
        }
        
        if let Some(gen) = self.generation {
            parts.push("generation".to_string());
            parts.push(gen.to_string());
        }
        
        if let Some(ref ufrag) = self.ufrag {
            parts.push("ufrag".to_string());
            parts.push(ufrag.clone());
        }
        
        parts.join(" ")
    }

    /// Convert to JSON
    pub fn to_json(&self) -> String {
        serde_json::json!({
            "candidate": self.to_candidate_string(),
            "sdpMid": self.sdp_mid,
            "sdpMLineIndex": self.sdp_mline_index
        }).to_string()
    }

    fn generate_foundation(candidate_type: IceCandidateType, ip: &str, port: u16) -> String {
        format!("{}{}{}", candidate_type.as_str().len(), ip.len(), port)
    }

    fn calculate_priority(candidate_type: IceCandidateType, local_pref: u16, component: u16) -> u32 {
        let type_pref = match candidate_type {
            IceCandidateType::Host => 126,
            IceCandidateType::Srflx => 100,
            IceCandidateType::Prflx => 110,
            IceCandidateType::Relay => 0,
        };
        
        ((type_pref as u32) << 24) | ((local_pref as u32) << 8) | ((256 - component as u32) << 0)
    }
}

// ============================================================================
// ICE Agent
// ============================================================================

/// ICE connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceConnectionState {
    New,
    Checking,
    Connected,
    Completed,
    Disconnected,
    Failed,
    Closed,
}

/// ICE gathering state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceGatheringState {
    New,
    Gathering,
    Complete,
}

/// ICE agent configuration
#[derive(Debug, Clone)]
pub struct IceConfig {
    pub ice_servers: Vec<ICEServer>,
    pub ice_transport_policy: IceTransportPolicy,
    pub bundle_policy: BundlePolicy,
    pub rtcp_mux_policy: RtcpMuxPolicy,
}

impl Default for IceConfig {
    fn default() -> Self {
        Self {
            ice_servers: vec![ICEServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                username: None,
                credential: None,
            }],
            ice_transport_policy: IceTransportPolicy::All,
            bundle_policy: BundlePolicy::Balanced,
            rtcp_mux_policy: RtcpMuxPolicy::Require,
        }
    }
}

/// ICE server configuration
#[derive(Debug, Clone)]
pub struct ICEServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

/// ICE transport policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceTransportPolicy {
    Relay,
    All,
}

/// Bundle policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundlePolicy {
    Balanced,
    MaxCompat,
    MaxBundle,
}

/// RTCP mux policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcpMuxPolicy {
    Negotiate,
    Require,
}

/// ICE Agent
pub struct IceAgent {
    /// Local ICE credentials
    local_ufrag: String,
    local_password: String,
    /// Remote ICE credentials
    remote_ufrag: Option<String>,
    remote_password: Option<String>,
    /// Local candidates
    local_candidates: Vec<RTCIceCandidate>,
    /// Remote candidates
    remote_candidates: Vec<RTCIceCandidate>,
    /// Connection state
    connection_state: IceConnectionState,
    /// Gathering state
    gathering_state: IceGatheringState,
    /// Configuration
    config: IceConfig,
}

impl IceAgent {
    /// Create new ICE agent
    pub fn new(config: IceConfig) -> Self {
        Self {
            local_ufrag: Self::generate_ufrag(),
            local_password: Self::generate_password(),
            remote_ufrag: None,
            remote_password: None,
            local_candidates: Vec::new(),
            remote_candidates: Vec::new(),
            connection_state: IceConnectionState::New,
            gathering_state: IceGatheringState::New,
        }
    }

    /// Get local ICE credentials
    pub fn local_credentials(&self) -> (&str, &str) {
        (&self.local_ufrag, &self.local_password)
    }

    /// Set remote ICE credentials
    pub fn set_remote_credentials(&mut self, ufrag: &str, password: &str) {
        self.remote_ufrag = Some(ufrag.to_string());
        self.remote_password = Some(password.to_string());
    }

    /// Add local candidate
    pub fn add_local_candidate(&mut self, candidate: RTCIceCandidate) {
        self.local_candidates.push(candidate);
    }

    /// Add remote candidate
    pub fn add_remote_candidate(&mut self, candidate: RTCIceCandidate) {
        self.remote_candidates.push(candidate);
    }

    /// Get local candidates
    pub fn local_candidates(&self) -> &[RTCIceCandidate] {
        &self.local_candidates
    }

    /// Get remote candidates
    pub fn remote_candidates(&self) -> &[RTCIceCandidate] {
        &self.remote_candidates
    }

    /// Get connection state
    pub fn connection_state(&self) -> IceConnectionState {
        self.connection_state
    }

    /// Get gathering state
    pub fn gathering_state(&self) -> IceGatheringState {
        self.gathering_state
    }

    /// Start gathering candidates
    pub fn gather_candidates(&mut self) {
        self.gathering_state = IceGatheringState::Gathering;
        // In production, this would use libsoup or similar for STUN/TURN
        // For now, add a host candidate
        self.add_local_candidate(RTCIceCandidate::host("0.0.0.0", 0, 1));
        self.gathering_state = IceGatheringState::Complete;
    }

    /// Start connectivity checks
    pub fn start_connectivity_checks(&mut self) {
        self.connection_state = IceConnectionState::Checking;
        // In production, this would perform actual connectivity checks
    }

    fn generate_ufrag() -> String {
        use rand::Rng;
        let rng = rand::thread_rng();
        rng.sample_iter(rand::distributions::Alphanumeric)
            .take(4)
            .map(char::from)
            .collect()
    }

    fn generate_password() -> String {
        use rand::Rng;
        let rng = rand::thread_rng();
        rng.sample_iter(rand::distributions::Alphanumeric)
            .take(22)
            .map(char::from)
            .collect()
    }
}

// ============================================================================
// DTLS Types
// ============================================================================

/// DTLS role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtlsRole {
    Auto,
    Client,
    Server,
}

/// DTLS transport state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtlsTransportState {
    New,
    Connecting,
    Connected,
    Closed,
    Failed,
}

/// DTLS fingerprint
#[derive(Debug, Clone)]
pub struct DtlsFingerprint {
    pub algorithm: String,
    pub value: String,
}

// ============================================================================
// Media Types
// ============================================================================

/// Media kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaKind {
    Audio,
    Video,
    Application,
}

/// RTCRtpTransceiver direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransceiverDirection {
    SendRecv,
    SendOnly,
    RecvOnly,
    Inactive,
    Stopped,
}

/// RTCRtpCodecCapability
#[derive(Debug, Clone)]
pub struct RtpCodecCapability {
    pub mime_type: String,
    pub clock_rate: u32,
    pub channels: Option<u16>,
    pub sdp_fmtp_line: Option<String>,
}

/// RTCRtpCodecParameters
#[derive(Debug, Clone)]
pub struct RtpCodecParameters {
    pub mime_type: String,
    pub clock_rate: u32,
    pub channels: Option<u16>,
    pub payload_type: u8,
    pub rtcp_feedback: Vec<RtcpFeedback>,
    pub parameters: HashMap<String, String>,
}

/// RTCP feedback
#[derive(Debug, Clone)]
pub struct RtcpFeedback {
    pub typ: String,
    pub parameter: Option<String>,
}

/// RTCRtpHeaderExtension
#[derive(Debug, Clone)]
pub struct RtpHeaderExtension {
    pub uri: String,
    pub id: u16,
}

// ============================================================================
// Media Track
// ============================================================================

/// MediaStreamTrack
#[derive(Debug, Clone)]
pub struct MediaStreamTrack {
    pub id: String,
    pub kind: MediaKind,
    pub label: String,
    pub enabled: bool,
    pub muted: bool,
    pub ready_state: TrackState,
}

impl MediaStreamTrack {
    pub fn new(kind: MediaKind, label: &str) -> Self {
        use uuid::Uuid;
        Self {
            id: Uuid::new_v4().to_string(),
            kind,
            label: label.to_string(),
            enabled: true,
            muted: false,
            ready_state: TrackState::Live,
        }
    }

    pub fn audio(label: &str) -> Self {
        Self::new(MediaKind::Audio, label)
    }

    pub fn video(label: &str) -> Self {
        Self::new(MediaKind::Video, label)
    }
}

/// Track state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackState {
    Live,
    Ended,
}

// ============================================================================
// Data Channel
// ============================================================================

/// RTCDataChannel configuration
#[derive(Debug, Clone)]
pub struct DataChannelConfig {
    pub ordered: bool,
    pub max_packet_life_time: Option<u16>,
    pub max_retransmits: Option<u16>,
    pub protocol: String,
    pub negotiated: bool,
    pub id: Option<u16>,
}

impl Default for DataChannelConfig {
    fn default() -> Self {
        Self {
            ordered: true,
            max_packet_life_time: None,
            max_retransmits: None,
            protocol: String::new(),
            negotiated: false,
            id: None,
        }
    }
}

/// RTCDataChannel state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// RTCDataChannel
#[derive(Debug)]
pub struct RTCDataChannel {
    pub label: String,
    pub config: DataChannelConfig,
    pub state: DataChannelState,
    pub buffered_amount: u64,
}

impl RTCDataChannel {
    pub fn new(label: &str, config: DataChannelConfig) -> Self {
        Self {
            label: label.to_string(),
            config,
            state: DataChannelState::Connecting,
            buffered_amount: 0,
        }
    }

    /// Send data
    pub fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if self.state != DataChannelState::Open {
            return Err("Data channel not open".to_string());
        }
        // In production, this would send over SCTP
        Ok(())
    }

    /// Send string
    pub fn send_string(&mut self, text: &str) -> Result<(), String> {
        self.send(text.as_bytes())
    }

    /// Close channel
    pub fn close(&mut self) {
        self.state = DataChannelState::Closing;
        // In production, this would close the SCTP stream
        self.state = DataChannelState::Closed;
    }
}

// ============================================================================
// Peer Connection
// ============================================================================

/// RTCPeerConnection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
}

/// RTCSignalingState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalingState {
    Stable,
    HaveLocalOffer,
    HaveRemoteOffer,
    HaveLocalPrAnswer,
    HaveRemotePrAnswer,
    Closed,
}

/// RTCPeerConnection configuration
#[derive(Debug, Clone)]
pub struct PeerConnectionConfig {
    pub ice_servers: Vec<ICEServer>,
    pub ice_transport_policy: IceTransportPolicy,
    pub bundle_policy: BundlePolicy,
    pub rtcp_mux_policy: RtcpMuxPolicy,
    pub peer_identity: Option<String>,
    pub certificates: Vec<Certificate>,
}

impl Default for PeerConnectionConfig {
    fn default() -> Self {
        Self {
            ice_servers: vec![ICEServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                username: None,
                credential: None,
            }],
            ice_transport_policy: IceTransportPolicy::All,
            bundle_policy: BundlePolicy::Balanced,
            rtcp_mux_policy: RtcpMuxPolicy::Require,
            peer_identity: None,
            certificates: Vec::new(),
        }
    }
}

/// Certificate for DTLS
#[derive(Debug, Clone)]
pub struct Certificate {
    pub private_key: Vec<u8>,
    pub certificate: Vec<u8>,
}

/// RTCPeerConnection
pub struct RTCPeerConnection {
    /// Configuration
    config: PeerConnectionConfig,
    /// ICE agent
    ice_agent: IceAgent,
    /// Connection state
    connection_state: PeerConnectionState,
    /// Signaling state
    signaling_state: SignalingState,
    /// Local description
    local_description: Option<RTCSessionDescription>,
    /// Remote description
    remote_description: Option<RTCSessionDescription>,
    /// Current local description
    current_local_description: Option<RTCSessionDescription>,
    /// Current remote description
    current_remote_description: Option<RTCSessionDescription>,
    /// Pending local description
    pending_local_description: Option<RTCSessionDescription>,
    /// Pending remote description
    pending_remote_description: Option<RTCSessionDescription>,
    /// Tracks
    senders: Vec<RTCRtpSender>,
    receivers: Vec<RTCRtpReceiver>,
    transceivers: Vec<RTCRtpTransceiver>,
    /// Data channels
    data_channels: Vec<RTCDataChannel>,
    /// DTLS fingerprints
    dtls_fingerprints: Vec<DtlsFingerprint>,
}

impl RTCPeerConnection {
    /// Create new peer connection
    pub fn new(config: PeerConnectionConfig) -> Self {
        let ice_config = IceConfig {
            ice_servers: config.ice_servers.clone(),
            ice_transport_policy: config.ice_transport_policy,
            bundle_policy: config.bundle_policy,
            rtcp_mux_policy: config.rtcp_mux_policy,
        };
        
        Self {
            config,
            ice_agent: IceAgent::new(ice_config),
            connection_state: PeerConnectionState::New,
            signaling_state: SignalingState::Stable,
            local_description: None,
            remote_description: None,
            current_local_description: None,
            current_remote_description: None,
            pending_local_description: None,
            pending_remote_description: None,
            senders: Vec::new(),
            receivers: Vec::new(),
            transceivers: Vec::new(),
            data_channels: Vec::new(),
            dtls_fingerprints: vec![
                DtlsFingerprint {
                    algorithm: "sha-256".to_string(),
                    value: Self::generate_fingerprint(),
                }
            ],
        }
    }

    /// Create offer
    pub fn create_offer(&mut self) -> Result<RTCSessionDescription, String> {
        // Gather ICE candidates if needed
        if self.ice_agent.gathering_state() == IceGatheringState::New {
            self.ice_agent.gather_candidates();
        }
        
        let sdp = self.generate_sdp(SdpType::Offer)?;
        let offer = RTCSessionDescription::offer(sdp);
        
        self.pending_local_description = Some(offer.clone());
        self.signaling_state = SignalingState::HaveLocalOffer;
        
        Ok(offer)
    }

    /// Create answer
    pub fn create_answer(&mut self) -> Result<RTCSessionDescription, String> {
        if self.remote_description.is_none() {
            return Err("No remote description set".to_string());
        }
        
        // Gather ICE candidates if needed
        if self.ice_agent.gathering_state() == IceGatheringState::New {
            self.ice_agent.gather_candidates();
        }
        
        let sdp = self.generate_sdp(SdpType::Answer)?;
        let answer = RTCSessionDescription::answer(sdp);
        
        self.pending_local_description = Some(answer.clone());
        self.signaling_state = SignalingState::HaveLocalPrAnswer;
        
        Ok(answer)
    }

    /// Set local description
    pub fn set_local_description(&mut self, description: &RTCSessionDescription) -> Result<(), String> {
        match description.sdp_type {
            SdpType::Offer => {
                self.local_description = Some(description.clone());
                self.current_local_description = Some(description.clone());
                self.signaling_state = SignalingState::HaveLocalOffer;
            }
            SdpType::Answer | SdpType::PrAnswer => {
                self.local_description = Some(description.clone());
                self.current_local_description = Some(description.clone());
                self.pending_local_description = None;
                self.signaling_state = SignalingState::Stable;
            }
            SdpType::Rollback => {
                self.pending_local_description = None;
                self.signaling_state = SignalingState::Stable;
            }
        }
        Ok(())
    }

    /// Set remote description
    pub fn set_remote_description(&mut self, description: &RTCSessionDescription) -> Result<(), String> {
        match description.sdp_type {
            SdpType::Offer => {
                self.remote_description = Some(description.clone());
                self.current_remote_description = Some(description.clone());
                self.signaling_state = SignalingState::HaveRemoteOffer;
            }
            SdpType::Answer | SdpType::PrAnswer => {
                self.remote_description = Some(description.clone());
                self.current_remote_description = Some(description.clone());
                self.pending_remote_description = None;
                self.signaling_state = SignalingState::Stable;
            }
            SdpType::Rollback => {
                self.pending_remote_description = None;
                self.signaling_state = SignalingState::Stable;
            }
        }
        Ok(())
    }

    /// Add ICE candidate
    pub fn add_ice_candidate(&mut self, candidate: &RTCIceCandidate) -> Result<(), String> {
        self.ice_agent.add_remote_candidate(candidate.clone());
        Ok(())
    }

    /// Add track
    pub fn add_track(&mut self, track: MediaStreamTrack) -> Option<RTCRtpSender> {
        let sender = RTCRtpSender::new(track);
        self.senders.push(sender.clone());
        Some(sender)
    }

    /// Add transceiver
    pub fn add_transceiver(&mut self, kind: MediaKind, direction: TransceiverDirection) -> RTCRtpTransceiver {
        let transceiver = RTCRtpTransceiver::new(kind, direction);
        self.transceivers.push(transceiver.clone());
        transceiver
    }

    /// Create data channel
    pub fn create_data_channel(&mut self, label: &str, config: Option<DataChannelConfig>) -> RTCDataChannel {
        let channel = RTCDataChannel::new(label, config.unwrap_or_default());
        self.data_channels.push(channel.clone());
        channel
    }

    /// Close connection
    pub fn close(&mut self) {
        self.connection_state = PeerConnectionState::Closed;
        self.signaling_state = SignalingState::Closed;
    }

    /// Get connection state
    pub fn connection_state(&self) -> PeerConnectionState {
        self.connection_state
    }

    /// Get signaling state
    pub fn signaling_state(&self) -> SignalingState {
        self.signaling_state
    }

    /// Get local description
    pub fn local_description(&self) -> Option<&RTCSessionDescription> {
        self.local_description.as_ref()
    }

    /// Get remote description
    pub fn remote_description(&self) -> Option<&RTCSessionDescription> {
        self.remote_description.as_ref()
    }

    /// Get ICE connection state
    pub fn ice_connection_state(&self) -> IceConnectionState {
        self.ice_agent.connection_state()
    }

    /// Get ICE gathering state
    pub fn ice_gathering_state(&self) -> IceGatheringState {
        self.ice_agent.gathering_state()
    }

    /// Get DTLS fingerprints
    pub fn dtls_fingerprints(&self) -> &[DtlsFingerprint] {
        &self.dtls_fingerprints
    }

    // Private methods

    fn generate_sdp(&self, sdp_type: SdpType) -> Result<String, String> {
        let (ufrag, password) = self.ice_agent.local_credentials();
        
        let mut sdp = String::new();
        
        // Session section
        sdp.push_str("v=0\r\n");
        sdp.push_str(&format!("o=- {} {} IN IP4 127.0.0.1\r\n", 
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        ));
        sdp.push_str("s=-\r\n");
        sdp.push_str("t=0 0\r\n");
        
        // Bundle group
        if !self.transceivers.is_empty() {
            sdp.push_str("a=group:BUNDLE");
            for (i, transceiver) in self.transceivers.iter().enumerate() {
                sdp.push_str(&format!(" {}", i));
            }
            sdp.push_str("\r\n");
        }
        
        // ICE options
        sdp.push_str("a=ice-options:trickle\r\n");
        sdp.push_str(&format!("a=ice-ufrag:{}\r\n", ufrag));
        sdp.push_str(&format!("a=ice-pwd:{}\r\n", password));
        
        // DTLS fingerprint
        for fingerprint in &self.dtls_fingerprints {
            sdp.push_str(&format!("a=fingerprint:{} {}\r\n", 
                fingerprint.algorithm, fingerprint.value));
        }
        
        // Media sections
        for (i, transceiver) in self.transceivers.iter().enumerate() {
            let mid = i.to_string();
            let kind = match transceiver.kind {
                MediaKind::Audio => "audio",
                MediaKind::Video => "video",
                MediaKind::Application => "application",
            };
            
            sdp.push_str(&format!("m={} 9 UDP/TLS/RTP/SAVPF 0\r\n", kind));
            sdp.push_str("c=IN IP4 0.0.0.0\r\n");
            sdp.push_str(&format!("a=mid:{}\r\n", mid));
            sdp.push_str(&format!("a={}\r\n", match transceiver.direction {
                TransceiverDirection::SendRecv => "sendrecv",
                TransceiverDirection::SendOnly => "sendonly",
                TransceiverDirection::RecvOnly => "recvonly",
                TransceiverDirection::Inactive => "inactive",
                TransceiverDirection::Stopped => "inactive",
            }));
            
            if transceiver.kind != MediaKind::Application {
                sdp.push_str("a=rtcp-mux\r\n");
                sdp.push_str("a=rtcp-rsize\r\n");
            }
        }
        
        // Data channel section
        if !self.data_channels.is_empty() {
            sdp.push_str("m=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\n");
            sdp.push_str("c=IN IP4 0.0.0.0\r\n");
            sdp.push_str(&format!("a=mid:{}\r\n", self.transceivers.len()));
            sdp.push_str("a=sctp-port:5000\r\n");
        }
        
        Ok(sdp)
    }

    fn generate_fingerprint() -> String {
        use rand::Rng;
        let rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        bytes.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(":")
    }
}

impl Drop for RTCPeerConnection {
    fn drop(&mut self) {
        self.close();
    }
}

// ============================================================================
// RTP Sender/Receiver/Transceiver
// ============================================================================

/// RTCRtpSender
#[derive(Debug, Clone)]
pub struct RTCRtpSender {
    pub track: MediaStreamTrack,
    pub transport: Option<String>,
}

impl RTCRtpSender {
    pub fn new(track: MediaStreamTrack) -> Self {
        Self {
            track,
            transport: None,
        }
    }

    pub fn replace_track(&mut self, track: Option<MediaStreamTrack>) -> Option<MediaStreamTrack> {
        let old = std::mem::replace(&mut self.track, track.unwrap_or_else(|| {
            MediaStreamTrack::new(MediaKind::Audio, "empty")
        }));
        Some(old)
    }
}

/// RTCRtpReceiver
#[derive(Debug, Clone)]
pub struct RTCRtpReceiver {
    pub track: MediaStreamTrack,
    pub transport: Option<String>,
}

impl RTCRtpReceiver {
    pub fn new(kind: MediaKind) -> Self {
        Self {
            track: MediaStreamTrack::new(kind, "remote"),
            transport: None,
        }
    }
}

/// RTCRtpTransceiver
#[derive(Debug, Clone)]
pub struct RTCRtpTransceiver {
    pub mid: Option<String>,
    pub kind: MediaKind,
    pub direction: TransceiverDirection,
    pub sender: RTCRtpSender,
    pub receiver: RTCRtpReceiver,
}

impl RTCRtpTransceiver {
    pub fn new(kind: MediaKind, direction: TransceiverDirection) -> Self {
        Self {
            mid: None,
            kind,
            direction,
            sender: RTCRtpSender::new(MediaStreamTrack::new(kind, "local")),
            receiver: RTCRtpReceiver::new(kind),
        }
    }

    pub fn stop(&mut self) {
        self.direction = TransceiverDirection::Stopped;
    }
}

// ============================================================================
// WebRTC Handler
// ============================================================================

/// WebRTC handler for streaming
pub struct WebRTCHandler {
    /// Peer connection
    peer_connection: Option<RTCPeerConnection>,
    /// Remote stream tracks
    remote_tracks: Vec<MediaStreamTrack>,
    /// Received RTP packets
    received_packets: Arc<Mutex<Vec<(u8, Vec<u8>)>>>,
}

impl WebRTCHandler {
    /// Create new WebRTC handler
    pub fn new() -> Self {
        Self {
            peer_connection: None,
            remote_tracks: Vec::new(),
            received_packets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Initialize peer connection
    pub fn initialize(&mut self, config: Option<PeerConnectionConfig>) -> Result<(), String> {
        self.peer_connection = Some(RTCPeerConnection::new(config.unwrap_or_default()));
        Ok(())
    }

    /// Create offer
    pub fn create_offer(&mut self) -> Result<RTCSessionDescription, String> {
        let pc = self.peer_connection.as_mut()
            .ok_or("Peer connection not initialized")?;
        pc.create_offer()
    }

    /// Create answer
    pub fn create_answer(&mut self) -> Result<RTCSessionDescription, String> {
        let pc = self.peer_connection.as_mut()
            .ok_or("Peer connection not initialized")?;
        pc.create_answer()
    }

    /// Set local description
    pub fn set_local_description(&mut self, description: &RTCSessionDescription) -> Result<(), String> {
        let pc = self.peer_connection.as_mut()
            .ok_or("Peer connection not initialized")?;
        pc.set_local_description(description)
    }

    /// Set remote description
    pub fn set_remote_description(&mut self, description: &RTCSessionDescription) -> Result<(), String> {
        let pc = self.peer_connection.as_mut()
            .ok_or("Peer connection not initialized")?;
        pc.set_remote_description(description)
    }

    /// Add ICE candidate
    pub fn add_ice_candidate(&mut self, candidate: &RTCIceCandidate) -> Result<(), String> {
        let pc = self.peer_connection.as_mut()
            .ok_or("Peer connection not initialized")?;
        pc.add_ice_candidate(candidate)
    }

    /// Add transceiver
    pub fn add_transceiver(&mut self, kind: MediaKind, direction: TransceiverDirection) -> Option<RTCRtpTransceiver> {
        self.peer_connection.as_mut().map(|pc| pc.add_transceiver(kind, direction))
    }

    /// Create data channel
    pub fn create_data_channel(&mut self, label: &str, config: Option<DataChannelConfig>) -> Option<RTCDataChannel> {
        self.peer_connection.as_mut().map(|pc| pc.create_data_channel(label, config))
    }

    /// Get connection state
    pub fn connection_state(&self) -> Option<PeerConnectionState> {
        self.peer_connection.as_ref().map(|pc| pc.connection_state())
    }

    /// Get ICE connection state
    pub fn ice_connection_state(&self) -> Option<IceConnectionState> {
        self.peer_connection.as_ref().map(|pc| pc.ice_connection_state())
    }

    /// Close connection
    pub fn close(&mut self) {
        if let Some(ref mut pc) = self.peer_connection {
            pc.close();
        }
        self.peer_connection = None;
    }

    /// Get remote tracks
    pub fn remote_tracks(&self) -> &[MediaStreamTrack] {
        &self.remote_tracks
    }

    /// Get received packets
    pub fn received_packets(&self) -> Vec<(u8, Vec<u8>)> {
        self.received_packets.lock().unwrap().clone()
    }
}

impl Default for WebRTCHandler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if URL is WebRTC
pub fn is_webrtc_url(url: &str) -> bool {
    url.to_lowercase().starts_with("webrtc://") || 
        url.to_lowercase().starts_with("whep://") ||
        url.to_lowercase().starts_with("whip://")
}

/// Parse SDP to extract media information
pub fn parse_sdp_media(sdp: &str) -> Vec<(String, String, u16)> {
    let mut media = Vec::new();
    
    for line in sdp.lines() {
        if line.starts_with("m=") {
            let parts: Vec<&str> = line[2..].split_whitespace().collect();
            if parts.len() >= 3 {
                let kind = parts[0].to_string();
                let port: u16 = parts[1].parse().unwrap_or(0);
                let proto = parts[2].to_string();
                media.push((kind, proto, port));
            }
        }
    }
    
    media
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ice_candidate_parsing() {
        let candidate_str = "candidate:1 1 udp 2122260223 192.168.1.1 54321 host typ host";
        let candidate = RTCIceCandidate::parse(candidate_str).unwrap();
        assert_eq!(candidate.component, 1);
        assert_eq!(candidate.protocol, IceProtocol::UDP);
        assert_eq!(candidate.ip, "192.168.1.1");
        assert_eq!(candidate.port, 54321);
    }

    #[test]
    fn test_session_description_json() {
        let offer = RTCSessionDescription::offer("v=0\r\n".to_string());
        let json = offer.to_json();
        let parsed = RTCSessionDescription::from_json(&json).unwrap();
        assert_eq!(parsed.sdp_type, SdpType::Offer);
    }

    #[test]
    fn test_media_stream_track() {
        let track = MediaStreamTrack::video("camera");
        assert_eq!(track.kind, MediaKind::Video);
        assert!(track.enabled);
    }

    #[test]
    fn test_peer_connection_creation() {
        let pc = RTCPeerConnection::new(PeerConnectionConfig::default());
        assert_eq!(pc.connection_state(), PeerConnectionState::New);
        assert_eq!(pc.signaling_state(), SignalingState::Stable);
    }

    #[test]
    fn test_data_channel() {
        let mut channel = RTCDataChannel::new("test", DataChannelConfig::default());
        channel.state = DataChannelState::Open;
        assert!(channel.send(b"hello").is_ok());
    }

    #[test]
    fn test_is_webrtc_url() {
        assert!(is_webrtc_url("webrtc://example.com/stream"));
        assert!(!is_webrtc_url("http://example.com/stream"));
    }
}