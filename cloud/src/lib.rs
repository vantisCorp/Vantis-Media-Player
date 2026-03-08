//! Vantis Cloud Sync Module
//! 
//! Provides cloud synchronization services for Vantis Media Player,
//! including settings sync, playlist sync, and cross-device synchronization.
//!
//! # Features
//! - Full sync engine with delta synchronization
//! - Advanced conflict resolution
//! - Device management and trust levels
//! - Backup and restore functionality
//! - Offline support with operation queue

pub mod auth;
pub mod sync;
pub mod sync_engine;
pub mod conflict_resolver;
pub mod device_manager;
pub mod backup;
pub mod offline;
pub mod storage;
pub mod models;
pub mod config;

pub use auth::{AuthProvider, User, Session};
pub use sync::{SyncService, SyncStatus, SyncConflict, SyncDataType};
pub use sync_engine::{SyncEngine, SyncEngineConfig, SyncEngineState, SyncEvent};
pub use conflict_resolver::{ConflictResolver, DefaultConflictResolver, ConflictSolution, ConflictRule};
pub use device_manager::{DeviceManager, Device, DeviceType, TrustLevel, DeviceCapabilities};
pub use backup::{BackupManager, BackupOptions, RestoreOptions, BackupMetadata};
pub use offline::{OfflineManager, OfflineStatus, QueuedOperation};
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