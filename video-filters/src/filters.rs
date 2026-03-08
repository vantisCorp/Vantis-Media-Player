//! Built-in video filters

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    Frame, FilterResult, FilterError, FilterInfo, FilterCategory,
    FilterParameter, ParameterInfo, ParameterType, ProcessingContext,
    ColorValue,
};

/// Trait for video filters
#[async_trait]
pub trait VideoFilter: Send + Sync {
    /// Get filter information
    fn info(&self) -> FilterInfo;
    
    /// Process a frame
    async fn process(&self, frame: &Frame, ctx: &ProcessingContext) -> FilterResult<Frame>;
    
    /// Set a parameter
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()>;
    
    /// Get a parameter
    fn get_parameter(&self, name: &str) -> Option<&FilterParameter>;
    
    /// Get all parameters
    fn get_parameters(&self) -> HashMap<String, FilterParameter>;
    
    /// Clone the filter
    fn clone_filter(&self) -> Box<dyn VideoFilter>;
}

// ============================================================================
// Color Correction Filters
// ============================================================================

/// Brightness adjustment filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessFilter {
    brightness: f32,
}

impl BrightnessFilter {
    pub fn new(brightness: f32) -> Self {
        Self { brightness: brightness.clamp(-1.0, 1.0) }
    }
}

