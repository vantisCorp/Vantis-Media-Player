//! Media Library Search Enhancements
//! 
//! Advanced search functionality:
//! - Fuzzy matching
//! - Faceted search with filters
//! - Search suggestions and autocomplete
//! - Search history
//! - Saved searches
//! - Advanced query syntax

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug};

// ============================================================================
// Search Query Types
// ============================================================================

/// Search query with advanced options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Raw query text
    pub text: String,
    
    /// Parsed filters
    pub filters: Vec<SearchFilter>,
    
    /// Sort order
    pub sort: Option<SortOption>,
    
    /// Maximum results
    pub limit: Option<usize>,
    
    /// Offset for pagination
    pub offset: Option<usize>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: String::new(),
            filters: Vec::new(),
            sort: None,
            limit: Some(100),
            offset: None,
        }
    }
}

impl SearchQuery {
    /// Create a new query from text
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }
    
    /// Parse a query string with advanced syntax
    /// Supports: type:video, duration:>60, artist:"Pink Floyd", etc.
    pub fn parse(query_text: &str) -> Self {
        let mut query = Self::default();
        let mut remaining_text = String::new();
        
        // Split by spaces, but respect quoted strings
        let tokens = tokenize_query(query_text);
        
        for token in tokens {
            if token.contains(':') {
                // This is a filter
                if let Some(filter) = parse_filter(&token) {
                    query.filters.push(filter);
                    continue;
                }
            }
            
            // Regular search term
            if !remaining_text.is_empty() {
                remaining_text.push(' ');
            }
            remaining_text.push_str(&token);
        }
        
        query.text = remaining_text;
        query
    }
    
    /// Add a filter
    pub fn with_filter(mut self, filter: SearchFilter) -> Self {
        self.filters.push(filter);
        self
    }
    
    /// Add sort option
    pub fn with_sort(mut self, sort: SortOption) -> Self {
        self.sort = Some(sort);
        self
    }
    
    /// Set limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Check if query is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.filters.is_empty()
    }
}

/// Search filter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchFilter {
    /// Media type filter
    MediaType(MediaFilter),
    
    /// Duration filter
    Duration(DurationFilter),
    
    /// File size filter
    FileSize(FileSizeFilter),
    
    /// Date range filter
    DateRange(DateRangeFilter),
    
    /// Resolution filter
    Resolution(ResolutionFilter),
    
    /// Codec filter
    Codec(String),
    
    /// Artist/author filter
    Artist(String),
    
    /// Album filter
    Album(String),
    
    /// Genre filter
    Genre(String),
    
    /// Rating filter
    Rating(RatingFilter),
    
    /// Play count filter
    PlayCount(PlayCountFilter),
    
    /// Custom field filter
    Custom { field: String, value: String },
}

/// Media type filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFilter {
    Video,
    Audio,
    Image,
    Playlist,
}

/// Duration filter with comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationFilter {
    pub comparison: Comparison,
    pub value: u64, // seconds
}

/// File size filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeFilter {
    pub comparison: Comparison,
    pub value: u64, // bytes
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeFilter {
    pub from: Option<i64>, // Unix timestamp
    pub to: Option<i64>,
}

/// Resolution filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionFilter {
    SD,      // < 720p
    HD,      // 720p - 1080p
    FullHD,  // 1080p
    UHD4K,   // 4K
    UHD8K,   // 8K
    Custom { min_width: u32, min_height: u32 },
}

/// Rating filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingFilter {
    pub comparison: Comparison,
    pub value: f32, // 0.0 - 10.0
}

/// Play count filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayCountFilter {
    pub comparison: Comparison,
    pub value: u32,
}

/// Comparison operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparison {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// Sort options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOption {
    pub field: SortField,
    pub direction: SortDirection,
}

impl Default for SortOption {
    fn default() -> Self {
        Self {
            field: SortField::Relevance,
            direction: SortDirection::Descending,
        }
    }
}

/// Sortable fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    Relevance,
    Title,
    Duration,
    DateAdded,
    DateModified,
    PlayCount,
    Rating,
    FileSize,
    Artist,
    Album,
    Genre,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

