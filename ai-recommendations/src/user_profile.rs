//! User profile management and preference learning

use crate::error::{RecommendationError, Result};
use crate::types::*;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc, Duration};

/// User profile builder for fluent construction
pub struct UserProfileBuilder {
    profile: UserProfile,
}

impl UserProfileBuilder {
    /// Create a new builder
    pub fn new(user_id: UserId) -> Self {
        Self {
            profile: UserProfile::new(user_id),
        }
    }

    /// Set display name
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.profile.display_name = name.into();
        self
    }

    /// Add a watched content
    pub fn watched(mut self, content_id: ContentId, engagement: f64) -> Self {
        let mut event = WatchEvent::new(content_id);
        event.completion_ratio = engagement;
        event.liked = Some(engagement > 0.7);
        self.profile.watch_history.add(event);
        self
    }

    /// Add genre preference
    pub fn prefer_genre(mut self, genre: impl Into<String>, weight: f64) -> Self {
        self.profile
            .preferences
            .genre_weights
            .insert(genre.into(), weight);
        self
    }

    /// Add content type preference
    pub fn prefer_content_type(mut self, content_type: ContentType, weight: f64) -> Self {
        self.profile
            .preferences
            .content_type_weights
            .insert(format!("{:?}", content_type), weight);
        self
    }

    /// Exclude a tag
    pub fn exclude_tag(mut self, tag: impl Into<String>) -> Self {
        self.profile.preferences.excluded_tags.insert(tag.into());
        self
    }

    /// Build the profile
    pub fn build(self) -> UserProfile {
        let mut profile = self.profile;
        profile.update_from_history();
        profile.updated_at = Utc::now();
        profile
    }
}

/// Preference builder for constructing user preferences
pub struct PreferenceBuilder {
    preferences: UserPreferences,
}

impl PreferenceBuilder {
    /// Create a new preference builder
    pub fn new() -> Self {
        Self {
            preferences: UserPreferences::new(),
        }
    }

    /// Add genre preference
    pub fn genre(mut self, genre: impl Into<String>, weight: f64) -> Self {
        self.preferences
            .genre_weights
            .insert(genre.into(), weight.clamp(0.0, 1.0));
        self
    }

    /// Add actor preference
    pub fn actor(mut self, actor: impl Into<String>, weight: f64) -> Self {
        self.preferences
            .actor_weights
            .insert(actor.into(), weight.clamp(0.0, 1.0));
        self
    }

    /// Add director preference
    pub fn director(mut self, director: impl Into<String>, weight: f64) -> Self {
        self.preferences
            .director_weights
            .insert(director.into(), weight.clamp(0.0, 1.0));
        self
    }

    /// Set language preference
    pub fn language(mut self, lang: impl Into<String>, weight: f64) -> Self {
        self.preferences
            .language_weights
            .insert(lang.into(), weight.clamp(0.0, 1.0));
        self
    }

    /// Set minimum rating preference
    pub fn min_rating(mut self, rating: f64) -> Self {
        self.preferences.min_rating_preference = Some(rating.clamp(0.0, 10.0));
        self
    }

    /// Set duration preference
    pub fn duration(mut self, min_minutes: u32, max_minutes: u32) -> Self {
        self.preferences.duration_preference = Some((min_minutes, max_minutes));
        self
    }

    /// Exclude a tag
    pub fn exclude(mut self, tag: impl Into<String>) -> Self {
        self.preferences.excluded_tags.insert(tag.into());
        self
    }

    /// Build preferences
    pub fn build(self) -> UserPreferences {
        self.preferences
    }
}

impl Default for PreferenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// User profile manager
pub struct UserProfileManager {
    /// User profiles
    profiles: HashMap<UserId, UserProfile>,
    /// Minimum events for learning
    min_events_for_learning: usize,
    /// Learning rate
    learning_rate: f64,
}

