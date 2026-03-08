//! Collaborative Viewing Module
//! 
//! Provides real-time synchronized viewing sessions for Vantis Media Player,
//! enabling users to watch content together from different locations.

pub mod session;
pub mod participant;
pub mod sync;
pub mod chat;
pub mod reaction;
pub mod config;

pub use session::{ViewingSession, SessionState};
pub use participant::{Participant, ParticipantRole};
pub use sync::{SyncManager, SyncState};
pub use chat::{ChatMessage, ChatManager};
pub use reaction::{Reaction, ReactionType};
pub use config::CollaborationConfig;

/// Collaboration error types
#[derive(Debug, thiserror::Error)]
pub enum CollaborationError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Session full")]
    SessionFull,
    
    #[error("Not authorized: {0}")]
    NotAuthorized(String),
    
    #[error("Participant not found: {0}")]
    ParticipantNotFound(String),
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Timeout")]
    Timeout,
}

/// Result type for collaboration operations
pub type CollaborationResult<T> = Result<T, CollaborationError>;

/// Session ID type
pub type SessionId = String;

/// Participant ID type
pub type ParticipantId = String;