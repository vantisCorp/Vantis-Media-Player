//! Network Streaming Protocol Handlers
//! 
//! This module provides comprehensive support for various streaming protocols:
//! 
//! - **HLS** (HTTP Live Streaming): M3U8 playlist parsing, adaptive bitrate streaming
//! - **DASH** (Dynamic Adaptive Streaming over HTTP): MPD parsing, multi-period support
//! - **RTSP** (Real-Time Streaming Protocol): RTP/RTCP handling, SDP parsing
//! - **WebRTC**: Peer-to-peer streaming, ICE/DTLS/SRTP support

pub mod hls;
pub mod dash;
pub mod rtsp;
pub mod webrtc;

// Re-export main types
pub use hls::{HLSHandler, M3U8Parser, M3U8Playlist, PlaylistType};
pub use dash::{DASHHandler, MPDParser, MediaPresentationDescription, MPDType};
pub use rtsp::{RTSPClient, RTSPRequest, RTSPResponse, Transport, SDPParser, SessionDescription};
pub use webrtc::{
    RTCPeerConnection, RTCSessionDescription, RTCIceCandidate, 
    WebRTCHandler, IceConnectionState, PeerConnectionState,
    MediaStreamTrack, MediaKind, DataChannelState,
};

use std::time::Duration;

/// Protocol type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType {
    HLS,
    DASH,
    RTSP,
    WebRTC,
    HTTP,
    File,
    Unknown,
}

impl ProtocolType {
    /// Detect protocol from URL
    pub fn from_url(url: &str) -> Self {
        let lower = url.to_lowercase();
        
        if lower.starts_with("rtsp://") || lower.starts_with("rtsps://") {
            Self::RTSP
        } else if lower.starts_with("webrtc://") || lower.starts_with("whep://") || lower.starts_with("whip://") {
            Self::WebRTC
        } else if lower.ends_with(".m3u8") || lower.contains(".m3u8?") {
            Self::HLS
        } else if lower.ends_with(".mpd") || lower.contains(".mpd?") {
            Self::DASH
        } else if lower.starts_with("http://") || lower.starts_with("https://") {
            // Could be HLS or DASH, check extension
            if lower.ends_with(".m3u8") || lower.contains(".m3u8") {
                Self::HLS
            } else if lower.ends_with(".mpd") || lower.contains(".mpd") {
                Self::DASH
            } else {
                Self::HTTP
            }
        } else if lower.starts_with("file://") || !lower.contains("://") {
            Self::File
        } else {
            Self::Unknown
        }
    }
}

impl std::fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolType::HLS => write!(f, "HLS"),
            ProtocolType::DASH => write!(f, "DASH"),
            ProtocolType::RTSP => write!(f, "RTSP"),
            ProtocolType::WebRTC => write!(f, "WebRTC"),
            ProtocolType::HTTP => write!(f, "HTTP"),
            ProtocolType::File => write!(f, "File"),
            ProtocolType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Common stream information
#[derive(Debug, Clone)]
pub struct StreamInfo {
    /// Protocol type
    pub protocol: ProtocolType,
    /// Stream duration (if known)
    pub duration: Option<Duration>,
    /// Is live stream
    pub is_live: bool,
    /// Available bitrates
    pub bitrates: Vec<u64>,
    /// Available tracks
    pub tracks: Vec<TrackInfo>,
}

/// Track information
#[derive(Debug, Clone)]
pub struct TrackInfo {
    /// Track type
    pub track_type: TrackType,
    /// Track ID
    pub id: String,
    /// Language
    pub lang: Option<String>,
    /// Label
    pub label: Option<String>,
    /// Codec
    pub codec: Option<String>,
    /// Bitrate
    pub bitrate: Option<u64>,
    /// Width (for video)
    pub width: Option<u32>,
    /// Height (for video)
    pub height: Option<u32>,
    /// Frame rate (for video)
    pub frame_rate: Option<f64>,
    /// Sample rate (for audio)
    pub sample_rate: Option<u32>,
    /// Channels (for audio)
    pub channels: Option<u32>,
}

/// Track type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
    Data,
}

impl std::fmt::Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackType::Video => write!(f, "Video"),
            TrackType::Audio => write!(f, "Audio"),
            TrackType::Subtitle => write!(f, "Subtitle"),
            TrackType::Data => write!(f, "Data"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detection() {
        assert_eq!(ProtocolType::from_url("https://example.com/stream.m3u8"), ProtocolType::HLS);
        assert_eq!(ProtocolType::from_url("https://example.com/stream.mpd"), ProtocolType::DASH);
        assert_eq!(ProtocolType::from_url("rtsp://example.com/stream"), ProtocolType::RTSP);
        assert_eq!(ProtocolType::from_url("webrtc://example.com/stream"), ProtocolType::WebRTC);
        assert_eq!(ProtocolType::from_url("https://example.com/video.mp4"), ProtocolType::HTTP);
        assert_eq!(ProtocolType::from_url("file:///path/to/video.mp4"), ProtocolType::File);
    }
}