//! Core types for the AI recommendations module

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use chrono::{DateTime, Utc, Duration};

/// Unique identifier for content
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentId(String);

impl ContentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ContentId {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

/// Unique identifier for a user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

/// Content type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Movie,
    Series,
    Episode,
    Documentary,
    Short,
    Music,
    Podcast,
    Audiobook,
    LiveStream,
    UserGenerated,
}

impl ContentType {
    /// Get typical watch duration in minutes
    pub fn typical_duration(&self) -> i32 {
        match self {
            ContentType::Movie => 120,
            ContentType::Series => 45,
            ContentType::Episode => 45,
            ContentType::Documentary => 90,
            ContentType::Short => 15,
            ContentType::Music => 4,
            ContentType::Podcast => 45,
            ContentType::Audiobook => 60,
            ContentType::LiveStream => 180,
            ContentType::UserGenerated => 10,
        }
    }

    /// Check if content is episodic
    pub fn is_episodic(&self) -> bool {
        matches!(self, ContentType::Series | ContentType::Episode | ContentType::Podcast)
    }
}

/// Genre classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Genre(String);

impl Genre {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Standard genres
impl Genre {
    pub const ACTION: &'static str = "Action";
    pub const COMEDY: &'static str = "Comedy";
    pub const DRAMA: &'static str = "Drama";
    pub const HORROR: &'static str = "Horror";
    pub const SCI_FI: &'static str = "Sci-Fi";
    pub const THRILLER: &'static str = "Thriller";
    pub const ROMANCE: &'static str = "Romance";
    pub const DOCUMENTARY: &'static str = "Documentary";
    pub const ANIMATION: &'static str = "Animation";
    pub const FAMILY: &'static str = "Family";
}

/// Content metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    /// Unique identifier
    pub id: ContentId,
    /// Title of the content
    pub title: String,
    /// Content type
    pub content_type: ContentType,
    /// Genres
    pub genres: Vec<Genre>,
    /// Release year
    pub release_year: i32,
    /// Average rating (0.0 - 10.0)
    pub rating: f64,
    /// Duration in seconds
    pub duration_seconds: u32,
    /// Synopsis/description
    pub synopsis: Option<String>,
    /// Cast members
    pub cast: Vec<String>,
    /// Director(s)
    pub directors: Vec<String>,
    /// Content tags
    pub tags: HashSet<String>,
    /// Language code (ISO 639-1)
    pub language: String,
    /// Content maturity rating
    pub maturity_rating: Option<String>,
    /// Popularity score (0.0 - 1.0)
    pub popularity: f64,
    /// Custom features for ML
    pub custom_features: HashMap<String, f64>,
}

impl ContentMetadata {
    /// Create new content metadata
    pub fn new(id: ContentId, title: String, content_type: ContentType) -> Self {
        Self {
            id,
            title,
            content_type,
            genres: Vec::new(),
            release_year: 2024,
            rating: 0.0,
            duration_seconds: 0,
            synopsis: None,
            cast: Vec::new(),
            directors: Vec::new(),
            tags: HashSet::new(),
            language: "en".to_string(),
            maturity_rating: None,
            popularity: 0.0,
            custom_features: HashMap::new(),
        }
    }

    /// Builder-style genre addition
    pub fn with_genre(mut self, genre: impl Into<String>) -> Self {
        self.genres.push(Genre::new(genre));
        self
    }

    /// Builder-style cast addition
    pub fn with_cast(mut self, cast: Vec<String>) -> Self {
        self.cast = cast;
        self
    }

    /// Builder-style rating
    pub fn with_rating(mut self, rating: f64) -> Self {
        self.rating = rating.clamp(0.0, 10.0);
        self
    }

    /// Calculate age suitability score
    pub fn age_suitability(&self) -> f64 {
        match self.maturity_rating.as_deref() {
            Some("G") | Some("PG") => 1.0,
            Some("PG-13") => 0.8,
            Some("R") => 0.5,
            Some("NC-17") => 0.3,
            _ => 0.7,
        }
    }
}

/// A single recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommended content
    pub content_id: ContentId,
    /// Recommendation score (0.0 - 1.0)
    pub score: f64,
    /// Reason for recommendation
    pub reason: RecommendationReason,
    /// Confidence in the recommendation
    pub confidence: f64,
    /// Which strategy produced this recommendation
    pub strategy: String,
    /// Related content that led to this recommendation
    pub based_on: Option<ContentId>,
    /// Timestamp when generated
    pub generated_at: DateTime<Utc>,
}

