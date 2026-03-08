//! Smart Content Analysis
//! 
//! Advanced content analysis features:
//! - Mood and emotion detection
//! - Sentiment analysis
//! - Content categorization
//! - Viewing pattern analysis
//! - Context-aware recommendations

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug};

// ============================================================================
// Mood and Emotion Analysis
// ============================================================================

/// Detected mood/emotion from content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mood {
    /// Happy, uplifting content
    Happy,
    /// Sad, melancholic content
    Sad,
    /// Exciting, thrilling content
    Exciting,
    /// Relaxing, calm content
    Relaxing,
    /// Dark, intense content
    Dark,
    /// Romantic, tender content
    Romantic,
    /// Funny, comedic content
    Funny,
    /// Scary, horror content
    Scary,
    /// Thought-provoking, intellectual
    ThoughtProvoking,
    /// Action-packed
    ActionPacked,
    /// Nostalgic
    Nostalgic,
    /// Inspirational
    Inspirational,
}

impl Mood {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Happy => "Happy & Uplifting",
            Self::Sad => "Emotional & Moving",
            Self::Exciting => "Exciting & Thrilling",
            Self::Relaxing => "Relaxing & Calm",
            Self::Dark => "Dark & Intense",
            Self::Romantic => "Romantic & Tender",
            Self::Funny => "Funny & Comedic",
            Self::Scary => "Scary & Horror",
            Self::ThoughtProvoking => "Thought-Provoking",
            Self::ActionPacked => "Action-Packed",
            Self::Nostalgic => "Nostalgic",
            Self::Inspirational => "Inspirational",
        }
    }
    
    /// Get emoji icon
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Happy => "😊",
            Self::Sad => "😢",
            Self::Exciting => "🎉",
            Self::Relaxing => "😌",
            Self::Dark => "🌑",
            Self::Romantic => "💕",
            Self::Funny => "😂",
            Self::Scary => "👻",
            Self::ThoughtProvoking => "🤔",
            Self::ActionPacked => "💥",
            Self::Nostalgic => "🕰️",
            Self::Inspirational => "✨",
        }
    }
}

/// Mood analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodAnalysis {
    /// Primary mood
    pub primary_mood: Mood,
    
    /// Mood scores (0.0 - 1.0)
    pub mood_scores: HashMap<Mood, f32>,
    
    /// Confidence score
    pub confidence: f32,
    
    /// Contributing factors
    pub factors: Vec<String>,
}

impl Default for MoodAnalysis {
    fn default() -> Self {
        Self {
            primary_mood: Mood::Relaxing,
            mood_scores: HashMap::new(),
            confidence: 0.5,
            factors: Vec::new(),
        }
    }
}

// ============================================================================
// Sentiment Analysis
// ============================================================================

/// Sentiment polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sentiment {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}

impl Sentiment {
    /// From score (-1.0 to 1.0)
    pub fn from_score(score: f32) -> Self {
        if score >= 0.6 {
            Self::VeryPositive
        } else if score >= 0.2 {
            Self::Positive
        } else if score <= -0.6 {
            Self::VeryNegative
        } else if score <= -0.2 {
            Self::Negative
        } else {
            Self::Neutral
        }
    }
    
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::VeryPositive => "Very Positive",
            Self::Positive => "Positive",
            Self::Neutral => "Neutral",
            Self::Negative => "Negative",
            Self::VeryNegative => "Very Negative",
        }
    }
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    /// Overall sentiment
    pub sentiment: Sentiment,
    
    /// Sentiment score (-1.0 to 1.0)
    pub score: f32,
    
    /// Aspect-based sentiments
    pub aspects: HashMap<String, f32>,
    
    /// Key phrases with sentiment
    pub key_phrases: Vec<(String, f32)>,
}

// ============================================================================
// Content Categorization
// ============================================================================

