//! Color Grading Engine
//! 
//! Provides color grading with presets and custom adjustments.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error, debug, warn};
use image::{RgbImage, Rgb, Pixel};

use crate::{ColorGradingConfig, ColorAdjustments};

/// Color grader
pub struct ColorGrader {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Color grading configuration
    config: Arc<RwLock<ColorGradingConfig>>,
    
    /// Color presets
    presets: Arc<RwLock<HashMap<String, ColorAdjustments>>>,
    
    /// LUT cache
    lut_cache: Arc<RwLock<HashMap<String, Vec<[u8; 3]>>>>,
}

impl ColorGrader {
    /// Create a new color grader
    pub fn new() -> Result<Self> {
        info!("Initializing color grader");
        
        let mut presets = HashMap::new();
        
        // Add default presets
        presets.insert("neutral".to_string(), ColorAdjustments::default());
        presets.insert("cinematic".to_string(), ColorAdjustments {
            brightness: -0.05,
            contrast: 0.1,
            saturation: 0.9,
            hue: 0.0,
            temperature: -10.0,
            tint: 0.0,
            vibrance: 0.1,
            exposure: -0.1,
            highlights: -0.1,
            shadows: 0.1,
            whites: 0.0,
            blacks: 0.0,
        });
        presets.insert("vivid".to_string(), ColorAdjustments {
            brightness: 0.0,
            contrast: 0.15,
            saturation: 0.3,
            hue: 0.0,
            temperature: 0.0,
            tint: 0.0,
            vibrance: 0.2,
            exposure: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.1,
            blacks: -0.1,
        });
        presets.insert("warm".to_string(), ColorAdjustments {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.1,
            hue: 0.0,
            temperature: 20.0,
            tint: 5.0,
            vibrance: 0.0,
            exposure: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
        });
        presets.insert("cool".to_string(), ColorAdjustments {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.1,
            hue: 0.0,
            temperature: -20.0,
            tint: -5.0,
            vibrance: 0.0,
            exposure: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
        });
        presets.insert("bw".to_string(), ColorAdjustments {
            brightness: 0.0,
            contrast: 0.1,
            saturation: -1.0,
            hue: 0.0,
            temperature: 0.0,
            tint: 0.0,
            vibrance: 0.0,
            exposure: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
        });
        presets.insert("vintage".to_string(), ColorAdjustments {
            brightness: -0.1,
            contrast: 0.2,
            saturation: 0.7,
            hue: 0.0,
            temperature: 15.0,
            tint: 10.0,
            vibrance: -0.1,
            exposure: -0.1,
            highlights: 0.1,
            shadows: -0.1,
            whites: 0.0,
            blacks: 0.1,
        });
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(ColorGradingConfig::default())),
            presets: Arc::new(RwLock::new(presets)),
            lut_cache: Arc::new(RwLock::new(HashMap::new())),
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
    
