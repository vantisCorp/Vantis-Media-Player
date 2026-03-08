//! Filter pipeline for chaining multiple filters

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::{
    Frame, FilterResult, FilterError, FilterInfo, FilterStats,
    FilterParameter, ProcessingContext, filters::VideoFilter,
};

/// Maximum number of filters in a pipeline
const MAX_PIPELINE_DEPTH: usize = 32;

/// Filter pipeline for processing frames through multiple filters
pub struct FilterPipeline {
    /// List of filters in processing order
    filters: Vec<Box<dyn VideoFilter>>,
    
    /// Pipeline name
    name: String,
    
    /// Whether to stop on first error
    fail_fast: bool,
    
    /// Processing statistics
    stats: FilterStats,
    
    /// Enabled/disabled filter indices
    enabled: Vec<bool>,
}

impl FilterPipeline {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            name: "Default Pipeline".to_string(),
            fail_fast: true,
            stats: FilterStats::new(),
            enabled: Vec::new(),
        }
    }
    
    /// Create a pipeline with a name
    pub fn with_name(name: impl Into<String>) -> Self {
        let mut pipeline = Self::new();
        pipeline.name = name.into();
        pipeline
    }
    
    /// Add a filter to the end of the pipeline
    pub fn add_filter(&mut self, filter: Box<dyn VideoFilter>) -> FilterResult<()> {
        if self.filters.len() >= MAX_PIPELINE_DEPTH {
            return Err(FilterError::ChainTooDeep(MAX_PIPELINE_DEPTH));
        }
        self.filters.push(filter);
        self.enabled.push(true);
        Ok(())
    }
    
    /// Insert a filter at a specific position
    pub fn insert_filter(&mut self, index: usize, filter: Box<dyn VideoFilter>) -> FilterResult<()> {
        if self.filters.len() >= MAX_PIPELINE_DEPTH {
            return Err(FilterError::ChainTooDeep(MAX_PIPELINE_DEPTH));
        }
        if index > self.filters.len() {
            return Err(FilterError::PipelineError("Invalid filter index".to_string()));
        }
        self.filters.insert(index, filter);
        self.enabled.insert(index, true);
        Ok(())
    }
    
    /// Remove a filter by index
    pub fn remove_filter(&mut self, index: usize) -> Option<Box<dyn VideoFilter>> {
        if index < self.filters.len() {
            self.enabled.remove(index);
            self.filters.remove(index)
        } else {
            None
        }
    }
    
    /// Enable or disable a filter
    pub fn set_filter_enabled(&mut self, index: usize, enabled: bool) {
        if index < self.enabled.len() {
            self.enabled[index] = enabled;
        }
    }
    
    /// Check if a filter is enabled
    pub fn is_filter_enabled(&self, index: usize) -> bool {
        self.enabled.get(index).copied().unwrap_or(false)
    }
    
    /// Get the number of filters
    pub fn len(&self) -> usize {
        self.filters.len()
    }
    
    /// Check if the pipeline is empty
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
    
    /// Get a filter by index
    pub fn get_filter(&self, index: usize) -> Option<&Box<dyn VideoFilter>> {
        self.filters.get(index)
    }
    
    /// Get a mutable reference to a filter
    pub fn get_filter_mut(&mut self, index: usize) -> Option<&mut Box<dyn VideoFilter>> {
        self.filters.get_mut(index)
    }
    
    /// Set a filter parameter by filter index and parameter name
    pub fn set_parameter(&mut self, filter_index: usize, param_name: &str, value: FilterParameter) -> FilterResult<()> {
        let filter = self.filters.get_mut(filter_index)
            .ok_or_else(|| FilterError::FilterNotFound(format!("Filter at index {}", filter_index)))?;
        filter.set_parameter(param_name, value)
    }
    
    /// Process a frame through all enabled filters
    pub async fn process_frame(&mut self, frame: &Frame, ctx: &ProcessingContext) -> FilterResult<Frame> {
        let start = Instant::now();
        let mut current_frame = frame.clone();
        
        for (i, filter) in self.filters.iter().enumerate() {
            if !self.enabled[i] {
                continue;
            }
            
            match filter.process(&current_frame, ctx).await {
                Ok(output) => current_frame = output,
                Err(e) if self.fail_fast => {
                    return Err(e);
                }
                Err(e) => {
                    tracing::warn!("Filter {} failed: {}", filter.info().name, e);
                    continue;
                }
            }
        }
        
        self.stats.record_frame(start.elapsed());
        Ok(current_frame)
    }
    
    /// Process multiple frames
    pub async fn process_frames<'a, I>(&mut self, frames: I, ctx: &ProcessingContext) -> FilterResult<Vec<Frame>>
    where
        I: IntoIterator<Item = &'a Frame>,
    {
        let mut results = Vec::new();
        for frame in frames {
            results.push(self.process_frame(frame, ctx).await?);
        }
        Ok(results)
    }
    
    /// Get processing statistics
    pub fn stats(&self) -> &FilterStats {
        &self.stats
    }
    
    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = FilterStats::new();
    }
    
    /// Get the pipeline name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Set whether to stop on first error
    pub fn set_fail_fast(&mut self, fail_fast: bool) {
        self.fail_fast = fail_fast;
    }
    
    /// Get all filter info
    pub fn filter_infos(&self) -> Vec<FilterInfo> {
        self.filters.iter().map(|f| f.info()).collect()
    }
    
    /// Clear all filters
    pub fn clear(&mut self) {
        self.filters.clear();
        self.enabled.clear();
    }
    
    /// Clone the pipeline with all filters
    pub fn clone_pipeline(&self) -> Self {
        Self {
            filters: self.filters.iter().map(|f| f.clone_filter()).collect(),
            name: self.name.clone(),
            fail_fast: self.fail_fast,
            stats: self.stats.clone(),
            enabled: self.enabled.clone(),
        }
    }
}

