//! Content analysis and feature extraction

use crate::error::Result;
use crate::types::*;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

/// Content features extracted for ML
#[derive(Debug, Clone, Default)]
pub struct ContentFeatures {
    /// Feature vector
    pub vector: Vec<f64>,
    /// Genres
    pub genres: Vec<String>,
    /// Content type
    pub content_type: ContentType,
}

impl ContentFeatures {
    /// Create new content features
    pub fn new(vector: Vec<f64>) -> Self {
        Self {
            vector,
            genres: Vec::new(),
            content_type: ContentType::Movie,
        }
    }

    /// Calculate cosine similarity with another feature set
    pub fn cosine_similarity(&self, other: &ContentFeatures) -> f64 {
        if self.vector.len() != other.vector.len() || self.vector.is_empty() {
            return 0.0;
        }

        let dot: f64 = self.vector.iter().zip(&other.vector).map(|(a, b)| a * b).sum();
        let norm_a: f64 = self.vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = other.vector.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }

    /// Get dimensionality
    pub fn dim(&self) -> usize {
        self.vector.len()
    }
}

/// Content cluster for grouping similar content
#[derive(Debug, Clone)]
pub struct ContentCluster {
    /// Cluster ID
    pub id: usize,
    /// Cluster centroid
    pub centroid: Vec<f64>,
    /// Content IDs in cluster
    pub content_ids: Vec<ContentId>,
    /// Dominant genres
    pub dominant_genres: Vec<String>,
    /// Cluster label
    pub label: String,
}

impl ContentCluster {
    /// Create a new cluster
    pub fn new(id: usize, centroid: Vec<f64>) -> Self {
        Self {
            id,
            centroid,
            content_ids: Vec::new(),
            dominant_genres: Vec::new(),
            label: format!("Cluster {}", id),
        }
    }

    /// Add content to cluster
    pub fn add(&mut self, content_id: ContentId) {
        self.content_ids.push(content_id);
    }

    /// Get cluster size
    pub fn size(&self) -> usize {
        self.content_ids.len()
    }
}

/// Content analyzer for feature extraction
pub struct ContentAnalyzer {
    /// Genre vocabulary
    genre_vocabulary: HashMap<String, usize>,
    /// Content type vocabulary
    type_vocabulary: HashMap<String, usize>,
    /// Feature dimensionality
    feature_dim: usize,
}

impl ContentAnalyzer {
    /// Create a new content analyzer
    pub fn new() -> Self {
        let mut genre_vocabulary = HashMap::new();
        let genres = [
            "Action", "Adventure", "Animation", "Comedy", "Crime",
            "Documentary", "Drama", "Family", "Fantasy", "History",
            "Horror", "Music", "Mystery", "Romance", "Sci-Fi",
            "Thriller", "War", "Western", "Biography", "Sport",
        ];
        
        for (i, genre) in genres.iter().enumerate() {
            genre_vocabulary.insert(genre.to_lowercase(), i);
        }
        
        let mut type_vocabulary = HashMap::new();
        let types = [
            "Movie", "Series", "Episode", "Documentary", "Short",
            "Music", "Podcast", "Audiobook", "LiveStream", "UserGenerated",
        ];
        
        for (i, t) in types.iter().enumerate() {
            type_vocabulary.insert(t.to_lowercase(), i);
        }
        
        // Feature dimensions: genres + types + numeric features
        let feature_dim = genres.len() + types.len() + 10;
        
        Self {
            genre_vocabulary,
            type_vocabulary,
            feature_dim,
        }
    }

