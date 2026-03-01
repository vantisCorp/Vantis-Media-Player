//! Configuration Management
//! 
/// Handles player settings, preferences, and system configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use num_cpus;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Audio settings
    pub audio: AudioConfig,
    
    /// Video settings
    pub video: VideoConfig,
    
    /// Subtitle settings
    pub subtitles: SubtitleConfig,
    
    /// UI settings
    pub ui: UIConfig,
    
    /// Advanced settings
    pub advanced: AdvancedConfig,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Use exclusive mode (bit-perfect)
    pub exclusive_mode: bool,
    
    /// Output device name
    pub output_device: Option<String>,
    
    /// Default sample rate
    pub sample_rate: u32,
    
    /// Default channel count
    pub channels: u32,
    
    /// Normalize loudness
    pub loudness_normalization: bool,
    
    /// Target loudness in LUFS
    pub target_loudness: f32,
}

/// Video configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    
    /// Use AI upscaling
    pub ai_upscaling: bool,
    
    /// Target resolution for upscaling
    pub target_resolution: Resolution,
    
    /// Enable HDR tone mapping
    pub hdr_tone_mapping: bool,
    
    /// Enable motion interpolation
    pub motion_interpolation: bool,
    
    /// Target frame rate for interpolation
    pub target_fps: u32,
    
    /// Renderer backend
    pub renderer: RendererBackend,
}

/// Subtitle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleConfig {
    /// Default subtitle language
    pub default_language: String,
    
    /// Enable subtitle aggregation
    pub enable_aggregation: bool,
    
    /// Auto-download subtitles
    pub auto_download: bool,
    
    /// Subtitle sources
    pub sources: Vec<String>,
    
    /// Enable AI subtitle sync
    pub ai_sync: bool,
    
    /// Subtitle font size
    pub font_size: u32,
    
    /// Subtitle position
    pub position: SubtitlePosition,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// Theme
    pub theme: Theme,
    
    /// Show system borders
    pub show_borders: bool,
    
    /// Enable animations
    pub animations: bool,
    
    /// Enable eye tracking
    pub eye_tracking: bool,
    
    /// Enable pie menus
    pub pie_menus: bool,
    
    /// Omnibar shortcut
    pub omnibar_shortcut: String,
}

/// Advanced configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    /// Enable WASM sandbox
    pub wasm_sandbox: bool,
    
    /// Enable IPC guard
    pub ipc_guard: bool,
    
    /// Buffer size in MB
    pub buffer_size_mb: usize,
    
    /// Maximum threads
    pub max_threads: usize,
    
    /// Log level
    pub log_level: String,
    
    /// Plugin directories
    pub plugin_directories: Vec<PathBuf>,
}

/// Resolution
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Resolution {
    Auto,
    HD(1920, 1080),
    FullHD(1920, 1080),
    UHD(3840, 2160),
    Custom(u32, u32),
}

/// Renderer backend
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RendererBackend {
    Auto,
    Vulkan,
    DirectX12,
    Metal,
    OpenGL,
}

/// Theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    System,
    Custom(String),
}

/// Subtitle position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SubtitlePosition {
    Bottom,
    Top,
    Middle,
    Custom(u32),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            video: VideoConfig::default(),
            subtitles: SubtitleConfig::default(),
            ui: UIConfig::default(),
            advanced: AdvancedConfig::default(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            exclusive_mode: true,
            output_device: None,
            sample_rate: 48000,
            channels: 2,
            loudness_normalization: true,
            target_loudness: -16.0,
        }
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            ai_upscaling: true,
            target_resolution: Resolution::UHD(3840, 2160),
            hdr_tone_mapping: true,
            motion_interpolation: true,
            target_fps: 60,
            renderer: RendererBackend::Auto,
        }
    }
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            default_language: "pl".to_string(),
            enable_aggregation: true,
            auto_download: true,
            sources: vec![
                "napprojekt".to_string(),
                "napisy24".to_string(),
                "opensubtitles".to_string(),
            ],
            ai_sync: true,
            font_size: 28,
            position: SubtitlePosition::Bottom,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            show_borders: false,
            animations: true,
            eye_tracking: true,
            pie_menus: true,
            omnibar_shortcut: "Ctrl+K".to_string(),
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            wasm_sandbox: true,
            ipc_guard: true,
            buffer_size_mb: 512,
            max_threads: num_cpus::get(),
            log_level: "INFO".to_string(),
            plugin_directories: vec![
                PathBuf::from("./plugins"),
                PathBuf::from("~/.vantis/plugins"),
            ],
        }
    }
}