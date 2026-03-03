//! Official Plugin Marketplace
//!
//! Backend API for plugin submission, review, and distribution.

use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, debug, warn};

/// Marketplace configuration
#[derive(Debug, Clone)]
pub struct MarketplaceConfig {
    /// Enable marketplace
    pub enabled: bool,
    
    /// Marketplace API URL
    pub api_url: String,
    
    /// Maximum plugin size in bytes (default: 50MB)
    pub max_plugin_size: usize,
    
    /// Require review for plugins
    pub require_review: bool,
    
    /// Auto-approve trusted developers
    pub auto_approve_trusted: bool,
}

impl Default for MarketplaceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "https://marketplace.vantis.app".to_string(),
            max_plugin_size: 50 * 1024 * 1024, // 50MB
            require_review: true,
            auto_approve_trusted: false,
        }
    }
}

/// Plugin marketplace
pub struct PluginMarketplace {
    /// Marketplace configuration
    config: MarketplaceConfig,
    
    /// Plugin registry
    plugins: Arc<RwLock<HashMap<String, MarketplacePlugin>>>,
    
    /// Plugin submissions
    submissions: Arc<RwLock<HashMap<String, PluginSubmission>>>,
    
    /// Reviews
    reviews: Arc<RwLock<HashMap<String, Vec<PluginReview>>>>,
    
    /// User ratings
    ratings: Arc<RwLock<HashMap<String, PluginRatingSummary>>>,
    
    /// Download tracking
    downloads: Arc<RwLock<HashMap<String, DownloadStats>>>,
    
    /// Developer accounts
    developers: Arc<RwLock<HashMap<String, Developer>>>,
}

/// Marketplace plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    /// Plugin ID
    pub id: String,
    
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,
    
    /// Author/developer
    pub author: String,
    
    /// Author ID
    pub author_id: String,
    
    /// Plugin categories
    pub categories: Vec<PluginCategory>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Plugin homepage URL
    pub homepage: Option<String>,
    
    /// Repository URL
    pub repository: Option<String>,
    
    /// License
    pub license: String,
    
    /// Download URL
    pub download_url: String,
    
    /// Checksum (SHA256)
    pub checksum: String,
    
    /// Supported platforms
    pub platforms: Vec<Platform>,
    
    /// Minimum Vantis version
    pub min_version: String,
    
    /// Maximum Vantis version
    pub max_version: Option<String>,
    
    /// Plugin status
    pub status: PluginStatus,
    
    /// Publication date
    pub published_at: Option<DateTime<Utc>>,
    
    /// Last updated
    pub updated_at: DateTime<Utc>,
    
    /// Icon URL
    pub icon_url: Option<String>,
    
    /// Screenshots
    pub screenshots: Vec<String>,
    
    /// Changelog
    pub changelog: Vec<ChangelogEntry>,
    
    /// Dependencies
    pub dependencies: Vec<PluginDependency>,
    
    /// Statistics
    pub stats: PluginStats,
    
    /// Verification status
    pub verified: bool,
    
    /// Featured status
    pub featured: bool,
}

/// Plugin category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    VideoProcessing,
    AudioProcessing,
    Subtitles,
    UI,
    Playback,
    Streaming,
    Conversion,
    Metadata,
    Integration,
    Other,
}

/// Platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    All,
}

/// Plugin status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginStatus {
    Draft,
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Deprecated,
    Removed,
}

/// Changelog entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: DateTime<Utc>,
    pub changes: Vec<String>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub min_version: String,
    pub max_version: Option<String>,
    pub optional: bool,
}

/// Plugin statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginStats {
    pub total_downloads: u64,
    pub monthly_downloads: u64,
    pub weekly_downloads: u64,
    pub daily_downloads: u64,
    pub active_installs: u64,
    pub star_rating: f64,
    pub rating_count: u64,
}

/// Plugin submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSubmission {
    /// Submission ID
    pub id: String,
    
    /// Plugin ID
    pub plugin_id: String,
    
    /// Plugin version
    pub version: String,
    
    /// Submitter ID
    pub submitter_id: String,
    
    /// Submission date
    pub submitted_at: DateTime<Utc>,
    
    /// Status
    pub status: SubmissionStatus,
    
    /// Review notes
    pub review_notes: Option<String>,
    
    /// Reviewer ID
    pub reviewer_id: Option<String>,
    
    /// Review date
    pub reviewed_at: Option<DateTime<Utc>>,
    
    /// Plugin manifest
    pub manifest: PluginManifest,
}