/// Smart content category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartCategory {
    /// Weekend movie
    WeekendWatch,
    /// Date night
    DateNight,
    /// Family time
    FamilyTime,
    /// Late night
    LateNight,
    /// Feel good
    FeelGood,
    /// Brain food
    BrainFood,
    /// Quick entertainment
    QuickWatch,
    /// Epic marathon
    MarathonWatch,
    /// Background viewing
    BackgroundWatch,
    /// Must see
    MustSee,
    /// Hidden gem
    HiddenGem,
    /// Cult classic
    CultClassic,
    /// Award winner
    AwardWinner,
    /// Comfort watch
    ComfortWatch,
}

impl SmartCategory {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::WeekendWatch => "Weekend Watch",
            Self::DateNight => "Date Night",
            Self::FamilyTime => "Family Time",
            Self::LateNight => "Late Night",
            Self::FeelGood => "Feel Good",
            Self::BrainFood => "Brain Food",
            Self::QuickWatch => "Quick Watch",
            Self::MarathonWatch => "Marathon Watch",
            Self::BackgroundWatch => "Background Watch",
            Self::MustSee => "Must See",
            Self::HiddenGem => "Hidden Gem",
            Self::CultClassic => "Cult Classic",
            Self::AwardWinner => "Award Winner",
            Self::ComfortWatch => "Comfort Watch",
        }
    }
    
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::WeekendWatch => "Perfect for a relaxed weekend viewing",
            Self::DateNight => "Great for watching with a partner",
            Self::FamilyTime => "Suitable for all ages, family-friendly",
            Self::LateNight => "Atmospheric for evening viewing",
            Self::FeelGood => "Uplifting and positive",
            Self::BrainFood => "Intellectually stimulating",
            Self::QuickWatch => "Short and engaging",
            Self::MarathonWatch => "Perfect for binge-watching",
            Self::BackgroundWatch => "Good for casual viewing",
            Self::MustSee => "Critical acclaim, highly recommended",
            Self::HiddenGem => "Underrated, deserves more attention",
            Self::CultClassic => "Devoted fan following",
            Self::AwardWinner => "Recognized with awards",
            Self::ComfortWatch => "Rewatchable, familiar favorite",
        }
    }
}

// ============================================================================
// Viewing Context
// ============================================================================

/// Viewing context for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewingContext {
    /// Time of day
    pub time_of_day: TimeOfDay,
    
    /// Day of week
    pub day_of_week: DayOfWeek,
    
    /// Viewing mode
    pub viewing_mode: ViewingMode,
    
    /// Company
    pub company: ViewingCompany,
    
    /// Mood preference
    pub mood_preference: Option<Mood>,
    
    /// Available time in minutes
    pub available_time: Option<u32>,
}

impl Default for ViewingContext {
    fn default() -> Self {
        Self {
            time_of_day: TimeOfDay::Evening,
            day_of_week: DayOfWeek::Weekday,
            viewing_mode: ViewingMode::Focused,
            company: ViewingCompany::Alone,
            mood_preference: None,
            available_time: None,
        }
    }
}

/// Time of day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    LateNight,
}

/// Day of week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayOfWeek {
    Weekday,
    Weekend,
    Friday,
    Saturday,
    Sunday,
}

/// Viewing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewingMode {
    Focused,
    Casual,
    Background,
}

/// Viewing company
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewingCompany {
    Alone,
    WithPartner,
    WithFamily,
    WithFriends,
}

// ============================================================================
// Smart Content Analyzer
// ============================================================================

/// Smart content analyzer with advanced analysis capabilities
pub struct SmartContentAnalyzer {
    /// Genre to mood mapping
    genre_mood_map: HashMap<String, Vec<(Mood, f32)>>,
    
    /// Keyword to mood mapping
    keyword_mood_map: HashMap<String, Vec<(Mood, f32)>>,
    
    /// Category rules
    category_rules: Vec<CategoryRule>,
}

