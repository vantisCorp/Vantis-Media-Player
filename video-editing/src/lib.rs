//! Video Editing Module for Vantis Media Player
//!
//! This module provides comprehensive video editing capabilities including:
//! - Multi-track timeline editing
//! - Clip management (cut, trim, split, join)
//! - Video and audio effects
//! - Transitions and animations
//! - Keyframe animation
//! - Export and rendering
//! - Undo/redo history

pub mod types;
pub mod error;
pub mod project_service;
pub mod clip_service;
pub mod render_service;

// Re-export key types
pub use types::*;
pub use error::{VideoEditingError, VideoEditingResult};

use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast;

use project_service::{ProjectService, DefaultProjectService, TimelineService, DefaultTimelineService};
use clip_service::{ClipService, DefaultClipService, EffectService, DefaultEffectService};
use render_service::{RenderService, DefaultRenderService, HistoryService, DefaultHistoryService};

/// Configuration for the video editing module
#[derive(Debug, Clone)]
pub struct VideoEditingConfig {
    /// Maximum undo history size
    pub max_undo_history: usize,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u64,
    /// Default preview quality
    pub default_preview_quality: PreviewQuality,
    /// Enable hardware acceleration for rendering
    pub hardware_acceleration: bool,
    /// Maximum concurrent render jobs
    pub max_concurrent_renders: usize,
    /// Temporary file directory
    pub temp_directory: Option<String>,
}

impl Default for VideoEditingConfig {
    fn default() -> Self {
        Self {
            max_undo_history: 100,
            auto_save_interval: 300, // 5 minutes
            default_preview_quality: PreviewQuality::Medium,
            hardware_acceleration: true,
            max_concurrent_renders: 2,
            temp_directory: None,
        }
    }
}

/// Container for all video editing services
pub struct VideoEditingServices {
    /// Project management service
    pub project: Arc<dyn ProjectService>,
    /// Timeline management service
    pub timeline: Arc<dyn TimelineService>,
    /// Clip management service
    pub clip: Arc<dyn ClipService>,
    /// Effect management service
    pub effect: Arc<dyn EffectService>,
    /// Rendering service
    pub render: Arc<dyn RenderService>,
    /// Undo/redo history service
    pub history: Arc<dyn HistoryService>,
    /// Configuration
    config: VideoEditingConfig,
    /// Event broadcaster
    event_sender: broadcast::Sender<EditingEvent>,
}

impl VideoEditingServices {
    /// Create a new instance of video editing services
    pub fn new(config: VideoEditingConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1024);
        let projects = Arc::new(DashMap::new());
        
        let project = Arc::new(DefaultProjectService::new());
        let timeline = Arc::new(DefaultTimelineService::new(projects.clone()));
        let clip = Arc::new(DefaultClipService::new(projects.clone()));
        let effect = Arc::new(DefaultEffectService::new(projects.clone()));
        let render = Arc::new(DefaultRenderService::new(projects.clone()));
        let history = Arc::new(DefaultHistoryService::new(projects));
        
        Self {
            project,
            timeline,
            clip,
            effect,
            render,
            history,
            config,
            event_sender,
        }
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(VideoEditingConfig::default())
    }
    
    /// Subscribe to editing events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<EditingEvent> {
        self.event_sender.subscribe()
    }
    
    /// Get current configuration
    pub fn config(&self) -> &VideoEditingConfig {
        &self.config
    }
}

