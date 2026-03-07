//! Cloud configuration
//! 
//! Configuration options for cloud services.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cloud service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    /// API endpoint
    pub api_endpoint: String,
    
    /// WebSocket endpoint for real-time sync
    pub ws_endpoint: String,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// Retry configuration
    pub retry: RetryConfig,
    
    /// Sync configuration
    pub sync: SyncConfig,
    
    /// Storage configuration
    pub storage: StorageConfig,
    
    /// Cache configuration
    pub cache: CacheConfig,
    
    /// Offline mode enabled
    pub offline_mode: bool,
    
    /// Debug mode
    pub debug: bool,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Initial delay between retries
    pub initial_delay_ms: u64,
    
    /// Maximum delay between retries
    pub max_delay_ms: u64,
    
    /// Multiplier for exponential backoff
    pub multiplier: f32,
    
    /// Retryable error codes
    pub retryable_codes: Vec<u16>,
}

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Enable automatic sync
    pub auto_sync: bool,
    
    /// Sync interval in seconds
    pub interval_seconds: u64,
    
    /// Sync on startup
    pub sync_on_startup: bool,
    
    /// Sync on shutdown
    pub sync_on_shutdown: bool,
    
    /// Maximum conflicts to keep
    pub max_conflicts: u32,
    
    /// Conflict resolution strategy
    pub default_resolution: ConflictStrategy,
    
    /// Batch size for sync operations
    pub batch_size: u32,
    
    /// Enable delta sync
    pub delta_sync: bool,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// Always prefer local version
    PreferLocal,
    
    /// Always prefer cloud version
    PreferCloud,
    
    /// Use the newer version
    PreferNewer,
    
    /// Keep both versions
    KeepBoth,
    
    /// Require manual resolution
    Manual,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Encryption enabled
    pub encryption: bool,
    
    /// Compression enabled
    pub compression: bool,
    
    /// Compression level (1-9)
    pub compression_level: u32,
    
    /// Maximum file size for upload (bytes)
    pub max_file_size: u64,
    
    /// Chunk size for large files (bytes)
    pub chunk_size: u64,
    
    /// Enable deduplication
    pub deduplication: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable local cache
    pub enabled: bool,
    
    /// Maximum cache size in MB
    pub max_size_mb: u64,
    
    /// Cache directory
    pub directory: String,
    
    /// Cache expiration in hours
    pub expiration_hours: u64,
    
    /// Encrypt cached data
    pub encrypt: bool,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.vantis.cloud".to_string(),
            ws_endpoint: "wss://sync.vantis.cloud".to_string(),
            timeout_seconds: 30,
            retry: RetryConfig::default(),
            sync: SyncConfig::default(),
            storage: StorageConfig::default(),
            cache: CacheConfig::default(),
            offline_mode: false,
            debug: false,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            multiplier: 2.0,
            retryable_codes: vec![429, 500, 502, 503, 504],
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            interval_seconds: 300, // 5 minutes
            sync_on_startup: true,
            sync_on_shutdown: true,
            max_conflicts: 100,
            default_resolution: ConflictStrategy::PreferNewer,
            batch_size: 50,
            delta_sync: true,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            encryption: true,
            compression: true,
            compression_level: 6,
            max_file_size: 100 * 1024 * 1024, // 100MB
            chunk_size: 5 * 1024 * 1024, // 5MB
            deduplication: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 500,
            directory: "~/.vantis/cache".to_string(),
            expiration_hours: 168, // 1 week
            encrypt: true,
        }
    }
}

impl CloudConfig {
    /// Get request timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }
    
    /// Create development configuration
    pub fn development() -> Self {
        Self {
            api_endpoint: "http://localhost:8080".to_string(),
            ws_endpoint: "ws://localhost:8081".to_string(),
            debug: true,
            ..Self::default()
        }
    }
    
    /// Create staging configuration
    pub fn staging() -> Self {
        Self {
            api_endpoint: "https://staging-api.vantis.cloud".to_string(),
            ws_endpoint: "wss://staging-sync.vantis.cloud".to_string(),
            ..Self::default()
        }
    }
}