//! Error types for the AI recommendations module

use std::fmt;
use thiserror::Error;

/// Main error type for recommendation operations
#[derive(Error, Debug)]
pub enum RecommendationError {
    /// Content not found in the catalog
    #[error("Content not found: {0}")]
    ContentNotFound(String),

    /// User not found in the system
    #[error("User not found: {0}")]
    UserNotFound(String),

    /// Insufficient data for recommendation
    #[error("Insufficient data for recommendation: {0}")]
    InsufficientData(String),

    /// Feature extraction failed
    #[error("Feature extraction failed: {0}")]
    FeatureExtractionFailed(String),

    /// Model training failed
    #[error("Model training failed: {0}")]
    ModelTrainingFailed(String),

    /// Similarity calculation error
    #[error("Similarity calculation error: {0}")]
    SimilarityError(String),

    /// Clustering error
    #[error("Clustering error: {0}")]
    ClusteringError(String),

    /// Cache operation failed
    #[error("Cache operation failed: {0}")]
    CacheError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Data parsing error
    #[error("Data parsing error: {0}")]
    ParsingError(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// NaN or invalid numerical value
    #[error("Invalid numerical value: {0}")]
    InvalidNumericalValue(String),

    /// Empty result set
    #[error("Empty result set: {0}")]
    EmptyResult(String),

    /// Operation timeout
    #[error("Operation timeout: {0}")]
    Timeout(String),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type alias for recommendation operations
pub type Result<T> = std::result::Result<T, RecommendationError>;

impl RecommendationError {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            RecommendationError::InsufficientData(_)
                | RecommendationError::CacheError(_)
                | RecommendationError::Timeout(_)
        )
    }

    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            RecommendationError::ContentNotFound(_)
            | RecommendationError::UserNotFound(_) => "not_found",
            RecommendationError::InsufficientData(_) => "data",
            RecommendationError::FeatureExtractionFailed(_)
            | RecommendationError::ModelTrainingFailed(_)
            | RecommendationError::SimilarityError(_)
            | RecommendationError::ClusteringError(_) => "processing",
            RecommendationError::CacheError(_) => "cache",
            RecommendationError::InvalidConfiguration(_) => "config",
            RecommendationError::ParsingError(_) | RecommendationError::IoError(_) => "io",
            RecommendationError::InvalidNumericalValue(_) | RecommendationError::EmptyResult(_) => {
                "validation"
            }
            RecommendationError::Timeout(_) => "timeout",
            RecommendationError::InternalError(_) => "internal",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = RecommendationError::ContentNotFound("movie-123".to_string());
        assert_eq!(err.to_string(), "Content not found: movie-123");
    }

    #[test]
    fn test_error_recoverable() {
        assert!(RecommendationError::InsufficientData("test".to_string()).is_recoverable());
        assert!(!RecommendationError::ContentNotFound("test".to_string()).is_recoverable());
    }

    #[test]
    fn test_error_category() {
        assert_eq!(
            RecommendationError::ContentNotFound("test".to_string()).category(),
            "not_found"
        );
        assert_eq!(
            RecommendationError::InsufficientData("test".to_string()).category(),
            "data"
        );
    }
}