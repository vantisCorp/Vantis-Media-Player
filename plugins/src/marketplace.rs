//! Plugin Marketplace System
//!
//! Provides access to the Vantis Plugin Marketplace for discovering,
//! downloading, and installing plugins from a central registry.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn, error};

// ============================================================================
// Marketplace Types
// ============================================================================

/// Plugin marketplace client
pub struct PluginMarketplace {
    /// API endpoint for the marketplace
    api_endpoint: String,
    
    /// HTTP client
    client: reqwest::blocking::Client,
    
    /// Local plugin cache
    cache: Arc<RwLock<MarketplaceCache>>,
    
    /// Download directory
    download_dir: PathBuf,
    
    /// Installation directory
    install_dir: PathBuf,
}

/// Marketplace cache
#[derive(Debug, Default)]
pub struct MarketplaceCache {
    /// Cached plugin listings
    plugins: HashMap<String, CachedPlugin>,
    
    /// Last refresh time
    last_refresh: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Cache TTL in seconds
    cache_ttl: u64,
}

/// Cached plugin entry
#[derive(Debug, Clone)]
pub struct CachedPlugin {
    /// Plugin metadata
    pub metadata: PluginListing,
    
    /// Cache timestamp
    pub cached_at: chrono::DateTime<chrono::Utc>,
}

/// Plugin listing from marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginListing {
    /// Unique plugin ID
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Short description
    pub description: String,
    
    /// Author information
    pub author: AuthorInfo,
    
    /// Latest version
    pub latest_version: String,
    
    /// All available versions
    pub versions: Vec<PluginVersion>,
    
    /// Download count
    pub downloads: u64,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Number of ratings
    pub rating_count: u32,
    
    /// Plugin categories
    pub categories: Vec<String>,
    
    /// Plugin tags
    pub tags: Vec<String>,
    
    /// Supported platforms
    pub platforms: Vec<Platform>,
    
    /// Homepage URL
    pub homepage: Option<String>,
    
    /// Repository URL
    pub repository: Option<String>,
    
    /// License
    pub license: String,
    
    /// Last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    /// Whether the plugin is verified
    pub verified: bool,
    
    /// Signature status
    pub signature_status: SignatureStatus,
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    /// Author ID
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Author avatar URL
    pub avatar_url: Option<String>,
    
    /// Whether the author is verified
    pub verified: bool,
}

/// Plugin version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVersion {
    /// Version string
    pub version: String,
    
    /// Release date
    pub released_at: chrono::DateTime<chrono::Utc>,
    
    /// Changelog
    pub changelog: Option<String>,
    
    /// Download URL
    pub download_url: String,
    
    /// File size in bytes
    pub size: u64,
    
    /// SHA256 hash
    pub sha256: String,
    
    /// Signature URL
    pub signature_url: Option<String>,
    
    /// Minimum API version required
    pub min_api_version: u32,
    
    /// Maximum API version supported
    pub max_api_version: Option<u32>,
    
    /// Dependencies
    pub dependencies: Vec<DependencyInfo>,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// Dependency plugin ID
    pub plugin_id: String,
    
    /// Version constraint (semver)
    pub version_constraint: String,
    
    /// Whether the dependency is optional
    pub optional: bool,
}

/// Supported platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Android,
    IOS,
    Web,
}

/// Signature verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureStatus {
    /// Signature is valid
    Valid,
    /// Signature is invalid
    Invalid,
    /// Plugin is not signed
    Unsigned,
    /// Signature verification pending
    Pending,
}

/// Search query for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search term
    pub query: String,
    
    /// Filter by category
    pub category: Option<String>,
    
    /// Filter by tag
    pub tags: Vec<String>,
    
    /// Filter by platform
    pub platform: Option<Platform>,
    
    /// Sort by field
    pub sort_by: SortBy,
    
    /// Sort order
    pub sort_order: SortOrder,
    
    /// Page number (1-indexed)
    pub page: u32,
    
    /// Results per page
    pub per_page: u32,
}

/// Sort field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    Downloads,
    Rating,
    Updated,
    Name,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Matching plugins
    pub plugins: Vec<PluginListing>,
    
    /// Total results
    pub total: u64,
    
    /// Current page
    pub page: u32,
    
    /// Total pages
    pub total_pages: u32,
}

