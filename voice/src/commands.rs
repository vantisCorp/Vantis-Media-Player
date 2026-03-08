//! Voice commands for media player control

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed voice command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    /// Raw transcript
    pub transcript: String,
    
    /// Parsed intent
    pub intent: CommandIntent,
    
    /// Extracted entities/slots
    pub entities: HashMap<String, EntityValue>,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    
    /// Source provider
    pub provider: VoiceAssistant,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Command intents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandIntent {
    // Playback
    Play,
    Pause,
    Resume,
    Stop,
    Next,
    Previous,
    Rewind,
    FastForward,
    SeekTo,
    
    // Volume
    SetVolume,
    IncreaseVolume,
    DecreaseVolume,
    Mute,
    Unmute,
    
    // Media
    OpenFile,
    OpenUrl,
    Search,
    PlayPlaylist,
    PlayAlbum,
    PlayArtist,
    PlayGenre,
    PlayFavorites,
    
    // Subtitles
    EnableSubtitles,
    DisableSubtitles,
    ChangeSubtitleLanguage,
    
    // Audio
    ChangeAudioTrack,
    SetAudioDelay,
    
    // Video
    SetAspectRatio,
    ToggleFullscreen,
    
    // Navigation
    GoBack,
    GoHome,
    OpenSettings,
    
    // Information
    WhatIsPlaying,
    GetDuration,
    GetPosition,
    WhatCanYouDo,
    
    // System
    Help,
    Cancel,
    Repeat,
}

/// Entity value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityValue {
    String(String),
    Number(f64),
    Duration(Duration),
    Percentage(u8),
    Boolean(bool),
    ListItem(String, usize),
}

/// Duration entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub value: f64,
    pub unit: DurationUnit,
}

/// Duration units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationUnit {
    Seconds,
    Minutes,
    Hours,
    Percent,
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    /// Success status
    pub success: bool,
    
    /// Result message
    pub message: String,
    
    /// Additional data
    pub data: HashMap<String, serde_json::Value>,
    
    /// Whether to speak the response
    pub should_speak: bool,
}

impl CommandResult {
    /// Create a successful result
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: HashMap::new(),
            should_speak: true,
        }
    }
    
    /// Create a failure result
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: HashMap::new(),
            should_speak: true,
        }
    }
    
    /// Add data to result
    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
    
    /// Set whether to speak the response
    pub fn with_speak(mut self, should_speak: bool) -> Self {
        self.should_speak = should_speak;
        self
    }
}

/// Sample commands for training/help
pub const SAMPLE_COMMANDS: &[(&str, CommandIntent)] = &[
    // Playback
    ("play", CommandIntent::Play),
    ("pause", CommandIntent::Pause),
    ("resume", CommandIntent::Resume),
    ("stop", CommandIntent::Stop),
    ("next track", CommandIntent::Next),
    ("previous track", CommandIntent::Previous),
    ("skip", CommandIntent::Next),
    ("go back", CommandIntent::Previous),
    ("rewind", CommandIntent::Rewind),
    ("fast forward", CommandIntent::FastForward),
    ("seek to 5 minutes", CommandIntent::SeekTo),
    ("skip to 30 seconds", CommandIntent::SeekTo),
    
    // Volume
    ("set volume to 50", CommandIntent::SetVolume),
    ("volume up", CommandIntent::IncreaseVolume),
    ("volume down", CommandIntent::DecreaseVolume),
    ("turn up the volume", CommandIntent::IncreaseVolume),
    ("mute", CommandIntent::Mute),
    ("unmute", CommandIntent::Unmute),
    
    // Media
    ("open file", CommandIntent::OpenFile),
    ("play [title]", CommandIntent::Search),
    ("play my playlist [name]", CommandIntent::PlayPlaylist),
    ("play album [name]", CommandIntent::PlayAlbum),
    ("play artist [name]", CommandIntent::PlayArtist),
    ("play some rock music", CommandIntent::PlayGenre),
    ("play my favorites", CommandIntent::PlayFavorites),
    
    // Subtitles
    ("turn on subtitles", CommandIntent::EnableSubtitles),
    ("turn off subtitles", CommandIntent::DisableSubtitles),
    ("change subtitle language to English", CommandIntent::ChangeSubtitleLanguage),
    
    // Information
    ("what is playing", CommandIntent::WhatIsPlaying),
    ("what's playing now", CommandIntent::WhatIsPlaying),
    ("how long is this video", CommandIntent::GetDuration),
    ("what can you do", CommandIntent::WhatCanYouDo),
    ("help", CommandIntent::Help),
];