//! Vantis Advanced Video Module
//! 
//! This module provides advanced video processing features including:
//! - Video stabilization
//! - Frame interpolation (AI-based)
//! - Video denoising
//! - Color grading presets
//! - Video comparison tools

pub mod stabilization;
pub mod frame_interpolation;
pub mod denoising;
pub mod color_grading;
pub mod comparison;
pub mod utils;

use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug};

use stabilization::VideoStabilizer;
use frame_interpolation::FrameInterpolator;
use denoising::VideoDenoiser;
use color_grading::ColorGrader;
use comparison::VideoComparator;

/// Configuration for advanced video features
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdvancedVideoConfig {
    /// Video stabilization settings
    pub stabilization: StabilizationConfig,
    
    /// Frame interpolation settings
    pub frame_interpolation: FrameInterpolationConfig,
    
    /// Video denoising settings
    pub denoising: DenoisingConfig,
    
    /// Color grading settings
    pub color_grading: ColorGradingConfig,
    
    /// Video comparison settings
    pub comparison: ComparisonConfig,
}

impl Default for AdvancedVideoConfig {
    fn default() -> Self {
        Self {
            stabilization: StabilizationConfig::default(),
            frame_interpolation: FrameInterpolationConfig::default(),
            denoising: DenoisingConfig::default(),
            color_grading: ColorGradingConfig::default(),
            comparison: ComparisonConfig::default(),
        }
    }
}

/// Video stabilization configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StabilizationConfig {
    /// Enable video stabilization
    pub enabled: bool,
    
    /// Stabilization strength (0.0 - 1.0)
    pub strength: f32,
    
    /// Motion analysis method
    pub motion_analysis: MotionAnalysisMethod,
    
    /// Rolling shutter correction
    pub rolling_shutter_correction: bool,
    
    /// Crop to stabilize (may reduce resolution)
    pub crop_to_stabilize: bool,
    
    /// Maximum crop percentage (0.0 - 1.0)
    pub max_crop: f32,
}

impl Default for StabilizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strength: 0.5,
            motion_analysis: MotionAnalysisMethod::OpticalFlow,
            rolling_shutter_correction: false,
            crop_to_stabilize: true,
            max_crop: 0.1,
        }
    }
}

/// Motion analysis method
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum MotionAnalysisMethod {
    /// Optical flow-based motion estimation
    OpticalFlow,
    /// Feature point tracking
    FeatureTracking,
    /// Global motion estimation
    GlobalMotion,
    /// Hybrid approach
    Hybrid,
}

/// Frame interpolation configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameInterpolationConfig {
    /// Enable frame interpolation
    pub enabled: bool,
    
    /// Target frame rate (fps)
    pub target_fps: u32,
    
    /// Interpolation method
    pub method: InterpolationMethod,
    
    /// AI model for interpolation
    pub ai_model: String,
    
    /// Quality vs speed tradeoff (0.0 - 1.0)
    pub quality: f32,
}

impl Default for FrameInterpolationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            target_fps: 60,
            method: InterpolationMethod::AI,
            ai_model: "default".to_string(),
            quality: 0.8,
        }
    }
}

/// Frame interpolation method
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum InterpolationMethod {
    /// Linear interpolation
    Linear,
    /// Motion-compensated interpolation
    MotionCompensated,
    /// AI-based interpolation
    AI,
    /// Optical flow interpolation
    OpticalFlow,
}

/// Video denoising configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DenoisingConfig {
    /// Enable video denoising
    pub enabled: bool,
    
    /// Denoising strength (0.0 - 1.0)
    pub strength: f32,
    
    /// Denoising method
    pub method: DenoisingMethod,
    
    /// Temporal denoising
    pub temporal_denoising: bool,
    
    /// Spatial denoising
    pub spatial_denoising: bool,
    
    /// Preserve detail level (0.0 - 1.0)
    pub detail_preservation: f32,
}

impl Default for DenoisingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strength: 0.5,
            method: DenoisingMethod::AI,
            temporal_denoising: true,
            spatial_denoising: true,
            detail_preservation: 0.7,
        }
    }
}

/// Video denoising method
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum DenoisingMethod {
    /// Bilateral filter
    Bilateral,
    /// Non-local means
    NonLocalMeans,
    /// Wavelet denoising
    Wavelet,
    /// AI-based denoising
    AI,
}

/// Color grading configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorGradingConfig {
    /// Enable color grading
    pub enabled: bool,
    
    /// Active preset
    pub preset: String,
    
    /// Custom adjustments
    pub adjustments: ColorAdjustments,
    
    /// LUT (Look-Up Table) path
    pub lut_path: Option<String>,
}

impl Default for ColorGradingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            preset: "neutral".to_string(),
            adjustments: ColorAdjustments::default(),
            lut_path: None,
        }
    }
}