/// Plugin review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginReview {
    /// Review ID
    pub id: String,
    
    /// Plugin ID
    pub plugin_id: String,
    
    /// Reviewer name
    pub reviewer: String,
    
    /// Rating (1-5)
    pub rating: u8,
    
    /// Review title
    pub title: String,
    
    /// Review content
    pub content: String,
    
    /// Review date
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Whether the reviewer verified ownership
    pub verified: bool,
    
    /// Helpful count
    pub helpful_count: u32,
}

/// Installation progress callback
pub type ProgressCallback = Box<dyn Fn(InstallProgress) + Send + Sync>;

/// Installation progress
#[derive(Debug, Clone)]
pub struct InstallProgress {
    /// Current phase
    pub phase: InstallPhase,
    
    /// Progress percentage (0-100)
    pub progress: u8,
    
    /// Current action description
    pub message: String,
}

/// Installation phase
#[derive(Debug, Clone, PartialEq)]
pub enum InstallPhase {
    Downloading,
    Verifying,
    Extracting,
    Installing,
    Completing,
    Done,
    Failed,
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Plugin not found: {0}")]
    NotFound(String),
    
    #[error("Signature verification failed")]
    SignatureFailed,
    
    #[error("Dependency resolution failed: {0}")]
    DependencyFailed(String),
    
    #[error("Installation failed: {0}")]
    InstallationFailed(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}

// ============================================================================
// Implementation
// ============================================================================

impl PluginMarketplace {
    /// Create a new marketplace client
    pub fn new(api_endpoint: &str, download_dir: &Path, install_dir: &Path) -> Self {
        Self {
            api_endpoint: api_endpoint.to_string(),
            client: reqwest::blocking::Client::new(),
            cache: Arc::new(RwLock::new(MarketplaceCache::default())),
            download_dir: download_dir.to_path_buf(),
            install_dir: install_dir.to_path_buf(),
        }
    }
    
    /// Create with default marketplace endpoint
    pub fn with_default_endpoint(download_dir: &Path, install_dir: &Path) -> Self {
        Self::new("https://marketplace.vantis.io/api/v1", download_dir, install_dir)
    }
    
    /// Search for plugins
    pub fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        info!("Searching marketplace: {}", query.query);
        
        let url = format!("{}/plugins/search", self.api_endpoint);
        
