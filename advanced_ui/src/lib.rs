//! Vantis Advanced UI Module
//! 
//! This module provides advanced UI components including:
//! - Picture-in-Picture mode
//! - Mini-player mode
//! - Theater mode
//! - Gesture controls
//! - Keyboard shortcut customization

pub mod pip;
pub mod mini_player;
pub mod theater_mode;
pub mod gestures;
pub mod shortcuts;

use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug};

use pip::PictureInPicture;
use mini_player::MiniPlayer;
use theater_mode::TheaterMode;
use gestures::GestureController;
use shortcuts::ShortcutManager;

/// Configuration for advanced UI features
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdvancedUIConfig {
    /// Picture-in-Picture settings
    pub pip: PipConfig,
    
    /// Mini-player settings
    pub mini_player: MiniPlayerConfig,
    
    /// Theater mode settings
    pub theater_mode: TheaterModeConfig,
    
    /// Gesture control settings
    pub gestures: GestureConfig,
    
    /// Keyboard shortcut settings
    pub shortcuts: ShortcutConfig,
}

impl Default for AdvancedUIConfig {
    fn default() -> Self {
        Self {
            pip: PipConfig::default(),
            mini_player: MiniPlayerConfig::default(),
            theater_mode: TheaterModeConfig::default(),
            gestures: GestureConfig::default(),
            shortcuts: ShortcutConfig::default(),
        }
    }
}

/// Picture-in-Picture configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipConfig {
    /// Enable Picture-in-Picture
    pub enabled: bool,
    
    /// Default position
    pub default_position: PipPosition,
    
    /// Default size
    pub default_size: (u32, u32),
    
    /// Allow dragging
    pub allow_dragging: bool,
    
    /// Allow resizing
    pub allow_resizing: bool,
    
    /// Snap to edges
    pub snap_to_edges: bool,
    
    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
}

impl Default for PipConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_position: PipPosition::BottomRight,
            default_size: (320, 180),
            allow_dragging: true,
            allow_resizing: true,
            snap_to_edges: true,
            opacity: 0.95,
        }
    }
}

/// Picture-in-Picture position
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum PipPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: i32, y: i32 },
}

/// Mini-player configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MiniPlayerConfig {
    /// Enable mini-player
    pub enabled: bool,
    
    /// Default position
    pub default_position: MiniPlayerPosition,
    
    /// Show controls
    pub show_controls: bool,
    
    /// Show progress bar
    pub show_progress: bool,
    
    /// Auto-hide controls
    pub auto_hide_controls: bool,
    
    /// Auto-hide delay in seconds
    pub auto_hide_delay: u32,
    
    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
}

impl Default for MiniPlayerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_position: MiniPlayerPosition::BottomCenter,
            show_controls: true,
            show_progress: true,
            auto_hide_controls: true,
            auto_hide_delay: 3,
            opacity: 0.9,
        }
    }
}

/// Mini-player position
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum MiniPlayerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Theater mode configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TheaterModeConfig {
    /// Enable theater mode
    pub enabled: bool,
    
    /// Hide UI elements
    pub hide_ui: bool,
    
    /// Dim background
    pub dim_background: bool,
    
    /// Background dimming level (0.0 - 1.0)
    pub dimming_level: f32,
    
    /// Fullscreen by default
    pub fullscreen_by_default: bool,
    
    /// Show controls on hover
    pub show_controls_on_hover: bool,
    
    /// Controls auto-hide delay in seconds
    pub controls_auto_hide_delay: u32,
}

impl Default for TheaterModeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hide_ui: true,
            dim_background: true,
            dimming_level: 0.8,
            fullscreen_by_default: false,
            show_controls_on_hover: true,
            controls_auto_hide_delay: 3,
        }
    }
}

/// Gesture control configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GestureConfig {
    /// Enable gesture controls
    pub enabled: bool,
    
    /// Enable swipe gestures
    pub enable_swipe: bool,
    
    /// Enable pinch gestures
    pub enable_pinch: bool,
    
    /// Enable tap gestures
    pub enable_tap: bool,
    
    /// Swipe sensitivity
    pub swipe_sensitivity: f32,
    
    /// Pinch sensitivity
    pub pinch_sensitivity: f32,
    
    /// Tap duration threshold in milliseconds
    pub tap_duration_threshold: u32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_swipe: true,
            enable_pinch: true,
            enable_tap: true,
            swipe_sensitivity: 1.0,
            pinch_sensitivity: 1.0,
            tap_duration_threshold: 300,
        }
    }
}