/// Rule for categorizing content
#[derive(Debug, Clone)]
struct CategoryRule {
    category: SmartCategory,
    conditions: Vec<RuleCondition>,
    priority: u32,
}

#[derive(Debug, Clone)]
struct RuleCondition {
    field: String,
    operator: RuleOperator,
    value: String,
}

#[derive(Debug, Clone, Copy)]
enum RuleOperator {
    Equals,
    Contains,
    GreaterThan,
    LessThan,
}

impl SmartContentAnalyzer {
    /// Create a new smart analyzer
    pub fn new() -> Self {
        Self {
            genre_mood_map: Self::build_genre_mood_map(),
            keyword_mood_map: Self::build_keyword_mood_map(),
            category_rules: Self::build_category_rules(),
        }
    }
    
    /// Build genre to mood mapping
    fn build_genre_mood_map() -> HashMap<String, Vec<(Mood, f32)>> {
        let mut map = HashMap::new();
        
        map.insert("comedy".to_string(), vec![(Mood::Funny, 0.9), (Mood::Happy, 0.7)]);
        map.insert("drama".to_string(), vec![(Mood::ThoughtProvoking, 0.8), (Mood::Sad, 0.5)]);
        map.insert("action".to_string(), vec![(Mood::ActionPacked, 0.9), (Mood::Exciting, 0.8)]);
        map.insert("horror".to_string(), vec![(Mood::Scary, 0.9), (Mood::Dark, 0.7)]);
        map.insert("romance".to_string(), vec![(Mood::Romantic, 0.9), (Mood::Happy, 0.5)]);
        map.insert("thriller".to_string(), vec![(Mood::Exciting, 0.8), (Mood::Dark, 0.6)]);
        map.insert("documentary".to_string(), vec![(Mood::ThoughtProvoking, 0.9), (Mood::Inspirational, 0.5)]);
        map.insert("animation".to_string(), vec![(Mood::Happy, 0.7), (Mood::Nostalgic, 0.5)]);
        map.insert("sci-fi".to_string(), vec![(Mood::ThoughtProvoking, 0.7), (Mood::Exciting, 0.6)]);
        map.insert("fantasy".to_string(), vec![(Mood::Exciting, 0.7), (Mood::Inspirational, 0.6)]);
        map.insert("family".to_string(), vec![(Mood::Happy, 0.8), (Mood::Inspirational, 0.6)]);
        map.insert("music".to_string(), vec![(Mood::Inspirational, 0.8), (Mood::Happy, 0.6)]);
        map.insert("war".to_string(), vec![(Mood::Dark, 0.8), (Mood::ThoughtProvoking, 0.7)]);
        map.insert("western".to_string(), vec![(Mood::Nostalgic, 0.7), (Mood::ActionPacked, 0.5)]);
        
        map
    }
    
    /// Build keyword to mood mapping
    fn build_keyword_mood_map() -> HashMap<String, Vec<(Mood, f32)>> {
        let mut map = HashMap::new();
        
        // Happy/Positive keywords
        for kw in &["love", "heartwarming", "uplifting", "joy", "happy", "inspiring", "hope"] {
            map.insert(kw.to_string(), vec![(Mood::Happy, 0.8), (Mood::Inspirational, 0.6)]);
        }
        
        // Sad/Emotional keywords
        for kw in &["tragic", "heartbreak", "loss", "death", "sad", "emotional", "moving"] {
            map.insert(kw.to_string(), vec![(Mood::Sad, 0.9), (Mood::ThoughtProvoking, 0.5)]);
        }
        
        // Exciting keywords
        for kw in &["adventure", "quest", "journey", "exciting", "thrilling", "suspense"] {
            map.insert(kw.to_string(), vec![(Mood::Exciting, 0.9), (Mood::ActionPacked, 0.5)]);
        }
        
        // Scary keywords
        for kw in &["terror", "haunted", "ghost", "nightmare", "scary", "creepy", "zombie"] {
            map.insert(kw.to_string(), vec![(Mood::Scary, 0.9), (Mood::Dark, 0.7)]);
        }
        
        // Dark keywords
        for kw in &["dark", "noir", "gritty", "crime", "murder", "serial killer", "psychopath"] {
            map.insert(kw.to_string(), vec![(Mood::Dark, 0.9), (Mood::Scary, 0.4)]);
        }
        
        // Romantic keywords
        for kw in &["romance", "wedding", "kiss", "passion", "lovestory", "beloved"] {
            map.insert(kw.to_string(), vec![(Mood::Romantic, 0.9), (Mood::Happy, 0.5)]);
        }
        
        // Funny keywords
        for kw in &["comedy", "funny", "hilarious", "laugh", "humor", "witty", "satire"] {
            map.insert(kw.to_string(), vec![(Mood::Funny, 0.9), (Mood::Happy, 0.6)]);
        }
        
        // Relaxing keywords
        for kw in &["peaceful", "calm", "serene", "meditative", "relaxing", "gentle"] {
            map.insert(kw.to_string(), vec![(Mood::Relaxing, 0.9), (Mood::Happy, 0.4)]);
        }
        
        map
    }
    
