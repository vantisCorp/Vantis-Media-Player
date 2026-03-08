//! Video Filters and Effects for Vantis Media Player
//!
//! This module provides real-time video processing capabilities including:
//! - Color correction and grading
//! - Blur and sharpen effects
//! - Film grain and vintage effects
//! - GPU-accelerated processing (optional)
//!
//! # Example
//!
//! ```
//! use vantis_video_filters::{FilterPipeline, filters::{BrightnessFilter, ContrastFilter}};
//!
//! let mut pipeline = FilterPipeline::new();
//! pipeline.add_filter(Box::new(BrightnessFilter::new(0.2)));
//! pipeline.add_filter(Box::new(ContrastFilter::new(1.1)));
//!
//! let processed_frame = pipeline.process_frame(&input_frame)?;
//! ```

pub mod filters;
pub mod effects;
pub mod pipeline;
pub mod types;
pub mod error;

#[cfg(feature = "gpu")]
pub mod gpu;

pub use error::FilterError;
pub use types::{Frame, PixelFormat, FrameInfo, ColorSpace};
pub use pipeline::FilterPipeline;
pub use filters::VideoFilter;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Result type for filter operations
pub type FilterResult<T> = Result<T, FilterError>;

/// Filter parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterParameter {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    String(String),
    Color(ColorValue),
    Range(f32, f32),
}

/// RGBA color value
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorValue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for ColorValue {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

impl ColorValue {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            Some(Self::from_rgb(r, g, b))
        } else {
            None
        }
    }
}

/// Filter metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    /// Unique filter identifier
    pub id: String,
    
    /// Human-readable filter name
    pub name: String,
    
    /// Filter category
    pub category: FilterCategory,
    
    /// Description of what the filter does
    pub description: String,
    
    /// Available parameters
    pub parameters: Vec<ParameterInfo>,
    
    /// Whether this filter supports GPU acceleration
    pub gpu_accelerated: bool,
    
    /// Processing cost (1-10, higher = more expensive)
    pub cost: u8,
}

/// Filter category for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilterCategory {
    ColorCorrection,
    Blur,
    Sharpen,
    Artistic,
    Distortion,
    Noise,
    Stylize,
    Transform,
    Custom,
}

/// Parameter information for UI generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub param_type: ParameterType,
    
    /// Default value
    pub default: FilterParameter,
    
    /// Minimum value (for numeric types)
    pub min: Option<f32>,
    
    /// Maximum value (for numeric types)
    pub max: Option<f32>,
    
    /// Human-readable description
    pub description: String,
}

/// Types of filter parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    Float,
    Integer,
    Boolean,
    String,
    Color,
    Range,
    Enum { variants: Vec<String> },
}

/// Processing context for filters
#[derive(Debug, Clone)]
pub struct ProcessingContext {
    /// Frame number in the video
    pub frame_number: u64,
    
    /// Timestamp of the frame
    pub timestamp: Duration,
    
    /// Video duration
    pub video_duration: Duration,
    
    /// Frame rate
    pub frame_rate: f32,
    
    /// Target resolution
    pub target_resolution: (u32, u32),
    
    /// Whether to use GPU acceleration
    pub use_gpu: bool,
    
    /// Quality preset (1-10)
    pub quality: u8,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            frame_number: 0,
            timestamp: Duration::ZERO,
            video_duration: Duration::ZERO,
            frame_rate: 30.0,
            target_resolution: (1920, 1080),
            use_gpu: false,
            quality: 8,
        }
    }
}

/// Performance statistics for a filter
#[derive(Debug, Clone, Default)]
pub struct FilterStats {
    /// Total frames processed
    pub frames_processed: u64,
    
    /// Total processing time
    pub total_time: Duration,
    
    /// Average processing time per frame
    pub avg_time_per_frame: Duration,
    
    /// Peak memory usage in bytes
    pub peak_memory: usize,
}

impl FilterStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_frame(&mut self, processing_time: Duration) {
        self.frames_processed += 1;
        self.total_time += processing_time;
        self.avg_time_per_frame = self.total_time / self.frames_processed as u32;
    }
}