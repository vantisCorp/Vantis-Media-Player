//! Vantis Cloud Sync Module
//! 
//! Provides cloud synchronization services for Vantis Media Player,
//! including settings sync, playlist sync, and cross-device synchronization.

pub mod auth;
pub mod sync;
pub mod storage;
pub mod models;
pub mod config;

pub use auth::{AuthProvider, User, Session};
pub use sync::{SyncService, SyncStatus, SyncConflict};
pub use storage::{CloudStorage, StorageProvider};
pub use models::{SyncData, Playlist, Settings};
pub use config::CloudConfig;

/// Cloud sync error types
#[derive(Debug, thiserror::Error)]
pub enum CloudError {
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Sync failed: {0}")]
    SyncError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Conflict detected: {0}")]
    ConflictError(String),
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Quota exceeded")]
    QuotaExceeded,
}

/// Result type for cloud operations
pub type CloudResult<T> = Result<T, CloudError>;