//! Content recommendation engine module
//! 
//! Provides AI-powered content recommendations including:
//! - Collaborative filtering
//! - Content-based filtering
/// - Hybrid recommendation
/// - Personalized suggestions
/// - Watch history analysis

use crate::{AIConfig, AIError, AIResult};
use crate::models::{AIModel, ModelType};
use crate::utils::{TensorOps, FeatureExtractor};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Recommendation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationConfig {
    /// Enable collaborative filtering
    pub enable_collaborative: bool,
    
    /// Enable content-based filtering
    pub enable_content_based: bool,
    
    /// Number of recommendations to return
    pub num_recommendations: usize,
    
    /// Minimum similarity threshold (0.0 - 1.0)
    pub min_similarity: f32,
    
    /// Enable watch history analysis
    pub enable_history_analysis: bool,
    
    /// History weight in hybrid model (0.0 - 1.0)
    pub history_weight: f32,
    
    /// Enable genre preferences
    pub enable_genre_preferences: bool,
    
    /// Enable time-based recommendations
    pub enable_time_based: bool,
    
    /// Cache recommendations
    pub enable_caching: bool,
    
    /// Cache duration in seconds
    pub cache_duration_secs: u64,
}

impl Default for RecommendationConfig {
    fn default() -> Self {
        Self {
            enable_collaborative: true,
            enable_content_based: true,
            num_recommendations: 10,
            min_similarity: 0.3,
            enable_history_analysis: true,
            history_weight: 0.6,
            enable_genre_preferences: true,
            enable_time_based: true,
            enable_caching: true,
            cache_duration_secs: 3600,
        }
    }
}

/// Content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    /// Unique identifier
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// Genres
    pub genres: Vec<String>,
    
    /// Year
    pub year: Option<u32>,
    
    /// Rating (0.0 - 10.0)
    pub rating: Option<f32>,
    
    /// Duration in minutes
    pub duration: Option<u32>,
    
    /// Language
    pub language: Option<String>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Cast
    pub cast: Vec<String>,
    
    /// Director
    pub director: Option<String>,
    
    /// Description
    pub description: Option<String>,
    
    /// Poster URL
    pub poster_url: Option<String>,
    
    /// Content features (vector)
    pub features: Option<Vec<f32>>,
}

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub user_id: String,
    
    /// Watch history
    pub watch_history: Vec<WatchEvent>,
    
    /// Genre preferences
    pub genre_preferences: HashMap<String, f32>,
    
    /// Favorite items
    pub favorites: HashSet<String>,
    
    /// Watchlist
    pub watchlist: HashSet<String>,
    
    /// Disliked items
    pub disliked: HashSet<String>,
    
    /// Average rating given
    pub avg_rating: f32,
    
    /// Total watch time in minutes
    pub total_watch_time: u64,
    
    /// Last active timestamp
    pub last_active: Option<u64>,
}

/// Watch event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    /// Content ID
    pub content_id: String,
    
    /// Start timestamp
    pub start_time: u64,
    
    /// End timestamp
    pub end_time: Option<u64>,
    
    /// Watch duration in seconds
    pub duration_secs: u64,
    
    /// Completion percentage (0.0 - 1.0)
    pub completion: f32,
    
    /// Rating given (0.0 - 10.0)
    pub rating: Option<f32>,
    
    /// Liked
    pub liked: Option<bool>,
}

/// Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Content item
    pub content: ContentItem,
    
    /// Relevance score (0.0 - 1.0)
    pub relevance_score: f32,
    
    /// Recommendation reason
    pub reason: RecommendationReason,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Source of recommendation
    pub source: RecommendationSource,
}

/// Recommendation reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationReason {
    /// Similar to watched content
    SimilarToWatched { content_id: String, similarity: f32 },
    
    /// Popular in genre
    PopularInGenre { genre: String },
    
    /// Trending
    Trending,
    
    /// Based on watch history
    BasedOnHistory,
    
    /// Similar users liked
    SimilarUsersLiked { user_count: usize },
    
    /// New release
    NewRelease,
    
    /// High rated
    HighRated { rating: f32 },
    
    /// Custom reason
    Custom(String),
}

/// Recommendation source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationSource {
    /// Collaborative filtering
    Collaborative,
    
    /// Content-based filtering
    ContentBased,
    
    /// Hybrid model
    Hybrid,
    
    /// Popular/trending
    Popular,
    
    /// Manual/curated
    Manual,
}