    /// Analyze content and extract features
    pub fn analyze(&self, content: &ContentMetadata) -> ContentFeatures {
        let mut vector = vec![0.0; self.feature_dim];
        
        // Encode genres (one-hot)
        let genre_offset = 0;
        for genre in &content.genres {
            if let Some(&idx) = self.genre_vocabulary.get(&genre.as_str().to_lowercase()) {
                vector[genre_offset + idx] = 1.0;
            }
        }
        
        // Encode content type (one-hot)
        let type_offset = self.genre_vocabulary.len();
        let type_str = format!("{:?}", content.content_type).to_lowercase();
        if let Some(&idx) = self.type_vocabulary.get(&type_str) {
            vector[type_offset + idx] = 1.0;
        }
        
        // Numeric features
        let numeric_offset = type_offset + self.type_vocabulary.len();
        
        // Rating normalized (0-1)
        vector[numeric_offset] = content.rating / 10.0;
        
        // Duration normalized (0-1, capped at 3 hours)
        let duration_hours = content.duration_seconds as f64 / 10800.0;
        vector[numeric_offset + 1] = duration_hours.min(1.0);
        
        // Release year normalized (1900-2030 -> 0-1)
        let year_norm = (content.release_year - 1900) as f64 / 130.0;
        vector[numeric_offset + 2] = year_norm.clamp(0.0, 1.0);
        
        // Popularity
        vector[numeric_offset + 3] = content.popularity;
        
        // Cast size normalized (log scale, max 20)
        let cast_size = (content.cast.len() as f64 + 1.0).ln() / (21.0_f64).ln();
        vector[numeric_offset + 4] = cast_size;
        
        // Tags count normalized (log scale, max 20)
        let tags_count = (content.tags.len() as f64 + 1.0).ln() / (21.0_f64).ln();
        vector[numeric_offset + 5] = tags_count;
        
        // Age suitability
        vector[numeric_offset + 6] = content.age_suitability();
        
        // Custom features
        for (key, value) in &content.custom_features {
            // Hash the key to get an index
            let hash = Self::hash_key(key);
            let idx = numeric_offset + 7 + (hash % 3);
            if idx < vector.len() {
                vector[idx] = *value;
            }
        }
        
        ContentFeatures {
            vector,
            genres: content.genres.iter().map(|g| g.as_str().to_string()).collect(),
            content_type: content.content_type,
        }
    }

    /// Batch analyze multiple content items
    pub fn analyze_batch(&self, contents: &[ContentMetadata]) -> Vec<ContentFeatures> {
        contents.par_iter().map(|c| self.analyze(c)).collect()
    }

    /// Hash a key to an index
    fn hash_key(key: &str) -> usize {
        let mut hash: usize = 0;
        for c in key.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as usize);
        }
        hash
    }

    /// Get feature dimensionality
    pub fn feature_dim(&self) -> usize {
        self.feature_dim
    }

    /// Get genre index
    pub fn genre_index(&self, genre: &str) -> Option<usize> {
        self.genre_vocabulary.get(&genre.to_lowercase()).copied()
    }

    /// Get all genres
    pub fn all_genres(&self) -> Vec<&str> {
        let mut genres: Vec<_> = self.genre_vocabulary.keys().map(|s| s.as_str()).collect();
        genres.sort_by_key(|g| self.genre_vocabulary.get(*g).unwrap());
        genres
    }

    /// Extract keywords from synopsis
    pub fn extract_keywords(&self, synopsis: &str) -> Vec<String> {
        // Simple keyword extraction
        let stop_words: HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "of", "with", "by", "from", "as", "is", "was", "are", "were", "been",
            "be", "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "need", "dare", "ought",
            "used", "this", "that", "these", "those", "i", "you", "he", "she", "it",
            "we", "they", "what", "which", "who", "whom", "whose", "where", "when",
            "why", "how", "all", "each", "every", "both", "few", "more", "most",
            "other", "some", "such", "no", "not", "only", "own", "same", "so",
            "than", "too", "very", "just", "also", "now", "here", "there", "then",
        ].iter().cloned().collect();

        synopsis
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|word| word.len() > 2 && !stop_words.contains(*word))
            .map(|s| s.to_string())
            .collect()
    }

    /// Calculate content similarity based on metadata
    pub fn content_similarity(&self, a: &ContentMetadata, b: &ContentMetadata) -> f64 {
        let features_a = self.analyze(a);
        let features_b = self.analyze(b);
        features_a.cosine_similarity(&features_b)
    }

    /// Cluster content into groups
    pub fn cluster_content(
        &self,
        contents: &[(ContentId, ContentFeatures)],
        k: usize,
    ) -> Vec<ContentCluster> {
        if contents.is_empty() || k == 0 {
            return Vec::new();
        }

        let k = k.min(contents.len());
        let dim = contents[0].1.vector.len();
        
        // Initialize clusters with first k items as centroids
        let mut clusters: Vec<ContentCluster> = contents
            .iter()
            .take(k)
            .enumerate()
            .map(|(i, (_, features))| ContentCluster::new(i, features.vector.clone()))
            .collect();
        
        // Simple K-means with limited iterations
        for _ in 0..10 {
            // Clear cluster assignments
            for cluster in &mut clusters {
                cluster.content_ids.clear();
            }
            
            // Assign each content to nearest cluster
            for (content_id, features) in contents {
                let mut best_cluster = 0;
                let mut best_distance = f64::MAX;
                
                for (i, cluster) in clusters.iter().enumerate() {
                    let distance = euclidean_distance(&features.vector, &cluster.centroid);
                    if distance < best_distance {
                        best_distance = distance;
                        best_cluster = i;
                    }
                }
                
                clusters[best_cluster].add(content_id.clone());
            }
            
            // Update centroids
            for cluster in &mut clusters {
                if cluster.content_ids.is_empty() {
                    continue;
                }
                
                let content_in_cluster: Vec<_> = contents
                    .iter()
                    .filter(|(id, _)| cluster.content_ids.contains(id))
                    .collect();
                
                if !content_in_cluster.is_empty() {
                    let mut new_centroid = vec![0.0; dim];
                    for (_, features) in &content_in_cluster {
                        for (i, val) in features.vector.iter().enumerate() {
                            new_centroid[i] += val;
                        }
                    }
                    for val in &mut new_centroid {
                        *val /= content_in_cluster.len() as f64;
                    }
                    cluster.centroid = new_centroid;
                }
            }
        }
        
        clusters
    }
}

