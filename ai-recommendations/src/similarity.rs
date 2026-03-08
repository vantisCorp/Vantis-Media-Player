//! Similarity calculation algorithms

use crate::error::{RecommendationError, Result};
use crate::types::*;
use std::collections::HashMap;

/// Similarity metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityMetric {
    /// Cosine similarity
    Cosine,
    /// Euclidean distance (converted to similarity)
    Euclidean,
    /// Jaccard similarity for sets
    Jaccard,
    /// Pearson correlation
    Pearson,
    /// Manhattan distance (converted to similarity)
    Manhattan,
    /// Dot product (normalized)
    DotProduct,
}

impl Default for SimilarityMetric {
    fn default() -> Self {
        Self::Cosine
    }
}

impl SimilarityMetric {
    /// Get metric name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cosine => "cosine",
            Self::Euclidean => "euclidean",
            Self::Jaccard => "jaccard",
            Self::Pearson => "pearson",
            Self::Manhattan => "manhattan",
            Self::DotProduct => "dot_product",
        }
    }
}

/// Similarity calculator
pub struct SimilarityCalculator {
    /// Default metric
    default_metric: SimilarityMetric,
    /// Cache for computed similarities
    cache: HashMap<(String, String), f64>,
    /// Cache enabled
    cache_enabled: bool,
}

impl SimilarityCalculator {
    /// Create a new similarity calculator
    pub fn new() -> Self {
        Self {
            default_metric: SimilarityMetric::Cosine,
            cache: HashMap::new(),
            cache_enabled: true,
        }
    }

    /// Create with specific default metric
    pub fn with_metric(metric: SimilarityMetric) -> Self {
        Self {
            default_metric: metric,
            cache: HashMap::new(),
            cache_enabled: true,
        }
    }

    /// Enable or disable cache
    pub fn set_cache(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
        if !enabled {
            self.cache.clear();
        }
    }

    /// Calculate similarity between two feature vectors
    pub fn similarity(&self, a: &FeatureVector, b: &FeatureVector, metric: SimilarityMetric) -> f64 {
        match metric {
            SimilarityMetric::Cosine => self.cosine_similarity(a, b),
            SimilarityMetric::Euclidean => self.euclidean_similarity(a, b),
            SimilarityMetric::Jaccard => self.jaccard_similarity(a, b),
            SimilarityMetric::Pearson => self.pearson_similarity(a, b),
            SimilarityMetric::Manhattan => self.manhattan_similarity(a, b),
            SimilarityMetric::DotProduct => self.dot_product_similarity(a, b),
        }
    }

    /// Calculate similarity using default metric
    pub fn similarity_default(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        self.similarity(a, b, self.default_metric)
    }

    /// Cosine similarity
    fn cosine_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() == 0 {
            return 0.0;
        }

