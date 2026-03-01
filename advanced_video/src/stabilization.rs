//! Video Stabilization Engine
//! 
//! Provides video stabilization using motion analysis and camera shake reduction.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use image::{RgbImage, Rgb, Pixel};
use nalgebra::{DMatrix, DVector, Vector2};

use crate::{MotionAnalysisMethod, StabilizationConfig};

/// Video stabilizer
pub struct VideoStabilizer {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Stabilization configuration
    config: Arc<RwLock<StabilizationConfig>>,
    
    /// Previous frame for motion analysis
    previous_frame: Arc<RwLock<Option<RgbImage>>>,
    
    /// Motion vectors
    motion_vectors: Arc<RwLock<Vec<MotionVector>>>,
    
    /// Camera path
    camera_path: Arc<RwLock<Vec<CameraTransform>>>,
    
    /// Smoothed camera path
    smoothed_path: Arc<RwLock<Vec<CameraTransform>>>,
}

/// Motion vector
#[derive(Debug, Clone)]
pub struct MotionVector {
    /// X displacement
    pub dx: f32,
    
    /// Y displacement
    pub dy: f32,
    
    /// Confidence (0.0 - 1.0)
    pub confidence: f32,
    
    /// Position in frame
    pub x: usize,
    pub y: usize,
}

/// Camera transform
#[derive(Debug, Clone)]
pub struct CameraTransform {
    /// Translation X
    pub tx: f32,
    
    /// Translation Y
    pub ty: f32,
    
    /// Rotation (radians)
    pub rotation: f32,
    
    /// Scale
    pub scale: f32,
    
    /// Frame index
    pub frame_index: usize,
}

impl Default for CameraTransform {
    fn default() -> Self {
        Self {
            tx: 0.0,
            ty: 0.0,
            rotation: 0.0,
            scale: 1.0,
            frame_index: 0,
        }
    }
}

/// Stabilization result
#[derive(Debug, Clone)]
pub struct StabilizationResult {
    /// Stabilized frame
    pub frame: RgbImage,
    
    /// Camera transform applied
    pub transform: CameraTransform,
    
    /// Crop rectangle
    pub crop: CropRect,
}

/// Crop rectangle
#[derive(Debug, Clone)]
pub struct CropRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl VideoStabilizer {
    /// Create a new video stabilizer
    pub fn new() -> Result<Self> {
        info!("Initializing video stabilizer");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(StabilizationConfig::default())),
            previous_frame: Arc::new(RwLock::new(None)),
            motion_vectors: Arc::new(RwLock::new(Vec::new())),
            camera_path: Arc::new(RwLock::new(Vec::new())),
            smoothed_path: Arc::new(RwLock::new(Vec::new())),
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
    
