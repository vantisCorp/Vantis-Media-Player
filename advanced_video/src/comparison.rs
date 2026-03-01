//! Video Comparison Engine
//! 
//! Provides video comparison tools for quality assessment and difference visualization.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use image::{RgbImage, Rgb, Pixel};

use crate::{ComparisonMethod, ComparisonConfig};

/// Video comparator
pub struct VideoComparator {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Comparison configuration
    config: Arc<RwLock<ComparisonConfig>>,
}

/// Comparison result
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    /// Similarity score (0.0 - 1.0)
    pub similarity: f32,
    
    /// Difference score (0.0 - 1.0)
    pub difference: f32,
    
    /// PSNR (Peak Signal-to-Noise Ratio) in dB
    pub psnr: f32,
    
    /// SSIM (Structural Similarity Index)
    pub ssim: f32,
    
    /// MSE (Mean Squared Error)
    pub mse: f32,
    
    /// Difference image
    pub difference_image: Option<RgbImage>,
}

impl VideoComparator {
    /// Create a new video comparator
    pub fn new() -> Result<Self> {
        info!("Initializing video comparator");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(ComparisonConfig::default())),
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
    
    /// Set comparison configuration
    pub async fn set_config(&self, config: ComparisonConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Comparison configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> ComparisonConfig {
        self.config.read().await.clone()
    }
    
    /// Compare two frames
    pub async fn compare(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<ComparisonResult> {
        let config = self.config.read().await;
        
        // Ensure frames are same size
        if frame1.dimensions() != frame2.dimensions() {
            return Err(anyhow::anyhow!("Frames must have the same dimensions"));
        }
        
        // Calculate metrics
        let mse = self.calculate_mse(frame1, frame2)?;
        let psnr = self.calculate_psnr(mse);
        let ssim = self.calculate_ssim(frame1, frame2)?;
        let difference = self.calculate_difference(frame1, frame2)?;
        
        // Generate difference image if requested
        let difference_image = if config.generate_difference_image {
            Some(self.generate_difference_image(frame1, frame2)?)
        } else {
            None
        };
        
        // Calculate overall similarity
        let similarity = match config.method {
            ComparisonMethod::PSNR => {
                // Normalize PSNR to 0-1 range (typical range 20-50 dB)
                ((psnr - 20.0) / 30.0).clamp(0.0, 1.0)
            }
            ComparisonMethod::SSIM => {
                ssim
            }
            ComparisonMethod::MSE => {
                // Normalize MSE to similarity (lower is better)
                1.0 - (mse / 255.0).min(1.0)
            }
            ComparisonMethod::VisualDifference => {
                1.0 - difference
            }
        };
        
        Ok(ComparisonResult {
            similarity,
            difference,
            psnr,
            ssim,
            mse,
            difference_image,
        })
    }
    
    /// Calculate Mean Squared Error
    fn calculate_mse(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<f32> {
        let mut sum_squared_error = 0.0;
        let pixel_count = (frame1.width() * frame1.height()) as f32;
        
        for y in 0..frame1.height() {
            for x in 0..frame1.width() {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);
                
                let dr = pixel1[0] as f32 - pixel2[0] as f32;
                let dg = pixel1[1] as f32 - pixel2[1] as f32;
                let db = pixel1[2] as f32 - pixel2[2] as f32;
                
                sum_squared_error += dr * dr + dg * dg + db * db;
            }
        }
        
        Ok(sum_squared_error / (3.0 * pixel_count))
    }
    
    /// Calculate Peak Signal-to-Noise Ratio
    fn calculate_psnr(&self, mse: f32) -> f32 {
        if mse == 0.0 {
            return f32::INFINITY;
        }
        
        let max_pixel = 255.0;
        10.0 * (max_pixel * max_pixel / mse).log10()
    }
    
    /// Calculate Structural Similarity Index
    fn calculate_ssim(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<f32> {
        // Simplified SSIM calculation
        let c1 = 6.5025; // (0.01 * 255)^2
        let c2 = 58.5225; // (0.03 * 255)^2
        
        let mut sum_mu1 = 0.0;
        let mut sum_mu2 = 0.0;
        let mut sum_mu1_sq = 0.0;
        let mut sum_mu2_sq = 0.0;
        let mut sum_mu1_mu2 = 0.0;
        let mut sum_sigma1_sq = 0.0;
        let mut sum_sigma2_sq = 0.0;
        let mut sum_sigma12 = 0.0;
        
        let pixel_count = (frame1.width() * frame1.height()) as f32;
        
        // Calculate means
        for y in 0..frame1.height() {
            for x in 0..frame1.width() {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);
                
                let gray1 = (pixel1[0] as f32 + pixel1[1] as f32 + pixel1[2] as f32) / 3.0;
                let gray2 = (pixel2[0] as f32 + pixel2[1] as f32 + pixel2[2] as f32) / 3.0;
                
                sum_mu1 += gray1;
                sum_mu2 += gray2;
                sum_mu1_sq += gray1 * gray1;
                sum_mu2_sq += gray2 * gray2;
                sum_mu1_mu2 += gray1 * gray2;
            }
        }
        
        let mu1 = sum_mu1 / pixel_count;
        let mu2 = sum_mu2 / pixel_count;
        let mu1_sq = sum_mu1_sq / pixel_count;
        let mu2_sq = sum_mu2_sq / pixel_count;
        let mu1_mu2 = sum_mu1_mu2 / pixel_count;
        
        // Calculate variances and covariance
        for y in 0..frame1.height() {
            for x in 0..frame1.width() {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);
                
                let gray1 = (pixel1[0] as f32 + pixel1[1] as f32 + pixel1[2] as f32) / 3.0;
                let gray2 = (pixel2[0] as f32 + pixel2[1] as f32 + pixel2[2] as f32) / 3.0;
                
                sum_sigma1_sq += (gray1 - mu1) * (gray1 - mu1);
                sum_sigma2_sq += (gray2 - mu2) * (gray2 - mu2);
                sum_sigma12 += (gray1 - mu1) * (gray2 - mu2);
            }
        }
        
        let sigma1_sq = sum_sigma1_sq / pixel_count;
        let sigma2_sq = sum_sigma2_sq / pixel_count;
        let sigma12 = sum_sigma12 / pixel_count;
        
        // Calculate SSIM
        let numerator = (2.0 * mu1 * mu2 + c1) * (2.0 * sigma12 + c2);
        let denominator = (mu1_sq + mu2_sq + c1) * (sigma1_sq + sigma2_sq + c2);
        
        Ok(numerator / denominator)
    }
    
    /// Calculate visual difference
    fn calculate_difference(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<f32> {
        let mut total_difference = 0.0;
        let pixel_count = (frame1.width() * frame1.height()) as f32;
        
        for y in 0..frame1.height() {
            for x in 0..frame1.width() {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);
                
                let dr = (pixel1[0] as f32 - pixel2[0] as f32).abs();
                let dg = (pixel1[1] as f32 - pixel2[1] as f32).abs();
                let db = (pixel1[2] as f32 - pixel2[2] as f32).abs();
                
                total_difference += (dr + dg + db) / 3.0;
            }
        }
        
        Ok(total_difference / (255.0 * pixel_count))
    }
    
    /// Generate difference image
    fn generate_difference_image(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame1.width(), frame2.height());
        
        for y in 0..frame1.height() {
            for x in 0..frame1.width() {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);
                
                // Calculate absolute difference
                let dr = (pixel1[0] as f32 - pixel2[0] as f32).abs();
                let dg = (pixel1[1] as f32 - pixel2[1] as f32).abs();
                let db = (pixel1[2] as f32 - pixel2[2] as f32).abs();
                
                // Scale difference for visibility
                let scale = 4.0;
                let r = (dr * scale).clamp(0.0, 255.0) as u8;
                let g = (dg * scale).clamp(0.0, 255.0) as u8;
                let b = (db * scale).clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identical_frames() {
        let comparator = VideoComparator::new().unwrap();
        
        let frame1 = RgbImage::new(100, 100);
        let frame2 = frame1.clone();
        
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(comparator.compare(&frame1, &frame2))
            .unwrap();
        
        assert!(result.similarity > 0.99);
        assert!(result.mse < 0.01);
    }
    
    #[test]
    fn test_different_frames() {
        let comparator = VideoComparator::new().unwrap();
        
        let mut frame1 = RgbImage::new(100, 100);
        let mut frame2 = RgbImage::new(100, 100);
        
        // Fill frame1 with white
        for pixel in frame1.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
        
        // Fill frame2 with black
        for pixel in frame2.pixels_mut() {
            *pixel = Rgb([0, 0, 0]);
        }
        
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(comparator.compare(&frame1, &frame2))
            .unwrap();
        
        assert!(result.similarity < 0.1);
        assert!(result.mse > 10000.0);
    }
}