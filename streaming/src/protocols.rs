//! Streaming protocols module
//! 
//! Support for various streaming protocols including HTTP, HLS, DASH, RTSP, etc.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use url::Url;

/// Streaming protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamProtocol {
    /// HTTP/HTTPS progressive download
    HTTP,
    
    /// HLS (HTTP Live Streaming)
    HLS,
    
    /// DASH (Dynamic Adaptive Streaming over HTTP)
    DASH,
    
    /// RTSP (Real Time Streaming Protocol)
    RTSP,
    
    /// RTMP (Real-Time Messaging Protocol)
    RTMP,
    
    /// WebRTC
    WebRTC,
    
    /// P2P (Peer-to-Peer)
    P2P,
    
    /// Unknown protocol
    Unknown,
}

impl StreamProtocol {
    /// Detect protocol from URL
    pub fn from_url(url: &str) -> Self {
        if let Ok(parsed) = Url::parse(url) {
            match parsed.scheme() {
                "http" | "https" => {
                    // Check for HLS or DASH based on extension
                    if let Some(path) = parsed.path() {
                        if path.ends_with(".m3u8") || path.ends_with(".m3u") {
                            return StreamProtocol::HLS;
                        } else if path.ends_with(".mpd") {
                            return StreamProtocol::DASH;
                        }
                    }
                    StreamProtocol::HTTP
                }
                "rtsp" => StreamProtocol::RTSP,
                "rtmp" | "rtmps" => StreamProtocol::RTMP,
                "webrtc" => StreamProtocol::WebRTC,
                "p2p" => StreamProtocol::P2P,
                _ => StreamProtocol::Unknown,
            }
        } else {
            StreamProtocol::Unknown
        }
    }
    
    /// Get protocol name
    pub fn name(&self) -> &str {
        match self {
            StreamProtocol::HTTP => "HTTP",
            StreamProtocol::HLS => "HLS",
            StreamProtocol::DASH => "DASH",
            StreamProtocol::RTSP => "RTSP",
            StreamProtocol::RTMP => "RTMP",
            StreamProtocol::WebRTC => "WebRTC",
            StreamProtocol::P2P => "P2P",
            StreamProtocol::Unknown => "Unknown",
        }
    }
    
    /// Check if protocol supports adaptive streaming
    pub fn supports_adaptive(&self) -> bool {
        matches!(self, StreamProtocol::HLS | StreamProtocol::DASH)
    }
    
    /// Check if protocol is real-time
    pub fn is_realtime(&self) -> bool {
        matches!(self, StreamProtocol::RTSP | StreamProtocol::RTMP | StreamProtocol::WebRTC)
    }
}

/// Stream source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSource {
    /// Source URL
    pub url: String,
    
    /// Protocol
    pub protocol: StreamProtocol,
    
    /// Is live stream
    pub is_live: bool,
    
    /// Authentication
    pub auth: Option<Authentication>,
    
    /// Custom headers
    pub headers: std::collections::HashMap<String, String>,
    
    /// Proxy configuration
    pub proxy: Option<ProxyConfig>,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Authentication {
    /// No authentication
    None,
    
    /// Basic authentication
    Basic { username: String, password: String },
    
    /// Bearer token
    Bearer { token: String },
    
    /// Custom authentication
    Custom { scheme: String, credentials: String },
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy URL
    pub url: String,
    
    /// Proxy authentication
    pub auth: Option<Authentication>,
    
    /// Enable proxy for HTTPS
    pub enable_https: bool,
}

/// Stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Stream ID
    pub stream_id: String,
    
    /// Stream source
    pub source: StreamSource,
    
    /// Duration in seconds (0 for live streams)
    pub duration_secs: f64,
    
    /// Available quality levels
    pub quality_levels: Vec<QualityLevel>,
    
    /// Video codec
    pub video_codec: Option<String>,
    
    /// Audio codec
    pub audio_codec: Option<String>,
    
    /// Resolution
    pub resolution: Option<(u32, u32)>,
    
    /// Frame rate
    pub frame_rate: Option<f32>,
    
    /// Bitrate in bps
    pub bitrate: Option<u64>,
    
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Quality level for adaptive streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityLevel {
    /// Quality level ID
    pub id: String,
    
    /// Bandwidth in bps
    pub bandwidth: u64,
    
    /// Resolution
    pub resolution: (u32, u32),
    
    /// Frame rate
    pub frame_rate: f32,
    
    /// Codec
    pub codec: String,
    
    /// URL for this quality level
    pub url: String,
}

/// Protocol handler trait
pub trait ProtocolHandler {
    /// Open stream
    fn open(&mut self, source: &StreamSource) -> StreamingResult<StreamInfo>;
    
    /// Read data from stream
    fn read(&mut self, buffer: &mut [u8]) -> StreamingResult<usize>;
    
    /// Seek to position
    fn seek(&mut self, position: f64) -> StreamingResult<()>;
    
    /// Close stream
    fn close(&mut self) -> StreamingResult<()>;
    
    /// Get current position
    fn position(&self) -> f64;
    
    /// Get stream duration
    fn duration(&self) -> f64;
    
    /// Is seekable
    fn is_seekable(&self) -> bool;
}

