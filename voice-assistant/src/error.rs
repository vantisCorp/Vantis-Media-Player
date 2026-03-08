//! Error types for Voice Assistant integration

use thiserror::Error;

/// Main error type for voice assistant operations
#[derive(Debug, Error)]
pub enum VoiceAssistantError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Account linking failed: {0}")]
    AccountLinkingFailed(String),
    
    #[error("Invalid request format: {0}")]
    InvalidRequest(String),
    
    #[error("Intent not recognized: {0}")]
    IntentNotRecognized(String),
    
    #[error("Media not found: {0}")]
    MediaNotFound(String),
    
    #[error("Playback error: {0}")]
    PlaybackError(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Internal error: {0}")]
    InternalError(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    #[error("Voice assistant not available: {0}")]
    AssistantNotAvailable(String),
}

impl VoiceAssistantError {
    /// Create a new authentication failed error
    pub fn auth_failed(msg: impl Into<String>) -> Self {
        Self::AuthenticationFailed(msg.into())
    }
    
    /// Create a new invalid request error
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self::InvalidRequest(msg.into())
    }
    
    /// Create a new media not found error
    pub fn media_not_found(msg: impl Into<String>) -> Self {
        Self::MediaNotFound(msg.into())
    }
    
    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::RateLimitExceeded => true,
            Self::TokenExpired => true,
            Self::ServiceUnavailable(_) => true,
            Self::HttpError(_) => true,
            _ => false,
        }
    }
    
    /// Get error code for the error
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed(_) => "AUTH_FAILED",
            Self::AccountLinkingFailed(_) => "ACCOUNT_LINKING_FAILED",
            Self::InvalidRequest(_) => "INVALID_REQUEST",
            Self::IntentNotRecognized(_) => "INTENT_NOT_RECOGNIZED",
            Self::MediaNotFound(_) => "MEDIA_NOT_FOUND",
            Self::PlaybackError(_) => "PLAYBACK_ERROR",
            Self::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::PermissionDenied(_) => "PERMISSION_DENIED",
            Self::ConfigurationError(_) => "CONFIGURATION_ERROR",
            Self::HttpError(_) => "HTTP_ERROR",
            Self::JsonError(_) => "JSON_ERROR",
            Self::IoError(_) => "IO_ERROR",
            Self::InternalError(_) => "INTERNAL_ERROR",
            Self::UnsupportedOperation(_) => "UNSUPPORTED_OPERATION",
            Self::AssistantNotAvailable(_) => "ASSISTANT_NOT_AVAILABLE",
        }
    }
}

/// Result type alias for voice assistant operations
pub type VoiceResult<T> = Result<T, VoiceAssistantError>;