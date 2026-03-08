//! Error types for the social module

use thiserror::Error;

/// Social module errors
#[derive(Debug, Error)]
pub enum SocialError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Post not found: {0}")]
    PostNotFound(String),

    #[error("Comment not found: {0}")]
    CommentNotFound(String),

    #[error("Conversation not found: {0}")]
    ConversationNotFound(String),

    #[error("Message not found: {0}")]
    MessageNotFound(String),

    #[error("Group not found: {0}")]
    GroupNotFound(String),

    #[error("Friendship already exists between users")]
    FriendshipAlreadyExists,

    #[error("Friendship request not found")]
    FriendshipNotFound,

    #[error("Cannot friend yourself")]
    SelfFriendship,

    #[error("User is blocked")]
    UserBlocked,

    #[error("User has blocked you")]
    YouAreBlocked,

    #[error("Privacy settings prevent this action")]
    PrivacyViolation,

    #[error("Not authorized to perform this action")]
    Unauthorized,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("User already in group")]
    AlreadyInGroup,

    #[error("User not in group")]
    NotInGroup,

    #[error("Group is full")]
    GroupFull,

    #[error("Content too long (max {max} characters)")]
    ContentTooLong { max: usize },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid media reference: {0}")]
    InvalidMediaReference(String),

    #[error("External platform error: {0}")]
    ExternalPlatformError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type alias for social operations
pub type SocialResult<T> = Result<T, SocialError>;

impl From<serde_json::Error> for SocialError {
    fn from(e: serde_json::Error) -> Self {
        SocialError::InternalError(e.to_string())
    }
}

impl From<std::io::Error> for SocialError {
    fn from(e: std::io::Error) -> Self {
        SocialError::InternalError(e.to_string())
    }
}