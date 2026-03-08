//! Recommendation engine with multiple strategies

use crate::error::{RecommendationError, Result};
use crate::types::*;
use crate::content_analyzer::{ContentAnalyzer, ContentFeatures};
use crate::user_profile::{UserProfileBuilder, PreferenceBuilder};
use crate::similarity::{SimilarityCalculator, SimilarityMetric};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use lru::LruCache;
use dashmap::DashMap;
use std::num::NonZeroUsize;

/// Recommendation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationStrategy {
    /// Content-based filtering
    ContentBased,
    /// Collaborative filtering
    Collaborative,
    /// Hybrid approach
    Hybrid,
    /// Popular/trending
    Popular,
    /// Personalized popular
    PersonalizedPopular,
}

impl Default for RecommendationStrategy {
    fn default() -> Self {
        Self::Hybrid
    }
}

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Maximum recommendations to return
    pub max_recommendations: usize,
    /// Minimum similarity threshold
    pub min_similarity: f64,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Weight for content-based recommendations
    pub content_weight: f64,
    /// Weight for collaborative recommendations
    pub collaborative_weight: f64,
    /// Weight for popularity
    pub popularity_weight: f64,
    /// Cache size for recommendations
    pub cache_size: usize,
    /// Enable diversity in results
    pub enable_diversity: bool,
    /// Diversity factor (0.0 - 1.0)
    pub diversity_factor: f64,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_recommendations: 50,
            min_similarity: 0.1,
            min_confidence: 0.3,
            content_weight: 0.4,
            collaborative_weight: 0.4,
            popularity_weight: 0.2,
            cache_size: 1000,
            enable_diversity: true,
            diversity_factor: 0.3,
        }
    }
}

