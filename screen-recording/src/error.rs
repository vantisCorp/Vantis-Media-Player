//! Error types for screen recording module

use thiserror::Error;

/// Result type for screen recording operations
pub type ScreenRecordingResult<T> = Result<T, ScreenRecordingError>;

/// Screen recording error types
#[derive(Debug, Error)]
pub enum ScreenRecordingError {
    #[error("Recording not found: {0}")]
    RecordingNotFound(String),

    #[error("Capture source not available: {0}")]
    SourceNotAvailable(String),

    #[error("Recording already in progress")]
    AlreadyRecording,

    #[error("No recording in progress")]
    NotRecording,

    #[error("Failed to start recording: {0}")]
    StartFailed(String),

    #[error("Failed to stop recording: {0}")]
    StopFailed(String),

    #[error("Failed to pause recording: {0}")]
    PauseFailed(String),

    #[error("Failed to capture frame: {0}")]
    CaptureFailed(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Output directory not found: {0}")]
    OutputDirectoryNotFound(String),

    #[error("Insufficient disk space: required {required}MB, available {available}MB")]
    InsufficientDiskSpace { required: u64, available: u64 },

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),

    #[error("Hardware encoding not available: {0}")]
    HardwareEncodingNotAvailable(String),

    #[error("Window not found: {0}")]
    WindowNotFound(String),

    #[error("Screen not found: {0}")]
    ScreenNotFound(String),

    #[error("Webcam not found: {0}")]
    WebcamNotFound(String),

    #[error("Microphone not found: {0}")]
    MicrophoneNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid crop region: {0}")]
    InvalidCropRegion(String),

    #[error("Overlay error: {0}")]
    OverlayError(String),

    #[error("Hotkey registration failed: {0}")]
    HotkeyRegistrationFailed(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<ScreenRecordingError> for anyhow::Error {
    fn from(err: ScreenRecordingError) -> Self {
        anyhow::anyhow!(err.to_string())
    }
}