//! Vantis Streaming - Advanced network streaming module
//! 
//! This module provides comprehensive streaming capabilities including:
//! - Adaptive streaming quality selection
//! - P2P streaming support
//! - Stream caching system
//! - Bandwidth-aware quality adjustment
//! - Stream recording capabilities

pub mod adaptive;
pub mod p2p;
pub mod cache;
pub mod bandwidth;
pub mod recorder;
pub mod protocols;
pub mod quality;
pub mod utils;

pub use adaptive::{AdaptiveStreamer, AdaptiveConfig, QualityLevel};
pub use p2p::{P2PStreamer, P2PConfig, PeerInfo};
pub use cache::{StreamCache, CacheConfig, CacheEntry};
pub use bandwidth::{BandwidthMonitor, BandwidthStats};
pub use recorder::{StreamRecorder, RecorderConfig, RecordingFormat};
pub use protocols::{
    ProtocolType, StreamInfo, TrackInfo, TrackType,
    HLSHandler, M3U8Parser, M3U8Playlist, PlaylistType,
    DASHHandler, MPDParser, MediaPresentationDescription, MPDType,
    RTSPClient, RTSPRequest, RTSPResponse, Transport, SDPParser, SessionDescription,
    WebRTCHandler, RTCPeerConnection, RTCSessionDescription, RTCIceCandidate,
    IceConnectionState, PeerConnectionState, MediaStreamTrack, MediaKind, DataChannelState,
};
pub use quality::{QualitySelector, QualityMetrics};

use thiserror::Error;

/// Streaming error types
#[derive(Error, Debug)]
pub enum StreamingError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Buffer error: {0}")]
    BufferError(String),
    
    #[error("Quality selection error: {0}")]
    QualityError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("P2P error: {0}")]
    P2PError(String),
    
    #[error("Recording error: {0}")]
    RecordingError(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for streaming operations
pub type StreamingResult<T> = Result<T, StreamingError>;

/// Streaming configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreamingConfig {
    /// Enable adaptive streaming
    pub enable_adaptive: bool,
    
    /// Enable P2P streaming
    pub enable_p2p: bool,
    
    /// Enable stream caching
    pub enable_caching: bool,
    
    /// Enable bandwidth monitoring
    pub enable_bandwidth_monitoring: bool,
    
    /// Enable stream recording
    pub enable_recording: bool,
    
    /// Buffer size in bytes
    pub buffer_size: usize,
    
    /// Maximum buffer duration in seconds
    pub max_buffer_duration_secs: f64,
    
    /// Reconnection attempts
    pub max_reconnect_attempts: usize,
    
    /// Reconnection delay in seconds
    pub reconnect_delay_secs: f64,
    
    /// User agent string
    pub user_agent: String,
    
    /// Enable HTTP/2
    pub enable_http2: bool,
    
    /// Enable TLS verification
    pub enable_tls_verification: bool,
    
    /// Custom headers
    pub custom_headers: std::collections::HashMap<String, String>,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enable_adaptive: true,
            enable_p2p: false,
            enable_caching: true,
            enable_bandwidth_monitoring: true,
            enable_recording: false,
            buffer_size: 16 * 1024 * 1024, // 16 MB
            max_buffer_duration_secs: 30.0,
            max_reconnect_attempts: 5,
            reconnect_delay_secs: 2.0,
            user_agent: "VantisMediaPlayer/1.0".to_string(),
            enable_http2: true,
            enable_tls_verification: true,
            custom_headers: std::collections::HashMap::new(),
        }
    }
}

/// Stream statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreamStats {
    /// Bytes received
    pub bytes_received: u64,
    
    /// Bytes sent
    pub bytes_sent: u64,
    
    /// Packets received
    pub packets_received: u64,
    
    /// Packets lost
    pub packets_lost: u64,
    
    /// Current bitrate in bps
    pub current_bitrate: u64,
    
    /// Average bitrate in bps
    pub average_bitrate: u64,
    
    /// Buffer health (0.0 - 1.0)
    pub buffer_health: f32,
    
    /// Rebuffer count
    pub rebuffer_count: usize,
    
    /// Total rebuffer duration in seconds
    pub total_rebuffer_duration_secs: f64,
    
    /// Connection uptime in seconds
    pub uptime_secs: f64,
    
    /// Current quality level
    pub current_quality: QualityLevel,
    
    /// Number of quality changes
    pub quality_changes: usize,
}