/// Color adjustments
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorAdjustments {
    /// Brightness (-1.0 to 1.0)
    pub brightness: f32,
    
    /// Contrast (-1.0 to 1.0)
    pub contrast: f32,
    
    /// Saturation (-1.0 to 1.0)
    pub saturation: f32,
    
    /// Hue shift (-180.0 to 180.0)
    pub hue: f32,
    
    /// Temperature (-100.0 to 100.0)
    pub temperature: f32,
    
    /// Tint (-100.0 to 100.0)
    pub tint: f32,
    
    /// Vibrance (-1.0 to 1.0)
    pub vibrance: f32,
    
    /// Exposure (-2.0 to 2.0)
    pub exposure: f32,
    
    /// Highlights (-1.0 to 1.0)
    pub highlights: f32,
    
    /// Shadows (-1.0 to 1.0)
    pub shadows: f32,
    
    /// Whites (-1.0 to 1.0)
    pub whites: f32,
    
    /// Blacks (-1.0 to 1.0)
    pub blacks: f32,
}

impl Default for ColorAdjustments {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
            hue: 0.0,
            temperature: 0.0,
            tint: 0.0,
            vibrance: 0.0,
            exposure: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
        }
    }
}

/// Video comparison configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComparisonConfig {
    /// Enable video comparison
    pub enabled: bool,
    
    /// Comparison method
    pub method: ComparisonMethod,
    
    /// Show difference visualization
    pub show_difference: bool,
    
    /// Difference threshold (0.0 - 1.0)
    pub difference_threshold: f32,
}

impl Default for ComparisonConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: ComparisonMethod::PSNR,
            show_difference: true,
            difference_threshold: 0.1,
        }
    }
}

/// Video comparison method
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ComparisonMethod {
    /// Peak Signal-to-Noise Ratio
    PSNR,
    /// Structural Similarity Index
    SSIM,
    /// Mean Squared Error
    MSE,
    /// Visual Difference
    VisualDifference,
}

/// Advanced video engine
pub struct AdvancedVideoEngine {
    config: Arc<RwLock<AdvancedVideoConfig>>,
    
    stabilizer: Arc<VideoStabilizer>,
    interpolator: Arc<FrameInterpolator>,
    denoiser: Arc<VideoDenoiser>,
    color_grader: Arc<ColorGrader>,
    comparator: Arc<VideoComparator>,
}

impl AdvancedVideoEngine {
    /// Create a new advanced video engine with default config
    pub fn new() -> Result<Self> {
        Self::new_with_config(AdvancedVideoConfig::default())
    }
    
    /// Create a new advanced video engine
    pub fn new_with_config(config: AdvancedVideoConfig) -> Result<Self> {
        info!("Initializing advanced video engine");
        
        let config = Arc::new(RwLock::new(config));
        
        let stabilizer = Arc::new(VideoStabilizer::new()?);
        let interpolator = Arc::new(FrameInterpolator::new()?);
        let denoiser = Arc::new(VideoDenoiser::new()?);
        let color_grader = Arc::new(ColorGrader::new()?);
        let comparator = Arc::new(VideoComparator::new()?);
        
        Ok(Self {
            config,
            stabilizer,
            interpolator,
            denoiser,
            color_grader,
            comparator,
        })
    }
    
    /// Get the video stabilizer
    pub fn stabilizer(&self) -> &VideoStabilizer {
        &self.stabilizer
    }
    
    /// Get the frame interpolator
    pub fn interpolator(&self) -> &FrameInterpolator {
        &self.interpolator
    }
    
    /// Get the video denoiser
    pub fn denoiser(&self) -> &VideoDenoiser {
        &self.denoiser
    }
    
    /// Get the color grader
    pub fn color_grader(&self) -> &ColorGrader {
        &self.color_grader
    }
    
    /// Get the video comparator
    pub fn comparator(&self) -> &VideoComparator {
        &self.comparator
    }
    
    /// Update configuration
    pub async fn update_config(&self, config: AdvancedVideoConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Advanced video configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> AdvancedVideoConfig {
        self.config.read().await.clone()
    }
    
    /// Process video frame with all enabled features
    pub async fn process_frame(&self, frame: &image::RgbImage) -> Result<image::RgbImage> {
        let config = self.config.read().await;
        let mut processed = frame.clone();
        
        // Apply denoising if enabled
        if config.denoising.enabled {
            processed = self.denoiser.process(&processed).await?;
        }
        
        // Apply color grading if enabled
        if config.color_grading.enabled {
            processed = self.color_grader.apply_preset(&processed, &config.color_grading.preset).await?;
        }
        
        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_advanced_video_engine_creation() {
        let config = AdvancedVideoConfig::default();
        let engine = AdvancedVideoEngine::new(config).unwrap();
        assert!(engine.stabilizer().is_initialized());
        assert!(engine.interpolator().is_initialized());
        assert!(engine.denoiser().is_initialized());
        assert!(engine.color_grader().is_initialized());
        assert!(engine.comparator().is_initialized());
    }
    
    #[tokio::test]
    async fn test_config_update() {
        let config = AdvancedVideoConfig::default();
        let engine = AdvancedVideoEngine::new(config).unwrap();
        
        let mut new_config = engine.get_config().await;
        new_config.stabilization.enabled = true;
        engine.update_config(new_config).await.unwrap();
        
        let updated = engine.get_config().await;
        assert!(updated.stabilization.enabled);
    }
}