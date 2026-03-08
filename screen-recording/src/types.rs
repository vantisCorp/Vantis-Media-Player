//! Core types for screen recording module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Unique identifier for a recording session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordingId(pub Uuid);

impl RecordingId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RecordingId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a capture source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(pub Uuid);

impl SourceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Recording session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSession {
    pub id: RecordingId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: RecordingStatus,
    pub config: RecordingConfig,
    pub stats: RecordingStats,
    pub output_path: Option<String>,
    pub metadata: RecordingMetadata,
}

impl RecordingSession {
    pub fn new(name: impl Into<String>, config: RecordingConfig) -> Self {
        Self {
            id: RecordingId::new(),
            name: name.into(),
            created_at: Utc::now(),
            started_at: None,
            ended_at: None,
            status: RecordingStatus::Idle,
            config,
            stats: RecordingStats::default(),
            output_path: None,
            metadata: RecordingMetadata::default(),
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.ended_at) {
            (Some(start), Some(end)) => Some(end - start),
            (Some(start), None) => Some(Utc::now() - start),
            _ => None,
        }
    }
}

/// Recording status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    Idle,
    Preparing,
    Recording,
    Paused,
    Stopping,
    Processing,
    Completed,
    Failed,
}

/// Recording configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfig {
    /// Video capture settings
    pub video: VideoCaptureConfig,
    /// Audio capture settings
    pub audio: AudioCaptureConfig,
    /// Output settings
    pub output: OutputConfig,
    /// Overlay settings
    pub overlays: Vec<OverlayConfig>,
    /// Hotkey settings
    pub hotkeys: HotkeyConfig,
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            video: VideoCaptureConfig::default(),
            audio: AudioCaptureConfig::default(),
            output: OutputConfig::default(),
            overlays: Vec::new(),
            hotkeys: HotkeyConfig::default(),
        }
    }
}

/// Video capture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCaptureConfig {
    /// Capture source
    pub source: CaptureSource,
    /// Frame rate
    pub frame_rate: u32,
    /// Resolution (None = native)
    pub resolution: Option<Resolution>,
    /// Scale mode
    pub scale_mode: ScaleMode,
    /// Crop region (None = no crop)
    pub crop: Option<CropRegion>,
    /// Cursor capture
    pub capture_cursor: bool,
    /// Cursor effects
    pub cursor_effects: CursorEffects,
    /// Hardware encoding
    pub hardware_encoding: bool,
    /// Codec
    pub codec: VideoCodec,
    /// Bitrate mode
    pub bitrate_mode: BitrateMode,
    /// Quality preset
    pub quality: QualityPreset,
}

impl Default for VideoCaptureConfig {
    fn default() -> Self {
        Self {
            source: CaptureSource::Screen { screen_index: 0 },
            frame_rate: 30,
            resolution: None,
            scale_mode: ScaleMode::Fit,
            crop: None,
            capture_cursor: true,
            cursor_effects: CursorEffects::default(),
            hardware_encoding: true,
            codec: VideoCodec::H264,
            bitrate_mode: BitrateMode::Quality(QualityPreset::High),
            quality: QualityPreset::High,
        }
    }
}

/// Capture source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureSource {
    /// Entire screen
    Screen { screen_index: usize },
    /// Specific window
    Window { window_id: u64, window_name: String },
    /// Region of screen
    Region { x: i32, y: i32, width: u32, height: u32, screen_index: usize },
    /// Application capture
    Application { process_id: u32, process_name: String },
    /// Game capture
    Game { process_id: u32, process_name: String },
    /// Webcam
    Webcam { device_index: usize },
}

/// Resolution
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn hd() -> Self { Self { width: 1280, height: 720 } }
    pub fn full_hd() -> Self { Self { width: 1920, height: 1080 } }
    pub fn ultra_hd() -> Self { Self { width: 3840, height: 2160 } }
}

/// Scale mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleMode {
    Fit,
    Fill,
    Stretch,
    None,
}

/// Crop region
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CropRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl CropRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

/// Cursor effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorEffects {
    /// Show cursor
    pub show_cursor: bool,
    /// Highlight on click
    pub highlight_clicks: bool,
    /// Click effect color
    pub click_color: ClickEffectColor,
    /// Click effect radius
    pub click_radius: u32,
    /// Show click animations
    pub click_animation: bool,
    /// Mouse trail
    pub mouse_trail: bool,
    /// Trail length
    pub trail_length: u32,
}