impl UserProfileManager {
    /// Create a new profile manager
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            min_events_for_learning: 5,
            learning_rate: 0.1,
        }
    }

    /// Get or create a profile
    pub fn get_or_create(&mut self, user_id: &UserId) -> &mut UserProfile {
        self.profiles
            .entry(user_id.clone())
            .or_insert_with(|| UserProfile::new(user_id.clone()))
    }

    /// Record a watch event and update profile
    pub fn record_watch(&mut self, user_id: &UserId, event: WatchEvent, content: &ContentMetadata) {
        let profile = self.get_or_create(user_id);
        profile.watch_history.add(event.clone());
        
        // Update preferences based on engagement
        let engagement = event.engagement_score();
        if engagement > 0.5 {
            self.update_preferences(profile, content, engagement);
        }
        
        profile.update_from_history();
    }

    /// Update preferences based on content engagement
    fn update_preferences(&self, profile: &mut UserProfile, content: &ContentMetadata, engagement: f64) {
        let weight = engagement * self.learning_rate;
        
        // Update genre preferences
        for genre in &content.genres {
            let entry = profile
                .preferences
                .genre_weights
                .entry(genre.as_str().to_string())
                .or_insert(0.0);
            *entry = (*entry + weight).min(1.0);
        }
        
        // Update actor preferences
        for actor in &content.cast {
            let entry = profile
                .preferences
                .actor_weights
                .entry(actor.clone())
                .or_insert(0.0);
            *entry = (*entry + weight * 0.5).min(1.0);
        }
        
        // Update director preferences
        for director in &content.directors {
            let entry = profile
                .preferences
                .director_weights
                .entry(director.clone())
                .or_insert(0.0);
            *entry = (*entry + weight * 0.5).min(1.0);
        }
        
        // Update content type preference
        let type_key = format!("{:?}", content.content_type);
        let entry = profile
            .preferences
            .content_type_weights
            .entry(type_key)
            .or_insert(0.0);
        *entry = (*entry + weight).min(1.0);
    }

    /// Learn preferences from history
    pub fn learn_from_history(&mut self, user_id: &UserId, contents: &HashMap<ContentId, ContentMetadata>) {
        let profile = match self.profiles.get_mut(user_id) {
            Some(p) => p,
            None => return,
        };
        
        if profile.watch_history.events.len() < self.min_events_for_learning {
            return;
        }
        
        // Aggregate engagement by genre, actor, etc.
        let mut genre_engagement: HashMap<String, (f64, usize)> = HashMap::new();
        let mut actor_engagement: HashMap<String, (f64, usize)> = HashMap::new();
        let mut director_engagement: HashMap<String, (f64, usize)> = HashMap::new();
        
        for event in &profile.watch_history.events {
            if let Some(content) = contents.get(&event.content_id) {
                let engagement = event.engagement_score();
                
                for genre in &content.genres {
                    let entry = genre_engagement
                        .entry(genre.as_str().to_string())
                        .or_default();
                    entry.0 += engagement;
                    entry.1 += 1;
                }
                
                for actor in &content.cast {
                    let entry = actor_engagement.entry(actor.clone()).or_default();
                    entry.0 += engagement;
                    entry.1 += 1;
                }
                
                for director in &content.directors {
                    let entry = director_engagement.entry(director.clone()).or_default();
                    entry.0 += engagement;
                    entry.1 += 1;
                }
            }
        }
        
        // Update preferences from aggregated data
        for (genre, (total, count)) in genre_engagement {
            let avg = total / count as f64;
            if avg > 0.5 {
                profile
                    .preferences
                    .genre_weights
                    .insert(genre, avg);
            }
        }
        
        for (actor, (total, count)) in actor_engagement {
            let avg = total / count as f64;
            if avg > 0.5 {
                profile.preferences.actor_weights.insert(actor, avg);
            }
        }
        
        for (director, (total, count)) in director_engagement {
            let avg = total / count as f64;
            if avg > 0.5 {
                profile.preferences.director_weights.insert(director, avg);
            }
        }
        
        profile.updated_at = Utc::now();
    }

    /// Get profile statistics
    pub fn profile_stats(&self, user_id: &UserId) -> Option<ProfileStats> {
        self.profiles.get(user_id).map(|p| ProfileStats {
            total_watched: p.watch_history.events.len(),
            total_watch_time_seconds: p.watch_history.total_watch_time_seconds(),
            genre_count: p.preferences.genre_weights.len(),
            actor_count: p.preferences.actor_weights.len(),
            last_updated: p.updated_at,
        })
    }

    /// Get similar users
    pub fn find_similar_users(&self, user_id: &UserId, min_similarity: f64) -> Vec<(UserId, f64)> {
        let profile = match self.profiles.get(user_id) {
            Some(p) => p,
            None => return Vec::new(),
        };
        
        let user_genres: HashSet<&str> = profile
            .preferences
            .genre_weights
            .keys()
            .map(|s| s.as_str())
            .collect();
        
        let user_actors: HashSet<&str> = profile
            .preferences
            .actor_weights
            .keys()
            .map(|s| s.as_str())
            .collect();
        
        let mut similar = Vec::new();
        
        for (other_id, other_profile) in &self.profiles {
            if other_id == user_id {
                continue;
            }
            
            let other_genres: HashSet<&str> = other_profile
                .preferences
                .genre_weights
                .keys()
                .map(|s| s.as_str())
                .collect();
            
            let other_actors: HashSet<&str> = other_profile
                .preferences
                .actor_weights
                .keys()
                .map(|s| s.as_str())
                .collect();
            
            // Jaccard similarity for genres
            let genre_intersection = user_genres.intersection(&other_genres).count();
            let genre_union = user_genres.union(&other_genres).count();
            let genre_sim = if genre_union > 0 {
                genre_intersection as f64 / genre_union as f64
            } else {
                0.0
            };
            
            // Jaccard similarity for actors
            let actor_intersection = user_actors.intersection(&other_actors).count();
            let actor_union = user_actors.union(&other_actors).count();
            let actor_sim = if actor_union > 0 {
                actor_intersection as f64 / actor_union as f64
            } else {
                0.0
            };
            
            // Combined similarity
            let similarity = genre_sim * 0.6 + actor_sim * 0.4;
            
            if similarity >= min_similarity {
                similar.push((other_id.clone(), similarity));
            }
        }
        
        similar.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similar
    }

    /// Prune old watch events
    pub fn prune_old_events(&mut self, days: i64) {
        let cutoff = Utc::now() - Duration::days(days);
        
        for profile in self.profiles.values_mut() {
            profile
                .watch_history
                .events
                .retain(|e| e.started_at > cutoff);
            profile.watch_history.updated_at = Utc::now();
        }
    }
}

