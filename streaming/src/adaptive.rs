//! Adaptive streaming module
//! 
//! Provides adaptive bitrate streaming (ABR) with automatic quality selection
//! based on network conditions and buffer health.

use crate::{StreamingConfig, StreamingError, StreamingResult, StreamStats};
use crate::quality::{QualityLevel, QualitySelector, QualityMetrics};
use crate::bandwidth::BandwidthMonitor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Adaptive streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    /// Enable adaptive quality selection
    pub enabled: bool,
    
    /// Minimum quality level
    pub min_quality: QualityLevel,
    
    /// Maximum quality level
    pub max_quality: QualityLevel,
    
    /// Buffer health threshold for quality upgrade (0.0 - 1.0)
    pub buffer_upgrade_threshold: f32,
    
    /// Buffer health threshold for quality downgrade (0.0 - 1.0)
    pub buffer_downgrade_threshold: f32,
    
    /// Bandwidth threshold for quality upgrade (in bps)
    pub bandwidth_upgrade_threshold: u64,
    
    /// Bandwidth threshold for quality downgrade (in bps)
    pub bandwidth_downgrade_threshold: u64,
    
    /// Minimum time between quality changes (in seconds)
    pub min_quality_change_interval_secs: f64,
    
    /// Enable predictive quality selection
    pub enable_prediction: bool,
    
    /// Prediction window size (number of samples)
    pub prediction_window_size: usize,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_quality: QualityLevel::Low,
            max_quality: QualityLevel::Ultra,
            buffer_upgrade_threshold: 0.8,
            buffer_downgrade_threshold: 0.3,
            bandwidth_upgrade_threshold: 5_000_000, // 5 Mbps
            bandwidth_downgrade_threshold: 1_000_000, // 1 Mbps
            min_quality_change_interval_secs: 10.0,
            enable_prediction: true,
            prediction_window_size: 10,
        }
    }
}

/// Quality representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualityLevel {
    /// Auto (automatic selection)
    Auto,
    /// Low quality (360p, 500 kbps)
    Low,
    /// Medium quality (480p, 1 Mbps)
    Medium,
    /// High quality (720p, 2.5 Mbps)
    High,
    /// Full HD (1080p, 5 Mbps)
    FullHD,
    /// Ultra HD (4K, 20 Mbps)
    Ultra,
}

impl QualityLevel {
    /// Get quality level from index
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => QualityLevel::Low,
            1 => QualityLevel::Medium,
            2 => QualityLevel::High,
            3 => QualityLevel::FullHD,
            4 => QualityLevel::Ultra,
            _ => QualityLevel::Auto,
        }
    }
    
    /// Get index of quality level
    pub fn index(&self) -> usize {
        match self {
            QualityLevel::Auto => 0,
            QualityLevel::Low => 1,
            QualityLevel::Medium => 2,
            QualityLevel::High => 3,
            QualityLevel::FullHD => 4,
            QualityLevel::Ultra => 5,
        }
    }
    
    /// Get typical bitrate for this quality
    pub fn typical_bitrate(&self) -> u64 {
        match self {
            QualityLevel::Auto => 0,
            QualityLevel::Low => 500_000,
            QualityLevel::Medium => 1_000_000,
            QualityLevel::High => 2_500_000,
            QualityLevel::FullHD => 5_000_000,
            QualityLevel::Ultra => 20_000_000,
        }
    }
    
    /// Get typical resolution for this quality
    pub fn typical_resolution(&self) -> (u32, u32) {
        match self {
            QualityLevel::Auto => (0, 0),
            QualityLevel::Low => (640, 360),
            QualityLevel::Medium => (854, 480),
            QualityLevel::High => (1280, 720),
            QualityLevel::FullHD => (1920, 1080),
            QualityLevel::Ultra => (3840, 2160),
        }
    }
}

/// Adaptive streamer
pub struct AdaptiveStreamer {
    config: AdaptiveConfig,
    quality_selector: QualitySelector,
    current_quality: QualityLevel,
    last_quality_change: Option<Instant>,
    bandwidth_history: Vec<u64>,
    buffer_health_history: Vec<f32>,
}

impl AdaptiveStreamer {
    /// Create a new adaptive streamer
    pub fn new() -> StreamingResult<Self> {
        Self::with_config(AdaptiveConfig::default())
    }
    
    /// Create a new adaptive streamer with custom configuration
    pub fn with_config(config: AdaptiveConfig) -> StreamingResult<Self> {
        Ok(Self {
            quality_selector: QualitySelector::new()?,
            current_quality: QualityLevel::Auto,
            last_quality_change: None,
            bandwidth_history: Vec::new(),
            buffer_health_history: Vec::new(),
            config,
        })
    }
    