/// Recommendation engine
pub struct RecommendationEngine {
    config: RecommendationConfig,
    model: Option<AIModel>,
    feature_extractor: FeatureExtractor,
    user_profiles: HashMap<String, UserProfile>,
    content_items: HashMap<String, ContentItem>,
    similarity_cache: HashMap<(String, String), f32>,
}

impl RecommendationEngine {
    /// Create a new recommendation engine
    pub fn new(ai_config: AIConfig) -> AIResult<Self> {
        let config = RecommendationConfig::default();
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            user_profiles: HashMap::new(),
            content_items: HashMap::new(),
            similarity_cache: HashMap::new(),
        })
    }
    
    /// Create a new recommendation engine with custom configuration
    pub fn with_config(config: RecommendationConfig, ai_config: AIConfig) -> AIResult<Self> {
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            user_profiles: HashMap::new(),
            content_items: HashMap::new(),
            similarity_cache: HashMap::new(),
        })
    }
    
    /// Load the recommendation model
    pub fn load_model(&mut self) -> AIResult<()> {
        self.model = Some(AIModel::load(ModelType::Recommendation)?);
        Ok(())
    }
    
    /// Add or update a content item
    pub fn add_content(&mut self, content: ContentItem) -> AIResult<()> {
        // Extract features if not present
        let content_with_features = if content.features.is_none() {
            let features = self.extract_content_features(&content)?;
            ContentItem {
                features: Some(features),
                ..content
            }
        } else {
            content
        };
        
        self.content_items.insert(content_with_features.id.clone(), content_with_features);
        Ok(())
    }
    
    /// Add or update a user profile
    pub fn add_user_profile(&mut self, profile: UserProfile) -> AIResult<()> {
        self.user_profiles.insert(profile.user_id.clone(), profile);
        Ok(())
    }
    
    /// Extract content features
    fn extract_content_features(&self, content: &ContentItem) -> AIResult<Vec<f32>> {
        let mut features = Vec::new();
        
        // Genre features (one-hot encoding for common genres)
        let common_genres = vec![
            "Action", "Comedy", "Drama", "Horror", "Sci-Fi", "Romance", "Thriller",
            "Animation", "Documentary", "Fantasy", "Mystery", "Adventure",
        ];
        
        for genre in &common_genres {
            features.push(if content.genres.contains(&genre.to_string()) { 1.0 } else { 0.0 });
        }
        
        // Year feature (normalized)
        let year_normalized = content.year.map_or(0.5, |y| {
            if y >= 2000 { (y - 2000) as f32 / 25.0 } else { 0.0 }
        });
        features.push(year_normalized);
        
        // Rating feature (normalized)
        let rating_normalized = content.rating.map_or(0.5, |r| r / 10.0);
        features.push(rating_normalized);
        
        // Duration feature (normalized)
        let duration_normalized = content.duration.map_or(0.5, |d| {
            (d as f32 / 180.0).min(1.0)
        });
        features.push(duration_normalized);
        
        Ok(features)
    }
    
    /// Get recommendations for a user
    pub fn get_recommendations(&mut self, user_id: &str) -> AIResult<Vec<Recommendation>> {
        let profile = self.user_profiles.get(user_id)
            .ok_or_else(|| AIError::InvalidInput(format!("User profile not found: {}", user_id)))?;
        
        let mut recommendations = Vec::new();
        
        // Collaborative filtering
        if self.config.enable_collaborative {
            let collab_recs = self.collaborative_filtering(profile)?;
            recommendations.extend(collab_recs);
        }
        
        // Content-based filtering
        if self.config.enable_content_based {
            let content_recs = self.content_based_filtering(profile)?;
            recommendations.extend(content_recs);
        }
        
        // Sort by relevance score
        recommendations.sort_by(|a, b| {
            b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Remove duplicates and limit
        let mut seen = HashSet::new();
        let mut unique_recs = Vec::new();
        
        for rec in recommendations {
            if !seen.contains(&rec.content.id) {
                seen.insert(rec.content.id.clone());
                unique_recs.push(rec);
                
                if unique_recs.len() >= self.config.num_recommendations {
                    break;
                }
            }
        }
        
        Ok(unique_recs)
    }
    
    /// Collaborative filtering
    fn collaborative_filtering(&self, profile: &UserProfile) -> AIResult<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        
        // Find similar users based on watch history
        let similar_users = self.find_similar_users(profile)?;
        
        // Aggregate recommendations from similar users
        let mut content_scores: HashMap<String, (f32, usize)> = HashMap::new();
        
        for (user_id, similarity) in similar_users {
            if let Some(other_profile) = self.user_profiles.get(&user_id) {
                for event in &other_profile.watch_history {
                    // Skip if user already watched
                    if profile.watch_history.iter().any(|e| e.content_id == event.content_id) {
                        continue;
                    }
                    
                    let score = similarity * event.completion;
                    content_scores
                        .entry(event.content_id.clone())
                        .and_modify(|(s, c)| {
                            *s += score;
                            *c += 1;
                        })
                        .or_insert((score, 1));
                }
            }
        }
        
        // Convert to recommendations
        for (content_id, (score, count)) in content_scores {
            if let Some(content) = self.content_items.get(&content_id) {
                let relevance_score = score / count as f32;
                
                if relevance_score >= self.config.min_similarity {
                    recommendations.push(Recommendation {
                        content: content.clone(),
                        relevance_score,
                        reason: RecommendationReason::SimilarUsersLiked { user_count: count },
                        confidence: 0.7,
                        source: RecommendationSource::Collaborative,
                    });
                }
            }
        }
        
        Ok(recommendations)
    }
    
    /// Find similar users
    fn find_similar_users(&self, profile: &UserProfile) -> AIResult<Vec<(String, f32)>> {
        let mut similar_users = Vec::new();
        
        for (user_id, other_profile) in &self.user_profiles {
            if user_id == &profile.user_id {
                continue;
            }
            
            let similarity = self.calculate_user_similarity(profile, other_profile)?;
            similar_users.push((user_id.clone(), similarity));
        }
        
        // Sort by similarity
        similar_users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top similar users
        Ok(similar_users.into_iter().take(20).collect())
    }
    
    /// Calculate user similarity
    fn calculate_user_similarity(&self, profile1: &UserProfile, profile2: &UserProfile) -> AIResult<f32> {
        // Jaccard similarity on watch history
        let set1: HashSet<_> = profile1.watch_history.iter().map(|e| &e.content_id).collect();
        let set2: HashSet<_> = profile2.watch_history.iter().map(|e| &e.content_id).collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 {
            return Ok(0.0);
        }
        
        let jaccard = intersection as f32 / union as f32;
        
        // Genre preference similarity
        let genre_sim = self.calculate_genre_similarity(&profile1.genre_preferences, &profile2.genre_preferences)?;
        
        // Combined similarity
        Ok(0.7 * jaccard + 0.3 * genre_sim)
    }
    
    /// Calculate genre preference similarity
    fn calculate_genre_similarity(
        &self,
        prefs1: &HashMap<String, f32>,
        prefs2: &HashMap<String, f32>,
    ) -> AIResult<f32> {
        let mut similarity = 0.0;
        let mut count = 0;
        
        for (genre, pref1) in prefs1 {
            if let Some(pref2) = prefs2.get(genre) {
                similarity += (pref1 - pref2).abs();
                count += 1;
            }
        }
        
        if count > 0 {
            Ok(1.0 - (similarity / count as f32))
        } else {
            Ok(0.0)
        }
    }
    
    /// Content-based filtering
    fn content_based_filtering(&self, profile: &UserProfile) -> AIResult<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        
        // Get recently watched content
        let recent_watched: Vec<_> = profile
            .watch_history
            .iter()
            .filter(|e| e.completion > 0.5) // Only completed or mostly completed
            .take(10)
            .collect();
        
        for event in recent_watched {
            if let Some(watched_content) = self.content_items.get(&event.content_id) {
                // Find similar content
                for (content_id, content) in &self.content_items {
                    // Skip if already watched
                    if profile.watch_history.iter().any(|e| e.content_id == *content_id) {
                        continue;
                    }
                    
                    let similarity = self.calculate_content_similarity(watched_content, content)?;
                    
                    if similarity >= self.config.min_similarity {
                        recommendations.push(Recommendation {
                            content: content.clone(),
                            relevance_score: similarity * event.completion,
                            reason: RecommendationReason::SimilarToWatched {
                                content_id: event.content_id.clone(),
                                similarity,
                            },
                            confidence: 0.8,
                            source: RecommendationSource::ContentBased,
                        });
                    }
                }
            }
        }
        
        Ok(recommendations)
    }
    
    /// Calculate content similarity
    fn calculate_content_similarity(&self, content1: &ContentItem, content2: &ContentItem) -> AIResult<f32> {
        // Check cache
        let cache_key = (content1.id.clone(), content2.id.clone());
        if let Some(&similarity) = self.similarity_cache.get(&cache_key) {
            return Ok(similarity);
        }
        
        let mut similarity = 0.0;
        let mut weight_sum = 0.0;
        
        // Genre similarity (weight: 0.4)
        let genre_sim = self.calculate_genre_overlap(&content1.genres, &content2.genres);
        similarity += 0.4 * genre_sim;
        weight_sum += 0.4;
        
        // Feature similarity (weight: 0.4)
        if let (Some(f1), Some(f2)) = (&content1.features, &content2.features) {
            let feature_sim = self.cosine_similarity(f1, f2);
            similarity += 0.4 * feature_sim;
            weight_sum += 0.4;
        }
        
        // Rating similarity (weight: 0.2)
        if let (Some(r1), Some(r2)) = (content1.rating, content2.rating) {
            let rating_sim = 1.0 - (r1 - r2).abs() / 10.0;
            similarity += 0.2 * rating_sim;
            weight_sum += 0.2;
        }
        
        // Normalize
        let normalized_similarity = if weight_sum > 0.0 {
            similarity / weight_sum
        } else {
            0.0
        };
        
        // Cache result
        self.similarity_cache.insert(cache_key, normalized_similarity);
        
        Ok(normalized_similarity)
    }
    
    /// Calculate genre overlap
    fn calculate_genre_overlap(&self, genres1: &[String], genres2: &[String]) -> f32 {
        if genres1.is_empty() || genres2.is_empty() {
            return 0.0;
        }
        
        let set1: HashSet<_> = genres1.iter().collect();
        let set2: HashSet<_> = genres2.iter().collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
    
    /// Cosine similarity
    fn cosine_similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;
        
        for (a, b) in vec1.iter().zip(vec2.iter()) {
            dot_product += a * b;
            norm1 += a * a;
            norm2 += b * b;
        }
        
        let norm1 = norm1.sqrt();
        let norm2 = norm2.sqrt();
        
        if norm1 > 0.0 && norm2 > 0.0 {
            dot_product / (norm1 * norm2)
        } else {
            0.0
        }
    }
    
    /// Update user profile with watch event
    pub fn add_watch_event(&mut self, user_id: &str, event: WatchEvent) -> AIResult<()> {
        let profile = self.user_profiles.get_mut(user_id)
            .ok_or_else(|| AIError::InvalidInput(format!("User profile not found: {}", user_id)))?;
        
        profile.watch_history.push(event.clone());
        profile.total_watch_time += event.duration_secs / 60; // Convert to minutes
        
        // Update genre preferences
        if let Some(content) = self.content_items.get(&event.content_id) {
            for genre in &content.genres {
                let weight = event.completion;
                *profile.genre_preferences.entry(genre.clone()).or_insert(0.0) += weight;
            }
        }
        
        // Update average rating
        if let Some(rating) = event.rating {
            let total_rating = profile.avg_rating * (profile.watch_history.len() - 1) as f32;
            profile.avg_rating = (total_rating + rating) / profile.watch_history.len() as f32;
        }
        
        Ok(())
    }
    
    /// Get trending content
    pub fn get_trending(&self, limit: usize) -> AIResult<Vec<Recommendation>> {
        let mut trending = Vec::new();
        
        // Sort by watch count across all users
        let mut watch_counts: HashMap<String, usize> = HashMap::new();
        
        for profile in self.user_profiles.values() {
            for event in &profile.watch_history {
                *watch_counts.entry(event.content_id.clone()).or_insert(0) += 1;
            }
        }
        
        let mut sorted: Vec<_> = watch_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (content_id, count) in sorted.into_iter().take(limit) {
            if let Some(content) = self.content_items.get(&content_id) {
                trending.push(Recommendation {
                    content: content.clone(),
                    relevance_score: (count as f32 / self.user_profiles.len() as f32).min(1.0),
                    reason: RecommendationReason::Trending,
                    confidence: 0.9,
                    source: RecommendationSource::Popular,
                });
            }
        }
        
        Ok(trending)
    }
    
    /// Clear similarity cache
    pub fn clear_cache(&mut self) {
        self.similarity_cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recommendation_config_default() {
        let config = RecommendationConfig::default();
        assert!(config.enable_collaborative);
        assert_eq!(config.num_recommendations, 10);
    }
    
    #[test]
    fn test_cosine_similarity() {
        let engine = RecommendationEngine::new(AIConfig::default()).unwrap();
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let sim = engine.cosine_similarity(&vec1, &vec2);
        assert!((sim - 1.0).abs() < 0.001);
    }
}