impl Default for FilterPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline name
    pub name: String,
    
    /// Filter configurations in order
    pub filters: Vec<FilterConfig>,
    
    /// Fail fast setting
    pub fail_fast: bool,
}

impl PipelineConfig {
    /// Create a new pipeline config
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            filters: Vec::new(),
            fail_fast: true,
        }
    }
    
    /// Add a filter config
    pub fn add_filter(&mut self, config: FilterConfig) {
        self.filters.push(config);
    }
    
    /// Export to JSON
    pub fn to_json(&self) -> FilterResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| FilterError::PipelineError(format!("Failed to serialize: {}", e)))
    }
    
    /// Import from JSON
    pub fn from_json(json: &str) -> FilterResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| FilterError::PipelineError(format!("Failed to deserialize: {}", e)))
    }
}

/// Serializable filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// Filter type identifier
    pub filter_type: String,
    
    /// Filter name (optional)
    pub name: Option<String>,
    
    /// Whether the filter is enabled
    pub enabled: bool,
    
    /// Parameter values
    pub parameters: HashMap<String, FilterParameter>,
}

impl FilterConfig {
    /// Create a new filter config
    pub fn new(filter_type: impl Into<String>) -> Self {
        Self {
            filter_type: filter_type.into(),
            name: None,
            enabled: true,
            parameters: HashMap::new(),
        }
    }
    
    /// Set the filter name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set a parameter
    pub fn with_parameter(mut self, name: impl Into<String>, value: FilterParameter) -> Self {
        self.parameters.insert(name.into(), value);
        self
    }
    
    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Filter registry for creating filters by type
pub struct FilterFactory {
    creators: HashMap<String, fn() -> Box<dyn VideoFilter>>,
}

impl FilterFactory {
    /// Create a new filter factory
    pub fn new() -> Self {
        Self {
            creators: HashMap::new(),
        }
    }
    
    /// Register a filter creator
    pub fn register(&mut self, filter_type: &str, creator: fn() -> Box<dyn VideoFilter>) {
        self.creators.insert(filter_type.to_string(), creator);
    }
    
    /// Create a filter by type
    pub fn create(&self, filter_type: &str) -> Option<Box<dyn VideoFilter>> {
        self.creators.get(filter_type).map(|f| f())
    }
    
    /// Create a filter from config
    pub fn create_from_config(&self, config: &FilterConfig) -> FilterResult<Box<dyn VideoFilter>> {
        let mut filter = self.create(&config.filter_type)
            .ok_or_else(|| FilterError::FilterNotFound(config.filter_type.clone()))?;
        
        for (name, value) in &config.parameters {
            filter.set_parameter(name, value.clone())?;
        }
        
        Ok(filter)
    }
    
