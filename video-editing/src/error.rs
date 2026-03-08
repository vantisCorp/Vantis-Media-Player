//! Error types for video editing module

use thiserror::Error;

/// Result type for video editing operations
pub type VideoEditingResult<T> = Result<T, VideoEditingError>;

/// Video editing error types
#[derive(Debug, Error)]
pub enum VideoEditingError {
    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Track not found: {0}")]
    TrackNotFound(String),

    #[error("Clip not found: {0}")]
    ClipNotFound(String),

    #[error("Effect not found: {0}")]
    EffectNotFound(String),

    #[error("Invalid time range: start {start} exceeds end {end}")]
    InvalidTimeRange { start: f64, end: f64 },

    #[error("Track is locked: {0}")]
    TrackLocked(String),

    #[error("Track is muted: {0}")]
    TrackMuted(String),

    #[error("Clip overlap detected at position {position}")]
    ClipOverlap { position: f64 },

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Media source not found: {0}")]
    MediaSourceNotFound(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),

    #[error("Render failed: {0}")]
    RenderFailed(String),

    #[error("Encoding failed: {0}")]
    EncodingFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Keyframe interpolation error: {0}")]
    KeyframeInterpolationError(String),

    #[error("Effect application failed: {0}")]
    EffectApplicationFailed(String),

    #[error("Transition failed: {0}")]
    TransitionFailed(String),

    #[error("Insufficient memory for operation")]
    InsufficientMemory,

    #[error("Hardware acceleration not available: {0}")]
    HardwareAccelerationNotAvailable(String),

    #[error("Project corruption detected: {0}")]
    ProjectCorruption(String),

    #[error("Undo stack empty")]
    UndoStackEmpty,

    #[error("Redo stack empty")]
    RedoStackEmpty,

    #[error("Render cancelled by user")]
    RenderCancelled,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<VideoEditingError> for anyhow::Error {
    fn from(err: VideoEditingError) -> Self {
        anyhow::anyhow!(err.to_string())
    }
}