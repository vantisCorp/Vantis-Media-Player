//! Error types for Vantis Media Player

use thiserror::Error;

/// Main error type for the core library
#[derive(Error, Debug)]
pub enum VantisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Media error: {0}")]
    Media(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Playback error: {0}")]
    Playback(String),

    #[error("Decode error: {0}")]
    Decode(String),

    #[error("Encode error: {0}")]
    Encode(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for Vantis operations
pub type Result<T> = std::result::Result<T, VantisError>;

impl From<serde_json::Error> for VantisError {
    fn from(err: serde_json::Error) -> Self {
        VantisError::Config(err.to_string())
    }
}