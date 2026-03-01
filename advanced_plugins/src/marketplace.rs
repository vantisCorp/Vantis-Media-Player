//! Plugin Marketplace
//! 
//! Provides plugin discovery, search, installation, and update capabilities
//! from a centralized plugin marketplace.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

/// Plugin marketplace
pub struct PluginMarketplace {
    /// Marketplace URL
    url: String,
    
    /// HTTP client
    client: Client,
    
    /// Plugin cache
    cache: Arc<RwLock<HashMap<String, PluginInfo>>>,
    
    /// Enabled flag
    enabled: bool,
}

/// Plugin information from marketplace
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin ID
    pub id: String,
    
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin author
    pub author: String,
    
    /// Plugin description
    pub description: String,
    
    /// Plugin category
    pub category: PluginCategory,
    
    /// Download URL
    pub download_url: String,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// SHA256 checksum
    pub checksum: String,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Permissions required
    pub permissions: Vec<PluginPermission>,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Download count
    pub downloads: u64,
    
    /// Last updated
    pub last_updated: String,
    
    /// License
    pub license: String,
    
    /// Homepage URL
    pub homepage: String,
    
    /// Repository URL
    pub repository: String,
    
    /// Screenshots
    pub screenshots: Vec<String>,
    
    /// Tags
    pub tags: Vec<String>,
}

/// Plugin category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PluginCategory {
    /// Audio processing
    Audio,
    
    /// Video processing
    Video,
    
    /// Subtitle processing
    Subtitles,
    
    /// User interface
    UI,
    
    /// Integration
    Integration,
    
    /// Utility
    Utility,
    
    /// Theme
    Theme,
    
    /// Other
    Other,
}

/// Plugin permission
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PluginPermission {
    /// File system access
    FileSystem,
    
    /// Network access
    Network,
    
    /// Media control
    MediaControl,
    
    /// Configuration access
    ConfigAccess,
    
    /// Logging
    Logging,
    
    /// Custom (with description)
    Custom(String),
}

/// Search query
#[derive(Clone, Debug)]
pub struct SearchQuery {
    /// Search term
    pub query: String,
    
    /// Category filter
    pub category: Option<PluginCategory>,
    
    /// Minimum rating
    pub min_rating: Option<f32>,
    
    /// Sort by
    pub sort_by: SortBy,
    
    /// Sort order
    pub sort_order: SortOrder,
    
    /// Limit results
    pub limit: Option<usize>,
}

/// Sort by field
#[derive(Clone, Debug, Copy)]
pub enum SortBy {
    /// Sort by name
    Name,
    
    /// Sort by rating
    Rating,
    
    /// Sort by downloads
    Downloads,
    
    /// Sort by last updated
    LastUpdated,
}

/// Sort order
#[derive(Clone, Debug, Copy)]
pub enum SortOrder {
    /// Ascending
    Asc,
    
    /// Descending
    Desc,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            category: None,
            min_rating: None,
            sort_by: SortBy::Rating,
            sort_order: SortOrder::Desc,
            limit: Some(20),
        }
    }
}

impl PluginMarketplace {
    /// Create a new plugin marketplace
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        info!("🏪 Plugin marketplace initialized: {}", url);
        
