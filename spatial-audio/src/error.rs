//! Error types for spatial audio

use thiserror::Error;

/// Error type for spatial audio operations
#[derive(Debug, Error)]
pub enum SpatialAudioError {
    #[error("Invalid sample rate: {0}")]
    InvalidSampleRate(u32),
    
    #[error("Invalid channel count: {0}")]
    InvalidChannelCount(usize),
    
    #[error("Buffer size mismatch: expected {0}, got {1}")]
    BufferSizeMismatch(usize, usize),
    
    #[error("HRTF error: {0}")]
    HrtfError(String),
    
    #[error("Ambisonics error: {0}")]
    AmbisonicsError(String),
    
    #[error("Reverb error: {0}")]
    ReverbError(String),
    
    #[error("FFT error: {0}")]
    FftError(String),
    
    #[error("Source not found: {0}")]
    SourceNotFound(String),
    
    #[error("Invalid position: {0}")]
    InvalidPosition(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl SpatialAudioError {
    pub fn hrtf(msg: impl Into<String>) -> Self {
        Self::HrtfError(msg.into())
    }
    
    pub fn ambisonics(msg: impl Into<String>) -> Self {
        Self::AmbisonicsError(msg.into())
    }
    
    pub fn processing(msg: impl Into<String>) -> Self {
        Self::ProcessingError(msg.into())
    }
}