    /// Set color grading configuration
    pub async fn set_config(&self, config: ColorGradingConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Color grading configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> ColorGradingConfig {
        self.config.read().await.clone()
    }
    
    /// Apply color preset to frame
    pub async fn apply_preset(&self, frame: &RgbImage, preset_name: &str) -> Result<RgbImage> {
        let presets = self.presets.read().await;
        
        let adjustments = presets.get(preset_name)
            .context(format!("Preset '{}' not found", preset_name))?;
        
        self.apply_adjustments(frame, adjustments).await
    }
    
    /// Apply custom adjustments to frame
    pub async fn apply_adjustments(&self, frame: &RgbImage, adjustments: &ColorAdjustments) -> Result<RgbImage> {
        let mut output = frame.clone();
        
        // Apply adjustments in order
        output = self.apply_exposure(&output, adjustments.exposure)?;
        output = self.apply_brightness_contrast(&output, adjustments.brightness, adjustments.contrast)?;
        output = self.apply_highlights_shadows(&output, adjustments.highlights, adjustments.shadows)?;
        output = self.apply_whites_blacks(&output, adjustments.whites, adjustments.blacks)?;
        output = self.apply_temperature_tint(&output, adjustments.temperature, adjustments.tint)?;
        output = self.apply_saturation_vibrance(&output, adjustments.saturation, adjustments.vibrance)?;
        output = self.apply_hue(&output, adjustments.hue)?;
        
        Ok(output)
    }
    
    /// Apply exposure adjustment
    fn apply_exposure(&self, frame: &RgbImage, exposure: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        let factor = 2.0_f32.powf(exposure);
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let r = (pixel[0] as f32 * factor).clamp(0.0, 255.0) as u8;
                let g = (pixel[1] as f32 * factor).clamp(0.0, 255.0) as u8;
                let b = (pixel[2] as f32 * factor).clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Apply brightness and contrast
    fn apply_brightness_contrast(&self, frame: &RgbImage, brightness: f32, contrast: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        let brightness_offset = brightness * 255.0;
        let contrast_factor = (1.0 + contrast) / (1.0 - contrast + 0.001);
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let r = ((pixel[0] as f32 - 128.0) * contrast_factor + 128.0 + brightness_offset).clamp(0.0, 255.0) as u8;
                let g = ((pixel[1] as f32 - 128.0) * contrast_factor + 128.0 + brightness_offset).clamp(0.0, 255.0) as u8;
                let b = ((pixel[2] as f32 - 128.0) * contrast_factor + 128.0 + brightness_offset).clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Apply highlights and shadows
    fn apply_highlights_shadows(&self, frame: &RgbImage, highlights: f32, shadows: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0 / 255.0;
                
                // Calculate highlight and shadow masks
                let highlight_mask = intensity.powf(2.0);
                let shadow_mask = (1.0 - intensity).powf(2.0);
                
                let highlight_adjust = highlights * highlight_mask * 255.0;
                let shadow_adjust = shadows * shadow_mask * 255.0;
                
                let r = (pixel[0] as f32 + highlight_adjust + shadow_adjust).clamp(0.0, 255.0) as u8;
                let g = (pixel[1] as f32 + highlight_adjust + shadow_adjust).clamp(0.0, 255.0) as u8;
                let b = (pixel[2] as f32 + highlight_adjust + shadow_adjust).clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Apply whites and blacks
    fn apply_whites_blacks(&self, frame: &RgbImage, whites: f32, blacks: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        let white_point = 255.0 - whites * 50.0;
        let black_point = blacks * 50.0;
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let r = self.adjust_white_black(pixel[0] as f32, white_point, black_point);
                let g = self.adjust_white_black(pixel[1] as f32, white_point, black_point);
                let b = self.adjust_white_black(pixel[2] as f32, white_point, black_point);
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Adjust white and black points
    fn adjust_white_black(&self, value: f32, white_point: f32, black_point: f32) -> u8 {
        let adjusted = (value - black_point) * 255.0 / (white_point - black_point);
        adjusted.clamp(0.0, 255.0) as u8
    }
    
    /// Apply temperature and tint
    fn apply_temperature_tint(&self, frame: &RgbImage, temperature: f32, tint: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        // Temperature: blue/yellow tint
        let temp_r = if temperature > 0.0 { temperature * 0.5 } else { 0.0 };
        let temp_b = if temperature < 0.0 { -temperature * 0.5 } else { 0.0 };
        
        // Tint: green/magenta tint
        let tint_g = if tint > 0.0 { tint * 0.3 } else { 0.0 };
        let tint_m = if tint < 0.0 { -tint * 0.3 } else { 0.0 };
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let r = (pixel[0] as f32 + temp_r + tint_m).clamp(0.0, 255.0) as u8;
                let g = (pixel[1] as f32 + tint_g).clamp(0.0, 255.0) as u8;
                let b = (pixel[2] as f32 + temp_b + tint_m).clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Apply saturation and vibrance
    fn apply_saturation_vibrance(&self, frame: &RgbImage, saturation: f32, vibrance: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                
                // Saturation adjustment
                let sat_factor = 1.0 + saturation;
                let r_sat = intensity + (pixel[0] as f32 - intensity) * sat_factor;
                let g_sat = intensity + (pixel[1] as f32 - intensity) * sat_factor;
                let b_sat = intensity + (pixel[2] as f32 - intensity) * sat_factor;
                
                // Vibrance adjustment (affects less saturated colors more)
                let max_val = pixel[0].max(pixel[1]).max(pixel[2]) as f32;
                let min_val = pixel[0].min(pixel[1]).min(pixel[2]) as f32;
                let current_sat = (max_val - min_val) / max_val.max(1.0);
                
                let vib_factor = 1.0 + vibrance * (1.0 - current_sat);
                let r_vib = intensity + (r_sat - intensity) * vib_factor;
                let g_vib = intensity + (g_sat - intensity) * vib_factor;
                let b_vib = intensity + (b_sat - intensity) * vib_factor;
                
                let r = r_vib.clamp(0.0, 255.0) as u8;
                let g = g_vib.clamp(0.0, 255.0) as u8;
                let b = b_vib.clamp(0.0, 255.0) as u8;
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Apply hue shift
    fn apply_hue(&self, frame: &RgbImage, hue: f32) -> Result<RgbImage> {
        let mut output = RgbImage::new(frame.width(), frame.height());
        
        let hue_rad = hue * std::f32::consts::PI / 180.0;
        let cos_hue = hue_rad.cos();
        let sin_hue = hue_rad.sin();
        
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                
                // Convert to HSL, shift hue, convert back
                let (h, s, l) = self.rgb_to_hsl(pixel[0], pixel[1], pixel[2]);
                let new_h = (h + hue / 360.0).rem_euclid(1.0);
                let (r, g, b) = self.hsl_to_rgb(new_h, s, l);
                
                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(output)
    }
    
    /// Convert RGB to HSL
    fn rgb_to_hsl(&self, r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r_norm = r as f32 / 255.0;
        let g_norm = g as f32 / 255.0;
        let b_norm = b as f32 / 255.0;
        
        let max = r_norm.max(g_norm).max(b_norm);
        let min = r_norm.min(g_norm).min(b_norm);
        let delta = max - min;
        
        let l = (max + min) / 2.0;
        
        let h = if delta == 0.0 {
            0.0
        } else if max == r_norm {
            ((g_norm - b_norm) / delta + if g_norm < b_norm { 6.0 } else { 0.0 }) / 6.0
        } else if max == g_norm {
            ((b_norm - r_norm) / delta + 2.0) / 6.0
        } else {
            ((r_norm - g_norm) / delta + 4.0) / 6.0
        };
        
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        
        (h, s, l)
    }
    
    /// Convert HSL to RGB
    fn hsl_to_rgb(&self, h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        
        let (r, g, b) = if h < 1.0 / 6.0 {
            (c, x, 0.0)
        } else if h < 2.0 / 6.0 {
            (x, c, 0.0)
        } else if h < 3.0 / 6.0 {
            (0.0, c, x)
        } else if h < 4.0 / 6.0 {
            (0.0, x, c)
        } else if h < 5.0 / 6.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        let r = ((r + m) * 255.0).clamp(0.0, 255.0) as u8;
        let g = ((g + m) * 255.0).clamp(0.0, 255.0) as u8;
        let b = ((b + m) * 255.0).clamp(0.0, 255.0) as u8;
        
        (r, g, b)
    }
    
    /// Add custom preset
    pub async fn add_preset(&self, name: String, adjustments: ColorAdjustments) -> Result<()> {
        let mut presets = self.presets.write().await;
        presets.insert(name, adjustments);
        info!("Added color preset");
        Ok(())
    }
    
    /// Remove preset
    pub async fn remove_preset(&self, name: &str) -> Result<bool> {
        let mut presets = self.presets.write().await;
        let removed = presets.remove(name).is_some();
        if removed {
            info!("Removed color preset: {}", name);
        }
        Ok(removed)
    }
    
    /// Get list of available presets
    pub async fn get_presets(&self) -> Vec<String> {
        self.presets.read().await.keys().cloned().collect()
    }
    
    /// Load LUT from file
    pub async fn load_lut(&self, path: &str) -> Result<()> {
        // Placeholder for LUT loading
        info!("Loading LUT from: {}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_color_grader_creation() {
        let grader = ColorGrader::new().unwrap();
        assert!(grader.is_initialized());
    }
    
    #[tokio::test]
    async fn test_apply_preset() {
        let grader = ColorGrader::new().unwrap();
        
        let frame = RgbImage::new(640, 480);
        let graded = grader.apply_preset(&frame, "cinematic").await.unwrap();
        
        assert_eq!(graded.width(), frame.width());
        assert_eq!(graded.height(), frame.height());
    }
    
    #[tokio::test]
    async fn test_apply_adjustments() {
        let grader = ColorGrader::new().unwrap();
        
        let frame = RgbImage::new(640, 480);
        let adjustments = ColorAdjustments {
            brightness: 0.1,
            contrast: 0.1,
            saturation: 0.1,
            ..Default::default()
        };
        
        let graded = grader.apply_adjustments(&frame, &adjustments).await.unwrap();
        assert_eq!(graded.width(), frame.width());
    }
    
    #[tokio::test]
    async fn test_presets() {
        let grader = ColorGrader::new().unwrap();
        
        let presets = grader.get_presets().await;
        assert!(presets.contains(&"neutral".to_string()));
        assert!(presets.contains(&"cinematic".to_string()));
        assert!(presets.contains(&"vivid".to_string()));
    }
    
    #[tokio::test]
    async fn test_add_remove_preset() {
        let grader = ColorGrader::new().unwrap();
        
        let adjustments = ColorAdjustments::default();
        grader.add_preset("test_preset".to_string(), adjustments).await.unwrap();
        
        let presets = grader.get_presets().await;
        assert!(presets.contains(&"test_preset".to_string()));
        
        let removed = grader.remove_preset("test_preset").await.unwrap();
        assert!(removed);
    }
}