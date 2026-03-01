//! Mini-Player Mode
//! 
//! Provides a compact mini-player that can be displayed in a corner of the screen
//! while browsing or using other applications.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use image::RgbImage;

use crate::{MiniPlayerConfig, MiniPlayerPosition};

/// Mini-player controller
pub struct MiniPlayer {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Configuration
    config: Arc<RwLock<MiniPlayerConfig>>,
    
    /// Current position
    position: Arc<RwLock<MiniPlayerPosition>>,
    
    /// Is mini-player active
    is_active: Arc<RwLock<bool>>,
    
    /// Is mini-player visible
    is_visible: Arc<RwLock<bool>>,
    
    /// Current frame
    current_frame: Arc<RwLock<Option<RgbImage>>>,
    
    /// Current playback state
    playback_state: Arc<RwLock<PlaybackState>>,
    
    /// Controls visibility
    controls_visible: Arc<RwLock<bool>>,
    
    /// Auto-hide timer
    auto_hide_timer: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

/// Playback state
#[derive(Debug, Clone)]
pub struct PlaybackState {
    /// Is playing
    pub is_playing: bool,
    
    /// Current position in seconds
    pub current_position: f64,
    
    /// Duration in seconds
    pub duration: f64,
    
    /// Volume (0.0 - 1.0)
    pub volume: f32,
    
    /// Is muted
    pub is_muted: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_position: 0.0,
            duration: 0.0,
            volume: 1.0,
            is_muted: false,
        }
    }
}

/// Mini-player state
#[derive(Debug, Clone)]
pub struct MiniPlayerState {
    /// Is mini-player active
    pub is_active: bool,
    
    /// Is mini-player visible
    pub is_visible: bool,
    
    /// Current position
    pub position: MiniPlayerPosition,
    
    /// Playback state
    pub playback_state: PlaybackState,
    
    /// Controls visible
    pub controls_visible: bool,
    
    /// Opacity
    pub opacity: f32,
}

