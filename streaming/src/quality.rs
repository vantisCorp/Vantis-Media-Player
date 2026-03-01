//! Quality selection module
//! 
//! Provides intelligent quality selection based on network conditions,
//! buffer health, and user preferences.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};

use super::adaptive::QualityLevel;

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Current bandwidth in bps
    pub current_bandwidth: u64,
    
    /// Average bandwidth in bps
    pub average_bandwidth: u64,
    
    /// Predicted bandwidth in bps
    pub predicted_bandwidth: u64,
    
    /// Buffer health (0.0 - 1.0)
    pub buffer_health: f32,
    
    /// Current quality level
    pub current_quality: QualityLevel,
}

/// Quality selector
pub struct QualitySelector {
    /// Minimum quality index
    min_quality: usize,
    
    /// Maximum quality index
    max_quality: usize,
    
    /// Bandwidth safety margin (0.0 - 1.0)
    bandwidth_safety_margin: f32,
    
    /// Buffer safety margin (0.0 - 1.0)
    buffer_safety_margin: f32,
}

impl QualitySelector {
    /// Create a new quality selector
    pub fn new() -> StreamingResult<Self> {
        Ok(Self {
            min_quality: 1, // Low
            max_quality: 5, // Ultra
            bandwidth_safety_margin: 0.2,
            buffer_safety_margin: 0.3,
        })
    }
    
    /// Select optimal quality based on metrics
    pub fn select_quality(&self, metrics: &QualityMetrics) -> StreamingResult<QualityLevel> {
        // Use predicted bandwidth for better decisions
        let effective_bandwidth = metrics.predicted_bandwidth;
        
        // Calculate effective buffer health with safety margin
        let effective_buffer = metrics.buffer_health * (1.0 - self.buffer_safety_margin);
        
        // Evaluate each quality level
        let mut best_quality = QualityLevel::Low;
        let mut best_score = f32::NEG_INFINITY;
        
        for quality_idx in self.min_quality..=self.max_quality {
            let quality = QualityLevel::from_index(quality_idx);
            let required_bandwidth = quality.typical_bitrate() as f32;
            
            // Calculate bandwidth score
            let bandwidth_score = if effective_bandwidth > 0 {
                let available_bandwidth = effective_bandwidth as f32 * (1.0 - self.bandwidth_safety_margin);
                if available_bandwidth >= required_bandwidth {
                    1.0
                } else {
                    available_bandwidth / required_bandwidth
                }
            } else {
                0.0
            };
            
            // Calculate buffer score
            let buffer_score = effective_buffer;
            
            // Calculate stability score (prefer current quality)
            let stability_score = if quality == metrics.current_quality {
                0.1
            } else {
                0.0
            };
            
            // Calculate total score
            let total_score = bandwidth_score * 0.6 + buffer_score * 0.3 + stability_score;
            
            if total_score > best_score {
                best_score = total_score;
                best_quality = quality;
            }
        }
        
        Ok(best_quality)
    }
    
    /// Set quality range
    pub fn set_quality_range(&mut self, min: QualityLevel, max: QualityLevel) {
        self.min_quality = min.index();
        self.max_quality = max.index();
    }
    
    /// Set bandwidth safety margin
    pub fn set_bandwidth_safety_margin(&mut self, margin: f32) {
        self.bandwidth_safety_margin = margin.clamp(0.0, 1.0);
    }
    
    /// Set buffer safety margin
    pub fn set_buffer_safety_margin(&mut self, margin: f32) {
        self.buffer_safety_margin = margin.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_selector_creation() {
        let selector = QualitySelector::new();
        assert!(selector.is_ok());
    }
    
    #[test]
    fn test_select_quality_high_bandwidth() {
        let selector = QualitySelector::new().unwrap();
        let metrics = QualityMetrics {
            current_bandwidth: 10_000_000,
            average_bandwidth: 10_000_000,
            predicted_bandwidth: 10_000_000,
            buffer_health: 0.9,
            current_quality: QualityLevel::Auto,
        };
        
        let quality = selector.select_quality(&metrics).unwrap();
        assert!(quality.index() >= QualityLevel::High.index());
    }
    
    #[test]
    fn test_select_quality_low_bandwidth() {
        let selector = QualitySelector::new().unwrap();
        let metrics = QualityMetrics {
            current_bandwidth: 500_000,
            average_bandwidth: 500_000,
            predicted_bandwidth: 500_000,
            buffer_health: 0.5,
            current_quality: QualityLevel::Auto,
        };
        
        let quality = selector.select_quality(&metrics).unwrap();
        assert_eq!(quality, QualityLevel::Low);
    }
}