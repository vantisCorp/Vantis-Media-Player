//! Screen Recording Module for Vantis Media Player
//!
//! This module provides comprehensive screen recording capabilities including:
//! - Multi-source capture (screen, window, region, webcam)
//! - Audio recording (system audio, microphone)
//! - Overlay support (text, images, webcam, timer)
//! - Cursor effects and highlighting
//! - Hotkey management
//! - Split recording
//! - Multiple output formats

pub mod types;
pub mod error;
pub mod recording_service;

// Re-export key types
pub use types::*;
pub use error::{ScreenRecordingError, ScreenRecordingResult};

use std::sync::Arc;
use tokio::sync::broadcast;

use recording_service::{RecordingService, DefaultRecordingService, CaptureDeviceService, DefaultCaptureDeviceService};

/// Configuration for the screen recording module
#[derive(Debug, Clone)]
pub struct ScreenRecordingConfig {
    /// Maximum concurrent recordings
    pub max_concurrent_recordings: usize,
    /// Default output directory
    pub default_output_directory: String,
    /// Default video codec
    pub default_video_codec: VideoCodec,
    /// Default audio codec
    pub default_audio_codec: AudioCodec,
    /// Default frame rate
    pub default_frame_rate: u32,
    /// Enable hardware encoding by default
    pub default_hardware_encoding: bool,
    /// Maximum recording duration (seconds, 0 = unlimited)
    pub max_recording_duration: u64,
    /// Minimum disk space required (MB)
    pub min_disk_space: u64,
    /// Temporary directory for processing
    pub temp_directory: Option<String>,
}

impl Default for ScreenRecordingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_recordings: 1,
            default_output_directory: "~/Videos/Recordings".to_string(),
            default_video_codec: VideoCodec::H264,
            default_audio_codec: AudioCodec::Aac,
            default_frame_rate: 30,
            default_hardware_encoding: true,
            max_recording_duration: 0,
            min_disk_space: 500,
            temp_directory: None,
        }
    }
}

/// Container for all screen recording services
pub struct ScreenRecordingServices {
    /// Recording management service
    pub recording: Arc<dyn RecordingService>,
    /// Capture device management service
    pub devices: Arc<dyn CaptureDeviceService>,
    /// Configuration
    config: ScreenRecordingConfig,
    /// Event broadcaster
    event_sender: broadcast::Sender<RecordingEvent>,
}

impl ScreenRecordingServices {
    /// Create a new instance of screen recording services
    pub fn new(config: ScreenRecordingConfig) -> Self {
        let (event_sender, _) = broadcast::channel(256);
        
        let recording = Arc::new(DefaultRecordingService::new());
        let devices = Arc::new(DefaultCaptureDeviceService::new());
        
        Self {
            recording,
            devices,
            config,
            event_sender,
        }
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ScreenRecordingConfig::default())
    }
    
    /// Subscribe to recording events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<RecordingEvent> {
        self.event_sender.subscribe()
    }
    
    /// Get current configuration
    pub fn config(&self) -> &ScreenRecordingConfig {
        &self.config
    }
}

/// Builder for creating screen recording services with custom configuration
pub struct ScreenRecordingServicesBuilder {
    config: ScreenRecordingConfig,
}

impl ScreenRecordingServicesBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: ScreenRecordingConfig::default(),
        }
    }
    
    /// Set default output directory
    pub fn output_directory(mut self, path: impl Into<String>) -> Self {
        self.config.default_output_directory = path.into();
        self
    }
    
    /// Set default video codec
    pub fn video_codec(mut self, codec: VideoCodec) -> Self {
        self.config.default_video_codec = codec;
        self
    }
    
    /// Set default frame rate
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.config.default_frame_rate = fps;
        self
    }
    
    /// Enable or disable hardware encoding
    pub fn hardware_encoding(mut self, enable: bool) -> Self {
        self.config.default_hardware_encoding = enable;
        self
    }
    
    /// Set maximum recording duration
    pub fn max_duration(mut self, seconds: u64) -> Self {
        self.config.max_recording_duration = seconds;
        self
    }
    
    /// Set minimum disk space required
    pub fn min_disk_space(mut self, mb: u64) -> Self {
        self.config.min_disk_space = mb;
        self
    }
    
    /// Build the screen recording services
    pub fn build(self) -> ScreenRecordingServices {
        ScreenRecordingServices::new(self.config)
    }
}

impl Default for ScreenRecordingServicesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick recording preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingPreset {
    /// Quick screen recording (30fps, medium quality)
    QuickScreen,
    /// High quality recording (60fps, high quality)
    HighQuality,
    /// Game recording (60fps, optimized for games)
    Game,
    /// Tutorial recording (30fps, includes cursor effects)
    Tutorial,
    /// Webcam only
    Webcam,
    /// Audio only
    AudioOnly,
}