impl MiniPlayer {
    /// Create a new mini-player controller
    pub fn new(config: MiniPlayerConfig) -> Result<Self> {
        info!("Initializing mini-player controller");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config)),
            position: Arc::new(RwLock::new(config.default_position)),
            is_active: Arc::new(RwLock::new(false)),
            is_visible: Arc::new(RwLock::new(false)),
            current_frame: Arc::new(RwLock::new(None)),
            playback_state: Arc::new(RwLock::new(PlaybackState::default())),
            controls_visible: Arc::new(RwLock::new(true)),
            auto_hide_timer: Arc::new(RwLock::new(None)),
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
    
    /// Set configuration
    pub async fn set_config(&self, config: MiniPlayerConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Mini-player configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> MiniPlayerConfig {
        self.config.read().await.clone()
    }
    
    /// Start mini-player
    pub async fn start(&self) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.enabled {
            return Err(anyhow::anyhow!("Mini-player is disabled"));
        }
        
        info!("Starting mini-player");
        
        *self.is_active.write().await = true;
        *self.is_visible.write().await = true;
        *self.position.write().await = config.default_position;
        
        // Start auto-hide timer if enabled
        if config.auto_hide_controls {
            self.start_auto_hide_timer().await;
        }
        
        info!("✓ Mini-player started");
        Ok(())
    }
    
    /// Stop mini-player
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping mini-player");
        
        *self.is_active.write().await = false;
        *self.is_visible.write().await = false;
        
        // Cancel auto-hide timer
        let mut timer = self.auto_hide_timer.write().await;
        if let Some(handle) = timer.take() {
            handle.abort();
        }
        
        info!("✓ Mini-player stopped");
        Ok(())
    }
    
    /// Show mini-player
    pub async fn show(&self) -> Result<()> {
        if !*self.is_active.read().await {
            return Err(anyhow::anyhow!("Mini-player is not active"));
        }
        
        *self.is_visible.write().await = true;
        info!("Mini-player shown");
        Ok(())
    }
    
    /// Hide mini-player
    pub async fn hide(&self) -> Result<()> {
        *self.is_visible.write().await = false;
        info!("Mini-player hidden");
        Ok(())
    }
    
    /// Set position
    pub async fn set_position(&self, position: MiniPlayerPosition) -> Result<()> {
        *self.position.write().await = position;
        info!("Mini-player position set to {:?}", position);
        Ok(())
    }
    
    /// Get position
    pub async fn get_position(&self) -> MiniPlayerPosition {
        *self.position.read().await
    }
    
    /// Update frame
    pub async fn update_frame(&self, frame: &RgbImage) -> Result<()> {
        if !*self.is_active.read().await {
            return Err(anyhow::anyhow!("Mini-player is not active"));
        }
        
        *self.current_frame.write().await = Some(frame.clone());
        Ok(())
    }
    
    /// Get current frame
    pub async fn get_current_frame(&self) -> Option<RgbImage> {
        self.current_frame.read().await.clone()
    }
    
    /// Update playback state
    pub async fn update_playback_state(&self, state: PlaybackState) -> Result<()> {
        *self.playback_state.write().await = state;
        Ok(())
    }
    
    /// Get playback state
    pub async fn get_playback_state(&self) -> PlaybackState {
        self.playback_state.read().await.clone()
    }
    
    /// Play/Pause
    pub async fn toggle_playback(&self) -> Result<()> {
        let mut state = self.playback_state.write().await;
        state.is_playing = !state.is_playing;
        info!("Playback toggled: {}", state.is_playing);
        Ok(())
    }
    
    /// Seek to position
    pub async fn seek(&self, position: f64) -> Result<()> {
        let mut state = self.playback_state.write().await;
        state.current_position = position.clamp(0.0, state.duration);
        info!("Seeked to {:.2}s", position);
        Ok(())
    }
    
    /// Set volume
    pub async fn set_volume(&self, volume: f32) -> Result<()> {
        let mut state = self.playback_state.write().await;
        state.volume = volume.clamp(0.0, 1.0);
        state.is_muted = false;
        info!("Volume set to {:.2}", volume);
        Ok(())
    }
    
    /// Toggle mute
    pub async fn toggle_mute(&self) -> Result<()> {
        let mut state = self.playback_state.write().await;
        state.is_muted = !state.is_muted;
        info!("Mute toggled: {}", state.is_muted);
        Ok(())
    }
    
    /// Show controls
    pub async fn show_controls(&self) -> Result<()> {
        *self.controls_visible.write().await = true;
        
        // Restart auto-hide timer
        let config = self.config.read().await;
        if config.auto_hide_controls {
            self.start_auto_hide_timer().await;
        }
        
        Ok(())
    }
    
    /// Hide controls
    pub async fn hide_controls(&self) -> Result<()> {
        *self.controls_visible.write().await = false;
        Ok(())
    }
    
    /// Start auto-hide timer
    async fn start_auto_hide_timer(&self) {
        let config = self.config.read().await;
        let delay = Duration::from_secs(config.auto_hide_delay as u64);
        
        let controls_visible = self.controls_visible.clone();
        
        // Cancel existing timer
        let mut timer = self.auto_hide_timer.write().await;
        if let Some(handle) = timer.take() {
            handle.abort();
        }
        
        // Start new timer
        let handle = tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            *controls_visible.write().await = false;
        });
        
        *timer = Some(handle);
    }
    
    /// Get state
    pub async fn get_state(&self) -> MiniPlayerState {
        MiniPlayerState {
            is_active: *self.is_active.read().await,
            is_visible: *self.is_visible.read().await,
            position: *self.position.read().await,
            playback_state: self.playback_state.read().await.clone(),
            controls_visible: *self.controls_visible.read().await,
            opacity: self.config.read().await.opacity,
        }
    }
}

use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mini_player_start_stop() {
        let config = MiniPlayerConfig::default();
        let mini_player = MiniPlayer::new(config).unwrap();
        
        assert!(!mini_player.get_state().await.is_active);
        
        mini_player.start().await.unwrap();
        assert!(mini_player.get_state().await.is_active);
        
        mini_player.stop().await.unwrap();
        assert!(!mini_player.get_state().await.is_active);
    }
    
    #[tokio::test]
    async fn test_playback_toggle() {
        let config = MiniPlayerConfig::default();
        let mini_player = MiniPlayer::new(config).unwrap();
        
        mini_player.start().await.unwrap();
        
        assert!(!mini_player.get_playback_state().await.is_playing);
        
        mini_player.toggle_playback().await.unwrap();
        assert!(mini_player.get_playback_state().await.is_playing);
        
        mini_player.toggle_playback().await.unwrap();
        assert!(!mini_player.get_playback_state().await.is_playing);
    }
    
    #[tokio::test]
    async fn test_volume_control() {
        let config = MiniPlayerConfig::default();
        let mini_player = MiniPlayer::new(config).unwrap();
        
        mini_player.start().await.unwrap();
        
        mini_player.set_volume(0.5).await.unwrap();
        assert_eq!(mini_player.get_playback_state().await.volume, 0.5);
        
        mini_player.toggle_mute().await.unwrap();
        assert!(mini_player.get_playback_state().await.is_muted);
    }
}