//! Sync data models
//! 
//! Data structures for synchronized content.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Synchronized data wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    /// Data version for conflict detection
    pub version: u64,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    
    /// Device that made the last change
    pub modified_by: Uuid,
    
    /// Data type
    pub data_type: String,
    
    /// Actual data payload
    pub payload: serde_json::Value,
    
    /// Checksum for integrity
    pub checksum: String,
}

/// Playlist model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Unique playlist ID
    pub id: Uuid,
    
    /// Playlist name
    pub name: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Playlist items
    pub items: Vec<PlaylistItem>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    
    /// Cover image URL
    pub cover_url: Option<String>,
    
    /// Is public
    pub is_public: bool,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Total duration in seconds
    pub total_duration: u64,
    
    /// Item count
    pub item_count: u64,
}

/// Playlist item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    /// Item ID
    pub id: Uuid,
    
    /// Media path or URL
    pub media_path: String,
    
    /// Media title
    pub title: String,
    
    /// Artist/creator
    pub artist: Option<String>,
    
    /// Duration in seconds
    pub duration: u64,
    
    /// Thumbnail URL
    pub thumbnail_url: Option<String>,
    
    /// Added timestamp
    pub added_at: DateTime<Utc>,
    
    /// Play count
    pub play_count: u64,
    
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Settings model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Settings version
    pub version: u32,
    
    /// Audio settings
    pub audio: AudioSettings,
    
    /// Video settings
    pub video: VideoSettings,
    
    /// UI settings
    pub ui: UISettings,
    
    /// Playback settings
    pub playback: PlaybackSettings,
    
    /// Subtitle settings
    pub subtitles: SubtitleSettings,
    
    /// Network settings
    pub network: NetworkSettings,
    
    /// Plugin settings
    pub plugins: PluginSettings,
    
    /// Custom settings
    pub custom: HashMap<String, serde_json::Value>,
}

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Volume level (0.0-1.0)
    pub volume: f32,
    
    /// Is muted
    pub muted: bool,
    
    /// Audio output device
    pub output_device: Option<String>,
    
    /// Audio language preference
    pub language: Option<String>,
    
    /// Audio delay in milliseconds
    pub delay_ms: i32,
    
    /// Equalizer preset
    pub equalizer_preset: Option<String>,
    
    /// Custom equalizer bands
    pub equalizer_bands: Option<Vec<f32>>,
    
    /// Spatial audio enabled
    pub spatial_audio: bool,
    
    /// Normalization enabled
    pub normalization: bool,
}

/// Video settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    /// Brightness (0.0-2.0)
    pub brightness: f32,
    
    /// Contrast (0.0-2.0)
    pub contrast: f32,
    
    /// Saturation (0.0-2.0)
    pub saturation: f32,
    
    /// Gamma (0.0-2.0)
    pub gamma: f32,
    
    /// Hue (-180 to 180)
    pub hue: f32,
    
    /// Deinterlacing mode
    pub deinterlace: DeinterlaceMode,
    
    /// Video scaling method
    pub scaling: ScalingMethod,
    
    /// Hardware acceleration enabled
    pub hw_accel: bool,
    
    /// Preferred video quality
    pub quality: VideoQuality,
    
    /// Aspect ratio
    pub aspect_ratio: Option<String>,
}

/// Deinterlace modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeinterlaceMode {
    Auto,
    Disabled,
    Bob,
    Yadif,
    Bwdif,
}

/// Scaling methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingMethod {
    Bilinear,
    Bicubic,
    Lanczos,
    Spline,
    Neighbor,
}

/// Video quality presets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoQuality {
    Auto,
    SD,
    HD,
    FullHD,
    UHD4K,
    UHD8K,
    Original,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// Theme name
    pub theme: String,
    
    /// Language code
    pub language: String,
    
    /// Font size scale
    pub font_scale: f32,
    
    /// Show tooltips
    pub show_tooltips: bool,
    
    /// Animation duration scale
    pub animation_scale: f32,
    
    /// Window state
    pub window_state: WindowState,
    
    /// Sidebar visible
    pub sidebar_visible: bool,
    
    /// Mini player enabled
    pub mini_player: bool,
}

/// Window state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window X position
    pub x: i32,
    
    /// Window Y position
    pub y: i32,
    
    /// Window width
    pub width: u32,
    
    /// Window height
    pub height: u32,
    
    /// Is maximized
    pub maximized: bool,
    
    /// Is fullscreen
    pub fullscreen: bool,
}

