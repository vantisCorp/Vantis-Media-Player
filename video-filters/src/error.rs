//! Error types for video filters

use thiserror::Error;

/// Error type for filter operations
#[derive(Debug, Error)]
pub enum FilterError {
    #[error("Invalid parameter '{0}': {1}")]
    InvalidParameter(String, String),
    
    #[error("Filter not found: {0}")]
    FilterNotFound(String),
    
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("Frame format not supported: {0}")]
    UnsupportedFormat(String),
    
    #[error("GPU processing failed: {0}")]
    GpuError(String),
    
    #[error("Memory allocation failed: requested {0} bytes")]
    MemoryError(usize),
    
    #[error("Pipeline error: {0}")]
    PipelineError(String),
    
    #[error("Invalid frame dimensions: {0}x{1}")]
    InvalidDimensions(u32, u32),
    
    #[error("Filter chain too deep (max {0})")]
    ChainTooDeep(usize),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

impl FilterError {
    pub fn invalid_param(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidParameter(name.into(), reason.into())
    }
    
    pub fn processing_failed(msg: impl Into<String>) -> Self {
        Self::ProcessingFailed(msg.into())
    }
    
    pub fn unsupported_format(format: impl Into<String>) -> Self {
        Self::UnsupportedFormat(format.into())
    }
    
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::InvalidParameter(_, _) |
            Self::MemoryError(_) |
            Self::ProcessingFailed(_)
        )
    }
}