/// HTTP protocol handler
pub struct HttpHandler {
    client: reqwest::blocking::Client,
    response: Option<reqwest::blocking::Response>,
    position: f64,
    duration: f64,
}

impl HttpHandler {
    /// Create a new HTTP handler
    pub fn new() -> StreamingResult<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            client,
            response: None,
            position: 0.0,
            duration: 0.0,
        })
    }
}

impl ProtocolHandler for HttpHandler {
    fn open(&mut self, source: &StreamSource) -> StreamingResult<StreamInfo> {
        let mut request = self.client.get(&source.url);
        
        // Add headers
        for (key, value) in &source.headers {
            request = request.header(key, value);
        }
        
        // Add authentication
        if let Some(auth) = &source.auth {
            match auth {
                Authentication::Basic { username, password } => {
                    request = request.basic_auth(username, Some(password));
                }
                Authentication::Bearer { token } => {
                    request = request.bearer_auth(token);
                }
                Authentication::Custom { scheme, credentials } => {
                    request = request.header("Authorization", format!("{} {}", scheme, credentials));
                }
                Authentication::None => {}
            }
        }
        
        // Send request
        let response = request.send()?;
        
        if !response.status().is_success() {
            return Err(StreamingError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }
        
        self.response = Some(response);
        
        // Get content length
        let content_length = self.response.as_ref()
            .and_then(|r| r.content_length())
            .unwrap_or(0);
        
        // Estimate duration (simplified)
        self.duration = if content_length > 0 {
            // Assume 5 Mbps average bitrate
            content_length as f64 / (5_000_000.0 / 8.0)
        } else {
            0.0
        };
        
        Ok(StreamInfo {
            stream_id: uuid::Uuid::new_v4().to_string(),
            source: source.clone(),
            duration_secs: self.duration,
            quality_levels: Vec::new(),
            video_codec: None,
            audio_codec: None,
            resolution: None,
            frame_rate: None,
            bitrate: None,
            metadata: std::collections::HashMap::new(),
        })
    }
    
    fn read(&mut self, buffer: &mut [u8]) -> StreamingResult<usize> {
        if let Some(ref mut response) = self.response {
            let mut chunk = response.take(buffer.len() as u64);
            let bytes_read = chunk.read_to_end(buffer)?;
            self.position += bytes_read as f64;
            Ok(bytes_read)
        } else {
            Err(StreamingError::NetworkError("Stream not open".to_string()))
        }
    }
    
    fn seek(&mut self, position: f64) -> StreamingResult<()> {
        // HTTP progressive download doesn't support seeking
        // Would need to reopen with Range header
        Err(StreamingError::ProtocolError("HTTP seeking not supported".to_string()))
    }
    
    fn close(&mut self) -> StreamingResult<()> {
        self.response = None;
        self.position = 0.0;
        Ok(())
    }
    
    fn position(&self) -> f64 {
        self.position
    }
    
    fn duration(&self) -> f64 {
        self.duration
    }
    
    fn is_seekable(&self) -> bool {
        false
    }
}

/// Create protocol handler for given source
pub fn create_handler(source: &StreamSource) -> StreamingResult<Box<dyn ProtocolHandler>> {
    match source.protocol {
        StreamProtocol::HTTP => Ok(Box::new(HttpHandler::new()?)),
        StreamProtocol::HLS => {
            // Would create HLS handler
            Ok(Box::new(HttpHandler::new()?))
        }
        StreamProtocol::DASH => {
            // Would create DASH handler
            Ok(Box::new(HttpHandler::new()?))
        }
        StreamProtocol::RTSP => {
            // Would create RTSP handler
            Err(StreamingError::ProtocolError("RTSP not yet implemented".to_string()))
        }
        StreamProtocol::RTMP => {
            // Would create RTMP handler
            Err(StreamingError::ProtocolError("RTMP not yet implemented".to_string()))
        }
        StreamProtocol::WebRTC => {
            // Would create WebRTC handler
            Err(StreamingError::ProtocolError("WebRTC not yet implemented".to_string()))
        }
        StreamProtocol::P2P => {
            // Would create P2P handler
            Err(StreamingError::ProtocolError("P2P not yet implemented".to_string()))
        }
        StreamProtocol::Unknown => {
            Err(StreamingError::UnsupportedProtocol(source.protocol.name().to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_protocol_detection() {
        assert_eq!(StreamProtocol::from_url("http://example.com/video.mp4"), StreamProtocol::HTTP);
        assert_eq!(StreamProtocol::from_url("https://example.com/stream.m3u8"), StreamProtocol::HLS);
        assert_eq!(StreamProtocol::from_url("https://example.com/stream.mpd"), StreamProtocol::DASH);
        assert_eq!(StreamProtocol::from_url("rtsp://example.com/stream"), StreamProtocol::RTSP);
    }
    
    #[test]
    fn test_protocol_properties() {
        assert!(StreamProtocol::HLS.supports_adaptive());
        assert!(StreamProtocol::DASH.supports_adaptive());
        assert!(!StreamProtocol::HTTP.supports_adaptive());
        
        assert!(StreamProtocol::RTSP.is_realtime());
        assert!(!StreamProtocol::HTTP.is_realtime());
    }
    
    #[test]
    fn test_http_handler_creation() {
        let handler = HttpHandler::new();
        assert!(handler.is_ok());
    }
}