    /// Select optimal quality based on current conditions
    pub fn select_quality(
        &mut self,
        bandwidth: u64,
        buffer_health: f32,
    ) -> StreamingResult<QualityLevel> {
        if !self.config.enabled {
            return Ok(QualityLevel::Auto);
        }
        
        // Update history
        self.bandwidth_history.push(bandwidth);
        self.buffer_health_history.push(buffer_health);
        
        // Maintain history size
        if self.bandwidth_history.len() > self.config.prediction_window_size {
            self.bandwidth_history.remove(0);
            self.buffer_health_history.remove(0);
        }
        
        // Check if enough time has passed since last quality change
        if let Some(last_change) = self.last_quality_change {
            let elapsed = last_change.elapsed().as_secs_f64();
            if elapsed < self.config.min_quality_change_interval_secs {
                return Ok(self.current_quality);
            }
        }
        
        // Calculate quality metrics
        let metrics = QualityMetrics {
            current_bandwidth: bandwidth,
            average_bandwidth: self.calculate_average_bandwidth(),
            predicted_bandwidth: if self.config.enable_prediction {
                self.predict_bandwidth()
            } else {
                bandwidth
            },
            buffer_health,
            current_quality: self.current_quality,
        };
        
        // Select quality
        let selected_quality = self.quality_selector.select_quality(&metrics)?;
        
        // Clamp to min/max
        let clamped_quality = self.clamp_quality(selected_quality);
        
        // Update current quality if changed
        if clamped_quality != self.current_quality {
            self.current_quality = clamped_quality;
            self.last_quality_change = Some(Instant::now());
        }
        
        Ok(self.current_quality)
    }
    
    /// Calculate average bandwidth
    fn calculate_average_bandwidth(&self) -> u64 {
        if self.bandwidth_history.is_empty() {
            return 0;
        }
        
        let sum: u64 = self.bandwidth_history.iter().sum();
        sum / self.bandwidth_history.len() as u64
    }
    
    /// Predict future bandwidth
    fn predict_bandwidth(&self) -> u64 {
        if self.bandwidth_history.len() < 3 {
            return *self.bandwidth_history.last().unwrap_or(&0);
        }
        
        // Simple linear prediction
        let recent: Vec<_> = self.bandwidth_history.iter().rev().take(3).collect();
        let trend = (recent[0] - recent[1]) + (recent[1] - recent[2]);
        let predicted = recent[0] + trend / 2;
        
        predicted.max(0)
    }
    
    /// Clamp quality to min/max range
    fn clamp_quality(&self, quality: QualityLevel) -> QualityLevel {
        let min_idx = self.config.min_quality.index();
        let max_idx = self.config.max_quality.index();
        let current_idx = quality.index();
        
        let clamped_idx = current_idx.clamp(min_idx, max_idx);
        QualityLevel::from_index(clamped_idx)
    }
    
    /// Get current quality
    pub fn current_quality(&self) -> QualityLevel {
        self.current_quality
    }
    
    /// Force set quality
    pub fn set_quality(&mut self, quality: QualityLevel) {
        self.current_quality = self.clamp_quality(quality);
        self.last_quality_change = Some(Instant::now());
    }
    
    /// Reset to auto quality
    pub fn reset_to_auto(&mut self) {
        self.current_quality = QualityLevel::Auto;
        self.last_quality_change = None;
    }
    
    /// Get quality statistics
    pub fn get_quality_stats(&self) -> QualityStats {
        QualityStats {
            current_quality: self.current_quality,
            quality_changes: self.bandwidth_history.len(),
            average_bandwidth: self.calculate_average_bandwidth(),
            buffer_health: *self.buffer_health_history.last().unwrap_or(&1.0),
        }
    }
}

/// Quality statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityStats {
    pub current_quality: QualityLevel,
    pub quality_changes: usize,
    pub average_bandwidth: u64,
    pub buffer_health: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_level_bitrate() {
        assert_eq!(QualityLevel::Low.typical_bitrate(), 500_000);
        assert_eq!(QualityLevel::FullHD.typical_bitrate(), 5_000_000);
    }
    
    #[test]
    fn test_quality_level_resolution() {
        assert_eq!(QualityLevel::High.typical_resolution(), (1280, 720));
        assert_eq!(QualityLevel::Ultra.typical_resolution(), (3840, 2160));
    }
    
    #[test]
    fn test_adaptive_streamer_creation() {
        let streamer = AdaptiveStreamer::new();
        assert!(streamer.is_ok());
    }
    
    #[test]
    fn test_select_quality() {
        let mut streamer = AdaptiveStreamer::new().unwrap();
        let quality = streamer.select_quality(3_000_000, 0.9).unwrap();
        assert!(quality != QualityLevel::Auto);
    }
}