impl Default for StreamStats {
    fn default() -> Self {
        Self {
            bytes_received: 0,
            bytes_sent: 0,
            packets_received: 0,
            packets_lost: 0,
            current_bitrate: 0,
            average_bitrate: 0,
            buffer_health: 1.0,
            rebuffer_count: 0,
            total_rebuffer_duration_secs: 0.0,
            uptime_secs: 0.0,
            current_quality: QualityLevel::Auto,
            quality_changes: 0,
        }
    }
}

/// Main streaming engine
pub struct StreamingEngine {
    config: StreamingConfig,
    adaptive_streamer: Option<AdaptiveStreamer>,
    p2p_streamer: Option<P2PStreamer>,
    cache: Option<StreamCache>,
    bandwidth_monitor: Option<BandwidthMonitor>,
    recorder: Option<StreamRecorder>,
    stats: StreamStats,
}

impl StreamingEngine {
    /// Create a new streaming engine
    pub fn new() -> StreamingResult<Self> {
        Self::with_config(StreamingConfig::default())
    }
    
    /// Create a new streaming engine with custom configuration
    pub fn with_config(config: StreamingConfig) -> StreamingResult<Self> {
        let mut engine = Self {
            adaptive_streamer: None,
            p2p_streamer: None,
            cache: None,
            bandwidth_monitor: None,
            recorder: None,
            stats: StreamStats::default(),
            config,
        };
        
        // Initialize components based on config
        if engine.config.enable_adaptive {
            engine.adaptive_streamer = Some(AdaptiveStreamer::new()?);
        }
        
        if engine.config.enable_p2p {
            engine.p2p_streamer = Some(P2PStreamer::new()?);
        }
        
        if engine.config.enable_caching {
            engine.cache = Some(StreamCache::new(CacheConfig::default())?);
        }
        
        if engine.config.enable_bandwidth_monitoring {
            engine.bandwidth_monitor = Some(BandwidthMonitor::new()?);
        }
        
        if engine.config.enable_recording {
            engine.recorder = Some(StreamRecorder::new(RecorderConfig::default())?);
        }
        
        Ok(engine)
    }
    
    /// Get the configuration
    pub fn config(&self) -> &StreamingConfig {
        &self.config
    }
    
    /// Get current statistics
    pub fn stats(&self) -> &StreamStats {
        &self.stats
    }
    
    /// Get adaptive streamer
    pub fn adaptive_streamer(&self) -> Option<&AdaptiveStreamer> {
        self.adaptive_streamer.as_ref()
    }
    
    /// Get P2P streamer
    pub fn p2p_streamer(&self) -> Option<&P2PStreamer> {
        self.p2p_streamer.as_ref()
    }
    
    /// Get cache
    pub fn cache(&self) -> Option<&StreamCache> {
        self.cache.as_ref()
    }
    
    /// Get bandwidth monitor
    pub fn bandwidth_monitor(&self) -> Option<&BandwidthMonitor> {
        self.bandwidth_monitor.as_ref()
    }
    
    /// Get recorder
    pub fn recorder(&self) -> Option<&StreamRecorder> {
        self.recorder.as_ref()
    }
    
    /// Update statistics
    pub fn update_stats(&mut self, stats: StreamStats) {
        self.stats = stats;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_streaming_config_default() {
        let config = StreamingConfig::default();
        assert!(config.enable_adaptive);
        assert_eq!(config.buffer_size, 16 * 1024 * 1024);
    }
    
    #[test]
    fn test_streaming_engine_creation() {
        let engine = StreamingEngine::new();
        assert!(engine.is_ok());
    }
}