        let response = self.client
            .get(&url)
            .query(&[
                ("q", query.query.as_str()),
                ("page", &query.page.to_string()),
                ("per_page", &query.per_page.to_string()),
            ])
            .send()
            .context("Failed to send search request")?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::ApiError(format!(
                "Search failed with status: {}",
                response.status()
            )).into());
        }
        
        let result: SearchResult = response
            .json()
            .context("Failed to parse search results")?;
        
        info!("Found {} plugins", result.plugins.len());
        Ok(result)
    }
    
    /// Get plugin details
    pub fn get_plugin(&self, plugin_id: &str) -> Result<PluginListing> {
        debug!("Fetching plugin: {}", plugin_id);
        
        // Check cache first
        {
            let cache = self.cache.read();
            if let Some(cached) = cache.plugins.get(plugin_id) {
                if cached.is_fresh(cache.cache_ttl) {
                    return Ok(cached.metadata.clone());
                }
            }
        }
        
        // Fetch from API
        let url = format!("{}/plugins/{}", self.api_endpoint, plugin_id);
        
        let response = self.client
            .get(&url)
            .send()
            .context("Failed to fetch plugin")?;
        
        if response.status() == 404 {
            return Err(MarketplaceError::NotFound(plugin_id.to_string()).into());
        }
        
        if !response.status().is_success() {
            return Err(MarketplaceError::ApiError(format!(
                "Failed to fetch plugin: {}",
                response.status()
            )).into());
        }
        
        let plugin: PluginListing = response
            .json()
            .context("Failed to parse plugin data")?;
        
        // Cache the result
        {
            let mut cache = self.cache.write();
            cache.plugins.insert(plugin_id.to_string(), CachedPlugin {
                metadata: plugin.clone(),
                cached_at: chrono::Utc::now(),
            });
        }
        
        Ok(plugin)
    }
    
    /// Get plugin reviews
    pub fn get_reviews(&self, plugin_id: &str, page: u32) -> Result<Vec<PluginReview>> {
        debug!("Fetching reviews for: {}", plugin_id);
        
        let url = format!("{}/plugins/{}/reviews", self.api_endpoint, plugin_id);
        
        let response = self.client
            .get(&url)
            .query(&[("page", &page.to_string())])
            .send()
            .context("Failed to fetch reviews")?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::ApiError(format!(
                "Failed to fetch reviews: {}",
                response.status()
            )).into());
        }
        
        let reviews: Vec<PluginReview> = response
            .json()
            .context("Failed to parse reviews")?;
        
        Ok(reviews)
    }
    
    /// Install a plugin
    pub fn install_plugin(
        &self,
        plugin_id: &str,
        version: Option<&str>,
        progress: Option<ProgressCallback>,
    ) -> Result<()> {
        info!("Installing plugin: {} (version: {:?})", plugin_id, version);
        
        // Get plugin info
        let plugin = self.get_plugin(plugin_id)?;
        
        // Select version
        let target_version = if let Some(v) = version {
            plugin.versions.iter()
                .find(|pv| pv.version == v)
                .ok_or_else(|| MarketplaceError::NotFound(format!("Version {} not found", v)))?
                .clone()
        } else {
            plugin.versions.first()
                .ok_or_else(|| MarketplaceError::NotFound("No versions available".to_string()))?
                .clone()
        };
        
        // Report progress
        if let Some(ref cb) = progress {
            cb(InstallProgress {
                phase: InstallPhase::Downloading,
                progress: 0,
                message: format!("Downloading {} v{}...", plugin.name, target_version.version),
            });
        }
        
        // Download plugin
        let download_path = self.download_plugin(&target_version)?;
        
        // Verify signature
        if let Some(ref cb) = progress {
            cb(InstallProgress {
                phase: InstallPhase::Verifying,
                progress: 50,
                message: "Verifying signature...".to_string(),
            });
        }
        
        self.verify_signature(&download_path, &target_version)?;
        
        // Install
        if let Some(ref cb) = progress {
            cb(InstallProgress {
                phase: InstallPhase::Installing,
                progress: 75,
                message: "Installing...".to_string(),
            });
        }
        
        self.install_downloaded_plugin(&download_path, &plugin)?;
        
        // Complete
        if let Some(ref cb) = progress {
            cb(InstallProgress {
                phase: InstallPhase::Done,
                progress: 100,
                message: format!("{} installed successfully!", plugin.name),
            });
        }
        
        info!("Plugin {} v{} installed successfully", plugin_id, target_version.version);
        Ok(())
    }
    
    /// Download plugin file
    fn download_plugin(&self, version: &PluginVersion) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.download_dir)
            .context("Failed to create download directory")?;
        
        let filename = format!("{}-{}.wasm", 
            version.download_url.split('/').last().unwrap_or("plugin"),
            version.version
        );
        let download_path = self.download_dir.join(&filename);
        
        let mut response = self.client
            .get(&version.download_url)
            .send()
            .context("Failed to download plugin")?;
        
        let mut file = std::fs::File::create(&download_path)
            .context("Failed to create download file")?;
        
        std::io::copy(&mut response, &mut file)
            .context("Failed to write download file")?;
        
        // Verify hash
        let file_bytes = std::fs::read(&download_path)?;
        let hash = format!("{:x}", sha2::Sha256::digest(&file_bytes));
        
        if hash != version.sha256 {
            std::fs::remove_file(&download_path)?;
            return Err(MarketplaceError::InstallationFailed(
                "Downloaded file hash mismatch".to_string()
            ).into());
        }
        
        Ok(download_path)
    }
    
    /// Verify plugin signature
    fn verify_signature(&self, _path: &Path, _version: &PluginVersion) -> Result<()> {
        // Signature verification is handled by the signing module
        // This is a placeholder for integration
        Ok(())
    }
    
    /// Install downloaded plugin
    fn install_downloaded_plugin(&self, download_path: &Path, plugin: &PluginListing) -> Result<()> {
        std::fs::create_dir_all(&self.install_dir)
            .context("Failed to create install directory")?;
        
        let plugin_dir = self.install_dir.join(&plugin.id);
        std::fs::create_dir_all(&plugin_dir)
            .context("Failed to create plugin directory")?;
        
        // Copy WASM file
        let wasm_dest = plugin_dir.join("plugin.wasm");
        std::fs::copy(download_path, &wasm_dest)
            .context("Failed to copy plugin file")?;
        
        // Create manifest
        let manifest = serde_json::json!({
            "id": plugin.id,
            "name": plugin.name,
            "version": plugin.latest_version,
            "description": plugin.description,
            "author": plugin.author.name,
            "license": plugin.license,
        });
        
        let manifest_path = plugin_dir.join("manifest.json");
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)
            .context("Failed to write manifest")?;
        
        Ok(())
    }
    
    /// Uninstall a plugin
    pub fn uninstall_plugin(&self, plugin_id: &str) -> Result<()> {
        info!("Uninstalling plugin: {}", plugin_id);
        
        let plugin_dir = self.install_dir.join(plugin_id);
        
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)
                .context("Failed to remove plugin directory")?;
        }
        
        info!("Plugin {} uninstalled", plugin_id);
        Ok(())
    }
    
    /// Refresh the plugin cache
    pub fn refresh_cache(&self) -> Result<()> {
        info!("Refreshing marketplace cache");
        
        let mut cache = self.cache.write();
        cache.plugins.clear();
        cache.last_refresh = Some(chrono::Utc::now());
        
        Ok(())
    }
    
    /// Get featured plugins
    pub fn get_featured(&self) -> Result<Vec<PluginListing>> {
        debug!("Fetching featured plugins");
        
        let url = format!("{}/plugins/featured", self.api_endpoint);
        
        let response = self.client
            .get(&url)
            .send()
            .context("Failed to fetch featured plugins")?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::ApiError(format!(
                "Failed to fetch featured: {}",
                response.status()
            )).into());
        }
        
        let plugins: Vec<PluginListing> = response
            .json()
            .context("Failed to parse featured plugins")?;
        
        Ok(plugins)
    }
    
    /// Get plugins by category
    pub fn get_by_category(&self, category: &str, page: u32) -> Result<Vec<PluginListing>> {
        debug!("Fetching plugins in category: {}", category);
        
        let url = format!("{}/plugins/category/{}", self.api_endpoint, category);
        
        let response = self.client
            .get(&url)
            .query(&[("page", &page.to_string())])
            .send()
            .context("Failed to fetch plugins by category")?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::ApiError(format!(
                "Failed to fetch category: {}",
                response.status()
            )).into());
        }
        
        let plugins: Vec<PluginListing> = response
            .json()
            .context("Failed to parse plugins")?;
        
        Ok(plugins)
    }
}