    /// Set stabilization configuration
    pub async fn set_config(&self, config: StabilizationConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Stabilization configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> StabilizationConfig {
        self.config.read().await.clone()
    }
    
    /// Stabilize a video frame
    pub async fn stabilize(&self, frame: &RgbImage) -> Result<StabilizationResult> {
        let config = self.config.read().await;
        
        if !config.enabled {
            return Ok(StabilizationResult {
                frame: frame.clone(),
                transform: CameraTransform::default(),
                crop: CropRect {
                    x: 0,
                    y: 0,
                    width: frame.width(),
                    height: frame.height(),
                },
            });
        }
        
        // Analyze motion
        let motion = self.analyze_motion(frame).await?;
        
        // Smooth camera path
        let smoothed_transform = self.smooth_motion(&motion).await?;
        
        // Apply stabilization
        let stabilized = self.apply_stabilization(frame, &smoothed_transform).await?;
        
        Ok(stabilized)
    }
    
    /// Analyze motion between frames
    async fn analyze_motion(&self, frame: &RgbImage) -> Result<CameraTransform> {
        let config = self.config.read().await;
        let previous = self.previous_frame.read().await;
        
        if previous.is_none() {
            // First frame, no motion
            return Ok(CameraTransform::default());
        }
        
        let prev_frame = previous.as_ref().unwrap();
        
        match config.motion_analysis {
            MotionAnalysisMethod::OpticalFlow => {
                self.analyze_optical_flow(prev_frame, frame).await
            }
            MotionAnalysisMethod::FeatureTracking => {
                self.analyze_feature_tracking(prev_frame, frame).await
            }
            MotionAnalysisMethod::GlobalMotion => {
                self.analyze_global_motion(prev_frame, frame).await
            }
            MotionAnalysisMethod::Hybrid => {
                self.analyze_hybrid(prev_frame, frame).await
            }
        }
    }
    
    /// Analyze motion using optical flow
    async fn analyze_optical_flow(&self, prev_frame: &RgbImage, curr_frame: &RgbImage) -> Result<CameraTransform> {
        debug!("Analyzing motion using optical flow");
        
        // Simplified optical flow - block matching
        let block_size = 16;
        let search_range = 16;
        
        let mut total_dx = 0.0;
        let mut total_dy = 0.0;
        let mut count = 0;
        
        let width = prev_frame.width() as usize;
        let height = prev_frame.height() as usize;
        
        for y in (0..height).step_by(block_size) {
            for x in (0..width).step_by(block_size) {
                // Find best matching block
                let (dx, dy) = self.find_best_block(prev_frame, curr_frame, x, y, block_size, search_range);
                
                total_dx += dx;
                total_dy += dy;
                count += 1;
            }
        }
        
        if count > 0 {
            total_dx /= count as f32;
            total_dy /= count as f32;
        }
        
        Ok(CameraTransform {
            tx: total_dx,
            ty: total_dy,
            rotation: 0.0,
            scale: 1.0,
            frame_index: 0,
        })
    }
    
    /// Find best matching block
    fn find_best_block(&self, prev: &RgbImage, curr: &RgbImage, x: usize, y: usize, block_size: usize, search_range: usize) -> (f32, f32) {
        let mut best_dx = 0.0;
        let mut best_dy = 0.0;
        let mut best_error = f32::MAX;
        
        let width = prev.width() as usize;
        let height = prev.height() as usize;
        
        for dy in -(search_range as isize)..=(search_range as isize) {
            for dx in -(search_range as isize)..=(search_range as isize) {
                let target_x = x as isize + dx;
                let target_y = y as isize + dy;
                
                if target_x < 0 || target_y < 0 || target_x + block_size as isize > width as isize || target_y + block_size as isize > height as isize {
                    continue;
                }
                
                let error = self.calculate_block_error(prev, curr, x, y, target_x as usize, target_y as usize, block_size);
                
                if error < best_error {
                    best_error = error;
                    best_dx = dx as f32;
                    best_dy = dy as f32;
                }
            }
        }
        
        (best_dx, best_dy)
    }
    
    /// Calculate block error (sum of absolute differences)
    fn calculate_block_error(&self, prev: &RgbImage, curr: &RgbImage, x1: usize, y1: usize, x2: usize, y2: usize, block_size: usize) -> f32 {
        let mut error = 0.0;
        
        for dy in 0..block_size {
            for dx in 0..block_size {
                let p1 = prev.get_pixel(x1 + dx, y1 + dy);
                let p2 = curr.get_pixel(x2 + dx, y2 + dy);
                
                error += (p1[0] as f32 - p2[0] as f32).abs();
                error += (p1[1] as f32 - p2[1] as f32).abs();
                error += (p1[2] as f32 - p2[2] as f32).abs();
            }
        }
        
        error
    }
    
    /// Analyze motion using feature tracking
    async fn analyze_feature_tracking(&self, prev_frame: &RgbImage, curr_frame: &RgbImage) -> Result<CameraTransform> {
        debug!("Analyzing motion using feature tracking");
        
        // Simplified feature tracking - use corners
        let features = self.detect_features(prev_frame).await?;
        let motion = self.track_features(prev_frame, curr_frame, &features).await?;
        
        Ok(motion)
    }
    
    /// Detect features in frame
    async fn detect_features(&self, frame: &RgbImage) -> Result<Vec<(usize, usize)>> {
        // Simplified corner detection
        let mut features = Vec::new();
        let threshold = 100.0;
        
        let width = frame.width() as usize;
        let height = frame.height() as usize;
        
        for y in 10..height-10 {
            for x in 10..width-10 {
                // Calculate corner response
                let response = self.calculate_corner_response(frame, x, y);
                
                if response > threshold {
                    features.push((x, y));
                }
            }
        }
        
        // Limit number of features
        features.truncate(100);
        
        Ok(features)
    }
    
    /// Calculate corner response
    fn calculate_corner_response(&self, frame: &RgbImage, x: usize, y: usize) -> f32 {
        // Simplified corner detection
        let mut gx = 0.0;
        let mut gy = 0.0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let pixel = frame.get_pixel(x + dx as usize, y + dy as usize);
                let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                
                gx += intensity * dx as f32;
                gy += intensity * dy as f32;
            }
        }
        