/// Keyboard shortcut configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortcutConfig {
    /// Enable custom shortcuts
    pub enabled: bool,
    
    /// Allow user customization
    pub allow_customization: true,
    
    /// Default shortcuts
    pub default_shortcuts: Vec<Shortcut>,
    
    /// Custom shortcuts
    pub custom_shortcuts: Vec<Shortcut>,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_customization: true,
            default_shortcuts: vec![
                Shortcut {
                    action: ShortcutAction::PlayPause,
                    keys: vec![Key::Space],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::SeekForward,
                    keys: vec![Key::Right],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::SeekBackward,
                    keys: vec![Key::Left],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::VolumeUp,
                    keys: vec![Key::Up],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::VolumeDown,
                    keys: vec![Key::Down],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::Mute,
                    keys: vec![Key::M],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::Fullscreen,
                    keys: vec![Key::F],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::ToggleSubtitles,
                    keys: vec![Key::S],
                    modifiers: vec![],
                },
                Shortcut {
                    action: ShortcutAction::PictureInPicture,
                    keys: vec![Key::P],
                    modifiers: vec![Modifier::Control],
                },
                Shortcut {
                    action: ShortcutAction::MiniPlayer,
                    keys: vec![Key::M],
                    modifiers: vec![Modifier::Control],
                },
                Shortcut {
                    action: ShortcutAction::TheaterMode,
                    keys: vec![Key::T],
                    modifiers: vec![Modifier::Control],
                },
            ],
            custom_shortcuts: vec![],
        }
    }
}

/// Keyboard shortcut
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    /// Action to perform
    pub action: ShortcutAction,
    
    /// Keys to press
    pub keys: Vec<Key>,
    
    /// Modifier keys
    pub modifiers: Vec<Modifier>,
}

/// Shortcut action
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    PlayPause,
    Stop,
    SeekForward,
    SeekBackward,
    SeekForwardLarge,
    SeekBackwardLarge,
    VolumeUp,
    VolumeDown,
    Mute,
    Fullscreen,
    ToggleSubtitles,
    NextSubtitle,
    PreviousSubtitle,
    NextTrack,
    PreviousTrack,
    PictureInPicture,
    MiniPlayer,
    TheaterMode,
    Screenshot,
    FrameStepForward,
    FrameStepBackward,
    SpeedUp,
    SpeedDown,
    ResetSpeed,
    ZoomIn,
    ZoomOut,
    ResetZoom,
    RotateLeft,
    RotateRight,
    FlipHorizontal,
    FlipVertical,
}

/// Key
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum Key {
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
}

/// Modifier key
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum Modifier {
    Control,
    Alt,
    Shift,
    Meta,
}

/// Advanced UI engine
pub struct AdvancedUIEngine {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Picture-in-Picture controller
    pip: Arc<PictureInPicture>,
    
    /// Mini-player controller
    mini_player: Arc<MiniPlayer>,
    
    /// Theater mode controller
    theater_mode: Arc<TheaterMode>,
    
    /// Gesture controller
    gestures: Arc<GestureController>,
    
    /// Shortcut manager
    shortcuts: Arc<ShortcutManager>,
}

impl AdvancedUIEngine {
    /// Create a new advanced UI engine with default config
    pub fn new() -> Result<Self> {
        Self::new_with_config(AdvancedUIConfig::default())
    }
    
    /// Create a new advanced UI engine
    pub fn new_with_config(config: AdvancedUIConfig) -> Result<Self> {
        info!("Initializing advanced UI engine");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            pip: Arc::new(PictureInPicture::new(config.pip)?),
            mini_player: Arc::new(MiniPlayer::new(config.mini_player)?),
            theater_mode: Arc::new(TheaterMode::new(config.theater_mode)?),
            gestures: Arc::new(GestureController::new(config.gestures)?),
            shortcuts: Arc::new(ShortcutManager::new(config.shortcuts)?),
        })
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *self.is_initialized.read().await
            })
        })
    }
    
    /// Get Picture-in-Picture controller
    pub fn pip(&self) -> &PictureInPicture {
        &self.pip
    }
    
    /// Get Mini-player controller
    pub fn mini_player(&self) -> &MiniPlayer {
        &self.mini_player
    }
    
    /// Get Theater mode controller
    pub fn theater_mode(&self) -> &TheaterMode {
        &self.theater_mode
    }
    
    /// Get Gesture controller
    pub fn gestures(&self) -> &GestureController {
        &self.gestures
    }
    
    /// Get Shortcut manager
    pub fn shortcuts(&self) -> &ShortcutManager {
        &self.shortcuts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_advanced_ui_engine_creation() {
        let config = AdvancedUIConfig::default();
        let engine = AdvancedUIEngine::new(config).unwrap();
        assert!(engine.is_initialized().await);
    }
    
    #[tokio::test]
    async fn test_config_update() {
        let config = AdvancedUIConfig::default();
        let engine = AdvancedUIEngine::new(config).unwrap();
        
        let mut new_config = engine.get_config().await;
        new_config.pip.enabled = true;
        engine.update_config(new_config).await.unwrap();
        
        let updated = engine.get_config().await;
        assert!(updated.pip.enabled);
    }
}