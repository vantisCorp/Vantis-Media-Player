//! Voice command types for media control

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Voice command parsed from natural language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    /// Unique command ID
    pub id: String,
    
    /// Type of command
    pub command_type: CommandType,
    
    /// Original utterance text
    pub utterance: String,
    
    /// Language/locale of the command
    pub locale: String,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Extracted slots/parameters
    pub slots: HashMap<String, SlotValue>,
    
    /// User ID making the request
    pub user_id: String,
    
    /// Device ID making the request
    pub device_id: Option<String>,
    
    /// Session ID for multi-turn conversations
    pub session_id: Option<String>,
    
    /// Timestamp of the command
    pub timestamp: i64,
}

/// Types of voice commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandType {
    /// Media playback control
    Media(MediaCommand),
    
    /// Library management
    Library(LibraryCommand),
    
    /// System control
    System(SystemCommand),
    
    /// Information query
    Query(QueryCommand),
    
    /// Custom skill command
    Custom(String),
}

/// Media playback commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaCommand {
    /// Play media
    Play {
        query: Option<String>,
        media_id: Option<String>,
        playlist: Option<String>,
    },
    
    /// Pause playback
    Pause,
    
    /// Resume playback
    Resume,
    
    /// Stop playback
    Stop,
    
    /// Next track
    Next,
    
    /// Previous track
    Previous,
    
    /// Seek to position
    Seek {
        position_ms: Option<u64>,
        relative_seconds: Option<i32>,
    },
    
    /// Set volume
    SetVolume {
        level: Option<u8>,
        relative: Option<i8>,
    },
    
    /// Mute/unmute
    Mute,
    Unmute,
    
    /// Toggle mute
    ToggleMute,
    
    /// Shuffle
    Shuffle {
        enable: bool,
    },
    
    /// Repeat mode
    Repeat {
        mode: RepeatMode,
    },
    
    /// Fast forward
    FastForward {
        seconds: Option<u32>,
    },
    
    /// Rewind
    Rewind {
        seconds: Option<u32>,
    },
    
    /// Add to queue
    AddToQueue {
        query: Option<String>,
        position: QueuePosition,
    },
    
    /// Clear queue
    ClearQueue,
}

/// Repeat modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RepeatMode {
    Off,
    One,
    All,
}

/// Queue positions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum QueuePosition {
    Next,
    Last,
}

/// Library management commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LibraryCommand {
    /// Add to favorites
    AddToFavorites {
        media_id: String,
    },
    
    /// Remove from favorites
    RemoveFromFavorites {
        media_id: String,
    },
    
    /// Create playlist
    CreatePlaylist {
        name: String,
    },
    
    /// Delete playlist
    DeletePlaylist {
        name: String,
    },
    
    /// Add to playlist
    AddToPlaylist {
        playlist_name: String,
        query: Option<String>,
    },
    
    /// Remove from playlist
    RemoveFromPlaylist {
        playlist_name: String,
        item_index: Option<u32>,
    },
    
    /// Share playlist
    SharePlaylist {
        playlist_name: String,
        with_user: String,
    },
}

/// System control commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemCommand {
    /// Set audio output device
    SetOutputDevice {
        device_name: String,
    },
    
    /// Toggle subtitles
    ToggleSubtitles {
        language: Option<String>,
    },
    
    /// Set video quality
    SetVideoQuality {
        quality: VideoQuality,
    },
    
    /// Toggle fullscreen
    ToggleFullscreen,
    
    /// Set sleep timer
    SetSleepTimer {
        minutes: u32,
    },
    
    /// Cancel sleep timer
    CancelSleepTimer,
}

/// Video quality levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VideoQuality {
    Auto,
    Low,
    Medium,
    High,
    Ultra4K,
}

/// Information query commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QueryCommand {
    /// Get current playback info
    GetCurrentPlayback,
    
    /// Get queue info
    GetQueue,
    
    /// Search media
    SearchMedia {
        query: String,
        filter: Option<SearchFilter>,
    },
    
    /// Get playlist info
    GetPlaylistInfo {
        name: String,
    },
    
    /// Get artist info
    GetArtistInfo {
        name: String,
    },
    
    /// Get recommendations
    GetRecommendations {
        context: Option<String>,
    },
    
    /// What's playing
    WhatsPlaying,
    
    /// Help
    Help {
        topic: Option<String>,
    },
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SearchFilter {
    All,
    Music,
    Video,
    Playlist,
    Artist,
    Album,
}

