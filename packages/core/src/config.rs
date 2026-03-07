//! Configuration management for Vantis Media Player

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::{Volume, Result};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub playback: PlaybackConfig,
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub ui: UiConfig,
    pub network: NetworkConfig,
    pub plugins: PluginConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            playback: PlaybackConfig::default(),
            audio: AudioConfig::default(),
            video: VideoConfig::default(),
            ui: UiConfig::default(),
            network: NetworkConfig::default(),
            plugins: PluginConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vantis-player")
    }
    
    pub fn default_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub language: String,
    pub theme: String,
    pub check_updates: bool,
    pub send_analytics: bool,
    pub start_minimized: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            theme: "dark".to_string(),
            check_updates: true,
            send_analytics: false,
            start_minimized: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackConfig {
    pub auto_play: bool,
    pub loop_mode: LoopMode,
    pub shuffle: bool,
    pub remember_position: bool,
    pub default_speed: f32,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            auto_play: true,
            loop_mode: LoopMode::None,
            shuffle: false,
            remember_position: true,
            default_speed: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopMode {
    None,
    Single,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub volume: Volume,
    pub muted: bool,
    pub output_device: Option<String>,
    pub equalizer_preset: Option<String>,
    pub normalize: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            volume: Volume::new(0.8),
            muted: false,
            output_device: None,
            equalizer_preset: None,
            normalize: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub hardware_acceleration: bool,
    pub deinterlace: bool,
    pub aspect_ratio: AspectRatio,
    pub crop: CropMode,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            deinterlace: false,
            aspect_ratio: AspectRatio::Auto,
            crop: CropMode::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatio {
    Auto,
    Ratio4x3,
    Ratio16x9,
    Ratio21x9,
    Custom(f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropMode {
    None,
    Detect,
    Custom(u32, u32, u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub show_toolbar: bool,
    pub show_sidebar: bool,
    pub show_statusbar: bool,
    pub compact_mode: bool,
    pub notifications: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_toolbar: true,
            show_sidebar: true,
            show_statusbar: true,
            compact_mode: false,
            notifications: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub proxy: Option<String>,
    pub cache_size_mb: u32,
    pub buffer_seconds: u32,
    pub user_agent: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            proxy: None,
            cache_size_mb: 500,
            buffer_seconds: 10,
            user_agent: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub auto_update: bool,
    pub trusted_sources_only: bool,
    pub sandbox_mode: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_update: true,
            trusted_sources_only: true,
            sandbox_mode: true,
        }
    }
}