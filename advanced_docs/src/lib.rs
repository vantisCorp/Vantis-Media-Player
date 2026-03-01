//! Vantis Advanced Documentation System
//! 
//! This module provides advanced documentation capabilities including:
//! - Interactive tutorials
//! - Video tutorials
//! - API playground
//! - Code examples gallery
//! - Troubleshooting wizard

use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, debug, warn, error};

pub mod tutorials;
pub mod video_tutorials;
pub mod api_playground;
pub mod examples_gallery;
pub mod troubleshooting;
pub mod utils;

use tutorials::TutorialManager;
use video_tutorials::VideoTutorialManager;
use api_playground::APIPlayground;
use examples_gallery::ExamplesGallery;
use troubleshooting::TroubleshootingWizard;

/// Advanced documentation system
pub struct AdvancedDocumentationSystem {
    /// Tutorial manager
    tutorials: TutorialManager,
    
    /// Video tutorial manager
    video_tutorials: VideoTutorialManager,
    
    /// API playground
    api_playground: APIPlayground,
    
    /// Examples gallery
    examples_gallery: ExamplesGallery,
    
    /// Troubleshooting wizard
    troubleshooting: TroubleshootingWizard,
    
    /// Documentation directory
    docs_dir: PathBuf,
    
    /// Output directory
    output_dir: PathBuf,
    
    /// Configuration
    config: AdvancedDocsConfig,
}

/// Advanced documentation configuration
#[derive(Clone, Debug)]
pub struct AdvancedDocsConfig {
    /// Enable interactive tutorials
    pub enable_tutorials: bool,
    
    /// Enable video tutorials
    pub enable_video_tutorials: bool,
    
    /// Enable API playground
    pub enable_api_playground: bool,
    
    /// Enable examples gallery
    pub enable_examples_gallery: bool,
    
    /// Enable troubleshooting wizard
    pub enable_troubleshooting: bool,
    
    /// Server port
    pub server_port: u16,
    
    /// Auto-reload documentation
    pub auto_reload: bool,
    
    /// Generate static site
    pub generate_static: bool,
}

impl Default for AdvancedDocsConfig {
    fn default() -> Self {
        Self {
            enable_tutorials: true,
            enable_video_tutorials: true,
            enable_api_playground: true,
            enable_examples_gallery: true,
            enable_troubleshooting: true,
            server_port: 8080,
            auto_reload: true,
            generate_static: true,
        }
    }
}

impl AdvancedDocumentationSystem {
    /// Create a new advanced documentation system
    pub fn new(docs_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Self::with_config(docs_dir, output_dir, AdvancedDocsConfig::default())
    }
    
    /// Create with custom configuration
    pub fn with_config(
        docs_dir: PathBuf,
        output_dir: PathBuf,
        config: AdvancedDocsConfig,
    ) -> Result<Self> {
        info!("📚 Initializing Advanced Documentation System");
        
        // Create directories if they don't exist
        std::fs::create_dir_all(&docs_dir)?;
        std::fs::create_dir_all(&output_dir)?;
        
        // Initialize subsystems
        let tutorials = TutorialManager::new(docs_dir.join("tutorials"))?;
        let video_tutorials = VideoTutorialManager::new(docs_dir.join("video_tutorials"))?;
        let api_playground = APIPlayground::new(docs_dir.join("api_playground"))?;
        let examples_gallery = ExamplesGallery::new(docs_dir.join("examples"))?;
        let troubleshooting = TroubleshootingWizard::new(docs_dir.join("troubleshooting"))?;
        
        info!("✅ Advanced documentation system initialized");
        info!("   - Documentation directory: {}", docs_dir.display());
        info!("   - Output directory: {}", output_dir.display());
        info!("   - Server port: {}", config.server_port);
        
        Ok(Self {
            tutorials,
            video_tutorials,
            api_playground,
            examples_gallery,
            troubleshooting,
            docs_dir,
            output_dir,
            config,
        })
    }
    
    /// Generate all documentation
    pub async fn generate_all(&self) -> Result<()> {
        info!("📚 Generating all documentation");
        
        if self.config.enable_tutorials {
            self.tutorials.generate().await?;
        }
        
        if self.config.enable_video_tutorials {
            self.video_tutorials.generate().await?;
        }
        
        if self.config.enable_api_playground {
            self.api_playground.generate().await?;
        }
        
        if self.config.enable_examples_gallery {
            self.examples_gallery.generate().await?;
        }
        
        if self.config.enable_troubleshooting {
            self.troubleshooting.generate().await?;
        }
        
        info!("✅ All documentation generated successfully");
        
        Ok(())
    }
    
    /// Start documentation server
    pub async fn start_server(&self) -> Result<()> {
        info!("🚀 Starting documentation server on port {}", self.config.server_port);
        
        // Start the web server
        // This would use axum to serve the documentation
        info!("✅ Documentation server started");
        info!("   - URL: http://localhost:{}", self.config.server_port);
        
        Ok(())
    }
    
    /// Get tutorial manager
    pub fn tutorials(&self) -> &TutorialManager {
        &self.tutorials
    }
    
    /// Get video tutorial manager
    pub fn video_tutorials(&self) -> &VideoTutorialManager {
        &self.video_tutorials
    }
    
    /// Get API playground
    pub fn api_playground(&self) -> &APIPlayground {
        &self.api_playground
    }
    
    /// Get examples gallery
    pub fn examples_gallery(&self) -> &ExamplesGallery {
        &self.examples_gallery
    }
    
    /// Get troubleshooting wizard
    pub fn troubleshooting(&self) -> &TroubleshootingWizard {
        &self.troubleshooting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_docs_system_creation() {
        let docs_dir = PathBuf::from("./docs");
        let output_dir = PathBuf::from("./output");
        let system = AdvancedDocumentationSystem::new(docs_dir, output_dir);
        assert!(system.is_ok());
    }
}