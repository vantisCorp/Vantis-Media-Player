//! Video enhancement module using AI
//! 
//! Provides content-aware video enhancement including:
//! - Super-resolution (upscaling)
//! - Denoising
//! - Deblurring
//! - Color enhancement
//! - Sharpness improvement

use crate::{AIConfig, AIError, AIResult};
use crate::models::{AIModel, ModelType};
use crate::utils::{TensorOps, FeatureExtractor};
use image::{DynamicImage, ImageBuffer, Rgb};
use ndarray::{Array4, Array3};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Video enhancement types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnhancementType {
    /// Super-resolution upscaling
    SuperResolution { scale: u32 },
    
    /// Denoising
    Denoising { strength: f32 },
    
    /// Deblurring
    Deblurring { strength: f32 },
    
    /// Color enhancement
    ColorEnhancement { saturation: f32, contrast: f32 },
    
    /// Sharpness improvement
    Sharpening { amount: f32 },
    
    /// Combined enhancement
    Combined {
        upscale: Option<u32>,
        denoise: Option<f32>,
        deblur: Option<f32>,
        color: Option<(f32, f32)>,
        sharpen: Option<f32>,
    },
}

impl EnhancementType {
    /// Get the enhancement name
    pub fn name(&self) -> &str {
        match self {
            EnhancementType::SuperResolution { .. } => "Super Resolution",
            EnhancementType::Denoising { .. } => "Denoising",
            EnhancementType::Deblurring { .. } => "Deblurring",
            EnhancementType::ColorEnhancement { .. } => "Color Enhancement",
            EnhancementType::Sharpening { .. } => "Sharpening",
            EnhancementType::Combined { .. } => "Combined Enhancement",
        }
    }
}

/// Video enhancement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementConfig {
    /// Enhancement type
    pub enhancement_type: EnhancementType,
    
    /// Enable GPU acceleration
    pub enable_gpu: bool,
    
    /// Batch size for processing
    pub batch_size: usize,
    
    /// Enable temporal consistency
    pub enable_temporal_consistency: bool,
    
    /// Temporal window size (number of frames)
    pub temporal_window_size: usize,
    
    /// Quality preset
    pub quality: QualityPreset,
}

impl Default for EnhancementConfig {
    fn default() -> Self {
        Self {
            enhancement_type: EnhancementType::SuperResolution { scale: 2 },
            enable_gpu: true,
            batch_size: 4,
            enable_temporal_consistency: true,
            temporal_window_size: 5,
            quality: QualityPreset::Balanced,
        }
    }
}

/// Quality presets for enhancement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityPreset {
    /// Fast processing, lower quality
    Fast,
    
    /// Balanced speed and quality
    Balanced,
    
    /// Best quality, slower processing
    Quality,
    
    /// Custom settings
    Custom,
}

impl QualityPreset {
    /// Get the recommended batch size
    pub fn batch_size(&self) -> usize {
        match self {
            QualityPreset::Fast => 8,
            QualityPreset::Balanced => 4,
            QualityPreset::Quality => 1,
            QualityPreset::Custom => 4,
        }
    }
}

/// Video enhancer
pub struct VideoEnhancer {
    config: EnhancementConfig,
    model: Option<AIModel>,
    feature_extractor: FeatureExtractor,
    temporal_buffer: Vec<DynamicImage>,
}