/// Slot value from parsed utterance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotValue {
    /// Slot name
    pub name: String,
    
    /// Raw value
    pub value: String,
    
    /// Resolved entity ID (if applicable)
    pub resolved_id: Option<String>,
    
    /// Confidence score
    pub confidence: f32,
}

/// Playback state for media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// Current media item
    pub media_id: Option<String>,
    
    /// Media title
    pub title: Option<String>,
    
    /// Artist/creator
    pub artist: Option<String>,
    
    /// Album/collection
    pub album: Option<String>,
    
    /// Duration in milliseconds
    pub duration_ms: u64,
    
    /// Current position in milliseconds
    pub position_ms: u64,
    
    /// Is playing
    pub is_playing: bool,
    
    /// Current volume (0-100)
    pub volume: u8,
    
    /// Is muted
    pub is_muted: bool,
    
    /// Shuffle enabled
    pub shuffle_enabled: bool,
    
    /// Repeat mode
    pub repeat_mode: RepeatMode,
    
    /// Queue length
    pub queue_length: u32,
    
    /// Queue position
    pub queue_position: u32,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            media_id: None,
            title: None,
            artist: None,
            album: None,
            duration_ms: 0,
            position_ms: 0,
            is_playing: false,
            volume: 50,
            is_muted: false,
            shuffle_enabled: false,
            repeat_mode: RepeatMode::Off,
            queue_length: 0,
            queue_position: 0,
        }
    }
}

impl VoiceCommand {
    /// Create a new voice command
    pub fn new(
        command_type: CommandType,
        utterance: String,
        locale: String,
        user_id: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            command_type,
            utterance,
            locale,
            confidence: 1.0,
            slots: HashMap::new(),
            user_id,
            device_id: None,
            session_id: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// Add a slot to the command
    pub fn with_slot(mut self, name: String, value: String, confidence: f32) -> Self {
        self.slots.insert(name.clone(), SlotValue {
            name,
            value,
            resolved_id: None,
            confidence,
        });
        self
    }
    
    /// Set device ID
    pub fn with_device(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }
    
    /// Set session ID
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

impl std::fmt::Display for MediaCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Play { query, media_id, playlist } => {
                if let Some(q) = query {
                    write!(f, "play '{}'", q)
                } else if let Some(id) = media_id {
                    write!(f, "play media {}", id)
                } else if let Some(p) = playlist {
                    write!(f, "play playlist '{}'", p)
                } else {
                    write!(f, "play")
                }
            }
            Self::Pause => write!(f, "pause"),
            Self::Resume => write!(f, "resume"),
            Self::Stop => write!(f, "stop"),
            Self::Next => write!(f, "next"),
            Self::Previous => write!(f, "previous"),
            Self::Seek { position_ms, relative_seconds } => {
                if let Some(pos) = position_ms {
                    write!(f, "seek to {}ms", pos)
                } else if let Some(rel) = relative_seconds {
                    write!(f, "seek {} seconds", rel)
                } else {
                    write!(f, "seek")
                }
            }
            Self::SetVolume { level, relative } => {
                if let Some(l) = level {
                    write!(f, "set volume to {}%", l)
                } else if let Some(r) = relative {
                    write!(f, "volume {}{}", if *r > 0 { "+" } else { "" }, r)
                } else {
                    write!(f, "set volume")
                }
            }
            Self::Mute => write!(f, "mute"),
            Self::Unmute => write!(f, "unmute"),
            Self::ToggleMute => write!(f, "toggle mute"),
            Self::Shuffle { enable } => write!(f, "shuffle {}", if *enable { "on" } else { "off" }),
            Self::Repeat { mode } => write!(f, "repeat {:?}", mode),
            Self::FastForward { seconds } => {
                if let Some(s) = seconds {
                    write!(f, "fast forward {} seconds", s)
                } else {
                    write!(f, "fast forward")
                }
            }
            Self::Rewind { seconds } => {
                if let Some(s) = seconds {
                    write!(f, "rewind {} seconds", s)
                } else {
                    write!(f, "rewind")
                }
            }
            Self::AddToQueue { query, position } => {
                if let Some(q) = query {
                    write!(f, "add '{}' to queue {:?}", q, position)
                } else {
                    write!(f, "add to queue")
                }
            }
            Self::ClearQueue => write!(f, "clear queue"),
        }
    }
}