impl Default for CursorEffects {
    fn default() -> Self {
        Self {
            show_cursor: true,
            highlight_clicks: true,
            click_color: ClickEffectColor::default(),
            click_radius: 30,
            click_animation: true,
            mouse_trail: false,
            trail_length: 10,
        }
    }
}

/// Click effect colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClickEffectColor {
    LeftRight,  // Left = green, Right = red
    Custom { left: Color, right: Color },
    Single(Color),
}

impl Default for ClickEffectColor {
    fn default() -> Self {
        Self::LeftRight
    }
}

/// RGBA color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn red() -> Self { Self::rgb(255, 0, 0) }
    pub fn green() -> Self { Self::rgb(0, 255, 0) }
    pub fn blue() -> Self { Self::rgb(0, 0, 255) }
    pub fn white() -> Self { Self::rgb(255, 255, 255) }
    pub fn transparent() -> Self { Self::new(0, 0, 0, 0) }
}

/// Video codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    Vp8,
    Vp9,
    Av1,
}

/// Bitrate mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BitrateMode {
    /// Constant bitrate
    CBR { bitrate: u64 },
    /// Variable bitrate
    VBR { target: u64, max: u64 },
    /// Quality-based
    Quality(QualityPreset),
}

/// Quality preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityPreset {
    Low,
    Medium,
    High,
    Ultra,
    Lossless,
}

/// Audio capture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCaptureConfig {
    /// Enable audio recording
    pub enabled: bool,
    /// Audio sources
    pub sources: Vec<AudioSourceConfig>,
    /// Sample rate
    pub sample_rate: u32,
    /// Channels
    pub channels: u16,
    /// Audio codec
    pub codec: AudioCodec,
    /// Bitrate
    pub bitrate: u32,
}

impl Default for AudioCaptureConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sources: vec![AudioSourceConfig::SystemAudio],
            sample_rate: 48000,
            channels: 2,
            codec: AudioCodec::Aac,
            bitrate: 192,
        }
    }
}

/// Audio source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioSourceConfig {
    /// System audio output
    SystemAudio,
    /// Microphone input
    Microphone { device_index: usize, name: String },
    /// Application audio
    ApplicationAudio { process_id: u32, process_name: String },
    /// Virtual audio device
    VirtualDevice { name: String },
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    Aac,
    Opus,
    Mp3,
    Flac,
    Pcm,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output format
    pub format: OutputFormat,
    /// Output path pattern
    pub path_pattern: String,
    /// File naming pattern
    pub naming: NamingPattern,
    /// Split recording
    pub split: Option<SplitConfig>,
    /// Auto-stop conditions
    pub auto_stop: Option<AutoStopConfig>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Mp4,
            path_pattern: "~/Videos/Recordings".to_string(),
            naming: NamingPattern::default(),
            split: None,
            auto_stop: None,
        }
    }
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Mp4,
    Mkv,
    Mov,
    Webm,
    Gif,
}

/// File naming pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingPattern {
    pub prefix: String,
    pub include_date: bool,
    pub include_time: bool,
    pub include_counter: bool,
    pub custom_pattern: Option<String>,
}

impl Default for NamingPattern {
    fn default() -> Self {
        Self {
            prefix: "Recording".to_string(),
            include_date: true,
            include_time: true,
            include_counter: true,
            custom_pattern: None,
        }
    }
}

/// Split recording configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Split by duration (seconds)
    pub duration: Option<u64>,
    /// Split by file size (MB)
    pub file_size: Option<u64>,
    /// Split on keyframe
    pub on_keyframe: bool,
}

/// Auto-stop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoStopConfig {
    /// Stop after duration (seconds)
    pub duration: Option<u64>,
    /// Stop when disk space below (MB)
    pub min_disk_space: Option<u64>,
    /// Stop when process ends
    pub on_process_exit: Option<u32>,
}

/// Overlay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub id: Uuid,
    pub enabled: bool,
    pub overlay_type: OverlayType,
    pub position: OverlayPosition,
    pub opacity: f32,
}

impl OverlayConfig {
    pub fn new(overlay_type: OverlayType) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: true,
            overlay_type,
            position: OverlayPosition::TopLeft { x: 10, y: 10 },
            opacity: 1.0,
        }
    }
}

