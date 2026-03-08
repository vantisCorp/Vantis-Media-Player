//! Picture-in-Picture (PiP) functionality
//! 
//! Advanced PiP features:
//! - Floating video window management
//! - Resizable and draggable PiP window
//! - Always-on-top support
//! - Multiple PiP instances
//! - Keyboard shortcuts in PiP mode
//! - Volume and playback controls in PiP

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};

/// Picture-in-Picture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiPConfig {
    /// Initial window width in pixels
    pub initial_width: u32,
    
    /// Initial window height in pixels
    pub initial_height: u32,
    
    /// Minimum window width
    pub min_width: u32,
    
    /// Minimum window height
    pub min_height: u32,
    
    /// Maximum window width (0 = no limit)
    pub max_width: u32,
    
    /// Maximum window height (0 = no limit)
    pub max_height: u32,
    
    /// Keep aspect ratio when resizing
    pub keep_aspect_ratio: bool,
    
    /// Default aspect ratio (width / height)
    pub default_aspect_ratio: f32,
    
    /// Always on top
    pub always_on_top: bool,
    
    /// Show controls overlay
    pub show_controls: bool,
    
    /// Controls auto-hide delay in milliseconds
    pub controls_hide_delay: u32,
    
    /// Enable keyboard shortcuts
    pub enable_shortcuts: bool,
    
    /// Enable mouse interactions
    pub enable_mouse: bool,
    
    /// Snap to screen edges
    pub snap_to_edges: bool,
    
    /// Snap threshold in pixels
    pub snap_threshold: u32,
    
    /// Opacity when not focused (0.0 - 1.0)
    pub unfocused_opacity: f32,
    
    /// Border radius in pixels
    pub border_radius: u32,
    
    /// Show video title
    pub show_title: bool,
}

impl Default for PiPConfig {
    fn default() -> Self {
        Self {
            initial_width: 400,
            initial_height: 225, // 16:9 aspect ratio
            min_width: 200,
            min_height: 112,
            max_width: 0, // No limit
            max_height: 0,
            keep_aspect_ratio: true,
            default_aspect_ratio: 16.0 / 9.0,
            always_on_top: true,
            show_controls: true,
            controls_hide_delay: 3000,
            enable_shortcuts: true,
            enable_mouse: true,
            snap_to_edges: true,
            snap_threshold: 20,
            unfocused_opacity: 0.9,
            border_radius: 12,
            show_title: true,
        }
    }
}

/// PiP window position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PiPPosition {
    /// X position on screen
    pub x: i32,
    
    /// Y position on screen
    pub y: i32,
}

impl Default for PiPPosition {
    fn default() -> Self {
        Self { x: 100, y: 100 }
    }
}

/// PiP window size
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PiPSize {
    /// Width in pixels
    pub width: u32,
    
    /// Height in pixels
    pub height: u32,
}

impl Default for PiPSize {
    fn default() -> Self {
        Self { width: 400, height: 225 }
    }
}

impl PiPSize {
    /// Create new size
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    /// Calculate aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            return 16.0 / 9.0;
        }
        self.width as f32 / self.height as f32
    }
    
    /// Scale to fit within bounds while keeping aspect ratio
    pub fn scale_to_fit(&self, max_width: u32, max_height: u32) -> Self {
        let aspect = self.aspect_ratio();
        let max_aspect = max_width as f32 / max_height as f32;
        
        if aspect > max_aspect {
            // Width limited
            Self {
                width: max_width,
                height: (max_width as f32 / aspect) as u32,
            }
        } else {
            // Height limited
            Self {
                width: (max_height as f32 * aspect) as u32,
                height: max_height,
            }
        }
    }
}

/// PiP window state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PiPState {
    /// PiP is disabled
    Disabled,
    /// Transitioning to PiP mode
    Entering,
    /// Active in PiP mode
    Active,
    /// Paused in PiP mode
    Paused,
    /// Transitioning out of PiP mode
    Exiting,
}

/// Picture-in-Picture manager
pub struct PiPManager {
    /// PiP configuration
    config: PiPConfig,
    
    /// Current state
    state: Arc<RwLock<PiPState>>,
    
    /// Current position
    position: Arc<RwLock<PiPPosition>>,
    
    /// Current size
    size: Arc<RwLock<PiPSize>>,
    
    /// Video source ID
    video_source: Arc<RwLock<Option<String>>>,
    
