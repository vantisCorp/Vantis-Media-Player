//! Core types for Vantis Media Player

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Unique identifier for media items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaId(pub Uuid);

impl MediaId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MediaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Media file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub id: MediaId,
    pub path: PathBuf,
    pub title: String,
    pub duration: Option<f64>,
    pub format: MediaFormat,
    pub metadata: MediaMetadata,
}

/// Supported media formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaFormat {
    // Video
    Mp4,
    Mkv,
    WebM,
    Avi,
    Mov,
    
    // Audio
    Mp3,
    Flac,
    Wav,
    Ogg,
    Aac,
    
    // Streaming
    Hls,
    Dash,
    Rtsp,
    
    // Unknown
    Unknown,
}

impl MediaFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp4" | "m4v" => MediaFormat::Mp4,
            "mkv" => MediaFormat::Mkv,
            "webm" => MediaFormat::WebM,
            "avi" => MediaFormat::Avi,
            "mov" | "qt" => MediaFormat::Mov,
            "mp3" => MediaFormat::Mp3,
            "flac" => MediaFormat::Flac,
            "wav" | "wave" => MediaFormat::Wav,
            "ogg" | "oga" => MediaFormat::Ogg,
            "aac" | "m4a" => MediaFormat::Aac,
            "m3u8" => MediaFormat::Hls,
            "mpd" => MediaFormat::Dash,
            "rtsp" | "rts" => MediaFormat::Rtsp,
            _ => MediaFormat::Unknown,
        }
    }
    
    pub fn is_video(&self) -> bool {
        matches!(self, 
            MediaFormat::Mp4 | MediaFormat::Mkv | MediaFormat::WebM |
            MediaFormat::Avi | MediaFormat::Mov
        )
    }
    
    pub fn is_audio(&self) -> bool {
        matches!(self,
            MediaFormat::Mp3 | MediaFormat::Flac | MediaFormat::Wav |
            MediaFormat::Ogg | MediaFormat::Aac
        )
    }
    
    pub fn is_streaming(&self) -> bool {
        matches!(self, MediaFormat::Hls | MediaFormat::Dash | MediaFormat::Rtsp)
    }
}

/// Media metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub genre: Option<String>,
    pub comment: Option<String>,
    pub cover_art: Option<Vec<u8>>,
}

/// Playback position
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PlaybackPosition {
    pub current: f64,
    pub total: f64,
}

impl PlaybackPosition {
    pub fn progress(&self) -> f64 {
        if self.total > 0.0 {
            (self.current / self.total).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    
    pub fn remaining(&self) -> f64 {
        (self.total - self.current).max(0.0)
    }
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PlaybackState {
    #[default]
    Stopped,
    Playing,
    Paused,
    Buffering,
    Error,
}

/// Volume level (0.0 to 1.0)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Volume(pub f64);

impl Volume {
    pub fn new(level: f64) -> Self {
        Self(level.clamp(0.0, 1.0))
    }
    
    pub fn muted() -> Self {
        Self(0.0)
    }
    
    pub fn max() -> Self {
        Self(1.0)
    }
    
    pub fn is_muted(&self) -> bool {
        self.0 == 0.0
    }
    
    pub fn as_percent(&self) -> u8 {
        (self.0 * 100.0) as u8
    }
}