impl VideoEnhancer {
    /// Create a new video enhancer
    pub fn new(ai_config: AIConfig) -> AIResult<Self> {
        let config = EnhancementConfig::default();
        let feature_extractor = FeatureExtractor::new(ai_config.clone())?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            temporal_buffer: Vec::new(),
        })
    }
    
    /// Create a new video enhancer with custom configuration
    pub fn with_config(config: EnhancementConfig, ai_config: AIConfig) -> AIResult<Self> {
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            temporal_buffer: Vec::new(),
        })
    }
    
    /// Load the enhancement model
    pub fn load_model(&mut self) -> AIResult<()> {
        let model_type = match self.config.enhancement_type {
            EnhancementType::SuperResolution { scale } => {
                ModelType::SuperResolution { scale }
            }
            EnhancementType::Denoising { .. } => ModelType::Denoising,
            EnhancementType::Deblurring { .. } => ModelType::Deblurring,
            EnhancementType::ColorEnhancement { .. } => ModelType::ColorEnhancement,
            EnhancementType::Sharpening { .. } => ModelType::Sharpening,
            EnhancementType::Combined { .. } => ModelType::CombinedEnhancement,
        };
        
        self.model = Some(AIModel::load(model_type)?);
        Ok(())
    }
    
    /// Enhance a single frame
    pub fn enhance_frame(&mut self, frame: &DynamicImage) -> AIResult<DynamicImage> {
        // Load model if not loaded
        if self.model.is_none() {
            self.load_model()?;
        }
        
        match &self.config.enhancement_type {
            EnhancementType::SuperResolution { scale } => {
                self.super_resolution(frame, *scale)
            }
            EnhancementType::Denoising { strength } => {
                self.denoise(frame, *strength)
            }
            EnhancementType::Deblurring { strength } => {
                self.deblur(frame, *strength)
            }
            EnhancementType::ColorEnhancement { saturation, contrast } => {
                self.enhance_color(frame, *saturation, *contrast)
            }
            EnhancementType::Sharpening { amount } => {
                self.sharpen(frame, *amount)
            }
            EnhancementType::Combined { upscale, denoise, deblur, color, sharpen } => {
                self.combined_enhancement(frame, *upscale, *denoise, *deblur, *color, *sharpen)
            }
        }
    }
    
    /// Super-resolution upscaling
    fn super_resolution(&self, frame: &DynamicImage, scale: u32) -> AIResult<DynamicImage> {
        // Convert frame to tensor
        let tensor = self.feature_extractor.image_to_tensor(frame)?;
        
        // Apply super-resolution model
        let enhanced = if let Some(model) = &self.model {
            model.inference(&tensor)?
        } else {
            // Fallback to bicubic upscaling
            return Ok(frame.resize(
                frame.width() * scale,
                frame.height() * scale,
                image::imageops::FilterType::Lanczos3,
            ));
        };
        
        // Convert tensor back to image
        self.feature_extractor.tensor_to_image(&enhanced)
    }
    
    /// Denoise frame
    fn denoise(&self, frame: &DynamicImage, strength: f32) -> AIResult<DynamicImage> {
        let tensor = self.feature_extractor.image_to_tensor(frame)?;
        
        let enhanced = if let Some(model) = &self.model {
            model.inference(&tensor)?
        } else {
            // Fallback to simple denoising
            return Ok(self.simple_denoise(frame, strength));
        };
        
        self.feature_extractor.tensor_to_image(&enhanced)
    }
    
    /// Simple denoising fallback
    fn simple_denoise(&self, frame: &DynamicImage, strength: f32) -> DynamicImage {
        // Apply Gaussian blur for denoising
        let sigma = (strength * 2.0) as f32;
        frame.blur(sigma)
    }
    
    /// Deblur frame
    fn deblur(&self, frame: &DynamicImage, strength: f32) -> AIResult<DynamicImage> {
        let tensor = self.feature_extractor.image_to_tensor(frame)?;
        
        let enhanced = if let Some(model) = &self.model {
            model.inference(&tensor)?
        } else {
            // Fallback to sharpening
            return Ok(self.sharpen(frame, strength));
        };
        
        self.feature_extractor.tensor_to_image(&enhanced)
    }
    
    /// Enhance color
    fn enhance_color(&self, frame: &DynamicImage, saturation: f32, contrast: f32) -> AIResult<DynamicImage> {
        let mut img = frame.to_rgb8();
        
        // Apply contrast
        let factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));
        
        for pixel in img.pixels_mut() {
            // Apply contrast
            let r = (factor * (pixel[0] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
            let g = (factor * (pixel[1] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
            let b = (factor * (pixel[2] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
            
            // Apply saturation
            let gray = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            pixel[0] = (gray + saturation * (r as f32 - gray)).clamp(0.0, 255.0) as u8;
            pixel[1] = (gray + saturation * (g as f32 - gray)).clamp(0.0, 255.0) as u8;
            pixel[2] = (gray + saturation * (b as f32 - gray)).clamp(0.0, 255.0) as u8;
        }
        
        Ok(DynamicImage::ImageRgb8(img))
    }
    
    /// Sharpen frame
    fn sharpen(&self, frame: &DynamicImage, amount: f32) -> AIResult<DynamicImage> {
        let mut img = frame.to_rgb8();
        let width = img.width();
        let height = img.height();
        
        // Apply sharpening kernel
        let kernel = [
            [0.0, -1.0, 0.0],
            [-1.0, 5.0, -1.0],
            [0.0, -1.0, 0.0],
        ];
        
        let mut result = img.clone();
        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                
                for ky in -1..=1 {
                    for kx in -1..=1 {
                        let px = x as i32 + kx;
                        let py = y as i32 + ky;
                        let pixel = img.get_pixel(px as u32, py as u32);
                        let weight = kernel[(ky + 1) as usize][(kx + 1) as usize];
                        r += pixel[0] as f32 * weight;
                        g += pixel[1] as f32 * weight;
                        b += pixel[2] as f32 * weight;
                    }
                }
                
                let original = img.get_pixel(x, y);
                let sharpened_r = original[0] as f32 + amount * (r - original[0] as f32);
                let sharpened_g = original[1] as f32 + amount * (g - original[1] as f32);
                let sharpened_b = original[2] as f32 + amount * (b - original[2] as f32);
                
                result.put_pixel(
                    x,
                    y,
                    Rgb([
                        sharpened_r.clamp(0.0, 255.0) as u8,
                        sharpened_g.clamp(0.0, 255.0) as u8,
                        sharpened_b.clamp(0.0, 255.0) as u8,
                    ]),
                );
            }
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }
    
    /// Combined enhancement
    fn combined_enhancement(
        &mut self,
        frame: &DynamicImage,
        upscale: Option<u32>,
        denoise: Option<f32>,
        deblur: Option<f32>,
        color: Option<(f32, f32)>,
        sharpen: Option<f32>,
    ) -> AIResult<DynamicImage> {
        let mut result = frame.clone();
        
        // Apply denoising
        if let Some(strength) = denoise {
            result = self.denoise(&result, strength)?;
        }
        
        // Apply deblurring
        if let Some(strength) = deblur {
            result = self.deblur(&result, strength)?;
        }
        
        // Apply color enhancement
        if let Some((saturation, contrast)) = color {
            result = self.enhance_color(&result, saturation, contrast)?;
        }
        
        // Apply sharpening
        if let Some(amount) = sharpen {
            result = self.sharpen(&result, amount)?;
        }
        
        // Apply upscaling last
        if let Some(scale) = upscale {
            result = self.super_resolution(&result, scale)?;
        }
        
        Ok(result)
    }
    
    /// Enhance a sequence of frames with temporal consistency
    pub fn enhance_sequence(&mut self, frames: &[DynamicImage]) -> AIResult<Vec<DynamicImage>> {
        if !self.config.enable_temporal_consistency {
            return frames
                .iter()
                .map(|frame| self.enhance_frame(frame))
                .collect();
        }
        
        // Add frames to temporal buffer
        self.temporal_buffer.extend(frames.iter().cloned());
        
        // Maintain buffer size
        while self.temporal_buffer.len() > self.config.temporal_window_size {
            self.temporal_buffer.remove(0);
        }
        
        // Process frames with temporal consistency
        let mut enhanced = Vec::new();
        for frame in frames {
            let enhanced_frame = self.enhance_frame_with_temporal(frame)?;
            enhanced.push(enhanced_frame);
        }
        
        Ok(enhanced)
    }
    
    /// Enhance frame with temporal consistency
    fn enhance_frame_with_temporal(&mut self, frame: &DynamicImage) -> AIResult<DynamicImage> {
        // Enhance current frame
        let mut enhanced = self.enhance_frame(frame)?;
        
        // Apply temporal smoothing if buffer has enough frames
        if self.temporal_buffer.len() >= 3 {
            let prev = &self.temporal_buffer[self.temporal_buffer.len() - 2];
            let curr = &self.temporal_buffer[self.temporal_buffer.len() - 1];
            
            // Simple temporal averaging
            enhanced = self.temporal_blend(prev, curr, &enhanced, 0.3)?;
        }
        
        Ok(enhanced)
    }
    
    /// Temporal blending
    fn temporal_blend(
        &self,
        prev: &DynamicImage,
        curr: &DynamicImage,
        next: &DynamicImage,
        alpha: f32,
    ) -> AIResult<DynamicImage> {
        let prev_rgb = prev.to_rgb8();
        let curr_rgb = curr.to_rgb8();
        let next_rgb = next.to_rgb8();
        
        let mut result = curr_rgb.clone();
        
        for (i, (p, c, n)) in prev_rgb
            .pixels()
            .zip(curr_rgb.pixels())
            .zip(next_rgb.pixels())
            .enumerate()
        {
            let ((p, c), n) = (p, c, n);
            result.pixels_mut().nth(i).map(|pixel| {
                pixel[0] = ((1.0 - alpha) * c[0] as f32 + alpha * 0.5 * (p[0] as f32 + n[0] as f32))
                    .clamp(0.0, 255.0) as u8;
                pixel[1] = ((1.0 - alpha) * c[1] as f32 + alpha * 0.5 * (p[1] as f32 + n[1] as f32))
                    .clamp(0.0, 255.0) as u8;
                pixel[2] = ((1.0 - alpha) * c[2] as f32 + alpha * 0.5 * (p[2] as f32 + n[2] as f32))
                    .clamp(0.0, 255.0) as u8;
            });
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }
    
    /// Get enhancement statistics
    pub fn get_stats(&self) -> EnhancementStats {
        EnhancementStats {
            frames_processed: 0,
            total_time_ms: 0,
            avg_time_per_frame_ms: 0.0,
            temporal_buffer_size: self.temporal_buffer.len(),
        }
    }
}

/// Enhancement statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementStats {
    pub frames_processed: usize,
    pub total_time_ms: u64,
    pub avg_time_per_frame_ms: f64,
    pub temporal_buffer_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhancement_config_default() {
        let config = EnhancementConfig::default();
        assert!(config.enable_gpu);
        assert_eq!(config.batch_size, 4);
    }
    
    #[test]
    fn test_quality_preset_batch_size() {
        assert_eq!(QualityPreset::Fast.batch_size(), 8);
        assert_eq!(QualityPreset::Balanced.batch_size(), 4);
        assert_eq!(QualityPreset::Quality.batch_size(), 1);
    }
}