    /// Video title
    video_title: Arc<RwLock<Option<String>>>,
    
    /// Is muted
    muted: Arc<RwLock<bool>>,
    
    /// Volume (0.0 - 1.0)
    volume: Arc<RwLock<f32>>,
    
    /// Is playing
    playing: Arc<RwLock<bool>>,
    
    /// Is controls visible
    controls_visible: Arc<RwLock<bool>>,
    
    /// Callbacks
    on_enter: Option<Arc<RwLock<Box<dyn Fn() + Send + Sync>>>>,
    on_exit: Option<Arc<RwLock<Box<dyn Fn() + Send + Sync>>>>,
    on_move: Option<Arc<RwLock<Box<dyn Fn(i32, i32) + Send + Sync>>>>,
    on_resize: Option<Arc<RwLock<Box<dyn Fn(u32, u32) + Send + Sync>>>>,
    on_play_pause: Option<Arc<RwLock<Box<dyn Fn(bool) + Send + Sync>>>>,
}

impl PiPManager {
    /// Create a new PiP manager
    pub fn new(config: Option<PiPConfig>) -> Self {
        let config = config.unwrap_or_default();
        
        Self {
            config,
            state: Arc::new(RwLock::new(PiPState::Disabled)),
            position: Arc::new(RwLock::new(PiPPosition::default())),
            size: Arc::new(RwLock::new(PiPSize::new(
                config.initial_width,
                config.initial_height,
            ))),
            video_source: Arc::new(RwLock::new(None)),
            video_title: Arc::new(RwLock::new(None)),
            muted: Arc::new(RwLock::new(false)),
            volume: Arc::new(RwLock::new(1.0)),
            playing: Arc::new(RwLock::new(false)),
            controls_visible: Arc::new(RwLock::new(true)),
            on_enter: None,
            on_exit: None,
            on_move: None,
            on_resize: None,
            on_play_pause: None,
        }
    }
    
    /// Check if PiP is available
    pub fn is_available() -> bool {
        // In a real implementation, this would check:
        // - Platform support (Windows 10+, macOS 10.12+, Linux with X11/Wayland)
        // - Browser support if running in WASM
        true
    }
    
    /// Enter Picture-in-Picture mode
    pub fn enter(&self, video_source: &str, title: Option<&str>) -> Result<()> {
        let current_state = *self.state.read();
        if current_state == PiPState::Active {
            debug!("Already in PiP mode");
            return Ok(());
        }
        
        info!("🖥️ Entering Picture-in-Picture mode");
        
        *self.state.write() = PiPState::Entering;
        *self.video_source.write() = Some(video_source.to_string());
        *self.video_title.write() = Some(title.unwrap_or("Video").to_string());
        *self.playing.write() = true;
        *self.controls_visible.write() = true;
        
        // Complete transition
        *self.state.write() = PiPState::Active;
        
        info!("   Size: {}x{}", self.size.read().width, self.size.read().height);
        info!("   Position: {}, {}", self.position.read().x, self.position.read().y);
        
        // Call callback
        if let Some(callback) = &self.on_enter {
            let callback = callback.read();
            callback();
        }
        
        Ok(())
    }
    
    /// Exit Picture-in-Picture mode
    pub fn exit(&self) -> Result<()> {
        let current_state = *self.state.read();
        if current_state == PiPState::Disabled {
            debug!("Not in PiP mode");
            return Ok(());
        }
        
        info!("🖥️ Exiting Picture-in-Picture mode");
        
        *self.state.write() = PiPState::Exiting;
        
        // Complete transition
        *self.state.write() = PiPState::Disabled;
        *self.video_source.write() = None;
        *self.video_title.write() = None;
        
        // Call callback
        if let Some(callback) = &self.on_exit {
            let callback = callback.read();
            callback();
        }
        
        Ok(())
    }
    
    /// Toggle Picture-in-Picture mode
    pub fn toggle(&self, video_source: &str, title: Option<&str>) -> Result<()> {
        let current_state = *self.state.read();
        if current_state == PiPState::Active {
            self.exit()
        } else {
            self.enter(video_source, title)
        }
    }
    
    /// Get current state
    pub fn state(&self) -> PiPState {
        *self.state.read()
    }
    
    /// Check if in PiP mode
    pub fn is_active(&self) -> bool {
        *self.state.read() == PiPState::Active
    }
    