    /// List available filter types
    pub fn available_filters(&self) -> Vec<&str> {
        self.creators.keys().map(String::as_str).collect()
    }
}

impl Default for FilterFactory {
    fn default() -> Self {
        let mut factory = Self::new();
        
        // Register built-in filters
        factory.register("brightness", || Box::new(crate::filters::BrightnessFilter::new(0.0)));
        factory.register("contrast", || Box::new(crate::filters::ContrastFilter::new(1.0)));
        factory.register("saturation", || Box::new(crate::filters::SaturationFilter::new(1.0)));
        factory.register("hue", || Box::new(crate::filters::HueFilter::new(0.0)));
        factory.register("gamma", || Box::new(crate::filters::GammaFilter::new(1.0)));
        factory.register("gaussian_blur", || Box::new(crate::filters::GaussianBlurFilter::new(5.0, 2.0)));
        factory.register("box_blur", || Box::new(crate::filters::BoxBlurFilter::new(3)));
        factory.register("sharpen", || Box::new(crate::filters::SharpenFilter::new(1.0, 1.0, 0.0)));
        factory.register("film_grain", || Box::new(crate::effects::FilmGrainEffect::new(0.3, 1.0, false)));
        factory.register("vignette", || Box::new(crate::effects::VignetteEffect::new(0.5, 0.3, 1.0)));
        factory.register("sepia", || Box::new(crate::effects::SepiaEffect::new(1.0)));
        factory.register("vintage", || Box::new(crate::effects::VintageEffect::new(0.3, 0.2, 0.9)));
        factory.register("invert", || Box::new(crate::effects::InvertEffect::new(1.0)));
        factory.register("posterize", || Box::new(crate::effects::PosterizeEffect::new(4)));
        factory.register("threshold", || Box::new(crate::effects::ThresholdEffect::new(128, false)));
        factory.register("emboss", || Box::new(crate::effects::EmbossEffect::new(1.0)));
        factory.register("chromatic_aberration", || Box::new(crate::effects::ChromaticAberrationEffect::new(5.0, true)));
        
        factory
    }
}

/// Builder for creating pipelines
pub struct PipelineBuilder {
    pipeline: FilterPipeline,
}

impl PipelineBuilder {
    /// Create a new pipeline builder
    pub fn new() -> Self {
        Self {
            pipeline: FilterPipeline::new(),
        }
    }
    
    /// Set pipeline name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.pipeline.name = name.into();
        self
    }
    
    /// Add a filter
    pub fn filter(mut self, filter: Box<dyn VideoFilter>) -> FilterResult<Self> {
        self.pipeline.add_filter(filter)?;
        Ok(self)
    }
    
    /// Set fail fast
    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.pipeline.fail_fast = fail_fast;
        self
    }
    
    /// Build the pipeline
    pub fn build(self) -> FilterPipeline {
        self.pipeline
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply a preset to a pipeline
pub fn apply_preset(pipeline: &mut FilterPipeline, preset: &str) -> FilterResult<()> {
    let filters: Vec<Box<dyn VideoFilter>> = match preset {
        "cinematic" => crate::effects::presets::cinematic(),
        "horror" => crate::effects::presets::horror(),
        "dreamy" => crate::effects::presets::dreamy(),
        "vintage_70s" => crate::effects::presets::vintage_70s(),
        "noir" => crate::effects::presets::noir(),
        _ => return Err(FilterError::PipelineError(format!("Unknown preset: {}", preset))),
    };
    
    for filter in filters {
        pipeline.add_filter(filter)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelFormat;
    
    #[tokio::test]
    async fn test_empty_pipeline() {
        let mut pipeline = FilterPipeline::new();
        let frame = Frame::new(10, 10, PixelFormat::RGBA);
        let ctx = ProcessingContext::default();
        
        let result = pipeline.process_frame(&frame, &ctx).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_single_filter() {
        let mut pipeline = FilterPipeline::new();
        pipeline.add_filter(Box::new(crate::filters::BrightnessFilter::new(0.2))).unwrap();
        
        let frame = Frame::new(10, 10, PixelFormat::RGBA);
        let ctx = ProcessingContext::default();
        
        let result = pipeline.process_frame(&frame, &ctx).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_multiple_filters() {
        let mut pipeline = FilterPipeline::new();
        pipeline.add_filter(Box::new(crate::filters::BrightnessFilter::new(0.1))).unwrap();
        pipeline.add_filter(Box::new(crate::filters::ContrastFilter::new(1.2))).unwrap();
        pipeline.add_filter(Box::new(crate::filters::SaturationFilter::new(0.8))).unwrap();
        
        let frame = Frame::new(10, 10, PixelFormat::RGBA);
        let ctx = ProcessingContext::default();
        
        let result = pipeline.process_frame(&frame, &ctx).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_disable_filter() {
        let mut pipeline = FilterPipeline::new();
        pipeline.add_filter(Box::new(crate::filters::BrightnessFilter::new(0.5))).unwrap();
        pipeline.set_filter_enabled(0, false);
        
        let frame = Frame::new(10, 10, PixelFormat::RGBA);
        let ctx = ProcessingContext::default();
        
        let result = pipeline.process_frame(&frame, &ctx).await.unwrap();
        
        // Frame should be unchanged since filter is disabled
        assert_eq!(result.data, frame.data);
    }
    
    #[test]
    fn test_filter_factory() {
        let factory = FilterFactory::default();
        
        let filter = factory.create("brightness");
        assert!(filter.is_some());
        
        let filter = factory.create("nonexistent");
        assert!(filter.is_none());
    }
    
    #[test]
    fn test_pipeline_config() {
        let config = PipelineConfig::new("Test Pipeline")
            .to_json()
            .unwrap();
        
        assert!(config.contains("Test Pipeline"));
    }
}