        Ok(Self {
            url: url.to_string(),
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
        })
    }
    
    /// Create a disabled marketplace
    pub fn disabled() -> Self {
        Self {
            url: String::new(),
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            enabled: false,
        }
    }
    
    /// Check if marketplace is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Search for plugins
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<PluginInfo>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("🔍 Searching plugins: {}", query.query);
        
        let url = format!("{}/api/plugins/search", self.url);
        
        let response = self.client
            .get(&url)
            .query(&[
                ("q", &query.query),
                ("category", &query.category.as_ref().map(|c| format!("{:?}", c)).unwrap_or_default()),
                ("min_rating", &query.min_rating.map(|r| r.to_string()).unwrap_or_default()),
                ("sort_by", &format!("{:?}", query.sort_by)),
                ("sort_order", &format!("{:?}", query.sort_order)),
                ("limit", &query.limit.map(|l| l.to_string()).unwrap_or_default()),
            ])
            .send()
            .await
            .context("Failed to search plugins")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Search failed: {}", response.status()));
        }
        
        let plugins: Vec<PluginInfo> = response.json().await?;
        
        // Update cache
        let mut cache = self.cache.write().await;
        for plugin in &plugins {
            cache.insert(plugin.id.clone(), plugin.clone());
        }
        
        debug!("✅ Found {} plugins", plugins.len());
        
        Ok(plugins)
    }
    
    /// Get plugin information
    pub async fn get_plugin(&self, id: &str) -> Result<PluginInfo> {
        if !self.enabled {
            return Err(anyhow::anyhow!("Marketplace is disabled"));
        }
        
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(plugin) = cache.get(id) {
                return Ok(plugin.clone());
            }
        }
        
        debug!("📦 Getting plugin info: {}", id);
        
        let url = format!("{}/api/plugins/{}", self.url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get plugin info")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Get plugin failed: {}", response.status()));
        }
        
        let plugin: PluginInfo = response.json().await?;
        
        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(plugin.id.clone(), plugin.clone());
        
        debug!("✅ Got plugin info: {}", plugin.name);
        
        Ok(plugin)
    }
    
    /// Download plugin
    pub async fn download_plugin(&self, id: &str, dest: &std::path::Path) -> Result<String> {
        if !self.enabled {
            return Err(anyhow::anyhow!("Marketplace is disabled"));
        }
        
        let plugin = self.get_plugin(id).await?;
        
        info!("⬇️  Downloading plugin: {} v{}", plugin.name, plugin.version);
        
        let response = self.client
            .get(&plugin.download_url)
            .send()
            .await
            .context("Failed to download plugin")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Download failed: {}", response.status()));
        }
        
        let bytes = response.bytes().await?;
        
        // Verify checksum
        let checksum = sha2::Sha256::digest(&bytes);
        let checksum_hex = hex::encode(checksum);
        
        if checksum_hex != plugin.checksum {
            return Err(anyhow::anyhow!("Checksum mismatch: expected {}, got {}", 
                plugin.checksum, checksum_hex));
        }
        
        // Write to file
        tokio::fs::write(dest, bytes).await?;
        
        info!("✅ Plugin downloaded: {}", dest.display());
        
        Ok(dest.to_string_lossy().to_string())
    }
    
    /// Get featured plugins
    pub async fn get_featured(&self) -> Result<Vec<PluginInfo>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("⭐ Getting featured plugins");
        
        let url = format!("{}/api/plugins/featured", self.url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get featured plugins")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Get featured failed: {}", response.status()));
        }
        
        let plugins: Vec<PluginInfo> = response.json().await?;
        
        debug!("✅ Got {} featured plugins", plugins.len());
        
        Ok(plugins)
    }
    
    /// Get popular plugins
    pub async fn get_popular(&self, limit: usize) -> Result<Vec<PluginInfo>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("🔥 Getting popular plugins");
        
        let url = format!("{}/api/plugins/popular?limit={}", self.url, limit);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get popular plugins")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Get popular failed: {}", response.status()));
        }
        
        let plugins: Vec<PluginInfo> = response.json().await?;
        
        debug!("✅ Got {} popular plugins", plugins.len());
        
        Ok(plugins)
    }
    
    /// Get plugins by category
    pub async fn get_by_category(&self, category: PluginCategory) -> Result<Vec<PluginInfo>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("📂 Getting plugins by category: {:?}", category);
        
        let url = format!("{}/api/plugins/category/{}", self.url, 
            format!("{:?}", category).to_lowercase());
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get plugins by category")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Get by category failed: {}", response.status()));
        }
        
        let plugins: Vec<PluginInfo> = response.json().await?;
        
        debug!("✅ Got {} plugins in category {:?}", plugins.len(), category);
        
        Ok(plugins)
    }
    
    /// Check for plugin updates
    pub async fn check_updates(&self) -> Result<Vec<PluginUpdate>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("🔄 Checking for plugin updates");
        
        let url = format!("{}/api/plugins/updates", self.url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to check updates")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Check updates failed: {}", response.status()));
        }
        
        let updates: Vec<PluginUpdate> = response.json().await?;
        
        debug!("✅ Found {} plugin updates", updates.len());
        
        Ok(updates)
    }
    
    /// Get plugin reviews
    pub async fn get_reviews(&self, id: &str) -> Result<Vec<PluginReview>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        debug!("💬 Getting reviews for plugin: {}", id);
        
        let url = format!("{}/api/plugins/{}/reviews", self.url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get reviews")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Get reviews failed: {}", response.status()));
        }
        
        let reviews: Vec<PluginReview> = response.json().await?;
        
        debug!("✅ Got {} reviews", reviews.len());
        
        Ok(reviews)
    }
    
    /// Submit plugin review
    pub async fn submit_review(&self, id: &str, review: PluginReview) -> Result<()> {
        if !self.enabled {
            return Err(anyhow::anyhow!("Marketplace is disabled"));
        }
        
        debug!("📝 Submitting review for plugin: {}", id);
        
        let url = format!("{}/api/plugins/{}/reviews", self.url, id);
        
        let response = self.client
            .post(&url)
            .json(&review)
            .send()
            .await
            .context("Failed to submit review")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Submit review failed: {}", response.status()));
        }
        
        debug!("✅ Review submitted");
        
        Ok(())
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        debug!("🗑️  Cache cleared");
    }
}

/// Plugin update information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginUpdate {
    /// Plugin ID
    pub id: String,
    
    /// Current version
    pub current_version: String,
    
    /// New version
    pub new_version: String,
    
    /// Update type
    pub update_type: UpdateType,
    
    /// Release notes
    pub release_notes: String,
    
    /// Update size in bytes
    pub update_size: u64,
}

/// Update type
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateType {
    /// Patch update (bug fixes)
    Patch,
    
    /// Minor update (new features)
    Minor,
    
    /// Major update (breaking changes)
    Major,
}

/// Plugin review
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginReview {
    /// Reviewer name
    pub reviewer: String,
    
    /// Rating (1-5)
    pub rating: u8,
    
    /// Review text
    pub text: String,
    
    /// Review date
    pub date: String,
    
    /// Helpful count
    pub helpful: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_marketplace_creation() {
        let marketplace = PluginMarketplace::new("https://plugins.vantis.io");
        assert!(marketplace.is_ok());
        assert!(marketplace.unwrap().is_enabled());
    }
    
    #[test]
    fn test_disabled_marketplace() {
        let marketplace = PluginMarketplace::disabled();
        assert!(!marketplace.is_enabled());
    }
    
    #[test]
    fn test_search_query_default() {
        let query = SearchQuery::default();
        assert_eq!(query.query, "");
        assert!(query.category.is_none());
        assert_eq!(query.sort_by, SortBy::Rating);
        assert_eq!(query.sort_order, SortOrder::Desc);
    }
    
    #[test]
    fn test_plugin_category() {
        assert_eq!(PluginCategory::Audio, PluginCategory::Audio);
        assert_eq!(PluginCategory::Video, PluginCategory::Video);
    }
    
    #[test]
    fn test_plugin_permission() {
        assert_eq!(PluginPermission::FileSystem, PluginPermission::FileSystem);
        assert_eq!(PluginPermission::Custom("test".to_string()), PluginPermission::Custom("test".to_string()));
    }
}