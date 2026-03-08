//! Error types for live streaming

use thiserror::Error;

/// Live streaming error types
#[derive(Debug, Error)]
pub enum LiveStreamingError {
    #[error("Stream not found: {0}")]
    StreamNotFound(String),

    #[error("Broadcaster not found: {0}")]
    BroadcasterNotFound(String),

    #[error("Viewer not found: {0}")]
    ViewerNotFound(String),

    #[error("Stream key invalid")]
    InvalidStreamKey,

    #[error("Stream is not live")]
    StreamNotLive,

    #[error("Stream already live")]
    StreamAlreadyLive,

    #[error("Stream has ended")]
    StreamEnded,

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Ingestion error: {0}")]
    IngestionError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Chat is disabled")]
    ChatDisabled,

    #[error("User is banned from chat")]
    UserBanned,

    #[error("User is timed out for {0} seconds")]
    UserTimeout(u32),

    #[error("Chat message too long (max {max} characters)")]
    MessageTooLong { max: usize },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Slow mode active - wait {0} seconds")]
    SlowModeActive(u32),

    #[error("Followers only mode - follow required")]
    FollowersOnlyMode,

    #[error("Subscribers only mode")]
    SubscribersOnlyMode,

    #[error("Emote only mode")]
    EmoteOnlyMode,

    #[error("Not authorized: {0}")]
    Unauthorized(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Poll not found")]
    PollNotFound,

    #[error("Poll is not active")]
    PollNotActive,

    #[error("Prediction not found")]
    PredictionNotFound,

    #[error("Prediction is not active")]
    PredictionNotActive,

    #[error("Invalid poll choice")]
    InvalidPollChoice,

    #[error("Invalid prediction outcome")]
    InvalidPredictionOutcome,

    #[error("Insufficient points: required {required}, have {have}")]
    InsufficientPoints { required: u64, have: u64 },

    #[error("Hype train not active")]
    HypeTrainNotActive,

    #[error("Goal not found")]
    GoalNotFound,

    #[error("Clip creation failed: {0}")]
    ClipCreationFailed(String),

    #[error("VOD not found")]
    VodNotFound,

    #[error("VOD processing")]
    VodProcessing,

    #[error("Raid already in progress")]
    RaidInProgress,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

/// Result type alias for live streaming operations
pub type LiveStreamingResult<T> = Result<T, LiveStreamingError>;