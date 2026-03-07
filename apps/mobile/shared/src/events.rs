//! Event system for mobile apps
//!
//! Events are serialized to JSON and passed to platform code via callback.

use serde::{Deserialize, Serialize};
use std::os::raw::c_char;

/// Event callback type
pub type EventCallback = extern "C" fn(*const c_char);

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    /// Playback state changed
    StateChanged {
        state: PlaybackState,
        previous: PlaybackState,
    },
    
    /// Position updated
    PositionChanged {
        position: f64,
        duration: f64,
    },
    
    /// Volume changed
    VolumeChanged {
        volume: f32,
        muted: bool,
    },
    
    /// Media loaded
    MediaLoaded {
        title: Option<String>,
        duration: f64,
        video_track: Option<TrackInfo>,
        audio_tracks: Vec<TrackInfo>,
        subtitle_tracks: Vec<TrackInfo>,
    },
    
    /// Playback error
    Error {
        code: String,
        message: String,
    },
    
    /// Buffering state
    Buffering {
        progress: f32,
    },
    
    /// Sync status changed
    SyncStatusChanged {
        status: SyncStatus,
    },
    
    /// Subtitle update
    SubtitleUpdate {
        text: String,
        start_ms: u64,
        end_ms: u64,
    },
}

/// Playback state for events
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlaybackState {
    Idle,
    Loading,
    Playing,
    Paused,
    Stopped,
    Buffering,
    Error,
}

/// Track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: String,
    pub title: Option<String>,
    pub language: Option<String>,
    pub is_default: bool,
}

/// Sync status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success,
    Error,
}

impl Event {
    /// Create a state changed event
    pub fn state_changed(state: PlaybackState, previous: PlaybackState) -> Self {
        Event::StateChanged { state, previous }
    }
    
    /// Create a position changed event
    pub fn position_changed(position: f64, duration: f64) -> Self {
        Event::PositionChanged { position, duration }
    }
    
    /// Create a volume changed event
    pub fn volume_changed(volume: f32, muted: bool) -> Self {
        Event::VolumeChanged { volume, muted }
    }
    
    /// Create an error event
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Event::Error {
            code: code.into(),
            message: message.into(),
        }
    }
    
    /// Create a buffering event
    pub fn buffering(progress: f32) -> Self {
        Event::Buffering { progress }
    }
    
    /// Create a subtitle update event
    pub fn subtitle(text: impl Into<String>, start_ms: u64, end_ms: u64) -> Self {
        Event::SubtitleUpdate {
            text: text.into(),
            start_ms,
            end_ms,
        }
    }
}