/// Submission status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmissionStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Cancelled,
}

/// Plugin manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub main: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub engines: PluginEngines,
    pub dependencies: HashMap<String, String>,
}

/// Plugin engines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEngines {
    pub vantis: String,
}

/// Plugin review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginReview {
    /// Review ID
    pub id: String,
    
    /// Plugin ID
    pub plugin_id: String,
    
    /// Reviewer ID
    pub reviewer_id: String,
    
    /// Review date
    pub reviewed_at: DateTime<Utc>,
    
    /// Overall decision
    pub decision: ReviewDecision,
    
    /// Security assessment
    pub security: SecurityAssessment,
    
    /// Code quality assessment
    pub code_quality: CodeQualityAssessment,
    
    /// Documentation assessment
    pub documentation: DocumentationAssessment,
    
    /// Comments
    pub comments: String,
    
    /// Issues found
    pub issues: Vec<ReviewIssue>,
}

/// Review decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReviewDecision {
    Approve,
    ApproveWithChanges,
    RequestChanges,
    Reject,
}

/// Security assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    pub passed: bool,
    pub score: f64, // 0-100
    pub vulnerabilities: Vec<Vulnerability>,
    pub recommendations: Vec<String>,
}

/// Vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub severity: VulnerabilitySeverity,
    pub title: String,
    pub description: String,
    pub location: String,
    pub recommendation: String,
}

/// Vulnerability severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Code quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityAssessment {
    pub passed: bool,
    pub score: f64,
    pub issues: Vec<CodeIssue>,
    pub recommendations: Vec<String>,
}

/// Code issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub severity: IssueSeverity,
    pub rule: String,
    pub message: String,
    pub location: String,
}

/// Issue severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Documentation assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationAssessment {
    pub passed: bool,
    pub score: f64,
    pub has_readme: bool,
    pub has_changelog: bool,
    pub has_api_docs: bool,
    pub has_examples: bool,
    pub missing_items: Vec<String>,
}

/// Review issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewIssue {
    pub id: String,
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
    pub location: Option<String>,
    pub recommendation: String,
}

/// Issue category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueCategory {
    Security,
    Performance,
    Compatibility,
    Documentation,
    CodeStyle,
    Functionality,
    Other,
}

/// Plugin rating summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRatingSummary {
    pub plugin_id: String,
    pub total_ratings: u64,
    pub average_rating: f64,
    pub rating_distribution: HashMap<u8, u64>, // 1-5 stars
    pub recent_ratings: Vec<UserRating>,
}

/// User rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRating {
    pub id: String,
    pub user_id: String,
    pub plugin_id: String,
    pub rating: u8, // 1-5
    pub title: String,
    pub review: String,
    pub created_at: DateTime<Utc>,
    pub helpful_count: u64,
    pub verified: bool, // Verified purchase/install
}

/// Download stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    pub plugin_id: String,
    pub total: u64,
    pub daily: Vec<DailyDownload>,
    pub by_platform: HashMap<String, u64>,
    pub by_version: HashMap<String, u64>,
}

/// Daily download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyDownload {
    pub date: DateTime<Utc>,
    pub count: u64,
}

/// Developer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Developer {
    /// Developer ID
    pub id: String,
    
    /// Username
    pub username: String,
    
    /// Display name
    pub display_name: String,
    
    /// Email
    pub email: String,
    
    /// Avatar URL
    pub avatar_url: Option<String>,
    
    /// Bio
    pub bio: Option<String>,
    
    /// Website
    pub website: Option<String>,
    
    /// Verified status
    pub verified: bool,
    
    /// Trusted status (auto-approve)
    pub trusted: bool,
    
    /// Total plugins
    pub total_plugins: u64,
    
    /// Total downloads
    pub total_downloads: u64,
    
    /// Member since
    pub created_at: DateTime<Utc>,
}