/// Overlay types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverlayType {
    /// Text overlay
    Text {
        text: String,
        font: FontConfig,
    },
    /// Image overlay (watermark, logo)
    Image {
        path: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    /// Webcam overlay
    Webcam {
        device_index: usize,
        width: u32,
        height: u32,
        shape: WebcamShape,
    },
    /// Timer/counter
    Timer {
        format: TimerFormat,
        font: FontConfig,
    },
    /// Date/time
    DateTime {
        format: String,
        font: FontConfig,
    },
    /// Logo watermark
    Watermark {
        path: String,
        size_percent: f32,
    },
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: u32,
    pub color: Color,
    pub background: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub shadow: Option<ShadowConfig>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "Arial".to_string(),
            size: 24,
            color: Color::white(),
            background: None,
            bold: false,
            italic: false,
            shadow: Some(ShadowConfig::default()),
        }
    }
}

/// Shadow configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub offset_x: i32,
    pub offset_y: i32,
    pub blur: u32,
    pub color: Color,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            offset_x: 2,
            offset_y: 2,
            blur: 2,
            color: Color::new(0, 0, 0, 128),
        }
    }
}

/// Overlay position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlayPosition {
    TopLeft { x: i32, y: i32 },
    TopCenter { y: i32 },
    TopRight { x: i32, y: i32 },
    CenterLeft { x: i32 },
    Center,
    CenterRight { x: i32 },
    BottomLeft { x: i32, y: i32 },
    BottomCenter { y: i32 },
    BottomRight { x: i32, y: i32 },
    Custom { x: i32, y: i32 },
}

/// Webcam shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebcamShape {
    Rectangle,
    Circle,
    RoundedCorners { radius: u32 },
}

/// Timer format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerFormat {
    HhMmSs,
    MmSs,
    Ss,
    Custom(String),
}

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Start recording
    pub start: Option<KeyCombo>,
    /// Stop recording
    pub stop: Option<KeyCombo>,
    /// Pause/resume
    pub pause: Option<KeyCombo>,
    /// Take screenshot
    pub screenshot: Option<KeyCombo>,
    /// Toggle webcam
    pub toggle_webcam: Option<KeyCombo>,
    /// Toggle microphone
    pub toggle_microphone: Option<KeyCombo>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            start: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::R])),
            stop: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::S])),
            pause: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::P])),
            screenshot: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::T])),
            toggle_webcam: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::W])),
            toggle_microphone: Some(KeyCombo::new(&[KeyCode::Control, KeyCode::Shift, KeyCode::M])),
        }
    }
}

/// Key combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCombo {
    pub keys: Vec<KeyCode>,
}

impl KeyCombo {
    pub fn new(keys: &[KeyCode]) -> Self {
        Self { keys: keys.to_vec() }
    }
}

/// Key codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyCode {
    // Modifiers
    Control,
    Alt,
    Shift,
    Super,
    
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Special keys
    Escape,
    Space,
    Return,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    
    // Arrow keys
    Up, Down, Left, Right,
}

/// Recording statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordingStats {
    /// Frames captured
    pub frames_captured: u64,
    /// Frames dropped
    pub frames_dropped: u64,
    /// Current frame rate
    pub current_fps: f64,
    /// Average frame rate
    pub average_fps: f64,
    /// Current bitrate
    pub current_bitrate: u64,
    /// File size in bytes
    pub file_size: u64,
    /// Disk write speed
    pub disk_write_speed: u64,
    /// CPU usage percent
    pub cpu_usage: f32,
    /// Memory usage in MB
    pub memory_usage: u64,
    /// GPU usage percent
    pub gpu_usage: Option<f32>,
}

impl RecordingStats {
    pub fn drop_rate(&self) -> f64 {
        let total = self.frames_captured + self.frames_dropped;
        if total == 0 { return 0.0; }
        (self.frames_dropped as f64 / total as f64) * 100.0
    }
}

/// Recording metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingMetadata {
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub custom: HashMap<String, String>,
}

/// Available capture device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureDevice {
    pub id: SourceId,
    pub name: String,
    pub device_type: CaptureDeviceType,
    pub is_default: bool,
    pub resolution: Option<Resolution>,
    pub frame_rates: Vec<u32>,
}

/// Capture device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureDeviceType {
    Screen,
    Window,
    Webcam,
    Microphone,
    VirtualAudio,
}

/// Recording event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingEvent {
    Started { recording_id: RecordingId },
    Stopped { recording_id: RecordingId, output_path: String },
    Paused { recording_id: RecordingId },
    Resumed { recording_id: RecordingId },
    Screenshot { recording_id: RecordingId, path: String },
    Error { recording_id: RecordingId, error: String },
    Stats { recording_id: RecordingId, stats: RecordingStats },
    Split { recording_id: RecordingId, part: u32, path: String },
}