impl RecordingPreset {
    /// Get the recording configuration for this preset
    pub fn config(&self) -> RecordingConfig {
        match self {
            RecordingPreset::QuickScreen => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Screen { screen_index: 0 },
                    frame_rate: 30,
                    hardware_encoding: true,
                    quality: QualityPreset::Medium,
                    ..Default::default()
                },
                audio: AudioCaptureConfig::default(),
                output: OutputConfig::default(),
                overlays: Vec::new(),
                hotkeys: HotkeyConfig::default(),
            },
            RecordingPreset::HighQuality => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Screen { screen_index: 0 },
                    frame_rate: 60,
                    hardware_encoding: true,
                    quality: QualityPreset::High,
                    ..Default::default()
                },
                audio: AudioCaptureConfig {
                    enabled: true,
                    sample_rate: 48000,
                    channels: 2,
                    codec: AudioCodec::Aac,
                    bitrate: 256,
                    sources: vec![AudioSourceConfig::SystemAudio],
                },
                output: OutputConfig::default(),
                overlays: Vec::new(),
                hotkeys: HotkeyConfig::default(),
            },
            RecordingPreset::Game => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Game { 
                        process_id: 0, 
                        process_name: String::new() 
                    },
                    frame_rate: 60,
                    hardware_encoding: true,
                    quality: QualityPreset::High,
                    ..Default::default()
                },
                audio: AudioCaptureConfig {
                    enabled: true,
                    sample_rate: 48000,
                    channels: 2,
                    codec: AudioCodec::Opus,
                    bitrate: 192,
                    sources: vec![
                        AudioSourceConfig::SystemAudio,
                    ],
                },
                output: OutputConfig::default(),
                overlays: Vec::new(),
                hotkeys: HotkeyConfig::default(),
            },
            RecordingPreset::Tutorial => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Screen { screen_index: 0 },
                    frame_rate: 30,
                    capture_cursor: true,
                    cursor_effects: CursorEffects {
                        show_cursor: true,
                        highlight_clicks: true,
                        click_animation: true,
                        ..Default::default()
                    },
                    hardware_encoding: true,
                    quality: QualityPreset::High,
                    ..Default::default()
                },
                audio: AudioCaptureConfig {
                    enabled: true,
                    sources: vec![
                        AudioSourceConfig::SystemAudio,
                        AudioSourceConfig::Microphone { 
                            device_index: 0, 
                            name: "Default".to_string() 
                        },
                    ],
                    ..Default::default()
                },
                output: OutputConfig::default(),
                overlays: vec![
                    OverlayConfig::new(OverlayType::Timer {
                        format: TimerFormat::MmSs,
                        font: FontConfig::default(),
                    }),
                ],
                hotkeys: HotkeyConfig::default(),
            },
            RecordingPreset::Webcam => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Webcam { device_index: 0 },
                    frame_rate: 30,
                    hardware_encoding: true,
                    quality: QualityPreset::High,
                    ..Default::default()
                },
                audio: AudioCaptureConfig {
                    enabled: true,
                    sources: vec![AudioSourceConfig::Microphone { 
                        device_index: 0, 
                        name: "Default".to_string() 
                    }],
                    ..Default::default()
                },
                output: OutputConfig::default(),
                overlays: Vec::new(),
                hotkeys: HotkeyConfig::default(),
            },
            RecordingPreset::AudioOnly => RecordingConfig {
                video: VideoCaptureConfig {
                    source: CaptureSource::Screen { screen_index: 0 }, // Placeholder
                    enabled: false,
                    ..Default::default()
                },
                audio: AudioCaptureConfig {
                    enabled: true,
                    sources: vec![
                        AudioSourceConfig::SystemAudio,
                        AudioSourceConfig::Microphone { 
                            device_index: 0, 
                            name: "Default".to_string() 
                        },
                    ],
                    ..Default::default()
                },
                output: OutputConfig {
                    format: OutputFormat::Mp4, // Will contain audio only
                    ..Default::default()
                },
                overlays: Vec::new(),
                hotkeys: HotkeyConfig::default(),
            },
        }
    }
}

/// Create a quick recording session
pub async fn create_quick_session(
    name: &str,
    preset: RecordingPreset,
) -> ScreenRecordingResult<(RecordingSession, ScreenRecordingServices)> {
    let services = ScreenRecordingServices::with_defaults();
    let config = preset.config();
    let session = services.recording.create_session(name, config).await?;
    Ok((session, services))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ScreenRecordingConfig::default();
        assert_eq!(config.default_frame_rate, 30);
        assert!(config.default_hardware_encoding);
    }
    
    #[test]
    fn test_builder() {
        let services = ScreenRecordingServicesBuilder::new()
            .frame_rate(60)
            .hardware_encoding(false)
            .build();
        
        assert_eq!(services.config().default_frame_rate, 60);
        assert!(!services.config().default_hardware_encoding);
    }
    
    #[test]
    fn test_presets() {
        let quick = RecordingPreset::QuickScreen.config();
        assert_eq!(quick.video.frame_rate, 30);
        
        let hq = RecordingPreset::HighQuality.config();
        assert_eq!(hq.video.frame_rate, 60);
        
        let game = RecordingPreset::Game.config();
        assert_eq!(game.video.frame_rate, 60);
    }
    
    #[test]
    fn test_resolution() {
        let hd = Resolution::hd();
        assert_eq!(hd.width, 1280);
        assert_eq!(hd.height, 720);
        
        let fhd = Resolution::full_hd();
        assert_eq!(fhd.width, 1920);
    }
    
    #[tokio::test]
    async fn test_create_session() {
        let services = ScreenRecordingServices::with_defaults();
        let config = RecordingConfig::default();
        
        let session = services.recording.create_session("Test", config).await.unwrap();
        assert_eq!(session.name, "Test");
        assert_eq!(session.status, RecordingStatus::Idle);
    }
}