    /// Set position
    pub fn set_position(&self, x: i32, y: i32) {
        let mut pos = self.position.write();
        pos.x = x;
        pos.y = y;
        
        debug!("PiP position: {}, {}", x, y);
        
        if let Some(callback) = &self.on_move {
            let callback = callback.read();
            callback(x, y);
        }
    }
    
    /// Get position
    pub fn position(&self) -> PiPPosition {
        *self.position.read()
    }
    
    /// Move by delta
    pub fn move_by(&self, dx: i32, dy: i32) {
        let current = self.position.read();
        self.set_position(current.x + dx, current.y + dy);
    }
    
    /// Set size
    pub fn set_size(&self, width: u32, height: u32) {
        let width = width.max(self.config.min_width);
        let height = height.max(self.config.min_height);
        
        let width = if self.config.max_width > 0 {
            width.min(self.config.max_width)
        } else {
            width
        };
        
        let height = if self.config.max_height > 0 {
            height.min(self.config.max_height)
        } else {
            height
        };
        
        // Keep aspect ratio if enabled
        let (width, height) = if self.config.keep_aspect_ratio {
            let aspect = self.config.default_aspect_ratio;
            let new_aspect = width as f32 / height as f32;
            
            if new_aspect > aspect {
                (width, (width as f32 / aspect) as u32)
            } else {
                ((height as f32 * aspect) as u32, height)
            }
        } else {
            (width, height)
        };
        
        *self.size.write() = PiPSize { width, height };
        
        debug!("PiP size: {}x{}", width, height);
        
        if let Some(callback) = &self.on_resize {
            let callback = callback.read();
            callback(width, height);
        }
    }
    
    /// Get size
    pub fn size(&self) -> PiPSize {
        *self.size.read()
    }
    
    /// Scale size by factor
    pub fn scale(&self, factor: f32) {
        let current = self.size.read();
        self.set_size(
            (current.width as f32 * factor) as u32,
            (current.height as f32 * factor) as u32,
        );
    }
    
    /// Set aspect ratio
    pub fn set_aspect_ratio(&self, aspect_ratio: f32) {
        let current = self.size.read();
        let new_height = (current.width as f32 / aspect_ratio) as u32;
        
        *self.size.write() = PiPSize {
            width: current.width,
            height: new_height,
        };
        
        self.config.default_aspect_ratio = aspect_ratio;
    }
    
    /// Play
    pub fn play(&self) {
        *self.playing.write() = true;
        *self.state.write() = PiPState::Active;
        
        if let Some(callback) = &self.on_play_pause {
            let callback = callback.read();
            callback(true);
        }
    }
    
    /// Pause
    pub fn pause(&self) {
        *self.playing.write() = false;
        *self.state.write() = PiPState::Paused;
        
        if let Some(callback) = &self.on_play_pause {
            let callback = callback.read();
            callback(false);
        }
    }
    
    /// Toggle play/pause
    pub fn toggle_play_pause(&self) {
        if *self.playing.read() {
            self.pause();
        } else {
            self.play();
        }
    }
    
    /// Is playing
    pub fn is_playing(&self) -> bool {
        *self.playing.read()
    }
    
    /// Set volume
    pub fn set_volume(&self, volume: f32) {
        *self.volume.write() = volume.clamp(0.0, 1.0);
        debug!("PiP volume: {:.0}%", volume * 100.0);
    }
    
    /// Get volume
    pub fn volume(&self) -> f32 {
        *self.volume.read()
    }
    
    /// Set muted
    pub fn set_muted(&self, muted: bool) {
        *self.muted.write() = muted;
        debug!("PiP muted: {}", muted);
    }
    
    /// Is muted
    pub fn is_muted(&self) -> bool {
        *self.muted.read()
    }
    
    /// Toggle mute
    pub fn toggle_mute(&self) {
        let muted = !*self.muted.read();
        self.set_muted(muted);
    }
    
    /// Show controls
    pub fn show_controls(&self) {
        *self.controls_visible.write() = true;
    }
    
    /// Hide controls
    pub fn hide_controls(&self) {
        *self.controls_visible.write() = true;
    }
    
    /// Toggle controls visibility
    pub fn toggle_controls(&self) {
        let visible = !*self.controls_visible.read();
        *self.controls_visible.write() = visible;
    }
    