impl Recommendation {
    /// Create a new recommendation
    pub fn new(content_id: ContentId, score: f64, reason: RecommendationReason) -> Self {
        Self {
            content_id,
            score: score.clamp(0.0, 1.0),
            reason,
            confidence: 0.5,
            strategy: "unknown".to_string(),
            based_on: None,
            generated_at: Utc::now(),
        }
    }

    /// Builder-style confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Builder-style strategy
    pub fn with_strategy(mut self, strategy: impl Into<String>) -> Self {
        self.strategy = strategy.into();
        self
    }
}

impl PartialOrd for Recommendation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Recommendation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by score descending, then by confidence
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                other
                    .confidence
                    .partial_cmp(&self.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

impl PartialEq for Recommendation {
    fn eq(&self, other: &Self) -> bool {
        self.content_id == other.content_id
    }
}

impl Eq for Recommendation {}

impl Hash for Recommendation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content_id.hash(state);
    }
}

/// Reason for recommendation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationReason {
    /// Similar to watched content
    SimilarToWatched,
    /// Similar to liked content
    SimilarToLiked,
    /// Popular among similar users
    PopularAmongSimilarUsers,
    /// Trending content
    Trending,
    /// New release matching preferences
    NewRelease,
    /// Because you watched X
    BecauseYouWatched(String),
    /// Because you liked X
    BecauseYouLiked(String),
    /// In your watchlist
    InWatchlist,
    /// Recommended by similar users
    RecommendedBySimilarUsers,
    /// Part of a series you follow
    PartOfFollowedSeries,
    /// Custom reason
    Custom(String),
}

/// A set of recommendations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationSet {
    /// Recommendations
    pub recommendations: Vec<Recommendation>,
    /// User the recommendations are for
    pub user_id: UserId,
    /// When recommendations were generated
    pub generated_at: DateTime<Utc>,
    /// Strategy used
    pub strategy: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

impl RecommendationSet {
    /// Create a new recommendation set
    pub fn new(user_id: UserId) -> Self {
        Self {
            recommendations: Vec::new(),
            user_id,
            generated_at: Utc::now(),
            strategy: "hybrid".to_string(),
            processing_time_ms: 0,
        }
    }

    /// Add a recommendation
    pub fn add(&mut self, recommendation: Recommendation) {
        self.recommendations.push(recommendation);
        self.recommendations.sort();
        self.recommendations.dedup();
    }

    /// Get top N recommendations
    pub fn top(&self, n: usize) -> Vec<&Recommendation> {
        self.recommendations.iter().take(n).collect()
    }

    /// Filter by minimum confidence
    pub fn with_min_confidence(&self, min: f64) -> Vec<&Recommendation> {
        self.recommendations
            .iter()
            .filter(|r| r.confidence >= min)
            .collect()
    }

    /// Get unique content IDs
    pub fn content_ids(&self) -> HashSet<&ContentId> {
        self.recommendations.iter().map(|r| &r.content_id).collect()
    }
}

/// Engagement level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl EngagementLevel {
    /// Convert from score (0.0 - 1.0)
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.8 => EngagementLevel::VeryHigh,
            s if s >= 0.6 => EngagementLevel::High,
            s if s >= 0.4 => EngagementLevel::Medium,
            _ => EngagementLevel::Low,
        }
    }
}

/// A watch event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    /// Content that was watched
    pub content_id: ContentId,
    /// When watching started
    pub started_at: DateTime<Utc>,
    /// When watching ended
    pub ended_at: Option<DateTime<Utc>>,
    /// Percentage watched (0.0 - 1.0)
    pub completion_ratio: f64,
    /// Whether user explicitly liked
    pub liked: Option<bool>,
    /// Whether added to watchlist
    pub added_to_watchlist: bool,
    /// Whether shared with others
    pub shared: bool,
    /// User rating given (1-5)
    pub user_rating: Option<u8>,
}

impl WatchEvent {
    /// Create a new watch event
    pub fn new(content_id: ContentId) -> Self {
        Self {
            content_id,
            started_at: Utc::now(),
            ended_at: None,
            completion_ratio: 0.0,
            liked: None,
            added_to_watchlist: false,
            shared: false,
            user_rating: None,
        }
    }

    /// Complete the watch event
    pub fn complete(&mut self, completion_ratio: f64) {
        self.ended_at = Some(Utc::now());
        self.completion_ratio = completion_ratio.clamp(0.0, 1.0);
    }

