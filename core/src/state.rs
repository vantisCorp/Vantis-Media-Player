//! Player State Management
//! 
//! Thread-safe state management for playback status,
/// volume, position, and other player properties.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Current player state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// Current playback state
    pub playback_state: PlaybackState,
    
    /// Current position in media (milliseconds)
    pub position_ms: u64,
    
    /// Total duration of media (milliseconds)
    pub duration_ms: u64,
    
    /// Volume level (0.0 - 1.0)
    pub volume: f32,
    
    /// Muted state
    pub muted: bool,
    
    /// Playback speed (0.5x - 2.0x)
    pub playback_speed: f32,
    
    /// Current subtitle track index
    pub subtitle_track: Option<usize>,
    
    /// Current audio track index
    pub audio_track: usize,
    
    /// Current video track index
    pub video_track: usize,
    
    /// Loop mode
    pub loop_mode: LoopMode,
    
    /// Shuffle mode
    pub shuffle: bool,
    
    /// Path to current media
    pub current_media: Option<String>,
}

/// Playback state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Buffering,
    Seeking,
}

impl std::fmt::Display for PlaybackState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaybackState::Playing => write!(f, "▶ Playing"),
            PlaybackState::Paused => write!(f, "⏸ Paused"),
            PlaybackState::Stopped => write!(f, "⏹ Stopped"),
            PlaybackState::Buffering => write!(f, "⏳ Buffering"),
            PlaybackState::Seeking => write!(f, "⏩ Seeking"),
        }
    }
}

/// Loop mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoopMode {
    NoLoop,
    LoopTrack,
    LoopAll,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerState {
    /// Create a new player state
    pub fn new() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            position_ms: 0,
            duration_ms: 0,
            volume: 1.0,
            muted: false,
            playback_speed: 1.0,
            subtitle_track: None,
            audio_track: 0,
            video_track: 0,
            loop_mode: LoopMode::NoLoop,
            shuffle: false,
            current_media: None,
        }
    }
    
    /// Get current position as Duration
    pub fn position_duration(&self) -> Duration {
        Duration::from_millis(self.position_ms)
    }
    
    /// Get total duration as Duration
    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.duration_ms)
    }
    
    /// Get progress as percentage (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        if self.duration_ms == 0 {
            0.0
        } else {
            self.position_ms as f64 / self.duration_ms as f64
        }
    }
    
    /// Set position in seconds
    pub fn set_position_seconds(&mut self, seconds: f64) {
        self.position_ms = (seconds * 1000.0) as u64;
    }
    
    /// Get position in seconds
    pub fn position_seconds(&self) -> f64 {
        self.position_ms as f64 / 1000.0
    }
    
    /// Get duration in seconds
    pub fn duration_seconds(&self) -> f64 {
        self.duration_ms as f64 / 1000.0
    }
    
    /// Set playing state
    pub fn set_playing(&mut self, playing: bool) {
        self.playback_state = if playing {
            PlaybackState::Playing
        } else {
            PlaybackState::Paused
        };
    }
    
    /// Set position in seconds
    pub fn set_position(&mut self, position: f64) {
        self.position_ms = (position * 1000.0) as u64;
    }
    
    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
    
    /// Set muted state
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }
    
    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }
    
    /// Set current media file
    pub fn set_current_file(&mut self, file: String) {
        self.current_media = Some(file);
    }
    
    /// Update state (called periodically)
    pub fn update(&mut self) {
        // Update position based on playback state
        if self.playback_state == PlaybackState::Playing {
            // In a real implementation, this would update based on elapsed time
            // For now, we'll just increment by a small amount
            let increment = (100.0 * self.playback_speed) as u64; // 100ms * speed
            self.position_ms = (self.position_ms + increment).min(self.duration_ms);
            
            // Check if we've reached the end
            if self.position_ms >= self.duration_ms && self.duration_ms > 0 {
                match self.loop_mode {
                    LoopMode::LoopTrack => {
                        self.position_ms = 0;
                    }
                    LoopMode::LoopAll => {
                        // In a real implementation, this would move to the next track
                        self.position_ms = 0;
                    }
                    LoopMode::NoLoop => {
                        self.playback_state = PlaybackState::Stopped;
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📺 Player State:")?;
        writeln!(f, "   Status: {}", self.playback_state)?;
        writeln!(f, "   Position: {:.2}s / {:.2}s", 
            self.position_seconds(), 
            self.duration_seconds())?;
        writeln!(f, "   Volume: {:.0}% (Muted: {})", 
            self.volume * 100.0, 
            self.muted)?;
        writeln!(f, "   Speed: {}x", self.playback_speed)?;
        writeln!(f, "   Loop: {:?}", self.loop_mode)?;
        
        if let Some(ref path) = self.current_media {
            writeln!(f, "   Media: {}", path)?;
        }
        
        Ok(())
    }
}