/// Editing events for real-time notifications
#[derive(Debug, Clone)]
pub enum EditingEvent {
    /// Project created
    ProjectCreated(ProjectId),
    /// Project opened
    ProjectOpened(ProjectId),
    /// Project saved
    ProjectSaved(ProjectId),
    /// Project closed
    ProjectClosed(ProjectId),
    /// Track added
    TrackAdded {
        project_id: ProjectId,
        track_id: TrackId,
    },
    /// Track removed
    TrackRemoved {
        project_id: ProjectId,
        track_id: TrackId,
    },
    /// Clip added
    ClipAdded {
        project_id: ProjectId,
        track_id: TrackId,
        clip_id: ClipId,
    },
    /// Clip removed
    ClipRemoved {
        project_id: ProjectId,
        track_id: TrackId,
        clip_id: ClipId,
    },
    /// Clip modified
    ClipModified {
        project_id: ProjectId,
        track_id: TrackId,
        clip_id: ClipId,
    },
    /// Effect added
    EffectAdded {
        project_id: ProjectId,
        track_id: TrackId,
        clip_id: ClipId,
        effect_id: EffectId,
    },
    /// Effect removed
    EffectRemoved {
        project_id: ProjectId,
        track_id: TrackId,
        clip_id: ClipId,
        effect_id: EffectId,
    },
    /// Playhead moved
    PlayheadMoved {
        project_id: ProjectId,
        position: TimePosition,
    },
    /// Selection changed
    SelectionChanged {
        project_id: ProjectId,
        selection: Option<TimeRange>,
    },
    /// Render started
    RenderStarted {
        render_id: uuid::Uuid,
        project_id: ProjectId,
    },
    /// Render completed
    RenderCompleted {
        render_id: uuid::Uuid,
        project_id: ProjectId,
    },
    /// Render failed
    RenderFailed {
        render_id: uuid::Uuid,
        project_id: ProjectId,
        error: String,
    },
    /// Undo performed
    UndoPerformed {
        project_id: ProjectId,
        action: String,
    },
    /// Redo performed
    RedoPerformed {
        project_id: ProjectId,
        action: String,
    },
}

/// Builder for creating video editing services with custom configuration
pub struct VideoEditingServicesBuilder {
    config: VideoEditingConfig,
}

impl VideoEditingServicesBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: VideoEditingConfig::default(),
        }
    }
    
    /// Set maximum undo history size
    pub fn max_undo_history(mut self, max: usize) -> Self {
        self.config.max_undo_history = max;
        self
    }
    
    /// Set auto-save interval in seconds
    pub fn auto_save_interval(mut self, seconds: u64) -> Self {
        self.config.auto_save_interval = seconds;
        self
    }
    
    /// Set default preview quality
    pub fn preview_quality(mut self, quality: PreviewQuality) -> Self {
        self.config.default_preview_quality = quality;
        self
    }
    
    /// Enable or disable hardware acceleration
    pub fn hardware_acceleration(mut self, enable: bool) -> Self {
        self.config.hardware_acceleration = enable;
        self
    }
    
    /// Set maximum concurrent render jobs
    pub fn max_concurrent_renders(mut self, max: usize) -> Self {
        self.config.max_concurrent_renders = max;
        self
    }
    
    /// Set temporary directory
    pub fn temp_directory(mut self, path: impl Into<String>) -> Self {
        self.config.temp_directory = Some(path.into());
        self
    }
    
    /// Build the video editing services
    pub fn build(self) -> VideoEditingServices {
        VideoEditingServices::new(self.config)
    }
}

impl Default for VideoEditingServicesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset for common video resolutions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionPreset {
    SD480p,
    HD720p,
    FullHD1080p,
    QHD1440p,
    UltraHD4K,
    UltraHD8K,
    InstagramSquare,
    InstagramPortrait,
    InstagramLandscape,
    YouTubeShorts,
    TikTok,
    Custom(u32, u32),
}