    /// Calculate engagement score
    pub fn engagement_score(&self) -> f64 {
        let mut score = 0.0;

        // Base: completion ratio (0-40 points)
        score += self.completion_ratio * 0.4;

        // Like/dislike (±20 points)
        if let Some(liked) = self.liked {
            score += if liked { 0.2 } else { -0.1 };
        }

        // Rating (0-20 points)
        if let Some(rating) = self.user_rating {
            score += (rating as f64 / 5.0) * 0.2;
        }

        // Watchlist (10 points)
        if self.added_to_watchlist {
            score += 0.1;
        }

        // Share (10 points)
        if self.shared {
            score += 0.1;
        }

        score.clamp(0.0, 1.0)
    }

    /// Get engagement level
    pub fn engagement_level(&self) -> EngagementLevel {
        EngagementLevel::from_score(self.engagement_score())
    }
}

/// Watch history for a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatchHistory {
    /// User ID
    pub user_id: UserId,
    /// Watch events
    pub events: Vec<WatchEvent>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

impl WatchHistory {
    /// Create new watch history
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            events: Vec::new(),
            updated_at: Utc::now(),
        }
    }

    /// Add a watch event
    pub fn add(&mut self, event: WatchEvent) {
        self.events.push(event);
        self.updated_at = Utc::now();
    }

    /// Get events for specific content
    pub fn for_content(&self, content_id: &ContentId) -> Vec<&WatchEvent> {
        self.events.iter().filter(|e| &e.content_id == content_id).collect()
    }

    /// Get recent events
    pub fn recent(&self, days: i64) -> Vec<&WatchEvent> {
        let cutoff = Utc::now() - Duration::days(days);
        self.events
            .iter()
            .filter(|e| e.started_at > cutoff)
            .collect()
    }

    /// Get total watch time in seconds
    pub fn total_watch_time_seconds(&self) -> i64 {
        self.events
            .iter()
            .filter_map(|e| {
                e.ended_at.map(|end| {
                    (end - e.started_at).num_seconds()
                })
            })
            .sum()
    }

    /// Get watched content IDs
    pub fn watched_content_ids(&self) -> HashSet<&ContentId> {
        self.events.iter().map(|e| &e.content_id).collect()
    }
}

/// Feature vector for content/user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureVector {
    /// Vector dimensions
    pub data: Vec<f64>,
    /// Feature names
    pub feature_names: Vec<String>,
}

impl FeatureVector {
    /// Create a new feature vector
    pub fn new(data: Vec<f64>) -> Self {
        let len = data.len();
        Self {
            data,
            feature_names: (0..len).map(|i| format!("feature_{}", i)).collect(),
        }
    }

    /// Create with named features
    pub fn with_names(data: Vec<f64>, names: Vec<String>) -> Self {
        Self {
            data,
            feature_names: names,
        }
    }

    /// Get dimensionality
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    /// Calculate cosine similarity with another vector
    pub fn cosine_similarity(&self, other: &FeatureVector) -> f64 {
        if self.dim() != other.dim() {
            return 0.0;
        }

        let dot: f64 = self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum();
        let norm_a: f64 = self.data.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = other.data.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }

    /// Calculate Euclidean distance
    pub fn euclidean_distance(&self, other: &FeatureVector) -> f64 {
        if self.dim() != other.dim() {
            return f64::MAX;
        }

        self.data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&self) -> Self {
        let norm: f64 = self.data.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm == 0.0 {
            return self.clone();
        }

        Self {
            data: self.data.iter().map(|x| x / norm).collect(),
            feature_names: self.feature_names.clone(),
        }
    }
}

/// Similarity score between two items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    /// First item ID
    pub item_a: String,
    /// Second item ID
    pub item_b: String,
    /// Similarity score (0.0 - 1.0)
    pub score: f64,
    /// Method used to calculate
    pub method: String,
}

impl SimilarityScore {
    pub fn new(item_a: impl Into<String>, item_b: impl Into<String>, score: f64) -> Self {
        Self {
            item_a: item_a.into(),
            item_b: item_b.into(),
            score: score.clamp(0.0, 1.0),
            method: "unknown".to_string(),
        }
    }
}

/// User preferences
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred genres (with weights)
    pub genre_weights: HashMap<String, f64>,
    /// Preferred content types
    pub content_type_weights: HashMap<String, f64>,
    /// Preferred actors
    pub actor_weights: HashMap<String, f64>,
    /// Preferred directors
    pub director_weights: HashMap<String, f64>,
    /// Preferred languages
    pub language_weights: HashMap<String, f64>,
    /// Preferred duration range (min, max in minutes)
    pub duration_preference: Option<(u32, u32)>,
    /// Minimum rating preference
    pub min_rating_preference: Option<f64>,
    /// Maximum maturity rating
    pub max_maturity_rating: Option<String>,
    /// Tags to avoid
    pub excluded_tags: HashSet<String>,
    /// Custom preferences
    pub custom: HashMap<String, f64>,
}