        (gx * gx + gy * gy).sqrt()
    }
    
    /// Track features between frames
    async fn track_features(&self, prev: &RgbImage, curr: &RgbImage, features: &[(usize, usize)]) -> Result<CameraTransform> {
        let mut total_dx = 0.0;
        let mut total_dy = 0.0;
        let mut count = 0;
        
        for &(x, y) in features {
            let (dx, dy) = self.find_best_block(prev, curr, x, y, 8, 16);
            total_dx += dx;
            total_dy += dy;
            count += 1;
        }
        
        if count > 0 {
            total_dx /= count as f32;
            total_dy /= count as f32;
        }
        
        Ok(CameraTransform {
            tx: total_dx,
            ty: total_dy,
            rotation: 0.0,
            scale: 1.0,
            frame_index: 0,
        })
    }
    
    /// Analyze global motion
    async fn analyze_global_motion(&self, prev_frame: &RgbImage, curr_frame: &RgbImage) -> Result<CameraTransform> {
        debug!("Analyzing global motion");
        
        // Use optical flow for global motion
        self.analyze_optical_flow(prev_frame, curr_frame).await
    }
    
    /// Analyze motion using hybrid approach
    async fn analyze_hybrid(&self, prev_frame: &RgbImage, curr_frame: &RgbImage) -> Result<CameraTransform> {
        debug!("Analyzing motion using hybrid approach");
        
        // Combine optical flow and feature tracking
        let optical_flow = self.analyze_optical_flow(prev_frame, curr_frame).await?;
        let feature_tracking = self.analyze_feature_tracking(prev_frame, curr_frame).await?;
        
        // Average the results
        Ok(CameraTransform {
            tx: (optical_flow.tx + feature_tracking.tx) / 2.0,
            ty: (optical_flow.ty + feature_tracking.ty) / 2.0,
            rotation: (optical_flow.rotation + feature_tracking.rotation) / 2.0,
            scale: (optical_flow.scale + feature_tracking.scale) / 2.0,
            frame_index: 0,
        })
    }
    
    /// Smooth motion using low-pass filter
    async fn smooth_motion(&self, motion: &CameraTransform) -> Result<CameraTransform> {
        let config = self.config.read().await;
        let strength = config.strength;
        
        let mut path = self.camera_path.write().await;
        let mut smoothed = self.smoothed_path.write().await;
        
        // Add to camera path
        path.push(motion.clone());
        
        // Keep only last 30 frames
        if path.len() > 30 {
            path.remove(0);
        }
        
        // Apply smoothing
        if path.len() < 3 {
            return Ok(motion.clone());
        }
        
        // Simple moving average
        let window_size = (path.len() as f32 * strength) as usize;
        let window_size = window_size.max(1).min(path.len());
        
        let mut smoothed_tx = 0.0;
        let mut smoothed_ty = 0.0;
        
        for i in path.len().saturating_sub(window_size)..path.len() {
            smoothed_tx += path[i].tx;
            smoothed_ty += path[i].ty;
        }
        
        smoothed_tx /= window_size as f32;
        smoothed_ty /= window_size as f32;
        
        let smoothed_transform = CameraTransform {
            tx: smoothed_tx,
            ty: smoothed_ty,
            rotation: motion.rotation,
            scale: motion.scale,
            frame_index: motion.frame_index,
        };
        
        smoothed.push(smoothed_transform.clone());
        
        Ok(smoothed_transform)
    }
    
    /// Apply stabilization to frame
    async fn apply_stabilization(&self, frame: &RgbImage, transform: &CameraTransform) -> Result<StabilizationResult> {
        let config = self.config.read().await;
        
        // Calculate crop rectangle
        let crop = self.calculate_crop(frame, transform, &config).await?;
        
        // Apply transform and crop
        let stabilized = self.transform_and_crop(frame, transform, &crop).await?;
        
        // Store current frame for next iteration
        *self.previous_frame.write().await = Some(frame.clone());
        
        Ok(StabilizationResult {
            frame: stabilized,
            transform: transform.clone(),
            crop,
        })
    }
    
    /// Calculate crop rectangle
    async fn calculate_crop(&self, frame: &RgbImage, transform: &CameraTransform, config: &StabilizationConfig) -> Result<CropRect> {
        let width = frame.width() as usize;
        let height = frame.height() as usize;
        
        if !config.crop_to_stabilize {
            return Ok(CropRect {
                x: 0,
                y: 0,
                width,
                height,
            });
        }
        
        // Calculate required crop based on motion
        let max_crop_x = (width as f32 * config.max_crop) as usize;
        let max_crop_y = (height as f32 * config.max_crop) as usize;
        
        let crop_x = transform.tx.abs() as usize;
        let crop_y = transform.ty.abs() as usize;
        
        let crop_x = crop_x.min(max_crop_x);
        let crop_y = crop_y.min(max_crop_y);
        
        Ok(CropRect {
            x: crop_x,
            y: crop_y,
            width: width - 2 * crop_x,
            height: height - 2 * crop_y,
        })
    }
    
    /// Transform and crop frame
    async fn transform_and_crop(&self, frame: &RgbImage, transform: &CameraTransform, crop: &CropRect) -> Result<RgbImage> {
        let mut output = RgbImage::new(crop.width as u32, crop.height as u32);
        
        // Apply translation and crop
        for y in 0..crop.height {
            for x in 0..crop.width {
                let src_x = (x + crop.x) as f32 - transform.tx;
                let src_y = (y + crop.y) as f32 - transform.ty;
                
                // Bilinear interpolation
                let pixel = self.bilinear_interpolate(frame, src_x, src_y);
                output.put_pixel(x as u32, y as u32, pixel);
            }
        }
        
        Ok(output)
    }
    
    /// Bilinear interpolation
    fn bilinear_interpolate(&self, frame: &RgbImage, x: f32, y: f32) -> Rgb<u8> {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(frame.width() as usize - 1);
        let y1 = (y0 + 1).min(frame.height() as usize - 1);
        
        let fx = x - x.floor();
        let fy = y - y.floor();
        
        let p00 = frame.get_pixel(x0 as u32, y0 as u32);
        let p10 = frame.get_pixel(x1 as u32, y0 as u32);
        let p01 = frame.get_pixel(x0 as u32, y1 as u32);
        let p11 = frame.get_pixel(x1 as u32, y1 as u32);
        
        let r = (p00[0] as f32 * (1.0 - fx) * (1.0 - fy) +
                p10[0] as f32 * fx * (1.0 - fy) +
                p01[0] as f32 * (1.0 - fx) * fy +
                p11[0] as f32 * fx * fy) as u8;
        
        let g = (p00[1] as f32 * (1.0 - fx) * (1.0 - fy) +
                p10[1] as f32 * fx * (1.0 - fy) +
                p01[1] as f32 * (1.0 - fx) * fy +
                p11[1] as f32 * fx * fy) as u8;
        
        let b = (p00[2] as f32 * (1.0 - fx) * (1.0 - fy) +
                p10[2] as f32 * fx * (1.0 - fy) +
                p01[2] as f32 * (1.0 - fx) * fy +
                p11[2] as f32 * fx * fy) as u8;
        
        Rgb([r, g, b])
    }
    
    /// Reset stabilizer state
    pub async fn reset(&self) -> Result<()> {
        *self.previous_frame.write().await = None;
        *self.motion_vectors.write().await = Vec::new();
        *self.camera_path.write().await = Vec::new();
        *self.smoothed_path.write().await = Vec::new();
        info!("Stabilizer reset");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_stabilizer_creation() {
        let stabilizer = VideoStabilizer::new().unwrap();
        assert!(stabilizer.is_initialized());
    }
    
    #[tokio::test]
    async fn test_stabilization() {
        let stabilizer = VideoStabilizer::new().unwrap();
        
        let config = StabilizationConfig {
            enabled: true,
            strength: 0.5,
            motion_analysis: MotionAnalysisMethod::OpticalFlow,
            rolling_shutter_correction: false,
            crop_to_stabilize: true,
            max_crop: 0.1,
        };
        stabilizer.set_config(config).await.unwrap();
        
        let frame = RgbImage::new(640, 480);
        let result = stabilizer.stabilize(&frame).await.unwrap();
        
        assert_eq!(result.frame.width(), frame.width());
        assert_eq!(result.frame.height(), frame.height());
    }
    
    #[tokio::test]
    async fn test_motion_analysis() {
        let stabilizer = VideoStabilizer::new().unwrap();
        
        let frame1 = RgbImage::new(640, 480);
        let frame2 = RgbImage::new(640, 480);
        
        // Store first frame
        *stabilizer.previous_frame.write().await = Some(frame1.clone());
        
        let motion = stabilizer.analyze_motion(&frame2).await.unwrap();
        assert_eq!(motion.tx, 0.0);
        assert_eq!(motion.ty, 0.0);
    }
}