//! Vantis AI - AI-powered features for Vantis Media Player
//! 
//! This module provides advanced AI capabilities including:
//! - Content-aware video enhancement
//! - Scene detection and chapter generation
//! - AI-powered audio enhancement
//! - Smart subtitle timing adjustment
//! - Content recommendation engine

pub mod enhancement;
pub mod scene_detection;
pub mod audio_enhancement;
pub mod subtitle_timing;
pub mod recommendation;
pub mod models;
pub mod utils;
pub mod video_tagging;

pub use enhancement::{VideoEnhancer, EnhancementConfig, EnhancementType};
pub use scene_detection::{SceneDetector, Chapter, Scene};
pub use audio_enhancement::{AudioEnhancer, AudioEnhancementConfig};
pub use subtitle_timing::{SubtitleTimingAdjuster, TimingAdjustment};
pub use recommendation::{RecommendationEngine, RecommendationConfig, Recommendation};
pub use models::{AIModel, ModelType, ModelManager};
pub use utils::{TensorOps, FeatureExtractor};

// Re-export video tagging types
pub use video_tagging::{
    TagCategory, VideoTag, TagSource, TaggingResult, TaggingConfig,
    AutomatedVideoTagger,
};

use thiserror::Error;

/// AI module error types
#[derive(Error, Debug)]
pub enum AIError {
    #[error("Model loading error: {0}")]
    ModelLoadError(String),
    
    #[error("Model inference error: {0}")]
    InferenceError(String),
    
    #[error("Feature extraction error: {0}")]
    FeatureExtractionError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("GPU not available")]
    GPUNotAvailable,
    
    #[error("Insufficient memory")]
    InsufficientMemory,
    
    #[error("Timeout")]
    Timeout,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for AI operations
pub type AIResult<T> = Result<T, AIError>;

/// AI module configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AIConfig {
    /// Enable GPU acceleration
    pub enable_gpu: bool,
    
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    
    /// Model cache directory
    pub model_cache_dir: String,
    
    /// Enable model downloading
    pub enable_model_download: bool,
    
    /// Model download URL
    pub model_download_url: String,
    
    /// Enable feature caching
    pub enable_feature_cache: bool,
    
    /// Feature cache directory
    pub feature_cache_dir: String,
    
    /// Maximum concurrent inferences
    pub max_concurrent_inferences: usize,
    
    /// Inference timeout in seconds
    pub inference_timeout_secs: u64,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enable_gpu: true,
            max_memory_mb: 4096,
            model_cache_dir: ".cache/models".to_string(),
            enable_model_download: true,
            model_download_url: "https://models.vantis.ai".to_string(),
            enable_feature_cache: true,
            feature_cache_dir: ".cache/features".to_string(),
            max_concurrent_inferences: 4,
            inference_timeout_secs: 30,
        }
    }
}

/// AI module initialization
pub struct AIEngine {
    config: AIConfig,
    model_manager: ModelManager,
}

impl AIEngine {
    /// Create a new AI engine with default configuration
    pub fn new() -> AIResult<Self> {
        Self::with_config(AIConfig::default())
    }
    
    /// Create a new AI engine with custom configuration
    pub fn with_config(config: AIConfig) -> AIResult<Self> {
        // Create cache directories
        std::fs::create_dir_all(&config.model_cache_dir)?;
        std::fs::create_dir_all(&config.feature_cache_dir)?;
        
        let model_manager = ModelManager::new(config.clone())?;
        
        Ok(Self {
            config,
            model_manager,
        })
    }
    
    /// Get the configuration
    pub fn config(&self) -> &AIConfig {
        &self.config
    }
    
    /// Get the model manager
    pub fn model_manager(&self) -> &ModelManager {
        &self.model_manager
    }
    
    /// Create a video enhancer
    pub fn video_enhancer(&self) -> AIResult<VideoEnhancer> {
        VideoEnhancer::new(self.config.clone())
    }
    
    /// Create a scene detector
    pub fn scene_detector(&self) -> AIResult<SceneDetector> {
        SceneDetector::new(self.config.clone())
    }
    
    /// Create an audio enhancer
    pub fn audio_enhancer(&self) -> AIResult<AudioEnhancer> {
        AudioEnhancer::new(self.config.clone())
    }
    
    /// Create a subtitle timing adjuster
    pub fn subtitle_timing_adjuster(&self) -> AIResult<SubtitleTimingAdjuster> {
        SubtitleTimingAdjuster::new(self.config.clone())
    }
    
    /// Create a recommendation engine
    pub fn recommendation_engine(&self) -> AIResult<RecommendationEngine> {
        RecommendationEngine::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_config_default() {
        let config = AIConfig::default();
        assert!(config.enable_gpu);
        assert_eq!(config.max_memory_mb, 4096);
    }
    
    #[test]
    fn test_ai_engine_creation() {
        let engine = AIEngine::new();
        assert!(engine.is_ok());
    }
}