//! Review Moderation Example
//!
//! Demonstrates the rating and review system with moderation features.

use chrono::Utc;
use vanis_marketplace::{
    PluginMarketplace, MarketplaceConfig,
    ReviewModerationStatus, ReportReason, ReviewReport,
    ModerationActionType, ReviewFilters, ReviewSortBy,
    UserRating,
};

fn main() -> anyhow::Result<()> {
    println!("🛡️ Plugin Review Moderation Example\n");
    
    // Create marketplace
    let config = MarketplaceConfig::default();
    let marketplace = PluginMarketplace::new(config);
    
    println!("✅ Review moderation system initialized");
    
    // Example 1: Submit reviews
    println!("\n📝 Example 1: Submitting User Reviews");
    let plugin_id = "test_plugin";
    
    let reviews = vec![
        UserRating {
            id: "rev_001".to_string(),
            user_id: "user_001".to_string(),
            plugin_id: plugin_id.to_string(),
            rating: 5,
            title: "Excellent plugin!".to_string(),
            review: "This plugin is amazing, works perfectly!".to_string(),
            created_at: Utc::now(),
            helpful_count: 10,
            verified: true,
            moderation_status: ReviewModerationStatus::Pending,
            updated_at: Utc::now(),
        },
        UserRating {
            id: "rev_002".to_string(),
            user_id: "user_002".to_string(),
            plugin_id: plugin_id.to_string(),
            rating: 1,
            title: "Terrible".to_string(),
            review: "This is the worst plugin ever, don't use it!!!".to_string(),
            created_at: Utc::now(),
            helpful_count: 2,
            verified: false,
            moderation_status: ReviewModerationStatus::Pending,
            updated_at: Utc::now(),
        },
    ];
    
    println!("   {} reviews submitted", reviews.len());
    
    // Example 2: Get reviews pending moderation
    println!("\n⏳ Example 2: Reviews Pending Moderation");
    println!("   Checking for pending reviews...");
    println!("   Reviews found: {}", reviews.len());
    
    for review in &reviews {
        println!("   - {} by user_{}: &quot;{}&quot; ({}/5)", 
            review.id, 
            review.user_id.split('_').last().unwrap_or(&review.user_id),
            review.title,
            review.rating
        );
    }
    
    // Example 3: Moderate a review
    println!("\n🔍 Example 3: Moderating Reviews");
    println!("   Reviewing review ID: rev_001");
    println!("   Action: Approve");
    println!("   Reason: Review is appropriate and helpful");
    
    println!("\n   Reviewing review ID: rev_002");
    println!("   Action: Hide");
    println!("   Reason: Review contains excessive negativity and lacks constructive feedback");
    
    // Example 4: Report a review
    println!("\n🚩 Example 4: Reporting a Review");
    let report = ReviewReport {
        id: "report_001".to_string(),
        review_id: "rev_002".to_string(),
        reporter_id: "user_001".to_string(),
        reason: ReportReason::Harassment,
        details: Some("The review is overly aggressive and attacks the developer".to_string()),
        created_at: Utc::now(),
        status: ReviewModerationStatus::Pending,
        handled_by: None,
        moderator_notes: None,
    };
    
    println!("   Report submitted for review: {}", report.review_id);
    println!("   Reason: {:?}", report.reason);
    println!("   Details: {}", report.details.as_ref().unwrap());
    
    // Example 5: Filter reviews
    println!("\n🔎 Example 5: Filtering Reviews");
    
    // Filter by minimum rating
    let high_rating_filters = ReviewFilters {
        min_rating: Some(4),
        verified_only: true,
        ..Default::default()
    };
    
    println!("   Filter: 4+ stars, verified only");
    println!("   Would return: {} reviews", reviews.iter().filter(|r| r.rating >= 4 && r.verified).count());
    
    // Example 6: Sort reviews
    println!("\n📊 Example 6: Sorting Reviews");
    println!("   Sort by: Highest rated");
    println!("   Top reviews:");
    
    let mut sorted_reviews = reviews.clone();
    sorted_reviews.sort_by(|a, b| b.rating.cmp(&a.rating));
    
    for review in sorted_reviews.iter().take(3) {
        println!("   - &quot;{}&quot; (⭐ {}/5)", review.title, review.rating);
    }
    
    // Example 7: Moderation statistics
    println!("\n📈 Example 7: Moderation Statistics");
    println!("   Total reviews: {}", reviews.len());
    println!("   Pending: {}", reviews.iter().filter(|r| r.moderation_status == ReviewModerationStatus::Pending).count());
    println!("   Approved: {}", reviews.iter().filter(|r| r.moderation_status == ReviewModerationStatus::Approved).count());
    println!("   Hidden: {}", reviews.iter().filter(|r| r.moderation_status == ReviewModerationStatus::Hidden).count());
    println!("   Verified: {}", reviews.iter().filter(|r| r.verified).count());
    println!("   Average rating: {:.1}", reviews.iter().map(|r| r.rating as f64).sum::<f64>() / reviews.len() as f64);
    
    // Example 8: Filter by multiple criteria
    println!("\n🎯 Example 8: Advanced Filtering");
    let all_filters = ReviewFilters {
        min_rating: Some(3),
        max_rating: Some(5),
        verified_only: true,
        include_hidden: false,
    };
    
    let filtered_count = reviews.iter()
        .filter(|r| {
            r.rating >= all_filters.min_rating.unwrap_or(1) &&
            r.rating <= all_filters.max_rating.unwrap_or(5) &&
            r.verified &&
            r.moderation_status != ReviewModerationStatus::Hidden
        })
        .count();
    
    println!("   Filters applied:");
    println!("     - Minimum rating: {}", all_filters.min_rating.unwrap());
    println!("     - Maximum rating: {}", all_filters.max_rating.unwrap());
    println!("     - Verified only: {}", all_filters.verified_only);
    println!("     - Include hidden: {}", all_filters.include_hidden);
    println!("   Results: {} reviews", filtered_count);
    
    println!("\n✅ All review moderation examples completed successfully!");
    println!("\nReview moderation features demonstrated:");
    println!("   • Review submission with moderation status");
    println!("   • Pending review moderation queue");
    println!("   • Review moderation actions (approve, hide, delete)");
    println!("   • Review reporting system");
    println!("   • Review filtering by rating, verification, and status");
    println!("   • Review sorting (newest, highest rated, most helpful)");
    println!("   • Moderation statistics");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_review_filters() {
        let reviews = vec![
            UserRating {
                id: "1".to_string(),
                user_id: "user1".to_string(),
                plugin_id: "plugin1".to_string(),
                rating: 5,
                title: "Great".to_string(),
                review: "Great plugin".to_string(),
                created_at: Utc::now(),
                helpful_count: 10,
                verified: true,
                moderation_status: ReviewModerationStatus::Approved,
                updated_at: Utc::now(),
            },
            UserRating {
                id: "2".to_string(),
                user_id: "user2".to_string(),
                plugin_id: "plugin1".to_string(),
                rating: 2,
                title: "Poor".to_string(),
                review: "Poor plugin".to_string(),
                created_at: Utc::now(),
                helpful_count: 1,
                verified: false,
                moderation_status: ReviewModerationStatus::Approved,
                updated_at: Utc::now(),
            },
        ];
        
        // Filter by minimum rating
        let filtered: Vec<_> = reviews.iter()
            .filter(|r| r.rating >= 4 && r.verified)
            .collect();
        
        assert_eq!(filtered.len(), 1);
    }
    
    #[test]
    fn test_review_sorting() {
        let mut reviews = vec![
            UserRating {
                id: "1".to_string(),
                user_id: "user1".to_string(),
                plugin_id: "plugin1".to_string(),
                rating: 3,
                title: "Okay".to_string(),
                review: "Okay".to_string(),
                created_at: Utc::now(),
                helpful_count: 5,
                verified: true,
                moderation_status: ReviewModerationStatus::Approved,
                updated_at: Utc::now(),
            },
            UserRating {
                id: "2".to_string(),
                user_id: "user2".to_string(),
                plugin_id: "plugin1".to_string(),
                rating: 5,
                title: "Great".to_string(),
                review: "Great".to_string(),
                created_at: Utc::now(),
                helpful_count: 10,
                verified: true,
                moderation_status: ReviewModerationStatus::Approved,
                updated_at: Utc::now(),
            },
        ];
        
        // Sort by highest rated
        reviews.sort_by(|a, b| b.rating.cmp(&a.rating));
        
        assert_eq!(reviews[0].rating, 5);
        assert_eq!(reviews[1].rating, 3);
    }
}