// ============================================================================
// Search Results
// ============================================================================

/// Search result with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Media item ID
    pub id: String,
    
    /// Relevance score (0.0 - 1.0)
    pub score: f32,
    
    /// Matched fields
    pub matched_fields: Vec<MatchedField>,
    
    /// Highlighted snippets
    pub highlights: HashMap<String, String>,
}

/// Matched field information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedField {
    pub field: String,
    pub score: f32,
    pub positions: Vec<(usize, usize)>, // Start, end positions
}

/// Complete search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Query that was executed
    pub query: SearchQuery,
    
    /// Results
    pub results: Vec<SearchResult>,
    
    /// Total count (may be more than results if paginated)
    pub total: usize,
    
    /// Facets for filtering
    pub facets: SearchFacets,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Suggestions for query refinement
    pub suggestions: Vec<String>,
}

/// Faceted search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacets {
    /// Media type counts
    pub media_types: HashMap<String, usize>,
    
    /// Genre counts
    pub genres: HashMap<String, usize>,
    
    /// Artist counts
    pub artists: HashMap<String, usize>,
    
    /// Decade counts
    pub decades: HashMap<String, usize>,
    
    /// Resolution counts
    pub resolutions: HashMap<String, usize>,
}

impl Default for SearchFacets {
    fn default() -> Self {
        Self {
            media_types: HashMap::new(),
            genres: HashMap::new(),
            artists: HashMap::new(),
            decades: HashMap::new(),
            resolutions: HashMap::new(),
        }
    }
}

// ============================================================================
// Search Engine
// ============================================================================

/// Advanced search engine for media library
pub struct LibrarySearchEngine {
    /// Search history
    history: Arc<RwLock<Vec<SearchHistoryEntry>>>,
    
    /// Saved searches
    saved_searches: Arc<RwLock<Vec<SavedSearch>>>,
    
    /// Maximum history size
    max_history: usize,
    
    /// Fuzzy matching threshold
    fuzzy_threshold: f32,
    
    /// Enable typo tolerance
    typo_tolerance: bool,
    
    /// Maximum typo distance
    max_typo_distance: usize,
}

