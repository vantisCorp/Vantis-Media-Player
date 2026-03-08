//! Visual effects for video processing

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

use crate::{
    Frame, FilterResult, FilterError, FilterInfo, FilterCategory,
    FilterParameter, ParameterInfo, ParameterType, ProcessingContext,
    ColorValue, filters::VideoFilter,
};

// ============================================================================
// Film Effects
// ============================================================================

/// Film grain effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilmGrainEffect {
    /// Grain intensity (0.0 to 1.0)
    intensity: f32,
    
    /// Grain size in pixels
    size: f32,
    
    /// Color grain (vs monochrome)
    colored: bool,
}

impl FilmGrainEffect {
    pub fn new(intensity: f32, size: f32, colored: bool) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            size: size.clamp(1.0, 10.0),
            colored,
        }
    }
}

#[async_trait]
impl VideoFilter for FilmGrainEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "film_grain".to_string(),
            name: "Film Grain".to_string(),
            category: FilterCategory::Noise,
            description: "Add realistic film grain to the video".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.3),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Grain intensity".to_string(),
                },
                ParameterInfo {
                    name: "size".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(1.0),
                    max: Some(10.0),
                    description: "Grain size in pixels".to_string(),
                },
                ParameterInfo {
                    name: "colored".to_string(),
                    param_type: ParameterType::Boolean,
                    default: FilterParameter::Boolean(false),
                    min: None,
                    max: None,
                    description: "Use colored grain".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 3,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let mut rng = rand::thread_rng();
        
        for pixel in output.data_mut().chunks_mut(4) {
            if self.colored {
                let noise_r = rng.gen_range(-self.intensity..self.intensity) * 255.0;
                let noise_g = rng.gen_range(-self.intensity..self.intensity) * 255.0;
                let noise_b = rng.gen_range(-self.intensity..self.intensity) * 255.0;
                
                pixel[0] = ((pixel[0] as f32 + noise_r).clamp(0.0, 255.0)) as u8;
                pixel[1] = ((pixel[1] as f32 + noise_g).clamp(0.0, 255.0)) as u8;
                pixel[2] = ((pixel[2] as f32 + noise_b).clamp(0.0, 255.0)) as u8;
            } else {
                let noise = rng.gen_range(-self.intensity..self.intensity) * 255.0;
                pixel[0] = ((pixel[0] as f32 + noise).clamp(0.0, 255.0)) as u8;
                pixel[1] = ((pixel[1] as f32 + noise).clamp(0.0, 255.0)) as u8;
                pixel[2] = ((pixel[2] as f32 + noise).clamp(0.0, 255.0)) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "intensity" => {
                if let FilterParameter::Float(v) = value {
                    self.intensity = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "size" => {
                if let FilterParameter::Float(v) = value {
                    self.size = v.clamp(1.0, 10.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "colored" => {
                if let FilterParameter::Boolean(v) = value {
                    self.colored = v;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected boolean value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "intensity" => Some(&FilterParameter::Float(self.intensity)),
            "size" => Some(&FilterParameter::Float(self.size)),
            "colored" => Some(&FilterParameter::Boolean(self.colored)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("intensity".to_string(), FilterParameter::Float(self.intensity));
        params.insert("size".to_string(), FilterParameter::Float(self.size));
        params.insert("colored".to_string(), FilterParameter::Boolean(self.colored));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Vignette effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VignetteEffect {
    /// Vignette intensity (0.0 to 1.0)
    intensity: f32,
    
    /// Inner radius (0.0 to 1.0)
    inner_radius: f32,
    
    /// Outer radius (0.0 to 1.0)
    outer_radius: f32,
    
    /// Vignette color (default black)
    color: ColorValue,
}

impl VignetteEffect {
    pub fn new(intensity: f32, inner_radius: f32, outer_radius: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            inner_radius: inner_radius.clamp(0.0, 1.0),
            outer_radius: outer_radius.clamp(0.0, 1.0),
            color: ColorValue::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

#[async_trait]
impl VideoFilter for VignetteEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "vignette".to_string(),
            name: "Vignette".to_string(),
            category: FilterCategory::Artistic,
            description: "Darken edges of the frame for a classic look".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.5),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Vignette intensity".to_string(),
                },
                ParameterInfo {
                    name: "inner_radius".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.3),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Inner radius where darkening starts".to_string(),
                },
                ParameterInfo {
                    name: "outer_radius".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Outer radius where darkening ends".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 2,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let width = frame.width as f32;
        let height = frame.height as f32;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let max_dist = (center_x * center_x + center_y * center_y).sqrt();
        
        for y in 0..frame.height {
            for x in 0..frame.width {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;
                
                let factor = if dist < self.inner_radius {
                    1.0
                } else if dist > self.outer_radius {
                    1.0 - self.intensity
                } else {
                    let t = (dist - self.inner_radius) / (self.outer_radius - self.inner_radius);
                    1.0 - t * self.intensity
                };
                
                let idx = (y * frame.width + x) as usize * 4;
                output.data_mut()[idx] = (output.data_mut()[idx] as f32 * factor) as u8;
                output.data_mut()[idx + 1] = (output.data_mut()[idx + 1] as f32 * factor) as u8;
                output.data_mut()[idx + 2] = (output.data_mut()[idx + 2] as f32 * factor) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "intensity" => {
                if let FilterParameter::Float(v) = value {
                    self.intensity = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "inner_radius" => {
                if let FilterParameter::Float(v) = value {
                    self.inner_radius = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "outer_radius" => {
                if let FilterParameter::Float(v) = value {
                    self.outer_radius = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "intensity" => Some(&FilterParameter::Float(self.intensity)),
            "inner_radius" => Some(&FilterParameter::Float(self.inner_radius)),
            "outer_radius" => Some(&FilterParameter::Float(self.outer_radius)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("intensity".to_string(), FilterParameter::Float(self.intensity));
        params.insert("inner_radius".to_string(), FilterParameter::Float(self.inner_radius));
        params.insert("outer_radius".to_string(), FilterParameter::Float(self.outer_radius));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Sepia tone effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SepiaEffect {
    intensity: f32,
}

impl SepiaEffect {
    pub fn new(intensity: f32) -> Self {
        Self { intensity: intensity.clamp(0.0, 1.0) }
    }
}

#[async_trait]
impl VideoFilter for SepiaEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "sepia".to_string(),
            name: "Sepia".to_string(),
            category: FilterCategory::Artistic,
            description: "Apply warm brown sepia tone for vintage look".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Sepia intensity".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Sepia transformation matrix
            let sepia_r = r * 0.393 + g * 0.769 + b * 0.189;
            let sepia_g = r * 0.349 + g * 0.686 + b * 0.168;
            let sepia_b = r * 0.272 + g * 0.534 + b * 0.131;
            
            // Blend original with sepia
            pixel[0] = ((r * (1.0 - self.intensity) + sepia_r * self.intensity).clamp(0.0, 255.0)) as u8;
            pixel[1] = ((g * (1.0 - self.intensity) + sepia_g * self.intensity).clamp(0.0, 255.0)) as u8;
            pixel[2] = ((b * (1.0 - self.intensity) + sepia_b * self.intensity).clamp(0.0, 255.0)) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "intensity" => {
                if let FilterParameter::Float(v) = value {
                    self.intensity = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "intensity" => Some(&FilterParameter::Float(self.intensity)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("intensity".to_string(), FilterParameter::Float(self.intensity));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Vintage/retro effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VintageEffect {
    /// Fade intensity
    fade: f32,
    
    /// Color shift
    color_shift: f32,
    
    /// Contrast adjustment
    contrast: f32,
}

impl VintageEffect {
    pub fn new(fade: f32, color_shift: f32, contrast: f32) -> Self {
        Self {
            fade: fade.clamp(0.0, 1.0),
            color_shift: color_shift.clamp(0.0, 1.0),
            contrast: contrast.clamp(0.5, 1.5),
        }
    }
}

#[async_trait]
impl VideoFilter for VintageEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "vintage".to_string(),
            name: "Vintage".to_string(),
            category: FilterCategory::Artistic,
            description: "Apply vintage/retro look with faded colors".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "fade".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.3),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Color fade amount".to_string(),
                },
                ParameterInfo {
                    name: "color_shift".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.2),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Color temperature shift".to_string(),
                },
                ParameterInfo {
                    name: "contrast".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.9),
                    min: Some(0.5),
                    max: Some(1.5),
                    description: "Contrast adjustment".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 3,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Apply contrast
            let mut r = (r - 128.0) * self.contrast + 128.0;
            let mut g = (g - 128.0) * self.contrast + 128.0;
            let mut b = (b - 128.0) * self.contrast + 128.0;
            
            // Color shift (warm up)
            r = r + self.color_shift * 30.0;
            b = b - self.color_shift * 20.0;
            
            // Fade (lift blacks)
            r = r + (255.0 - r) * self.fade * 0.1;
            g = g + (255.0 - g) * self.fade * 0.08;
            b = b + (255.0 - b) * self.fade * 0.12;
            
            pixel[0] = r.clamp(0.0, 255.0) as u8;
            pixel[1] = g.clamp(0.0, 255.0) as u8;
            pixel[2] = b.clamp(0.0, 255.0) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "fade" => {
                if let FilterParameter::Float(v) = value {
                    self.fade = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "color_shift" => {
                if let FilterParameter::Float(v) = value {
                    self.color_shift = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "contrast" => {
                if let FilterParameter::Float(v) = value {
                    self.contrast = v.clamp(0.5, 1.5);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "fade" => Some(&FilterParameter::Float(self.fade)),
            "color_shift" => Some(&FilterParameter::Float(self.color_shift)),
            "contrast" => Some(&FilterParameter::Float(self.contrast)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("fade".to_string(), FilterParameter::Float(self.fade));
        params.insert("color_shift".to_string(), FilterParameter::Float(self.color_shift));
        params.insert("contrast".to_string(), FilterParameter::Float(self.contrast));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Color Effects
// ============================================================================

/// Color inversion effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvertEffect {
    intensity: f32,
}

impl InvertEffect {
    pub fn new(intensity: f32) -> Self {
        Self { intensity: intensity.clamp(0.0, 1.0) }
    }
}

#[async_trait]
impl VideoFilter for InvertEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "invert".to_string(),
            name: "Invert Colors".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Invert all colors in the frame".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Inversion intensity".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            pixel[0] = ((255.0 - pixel[0] as f32) * self.intensity + pixel[0] as f32 * (1.0 - self.intensity)) as u8;
            pixel[1] = ((255.0 - pixel[1] as f32) * self.intensity + pixel[1] as f32 * (1.0 - self.intensity)) as u8;
            pixel[2] = ((255.0 - pixel[2] as f32) * self.intensity + pixel[2] as f32 * (1.0 - self.intensity)) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "intensity" => {
                if let FilterParameter::Float(v) = value {
                    self.intensity = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "intensity" => Some(&FilterParameter::Float(self.intensity)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("intensity".to_string(), FilterParameter::Float(self.intensity));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Color tint effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintEffect {
    /// Tint color
    color: ColorValue,
    
    /// Tint intensity
    intensity: f32,
    
    /// Preserve luminance
    preserve_luminance: bool,
}

impl TintEffect {
    pub fn new(color: ColorValue, intensity: f32) -> Self {
        Self {
            color,
            intensity: intensity.clamp(0.0, 1.0),
            preserve_luminance: true,
        }
    }
}

#[async_trait]
impl VideoFilter for TintEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "tint".to_string(),
            name: "Color Tint".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Apply a color tint to the frame".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.5),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "Tint intensity".to_string(),
                },
                ParameterInfo {
                    name: "preserve_luminance".to_string(),
                    param_type: ParameterType::Boolean,
                    default: FilterParameter::Boolean(true),
                    min: None,
                    max: None,
                    description: "Preserve original luminance".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 2,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            if self.preserve_luminance {
                // Calculate luminance
                let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                
                // Tint with luminance preservation
                let tint_r = self.color.r * lum;
                let tint_g = self.color.g * lum;
                let tint_b = self.color.b * lum;
                
                pixel[0] = ((r * (1.0 - self.intensity) + tint_r * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
                pixel[1] = ((g * (1.0 - self.intensity) + tint_g * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
                pixel[2] = ((b * (1.0 - self.intensity) + tint_b * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
            } else {
                pixel[0] = ((r * (1.0 - self.intensity) + self.color.r * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
                pixel[1] = ((g * (1.0 - self.intensity) + self.color.g * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
                pixel[2] = ((b * (1.0 - self.intensity) + self.color.b * self.intensity) * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "intensity" => {
                if let FilterParameter::Float(v) = value {
                    self.intensity = v.clamp(0.0, 1.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "preserve_luminance" => {
                if let FilterParameter::Boolean(v) = value {
                    self.preserve_luminance = v;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected boolean value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "intensity" => Some(&FilterParameter::Float(self.intensity)),
            "preserve_luminance" => Some(&FilterParameter::Boolean(self.preserve_luminance)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("intensity".to_string(), FilterParameter::Float(self.intensity));
        params.insert("preserve_luminance".to_string(), FilterParameter::Boolean(self.preserve_luminance));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Stylize Effects
// ============================================================================

/// Posterize effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosterizeEffect {
    /// Number of color levels per channel
    levels: u8,
}

impl PosterizeEffect {
    pub fn new(levels: u8) -> Self {
        Self { levels: levels.clamp(2, 32) }
    }
}

#[async_trait]
impl VideoFilter for PosterizeEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "posterize".to_string(),
            name: "Posterize".to_string(),
            category: FilterCategory::Stylize,
            description: "Reduce number of colors for poster effect".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "levels".to_string(),
                    param_type: ParameterType::Integer,
                    default: FilterParameter::Integer(4),
                    min: Some(2.0),
                    max: Some(32.0),
                    description: "Number of color levels per channel".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let step = 255.0 / (self.levels - 1) as f32;
        
        for pixel in output.data_mut().chunks_mut(4) {
            for c in 0..3 {
                let level = (pixel[c] as f32 / step).round();
                pixel[c] = (level * step).clamp(0.0, 255.0) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "levels" => {
                if let FilterParameter::Integer(v) = value {
                    self.levels = v.clamp(2, 32) as u8;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected integer value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "levels" => Some(&FilterParameter::Integer(self.levels as i32)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("levels".to_string(), FilterParameter::Integer(self.levels as i32));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Threshold/Binary effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdEffect {
    /// Threshold value (0-255)
    threshold: u8,
    
    /// Invert the result
    invert: bool,
}

impl ThresholdEffect {
    pub fn new(threshold: u8, invert: bool) -> Self {
        Self { threshold, invert }
    }
}

#[async_trait]
impl VideoFilter for ThresholdEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "threshold".to_string(),
            name: "Threshold".to_string(),
            category: FilterCategory::Stylize,
            description: "Convert to black and white based on threshold".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "threshold".to_string(),
                    param_type: ParameterType::Integer,
                    default: FilterParameter::Integer(128),
                    min: Some(0.0),
                    max: Some(255.0),
                    description: "Threshold value".to_string(),
                },
                ParameterInfo {
                    name: "invert".to_string(),
                    param_type: ParameterType::Boolean,
                    default: FilterParameter::Boolean(false),
                    min: None,
                    max: None,
                    description: "Invert the result".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            // Calculate luminance
            let lum = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            
            let value = if self.invert {
                if lum < self.threshold { 255 } else { 0 }
            } else {
                if lum > self.threshold { 255 } else { 0 }
            };
            
            pixel[0] = value;
            pixel[1] = value;
            pixel[2] = value;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "threshold" => {
                if let FilterParameter::Integer(v) = value {
                    self.threshold = v.clamp(0, 255) as u8;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected integer value"))
                }
            }
            "invert" => {
                if let FilterParameter::Boolean(v) = value {
                    self.invert = v;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected boolean value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "threshold" => Some(&FilterParameter::Integer(self.threshold as i32)),
            "invert" => Some(&FilterParameter::Boolean(self.invert)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), FilterParameter::Integer(self.threshold as i32));
        params.insert("invert".to_string(), FilterParameter::Boolean(self.invert));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Emboss effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbossEffect {
    strength: f32,
}

impl EmbossEffect {
    pub fn new(strength: f32) -> Self {
        Self { strength: strength.clamp(0.1, 5.0) }
    }
}

#[async_trait]
impl VideoFilter for EmbossEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "emboss".to_string(),
            name: "Emboss".to_string(),
            category: FilterCategory::Stylize,
            description: "Create embossed relief effect".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "strength".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.1),
                    max: Some(5.0),
                    description: "Emboss strength".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 3,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let width = frame.width as usize;
        let height = frame.height as usize;
        
        // Emboss kernel
        let kernel = [
            [-2, -1, 0],
            [-1,  1, 1],
            [ 0,  1, 2],
        ];
        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                
                for ky in 0..3 {
                    for kx in 0..3 {
                        let idx = ((y + ky - 1) * width + (x + kx - 1)) * 4;
                        let weight = kernel[ky][kx] as f32 * self.strength;
                        r_sum += frame.data[idx] as f32 * weight;
                        g_sum += frame.data[idx + 1] as f32 * weight;
                        b_sum += frame.data[idx + 2] as f32 * weight;
                    }
                }
                
                let out_idx = (y * width + x) * 4;
                output.data_mut()[out_idx] = (r_sum + 128.0).clamp(0.0, 255.0) as u8;
                output.data_mut()[out_idx + 1] = (g_sum + 128.0).clamp(0.0, 255.0) as u8;
                output.data_mut()[out_idx + 2] = (b_sum + 128.0).clamp(0.0, 255.0) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "strength" => {
                if let FilterParameter::Float(v) = value {
                    self.strength = v.clamp(0.1, 5.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "strength" => Some(&FilterParameter::Float(self.strength)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("strength".to_string(), FilterParameter::Float(self.strength));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Chromatic aberration effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaticAberrationEffect {
    /// Offset amount
    amount: f32,
    
    /// Radial falloff
    radial: bool,
}

impl ChromaticAberrationEffect {
    pub fn new(amount: f32, radial: bool) -> Self {
        Self {
            amount: amount.clamp(0.0, 20.0),
            radial,
        }
    }
}

#[async_trait]
impl VideoFilter for ChromaticAberrationEffect {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "chromatic_aberration".to_string(),
            name: "Chromatic Aberration".to_string(),
            category: FilterCategory::Artistic,
            description: "Simulate lens chromatic aberration".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "amount".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(5.0),
                    min: Some(0.0),
                    max: Some(20.0),
                    description: "Aberration amount in pixels".to_string(),
                },
                ParameterInfo {
                    name: "radial".to_string(),
                    param_type: ParameterType::Boolean,
                    default: FilterParameter::Boolean(true),
                    min: None,
                    max: None,
                    description: "Apply radial falloff".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 4,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let width = frame.width as i32;
        let height = frame.height as i32;
        let center_x = width / 2;
        let center_y = height / 2;
        let max_dist = ((center_x * center_x + center_y * center_y) as f32).sqrt();
        
        for y in 0..height {
            for x in 0..width {
                let mut offset = self.amount;
                
                if self.radial {
                    let dx = x - center_x;
                    let dy = y - center_y;
                    let dist = ((dx * dx + dy * dy) as f32).sqrt() / max_dist;
                    offset *= dist;
                }
                
                let r_x = (x as f32 - offset) as i32;
                let b_x = (x as f32 + offset) as i32;
                
                let idx = (y * width + x) as usize * 4;
                
                // Red channel from left
                if r_x >= 0 && r_x < width {
                    let r_idx = (y * width + r_x) as usize * 4;
                    output.data_mut()[idx] = frame.data[r_idx];
                }
                
                // Blue channel from right
                if b_x >= 0 && b_x < width {
                    let b_idx = (y * width + b_x) as usize * 4;
                    output.data_mut()[idx + 2] = frame.data[b_idx + 2];
                }
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "amount" => {
                if let FilterParameter::Float(v) = value {
                    self.amount = v.clamp(0.0, 20.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "radial" => {
                if let FilterParameter::Boolean(v) = value {
                    self.radial = v;
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected boolean value"))
                }
            }
            _ => Err(FilterError::invalid_param(name, "Unknown parameter"))
        }
    }
    
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter> {
        match name {
            "amount" => Some(&FilterParameter::Float(self.amount)),
            "radial" => Some(&FilterParameter::Boolean(self.radial)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("amount".to_string(), FilterParameter::Float(self.amount));
        params.insert("radial".to_string(), FilterParameter::Boolean(self.radial));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Preset effects that combine multiple filters
pub mod presets {
    use super::*;
    
    /// Create a cinematic film look
    pub fn cinematic() -> Vec<Box<dyn VideoFilter>> {
        vec![
            Box::new(VintageEffect::new(0.15, 0.1, 1.1)),
            Box::new(VignetteEffect::new(0.4, 0.5, 1.0)),
            Box::new(FilmGrainEffect::new(0.1, 1.5, false)),
        ]
    }
    
    /// Create a horror film look
    pub fn horror() -> Vec<Box<dyn VideoFilter>> {
        vec![
            Box::new(crate::filters::SaturationFilter::new(0.3)),
            Box::new(crate::filters::ContrastFilter::new(1.3)),
            Box::new(VignetteEffect::new(0.6, 0.3, 0.9)),
            Box::new(FilmGrainEffect::new(0.2, 2.0, false)),
        ]
    }
    
    /// Create a dreamy look
    pub fn dreamy() -> Vec<Box<dyn VideoFilter>> {
        vec![
            Box::new(crate::filters::BrightnessFilter::new(0.05)),
            Box::new(crate::filters::SaturationFilter::new(0.7)),
            Box::new(crate::filters::GaussianBlurFilter::new(2.0, 1.0)),
        ]
    }
    
    /// Create a vintage 70s look
    pub fn vintage_70s() -> Vec<Box<dyn VideoFilter>> {
        vec![
            Box::new(VintageEffect::new(0.4, 0.3, 0.85)),
            Box::new(SepiaEffect::new(0.3)),
            Box::new(VignetteEffect::new(0.3, 0.6, 1.0)),
            Box::new(FilmGrainEffect::new(0.25, 2.5, true)),
        ]
    }
    
    /// Create a noir look
    pub fn noir() -> Vec<Box<dyn VideoFilter>> {
        vec![
            Box::new(crate::filters::SaturationFilter::new(0.0)),
            Box::new(crate::filters::ContrastFilter::new(1.4)),
            Box::new(VignetteEffect::new(0.5, 0.4, 1.0)),
        ]
    }
}