#[async_trait]
impl VideoFilter for BrightnessFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "brightness".to_string(),
            name: "Brightness".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Adjust the brightness of the video".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "brightness".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.0),
                    min: Some(-1.0),
                    max: Some(1.0),
                    description: "Brightness adjustment (-1.0 to 1.0)".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let adjustment = (self.brightness * 255.0) as i16;
        
        for pixel in output.data_mut().chunks_mut(4) {
            pixel[0] = (pixel[0] as i16 + adjustment).clamp(0, 255) as u8;
            pixel[1] = (pixel[1] as i16 + adjustment).clamp(0, 255) as u8;
            pixel[2] = (pixel[2] as i16 + adjustment).clamp(0, 255) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "brightness" => {
                if let FilterParameter::Float(v) = value {
                    self.brightness = v.clamp(-1.0, 1.0);
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
            "brightness" => Some(&FilterParameter::Float(self.brightness)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("brightness".to_string(), FilterParameter::Float(self.brightness));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Contrast adjustment filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastFilter {
    contrast: f32,
}

impl ContrastFilter {
    pub fn new(contrast: f32) -> Self {
        Self { contrast: contrast.clamp(0.0, 3.0) }
    }
}

#[async_trait]
impl VideoFilter for ContrastFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "contrast".to_string(),
            name: "Contrast".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Adjust the contrast of the video".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "contrast".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(3.0),
                    description: "Contrast factor (0.0 to 3.0)".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 1,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let factor = self.contrast;
        let offset = 128.0 * (1.0 - factor);
        
        for pixel in output.data_mut().chunks_mut(4) {
            pixel[0] = ((pixel[0] as f32 * factor) + offset).clamp(0.0, 255.0) as u8;
            pixel[1] = ((pixel[1] as f32 * factor) + offset).clamp(0.0, 255.0) as u8;
            pixel[2] = ((pixel[2] as f32 * factor) + offset).clamp(0.0, 255.0) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "contrast" => {
                if let FilterParameter::Float(v) = value {
                    self.contrast = v.clamp(0.0, 3.0);
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
            "contrast" => Some(&FilterParameter::Float(self.contrast)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("contrast".to_string(), FilterParameter::Float(self.contrast));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Saturation adjustment filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaturationFilter {
    saturation: f32,
}

impl SaturationFilter {
    pub fn new(saturation: f32) -> Self {
        Self { saturation: saturation.clamp(0.0, 3.0) }
    }
}

#[async_trait]
impl VideoFilter for SaturationFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "saturation".to_string(),
            name: "Saturation".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Adjust color saturation".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "saturation".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(3.0),
                    description: "Saturation factor (0.0 = grayscale, 1.0 = normal)".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 2,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Convert to grayscale
            let gray = 0.299 * r + 0.587 * g + 0.114 * b;
            
            // Interpolate between gray and original color
            pixel[0] = (gray + (r - gray) * self.saturation).clamp(0.0, 255.0) as u8;
            pixel[1] = (gray + (g - gray) * self.saturation).clamp(0.0, 255.0) as u8;
            pixel[2] = (gray + (b - gray) * self.saturation).clamp(0.0, 255.0) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "saturation" => {
                if let FilterParameter::Float(v) = value {
                    self.saturation = v.clamp(0.0, 3.0);
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
            "saturation" => Some(&FilterParameter::Float(self.saturation)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("saturation".to_string(), FilterParameter::Float(self.saturation));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Hue rotation filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HueFilter {
    hue_shift: f32,
}

impl HueFilter {
    pub fn new(hue_shift: f32) -> Self {
        Self { hue_shift: hue_shift % 360.0 }
    }
}

#[async_trait]
impl VideoFilter for HueFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "hue".to_string(),
            name: "Hue Shift".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Rotate the hue of colors".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "hue_shift".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.0),
                    min: Some(-180.0),
                    max: Some(180.0),
                    description: "Hue rotation in degrees".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 3,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        
        for pixel in output.data_mut().chunks_mut(4) {
            let (h, s, l) = rgb_to_hsl(pixel[0], pixel[1], pixel[2]);
            let new_h = (h + self.hue_shift / 360.0) % 1.0;
            let (r, g, b) = hsl_to_rgb(new_h, s, l);
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "hue_shift" => {
                if let FilterParameter::Float(v) = value {
                    self.hue_shift = v % 360.0;
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
            "hue_shift" => Some(&FilterParameter::Float(self.hue_shift)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("hue_shift".to_string(), FilterParameter::Float(self.hue_shift));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Gamma correction filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaFilter {
    gamma: f32,
}

impl GammaFilter {
    pub fn new(gamma: f32) -> Self {
        Self { gamma: gamma.clamp(0.1, 5.0) }
    }
}

#[async_trait]
impl VideoFilter for GammaFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "gamma".to_string(),
            name: "Gamma Correction".to_string(),
            category: FilterCategory::ColorCorrection,
            description: "Apply gamma correction to adjust midtones".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "gamma".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.1),
                    max: Some(5.0),
                    description: "Gamma value (1.0 = no change)".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 2,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let inv_gamma = 1.0 / self.gamma;
        
        for pixel in output.data_mut().chunks_mut(4) {
            pixel[0] = ((pixel[0] as f32 / 255.0).powf(inv_gamma) * 255.0).clamp(0.0, 255.0) as u8;
            pixel[1] = ((pixel[1] as f32 / 255.0).powf(inv_gamma) * 255.0).clamp(0.0, 255.0) as u8;
            pixel[2] = ((pixel[2] as f32 / 255.0).powf(inv_gamma) * 255.0).clamp(0.0, 255.0) as u8;
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "gamma" => {
                if let FilterParameter::Float(v) = value {
                    self.gamma = v.clamp(0.1, 5.0);
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
            "gamma" => Some(&FilterParameter::Float(self.gamma)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("gamma".to_string(), FilterParameter::Float(self.gamma));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Blur Filters
// ============================================================================

/// Gaussian blur filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaussianBlurFilter {
    radius: f32,
    sigma: f32,
}

impl GaussianBlurFilter {
    pub fn new(radius: f32, sigma: f32) -> Self {
        Self {
            radius: radius.clamp(1.0, 50.0),
            sigma: sigma.clamp(0.1, 20.0),
        }
    }
}

#[async_trait]
impl VideoFilter for GaussianBlurFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "gaussian_blur".to_string(),
            name: "Gaussian Blur".to_string(),
            category: FilterCategory::Blur,
            description: "Apply Gaussian blur to soften the image".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "radius".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(5.0),
                    min: Some(1.0),
                    max: Some(50.0),
                    description: "Blur radius in pixels".to_string(),
                },
                ParameterInfo {
                    name: "sigma".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(2.0),
                    min: Some(0.1),
                    max: Some(20.0),
                    description: "Gaussian standard deviation".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 5,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let width = frame.width as usize;
        let height = frame.height as usize;
        
        // Create Gaussian kernel
        let kernel_size = (self.radius * 2.0 + 1.0) as usize;
        let mut kernel = vec![0.0f32; kernel_size];
        let center = self.radius as usize;
        let two_sigma_sq = 2.0 * self.sigma * self.sigma;
        let mut sum = 0.0;
        
        for i in 0..kernel_size {
            let x = (i as f32 - center as f32).abs();
            kernel[i] = (-x * x / two_sigma_sq).exp();
            sum += kernel[i];
        }
        for k in &mut kernel {
            *k /= sum;
        }
        
        // Horizontal pass
        let mut temp = vec![0u8; frame.data.len()];
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                
                for (i, &k) in kernel.iter().enumerate() {
                    let nx = (x as i32 + i as i32 - center as i32).clamp(0, width as i32 - 1) as usize;
                    let idx = (y * width + nx) * 4;
                    r_sum += frame.data[idx] as f32 * k;
                    g_sum += frame.data[idx + 1] as f32 * k;
                    b_sum += frame.data[idx + 2] as f32 * k;
                }
                
                let out_idx = (y * width + x) * 4;
                temp[out_idx] = r_sum.clamp(0.0, 255.0) as u8;
                temp[out_idx + 1] = g_sum.clamp(0.0, 255.0) as u8;
                temp[out_idx + 2] = b_sum.clamp(0.0, 255.0) as u8;
                temp[out_idx + 3] = frame.data[(y * width + x) * 4 + 3];
            }
        }
        
        // Vertical pass
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                
                for (i, &k) in kernel.iter().enumerate() {
                    let ny = (y as i32 + i as i32 - center as i32).clamp(0, height as i32 - 1) as usize;
                    let idx = (ny * width + x) * 4;
                    r_sum += temp[idx] as f32 * k;
                    g_sum += temp[idx + 1] as f32 * k;
                    b_sum += temp[idx + 2] as f32 * k;
                }
                
                let out_idx = (y * width + x) * 4;
                output.data_mut()[out_idx] = r_sum.clamp(0.0, 255.0) as u8;
                output.data_mut()[out_idx + 1] = g_sum.clamp(0.0, 255.0) as u8;
                output.data_mut()[out_idx + 2] = b_sum.clamp(0.0, 255.0) as u8;
                output.data_mut()[out_idx + 3] = temp[(y * width + x) * 4 + 3];
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "radius" => {
                if let FilterParameter::Float(v) = value {
                    self.radius = v.clamp(1.0, 50.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "sigma" => {
                if let FilterParameter::Float(v) = value {
                    self.sigma = v.clamp(0.1, 20.0);
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
            "radius" => Some(&FilterParameter::Float(self.radius)),
            "sigma" => Some(&FilterParameter::Float(self.sigma)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("radius".to_string(), FilterParameter::Float(self.radius));
        params.insert("sigma".to_string(), FilterParameter::Float(self.sigma));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

/// Box blur filter (fast)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxBlurFilter {
    radius: u32,
}

impl BoxBlurFilter {
    pub fn new(radius: u32) -> Self {
        Self { radius: radius.clamp(1, 50) }
    }
}

#[async_trait]
impl VideoFilter for BoxBlurFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "box_blur".to_string(),
            name: "Box Blur".to_string(),
            category: FilterCategory::Blur,
            description: "Fast box blur effect".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "radius".to_string(),
                    param_type: ParameterType::Integer,
                    default: FilterParameter::Integer(3),
                    min: Some(1.0),
                    max: Some(50.0),
                    description: "Blur radius in pixels".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 3,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let width = frame.width;
        let height = frame.height;
        
        // Horizontal pass
        let mut temp = vec![0u8; frame.data.len()];
        let kernel_size = (self.radius * 2 + 1) as f32;
        
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut count = 0u32;
                
                for dx in -(self.radius as i32)..=(self.radius as i32) {
                    let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                    let idx = (y * width + nx) as usize * 4;
                    r_sum += frame.data[idx] as u32;
                    g_sum += frame.data[idx + 1] as u32;
                    b_sum += frame.data[idx + 2] as u32;
                    count += 1;
                }
                
                let out_idx = (y * width + x) as usize * 4;
                temp[out_idx] = (r_sum / count) as u8;
                temp[out_idx + 1] = (g_sum / count) as u8;
                temp[out_idx + 2] = (b_sum / count) as u8;
                temp[out_idx + 3] = frame.data[(y * width + x) as usize * 4 + 3];
            }
        }
        
        // Vertical pass
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut count = 0u32;
                
                for dy in -(self.radius as i32)..=(self.radius as i32) {
                    let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;
                    let idx = (ny * width + x) as usize * 4;
                    r_sum += temp[idx] as u32;
                    g_sum += temp[idx + 1] as u32;
                    b_sum += temp[idx + 2] as u32;
                    count += 1;
                }
                
                let out_idx = (y * width + x) as usize * 4;
                output.data_mut()[out_idx] = (r_sum / count) as u8;
                output.data_mut()[out_idx + 1] = (g_sum / count) as u8;
                output.data_mut()[out_idx + 2] = (b_sum / count) as u8;
                output.data_mut()[out_idx + 3] = temp[(y * width + x) as usize * 4 + 3];
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "radius" => {
                if let FilterParameter::Integer(v) = value {
                    self.radius = v.clamp(1, 50) as u32;
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
            "radius" => Some(&FilterParameter::Integer(self.radius as i32)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("radius".to_string(), FilterParameter::Integer(self.radius as i32));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Sharpen Filters
// ============================================================================

/// Sharpen filter using unsharp mask
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpenFilter {
    amount: f32,
    radius: f32,
    threshold: f32,
}

impl SharpenFilter {
    pub fn new(amount: f32, radius: f32, threshold: f32) -> Self {
        Self {
            amount: amount.clamp(0.0, 5.0),
            radius: radius.clamp(0.1, 10.0),
            threshold: threshold.clamp(0.0, 255.0),
        }
    }
}

#[async_trait]
impl VideoFilter for SharpenFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "sharpen".to_string(),
            name: "Sharpen".to_string(),
            category: FilterCategory::Sharpen,
            description: "Sharpen the image using unsharp mask".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "amount".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(5.0),
                    description: "Sharpening strength".to_string(),
                },
                ParameterInfo {
                    name: "radius".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.1),
                    max: Some(10.0),
                    description: "Sharpening radius".to_string(),
                },
                ParameterInfo {
                    name: "threshold".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(0.0),
                    min: Some(0.0),
                    max: Some(255.0),
                    description: "Minimum difference to apply sharpening".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 4,
        }
    }
    
    async fn process(&self, frame: &Frame, ctx: &ProcessingContext) -> FilterResult<Frame> {
        // Create blurred version
        let blur_filter = GaussianBlurFilter::new(self.radius, self.radius / 2.0);
        let blurred = blur_filter.process(frame, ctx).await?;
        
        let mut output = frame.clone();
        
        // Unsharp mask: output = original + amount * (original - blurred)
        for i in 0..frame.data.len() / 4 {
            let idx = i * 4;
            for c in 0..3 {
                let original = frame.data[idx + c] as f32;
                let blur = blurred.data[idx + c] as f32;
                let diff = (original - blur).abs();
                
                if diff > self.threshold {
                    let sharpened = original + self.amount * (original - blur);
                    output.data_mut()[idx + c] = sharpened.clamp(0.0, 255.0) as u8;
                }
            }
        }
        
        Ok(output)
    }
    
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> FilterResult<()> {
        match name {
            "amount" => {
                if let FilterParameter::Float(v) = value {
                    self.amount = v.clamp(0.0, 5.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "radius" => {
                if let FilterParameter::Float(v) = value {
                    self.radius = v.clamp(0.1, 10.0);
                    Ok(())
                } else {
                    Err(FilterError::invalid_param(name, "Expected float value"))
                }
            }
            "threshold" => {
                if let FilterParameter::Float(v) = value {
                    self.threshold = v.clamp(0.0, 255.0);
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
            "amount" => Some(&FilterParameter::Float(self.amount)),
            "radius" => Some(&FilterParameter::Float(self.radius)),
            "threshold" => Some(&FilterParameter::Float(self.threshold)),
            _ => None,
        }
    }
    
    fn get_parameters(&self) -> HashMap<String, FilterParameter> {
        let mut params = HashMap::new();
        params.insert("amount".to_string(), FilterParameter::Float(self.amount));
        params.insert("radius".to_string(), FilterParameter::Float(self.radius));
        params.insert("threshold".to_string(), FilterParameter::Float(self.threshold));
        params
    }
    
    fn clone_filter(&self) -> Box<dyn VideoFilter> {
        Box::new(self.clone())
    }
}

// ============================================================================
// Color Lookup Table (LUT) Filter
// ============================================================================

/// 3D LUT color grading filter
#[derive(Debug, Clone)]
pub struct LutFilter {
    /// LUT size (typically 17, 33, or 65)
    lut_size: usize,
    /// LUT data as RGB values
    lut_data: Vec<[u8; 3]>,
    /// LUT name
    name: String,
    /// Intensity (0.0 to 1.0)
    intensity: f32,
}

impl LutFilter {
    pub fn new(name: String, lut_size: usize, lut_data: Vec<[u8; 3]>) -> Self {
        Self {
            lut_size,
            lut_data,
            name,
            intensity: 1.0,
        }
    }
    
    /// Load a LUT from a CUBE file
    pub fn from_cube_file(content: &str) -> FilterResult<Self> {
        let mut lut_size = 33usize;
        let mut lut_data = Vec::new();
        let mut name = "Custom LUT".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            if line.starts_with("TITLE") {
                name = line.split('"').nth(1).unwrap_or("Custom LUT").to_string();
            } else if line.starts_with("LUT_3D_SIZE") {
                lut_size = line.split_whitespace().last()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(33);
            } else {
                let parts: Vec<f32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if parts.len() == 3 {
                    lut_data.push([
                        (parts[0] * 255.0) as u8,
                        (parts[1] * 255.0) as u8,
                        (parts[2] * 255.0) as u8,
                    ]);
                }
            }
        }
        
        if lut_data.is_empty() {
            return Err(FilterError::ProcessingFailed("Empty LUT data".to_string()));
        }
        
        Ok(Self::new(name, lut_size, lut_data))
    }
}

#[async_trait]
impl VideoFilter for LutFilter {
    fn info(&self) -> FilterInfo {
        FilterInfo {
            id: "lut".to_string(),
            name: format!("LUT: {}", self.name),
            category: FilterCategory::ColorCorrection,
            description: "Apply 3D color lookup table for color grading".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "intensity".to_string(),
                    param_type: ParameterType::Float,
                    default: FilterParameter::Float(1.0),
                    min: Some(0.0),
                    max: Some(1.0),
                    description: "LUT intensity (0.0 to 1.0)".to_string(),
                },
            ],
            gpu_accelerated: true,
            cost: 4,
        }
    }
    
    async fn process(&self, frame: &Frame, _ctx: &ProcessingContext) -> FilterResult<Frame> {
        let mut output = frame.clone();
        let size = self.lut_size as f32;
        let scale = (size - 1) as f32 / 255.0;
        
        for pixel in output.data_mut().chunks_mut(4) {
            let r = pixel[0] as f32 * scale;
            let g = pixel[1] as f32 * scale;
            let b = pixel[2] as f32 * scale;
            
            // Trilinear interpolation (simplified)
            let r0 = r.floor() as usize;
            let g0 = g.floor() as usize;
            let b0 = b.floor() as usize;
            
            let idx = r0 + g0 * self.lut_size + b0 * self.lut_size * self.lut_size;
            
            if idx < self.lut_data.len() {
                let lut_color = self.lut_data[idx];
                
                // Blend with original based on intensity
                pixel[0] = ((1.0 - self.intensity) * pixel[0] as f32 + self.intensity * lut_color[0] as f32) as u8;
                pixel[1] = ((1.0 - self.intensity) * pixel[1] as f32 + self.intensity * lut_color[1] as f32) as u8;
                pixel[2] = ((1.0 - self.intensity) * pixel[2] as f32 + self.intensity * lut_color[2] as f32) as u8;
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

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert RGB to HSL
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    
    if max == min {
        return (0.0, 0.0, l);
    }
    
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } / 6.0;
    
    (h, s, l)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        return ((l * 255.0) as u8, (l * 255.0) as u8, (l * 255.0) as u8);
    }
    
    fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
        let t = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };
        if t < 1.0 / 6.0 { p + (q - p) * 6.0 * t }
        else if t < 1.0 / 2.0 { q }
        else if t < 2.0 / 3.0 { p + (q - p) * (2.0 / 3.0 - t) * 6.0 }
        else { p }
    }
    
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0;
    let g = hue_to_rgb(p, q, h) * 255.0;
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0;
    
    (r as u8, g as u8, b as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_brightness_filter() {
        let frame = Frame::new(10, 10, crate::PixelFormat::RGBA);
        let filter = BrightnessFilter::new(0.2);
        let result = filter.process(&frame, &ProcessingContext::default()).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_contrast_filter() {
        let frame = Frame::new(10, 10, crate::PixelFormat::RGBA);
        let filter = ContrastFilter::new(1.5);
        let result = filter.process(&frame, &ProcessingContext::default()).await;
        assert!(result.is_ok());
    }
}