impl LibrarySearchEngine {
    /// Create a new search engine
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            saved_searches: Arc::new(RwLock::new(Vec::new())),
            max_history: 100,
            fuzzy_threshold: 0.6,
            typo_tolerance: true,
            max_typo_distance: 2,
        }
    }
    
    /// Execute a search query
    pub fn search(&self, query: &SearchQuery) -> SearchResponse {
        let start = std::time::Instant::now();
        
        info!("🔍 Searching: &quot;{}&quot;", query.text);
        
        // Add to history
        self.add_to_history(&query.text);
        
        // In a real implementation, this would:
        // 1. Parse query
        // 2. Apply filters
        // 3. Execute full-text search
        // 4. Calculate relevance scores
        // 5. Apply sorting
        // 6. Generate facets
        
        let results = Vec::new();
        let facets = SearchFacets::default();
        
        let execution_time_ms = start.elapsed().as_millis() as u64;
        
        SearchResponse {
            query: query.clone(),
            results,
            total: 0,
            facets,
            execution_time_ms,
            suggestions: Vec::new(),
        }
    }
    
    /// Quick search for autocomplete
    pub fn quick_search(&self, prefix: &str, limit: usize) -> Vec<String> {
        // In a real implementation, this would search an index
        // and return matching titles, artists, etc.
        let mut suggestions = Vec::new();
        
        // Placeholder: return the prefix itself
        if !prefix.is_empty() {
            suggestions.push(prefix.to_string());
        }
        
        suggestions.into_iter().take(limit).collect()
    }
    
    /// Get search suggestions for a partial query
    pub fn get_suggestions(&self, partial: &str) -> Vec<SearchSuggestion> {
        let mut suggestions = Vec::new();
        
        // Check history for matching queries
        let history = self.history.read();
        for entry in history.iter().rev() {
            if entry.query.starts_with(partial) {
                suggestions.push(SearchSuggestion {
                    text: entry.query.clone(),
                    kind: SuggestionKind::History,
                    score: 1.0,
                });
            }
        }
        
        // Add field suggestions if it looks like a filter
        if partial.contains(':') && !partial.ends_with(':') {
            // Already typing a filter value
        } else if partial.ends_with(':') {
            // Suggest values for this field
            let field = partial.trim_end_matches(':');
            suggestions.extend(self.get_field_value_suggestions(field));
        } else if partial.is_empty() {
            // Suggest common filters
            suggestions.extend(self.get_common_filter_suggestions());
        }
        
        suggestions.into_iter().take(10).collect()
    }
    
    /// Get suggestions for field values
    fn get_field_value_suggestions(&self, field: &str) -> Vec<SearchSuggestion> {
        let mut suggestions = Vec::new();
        
        match field.to_lowercase().as_str() {
            "type" | "media" => {
                for t in ["video", "audio", "image", "playlist"] {
                    suggestions.push(SearchSuggestion {
                        text: format!("type:{}", t),
                        kind: SuggestionKind::Filter,
                        score: 0.9,
                    });
                }
            }
            "genre" => {
                for g in ["rock", "pop", "jazz", "classical", "electronic", "hip-hop"] {
                    suggestions.push(SearchSuggestion {
                        text: format!("genre:{}", g),
                        kind: SuggestionKind::Filter,
                        score: 0.8,
                    });
                }
            }
            _ => {}
        }
        
        suggestions
    }
    
    /// Get common filter suggestions
    fn get_common_filter_suggestions(&self) -> Vec<SearchSuggestion> {
        vec![
            SearchSuggestion {
                text: "type:video".to_string(),
                kind: SuggestionKind::Filter,
                score: 1.0,
            },
            SearchSuggestion {
                text: "type:audio".to_string(),
                kind: SuggestionKind::Filter,
                score: 0.95,
            },
            SearchSuggestion {
                text: "duration:>60".to_string(),
                kind: SuggestionKind::Filter,
                score: 0.9,
            },
            SearchSuggestion {
                text: "rating:>7".to_string(),
                kind: SuggestionKind::Filter,
                score: 0.85,
            },
        ]
    }
    
    /// Add query to history
    fn add_to_history(&self, query: &str) {
        if query.is_empty() {
            return;
        }
        
        let mut history = self.history.write();
        
        // Remove duplicate
        history.retain(|e| e.query != query);
        
        // Add new entry
        history.push(SearchHistoryEntry {
            query: query.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        
        // Trim to max size
        if history.len() > self.max_history {
            history.remove(0);
        }
    }
    
    /// Get search history
    pub fn get_history(&self) -> Vec<SearchHistoryEntry> {
        self.history.read().clone()
    }
    
    /// Clear search history
    pub fn clear_history(&self) {
        self.history.write().clear();
    }
    
    /// Save a search
    pub fn save_search(&self, name: &str, query: &SearchQuery) {
        let mut saved = self.saved_searches.write();
        
        saved.push(SavedSearch {
            name: name.to_string(),
            query: query.clone(),
            created_at: chrono::Utc::now().timestamp(),
        });
    }
    
    /// Get saved searches
    pub fn get_saved_searches(&self) -> Vec<SavedSearch> {
        self.saved_searches.read().clone()
    }
    
    /// Delete a saved search
    pub fn delete_saved_search(&self, name: &str) {
        self.saved_searches.write().retain(|s| s.name != name);
    }
    
    /// Calculate fuzzy match score
    pub fn fuzzy_match(&self, text: &str, pattern: &str) -> f32 {
        if pattern.is_empty() {
            return 1.0;
        }
        
        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        
        // Exact substring match
        if text_lower.contains(&pattern_lower) {
            return 1.0;
        }
        
        // Fuzzy match using Levenshtein distance
        if self.typo_tolerance {
            let distance = levenshtein_distance(&text_lower, &pattern_lower);
            let max_len = text_lower.len().max(pattern_lower.len());
            
            if distance <= self.max_typo_distance {
                let similarity = 1.0 - (distance as f32 / max_len as f32);
                return similarity.max(self.fuzzy_threshold);
            }
        }
        
        0.0
    }
}

impl Default for LibrarySearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Search Suggestions
// ============================================================================

/// Search suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub kind: SuggestionKind,
    pub score: f32,
}

