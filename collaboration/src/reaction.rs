//! Reaction System for Collaborative Viewing
//! 
//! Provides real-time reactions and emoji responses during viewing sessions.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::{ParticipantId, SessionId};

/// Reaction in a viewing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// Unique reaction ID
    pub id: String,
    
    /// Session ID
    pub session_id: SessionId,
    
    /// Participant who reacted
    pub participant_id: ParticipantId,
    
    /// Reaction type
    pub reaction_type: ReactionType,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Position in content when reacted (seconds)
    pub position_seconds: Option<f64>,
    
    /// Custom text (for text reactions)
    pub custom_text: Option<String>,
}

/// Type of reaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReactionType {
    /// Standard emoji
    Emoji(String),
    /// Predefined reaction
    Predefined(StandardReaction),
    /// Animated GIF reaction
    Animated { 
        url: String,
        name: String,
    },
    /// Sound reaction
    Sound {
        sound_id: String,
        volume: f32,
    },
}

/// Standard/predefined reactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StandardReaction {
    // Positive
    Like,
    Love,
    Wow,
    Haha,
    Clap,
    Fire,
    
    // Negative
    Sad,
    Angry,
    Dislike,
    
    // Engagement
    Laugh,
    Cry,
    Shock,
    Think,
    
    // Actions
    Heart,
    Star,
    ThumbsUp,
    ThumbsDown,
    
    // Movie/Show specific
    Popcorn,
    MovieTicket,
    Spoiler,
    Skip,
}

impl StandardReaction {
    /// Get the emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            StandardReaction::Like => "👍",
            StandardReaction::Love => "❤️",
            StandardReaction::Wow => "😮",
            StandardReaction::Haha => "😂",
            StandardReaction::Clap => "👏",
            StandardReaction::Fire => "🔥",
            StandardReaction::Sad => "😢",
            StandardReaction::Angry => "😠",
            StandardReaction::Dislike => "👎",
            StandardReaction::Laugh => "🤣",
            StandardReaction::Cry => "😭",
            StandardReaction::Shock => "😱",
            StandardReaction::Think => "🤔",
            StandardReaction::Heart => "💕",
            StandardReaction::Star => "⭐",
            StandardReaction::ThumbsUp => "👍",
            StandardReaction::ThumbsDown => "👎",
            StandardReaction::Popcorn => "🍿",
            StandardReaction::MovieTicket => "🎫",
            StandardReaction::Spoiler => "🚫",
            StandardReaction::Skip => "⏭️",
        }
    }
    
    /// Get the name
    pub fn name(&self) -> &'static str {
        match self {
            StandardReaction::Like => "Like",
            StandardReaction::Love => "Love",
            StandardReaction::Wow => "Wow",
            StandardReaction::Haha => "Haha",
            StandardReaction::Clap => "Clap",
            StandardReaction::Fire => "Fire",
            StandardReaction::Sad => "Sad",
            StandardReaction::Angry => "Angry",
            StandardReaction::Dislike => "Dislike",
            StandardReaction::Laugh => "Laugh",
            StandardReaction::Cry => "Cry",
            StandardReaction::Shock => "Shock",
            StandardReaction::Think => "Think",
            StandardReaction::Heart => "Heart",
            StandardReaction::Star => "Star",
            StandardReaction::ThumbsUp => "Thumbs Up",
            StandardReaction::ThumbsDown => "Thumbs Down",
            StandardReaction::Popcorn => "Popcorn",
            StandardReaction::MovieTicket => "Movie Ticket",
            StandardReaction::Spoiler => "Spoiler Alert",
            StandardReaction::Skip => "Skip",
        }
    }
    
    /// Get all standard reactions
    pub fn all() -> Vec<StandardReaction> {
        vec![
            StandardReaction::Like,
            StandardReaction::Love,
            StandardReaction::Wow,
            StandardReaction::Haha,
            StandardReaction::Clap,
            StandardReaction::Fire,
            StandardReaction::Sad,
            StandardReaction::Angry,
            StandardReaction::Popcorn,
            StandardReaction::Heart,
        ]
    }
}

