//! AI-Powered Content Recommendations for Vantis Media Player
//!
//! This module provides intelligent content recommendations using multiple strategies:
//! - Content-based filtering: Recommends similar content based on features
//! - Collaborative filtering: Recommends content based on similar users
//! - Hybrid approaches: Combines multiple strategies for better results
//!
//! # Example
//! ```rust
//! use vantis_ai_recommendations::{RecommendationEngine, ContentMetadata, UserId};
//!
//! #[tokio::main]
//! async fn main() {
//!     let engine = RecommendationEngine::new(Default::default());
//!     let user_id = UserId::new("user-123");
//!     let recommendations = engine.recommend(&user_id, 10).await;
//! }
//! ```

mod error;
mod types;
mod engine;
mod content_analyzer;
mod user_profile;
mod similarity;
mod smart_analysis;

pub use error::{RecommendationError, Result};
pub use types::{
    ContentId, UserId, ContentMetadata, ContentType, Genre,
    Recommendation, RecommendationSet, RecommendationReason,
    WatchEvent, WatchHistory, EngagementLevel, FeatureVector,
    UserPreferences, UserProfile, SimilarityScore,
};
pub use engine::{RecommendationEngine, EngineConfig, RecommendationStrategy};
pub use content_analyzer::{ContentAnalyzer, ContentFeatures, ContentCluster};
pub use user_profile::{UserProfileBuilder, PreferenceBuilder};
pub use similarity::{SimilarityCalculator, SimilarityMetric};

// Re-export smart analysis types
pub use smart_analysis::{
    Mood, MoodAnalysis, Sentiment, SentimentAnalysis,
    SmartCategory, SmartContentAnalyzer, ViewingContext,
    TimeOfDay, DayOfWeek, ViewingMode, ViewingCompany,
    ContentMetadataSimple, ContentRecommendation,
};

/// Prelude for common imports
pub mod prelude {
    pub use crate::types::{
        ContentId, UserId, ContentMetadata, ContentType, Genre,
        Recommendation, RecommendationSet, WatchEvent,
    };
    pub use crate::engine::{RecommendationEngine, EngineConfig};
    pub use crate::content_analyzer::ContentAnalyzer;
    pub use crate::user_profile::UserProfileBuilder;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");