/// Suggestion kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionKind {
    /// From search history
    History,
    /// Filter suggestion
    Filter,
    /// Field value suggestion
    FieldValue,
    /// Popular query
    Popular,
    /// Title match
    Title,
    /// Artist match
    Artist,
}

// ============================================================================
// History and Saved Searches
// ============================================================================

/// Search history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryEntry {
    pub query: String,
    pub timestamp: i64,
}

/// Saved search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    pub name: String,
    pub query: SearchQuery,
    pub created_at: i64,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Tokenize a query string, respecting quotes
fn tokenize_query(query: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    
    for ch in query.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}

/// Parse a filter from a token
fn parse_filter(token: &str) -> Option<SearchFilter> {
    let parts: Vec<&str> = token.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let field = parts[0].to_lowercase();
    let value = parts[1];
    
    match field.as_str() {
        "type" | "media" => match value.to_lowercase().as_str() {
            "video" => Some(SearchFilter::MediaType(MediaFilter::Video)),
            "audio" => Some(SearchFilter::MediaType(MediaFilter::Audio)),
            "image" => Some(SearchFilter::MediaType(MediaFilter::Image)),
            "playlist" => Some(SearchFilter::MediaType(MediaFilter::Playlist)),
            _ => None,
        },
        "artist" => Some(SearchFilter::Artist(value.to_string())),
        "album" => Some(SearchFilter::Album(value.to_string())),
        "genre" => Some(SearchFilter::Genre(value.to_string())),
        "codec" => Some(SearchFilter::Codec(value.to_string())),
        _ => Some(SearchFilter::Custom {
            field: parts[0].to_string(),
            value: value.to_string(),
        }),
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    
    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }
    
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    
    for j in 0..=b_len {
        matrix[0][j] = j;
    }
    
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_query_parse() {
        let query = SearchQuery::parse("pink floyd type:audio genre:rock");
        
        assert!(query.text.contains("pink floyd"));
        assert_eq!(query.filters.len(), 2);
    }
    
    #[test]
    fn test_search_query_with_quotes() {
        let query = SearchQuery::parse("artist:&quot;Pink Floyd&quot; type:album");
        
        assert_eq!(query.filters.len(), 2);
    }
    
    #[test]
    fn test_tokenize_query() {
        let tokens = tokenize_query("hello world &quot;quoted phrase&quot; test");
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[2], "quoted phrase");
    }
    
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("hello", "helo"), 1);
        assert_eq!(levenshtein_distance("hello", ""), 5);
    }
    
    #[test]
    fn test_fuzzy_match() {
        let engine = LibrarySearchEngine::new();
        
        let score = engine.fuzzy_match("Hello World", "hello");
        assert!(score > 0.5);
        
        let score = engine.fuzzy_match("Hello World", "xyz");
        assert!(score < 0.5);
    }
    
    #[test]
    fn test_search_history() {
        let engine = LibrarySearchEngine::new();
        
        engine.search(&SearchQuery::new("test query"));
        
        let history = engine.get_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].query, "test query");
    }
    
    #[test]
    fn test_saved_search() {
        let engine = LibrarySearchEngine::new();
        let query = SearchQuery::new("rock music").with_filter(SearchFilter::Genre("rock".to_string()));
        
        engine.save_search("My Rock Music", &query);
        
        let saved = engine.get_saved_searches();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].name, "My Rock Music");
    }
    
    #[test]
    fn test_search_suggestions() {
        let engine = LibrarySearchEngine::new();
        
        // Add to history first
        engine.search(&SearchQuery::new("pink floyd"));
        
        let suggestions = engine.get_suggestions("pin");
        assert!(!suggestions.is_empty());
    }
    
    #[test]
    fn test_sort_option_default() {
        let sort = SortOption::default();
        assert_eq!(sort.field, SortField::Relevance);
        assert_eq!(sort.direction, SortDirection::Descending);
    }
}