impl Reaction {
    /// Create a new reaction
    pub fn new(
        session_id: SessionId,
        participant_id: ParticipantId,
        reaction_type: ReactionType,
    ) -> Self {
        Self {
            id: format!("react_{}", uuid::Uuid::new_v4()),
            session_id,
            participant_id,
            reaction_type,
            timestamp: Utc::now(),
            position_seconds: None,
            custom_text: None,
        }
    }
    
    /// Create an emoji reaction
    pub fn emoji(
        session_id: SessionId,
        participant_id: ParticipantId,
        emoji: String,
    ) -> Self {
        Self::new(session_id, participant_id, ReactionType::Emoji(emoji))
    }
    
    /// Create a standard reaction
    pub fn standard(
        session_id: SessionId,
        participant_id: ParticipantId,
        reaction: StandardReaction,
    ) -> Self {
        Self::new(session_id, participant_id, ReactionType::Predefined(reaction))
    }
    
    /// Set position
    pub fn at_position(mut self, position: f64) -> Self {
        self.position_seconds = Some(position);
        self
    }
    
    /// Set custom text
    pub fn with_text(mut self, text: String) -> Self {
        self.custom_text = Some(text);
        self
    }
}

/// Reaction manager for a session
pub struct ReactionManager {
    /// Recent reactions
    reactions: VecDeque<Reaction>,
    
    /// Maximum reactions to keep
    max_reactions: usize,
    
    /// Rate limit per participant (reactions per minute)
    rate_limit: u32,
    
    /// Reactions enabled
    enabled: bool,
    
    /// Participant reaction counts (for rate limiting)
    participant_counts: HashMap<ParticipantId, (u32, DateTime<Utc>)>,
}

use std::collections::VecDeque;

impl ReactionManager {
    /// Create a new reaction manager
    pub fn new(max_reactions: usize, rate_limit: u32) -> Self {
        Self {
            reactions: VecDeque::with_capacity(max_reactions),
            max_reactions,
            rate_limit,
            enabled: true,
            participant_counts: HashMap::new(),
        }
    }
    
    /// Add a reaction
    pub fn add_reaction(&mut self, mut reaction: Reaction) -> crate::CollaborationResult<String> {
        if !self.enabled {
            return Err(crate::CollaborationError::InvalidState("Reactions are disabled".to_string()));
        }
        
        // Check rate limit
        if !self.check_rate_limit(&reaction.participant_id) {
            return Err(crate::CollaborationError::InvalidState("Rate limit exceeded".to_string()));
        }
        
        let reaction_id = reaction.id.clone();
        
        // Add to queue
        if self.reactions.len() >= self.max_reactions {
            self.reactions.pop_front();
        }
        
        self.reactions.push_back(reaction);
        
        // Update rate limit count
        self.increment_rate_limit(reaction_id);
        
        Ok(reaction_id)
    }
    
    /// Check rate limit for participant
    fn check_rate_limit(&self, participant_id: &ParticipantId) -> bool {
        if let Some((count, window_start)) = self.participant_counts.get(participant_id) {
            let minute_ago = Utc::now() - chrono::Duration::minutes(1);
            
            if *window_start > minute_ago && *count >= self.rate_limit {
                return false;
            }
        }
        
        true
    }
    
    /// Increment rate limit counter
    fn increment_rate_limit(&mut self, participant_id: ParticipantId) {
        let now = Utc::now();
        let minute_ago = now - chrono::Duration::minutes(1);
        
        let entry = self.participant_counts.entry(participant_id).or_insert((0, now));
        
        if entry.1 < minute_ago {
            // Reset window
            entry.0 = 1;
            entry.1 = now;
        } else {
            entry.0 += 1;
        }
    }
    
    /// Get recent reactions
    pub fn get_recent(&self, count: usize) -> Vec<&Reaction> {
        self.reactions.iter().rev().take(count).collect()
    }
    
