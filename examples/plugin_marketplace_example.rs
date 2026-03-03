//! Plugin Marketplace Example
//!
//! Demonstrates the official plugin marketplace functionality including
//! plugin submission, review, rating, and distribution.

use chrono::Utc;
use vanis_marketplace::{
    PluginMarketplace, MarketplaceConfig,
    MarketplacePlugin, PluginCategory, PluginStatus, Platform,
    PluginSubmission, SubmissionStatus, PluginManifest, PluginEngines,
    PluginReview, ReviewDecision, SecurityAssessment, CodeQualityAssessment,
    DocumentationAssessment, UserRating, Developer,
    SearchFilters,
};

fn main() -> anyhow::Result<()> {
    println!("🛒 Vantis Plugin Marketplace Example\n");
    
    // Create marketplace with default configuration
    let config = MarketplaceConfig::default();
    let marketplace = PluginMarketplace::new(config);
    
    println!("✅ Plugin marketplace initialized");
    println!("   - Marketplace API: {}", marketplace.config.api_url);
    println!("   - Max plugin size: {} MB", marketplace.config.max_plugin_size / (1024 * 1024));
    println!("   - Review required: {}", marketplace.config.require_review);
    
    // Example 1: Register a developer
    println!("\n👤 Example 1: Registering a Developer");
    let developer = Developer {
        id: "dev_001".to_string(),
        username: "vantis_studios".to_string(),
        display_name: "Vantis Studios".to_string(),
        email: "contact@vantisstudios.com".to_string(),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        bio: Some("Creating awesome media plugins since 2024".to_string()),
        website: Some("https://vantisstudios.com".to_string()),
        verified: true,
        trusted: false,
        total_plugins: 0,
        total_downloads: 0,
        created_at: Utc::now(),
    };
    
    marketplace.register_developer(developer)?;
    println!("   Developer registered: Vantis Studios");
    
    // Example 2: Create and add a plugin
    println!("\n📦 Example 2: Adding a Plugin to Marketplace");
    let plugin = MarketplacePlugin {
        id: "video_enhancer_pro".to_string(),
        name: "Video Enhancer Pro".to_string(),
        version: "2.1.0".to_string(),
        description: "Advanced video enhancement with AI upscaling, noise reduction, and color correction".to_string(),
        author: "Vantis Studios".to_string(),
        author_id: "dev_001".to_string(),
        categories: vec![PluginCategory::VideoProcessing],
        tags: vec!["upscaling".to_string(), "ai".to_string(), "enhancement".to_string()],
        homepage: Some("https://vantisstudios.com/video-enhancer".to_string()),
        repository: Some("https://github.com/vantisstudios/video-enhancer".to_string()),
        license: "MIT".to_string(),
        download_url: "https://marketplace.vantis.app/plugins/video_enhancer_pro/v2.1.0/plugin.wasm".to_string(),
        checksum: "a1b2c3d4e5f6".to_string(),
        platforms: vec![Platform::All],
        min_version: "1.0.0".to_string(),
        max_version: None,
        status: PluginStatus::Approved,
        published_at: Some(Utc::now()),
        updated_at: Utc::now(),
        icon_url: Some("https://example.com/icon.png".to_string()),
        screenshots: vec![
            "https://example.com/screenshot1.png".to_string(),
            "https://example.com/screenshot2.png".to_string(),
        ],
        changelog: vec![],
        dependencies: vec![],
        stats: Default::default(),
        verified: true,
        featured: true,
    };
    
    // Add plugin to marketplace (in real implementation, this would go through submission)
    let mut plugins = marketplace.plugins.write();
    plugins.insert(plugin.id.clone(), plugin.clone());
    drop(plugins);
    
    println!("   Plugin added: {} v{}", plugin.name, plugin.version);
    println!("   Categories: {:?}", plugin.categories);
    println!("   Status: {:?}", plugin.status);
    
    // Example 3: Search for plugins
    println!("\n🔍 Example 3: Searching for Plugins");
    let search_results = marketplace.search_plugins("video", SearchFilters::default());
    println!("   Found {} plugins matching 'video'", search_results.len());
    
    if !search_results.is_empty() {
        let first = &search_results[0];
        println!("   First result: {} - {}", first.name, first.description);
    }
    
    // Example 4: Get plugins by category
    println!("\n📂 Example 4: Get Plugins by Category");
    let video_plugins = marketplace.get_plugins_by_category(&PluginCategory::VideoProcessing);
    println!("   Video processing plugins: {}", video_plugins.len());
    
    for plugin in video_plugins.iter().take(3) {
        println!("   - {} v{} (⭐ {:.1})", plugin.name, plugin.version, plugin.stats.star_rating);
    }
    
    // Example 5: Get featured plugins
    println!("\n⭐ Example 5: Get Featured Plugins");
    let featured = marketplace.get_featured_plugins();
    println!("   Featured plugins: {}", featured.len());
    
    for plugin in featured.iter().take(5) {
        println!("   - {} (verified: {})", plugin.name, plugin.verified);
    }
    
    // Example 6: Submit a plugin for review
    println!("\n📝 Example 6: Submitting a Plugin for Review");
    let submission = PluginSubmission {
        id: "sub_001".to_string(),
        plugin_id: "video_enhancer_pro".to_string(),
        version: "2.2.0".to_string(),
        submitter_id: "dev_001".to_string(),
        submitted_at: Utc::now(),
        status: SubmissionStatus::Pending,
        review_notes: None,
        reviewer_id: None,
        reviewed_at: None,
        manifest: PluginManifest {
            name: "video_enhancer_pro".to_string(),
            version: "2.2.0".to_string(),
            description: plugin.description.clone(),
            author: plugin.author.clone(),
            license: "MIT".to_string(),
            main: "src/lib.rs".to_string(),
            homepage: plugin.homepage.clone(),
            repository: plugin.repository.clone(),
            keywords: plugin.tags,
            categories: vec!["video".to_string()],
            engines: PluginEngines {
                vanis: ">=1.0.0".to_string(),
            },
            dependencies: Default::default(),
        },
    };
    
    marketplace.submit_plugin(submission)?;
    println!("   Plugin submitted for review");
    
    // Example 7: Submit a plugin review
    println!("\n🔬 Example 7: Submitting a Plugin Review");
    let review = PluginReview {
        id: "review_001".to_string(),
        plugin_id: "video_enhancer_pro".to_string(),
        reviewer_id: "reviewer_001".to_string(),
        reviewed_at: Utc::now(),
        decision: ReviewDecision::Approve,
        security: SecurityAssessment {
            passed: true,
            score: 95.0,
            vulnerabilities: vec![],
            recommendations: vec![
                "Consider adding input validation".to_string(),
            ],
        },
        code_quality: CodeQualityAssessment {
            passed: true,
            score: 88.0,
            issues: vec![],
            recommendations: vec![
                "Add more unit tests".to_string(),
            ],
        },
        documentation: DocumentationAssessment {
            passed: true,
            score: 90.0,
            has_readme: true,
            has_changelog: true,
            has_api_docs: true,
            has_examples: true,
            missing_items: vec![],
        },
        comments: "Great plugin! Ready for distribution.".to_string(),
        issues: vec![],
    };
    
    marketplace.submit_review(review)?;
    println!("   Review submitted: Approved");
    
    // Example 8: Approve a submission
    println!("\n✅ Example 8: Approving a Plugin Submission");
    marketplace.approve_submission("sub_001", "reviewer_001", Some("Approved for distribution".to_string()))?;
    println!("   Submission approved");
    
    // Example 9: Submit user ratings
    println!("\n⭐ Example 9: Submitting User Ratings");
    let ratings = vec![
        UserRating {
            id: "rating_001".to_string(),
            user_id: "user_001".to_string(),
            plugin_id: "video_enhancer_pro".to_string(),
            rating: 5,
            title: "Amazing plugin!".to_string(),
            review: "This plugin transformed my video quality. Highly recommended!".to_string(),
            created_at: Utc::now(),
            helpful_count: 12,
            verified: true,
        },
        UserRating {
            id: "rating_002".to_string(),
            user_id: "user_002".to_string(),
            plugin_id: "video_enhancer_pro".to_string(),
            rating: 4,
            title: "Great but needs more features".to_string(),
            review: "Good enhancement quality, but I'd like more customization options.".to_string(),
            created_at: Utc::now(),
            helpful_count: 5,
            verified: true,
        },
    ];
    
    for rating in ratings {
        marketplace.submit_rating(rating)?;
    }
    
    println!("   Ratings submitted");
    
    // Example 10: Get rating summary
    println!("\n📊 Example 10: Get Rating Summary");
    let rating_summary = marketplace.get_rating_summary("video_enhancer_pro");
    
    if let Some(summary) = rating_summary {
        println!("   Plugin: {}", summary.plugin_id);
        println!("   Total ratings: {}", summary.total_ratings);
        println!("   Average rating: {:.1}/5.0", summary.average_rating);
        println!("   Rating distribution:");
        for (stars, count) in &summary.rating_distribution {
            println!("     {} stars: {} ratings", stars, count);
        }
    }
    
    // Example 11: Record downloads
    println!("\n⬇️  Example 11: Recording Plugin Downloads");
    for platform in [Platform::Windows, Platform::MacOS, Platform::Linux] {
        marketplace.record_download("video_enhancer_pro", platform, "2.1.0")?;
    }
    
    println!("   Downloads recorded");
    
    // Example 12: Get download statistics
    println!("\n📈 Example 12: Get Download Statistics");
    let download_stats = marketplace.get_download_stats("video_enhancer_pro");
    
    if let Some(stats) = download_stats {
        println!("   Total downloads: {}", stats.total);
        println!("   Downloads by platform:");
        for (platform, count) in &stats.by_platform {
            println!("     {}: {}", platform, count);
        }
        println!("   Downloads by version:");
        for (version, count) in &stats.by_version {
            println!("     {}: {}", version, count);
        }
    }
    
    // Example 13: Get plugin download URL
    println!("\n🔗 Example 13: Get Plugin Download URL");
    match marketplace.get_download_url("video_enhancer_pro", None) {
        Ok(url) => println!("   Download URL: {}", url),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Example 14: Verify plugin checksum
    println!("\n✓ Example 14: Verify Plugin Checksum");
    match marketplace.verify_checksum("video_enhancer_pro", "a1b2c3d4e5f6") {
        Ok(valid) => println!("   Checksum valid: {}", valid),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Example 15: Advanced search with filters
    println!("\n🎯 Example 15: Advanced Search with Filters");
    let filters = SearchFilters {
        category: Some(PluginCategory::VideoProcessing),
        platform: Some(Platform::Linux),
        min_rating: Some(4.0),
        verified_only: true,
        featured_only: false,
        ..Default::default()
    };
    
    let filtered_results = marketplace.search_plugins("", filters);
    println!("   Filtered results: {} plugins", filtered_results.len());
    
    println!("\n✅ All marketplace examples completed successfully!");
    println!("\nThe plugin marketplace provides:");
    println!("   • Plugin submission and review workflow");
    println!("   • Security, code quality, and documentation assessments");
    println!("   • Plugin rating and review system");
    println!("   • Download tracking and analytics");
    println!("   • Developer accounts and verification");
    println!("   • Plugin distribution with checksums");
    println!("   • Advanced search and filtering");
    
    Ok(())
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
            id: "test_dev".to_string(),
            username: "test".to_string(),
            display_name: "Test".to_string(),
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
    fn test_search_plugins() {
        let marketplace = PluginMarketplace::default_config();
        let results = marketplace.search_plugins("", SearchFilters::default());
        assert_eq!(results.len(), 0); // No plugins yet
    }
}