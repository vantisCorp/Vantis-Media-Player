//! Enhanced Plugin Marketplace UI
//!
//! Enhanced user interface for discovering, evaluating, and installing plugins
//! with advanced search, filtering, screenshots, videos, ratings, and reviews.

use iced::{
    widget::{
        button, column, container, horizontal_rule, row, scrollable, slider, text,
        text_input, Space, Image,
    },
    Alignment, Element, Length, Padding, Renderer, Theme,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::theme::Theme;

/// Enhanced plugin marketplace state
pub struct EnhancedMarketplaceState {
    /// Search query
    search_query: String,
    
    /// Advanced filters
    filters: AdvancedFilters,
    
    /// Selected category
    selected_category: Option<String>,
    
    /// Selected sort option
    sort_option: SortOption,
    
    /// Plugin list
    plugins: Vec<EnhancedPluginDisplayInfo>,
    
    /// Filtered plugin list
    filtered_plugins: Vec<EnhancedPluginDisplayInfo>,
    
    /// Selected plugin
    selected_plugin: Option<EnhancedPluginDisplayInfo>,
    
    /// Loading state
    loading: bool,
    
    /// Error message
    error: Option<String>,
    
    /// Installation progress
    installation_progress: HashMap<String, InstallationProgress>,
    
    /// Plugin reviews
    reviews: HashMap<String, Vec<PluginReview>>,
    
    /// Featured plugins
    featured_plugins: Vec<EnhancedPluginDisplayInfo>,
    
    /// Popular plugins
    popular_plugins: Vec<EnhancedPluginDisplayInfo>,
    
    /// New plugins
    new_plugins: Vec<EnhancedPluginDisplayInfo>,
}

/// Advanced filters for plugin search
#[derive(Clone, Debug, Default)]
pub struct AdvancedFilters {
    /// Minimum rating
    min_rating: f32,
    
    /// Maximum file size in MB
    max_file_size: Option<u64>,
    
    /// Free only
    free_only: bool,
    
    /// Installed only
    installed_only: bool,
    
    /// Update available only
    update_available_only: bool,
    
    /// Selected tags
    selected_tags: Vec<String>,
    
    /// Author filter
    author: Option<String>,
}

/// Enhanced plugin display information
#[derive(Clone, Debug)]
pub struct EnhancedPluginDisplayInfo {
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
    
    /// Long description
    pub long_description: String,
    
    /// Plugin category
    pub category: String,
    
    /// Subcategories
    pub subcategories: Vec<String>,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Rating count
    pub rating_count: u32,
    
    /// Download count
    pub downloads: u64,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// Price (0 for free)
    pub price: f32,
    
    /// Installed flag
    pub installed: bool,
    
    /// Update available flag
    pub update_available: bool,
    
    /// Enabled flag
    pub enabled: bool,
    
    /// Screenshots
    pub screenshots: Vec<MediaAsset>,
    
    /// Videos
    pub videos: Vec<MediaAsset>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// License
    pub license: String,
    
    /// Last updated
    pub last_updated: String,
    
    /// Featured flag
    pub featured: bool,
    
    /// Verified flag
    pub verified: bool,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Compatibility
    pub compatibility: Vec<String>,
}

/// Media asset (screenshot or video)
#[derive(Clone, Debug)]
pub struct MediaAsset {
    /// Asset URL
    pub url: String,
    
    /// Asset type
    pub asset_type: MediaType,
    
    /// Thumbnail URL
    pub thumbnail_url: Option<String>,
    
    /// Caption
    pub caption: Option<String>,
}

/// Media type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MediaType {
    /// Image
    Image,
    
    /// Video
    Video,
}

/// Plugin review
#[derive(Clone, Debug)]
pub struct PluginReview {
    /// Review ID
    pub id: String,
    
    /// Reviewer name
    pub reviewer_name: String,
    
    /// Reviewer avatar
    pub reviewer_avatar: Option<String>,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Review title
    pub title: String,
    
    /// Review content
    pub content: String,
    
    /// Review date
    pub date: String,
    
    /// Helpful count
    pub helpful_count: u32,
    