impl UserPreferences {
    /// Create new preferences
    pub fn new() -> Self {
        Self::default()
    }

    /// Add genre preference
    pub fn with_genre(mut self, genre: impl Into<String>, weight: f64) -> Self {
        self.genre_weights.insert(genre.into(), weight.clamp(0.0, 1.0));
        self
    }

    /// Add excluded tag
    pub fn exclude_tag(mut self, tag: impl Into<String>) -> Self {
        self.excluded_tags.insert(tag.into());
        self
    }

    /// Check if content matches preferences
    pub fn matches(&self, content: &ContentMetadata) -> bool {
        // Check excluded tags
        if content.tags.iter().any(|t| self.excluded_tags.contains(t)) {
            return false;
        }

        // Check minimum rating
        if let Some(min) = self.min_maturity_rating {
            if content.rating < min {
                return false;
            }
        }

        true
    }
}

/// User profile for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub user_id: UserId,
    /// Display name
    pub display_name: String,
    /// Preferences
    pub preferences: UserPreferences,
    /// Watch history
    pub watch_history: WatchHistory,
    /// Feature vector
    pub feature_vector: Option<FeatureVector>,
    /// Similar users
    pub similar_users: Vec<(UserId, f64)>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    /// Create a new user profile
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            display_name: String::new(),
            preferences: UserPreferences::new(),
            watch_history: WatchHistory::new(user_id.clone()),
            feature_vector: None,
            similar_users: Vec::new(),
            updated_at: Utc::now(),
        }
    }

    /// Update from watch history
    pub fn update_from_history(&mut self) {
        // Update genre weights based on watch history
        for event in &self.watch_history.events {
            let weight = event.engagement_score() * 0.1;
            let genre_key = format!("content_{}", event.content_id);
            *self.preferences.genre_weights.entry(genre_key).or_insert(0.0) += weight;
        }
        self.updated_at = Utc::now();
    }

    /// Get total content watched
    pub fn total_watched(&self) -> usize {
        self.watch_history.events.len()
    }
}

// Internal module for uuid generation
mod uuid {
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> Self {
            Self
        }
        
        pub fn to_string(&self) -> String {
            format!("uuid-{}", COUNTER.fetch_add(1, Ordering::SeqCst))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_id() {
        let id = ContentId::new("movie-123");
        assert_eq!(id.as_str(), "movie-123");
        assert_eq!(id.to_string(), "movie-123");
    }

    #[test]
    fn test_user_id() {
        let id = UserId::new("user-456");
        assert_eq!(id.as_str(), "user-456");
    }

    #[test]
    fn test_content_type() {
        assert_eq!(ContentType::Movie.typical_duration(), 120);
        assert!(ContentType::Series.is_episodic());
        assert!(!ContentType::Movie.is_episodic());
    }

    #[test]
    fn test_recommendation_ordering() {
        let r1 = Recommendation::new(ContentId::new("1"), 0.8, RecommendationReason::Trending);
        let r2 = Recommendation::new(ContentId::new("2"), 0.9, RecommendationReason::Trending);
        let mut recommendations = vec![r1, r2];
        recommendations.sort();
        assert_eq!(recommendations[0].score, 0.9);
    }

    #[test]
    fn test_watch_event_engagement() {
        let mut event = WatchEvent::new(ContentId::new("movie-1"));
        event.complete(1.0);
        event.liked = Some(true);
        event.user_rating = Some(5);
        
        let score = event.engagement_score();
        assert!(score > 0.7);
        assert_eq!(event.engagement_level(), EngagementLevel::VeryHigh);
    }

    #[test]
    fn test_feature_vector_similarity() {
        let v1 = FeatureVector::new(vec![1.0, 0.0, 0.0]);
        let v2 = FeatureVector::new(vec![1.0, 0.0, 0.0]);
        let v3 = FeatureVector::new(vec![0.0, 1.0, 0.0]);

        assert!((v1.cosine_similarity(&v2) - 1.0).abs() < 0.001);
        assert!((v1.cosine_similarity(&v3) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_user_preferences() {
        let prefs = UserPreferences::new()
            .with_genre("Action", 0.8)
            .with_genre("Comedy", 0.5)
            .exclude_tag("horror");

        assert_eq!(prefs.genre_weights.get("Action"), Some(&0.8));
        assert!(prefs.excluded_tags.contains("horror"));
    }
}