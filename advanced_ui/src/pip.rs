//! Picture-in-Picture Mode
//! 
//! Provides Picture-in-Picture functionality allowing users to watch videos
//! in a small floating window while using other applications.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use image::RgbImage;

use crate::{PipConfig, PipPosition};

/// Picture-in-Picture controller
pub struct PictureInPicture {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Configuration
    config: Arc<RwLock<PipConfig>>,
    
    /// Current position
    position: Arc<RwLock<PipPosition>>,
    
    /// Current size
    size: Arc<RwLock<(u32, u32)>>,
    
    /// Is PiP active
    is_active: Arc<RwLock<bool>>,
    
    /// Is PiP visible
    is_visible: Arc<RwLock<bool>>,
    
    /// Current frame
    current_frame: Arc<RwLock<Option<RgbImage>>>,
    
    /// Window handle (placeholder)
    window_handle: Arc<RwLock<Option<u64>>>,
}

/// PiP state
#[derive(Debug, Clone)]
pub struct PipState {
    /// Is PiP active
    pub is_active: bool,
    
    /// Is PiP visible
    pub is_visible: bool,
    
    /// Current position
    pub position: PipPosition,
    
    /// Current size
    pub size: (u32, u32),
    
    /// Opacity
    pub opacity: f32,
}

impl PictureInPicture {
    /// Create a new Picture-in-Picture controller
    pub fn new(config: PipConfig) -> Result<Self> {
        info!("Initializing Picture-in-Picture controller");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config)),
            position: Arc::new(RwLock::new(config.default_position)),
            size: Arc::new(RwLock::new(config.default_size)),
            is_active: Arc::new(RwLock::new(false)),
            is_visible: Arc::new(RwLock::new(false)),
            current_frame: Arc::new(RwLock::new(None)),
            window_handle: Arc::new(RwLock::new(None)),
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
    pub async fn set_config(&self, config: PipConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("PiP configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> PipConfig {
        self.config.read().await.clone()
    }
    
    /// Start Picture-in-Picture
    pub async fn start(&self) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.enabled {
            return Err(anyhow::anyhow!("Picture-in-Picture is disabled"));
        }
        
        info!("Starting Picture-in-Picture");
        
        // Create PiP window (placeholder)
        *self.window_handle.write().await = Some(1);
        *self.is_active.write().await = true;
        *self.is_visible.write().await = true;
        
        // Set default position and size
        *self.position.write().await = config.default_position;
        *self.size.write().await = config.default_size;
        
        info!("✓ Picture-in-Picture started");
        Ok(())
    }
    
    /// Stop Picture-in-Picture
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping Picture-in-Picture");
        
        *self.is_active.write().await = false;
        *self.is_visible.write().await = false;
        *self.window_handle.write().await = None;
        
        info!("✓ Picture-in-Picture stopped");
        Ok(())
    }
    
    /// Show PiP window
    pub async fn show(&self) -> Result<()> {
        if !*self.is_active.read().await {
            return Err(anyhow::anyhow!("Picture-in-Picture is not active"));
        }
        
        *self.is_visible.write().await = true;
        info!("PiP window shown");
        Ok(())
    }
    
    /// Hide PiP window
    pub async fn hide(&self) -> Result<()> {
        *self.is_visible.write().await = false;
        info!("PiP window hidden");
        Ok(())
    }
    
    /// Set position
    pub async fn set_position(&self, position: PipPosition) -> Result<()> {
        *self.position.write().await = position;
        info!("PiP position set to {:?}", position);
        Ok(())
    }
    
    /// Get position
    pub async fn get_position(&self) -> PipPosition {
        *self.position.read().await
    }
    
    /// Set size
    pub async fn set_size(&self, width: u32, height: u32) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.allow_resizing {
            return Err(anyhow::anyhow!("Resizing is disabled"));
        }
        
        *self.size.write().await = (width, height);
        info!("PiP size set to {}x{}", width, height);
        Ok(())
    }
    
    /// Get size
    pub async fn get_size(&self) -> (u32, u32) {
        *self.size.read().await
    }
    
    /// Set opacity
    pub async fn set_opacity(&self, opacity: f32) -> Result<()> {
        let opacity = opacity.clamp(0.0, 1.0);
        let mut config = self.config.write().await;
        config.opacity = opacity;
        info!("PiP opacity set to {:.2}", opacity);
        Ok(())
    }
    
    /// Get opacity
    pub async fn get_opacity(&self) -> f32 {
        self.config.read().await.opacity
    }
    
    /// Update frame
    pub async fn update_frame(&self, frame: &RgbImage) -> Result<()> {
        if !*self.is_active.read().await {
            return Err(anyhow::anyhow!("Picture-in-Picture is not active"));
        }
        
        *self.current_frame.write().await = Some(frame.clone());
        Ok(())
    }
    
    /// Get current frame
    pub async fn get_current_frame(&self) -> Option<RgbImage> {
        self.current_frame.read().await.clone()
    }
    
    /// Get state
    pub async fn get_state(&self) -> PipState {
        PipState {
            is_active: *self.is_active.read().await,
            is_visible: *self.is_visible.read().await,
            position: *self.position.read().await,
            size: *self.size.read().await,
            opacity: self.config.read().await.opacity,
        }
    }
    
    /// Snap to nearest edge
    pub async fn snap_to_edge(&self) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.snap_to_edges {
            return Ok(());
        }
        
        let position = *self.position.read().await;
        let snapped = match position {
            PipPosition::Custom { x, y } => {
                // Determine nearest edge based on screen coordinates
                // This is a simplified implementation
                if x < 0 {
                    PipPosition::TopLeft
                } else if x > 1000 {
                    PipPosition::TopRight
                } else if y < 0 {
                    PipPosition::BottomLeft
                } else {
                    PipPosition::BottomRight
                }
            }
            _ => position,
        };
        
        *self.position.write().await = snapped;
        info!("PiP snapped to {:?}", snapped);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pip_start_stop() {
        let config = PipConfig::default();
        let pip = PictureInPicture::new(config).unwrap();
        
        assert!(!pip.get_state().await.is_active);
        
        pip.start().await.unwrap();
        assert!(pip.get_state().await.is_active);
        
        pip.stop().await.unwrap();
        assert!(!pip.get_state().await.is_active);
    }
    
    #[tokio::test]
    async fn test_pip_show_hide() {
        let config = PipConfig::default();
        let pip = PictureInPicture::new(config).unwrap();
        
        pip.start().await.unwrap();
        
        pip.hide().await.unwrap();
        assert!(!pip.get_state().await.is_visible);
        
        pip.show().await.unwrap();
        assert!(pip.get_state().await.is_visible);
    }
    
    #[tokio::test]
    async fn test_pip_position() {
        let config = PipConfig::default();
        let pip = PictureInPicture::new(config).unwrap();
        
        pip.set_position(PipPosition::TopLeft).await.unwrap();
        assert_eq!(pip.get_position().await, PipPosition::TopLeft);
        
        pip.set_position(PipPosition::BottomRight).await.unwrap();
        assert_eq!(pip.get_position().await, PipPosition::BottomRight);
    }
}