    /// User marked as helpful
    pub marked_helpful: bool,
}

/// Installation progress
#[derive(Clone, Debug)]
pub struct InstallationProgress {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Progress (0-100)
    pub progress: f32,
    
    /// Status
    pub status: InstallationStatus,
    
    /// Current step
    pub current_step: String,
    
    /// Error message
    pub error: Option<String>,
}

/// Installation status
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstallationStatus {
    /// Downloading
    Downloading,
    
    /// Installing
    Installing,
    
    /// Verifying
    Verifying,
    
    /// Complete
    Complete,
    
    /// Failed
    Failed,
}

/// Sort options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortOption {
    /// Sort by popularity
    Popularity,
    
    /// Sort by rating
    Rating,
    
    /// Sort by downloads
    Downloads,
    
    /// Sort by name
    Name,
    
    /// Sort by date
    Date,
    
    /// Sort by price
    Price,
}

/// Enhanced marketplace message
#[derive(Clone, Debug)]
pub enum Message {
    /// Search query changed
    SearchQueryChanged(String),
    
    /// Advanced filters changed
    FiltersChanged(AdvancedFilters),
    
    /// Category selected
    CategorySelected(Option<String>),
    
    /// Sort option changed
    SortOptionChanged(SortOption),
    
    /// Plugin selected
    PluginSelected(EnhancedPluginDisplayInfo),
    
    /// Install plugin
    InstallPlugin(String),
    
    /// Uninstall plugin
    UninstallPlugin(String),
    
    /// Update plugin
    UpdatePlugin(String),
    
    /// Enable plugin
    EnablePlugin(String),
    
    /// Disable plugin
    DisablePlugin(String),
    
    /// Refresh plugin list
    Refresh,
    
    /// Close plugin details
    CloseDetails,
    
    /// Load reviews
    LoadReviews(String),
    
    /// Mark review as helpful
    MarkReviewHelpful(String, String),
    
    /// View screenshot
    ViewScreenshot(String, usize),
    
    /// View video
    ViewVideo(String, usize),
    
    /// Clear filters
    ClearFilters,
}