impl EngineConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_recommendations == 0 {
            return Err(RecommendationError::InvalidConfiguration(
                "max_recommendations must be > 0".to_string(),
            ));
        }
        if self.content_weight + self.collaborative_weight + self.popularity_weight == 0.0 {
            return Err(RecommendationError::InvalidConfiguration(
                "at least one weight must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Main recommendation engine
pub struct RecommendationEngine {
    /// Configuration
    config: EngineConfig,
    /// Content catalog
    catalog: Arc<RwLock<HashMap<ContentId, ContentMetadata>>>,
    /// User profiles
    user_profiles: Arc<DashMap<UserId, UserProfile>>,
    /// Content features cache
    content_features: Arc<DashMap<ContentId, ContentFeatures>>,
    /// Similarity calculator
    similarity_calculator: SimilarityCalculator,
    /// Recommendation cache
    cache: Arc<RwLock<LruCache<UserId, RecommendationSet>>>,
    /// Content analyzer
    content_analyzer: ContentAnalyzer,
}

impl RecommendationEngine {
    /// Create a new recommendation engine
    pub fn new(config: EngineConfig) -> Self {
        let cache_size = NonZeroUsize::new(config.cache_size).unwrap_or(NonZeroUsize::new(100).unwrap());
        
        Self {
            config,
            catalog: Arc::new(RwLock::new(HashMap::new())),
            user_profiles: Arc::new(DashMap::new()),
            content_features: Arc::new(DashMap::new()),
            similarity_calculator: SimilarityCalculator::new(),
            cache: Arc::new(RwLock::new(LruCache::new(cache_size))),
            content_analyzer: ContentAnalyzer::new(),
        }
    }

    /// Add content to the catalog
    pub async fn add_content(&self, content: ContentMetadata) {
        let id = content.id.clone();
        let features = self.content_analyzer.analyze(&content);
        
        self.content_features.insert(id.clone(), features);
        self.catalog.write().await.insert(id, content);
    }

    /// Add multiple content items
    pub async fn add_content_batch(&self, items: Vec<ContentMetadata>) {
        for content in items {
            self.add_content(content).await;
        }
    }

    /// Record a watch event
    pub async fn record_watch(&self, user_id: &UserId, event: WatchEvent) -> Result<()> {
        let mut profile = self.get_or_create_profile(user_id).await;
        profile.watch_history.add(event);
        profile.update_from_history();
        
        // Invalidate cache for this user
        self.cache.write().await.pop(user_id);
        
        Ok(())
    }

    /// Get or create user profile
    async fn get_or_create_profile(&self, user_id: &UserId) -> UserProfile {
        if let Some(profile) = self.user_profiles.get(user_id) {
            return profile.clone();
        }
        
        let profile = UserProfile::new(user_id.clone());
        self.user_profiles.insert(user_id.clone(), profile.clone());
        profile
    }

    /// Generate recommendations for a user
    pub async fn recommend(&self, user_id: &UserId, count: usize) -> Result<RecommendationSet> {
        let start = Instant::now();
        
        // Check cache
        {
            let mut cache = self.cache.write().await;
            if let Some(cached) = cache.get(user_id) {
                if cached.recommendations.len() >= count {
                    let mut result = cached.clone();
                    result.recommendations.truncate(count);
                    return Ok(result);
                }
            }
        }

        let profile = self.get_or_create_profile(user_id).await;
        let catalog = self.catalog.read().await;
        
        if catalog.is_empty() {
            return Err(RecommendationError::InsufficientData(
                "Content catalog is empty".to_string(),
            ));
        }

        // Get watched content
        let watched: HashSet<&ContentId> = profile.watch_history.watched_content_ids();
        
        // Generate recommendations using different strategies
        let mut recommendations = Vec::new();
        
        // Content-based recommendations
        let content_based = self.content_based_recommendations(&profile, &watched, &catalog).await?;
        recommendations.extend(content_based);
        
        // Collaborative filtering recommendations
        let collaborative = self.collaborative_recommendations(&profile, &watched, &catalog).await?;
        recommendations.extend(collaborative);
        
        // Popular recommendations
        let popular = self.popular_recommendations(&watched, &catalog)?;
        recommendations.extend(popular);
        
        // Combine and rank
        recommendations = self.combine_recommendations(recommendations);
        
        // Apply diversity if enabled
        if self.config.enable_diversity {
            recommendations = self.apply_diversity(recommendations);
        }
        
        // Remove duplicates and sort
        recommendations.sort();
        recommendations.dedup();
        
        // Truncate to requested count
        recommendations.truncate(count);
        
        let mut result = RecommendationSet::new(user_id.clone());
        result.recommendations = recommendations;
        result.processing_time_ms = start.elapsed().as_millis() as u64;
        
        // Cache result
        {
            let mut cache = self.cache.write().await;
            cache.put(user_id.clone(), result.clone());
        }
        
        Ok(result)
    }

    /// Content-based recommendations
    async fn content_based_recommendations(
        &self,
        profile: &UserProfile,
        watched: &HashSet<&ContentId>,
        catalog: &HashMap<ContentId, ContentMetadata>,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        
        // Get liked content features
        let mut liked_features: Vec<ContentFeatures> = Vec::new();
        for event in &profile.watch_history.events {
            if event.liked == Some(true) || event.engagement_score() > 0.7 {
                if let Some(features) = self.content_features.get(&event.content_id) {
                    liked_features.push(features.clone());
                }
            }
        }
        
        if liked_features.is_empty() {
            // Fall back to preferences
            return self.preference_based_recommendations(profile, watched, catalog);
        }
        
        // Calculate average feature vector from liked content
        let avg_features = self.average_features(&liked_features);
        
        // Find similar content
        for (id, content) in catalog {
            if watched.contains(id) {
                continue;
            }
            
            if !profile.preferences.matches(content) {
                continue;
            }
            
            if let Some(features) = self.content_features.get(id) {
                let similarity = avg_features.cosine_similarity(features);
                
                if similarity >= self.config.min_similarity {
                    let recommendation = Recommendation::new(
                        id.clone(),
                        similarity * self.config.content_weight,
                        RecommendationReason::SimilarToLiked,
                    )
                    .with_confidence(similarity)
                    .with_strategy("content_based");
                    
                    recommendations.push(recommendation);
                }
            }
        }
        
        Ok(recommendations)
    }

    /// Preference-based recommendations (fallback)
    fn preference_based_recommendations(
        &self,
        profile: &UserProfile,
        watched: &HashSet<&ContentId>,
        catalog: &HashMap<ContentId, ContentMetadata>,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        
        for (id, content) in catalog {
            if watched.contains(id) {
                continue;
            }
            
            if !profile.preferences.matches(content) {
                continue;
            }
            
            // Calculate preference match score
            let mut score = 0.0;
            
            // Genre match
            for genre in &content.genres {
                if let Some(weight) = profile.preferences.genre_weights.get(genre.as_str()) {
                    score += weight * 0.4;
                }
            }
            
            // Content type match
            let type_str = format!("{:?}", content.content_type);
            if let Some(weight) = profile.preferences.content_type_weights.get(&type_str) {
                score += weight * 0.2;
            }
            
            // Rating bonus
            score += (content.rating / 10.0) * 0.2;
            
            // Popularity bonus
            score += content.popularity * 0.2;
            
            if score >= self.config.min_similarity {
                let recommendation = Recommendation::new(
                    id.clone(),
                    score.min(1.0),
                    RecommendationReason::SimilarToWatched,
                )
                .with_confidence(score * 0.8)
                .with_strategy("preference_based");
                
                recommendations.push(recommendation);
            }
        }
        
        Ok(recommendations)
    }

    /// Collaborative filtering recommendations
    async fn collaborative_recommendations(
        &self,
        profile: &UserProfile,
        watched: &HashSet<&ContentId>,
        catalog: &HashMap<ContentId, ContentMetadata>,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        let mut content_scores: HashMap<ContentId, (f64, usize)> = HashMap::new();
        
        // Find similar users
        let similar_users = self.find_similar_users(profile).await;
        
        for (similar_user_id, similarity) in similar_users {
            if similarity < self.config.min_similarity {
                continue;
            }
            
            if let Some(similar_profile) = self.user_profiles.get(&similar_user_id) {
                // Get highly-rated content from similar user
                for event in &similar_profile.watch_history.events {
                    if event.engagement_score() < 0.5 {
                        continue;
                    }
                    
                    if watched.contains(&event.content_id) {
                        continue;
                    }
                    
                    let entry = content_scores.entry(event.content_id.clone()).or_default();
                    entry.0 += similarity * event.engagement_score();
                    entry.1 += 1;
                }
            }
        }
        
        // Convert to recommendations
        for (content_id, (total_score, count)) in content_scores {
            if let Some(content) = catalog.get(&content_id) {
                if !profile.preferences.matches(content) {
                    continue;
                }
                
                let avg_score = total_score / count as f64;
                let final_score = avg_score * self.config.collaborative_weight;
                
                let recommendation = Recommendation::new(
                    content_id,
                    final_score,
                    RecommendationReason::PopularAmongSimilarUsers,
                )
                .with_confidence(avg_score)
                .with_strategy("collaborative");
                
                recommendations.push(recommendation);
            }
        }
        
        Ok(recommendations)
    }

    /// Popular recommendations
    fn popular_recommendations(
        &self,
        watched: &HashSet<&ContentId>,
        catalog: &HashMap<ContentId, ContentMetadata>,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();
        
        // Sort by popularity and rating
        let mut sorted: Vec<_> = catalog.iter().collect();
        sorted.sort_by(|a, b| {
            let score_a = a.1.popularity * 0.6 + (a.1.rating / 10.0) * 0.4;
            let score_b = b.1.popularity * 0.6 + (b.1.rating / 10.0) * 0.4;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        for (id, content) in sorted.iter().take(self.config.max_recommendations) {
            if watched.contains(*id) {
                continue;
            }
            
            let score = (content.popularity * 0.6 + (content.rating / 10.0) * 0.4)
                * self.config.popularity_weight;
            
            let recommendation = Recommendation::new(
                (*id).clone(),
                score,
                RecommendationReason::Trending,
            )
            .with_confidence(0.5)
            .with_strategy("popular");
            
            recommendations.push(recommendation);
        }
        
        Ok(recommendations)
    }

    /// Find similar users
    async fn find_similar_users(&self, profile: &UserProfile) -> Vec<(UserId, f64)> {
        let mut similar_users = Vec::new();
        
        // Get user's watched content
        let user_content: HashSet<&ContentId> = profile.watch_history.watched_content_ids();
        
        if user_content.is_empty() {
            return similar_users;
        }
        
        // Compare with other users
        for entry in self.user_profiles.iter() {
            let other_id = entry.key().clone();
            if other_id == profile.user_id {
                continue;
            }
            
            let other_content: HashSet<&ContentId> = entry.watch_history.watched_content_ids();
            
            // Jaccard similarity
            let intersection = user_content.intersection(&other_content).count();
            let union = user_content.union(&other_content).count();
            
            if union > 0 {
                let similarity = intersection as f64 / union as f64;
                
                if similarity >= self.config.min_similarity {
                    similar_users.push((other_id, similarity));
                }
            }
        }
        
        // Sort by similarity
        similar_users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similar_users.truncate(10);
        
        similar_users
    }

    /// Combine recommendations from different strategies
    fn combine_recommendations(&self, mut recommendations: Vec<Recommendation>) -> Vec<Recommendation> {
        // Group by content ID and combine scores
        let mut combined: HashMap<ContentId, Recommendation> = HashMap::new();
        
        for rec in recommendations {
            let entry = combined.entry(rec.content_id.clone()).or_insert_with(|| rec.clone());
            entry.score += rec.score * 0.5; // Weight additional signals
            entry.confidence = (entry.confidence + rec.confidence) / 2.0;
        }
        
        combined.into_values().collect()
    }

    /// Apply diversity to recommendations
    fn apply_diversity(&self, mut recommendations: Vec<Recommendation>) -> Vec<Recommendation> {
        if recommendations.len() <= 5 {
            return recommendations;
        }
        
        // Re-order to ensure genre diversity
        let mut diverse = Vec::new();
        let mut seen_genres: HashSet<String> = HashSet::new();
        let mut remaining = recommendations.clone();
        
        // First pass: add one of each genre
        remaining.retain(|rec| {
            if diverse.len() >= self.config.max_recommendations {
                return false;
            }
            
            // Check if this adds diversity
            let genre_key = format!("{:?}", rec.content_id);
            if !seen_genres.contains(&genre_key) {
                seen_genres.insert(genre_key);
                diverse.push(rec.clone());
                false
            } else {
                true
            }
        });
        
        // Second pass: add remaining
        for rec in remaining {
            if diverse.len() >= self.config.max_recommendations {
                break;
            }
            diverse.push(rec);
        }
        
        diverse
    }

    /// Average multiple feature sets
    fn average_features(&self, features: &[ContentFeatures]) -> ContentFeatures {
        if features.is_empty() {
            return ContentFeatures::default();
        }
        
        // Simple averaging of the vector components
        let mut sum = vec![0.0; features[0].vector.len()];
        for f in features {
            for (i, val) in f.vector.iter().enumerate() {
                sum[i] += val;
            }
        }
        
        let avg: Vec<f64> = sum.iter().map(|x| x / features.len() as f64).collect();
        
        ContentFeatures {
            vector: avg,
            genres: features[0].genres.clone(),
            content_type: features[0].content_type,
        }
    }

    /// Get similar content
    pub async fn get_similar(&self, content_id: &ContentId, count: usize) -> Result<Vec<Recommendation>> {
        let catalog = self.catalog.read().await;
        
        let features = self
            .content_features
            .get(content_id)
            .ok_or_else(|| RecommendationError::ContentNotFound(content_id.to_string()))?
            .clone();
        
        let mut similar = Vec::new();
        
        for (id, _) in catalog.iter() {
            if id == content_id {
                continue;
            }
            
            if let Some(other_features) = self.content_features.get(id) {
                let similarity = features.cosine_similarity(&other_features);
                
                if similarity >= self.config.min_similarity {
                    let recommendation = Recommendation::new(
                        id.clone(),
                        similarity,
                        RecommendationReason::SimilarToWatched,
                    )
                    .with_confidence(similarity);
                    
                    similar.push(recommendation);
                }
            }
        }
        
        similar.sort();
        similar.truncate(count);
        
        Ok(similar)
    }

    /// Get statistics
    pub async fn stats(&self) -> EngineStats {
        EngineStats {
            content_count: self.catalog.read().await.len(),
            user_count: self.user_profiles.len(),
            cache_size: self.cache.read().await.len(),
        }
    }
}

/// Engine statistics
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub content_count: usize,
    pub user_count: usize,
    pub cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let config = EngineConfig::default();
        let engine = RecommendationEngine::new(config);
        let stats = engine.stats().await;
        assert_eq!(stats.content_count, 0);
    }

    #[tokio::test]
    async fn test_add_content() {
        let engine = RecommendationEngine::new(Default::default());
        let content = ContentMetadata::new(
            ContentId::new("movie-1"),
            "Test Movie".to_string(),
            ContentType::Movie,
        );
        
        engine.add_content(content).await;
        let stats = engine.stats().await;
        assert_eq!(stats.content_count, 1);
    }

    #[tokio::test]
    async fn test_recommend_no_history() {
        let engine = RecommendationEngine::new(Default::default());
        
        // Add some content
        for i in 0..10 {
            let content = ContentMetadata::new(
                ContentId::new(format!("movie-{}", i)),
                format!("Movie {}", i),
                ContentType::Movie,
            ).with_rating(7.0 + i as f64 * 0.1)
             .with_genre("Action");
            
            engine.add_content(content).await;
        }
        
        let user_id = UserId::new("user-1");
        let result = engine.recommend(&user_id, 5).await;
        
        assert!(result.is_ok());
        let recs = result.unwrap();
        assert!(recs.recommendations.len() <= 5);
    }

    #[test]
    fn test_config_validation() {
        let valid_config = EngineConfig::default();
        assert!(valid_config.validate().is_ok());
        
        let invalid_config = EngineConfig {
            max_recommendations: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}