    /// Get reactions since timestamp
    pub fn get_since(&self, since: DateTime<Utc>) -> Vec<&Reaction> {
        self.reactions.iter().filter(|r| r.timestamp > since).collect()
    }
    
    /// Get reactions by participant
    pub fn get_by_participant(&self, participant_id: &ParticipantId) -> Vec<&Reaction> {
        self.reactions.iter().filter(|r| r.participant_id == *participant_id).collect()
    }
    
    /// Get reaction counts by type
    pub fn get_reaction_counts(&self) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        
        for reaction in &self.reactions {
            let key = match &reaction.reaction_type {
                ReactionType::Emoji(e) => e.clone(),
                ReactionType::Predefined(p) => p.emoji().to_string(),
                ReactionType::Animated { name, .. } => name.clone(),
                ReactionType::Sound { sound_id, .. } => sound_id.clone(),
            };
            
            *counts.entry(key).or_insert(0) += 1;
        }
        
        counts
    }
    
    /// Clear old reactions
    pub fn clear_old(&mut self, max_age_minutes: i64) {
        let threshold = Utc::now() - chrono::Duration::minutes(max_age_minutes);
        self.reactions.retain(|r| r.timestamp > threshold);
    }
    
    /// Enable/disable reactions
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if reactions are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get reaction count
    pub fn reaction_count(&self) -> usize {
        self.reactions.len()
    }
}

impl Default for ReactionManager {
    fn default() -> Self {
        Self::new(100, 30)
    }
}

/// Reaction burst (multiple reactions at once)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionBurst {
    /// Reactions in the burst
    pub reactions: Vec<Reaction>,
    
    /// Start timestamp
    pub start_time: DateTime<Utc>,
    
    /// End timestamp
    pub end_time: DateTime<Utc>,
}

impl ReactionBurst {
    /// Create a reaction burst from reactions
    pub fn from_reactions(reactions: Vec<Reaction>) -> Self {
        let start_time = reactions.iter().map(|r| r.timestamp).min().unwrap_or_else(Utc::now);
        let end_time = reactions.iter().map(|r| r.timestamp).max().unwrap_or_else(Utc::now);
        
        Self {
            reactions,
            start_time,
            end_time,
        }
    }
    
    /// Get most common reaction type
    pub fn most_common(&self) -> Option<&ReactionType> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut types: HashMap<String, &ReactionType> = HashMap::new();
        
        for reaction in &self.reactions {
            let key = format!("{:?}", reaction.reaction_type);
            *counts.entry(key.clone()).or_insert(0) += 1;
            types.insert(key, &reaction.reaction_type);
        }
        
        counts.into_iter()
            .max_by_key(|(_, c)| *c)
            .and_then(|(k, _)| types.get(&k).copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reaction_creation() {
        let reaction = Reaction::emoji(
            "sess_1".to_string(),
            "part_1".to_string(),
            "👍".to_string(),
        );
        
        assert!(matches!(reaction.reaction_type, ReactionType::Emoji(_)));
    }
    
    #[test]
    fn test_standard_reaction_emoji() {
        assert_eq!(StandardReaction::Love.emoji(), "❤️");
        assert_eq!(StandardReaction::Popcorn.emoji(), "🍿");
    }
    
    #[test]
    fn test_reaction_manager() {
        let mut manager = ReactionManager::default();
        
        let reaction = Reaction::standard(
            "sess_1".to_string(),
            "part_1".to_string(),
            StandardReaction::Like,
        );
        
        let result = manager.add_reaction(reaction);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_reaction_counts() {
        let mut manager = ReactionManager::default();
        
        manager.add_reaction(Reaction::standard(
            "sess_1".to_string(),
            "part_1".to_string(),
            StandardReaction::Like,
        )).unwrap();
        
        manager.add_reaction(Reaction::standard(
            "sess_1".to_string(),
            "part_2".to_string(),
            StandardReaction::Like,
        )).unwrap();
        
        let counts = manager.get_reaction_counts();
        assert_eq!(counts.get("👍"), Some(&2));
    }
}