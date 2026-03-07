//! Cloud storage abstraction
//! 
//! Provides unified interface for various cloud storage providers.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::path::Path;

/// Cloud storage provider trait
#[async_trait]
pub trait CloudStorage: Send + Sync {
    /// Upload data to cloud
    async fn upload(&self, key: &str, data: &[u8]) -> Result<StorageObject>;
    
    /// Download data from cloud
    async fn download(&self, key: &str) -> Result<Vec<u8>>;
    
    /// Delete object from cloud
    async fn delete(&self, key: &str) -> Result<()>;
    
    /// List objects with prefix
    async fn list(&self, prefix: &str) -> Result<Vec<StorageObject>>;
    
    /// Get object metadata
    async fn get_metadata(&self, key: &str) -> Result<ObjectMetadata>;
    
    /// Check if object exists
    async fn exists(&self, key: &str) -> Result<bool>;
    
    /// Copy object
    async fn copy(&self, source: &str, destination: &str) -> Result<StorageObject>;
    
    /// Get signed URL for direct upload
    async fn get_upload_url(&self, key: &str, expires_in: u64) -> Result<String>;
    
    /// Get signed URL for direct download
    async fn get_download_url(&self, key: &str, expires_in: u64) -> Result<String>;
}

/// Storage object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageObject {
    /// Object key (path)
    pub key: String,
    
    /// Object size in bytes
    pub size: u64,
    
    /// Content type (MIME type)
    pub content_type: String,
    
    /// ETag for versioning
    pub etag: String,
    
    /// Last modified timestamp
    pub last_modified: chrono::DateTime<chrono::Utc>,
    
    /// Storage class
    pub storage_class: StorageClass,
    
    /// Custom metadata
    pub metadata: ObjectMetadata,
}

/// Object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    /// Content encoding
    pub content_encoding: Option<String>,
    
    /// Cache control
    pub cache_control: Option<String>,
    
    /// Content disposition
    pub content_disposition: Option<String>,
    
    /// Custom headers
    pub custom: std::collections::HashMap<String, String>,
}

/// Storage class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageClass {
    /// Standard storage (hot)
    Standard,
    
    /// Infrequent access (warm)
    InfrequentAccess,
    
    /// Archive storage (cold)
    Archive,
    
    /// Glacier-style deep archive
    DeepArchive,
}

/// Storage provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageProvider {
    AWS(AWSConfig),
    GCP(GCPConfig),
    Azure(AzureConfig),
    Cloudflare(CloudflareConfig),
    Backblaze(BackblazeConfig),
    SelfHosted(SelfHostedConfig),
}

/// AWS S3 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AWSConfig {
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint: Option<String>,
}

/// Google Cloud Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCPConfig {
    pub bucket: String,
    pub project_id: String,
    pub credentials_json: String,
}

/// Azure Blob Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    pub container: String,
    pub connection_string: String,
    pub account_name: String,
}

/// Cloudflare R2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareConfig {
    pub bucket: String,
    pub account_id: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

/// Backblaze B2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackblazeConfig {
    pub bucket: String,
    pub key_id: String,
    pub key: String,
    pub endpoint: String,
}

/// Self-hosted storage configuration (MinIO, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHostedConfig {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub use_ssl: bool,
}

/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsage {
    /// Total bytes used
    pub bytes_used: u64,
    
    /// Total objects
    pub object_count: u64,
    
    /// Bytes by storage class
    pub by_class: std::collections::HashMap<StorageClass, u64>,
    
    /// Monthly cost estimate (in cents)
    pub estimated_monthly_cost: u64,
}

impl Default for ObjectMetadata {
    fn default() -> Self {
        Self {
            content_encoding: None,
            cache_control: Some("max-age=3600".to_string()),
            content_disposition: None,
            custom: std::collections::HashMap::new(),
        }
    }
}