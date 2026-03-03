//! Enhanced Plugin Marketplace UI Example
//!
//! This example demonstrates the enhanced plugin marketplace UI features of Vantis Media Player:
//! - Advanced search with multiple filters
//! - Plugin categories and tags system
//! - Plugin screenshots and videos
//! - Rating and review display
//! - Installation progress indicator

use std::collections::HashMap;
use vantisui::enhanced_marketplace::{
    AdvancedFilters, EnhancedMarketplaceState, EnhancedPluginDisplayInfo, InstallationProgress,
    InstallationStatus, MediaAsset, MediaType, Message, PluginReview, SortOption,
};

fn main() -> anyhow::Result<()> {
    println!("=== Vantis Media Player - Enhanced Plugin Marketplace UI Example ===\n");

    // Example 1: Basic marketplace setup
    println!("Example 1: Basic Marketplace Setup");
    basic_marketplace_setup()?;

    // Example 2: Advanced search and filtering
    println!("\nExample 2: Advanced Search and Filtering");
    advanced_search_filtering()?;

    // Example 3: Plugin categories and tags
    println!("\nExample 3: Plugin Categories and Tags");
    plugin_categories_tags()?;

    // Example 4: Screenshots and videos
    println!("\nExample 4: Screenshots and Videos");
    screenshots_videos()?;

    // Example 5: Ratings and reviews
    println!("\nExample 5: Ratings and Reviews");
    ratings_reviews()?;

    // Example 6: Installation progress
    println!("\nExample 6: Installation Progress");
    installation_progress()?;

    // Example 7: Complete workflow
    println!("\nExample 7: Complete Workflow");
    complete_workflow()?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Basic marketplace setup
fn basic_marketplace_setup() -> anyhow::Result<()> {
    let mut state = EnhancedMarketplaceState::new();

    println!("  Marketplace state created");
    println!("    Search query: '{}'", state.search_query);
    println!("    Plugins count: {}", state.plugins.len());
    println!("    Selected plugin: {:?}", state.selected_plugin);

    Ok(())
}

/// Example 2: Advanced search and filtering
fn advanced_search_filtering() -> anyhow::Result<()> {
    let mut state = EnhancedMarketplaceState::new();

    // Create sample plugins
    let plugins = create_sample_plugins();
    state.set_plugins(plugins);

    println!("  Total plugins: {}", state.plugins.len());

    // Search by name
    state.update(Message::SearchQueryChanged("audio".to_string()));
    println!("\n  Search for 'audio': {} results", state.filtered_plugins.len());

    // Filter by rating
    let mut filters = AdvancedFilters::default();
    filters.min_rating = 4.0;
    state.update(Message::FiltersChanged(filters));
    println!("  Filter by rating >= 4.0: {} results", state.filtered_plugins.len());

    // Filter by category
    state.update(Message::CategorySelected(Some("Audio".to_string())));
    println!("  Filter by category 'Audio': {} results", state.filtered_plugins.len());

    // Sort by rating
    state.update(Message::SortOptionChanged(SortOption::Rating));
    println!("\n  Top 3 plugins by rating:");
    for (i, plugin) in state.filtered_plugins.iter().take(3).enumerate() {
        println!("    {}. {} - Rating: {:.1}", i + 1, plugin.name, plugin.rating);
    }

    Ok(())
}

/// Example 3: Plugin categories and tags
fn plugin_categories_tags() -> anyhow::Result<()> {
    let plugins = create_sample_plugins();

    println!("  Plugin categories:");
    let categories: std::collections::HashSet<_> = plugins.iter().map(|p| p.category.clone()).collect();
    for category in categories {
        println!("    - {}", category);
    }

    println!("\n  Plugin tags:");
    let tags: std::collections::HashSet<_> = plugins.iter().flat_map(|p| p.tags.clone()).collect();
    for tag in tags {
        println!("    - {}", tag);
    }

    println!("\n  Plugins with subcategories:");
    for plugin in &plugins {
        if !plugin.subcategories.is_empty() {
            println!("    {} - Subcategories: {:?}", plugin.name, plugin.subcategories);
        }
    }

    Ok(())
}

/// Example 4: Screenshots and videos
fn screenshots_videos() -> anyhow::Result<()> {
    let plugins = create_sample_plugins();

    println!("  Plugins with screenshots:");
    for plugin in &plugins {
        if !plugin.screenshots.is_empty() {
            println!("    {} - {} screenshots", plugin.name, plugin.screenshots.len());
            for (i, screenshot) in plugin.screenshots.iter().enumerate() {
                println!("      {}. {} ({:?})", i + 1, screenshot.url, screenshot.asset_type);
                if let Some(caption) = &screenshot.caption {
                    println!("         Caption: {}", caption);
                }
            }
        }
    }

    println!("\n  Plugins with videos:");
    for plugin in &plugins {
        if !plugin.videos.is_empty() {
            println!("    {} - {} videos", plugin.name, plugin.videos.len());
            for (i, video) in plugin.videos.iter().enumerate() {
                println!("      {}. {} ({:?})", i + 1, video.url, video.asset_type);
            }
        }
    }

    Ok(())
}

/// Example 5: Ratings and reviews
fn ratings_reviews() -> anyhow::Result<()> {
    let mut state = EnhancedMarketplaceState::new();

    // Create sample plugins
    let plugins = create_sample_plugins();
    state.set_plugins(plugins);

    // Add reviews for a plugin
    let reviews = vec![
        PluginReview {
            id: "1".to_string(),
            reviewer_name: "John Doe".to_string(),
            reviewer_avatar: Some("https://example.com/avatar1.jpg".to_string()),
            rating: 5.0,
            title: "Excellent plugin!".to_string(),
            content: "This plugin works perfectly and has great features.".to_string(),
            date: "2025-01-15".to_string(),
            helpful_count: 10,
            marked_helpful: false,
        },
        PluginReview {
            id: "2".to_string(),
            reviewer_name: "Jane Smith".to_string(),
            reviewer_avatar: Some("https://example.com/avatar2.jpg".to_string()),
            rating: 4.0,
            title: "Good but needs improvement".to_string(),
            content: "Overall good, but could use some UI improvements.".to_string(),
            date: "2025-01-10".to_string(),
            helpful_count: 5,
            marked_helpful: false,
        },
    ];

    state.reviews.insert("audio-enhancer".to_string(), reviews);

    println!("  Plugin ratings:");
    for plugin in &state.plugins {
        println!("    {} - Rating: {:.1}/5 ({} reviews)", 
                 plugin.name, plugin.rating, plugin.rating_count);
    }

    println!("\n  Reviews for 'Audio Enhancer':");
    if let Some(reviews) = state.get_reviews("audio-enhancer") {
        for (i, review) in reviews.iter().enumerate() {
            println!("    {}. {} - {:.1}/5", i + 1, review.title, review.rating);
            println!("       By: {}", review.reviewer_name);
            println!("       Date: {}", review.date);
            println!("       Helpful: {}", review.helpful_count);
            println!("       Content: {}", review.content);
        }
    }

    // Mark review as helpful
    state.update(Message::MarkReviewHelpful("audio-enhancer".to_string(), "1".to_string()));
    println!("\n  After marking review as helpful:");
    if let Some(reviews) = state.get_reviews("audio-enhancer") {
        println!("    Review 1 helpful count: {}", reviews[0].helpful_count);
    }

    Ok(())
}

/// Example 6: Installation progress
fn installation_progress() -> anyhow::Result<()> {
    let mut state = EnhancedMarketplaceState::new();

    // Create sample plugins
    let plugins = create_sample_plugins();
    state.set_plugins(plugins);

    // Install a plugin
    println!("  Installing 'Audio Enhancer'...");
    state.update(Message::InstallPlugin("audio-enhancer".to_string()));

    // Check installation progress
    if let Some(progress) = state.get_installation_progress("audio-enhancer") {
        println!("    Status: {:?}", progress.status);
        println!("    Progress: {:.1}%", progress.progress);
        println!("    Current step: {}", progress.current_step);
    }

    // Simulate progress update
    let mut progress = InstallationProgress {
        plugin_id: "audio-enhancer".to_string(),
        progress: 50.0,
        status: InstallationStatus::Installing,
        current_step: "Installing files...".to_string(),
        error: None,
    };
    state.installation_progress.insert("audio-enhancer".to_string(), progress);

    println!("\n  Installation progress updated:");
    if let Some(progress) = state.get_installation_progress("audio-enhancer") {
        println!("    Status: {:?}", progress.status);
        println!("    Progress: {:.1}%", progress.progress);
        println!("    Current step: {}", progress.current_step);
    }

    // Complete installation
    progress.progress = 100.0;
    progress.status = InstallationStatus::Complete;
    progress.current_step = "Installation complete!".to_string();
    state.installation_progress.insert("audio-enhancer".to_string(), progress);

    println!("\n  Installation complete:");
    if let Some(progress) = state.get_installation_progress("audio-enhancer") {
        println!("    Status: {:?}", progress.status);
        println!("    Progress: {:.1}%", progress.progress);
        println!("    Current step: {}", progress.current_step);
    }

    Ok(())
}

/// Example 7: Complete workflow
fn complete_workflow() -> anyhow::Result<()> {
    let mut state = EnhancedMarketplaceState::new();

    // Load plugins
    println!("  Step 1: Loading plugins...");
    let plugins = create_sample_plugins();
    state.set_plugins(plugins);
    println!("    Loaded {} plugins", state.plugins.len());

    // Search for plugins
    println!("\n  Step 2: Searching for plugins...");
    state.update(Message::SearchQueryChanged("audio".to_string()));
    println!("    Found {} plugins matching 'audio'", state.filtered_plugins.len());

    // Apply filters
    println!("\n  Step 3: Applying filters...");
    let mut filters = AdvancedFilters::default();
    filters.min_rating = 4.0;
    filters.free_only = true;
    state.update(Message::FiltersChanged(filters));
    println!("    After filters: {} plugins", state.filtered_plugins.len());

    // Sort results
    println!("\n  Step 4: Sorting results...");
    state.update(Message::SortOptionChanged(SortOption::Rating));
    println!("    Sorted by rating");

    // Select plugin
    println!("\n  Step 5: Selecting plugin...");
    if let Some(plugin) = state.filtered_plugins.first() {
        state.update(Message::PluginSelected(plugin.clone()));
        println!("    Selected: {}", plugin.name);
        println!("    Rating: {:.1}/5", plugin.rating);
        println!("    Downloads: {}", plugin.downloads);
        println!("    Price: ${}", plugin.price);
    }

    // Load reviews
    println!("\n  Step 6: Loading reviews...");
    state.update(Message::LoadReviews("audio-enhancer".to_string()));
    if let Some(reviews) = state.get_reviews("audio-enhancer") {
        println!("    Loaded {} reviews", reviews.len());
    }

    // Install plugin
    println!("\n  Step 7: Installing plugin...");
    state.update(Message::InstallPlugin("audio-enhancer".to_string()));
    println!("    Installation started");

    // Check progress
    if let Some(progress) = state.get_installation_progress("audio-enhancer") {
        println!("    Status: {:?}", progress.status);
        println!("    Progress: {:.1}%", progress.progress);
    }

    Ok(())
}

/// Create sample plugins for testing
fn create_sample_plugins() -> Vec<EnhancedPluginDisplayInfo> {
    vec![
        EnhancedPluginDisplayInfo {
            id: "audio-enhancer".to_string(),
            name: "Audio Enhancer".to_string(),
            version: "2.1.0".to_string(),
            author: "AudioMaster".to_string(),
            description: "Enhance audio quality with advanced effects".to_string(),
            long_description: "A comprehensive audio enhancement plugin with EQ, reverb, and spatial audio support.".to_string(),
            category: "Audio".to_string(),
            subcategories: vec!["Effects".to_string(), "Equalizer".to_string()],
            rating: 4.5,
            rating_count: 150,
            downloads: 10000,
            file_size: 5 * 1024 * 1024,
            price: 0.0,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![
                MediaAsset {
                    url: "https://example.com/screenshot1.jpg".to_string(),
                    asset_type: MediaType::Image,
                    thumbnail_url: Some("https://example.com/thumb1.jpg".to_string()),
                    caption: Some("Main interface".to_string()),
                },
                MediaAsset {
                    url: "https://example.com/screenshot2.jpg".to_string(),
                    asset_type: MediaType::Image,
                    thumbnail_url: Some("https://example.com/thumb2.jpg".to_string()),
                    caption: Some("Settings panel".to_string()),
                },
            ],
            videos: vec![
                MediaAsset {
                    url: "https://example.com/demo.mp4".to_string(),
                    asset_type: MediaType::Video,
                    thumbnail_url: Some("https://example.com/video-thumb.jpg".to_string()),
                    caption: Some("Demo video".to_string()),
                },
            ],
            tags: vec!["audio".to_string(), "enhancement".to_string(), "eq".to_string()],
            license: "MIT".to_string(),
            last_updated: "2025-02-15".to_string(),
            featured: true,
            verified: true,
            dependencies: vec![],
            compatibility: vec!["v1.0.0".to_string(), "v1.1.0".to_string()],
        },
        EnhancedPluginDisplayInfo {
            id: "video-upscaler".to_string(),
            name: "Video Upscaler".to_string(),
            version: "1.5.0".to_string(),
            author: "VideoPro".to_string(),
            description: "AI-powered video upscaling to 4K".to_string(),
            long_description: "Use advanced AI to upscale videos to 4K resolution with minimal quality loss.".to_string(),
            category: "Video".to_string(),
            subcategories: vec!["Upscaling".to_string(), "AI".to_string()],
            rating: 4.8,
            rating_count: 200,
            downloads: 15000,
            file_size: 10 * 1024 * 1024,
            price: 9.99,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![],
            videos: vec![],
            tags: vec!["video".to_string(), "upscaling".to_string(), "ai".to_string(), "4k".to_string()],
            license: "Commercial".to_string(),
            last_updated: "2025-02-20".to_string(),
            featured: true,
            verified: true,
            dependencies: vec![],
            compatibility: vec!["v1.1.0".to_string()],
        },
        EnhancedPluginDisplayInfo {
            id: "subtitle-sync".to_string(),
            name: "Subtitle Sync".to_string(),
            version: "3.0.0".to_string(),
            author: "SubtitleMaster".to_string(),
            description: "Automatic subtitle synchronization".to_string(),
            long_description: "Automatically synchronize subtitles with video using AI-powered timing adjustment.".to_string(),
            category: "Subtitles".to_string(),
            subcategories: vec!["Synchronization".to_string()],
            rating: 4.2,
            rating_count: 80,
            downloads: 5000,
            file_size: 2 * 1024 * 1024,
            price: 0.0,
            installed: false,
            update_available: false,
            enabled: false,
            screenshots: vec![],
            videos: vec![],
            tags: vec!["subtitles".to_string(), "sync".to_string(), "timing".to_string()],
            license: "MIT".to_string(),
            last_updated: "2025-02-10".to_string(),
            featured: false,
            verified: true,
            dependencies: vec![],
            compatibility: vec!["v1.0.0".to_string(), "v1.1.0".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_marketplace_setup() {
        let result = basic_marketplace_setup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_search_filtering() {
        let result = advanced_search_filtering();
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_categories_tags() {
        let result = plugin_categories_tags();
        assert!(result.is_ok());
    }

    #[test]
    fn test_screenshots_videos() {
        let result = screenshots_videos();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ratings_reviews() {
        let result = ratings_reviews();
        assert!(result.is_ok());
    }

    #[test]
    fn test_installation_progress() {
        let result = installation_progress();
        assert!(result.is_ok());
    }

    #[test]
    fn test_complete_workflow() {
        let result = complete_workflow();
        assert!(result.is_ok());
    }
}