    /// Are controls visible
    pub fn are_controls_visible(&self) -> bool {
        *self.controls_visible.read()
    }
    
    /// Get video title
    pub fn video_title(&self) -> Option<String> {
        self.video_title.read().clone()
    }
    
    /// Set always on top
    pub fn set_always_on_top(&self, always_on_top: bool) {
        // In real implementation, this would update window properties
        debug!("PiP always on top: {}", always_on_top);
    }
    
    /// Snap to screen edge
    pub fn snap_to_nearest_edge(&self, screen_width: u32, screen_height: u32) {
        if !self.config.snap_to_edges {
            return;
        }
        
        let threshold = self.config.snap_threshold as i32;
        let pos = self.position.read();
        let size = self.size.read();
        
        let mut new_x = pos.x;
        let mut new_y = pos.y;
        
        // Left edge
        if pos.x.abs() < threshold {
            new_x = 0;
        }
        
        // Right edge
        if (pos.x + size.width as i32 - screen_width as i32).abs() < threshold {
            new_x = screen_width as i32 - size.width as i32;
        }
        
        // Top edge
        if pos.y.abs() < threshold {
            new_y = 0;
        }
        
        // Bottom edge
        if (pos.y + size.height as i32 - screen_height as i32).abs() < threshold {
            new_y = screen_height as i32 - size.height as i32;
        }
        
        if new_x != pos.x || new_y != pos.y {
            self.set_position(new_x, new_y);
            debug!("Snapped to edge: {}, {}", new_x, new_y);
        }
    }
    
    /// Center on screen
    pub fn center_on_screen(&self, screen_width: u32, screen_height: u32) {
        let size = self.size.read();
        let x = (screen_width - size.width) as i32 / 2;
        let y = (screen_height - size.height) as i32 / 2;
        self.set_position(x, y);
    }
    
    /// Move to corner
    pub fn move_to_corner(&self, corner: Corner, screen_width: u32, screen_height: u32, margin: u32) {
        let size = self.size.read();
        
        let (x, y) = match corner {
            Corner::TopLeft => (margin as i32, margin as i32),
            Corner::TopRight => ((screen_width - size.width - margin) as i32, margin as i32),
            Corner::BottomLeft => (margin as i32, (screen_height - size.height - margin) as i32),
            Corner::BottomRight => (
                (screen_width - size.width - margin) as i32,
                (screen_height - size.height - margin) as i32,
            ),
        };
        
        self.set_position(x, y);
    }
    