        let dot: f64 = a.data.iter().zip(&b.data).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.data.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.data.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }

    /// Euclidean distance converted to similarity
    fn euclidean_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() == 0 {
            return 0.0;
        }

        let distance: f64 = a
            .data
            .iter()
            .zip(&b.data)
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt();

        // Convert distance to similarity: 1 / (1 + distance)
        1.0 / (1.0 + distance)
    }

    /// Jaccard similarity for binary-like vectors
    fn jaccard_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() == 0 {
            return 0.0;
        }

        // Treat as binary (non-zero = 1)
        let intersection = a
            .data
            .iter()
            .zip(&b.data)
            .filter(|(x, y)| *x != 0.0 && *y != 0.0)
            .count();

        let union = a
            .data
            .iter()
            .zip(&b.data)
            .filter(|(x, y)| *x != 0.0 || *y != 0.0)
            .count();

        if union == 0 {
            return 0.0;
        }

        intersection as f64 / union as f64
    }

    /// Pearson correlation coefficient
    fn pearson_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() < 2 {
            return 0.0;
        }

        let n = a.dim() as f64;
        let mean_a: f64 = a.data.iter().sum::<f64>() / n;
        let mean_b: f64 = b.data.iter().sum::<f64>() / n;

        let cov: f64 = a
            .data
            .iter()
            .zip(&b.data)
            .map(|(x, y)| (x - mean_a) * (y - mean_b))
            .sum::<f64>()
            / n;

        let std_a: f64 = (a.data.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>() / n).sqrt();
        let std_b: f64 = (b.data.iter().map(|y| (y - mean_b).powi(2)).sum::<f64>() / n).sqrt();

        if std_a == 0.0 || std_b == 0.0 {
            return 0.0;
        }

        (cov / (std_a * std_b)).clamp(-1.0, 1.0).max(0.0)
    }

    /// Manhattan distance converted to similarity
    fn manhattan_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() == 0 {
            return 0.0;
        }

        let distance: f64 = a.data.iter().zip(&b.data).map(|(x, y)| (x - y).abs()).sum();

        1.0 / (1.0 + distance)
    }

    /// Normalized dot product
    fn dot_product_similarity(&self, a: &FeatureVector, b: &FeatureVector) -> f64 {
        if a.dim() != b.dim() || a.dim() == 0 {
            return 0.0;
        }

        let dot: f64 = a.data.iter().zip(&b.data).map(|(x, y)| x * y).sum();
        
        // Normalize by maximum possible dot product
        let max_a: f64 = a.data.iter().map(|x| x.abs()).sum();
        let max_b: f64 = b.data.iter().map(|x| x.abs()).sum();

        if max_a == 0.0 || max_b == 0.0 {
            return 0.0;
        }

        (dot / (max_a * max_b)).clamp(0.0, 1.0)
    }

    /// Calculate similarity matrix for multiple vectors
    pub fn similarity_matrix(&self, vectors: &[FeatureVector], metric: SimilarityMetric) -> Vec<Vec<f64>> {
        let n = vectors.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            matrix[i][i] = 1.0;
            for j in (i + 1)..n {
                let sim = self.similarity(&vectors[i], &vectors[j], metric);
                matrix[i][j] = sim;
                matrix[j][i] = sim;
            }
        }

        matrix
    }

    /// Find k nearest neighbors
    pub fn find_nearest(
        &self,
        query: &FeatureVector,
        candidates: &[FeatureVector],
        k: usize,
        metric: SimilarityMetric,
    ) -> Vec<(usize, f64)> {
        let mut scored: Vec<_> = candidates
            .iter()
            .enumerate()
            .map(|(i, v)| (i, self.similarity(query, v, metric)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }

    /// Calculate user similarity based on preferences
    pub fn user_similarity(&self, a: &UserProfile, b: &UserProfile) -> f64 {
        // Compare genre preferences
        let mut genre_sim = 0.0;
        let mut genre_count = 0;

        for (genre, weight_a) in &a.preferences.genre_weights {
            if let Some(weight_b) = b.preferences.genre_weights.get(genre) {
                genre_sim += 1.0 - (weight_a - weight_b).abs();
                genre_count += 1;
            }
        }

        // Compare actor preferences
        let mut actor_sim = 0.0;
        let mut actor_count = 0;

        for (actor, weight_a) in &a.preferences.actor_weights {
            if let Some(weight_b) = b.preferences.actor_weights.get(actor) {
                actor_sim += 1.0 - (weight_a - weight_b).abs();
                actor_count += 1;
            }
        }

        // Combine similarities
        let total_count = genre_count + actor_count;
        if total_count == 0 {
            return 0.0;
        }

        let combined = (genre_sim + actor_sim) / total_count as f64;
        combined.clamp(0.0, 1.0)
    }

    /// Calculate content similarity based on features
    pub fn content_similarity(&self, a: &ContentMetadata, b: &ContentMetadata) -> f64 {
        let mut scores = Vec::new();

        // Genre similarity (Jaccard)
        let genres_a: std::collections::HashSet<&str> = a.genres.iter().map(|g| g.as_str()).collect();
        let genres_b: std::collections::HashSet<&str> = b.genres.iter().map(|g| g.as_str()).collect();
        
        let intersection = genres_a.intersection(&genres_b).count();
        let union = genres_a.union(&genres_b).count();
        
        if union > 0 {
            scores.push(intersection as f64 / union as f64);
        }

        // Content type match
        if a.content_type == b.content_type {
            scores.push(1.0);
        } else {
            scores.push(0.0);
        }

        // Rating similarity
        let rating_diff = (a.rating - b.rating).abs() / 10.0;
        scores.push(1.0 - rating_diff);

        // Year similarity
        let year_diff = (a.release_year - b.release_year).abs() as f64;
        let year_sim = 1.0 / (1.0 + year_diff / 10.0);
        scores.push(year_sim);

        // Average all scores
        scores.iter().sum::<f64>() / scores.len() as f64
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for SimilarityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vector(data: Vec<f64>) -> FeatureVector {
        FeatureVector::new(data)
    }

    #[test]
    fn test_cosine_similarity() {
        let calc = SimilarityCalculator::new();
        
        let v1 = make_vector(vec![1.0, 0.0, 0.0]);
        let v2 = make_vector(vec![1.0, 0.0, 0.0]);
        let v3 = make_vector(vec![0.0, 1.0, 0.0]);
        let v4 = make_vector(vec![0.5, 0.5, 0.0]);

        // Same vector
        assert!((calc.cosine_similarity(&v1, &v2) - 1.0).abs() < 0.001);
        
        // Orthogonal
        assert!((calc.cosine_similarity(&v1, &v3) - 0.0).abs() < 0.001);
        
        // 45 degrees
        let sim = calc.cosine_similarity(&v1, &v4);
        assert!(sim > 0.7 && sim < 0.8);
    }

    #[test]
    fn test_euclidean_similarity() {
        let calc = SimilarityCalculator::new();
        
        let v1 = make_vector(vec![0.0, 0.0]);
        let v2 = make_vector(vec![1.0, 0.0]);
        let v3 = make_vector(vec![1.0, 1.0]);

        let sim12 = calc.euclidean_similarity(&v1, &v2);
        let sim13 = calc.euclidean_similarity(&v1, &v3);

        assert!(sim12 > sim13);
        assert!(sim12 > 0.4 && sim12 < 0.6);
    }

    #[test]
    fn test_jaccard_similarity() {
        let calc = SimilarityCalculator::new();
        
        let v1 = make_vector(vec![1.0, 0.0, 1.0]);
        let v2 = make_vector(vec![1.0, 1.0, 0.0]);
        let v3 = make_vector(vec![1.0, 1.0, 1.0]);

        let sim12 = calc.jaccard_similarity(&v1, &v2);
        assert!((sim12 - 0.333).abs() < 0.01);

        let sim13 = calc.jaccard_similarity(&v1, &v3);
        assert!((sim13 - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_pearson_similarity() {
        let calc = SimilarityCalculator::new();
        
        let v1 = make_vector(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let v2 = make_vector(vec![2.0, 4.0, 6.0, 8.0, 10.0]); // Perfect positive correlation
        let v3 = make_vector(vec![5.0, 4.0, 3.0, 2.0, 1.0]); // Perfect negative correlation

        let sim12 = calc.pearson_similarity(&v1, &v2);
        assert!((sim12 - 1.0).abs() < 0.001);

        let sim13 = calc.pearson_similarity(&v1, &v3);
        assert!((sim13 - 0.0).abs() < 0.001); // Negative clipped to 0
    }

    #[test]
    fn test_similarity_matrix() {
        let calc = SimilarityCalculator::new();
        
        let vectors = vec![
            make_vector(vec![1.0, 0.0]),
            make_vector(vec![0.0, 1.0]),
            make_vector(vec![1.0, 1.0]),
        ];

        let matrix = calc.similarity_matrix(&vectors, SimilarityMetric::Cosine);

        assert_eq!(matrix.len(), 3);
        assert!((matrix[0][0] - 1.0).abs() < 0.001); // Self-similarity
        assert!((matrix[0][1] - 0.0).abs() < 0.001); // Orthogonal
    }

    #[test]
    fn test_find_nearest() {
        let calc = SimilarityCalculator::new();
        
        let query = make_vector(vec![1.0, 0.0]);
        let candidates = vec![
            make_vector(vec![0.9, 0.1]),
            make_vector(vec![0.0, 1.0]),
            make_vector(vec![0.8, 0.2]),
        ];

        let nearest = calc.find_nearest(&query, &candidates, 2, SimilarityMetric::Cosine);
        
        assert_eq!(nearest.len(), 2);
        assert_eq!(nearest[0].0, 0); // First candidate is most similar
    }

    #[test]
    fn test_content_similarity() {
        let calc = SimilarityCalculator::new();
        
        let content1 = ContentMetadata::new(
            ContentId::new("1"),
            "Action Movie".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_rating(8.0);

        let content2 = ContentMetadata::new(
            ContentId::new("2"),
            "Another Action Movie".to_string(),
            ContentType::Movie,
        )
        .with_genre("Action")
        .with_rating(7.5);

        let content3 = ContentMetadata::new(
            ContentId::new("3"),
            "Comedy".to_string(),
            ContentType::Movie,
        )
        .with_genre("Comedy")
        .with_rating(8.0);

        let sim12 = calc.content_similarity(&content1, &content2);
        let sim13 = calc.content_similarity(&content1, &content3);

        assert!(sim12 > sim13);
    }
}