    /// Build category rules
    fn build_category_rules() -> Vec<CategoryRule> {
        vec![
            // Must See - high rating
            CategoryRule {
                category: SmartCategory::MustSee,
                conditions: vec![],
                priority: 100,
            },
            // Family Time
            CategoryRule {
                category: SmartCategory::FamilyTime,
                conditions: vec![],
                priority: 80,
            },
            // Date Night
            CategoryRule {
                category: SmartCategory::DateNight,
                conditions: vec![],
                priority: 75,
            },
            // Quick Watch
            CategoryRule {
                category: SmartCategory::QuickWatch,
                conditions: vec![],
                priority: 70,
            },
            // Marathon Watch
            CategoryRule {
                category: SmartCategory::MarathonWatch,
                conditions: vec![],
                priority: 65,
            },
            // Late Night
            CategoryRule {
                category: SmartCategory::LateNight,
                conditions: vec![],
                priority: 60,
            },
            // Feel Good
            CategoryRule {
                category: SmartCategory::FeelGood,
                conditions: vec![],
                priority: 55,
            },
            // Brain Food
            CategoryRule {
                category: SmartCategory::BrainFood,
                conditions: vec![],
                priority: 50,
            },
            // Hidden Gem
            CategoryRule {
                category: SmartCategory::HiddenGem,
                conditions: vec![],
                priority: 45,
            },
            // Comfort Watch
            CategoryRule {
                category: SmartCategory::ComfortWatch,
                conditions: vec![],
                priority: 40,
            },
        ]
    }
    
    /// Analyze mood of content
    pub fn analyze_mood(&self, genres: &[String], synopsis: Option<&str>, keywords: &[String]) -> MoodAnalysis {
        let mut mood_scores: HashMap<Mood, f32> = HashMap::new();
        let mut factors = Vec::new();
        
        // Analyze genres
        for genre in genres {
            let genre_lower = genre.to_lowercase();
            if let Some(moods) = self.genre_mood_map.get(&genre_lower) {
                for (mood, score) in moods {
                    let current = mood_scores.entry(*mood).or_insert(0.0);
                    *current = (*current).max(*score);
                    factors.push(format!("Genre: {}", genre));
                }
            }
        }
        
        // Analyze synopsis keywords
        if let Some(synopsis) = synopsis {
            let synopsis_lower = synopsis.to_lowercase();
            for (keyword, moods) in &self.keyword_mood_map {
                if synopsis_lower.contains(keyword) {
                    for (mood, score) in moods {
                        let current = mood_scores.entry(*mood).or_insert(0.0);
                        *current = (*current + score * 0.5).min(1.0);
                    }
                    factors.push(format!("Keyword: {}", keyword));
                }
            }
        }
        
        // Analyze explicit keywords
        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();
            if let Some(moods) = self.keyword_mood_map.get(&keyword_lower) {
                for (mood, score) in moods {
                    let current = mood_scores.entry(*mood).or_insert(0.0);
                    *current = (*current + score * 0.3).min(1.0);
                }
            }
        }
        
