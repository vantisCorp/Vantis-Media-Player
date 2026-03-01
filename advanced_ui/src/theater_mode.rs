//! Theater Mode
//! 
//! Provides an immersive theater mode experience with dimmed background
//! and minimal UI distractions.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};

use crate::TheaterModeConfig;

/// Theater mode controller
pub struct TheaterMode {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Configuration
    config: Arc<RwLock<TheaterModeConfig>>,
    
    /// Is theater mode active
    is_active: Arc<RwLock<bool>>,
    
    /// Is fullscreen
    is_fullscreen: Arc<RwLock<bool>>,
    
    /// UI visibility
    ui_visible: Arc<RwLock<bool>>,
    
    /// Background dimming level
    dimming_level: Arc<RwLock<f32>>,
    
    /// Controls visibility
    controls_visible: Arc<RwLock<bool>>,
    
    /// Auto-hide timer
    auto_hide_timer: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

/// Theater mode state
#[derive(Debug, Clone)]
pub struct TheaterModeState {
    /// Is theater mode active
    pub is_active: bool,
    
    /// Is fullscreen
    pub is_fullscreen: bool,
    
    /// UI visible
    pub ui_visible: bool,
    
    /// Background dimming level
    pub dimming_level: f32,
    
    /// Controls visible
    pub controls_visible: bool,
}

impl TheaterMode {
    /// Create a new theater mode controller
    pub fn new(config: TheaterModeConfig) -> Result<Self> {
        info!("Initializing theater mode controller");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config)),
            is_active: Arc::new(RwLock::new(false)),
            is_fullscreen: Arc::new(RwLock::new(false)),
            ui_visible: Arc::new(RwLock::new(true)),
            dimming_level: Arc::new(RwLock::new(config.dimming_level)),
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
    pub async fn set_config(&self, config: TheaterModeConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Theater mode configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> TheaterModeConfig {
        self.config.read().await.clone()
    }
    
    /// Enter theater mode
    pub async fn enter(&self) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.enabled {
            return Err(anyhow::anyhow!("Theater mode is disabled"));
        }
        
        info!("Entering theater mode");
        
        *self.is_active.write().await = true;
        *self.dimming_level.write().await = config.dimming_level;
        
        // Hide UI if configured
        if config.hide_ui {
            *self.ui_visible.write().await = false;
        }
        
        // Enter fullscreen if configured
        if config.fullscreen_by_default {
            self.enter_fullscreen().await?;
        }
        
        // Start auto-hide timer for controls
        if config.show_controls_on_hover {
            self.start_auto_hide_timer().await;
        }
        
        info!("✓ Theater mode entered");
        Ok(())
    }
    
    /// Exit theater mode
    pub async fn exit(&self) -> Result<()> {
        info!("Exiting theater mode");
        
        *self.is_active.write().await = false;
        *self.ui_visible.write().await = true;
        *self.controls_visible.write().await = true;
        
        // Exit fullscreen
        self.exit_fullscreen().await?;
        
        // Cancel auto-hide timer
        let mut timer = self.auto_hide_timer.write().await;
        if let Some(handle) = timer.take() {
            handle.abort();
        }
        
        info!("✓ Theater mode exited");
        Ok(())
    }
    
    /// Toggle theater mode
    pub async fn toggle(&self) -> Result<()> {
        if *self.is_active.read().await {
            self.exit().await
        } else {
            self.enter().await
        }
    }
    
    /// Enter fullscreen
    pub async fn enter_fullscreen(&self) -> Result<()> {
        info!("Entering fullscreen");
        *self.is_fullscreen.write().await = true;
        Ok(())
    }
    
    /// Exit fullscreen
    pub async fn exit_fullscreen(&self) -> Result<()> {
        info!("Exiting fullscreen");
        *self.is_fullscreen.write().await = false;
        Ok(())
    }
    
    /// Toggle fullscreen
    pub async fn toggle_fullscreen(&self) -> Result<()> {
        if *self.is_fullscreen.read().await {
            self.exit_fullscreen().await
        } else {
            self.enter_fullscreen().await
        }
    }
    
    /// Show UI
    pub async fn show_ui(&self) -> Result<()> {
        *self.ui_visible.write().await = true;
        info!("UI shown");
        Ok(())
    }
    
    /// Hide UI
    pub async fn hide_ui(&self) -> Result<()> {
        *self.ui_visible.write().await = false;
        info!("UI hidden");
        Ok(())
    }
    
    /// Toggle UI
    pub async fn toggle_ui(&self) -> Result<()> {
        let visible = *self.ui_visible.read().await;
        if visible {
            self.hide_ui().await
        } else {
            self.show_ui().await
        }
    }
    
    /// Show controls
    pub async fn show_controls(&self) -> Result<()> {
        *self.controls_visible.write().await = true;
        
        // Restart auto-hide timer
        let config = self.config.read().await;
        if config.show_controls_on_hover {
            self.start_auto_hide_timer().await;
        }
        
        Ok(())
    }
    
    /// Hide controls
    pub async fn hide_controls(&self) -> Result<()> {
        *self.controls_visible.write().await = false;
        Ok(())
    }
    
    /// Set dimming level
    pub async fn set_dimming_level(&self, level: f32) -> Result<()> {
        let level = level.clamp(0.0, 1.0);
        *self.dimming_level.write().await = level;
        info!("Dimming level set to {:.2}", level);
        Ok(())
    }
    
    /// Get dimming level
    pub async fn get_dimming_level(&self) -> f32 {
        *self.dimming_level.read().await
    }
    
    /// Start auto-hide timer
    async fn start_auto_hide_timer(&self) {
        let config = self.config.read().await;
        let delay = Duration::from_secs(config.controls_auto_hide_delay as u64);
        
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
    pub async fn get_state(&self) -> TheaterModeState {
        TheaterModeState {
            is_active: *self.is_active.read().await,
            is_fullscreen: *self.is_fullscreen.read().await,
            ui_visible: *self.ui_visible.read().await,
            dimming_level: *self.dimming_level.read().await,
            controls_visible: *self.controls_visible.read().await,
        }
    }
}

use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_theater_mode_enter_exit() {
        let config = TheaterModeConfig::default();
        let theater_mode = TheaterMode::new(config).unwrap();
        
        assert!(!theater_mode.get_state().await.is_active);
        
        theater_mode.enter().await.unwrap();
        assert!(theater_mode.get_state().await.is_active);
        
        theater_mode.exit().await.unwrap();
        assert!(!theater_mode.get_state().await.is_active);
    }
    
    #[tokio::test]
    async fn test_theater_mode_toggle() {
        let config = TheaterModeConfig::default();
        let theater_mode = TheaterMode::new(config).unwrap();
        
        theater_mode.toggle().await.unwrap();
        assert!(theater_mode.get_state().await.is_active);
        
        theater_mode.toggle().await.unwrap();
        assert!(!theater_mode.get_state().await.is_active);
    }
    
    #[tokio::test]
    async fn test_fullscreen_toggle() {
        let config = TheaterModeConfig::default();
        let theater_mode = TheaterMode::new(config).unwrap();
        
        theater_mode.enter_fullscreen().await.unwrap();
        assert!(theater_mode.get_state().await.is_fullscreen);
        
        theater_mode.toggle_fullscreen().await.unwrap();
        assert!(!theater_mode.get_state().await.is_fullscreen);
    }
}