impl Default for UserProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Profile statistics
#[derive(Debug, Clone)]
pub struct ProfileStats {
    pub total_watched: usize,
    pub total_watch_time_seconds: i64,
    pub genre_count: usize,
    pub actor_count: usize,
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_builder() {
        let profile = UserProfileBuilder::new(UserId::new("user-1"))
            .display_name("Test User")
            .prefer_genre("Action", 0.8)
            .prefer_genre("Comedy", 0.5)
            .watched(ContentId::new("movie-1"), 0.9)
            .exclude_tag("horror")
            .build();

        assert_eq!(profile.display_name, "Test User");
        assert_eq!(profile.preferences.genre_weights.get("Action"), Some(&0.8));
        assert!(profile.preferences.excluded_tags.contains("horror"));
    }

    #[test]
    fn test_preference_builder() {
        let prefs = PreferenceBuilder::new()
            .genre("Action", 0.9)
            .actor("Tom Hanks", 0.8)
            .director("Spielberg", 0.7)
            .language("en", 1.0)
            .min_rating(6.0)
            .exclude("violence")
            .build();

        assert_eq!(prefs.genre_weights.get("Action"), Some(&0.9));
        assert_eq!(prefs.actor_weights.get("Tom Hanks"), Some(&0.8));
        assert!(prefs.excluded_tags.contains("violence"));
    }

    #[test]
    fn test_profile_manager() {
        let mut manager = UserProfileManager::new();
        let user_id = UserId::new("user-1");
        
        let content = ContentMetadata::new(
            ContentId::new("movie-1"),
            "Test Movie".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_cast(vec!["Actor A".to_string()]);
        
        let mut event = WatchEvent::new(ContentId::new("movie-1"));
        event.completion_ratio = 0.9;
        event.liked = Some(true);
        
        manager.record_watch(&user_id, event, &content);
        
        let stats = manager.profile_stats(&user_id).unwrap();
        assert_eq!(stats.total_watched, 1);
    }

    #[test]
    fn test_find_similar_users() {
        let mut manager = UserProfileManager::new();
        
        // Create user 1 with Action preference
        let user1 = UserId::new("user-1");
        let profile1 = UserProfileBuilder::new(user1.clone())
            .prefer_genre("Action", 0.9)
            .prefer_genre("Thriller", 0.7)
            .build();
        manager.profiles.insert(user1.clone(), profile1);
        
        // Create user 2 with similar preferences
        let user2 = UserId::new("user-2");
        let profile2 = UserProfileBuilder::new(user2.clone())
            .prefer_genre("Action", 0.8)
            .prefer_genre("Thriller", 0.9)
            .build();
        manager.profiles.insert(user2.clone(), profile2);
        
        // Create user 3 with different preferences
        let user3 = UserId::new("user-3");
        let profile3 = UserProfileBuilder::new(user3.clone())
            .prefer_genre("Romance", 0.9)
            .prefer_genre("Comedy", 0.8)
            .build();
        manager.profiles.insert(user3, profile3);
        
        let similar = manager.find_similar_users(&user1, 0.3);
        assert!(!similar.is_empty());
        assert_eq!(similar[0].0, user2);
    }
}