        // Find primary mood
        let primary_mood = mood_scores.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(m, _)| *m)
            .unwrap_or(Mood::Relaxing);
        
        // Calculate confidence
        let top_scores: Vec<f32> = mood_scores.values().cloned().collect();
        let confidence = if top_scores.is_empty() {
            0.3
        } else {
            let max_score = top_scores.iter().cloned().fold(0.0, f32::max);
            max_score.min(1.0)
        };
        
        MoodAnalysis {
            primary_mood,
            mood_scores,
            confidence,
            factors: factors.into_iter().take(5).collect(),
        }
    }
    
    /// Analyze sentiment
    pub fn analyze_sentiment(&self, text: &str) -> SentimentAnalysis {
        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut key_phrases = Vec::new();
        
        let positive_words: HashSet<&str> = [
            "good", "great", "excellent", "amazing", "wonderful", "fantastic",
            "love", "beautiful", "perfect", "best", "brilliant", "outstanding",
            "inspiring", "uplifting", "masterpiece", "captivating", "stunning",
        ].iter().cloned().collect();
        
        let negative_words: HashSet<&str> = [
            "bad", "terrible", "awful", "horrible", "worst", "poor", "boring",
            "disappointing", "waste", "dull", "mediocre", "weak", "fails",
            "annoying", "confusing", "predictable", "slow", "forgettable",
        ].iter().cloned().collect();
        
        let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();
        
        for word in &words {
            if positive_words.contains(word) {
                positive_count += 1;
                key_phrases.push((word.to_string(), 0.5));
            } else if negative_words.contains(word) {
                negative_count += 1;
                key_phrases.push((word.to_string(), -0.5));
            }
        }
        
        let total = positive_count + negative_count;
        let score = if total == 0 {
            0.0
        } else {
            (positive_count as f32 - negative_count as f32) / total as f32
        };
        
        SentimentAnalysis {
            sentiment: Sentiment::from_score(score),
            score,
            aspects: HashMap::new(),
            key_phrases: key_phrases.into_iter().take(10).collect(),
        }
    }
    
    /// Categorize content
    pub fn categorize(&self, metadata: &ContentMetadataSimple) -> Vec<SmartCategory> {
        let mut categories = Vec::new();
        
        // Must See - high rating
        if metadata.rating >= 8.0 {
            categories.push(SmartCategory::MustSee);
        }
        
        // Family Time - appropriate for all ages
        if metadata.age_rating.as_deref() == Some("G") || metadata.age_rating.as_deref() == Some("PG") {
            categories.push(SmartCategory::FamilyTime);
        }
        
        // Date Night - romantic genres
        if metadata.genres.iter().any(|g| g.to_lowercase().contains("romance")) {
            categories.push(SmartCategory::DateNight);
        }
        
        // Quick Watch - short content
        if metadata.duration_minutes.map(|d| d < 90).unwrap_or(false) {
            categories.push(SmartCategory::QuickWatch);
        }
        
        // Marathon Watch - series or long content
        if metadata.is_series || metadata.duration_minutes.map(|d| d > 150).unwrap_or(false) {
            categories.push(SmartCategory::MarathonWatch);
        }
        
        // Late Night - dark or horror
        if metadata.genres.iter().any(|g| {
            let g_lower = g.to_lowercase();
            g_lower.contains("horror") || g_lower.contains("thriller") || g_lower.contains("noir")
        }) {
            categories.push(SmartCategory::LateNight);
        }
        
        // Feel Good - comedy or happy mood
        if metadata.genres.iter().any(|g| g.to_lowercase().contains("comedy")) {
            categories.push(SmartCategory::FeelGood);
        }
        
        // Brain Food - documentary or thought-provoking
        if metadata.genres.iter().any(|g| {
            let g_lower = g.to_lowercase();
            g_lower.contains("documentary") || g_lower.contains("biography")
        }) {
            categories.push(SmartCategory::BrainFood);
        }
        
        // Hidden Gem - low popularity but good rating
        if metadata.rating >= 7.0 && metadata.popularity.map(|p| p < 0.3).unwrap_or(true) {
            categories.push(SmartCategory::HiddenGem);
        }
        
        // Award Winner
        if metadata.awards.as_deref().map(|a| !a.is_empty()).unwrap_or(false) {
            categories.push(SmartCategory::AwardWinner);
        }
        
        categories
    }
    
    /// Get recommendations based on context
    pub fn get_contextual_recommendations(
        &self,
        content: &[ContentMetadataSimple],
        context: &ViewingContext,
    ) -> Vec<ContentRecommendation> {
        let mut recommendations = Vec::new();
        
        for item in content {
            let mut score = 0.0;
            let mut reasons = Vec::new();
            
            // Time of day matching
            match context.time_of_day {
                TimeOfDay::Morning => {
                    if item.genres.iter().any(|g| g.to_lowercase().contains("comedy")) {
                        score += 0.2;
                        reasons.push("Good morning watch".to_string());
                    }
                }
                TimeOfDay::Evening => {
                    if item.genres.iter().any(|g| g.to_lowercase().contains("drama")) {
                        score += 0.2;
                        reasons.push("Perfect for evening".to_string());
                    }
                }
                TimeOfDay::LateNight => {
                    if item.genres.iter().any(|g| {
                        g.to_lowercase().contains("horror") || g.to_lowercase().contains("thriller")
                    }) {
                        score += 0.3;
                        reasons.push("Great for late night".to_string());
                    }
                }
                _ => {}
            }
            
            // Weekend vs Weekday
            match context.day_of_week {
                DayOfWeek::Weekend | DayOfWeek::Saturday | DayOfWeek::Sunday => {
                    if item.duration_minutes.map(|d| d > 120).unwrap_or(false) {
                        score += 0.2;
                        reasons.push("Perfect for weekend".to_string());
                    }
                }
                DayOfWeek::Weekday => {
                    if item.duration_minutes.map(|d| d < 120).unwrap_or(true) {
                        score += 0.1;
                        reasons.push("Good for weekday".to_string());
                    }
                }
                _ => {}
            }
            
            // Company
            match context.company {
                ViewingCompany::WithFamily => {
                    if item.age_rating.as_deref().map(|a| a == "G" || a == "PG").unwrap_or(false) {
                        score += 0.3;
                        reasons.push("Family-friendly".to_string());
                    }
                }
                ViewingCompany::WithPartner => {
                    if item.genres.iter().any(|g| g.to_lowercase().contains("romance")) {
                        score += 0.2;
                        reasons.push("Great for date night".to_string());
                    }
                }
                _ => {}
            }
            
            // Available time
            if let Some(available) = context.available_time {
                if item.duration_minutes.map(|d| d <= available).unwrap_or(false) {
                    score += 0.2;
                    reasons.push(format!("Fits your {} min window", available));
                }
            }
            
            // Mood preference
            if let Some(mood) = &context.mood_preference {
                let mood_analysis = self.analyze_mood(&item.genres, None, &[]);
                if mood_analysis.primary_mood == *mood {
                    score += 0.3;
                    reasons.push(format!("Matches your {} mood", mood.display_name()));
                }
            }
            
            if score > 0.0 {
                recommendations.push(ContentRecommendation {
                    id: item.id.clone(),
                    score,
                    reasons,
                });
            }
        }
        
        // Sort by score
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        recommendations
    }
}