impl Default for ContentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Euclidean distance
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::MAX;
    }
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = ContentAnalyzer::new();
        assert!(analyzer.feature_dim() > 0);
    }

    #[test]
    fn test_content_analysis() {
        let analyzer = ContentAnalyzer::new();
        let content = ContentMetadata::new(
            ContentId::new("test-1"),
            "Test Movie".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_genre("Drama")
        .with_rating(8.0);

        let features = analyzer.analyze(&content);
        assert!(!features.vector.is_empty());
        assert!(features.genres.contains(&"action".to_string()));
    }

    #[test]
    fn test_feature_similarity() {
        let analyzer = ContentAnalyzer::new();
        
        let content1 = ContentMetadata::new(
            ContentId::new("1"),
            "Movie 1".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_rating(8.0);

        let content2 = ContentMetadata::new(
            ContentId::new("2"),
            "Movie 2".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_rating(7.5);

        let features1 = analyzer.analyze(&content1);
        let features2 = analyzer.analyze(&content2);
        
        let similarity = features1.cosine_similarity(&features2);
        assert!(similarity > 0.8); // Should be very similar
    }

    #[test]
    fn test_keyword_extraction() {
        let analyzer = ContentAnalyzer::new();
        let synopsis = "A young hero embarks on an epic adventure to save the world from evil forces.";
        
        let keywords = analyzer.extract_keywords(synopsis);
        assert!(keywords.contains(&"young".to_string()));
        assert!(keywords.contains(&"hero".to_string()));
        assert!(!keywords.contains(&"the".to_string())); // Stop word
    }

    #[test]
    fn test_clustering() {
        let analyzer = ContentAnalyzer::new();
        
        let contents: Vec<(ContentId, ContentFeatures)> = (0..10)
            .map(|i| {
                let content = ContentMetadata::new(
                    ContentId::new(format!("movie-{}", i)),
                    format!("Movie {}", i),
                    ContentType::Movie,
                )
                .with_genre(if i < 5 { "Action" } else { "Comedy" });
                (content.id.clone(), analyzer.analyze(&content))
            })
            .collect();

        let clusters = analyzer.cluster_content(&contents, 2);
        assert_eq!(clusters.len(), 2);
    }
}