impl ResolutionPreset {
    /// Get the resolution for this preset
    pub fn resolution(&self) -> Resolution {
        match self {
            ResolutionPreset::SD480p => Resolution::new(854, 480),
            ResolutionPreset::HD720p => Resolution::new(1280, 720),
            ResolutionPreset::FullHD1080p => Resolution::new(1920, 1080),
            ResolutionPreset::QHD1440p => Resolution::new(2560, 1440),
            ResolutionPreset::UltraHD4K => Resolution::new(3840, 2160),
            ResolutionPreset::UltraHD8K => Resolution::new(7680, 4320),
            ResolutionPreset::InstagramSquare => Resolution::new(1080, 1080),
            ResolutionPreset::InstagramPortrait => Resolution::new(1080, 1350),
            ResolutionPreset::InstagramLandscape => Resolution::new(1080, 608),
            ResolutionPreset::YouTubeShorts => Resolution::new(1080, 1920),
            ResolutionPreset::TikTok => Resolution::new(1080, 1920),
            ResolutionPreset::Custom(w, h) => Resolution::new(*w, *h),
        }
    }
}

/// Preset for common frame rates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRatePreset {
    Film24fps,
    PAL25fps,
    NTSC30fps,
    High60fps,
    Ultra120fps,
    Custom(u32, u32),
}

impl FrameRatePreset {
    /// Get the frame rate for this preset
    pub fn frame_rate(&self) -> FrameRate {
        match self {
            FrameRatePreset::Film24fps => FrameRate::fps24(),
            FrameRatePreset::PAL25fps => FrameRate::fps25(),
            FrameRatePreset::NTSC30fps => FrameRate::fps30(),
            FrameRatePreset::High60fps => FrameRate::fps60(),
            FrameRatePreset::Ultra120fps => FrameRate(120.0),
            FrameRatePreset::Custom(num, den) => FrameRate::from_rational(*num, *den),
        }
    }
}

/// Quick project creation helper
pub fn create_quick_project(name: &str, preset: ResolutionPreset, fps: FrameRatePreset) -> Project {
    let mut project = Project::new(name);
    project.settings.resolution = preset.resolution();
    project.settings.frame_rate = fps.frame_rate();
    project
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = VideoEditingConfig::default();
        assert_eq!(config.max_undo_history, 100);
        assert_eq!(config.auto_save_interval, 300);
        assert!(config.hardware_acceleration);
    }
    
    #[test]
    fn test_resolution_presets() {
        assert_eq!(ResolutionPreset::FullHD1080p.resolution().width, 1920);
        assert_eq!(ResolutionPreset::FullHD1080p.resolution().height, 1080);
        assert_eq!(ResolutionPreset::UltraHD4K.resolution().width, 3840);
    }
    
    #[test]
    fn test_frame_rate_presets() {
        assert_eq!(FrameRatePreset::Film24fps.frame_rate().0, 24.0);
        assert_eq!(FrameRatePreset::NTSC30fps.frame_rate().0, 30.0);
    }
    
    #[test]
    fn test_builder() {
        let services = VideoEditingServicesBuilder::new()
            .max_undo_history(50)
            .hardware_acceleration(false)
            .build();
        
        assert_eq!(services.config().max_undo_history, 50);
        assert!(!services.config().hardware_acceleration);
    }
    
    #[test]
    fn test_time_position() {
        let pos = TimePosition::new(10.5);
        assert_eq!(pos.as_seconds(), 10.5);
        assert_eq!(pos.as_millis(), 10500);
        assert_eq!(pos.as_frames(30.0), 315);
    }
    
    #[test]
    fn test_time_range() {
        let range = TimeRange::new(5.0, 10.0);
        assert_eq!(range.start.0, 5.0);
        assert_eq!(range.duration.0, 10.0);
        assert_eq!(range.end().0, 15.0);
        assert!(range.contains(TimePosition::new(10.0)));
        assert!(!range.contains(TimePosition::new(20.0)));
    }
    
    #[test]
    fn test_quick_project() {
        let project = create_quick_project(
            "Test Project",
            ResolutionPreset::FullHD1080p,
            FrameRatePreset::NTSC30fps,
        );
        
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.settings.resolution.width, 1920);
        assert_eq!(project.settings.frame_rate.0, 30.0);
    }
}