/// Playback settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackSettings {
    /// Auto-play next
    pub auto_play_next: bool,
    
    /// Loop mode
    pub loop_mode: LoopMode,
    
    /// Shuffle enabled
    pub shuffle: bool,
    
    /// Default speed
    pub default_speed: f32,
    
    /// Remember position
    pub remember_position: bool,
    
    /// Skip intros
    pub skip_intros: bool,
    
    /// Skip credits
    pub skip_credits: bool,
}

/// Loop modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopMode {
    None,
    One,
    All,
}

/// Subtitle settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleSettings {
    /// Enabled
    pub enabled: bool,
    
    /// Preferred language
    pub language: Option<String>,
    
    /// Font family
    pub font_family: String,
    
    /// Font size
    pub font_size: u32,
    
    /// Font color (hex)
    pub font_color: String,
    
    /// Background color (hex)
    pub background_color: String,
    
    /// Outline color (hex)
    pub outline_color: String,
    
    /// Outline width
    pub outline_width: u32,
    
    /// Position offset
    pub position_offset: i32,
}

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Proxy URL
    pub proxy: Option<String>,
    
    /// Bandwidth limit (bytes/sec, 0 = unlimited)
    pub bandwidth_limit: u64,
    
    /// Buffer size in seconds
    pub buffer_size: u32,
    
    /// Cache size in MB
    pub cache_size: u32,
}

/// Plugin settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettings {
    /// Enabled plugins
    pub enabled: Vec<String>,
    
    /// Plugin configurations
    pub configs: HashMap<String, serde_json::Value>,
}

/// Watch history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistoryEntry {
    /// Entry ID
    pub id: Uuid,
    
    /// Media path or URL
    pub media_path: String,
    
    /// Media title
    pub title: String,
    
    /// Last position in seconds
    pub position: u64,
    
    /// Total duration in seconds
    pub duration: u64,
    
    /// Completion percentage
    pub completion: f32,
    
    /// Last played timestamp
    pub last_played: DateTime<Utc>,
    
    /// Play count
    pub play_count: u32,
    
    /// Rating (1-5)
    pub rating: Option<u32>,
}

/// Bookmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Bookmark ID
    pub id: Uuid,
    
    /// Media path
    pub media_path: String,
    
    /// Position in seconds
    pub position: u64,
    
    /// Bookmark name
    pub name: String,
    
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    
    /// Note
    pub note: Option<String>,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            muted: false,
            output_device: None,
            language: None,
            delay_ms: 0,
            equalizer_preset: None,
            equalizer_bands: None,
            spatial_audio: false,
            normalization: false,
        }
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            gamma: 1.0,
            hue: 0.0,
            deinterlace: DeinterlaceMode::Auto,
            scaling: ScalingMethod::Lanczos,
            hw_accel: true,
            quality: VideoQuality::Auto,
            aspect_ratio: None,
        }
    }
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "en".to_string(),
            font_scale: 1.0,
            show_tooltips: true,
            animation_scale: 1.0,
            window_state: WindowState::default(),
            sidebar_visible: true,
            mini_player: true,
        }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1280,
            height: 720,
            maximized: false,
            fullscreen: false,
        }
    }
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            auto_play_next: true,
            loop_mode: LoopMode::None,
            shuffle: false,
            default_speed: 1.0,
            remember_position: true,
            skip_intros: false,
            skip_credits: false,
        }
    }
}

impl Default for SubtitleSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            language: None,
            font_family: "Arial".to_string(),
            font_size: 24,
            font_color: "#FFFFFF".to_string(),
            background_color: "#000000AA".to_string(),
            outline_color: "#000000".to_string(),
            outline_width: 2,
            position_offset: 0,
        }
    }
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            proxy: None,
            bandwidth_limit: 0,
            buffer_size: 30,
            cache_size: 500,
        }
    }
}

impl Default for PluginSettings {
    fn default() -> Self {
        Self {
            enabled: Vec::new(),
            configs: HashMap::new(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            audio: AudioSettings::default(),
            video: VideoSettings::default(),
            ui: UISettings::default(),
            playback: PlaybackSettings::default(),
            subtitles: SubtitleSettings::default(),
            network: NetworkSettings::default(),
            plugins: PluginSettings::default(),
            custom: HashMap::new(),
        }
    }
}