impl CachedPlugin {
    /// Check if the cached entry is still fresh
    fn is_fresh(&self, ttl_seconds: u64) -> bool {
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(self.cached_at);
        elapsed.num_seconds() < ttl_seconds as i64
    }
}

impl Default for MarketplaceCache {
    fn default() -> Self {
        Self {
            plugins: HashMap::new(),
            last_refresh: None,
            cache_ttl: 3600, // 1 hour
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_query_creation() {
        let query = SearchQuery {
            query: "video filter".to_string(),
            category: None,
            tags: vec![],
            platform: Some(Platform::Linux),
            sort_by: SortBy::Downloads,
            sort_order: SortOrder::Desc,
            page: 1,
            per_page: 20,
        };
        
        assert_eq!(query.query, "video filter");
        assert_eq!(query.page, 1);
    }
    
    #[test]
    fn test_cached_plugin_freshness() {
        let cached = CachedPlugin {
            metadata: PluginListing {
                id: "test-plugin".to_string(),
                name: "Test Plugin".to_string(),
                description: "A test plugin".to_string(),
                author: AuthorInfo {
                    id: "author-1".to_string(),
                    name: "Test Author".to_string(),
                    avatar_url: None,
                    verified: false,
                },
                latest_version: "1.0.0".to_string(),
                versions: vec![],
                downloads: 100,
                rating: 4.5,
                rating_count: 10,
                categories: vec![],
                tags: vec![],
                platforms: vec![],
                homepage: None,
                repository: None,
                license: "MIT".to_string(),
                updated_at: chrono::Utc::now(),
                verified: false,
                signature_status: SignatureStatus::Unsigned,
            },
            cached_at: chrono::Utc::now(),
        };
        
        assert!(cached.is_fresh(3600));
    }
}