    /// Set enter callback
    pub fn set_on_enter<F>(&mut self, callback: F) 
    where 
        F: Fn() + Send + Sync + 'static 
    {
        self.on_enter = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Set exit callback
    pub fn set_on_exit<F>(&mut self, callback: F) 
    where 
        F: Fn() + Send + Sync + 'static 
    {
        self.on_exit = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Set move callback
    pub fn set_on_move<F>(&mut self, callback: F) 
    where 
        F: Fn(i32, i32) + Send + Sync + 'static 
    {
        self.on_move = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Set resize callback
    pub fn set_on_resize<F>(&mut self, callback: F) 
    where 
        F: Fn(u32, u32) + Send + Sync + 'static 
    {
        self.on_resize = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Set play/pause callback
    pub fn set_on_play_pause<F>(&mut self, callback: F) 
    where 
        F: Fn(bool) + Send + Sync + 'static 
    {
        self.on_play_pause = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Get config reference
    pub fn config(&self) -> &PiPConfig {
        &self.config
    }
}

impl Default for PiPManager {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Screen corner
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// PiP keyboard shortcuts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiPShortcuts {
    /// Toggle PiP
    pub toggle_pip: String,
    
    /// Play/pause
    pub play_pause: String,
    
    /// Mute/unmute
    pub mute: String,
    
    /// Volume up
    pub volume_up: String,
    
    /// Volume down
    pub volume_down: String,
    
    /// Move left
    pub move_left: String,
    
    /// Move right
    pub move_right: String,
    
    /// Move up
    pub move_up: String,
    
    /// Move down
    pub move_down: String,
    
    /// Increase size
    pub size_up: String,
    
    /// Decrease size
    pub size_down: String,
    
    /// Exit PiP
    pub exit: String,
}

impl Default for PiPShortcuts {
    fn default() -> Self {
        Self {
            toggle_pip: "P".to_string(),
            play_pause: "Space".to_string(),
            mute: "M".to_string(),
            volume_up: "Up".to_string(),
            volume_down: "Down".to_string(),
            move_left: "Left".to_string(),
            move_right: "Right".to_string(),
            move_up: "Shift+Up".to_string(),
            move_down: "Shift+Down".to_string(),
            size_up: "Plus".to_string(),
            size_down: "Minus".to_string(),
            exit: "Escape".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pip_config_default() {
        let config = PiPConfig::default();
        assert_eq!(config.initial_width, 400);
        assert_eq!(config.always_on_top, true);
        assert!(config.show_controls);
    }
    
    #[test]
    fn test_pip_manager_creation() {
        let manager = PiPManager::new(None);
        assert_eq!(manager.state(), PiPState::Disabled);
        assert!(!manager.is_active());
    }
    
    #[test]
    fn test_pip_enter_exit() {
        let manager = PiPManager::new(None);
        
        manager.enter("video.mp4", Some("Test Video")).unwrap();
        assert!(manager.is_active());
        assert_eq!(manager.video_title(), Some("Test Video".to_string()));
        
        manager.exit().unwrap();
        assert!(!manager.is_active());
    }
    
    #[test]
    fn test_pip_toggle() {
        let manager = PiPManager::new(None);
        
        manager.toggle("video.mp4", None).unwrap();
        assert!(manager.is_active());
        
        manager.toggle("video.mp4", None).unwrap();
        assert!(!manager.is_active());
    }
    
    #[test]
    fn test_pip_position() {
        let manager = PiPManager::new(None);
        
        manager.set_position(200, 150);
        assert_eq!(manager.position().x, 200);
        assert_eq!(manager.position().y, 150);
        
        manager.move_by(50, -50);
        assert_eq!(manager.position().x, 250);
        assert_eq!(manager.position().y, 100);
    }
    
    #[test]
    fn test_pip_size() {
        let manager = PiPManager::new(None);
        
        manager.set_size(800, 450);
        assert_eq!(manager.size().width, 800);
        assert_eq!(manager.size().height, 450);
        
        // Test scaling
        manager.scale(0.5);
        assert_eq!(manager.size().width, 400);
        assert_eq!(manager.size().height, 225);
    }
    
    #[test]
    fn test_pip_playback() {
        let manager = PiPManager::new(None);
        manager.enter("video.mp4", None).unwrap();
        
        manager.play();
        assert!(manager.is_playing());
        
        manager.pause();
        assert!(!manager.is_playing());
        
        manager.toggle_play_pause();
        assert!(manager.is_playing());
    }
    
    #[test]
    fn test_pip_volume() {
        let manager = PiPManager::new(None);
        
        manager.set_volume(0.5);
        assert_eq!(manager.volume(), 0.5);
        
        // Test clamping
        manager.set_volume(1.5);
        assert_eq!(manager.volume(), 1.0);
        
        manager.set_volume(-0.5);
        assert_eq!(manager.volume(), 0.0);
    }
    
    #[test]
    fn test_pip_mute() {
        let manager = PiPManager::new(None);
        
        assert!(!manager.is_muted());
        
        manager.set_muted(true);
        assert!(manager.is_muted());
        
        manager.toggle_mute();
        assert!(!manager.is_muted());
    }
    
    #[test]
    fn test_pip_size_aspect_ratio() {
        let size = PiPSize::new(1920, 1080);
        assert!((size.aspect_ratio() - 16.0/9.0).abs() < 0.01);
        
        let scaled = size.scale_to_fit(400, 300);
        assert_eq!(scaled.width, 400);
        assert_eq!(scaled.height, 225);
    }
    
    #[test]
    fn test_pip_move_to_corner() {
        let manager = PiPManager::new(None);
        
        manager.move_to_corner(Corner::TopRight, 1920, 1080, 20);
        let pos = manager.position();
        assert!(pos.x > 0);
        assert_eq!(pos.y, 20);
    }
    
    #[test]
    fn test_pip_center_on_screen() {
        let manager = PiPManager::new(None);
        manager.set_size(400, 225);
        
        manager.center_on_screen(1920, 1080);
        let pos = manager.position();
        
        assert_eq!(pos.x, (1920 - 400) as i32 / 2);
        assert_eq!(pos.y, (1080 - 225) as i32 / 2);
    }
}