impl PluginMarketplace {
    /// Create a new plugin marketplace
    pub fn new(config: MarketplaceConfig) -> Self {
        Self {
            config,
            plugins: Arc::new(RwLock::new(HashMap::new())),
            submissions: Arc::new(RwLock::new(HashMap::new())),
            reviews: Arc::new(RwLock::new(HashMap::new())),
            ratings: Arc::new(RwLock::new(HashMap::new())),
            downloads: Arc::new(RwLock::new(HashMap::new())),
            developers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(MarketplaceConfig::default())
    }
    
    // ==================== Plugin Operations ====================
    
    /// Submit a new plugin or plugin update
    pub fn submit_plugin(&self, submission: PluginSubmission) -> Result<()> {
        if !self.config.enabled {
            return Err(anyhow!("Marketplace is disabled"));
        }
        
        let mut submissions = self.submissions.write();
        
        // Check for existing pending submission
        let existing = submissions.values().find(|s| {
            s.plugin_id == submission.plugin_id && 
            s.status == SubmissionStatus::Pending
        });
        
        if existing.is_some() {
            return Err(anyhow!("A pending submission already exists for this plugin"));
        }
        
        submissions.insert(submission.id.clone(), submission.clone());
        
        info!("📦 Plugin submitted: {} v{}", submission.plugin_id, submission.version);
        
        Ok(())
    }
    
    /// Get plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<MarketplacePlugin> {
        self.plugins.read().get(id).cloned()
    }
    