impl EnhancedMarketplaceState {
    /// Create a new enhanced marketplace state
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            filters: AdvancedFilters::default(),
            selected_category: None,
            sort_option: SortOption::Popularity,
            plugins: Vec::new(),
            filtered_plugins: Vec::new(),
            selected_plugin: None,
            loading: false,
            error: None,
            installation_progress: HashMap::new(),
            reviews: HashMap::new(),
            featured_plugins: Vec::new(),
            popular_plugins: Vec::new(),
            new_plugins: Vec::new(),
        }
    }
    
    /// Update marketplace state
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                self.filter_plugins();
            }
            
            Message::FiltersChanged(filters) => {
                self.filters = filters;
                self.filter_plugins();
            }
            
            Message::CategorySelected(category) => {
                self.selected_category = category;
                self.filter_plugins();
            }
            
            Message::SortOptionChanged(sort_option) => {
                self.sort_option = sort_option;
                self.sort_plugins();
            }
            
            Message::PluginSelected(plugin) => {
                self.selected_plugin = Some(plugin);
            }
            
            Message::InstallPlugin(id) => {
                self.install_plugin(id);
            }
            
            Message::UninstallPlugin(id) => {
                self.uninstall_plugin(id);
            }
            
            Message::UpdatePlugin(id) => {
                self.update_plugin(id);
            }
            
            Message::EnablePlugin(id) => {
                self.enable_plugin(id);
            }
            
            Message::DisablePlugin(id) => {
                self.disable_plugin(id);
            }
            
            Message::Refresh => {
                self.refresh_plugins();
            }
            
            Message::CloseDetails => {
                self.selected_plugin = None;
            }
            
            Message::LoadReviews(plugin_id) => {
                self.load_reviews(plugin_id);
            }
            
            Message::MarkReviewHelpful(plugin_id, review_id) => {
                self.mark_review_helpful(plugin_id, review_id);
            }
            
            Message::ViewScreenshot(plugin_id, index) => {
                // Handle screenshot viewing
                debug!("Viewing screenshot {} for plugin {}", index, plugin_id);
            }
            
            Message::ViewVideo(plugin_id, index) => {
                // Handle video viewing
                debug!("Viewing video {} for plugin {}", index, plugin_id);
            }
            
            Message::ClearFilters => {
                self.filters = AdvancedFilters::default();
                self.filter_plugins();
            }
        }
    }
    
    /// Filter plugins based on search and filters
    fn filter_plugins(&mut self) {
        let query_lower = self.search_query.to_lowercase();
        
        self.filtered_plugins = self.plugins.iter().filter(|plugin| {
            // Search query match
            let query_match = query_lower.is_empty()
                || plugin.name.to_lowercase().contains(&query_lower)
                || plugin.description.to_lowercase().contains(&query_lower)
                || plugin.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower));
            
            // Category match
            let category_match = self.selected_category.as_ref().map_or(true, |cat| {
                plugin.category == *cat || plugin.subcategories.contains(cat)
            });
            
            // Rating filter
            let rating_match = plugin.rating >= self.filters.min_rating;
            
            // File size filter
            let size_match = self.filters.max_file_size.map_or(true, |max_size| {
                plugin.file_size <= max_size * 1024 * 1024
            });
            
            // Free only filter
            let free_match = !self.filters.free_only || plugin.price == 0.0;
            
            // Installed only filter
            let installed_match = !self.filters.installed_only || plugin.installed;
            
            // Update available only filter
            let update_match = !self.filters.update_available_only || plugin.update_available;
            
            // Tags filter
            let tags_match = self.filters.selected_tags.is_empty()
                || self.filters.selected_tags.iter().all(|tag| plugin.tags.contains(tag));
            
            // Author filter
            let author_match = self.filters.author.as_ref().map_or(true, |author| {
                plugin.author.to_lowercase().contains(&author.to_lowercase())
            });
            
            query_match
                && category_match
                && rating_match
                && size_match
                && free_match
                && installed_match
                && update_match
                && tags_match
                && author_match
        }).cloned().collect();
        
        self.sort_plugins();
    }
    
    /// Sort plugins based on sort option
    fn sort_plugins(&mut self) {
        match self.sort_option {
            SortOption::Popularity => {
                self.filtered_plugins.sort_by(|a, b| {
                    b.downloads.cmp(&a.downloads)
                });
            }
            
            SortOption::Rating => {
                self.filtered_plugins.sort_by(|a, b| {
                    b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            
            SortOption::Downloads => {
                self.filtered_plugins.sort_by(|a, b| {
                    b.downloads.cmp(&a.downloads)
                });
            }
            
            SortOption::Name => {
                self.filtered_plugins.sort_by(|a, b| {
                    a.name.cmp(&b.name)
                });
            }
            
            SortOption::Date => {
                self.filtered_plugins.sort_by(|a, b| {
                    b.last_updated.cmp(&a.last_updated)
                });
            }
            
            SortOption::Price => {
                self.filtered_plugins.sort_by(|a, b| {
                    a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
    }
    
    /// Install plugin
    fn install_plugin(&mut self, id: String) {
        let progress = InstallationProgress {
            plugin_id: id.clone(),
            progress: 0.0,
            status: InstallationStatus::Downloading,
            current_step: "Downloading plugin...".to_string(),
            error: None,
        };
        
        self.installation_progress.insert(id.clone(), progress);
        
        // Simulate installation progress
        // In a real implementation, this would be async
        debug!("Installing plugin: {}", id);
    }
    
    /// Uninstall plugin
    fn uninstall_plugin(&mut self, id: String) {
        debug!("Uninstalling plugin: {}", id);
        
        // Update plugin state
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.installed = false;
            plugin.enabled = false;
        }
        
        self.filter_plugins();
    }
    
    /// Update plugin
    fn update_plugin(&mut self, id: String) {
        debug!("Updating plugin: {}", id);
        
        let progress = InstallationProgress {
            plugin_id: id.clone(),
            progress: 0.0,
            status: InstallationStatus::Downloading,
            current_step: "Downloading update...".to_string(),
            error: None,
        };
        
        self.installation_progress.insert(id.clone(), progress);
    }
    
    /// Enable plugin
    fn enable_plugin(&mut self, id: String) {
        debug!("Enabling plugin: {}", id);
        
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = true;
        }
    }
    
    /// Disable plugin
    fn disable_plugin(&mut self, id: String) {
        debug!("Disabling plugin: {}", id);
        
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = false;
        }
    }
    
    /// Refresh plugins
    fn refresh_plugins(&mut self) {
        self.loading = true;
        // In a real implementation, this would fetch from API
        self.loading = false;
    }
    
    /// Load reviews for plugin
    fn load_reviews(&mut self, plugin_id: String) {
        debug!("Loading reviews for plugin: {}", plugin_id);
        // In a real implementation, this would fetch from API
    }
    
    /// Mark review as helpful
    fn mark_review_helpful(&mut self, plugin_id: String, review_id: String) {
        debug!("Marking review {} as helpful for plugin {}", review_id, plugin_id);
        
        if let Some(reviews) = self.reviews.get_mut(&plugin_id) {
            if let Some(review) = reviews.iter_mut().find(|r| r.id == review_id) {
                review.marked_helpful = !review.marked_helpful;
                review.helpful_count += if review.marked_helped { 1 } else { -1 };
            }
        }
    }
    
    /// Get installation progress for plugin
    pub fn get_installation_progress(&self, plugin_id: &str) -> Option<&InstallationProgress> {
        self.installation_progress.get(plugin_id)
    }
    
    /// Get reviews for plugin
    pub fn get_reviews(&self, plugin_id: &str) -> Option<&[PluginReview]> {
        self.reviews.get(plugin_id).map(|reviews| reviews.as_slice())
    }
    
    /// Set plugins
    pub fn set_plugins(&mut self, plugins: Vec<EnhancedPluginDisplayInfo>) {
        self.plugins = plugins;
        
        // Update featured, popular, and new plugins
        self.featured_plugins = self.plugins.iter().filter(|p| p.featured).cloned().collect();
        self.popular_plugins = self.plugins.iter().take(10).cloned().collect();
        self.new_plugins = self.plugins.iter().rev().take(10).cloned().collect();
        
        self.filter_plugins();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_marketplace_state_creation() {
        let state = EnhancedMarketplaceState::new();
        assert!(state.search_query.is_empty());
        assert!(state.plugins.is_empty());
        assert!(state.selected_plugin.is_none());
    }

    #[test]
    fn test_advanced_filters_default() {
        let filters = AdvancedFilters::default();
        assert_eq!(filters.min_rating, 0.0);
        assert!(filters.max_file_size.is_none());
        assert!(!filters.free_only);
        assert!(!filters.installed_only);
        assert!(filters.selected_tags.is_empty());
    }

    #[test]
    fn test_media_type() {
        assert_eq!(MediaType::Image as i32, 0);
        assert_eq!(MediaType::Video as i32, 1);
    }

    #[test]
    fn test_installation_status() {
        assert_eq!(InstallationStatus::Downloading as i32, 0);
        assert_eq!(InstallationStatus::Installing as i32, 1);
        assert_eq!(InstallationStatus::Verifying as i32, 2);
        assert_eq!(InstallationStatus::Complete as i32, 3);
        assert_eq!(InstallationStatus::Failed as i32, 4);
    }

    #[test]
    fn test_sort_option() {
        assert_eq!(SortOption::Popularity as i32, 0);
        assert_eq!(SortOption::Rating as i32, 1);
        assert_eq!(SortOption::Downloads as i32, 2);
        assert_eq!(SortOption::Name as i32, 3);
        assert_eq!(SortOption::Date as i32, 4);
        assert_eq!(SortOption::Price as i32, 5);
    }

    #[test]
    fn test_filter_plugins() {
        let mut state = EnhancedMarketplaceState::new();
        
        let plugin1 = EnhancedPluginDisplayInfo {
            id: "1".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "A test plugin".to_string(),
            long_description: String::new(),
            category: "Audio".to_string(),
            subcategories: vec![],
            rating: 4.5,
            rating_count: 100,
            downloads: 1000,
            file_size: 1024 * 1024,
            price: 0.0,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![],
            videos: vec![],
            tags: vec!["audio".to_string(), "test".to_string()],
            license: "MIT".to_string(),
            last_updated: "2025-01-01".to_string(),
            featured: false,
            verified: false,
            dependencies: vec![],
            compatibility: vec![],
        };
        
        state.set_plugins(vec![plugin1.clone()]);
        
        // Test search filter
        state.search_query = "test".to_string();
        state.filter_plugins();
        assert_eq!(state.filtered_plugins.len(), 1);
        
        // Test category filter
        state.search_query = String::new();
        state.selected_category = Some("Audio".to_string());
        state.filter_plugins();
        assert_eq!(state.filtered_plugins.len(), 1);
        
        // Test rating filter
        state.selected_category = None;
        state.filters.min_rating = 5.0;
        state.filter_plugins();
        assert_eq!(state.filtered_plugins.len(), 0);
    }

    #[test]
    fn test_sort_plugins() {
        let mut state = EnhancedMarketplaceState::new();
        
        let plugin1 = EnhancedPluginDisplayInfo {
            id: "1".to_string(),
            name: "A Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Author".to_string(),
            description: "Description".to_string(),
            long_description: String::new(),
            category: "Category".to_string(),
            subcategories: vec![],
            rating: 4.0,
            rating_count: 100,
            downloads: 1000,
            file_size: 1024 * 1024,
            price: 0.0,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![],
            videos: vec![],
            tags: vec![],
            license: "MIT".to_string(),
            last_updated: "2025-01-01".to_string(),
            featured: false,
            verified: false,
            dependencies: vec![],
            compatibility: vec![],
        };
        
        let plugin2 = EnhancedPluginDisplayInfo {
            id: "2".to_string(),
            name: "B Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Author".to_string(),
            description: "Description".to_string(),
            long_description: String::new(),
            category: "Category".to_string(),
            subcategories: vec![],
            rating: 5.0,
            rating_count: 100,
            downloads: 500,
            file_size: 1024 * 1024,
            price: 0.0,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![],
            videos: vec![],
            tags: vec![],
            license: "MIT".to_string(),
            last_updated: "2025-01-01".to_string(),
            featured: false,
            verified: false,
            dependencies: vec![],
            compatibility: vec![],
        };
        
        state.set_plugins(vec![plugin1, plugin2]);
        
        // Sort by name
        state.sort_option = SortOption::Name;
        state.sort_plugins();
        assert_eq!(state.filtered_plugins[0].name, "A Plugin");
        
        // Sort by rating
        state.sort_option = SortOption::Rating;
        state.sort_plugins();
        assert_eq!(state.filtered_plugins[0].rating, 5.0);
    }

    #[test]
    fn test_installation_progress() {
        let mut state = EnhancedMarketplaceState::new();
        
        let progress = InstallationProgress {
            plugin_id: "test".to_string(),
            progress: 50.0,
            status: InstallationStatus::Installing,
            current_step: "Installing...".to_string(),
            error: None,
        };
        
        state.installation_progress.insert("test".to_string(), progress);
        
        let retrieved = state.get_installation_progress("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().progress, 50.0);
    }

    #[test]
    fn test_mark_review_helpful() {
        let mut state = EnhancedMarketplaceState::new();
        
        let review = PluginReview {
            id: "1".to_string(),
            reviewer_name: "User".to_string(),
            reviewer_avatar: None,
            rating: 5.0,
            title: "Great".to_string(),
            content: "Great plugin".to_string(),
            date: "2025-01-01".to_string(),
            helpful_count: 0,
            marked_helpful: false,
        };
        
        state.reviews.insert("plugin1".to_string(), vec![review]);
        
        state.mark_review_helpful("plugin1".to_string(), "1".to_string());
        
        let reviews = state.get_reviews("plugin1");
        assert!(reviews.is_some());
        assert_eq!(reviews.unwrap()[0].helpful_count, 1);
    }
}