impl Default for SmartContentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Types
// ============================================================================

/// Simplified content metadata for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadataSimple {
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub rating: f32,
    pub duration_minutes: Option<u32>,
    pub is_series: bool,
    pub age_rating: Option<String>,
    pub popularity: Option<f32>,
    pub awards: Option<String>,
}

/// Content recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRecommendation {
    pub id: String,
    pub score: f32,
    pub reasons: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mood_display() {
        assert_eq!(Mood::Happy.display_name(), "Happy & Uplifting");
        assert_eq!(Mood::Funny.emoji(), "😂");
    }
    
    #[test]
    fn test_sentiment_from_score() {
        assert_eq!(Sentiment::from_score(0.7), Sentiment::VeryPositive);
        assert_eq!(Sentiment::from_score(0.3), Sentiment::Positive);
        assert_eq!(Sentiment::from_score(0.0), Sentiment::Neutral);
        assert_eq!(Sentiment::from_score(-0.3), Sentiment::Negative);
        assert_eq!(Sentiment::from_score(-0.7), Sentiment::VeryNegative);
    }
    
    #[test]
    fn test_mood_analysis() {
        let analyzer = SmartContentAnalyzer::new();
        
        let genres = vec!["Comedy".to_string(), "Romance".to_string()];
        let analysis = analyzer.analyze_mood(&genres, Some("A heartwarming love story"), &[]);
        
        assert!(!analysis.mood_scores.is_empty());
        assert!(analysis.confidence > 0.0);
    }
    
    #[test]
    fn test_sentiment_analysis() {
        let analyzer = SmartContentAnalyzer::new();
        
        let text = "This is a great and amazing movie with wonderful acting";
        let sentiment = analyzer.analyze_sentiment(text);
        
        assert!(sentiment.score > 0.0);
        assert!(matches!(sentiment.sentiment, Sentiment::Positive | Sentiment::VeryPositive));
    }
    
    #[test]
    fn test_content_categorization() {
        let analyzer = SmartContentAnalyzer::new();
        
        let metadata = ContentMetadataSimple {
            id: "1".to_string(),
            title: "Test Movie".to_string(),
            genres: vec!["Comedy".to_string()],
            rating: 8.5,
            duration_minutes: Some(85),
            is_series: false,
            age_rating: Some("PG".to_string()),
            popularity: Some(0.5),
            awards: None,
        };
        
        let categories = analyzer.categorize(&metadata);
        
        assert!(categories.contains(&SmartCategory::MustSee));
        assert!(categories.contains(&SmartCategory::FamilyTime));
        assert!(categories.contains(&SmartCategory::FeelGood));
        assert!(categories.contains(&SmartCategory::QuickWatch));
    }
    
    #[test]
    fn test_contextual_recommendations() {
        let analyzer = SmartContentAnalyzer::new();
        
        let content = vec![
            ContentMetadataSimple {
                id: "1".to_string(),
                title: "Horror Movie".to_string(),
                genres: vec!["Horror".to_string()],
                rating: 7.5,
                duration_minutes: Some(110),
                is_series: false,
                age_rating: Some("R".to_string()),
                popularity: Some(0.5),
                awards: None,
            },
            ContentMetadataSimple {
                id: "2".to_string(),
                title: "Family Comedy".to_string(),
                genres: vec!["Comedy".to_string(), "Family".to_string()],
                rating: 7.0,
                duration_minutes: Some(90),
                is_series: false,
                age_rating: Some("PG".to_string()),
                popularity: Some(0.6),
                awards: None,
            },
        ];
        
        let context = ViewingContext {
            time_of_day: TimeOfDay::LateNight,
            day_of_week: DayOfWeek::Weekend,
            viewing_mode: ViewingMode::Focused,
            company: ViewingCompany::Alone,
            mood_preference: Some(Mood::Scary),
            available_time: Some(120),
        };
        
        let recommendations = analyzer.get_contextual_recommendations(&content, &context);
        
        // Horror movie should score higher for late night + scary mood
        assert!(!recommendations.is_empty());
        assert_eq!(recommendations[0].id, "1"); // Horror movie first
    }
}