    /// Search plugins
    pub fn search_plugins(&self, query: &str, filters: SearchFilters) -> Vec<MarketplacePlugin> {
        let plugins = self.plugins.read();
        let query_lower = query.to_lowercase();
        
        plugins.values()
            .filter(|p| p.status == PluginStatus::Approved)
            .filter(|p| {
                if query.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&query_lower) ||
                    p.description.to_lowercase().contains(&query_lower) ||
                    p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                }
            })
            .filter(|p| {
                if let Some(category) = &filters.category {
                    p.categories.contains(category)
                } else {
                    true
                }
            })
            .filter(|p| {
                if let Some(platform) = &filters.platform {
                    p.platforms.contains(platform) || p.platforms.contains(&Platform::All)
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
    
    /// Get featured plugins
    pub fn get_featured_plugins(&self) -> Vec<MarketplacePlugin> {
        self.plugins.read().values()
            .filter(|p| p.featured && p.status == PluginStatus::Approved)
            .cloned()
            .collect()
    }
    
    /// Get plugins by category
    pub fn get_plugins_by_category(&self, category: &PluginCategory) -> Vec<MarketplacePlugin> {
        self.plugins.read().values()
            .filter(|p| p.categories.contains(category) && p.status == PluginStatus::Approved)
            .cloned()
            .collect()
    }
    
    /// Get plugins by developer
    pub fn get_plugins_by_developer(&self, developer_id: &str) -> Vec<MarketplacePlugin> {
        self.plugins.read().values()
            .filter(|p| p.author_id == developer_id && p.status == PluginStatus::Approved)
            .cloned()
            .collect()
    }
    
    // ==================== Review Operations ====================
    
    /// Submit a review for a plugin
    pub fn submit_review(&self, review: PluginReview) -> Result<()> {
        let mut reviews = self.reviews.write();
        let plugin_reviews = reviews.entry(review.plugin_id.clone()).or_insert_with(Vec::new);
        plugin_reviews.push(review.clone());
        
        info!("✅ Plugin review submitted for {}", review.plugin_id);
        
        Ok(())
    }
    
    /// Get reviews for a plugin
    pub fn get_reviews(&self, plugin_id: &str) -> Vec<PluginReview> {
        self.reviews.read().get(plugin_id).cloned().unwrap_or_default()
    }
    
    /// Approve a plugin submission
    pub fn approve_submission(&self, submission_id: &str, reviewer_id: &str, notes: Option<String>) -> Result<()> {
        let mut submissions = self.submissions.write();
        
        if let Some(submission) = submissions.get_mut(submission_id) {
            submission.status = SubmissionStatus::Approved;
            submission.reviewer_id = Some(reviewer_id.to_string());
            submission.reviewed_at = Some(Utc::now());
            submission.review_notes = notes;
            
            // Update plugin status
            let mut plugins = self.plugins.write();
            if let Some(plugin) = plugins.get_mut(&submission.plugin_id) {
                plugin.status = PluginStatus::Approved;
                plugin.published_at = Some(Utc::now());
                plugin.updated_at = Utc::now();
            }
            
            info!("✅ Plugin submission approved: {}", submission_id);
        }
        
        Ok(())
    }
    
    /// Reject a plugin submission
    pub fn reject_submission(&self, submission_id: &str, reviewer_id: &str, reason: String) -> Result<()> {
        let mut submissions = self.submissions.write();
        
        if let Some(submission) = submissions.get_mut(submission_id) {
            submission.status = SubmissionStatus::Rejected;
            submission.reviewer_id = Some(reviewer_id.to_string());
            submission.reviewed_at = Some(Utc::now());
            submission.review_notes = Some(reason.clone());
            
            // Update plugin status
            let mut plugins = self.plugins.write();
            if let Some(plugin) = plugins.get_mut(&submission.plugin_id) {
                plugin.status = PluginStatus::Rejected;
                plugin.updated_at = Utc::now();
            }
            
            info!("❌ Plugin submission rejected: {}", submission_id);
        }
        
        Ok(())
    }
    
    // ==================== Rating Operations ====================
    
    /// Submit a user rating
    pub fn submit_rating(&self, rating: UserRating) -> Result<()> {
        if rating.rating < 1 || rating.rating > 5 {
            return Err(anyhow!("Rating must be between 1 and 5"));
        }
        
        let mut ratings = self.ratings.write();
        
        let summary = ratings.entry(rating.plugin_id.clone()).or_insert_with(|| {
            PluginRatingSummary {
                plugin_id: rating.plugin_id.clone(),
                total_ratings: 0,
                average_rating: 0.0,
                rating_distribution: HashMap::new(),
                recent_ratings: Vec::new(),
            }
        });
        
        summary.total_ratings += 1;
        summary.rating_distribution.entry(rating.rating).or_insert(0);
        *summary.rating_distribution.entry(rating.rating).or_insert(0) += 1;
        
        // Recalculate average
        let total_score: u64 = summary.rating_distribution.iter()
            .map(|(stars, count)| (*stars as u64) * count)
            .sum();
        summary.average_rating = total_score as f64 / summary.total_ratings as f64;
        
        // Add to recent ratings
        summary.recent_ratings.push(rating.clone());
        if summary.recent_ratings.len() > 10 {
            summary.recent_ratings.remove(0);
        }
        
        // Update plugin stats
        let mut plugins = self.plugins.write();
        if let Some(plugin) = plugins.get_mut(&rating.plugin_id) {
            plugin.stats.star_rating = summary.average_rating;
            plugin.stats.rating_count = summary.total_ratings;
        }
        
        Ok(())
    }
    
    /// Get rating summary for a plugin
    pub fn get_rating_summary(&self, plugin_id: &str) -> Option<PluginRatingSummary> {
        self.ratings.read().get(plugin_id).cloned()
    }
    
    // ==================== Download Operations ====================
    
    /// Record a plugin download
    pub fn record_download(&self, plugin_id: &str, platform: Platform, version: &str) -> Result<()> {
        let mut downloads = self.downloads.write();
        
        let stats = downloads.entry(plugin_id.to_string()).or_insert_with(|| {
            DownloadStats {
                plugin_id: plugin_id.to_string(),
                total: 0,
                daily: Vec::new(),
                by_platform: HashMap::new(),
                by_version: HashMap::new(),
            }
        });
        
        stats.total += 1;
        *stats.by_platform.entry(format!("{:?}", platform)).or_insert(0) += 1;
        *stats.by_version.entry(version.to_string()).or_insert(0) += 1;
        
        // Update plugin stats
        let mut plugins = self.plugins.write();
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.stats.total_downloads += 1;
            plugin.stats.daily_downloads += 1;
            plugin.stats.weekly_downloads += 1;
            plugin.stats.monthly_downloads += 1;
        }
        
        Ok(())
    }
    
    /// Get download stats for a plugin
    pub fn get_download_stats(&self, plugin_id: &str) -> Option<DownloadStats> {
        self.downloads.read().get(plugin_id).cloned()
    }
    
    // ==================== Developer Operations ====================
    
    /// Register a new developer
    pub fn register_developer(&self, developer: Developer) -> Result<()> {
        let mut developers = self.developers.write();
        
        if developers.contains_key(&developer.id) {
            return Err(anyhow!("Developer already exists"));
        }
        
        developers.insert(developer.id.clone(), developer.clone());
        
        info!("👤 Developer registered: {}", developer.username);
        
        Ok(())
    }
    
    /// Get developer by ID
    pub fn get_developer(&self, id: &str) -> Option<Developer> {
        self.developers.read().get(id).cloned()
    }
    
    // ==================== Distribution Operations ====================
    
    /// Get plugin download URL
    pub fn get_download_url(&self, plugin_id: &str, version: Option<&str>) -> Result<String> {
        let plugins = self.plugins.read();
        
        if let Some(plugin) = plugins.get(plugin_id) {
            if plugin.status != PluginStatus::Approved {
                return Err(anyhow!("Plugin is not approved for distribution"));
            }
            
            // For now, return the main download URL
            // In production, this would handle version-specific URLs
            Ok(plugin.download_url.clone())
        } else {
            Err(anyhow!("Plugin not found"))
        }
    }
    
    /// Verify plugin checksum
    pub fn verify_checksum(&self, plugin_id: &str, expected_checksum: &str) -> Result<bool> {
        let plugins = self.plugins.read();
        
        if let Some(plugin) = plugins.get(plugin_id) {
            Ok(plugin.checksum == expected_checksum)
        } else {
            Err(anyhow!("Plugin not found"))
        }
    }
}

/// Search filters
#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub category: Option<PluginCategory>,
    pub platform: Option<Platform>,
    pub min_rating: Option<f64>,
    pub max_price: Option<f64>,
    pub verified_only: bool,
    pub featured_only: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_marketplace_creation() {
        let marketplace = PluginMarketplace::default_config();
        assert!(marketplace.config.enabled);
    }
    
    #[test]
    fn test_register_developer() {
        let marketplace = PluginMarketplace::default_config();
        let developer = Developer {
            id: "dev1".to_string(),
            username: "testdev".to_string(),
            display_name: "Test Developer".to_string(),
            email: "test@example.com".to_string(),
            avatar_url: None,
            bio: None,
            website: None,
            verified: false,
            trusted: false,
            total_plugins: 0,
            total_downloads: 0,
            created_at: Utc::now(),
        };
        
        assert!(marketplace.register_developer(developer).is_ok());
    }
    
    #[test]
    fn test_submit_rating() {
        let marketplace = PluginMarketplace::default_config();
        
        // Add a plugin first
        let plugin = MarketplacePlugin {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            status: PluginStatus::Approved,
            author: "Test".to_string(),
            author_id: "dev1".to_string(),
            description: "Test".to_string(),
            categories: vec![PluginCategory::Other],
            tags: vec![],
            homepage: None,
            repository: None,
            license: "MIT".to_string(),
            download_url: "https://example.com/plugin.wasm".to_string(),
            checksum: "abc123".to_string(),
            platforms: vec![Platform::All],
            min_version: "1.0.0".to_string(),
            max_version: None,
            published_at: None,
            updated_at: Utc::now(),
            icon_url: None,
            screenshots: vec![],
            changelog: vec![],
            dependencies: vec![],
            stats: PluginStats::default(),
            verified: false,
            featured: false,
        };
        
        marketplace.plugins.write().insert("test-plugin".to_string(), plugin);
        
        let rating = UserRating {
            id: "r1".to_string(),
            user_id: "u1".to_string(),
            plugin_id: "test-plugin".to_string(),
            rating: 5,
            title: "Great plugin!".to_string(),
            review: "Excellent work!".to_string(),
            created_at: Utc::now(),
            helpful_count: 0,
            verified: true,
        };
        
        assert!(marketplace.submit_rating(rating).is_ok());
        
        let summary = marketplace.get_rating_summary("test-plugin");
        assert!(summary.is_some());
        assert_eq!(summary.unwrap().average_rating, 5.0);
    }
    
    #[test]
    fn test_record_download() {
        let marketplace = PluginMarketplace::default_config();
        
        let plugin = MarketplacePlugin {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            status: PluginStatus::Approved,
            author: "Test".to_string(),
            author_id: "dev1".to_string(),
            description: "Test".to_string(),
            categories: vec![PluginCategory::Other],
            tags: vec![],
            homepage: None,
            repository: None,
            license: "MIT".to_string(),
            download_url: "https://example.com/plugin.wasm".to_string(),
            checksum: "abc123".to_string(),
            platforms: vec![Platform::All],
            min_version: "1.0.0".to_string(),
            max_version: None,
            published_at: None,
            updated_at: Utc::now(),
            icon_url: None,
            screenshots: vec![],
            changelog: vec![],
            dependencies: vec![],
            stats: PluginStats::default(),
            verified: false,
            featured: false,
        };
        
        marketplace.plugins.write().insert("test-plugin".to_string(), plugin);
        
        assert!(marketplace.record_download("test-plugin", Platform::Linux, "1.0.0").is_ok());
        
        let stats = marketplace.get_download_stats("test-plugin");
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().total, 1);
    }
}