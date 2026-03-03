//! Subtitle Synchronization Module
//!
//! This module provides advanced subtitle synchronization features including:
//! - Advanced sync algorithms
//! - Manual sync adjustment
//! - Subtitle delay support
//! - Sync presets
//! - Sync preview functionality

use anyhow::{Context, Result};
use std::time::Duration;
use tracing::{debug, info, warn};

/// Subtitle synchronization configuration
#[derive(Debug, Clone)]
pub struct SyncConfig {
    /// Enable auto-sync
    pub auto_sync: bool,
    
    /// Sync algorithm
    pub sync_algorithm: SyncAlgorithm,
    
    /// Default delay in milliseconds
    pub default_delay: i64,
    
    /// Maximum delay in milliseconds
    pub max_delay: i64,
    
    /// Minimum delay in milliseconds
    pub min_delay: i64,
    
    /// Sync sensitivity (0.0-1.0)
    pub sync_sensitivity: f32,
    
    /// Enable sync preview
    pub enable_preview: bool,
    
    /// Preview duration in seconds
    pub preview_duration: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_algorithm: SyncAlgorithm::Adaptive,
            default_delay: 0,
            max_delay: 10000, // 10 seconds
            min_delay: -10000, // -10 seconds
            sync_sensitivity: 0.5,
            enable_preview: true,
            preview_duration: 5,
        }
    }
}

/// Sync algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAlgorithm {
    /// Linear interpolation
    Linear,
    
    /// Adaptive sync
    Adaptive,
    
    /// Waveform-based sync
    Waveform,
    
    /// Speech recognition sync
    SpeechRecognition,
    
    /// Manual sync
    Manual,
}

impl SyncAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::Adaptive => "Adaptive",
            Self::Waveform => "Waveform",
            Self::SpeechRecognition => "Speech Recognition",
            Self::Manual => "Manual",
        }
    }
}

/// Subtitle sync point
#[derive(Debug, Clone)]
pub struct SyncPoint {
    /// Video timestamp
    pub video_timestamp: Duration,
    
    /// Subtitle timestamp
    pub subtitle_timestamp: Duration,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
}

/// Subtitle synchronization result
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// Applied delay in milliseconds
    pub applied_delay: i64,
    
    /// Sync algorithm used
    pub algorithm: SyncAlgorithm,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    
    /// Number of sync points used
    pub sync_points_used: usize,
    
    /// Sync quality
    pub quality: SyncQuality,
}

/// Sync quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncQuality {
    /// Excellent sync
    Excellent,
    
    /// Good sync
    Good,
    
    /// Fair sync
    Fair,
    
    /// Poor sync
    Poor,
}

impl SyncQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Excellent => "Excellent",
            Self::Good => "Good",
            Self::Fair => "Fair",
            Self::Poor => "Poor",
        }
    }
}

/// Sync preset
#[derive(Debug, Clone)]
pub struct SyncPreset {
    /// Preset name
    pub name: String,
    
    /// Delay in milliseconds
    pub delay: i64,
    
    /// Sync algorithm
    pub algorithm: SyncAlgorithm,
    
    /// Description
    pub description: String,
}

/// Subtitle synchronizer
pub struct SubtitleSynchronizer {
    /// Configuration
    config: SyncConfig,
    
    /// Current delay in milliseconds
    current_delay: i64,
    
    /// Sync points
    sync_points: Vec<SyncPoint>,
    
    /// Sync presets
    presets: Vec<SyncPreset>,
}

impl SubtitleSynchronizer {
    /// Create a new subtitle synchronizer
    pub fn new(config: SyncConfig) -> Self {
        let presets = Self::create_presets();
        
        Self {
            config,
            current_delay: config.default_delay,
            sync_points: Vec::new(),
            presets,
        }
    }
    
    /// Create sync presets
    fn create_presets() -> Vec<SyncPreset> {
        vec![
            SyncPreset {
                name: "No Delay".to_string(),
                delay: 0,
                algorithm: SyncAlgorithm::Adaptive,
                description: "No subtitle delay".to_string(),
            },
            SyncPreset {
                name: "Early 100ms".to_string(),
                delay: -100,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 100ms early".to_string(),
            },
            SyncPreset {
                name: "Early 250ms".to_string(),
                delay: -250,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 250ms early".to_string(),
            },
            SyncPreset {
                name: "Early 500ms".to_string(),
                delay: -500,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 500ms early".to_string(),
            },
            SyncPreset {
                name: "Late 100ms".to_string(),
                delay: 100,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 100ms late".to_string(),
            },
            SyncPreset {
                name: "Late 250ms".to_string(),
                delay: 250,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 250ms late".to_string(),
            },
            SyncPreset {
                name: "Late 500ms".to_string(),
                delay: 500,
                algorithm: SyncAlgorithm::Adaptive,
                description: "Subtitles appear 500ms late".to_string(),
            },
        ]
    }
    
    /// Get current delay
    pub fn current_delay(&self) -> i64 {
        self.current_delay
    }
    
    /// Set delay
    pub fn set_delay(&mut self, delay: i64) -> Result<()> {
        if delay < self.config.min_delay || delay > self.config.max_delay {
            anyhow::bail!(
                "Delay {}ms is out of range. Must be between {}ms and {}ms",
                delay,
                self.config.min_delay,
                self.config.max_delay
            );
        }
        
        info!("Setting subtitle delay to {}ms", delay);
        self.current_delay = delay;
        Ok(())
    }
    
    /// Adjust delay
    pub fn adjust_delay(&mut self, adjustment: i64) -> Result<()> {
        let new_delay = self.current_delay + adjustment;
        self.set_delay(new_delay)
    }
    
    /// Reset delay to default
    pub fn reset_delay(&mut self) {
        info!("Resetting subtitle delay to default ({}ms)", self.config.default_delay);
        self.current_delay = self.config.default_delay;
    }
    
    /// Add sync point
    pub fn add_sync_point(&mut self, video_timestamp: Duration, subtitle_timestamp: Duration, confidence: f32) {
        let sync_point = SyncPoint {
            video_timestamp,
            subtitle_timestamp,
            confidence,
        };
        
        debug!("Adding sync point: video={:?}, subtitle={:?}, confidence={}", 
               video_timestamp, subtitle_timestamp, confidence);
        
        self.sync_points.push(sync_point);
    }
    
    /// Clear sync points
    pub fn clear_sync_points(&mut self) {
        debug!("Clearing all sync points");
        self.sync_points.clear();
    }
    
    /// Get sync points
    pub fn sync_points(&self) -> &[SyncPoint] {
        &self.sync_points
    }
    
    /// Perform auto-sync
    pub fn auto_sync(&mut self) -> Result<SyncResult> {
        if self.sync_points.is_empty() {
            anyhow::bail!("No sync points available for auto-sync");
        }
        
        info!("Performing auto-sync with {} sync points", self.sync_points.len());
        
        let result = match self.config.sync_algorithm {
            SyncAlgorithm::Linear => self.linear_sync(),
            SyncAlgorithm::Adaptive => self.adaptive_sync(),
            SyncAlgorithm::Waveform => self.waveform_sync(),
            SyncAlgorithm::SpeechRecognition => self.speech_recognition_sync(),
            SyncAlgorithm::Manual => self.manual_sync(),
        };
        
        info!("Auto-sync completed: delay={}ms, quality={}", 
              result.applied_delay, result.quality.as_str());
        
        Ok(result)
    }
    
    /// Linear sync algorithm
    fn linear_sync(&mut self) -> SyncResult {
        if self.sync_points.is_empty() {
            return SyncResult {
                applied_delay: self.current_delay,
                algorithm: SyncAlgorithm::Linear,
                confidence: 0.0,
                sync_points_used: 0,
                quality: SyncQuality::Poor,
            };
        }
        
        // Calculate average delay from sync points
        let total_delay: i64 = self.sync_points
            .iter()
            .map(|sp| {
                let video_ms = sp.video_timestamp.as_millis() as i64;
                let sub_ms = sp.subtitle_timestamp.as_millis() as i64;
                sub_ms - video_ms
            })
            .sum();
        
        let avg_delay = total_delay / self.sync_points.len() as i64;
        
        // Apply delay
        self.current_delay = avg_delay;
        
        // Calculate confidence
        let confidence = self.sync_points
            .iter()
            .map(|sp| sp.confidence)
            .sum::<f32>() / self.sync_points.len() as f32;
        
        // Determine quality
        let quality = if confidence > 0.8 {
            SyncQuality::Excellent
        } else if confidence > 0.6 {
            SyncQuality::Good
        } else if confidence > 0.4 {
            SyncQuality::Fair
        } else {
            SyncQuality::Poor
        };
        
        SyncResult {
            applied_delay: avg_delay,
            algorithm: SyncAlgorithm::Linear,
            confidence,
            sync_points_used: self.sync_points.len(),
            quality,
        }
    }
    
    /// Adaptive sync algorithm
    fn adaptive_sync(&mut self) -> SyncResult {
        if self.sync_points.is_empty() {
            return SyncResult {
                applied_delay: self.current_delay,
                algorithm: SyncAlgorithm::Adaptive,
                confidence: 0.0,
                sync_points_used: 0,
                quality: SyncQuality::Poor,
            };
        }
        
        // Weighted average based on confidence
        let mut weighted_sum = 0i64;
        let mut total_weight = 0f32;
        
        for sp in &self.sync_points {
            let video_ms = sp.video_timestamp.as_millis() as i64;
            let sub_ms = sp.subtitle_timestamp.as_millis() as i64;
            let delay = sub_ms - video_ms;
            
            weighted_sum += (delay as f32 * sp.confidence) as i64;
            total_weight += sp.confidence;
        }
        
        let avg_delay = if total_weight > 0.0 {
            (weighted_sum as f32 / total_weight) as i64
        } else {
            0
        };
        
        // Apply delay
        self.current_delay = avg_delay;
        
        // Calculate confidence
        let confidence = total_weight / self.sync_points.len() as f32;
        
        // Determine quality
        let quality = if confidence > 0.8 {
            SyncQuality::Excellent
        } else if confidence > 0.6 {
            SyncQuality::Good
        } else if confidence > 0.4 {
            SyncQuality::Fair
        } else {
            SyncQuality::Poor
        };
        
        SyncResult {
            applied_delay: avg_delay,
            algorithm: SyncAlgorithm::Adaptive,
            confidence,
            sync_points_used: self.sync_points.len(),
            quality,
        }
    }
    
    /// Waveform-based sync algorithm
    fn waveform_sync(&mut self) -> SyncResult {
        // Simulated waveform sync
        // In a real implementation, this would analyze audio waveforms
        let result = self.adaptive_sync();
        
        SyncResult {
            applied_delay: result.applied_delay,
            algorithm: SyncAlgorithm::Waveform,
            confidence: result.confidence * 0.9, // Slightly lower confidence
            sync_points_used: result.sync_points_used,
            quality: result.quality,
        }
    }
    
    /// Speech recognition sync algorithm
    fn speech_recognition_sync(&mut self) -> SyncResult {
        // Simulated speech recognition sync
        // In a real implementation, this would use speech recognition
        let result = self.adaptive_sync();
        
        SyncResult {
            applied_delay: result.applied_delay,
            algorithm: SyncAlgorithm::SpeechRecognition,
            confidence: result.confidence * 0.95, // High confidence
            sync_points_used: result.sync_points_used,
            quality: result.quality,
        }
    }
    
    /// Manual sync algorithm
    fn manual_sync(&mut self) -> SyncResult {
        SyncResult {
            applied_delay: self.current_delay,
            algorithm: SyncAlgorithm::Manual,
            confidence: 1.0, // Manual sync has full confidence
            sync_points_used: 0,
            quality: SyncQuality::Good,
        }
    }
    
    /// Get sync presets
    pub fn presets(&self) -> &[SyncPreset] {
        &self.presets
    }
    
    /// Apply sync preset
    pub fn apply_preset(&mut self, preset_name: &str) -> Result<()> {
        let preset = self.presets
            .iter()
            .find(|p| p.name == preset_name)
            .context(format!("Preset '{}' not found", preset_name))?;
        
        info!("Applying sync preset: {}", preset.name);
        self.set_delay(preset.delay)?;
        self.config.sync_algorithm = preset.algorithm;
        
        Ok(())
    }
    
    /// Get configuration
    pub fn config(&self) -> &SyncConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: SyncConfig) {
        info!("Updating sync configuration");
        self.config = config;
    }
    
    /// Preview sync
    pub fn preview_sync(&self, delay: i64) -> SyncResult {
        info!("Previewing sync with delay: {}ms", delay);
        
        SyncResult {
            applied_delay: delay,
            algorithm: self.config.sync_algorithm,
            confidence: 1.0,
            sync_points_used: 0,
            quality: SyncQuality::Good,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_synchronizer_creation() {
        let config = SyncConfig::default();
        let synchronizer = SubtitleSynchronizer::new(config);
        
        assert_eq!(synchronizer.current_delay(), 0);
        assert_eq!(synchronizer.presets().len(), 7);
    }
    
    #[test]
    fn test_set_delay() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.set_delay(100).is_ok());
        assert_eq!(synchronizer.current_delay(), 100);
    }
    
    #[test]
    fn test_set_delay_out_of_range() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.set_delay(20000).is_err());
        assert!(synchronizer.set_delay(-20000).is_err());
    }
    
    #[test]
    fn test_adjust_delay() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.adjust_delay(100).is_ok());
        assert_eq!(synchronizer.current_delay(), 100);
        
        assert!(synchronizer.adjust_delay(-50).is_ok());
        assert_eq!(synchronizer.current_delay(), 50);
    }
    
    #[test]
    fn test_reset_delay() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        synchronizer.set_delay(100).unwrap();
        synchronizer.reset_delay();
        
        assert_eq!(synchronizer.current_delay(), 0);
    }
    
    #[test]
    fn test_add_sync_point() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        synchronizer.add_sync_point(Duration::from_secs(10), Duration::from_secs(10), 1.0);
        
        assert_eq!(synchronizer.sync_points().len(), 1);
    }
    
    #[test]
    fn test_clear_sync_points() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        synchronizer.add_sync_point(Duration::from_secs(10), Duration::from_secs(10), 1.0);
        synchronizer.clear_sync_points();
        
        assert_eq!(synchronizer.sync_points().len(), 0);
    }
    
    #[test]
    fn test_linear_sync() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        synchronizer.add_sync_point(Duration::from_secs(10), Duration::from_secs(10), 1.0);
        synchronizer.add_sync_point(Duration::from_secs(20), Duration::from_secs(20), 1.0);
        
        let result = synchronizer.auto_sync().unwrap();
        
        assert_eq!(result.applied_delay, 0);
        assert_eq!(result.algorithm, SyncAlgorithm::Linear);
        assert_eq!(result.sync_points_used, 2);
    }
    
    #[test]
    fn test_auto_sync_no_points() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.auto_sync().is_err());
    }
    
    #[test]
    fn test_apply_preset() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.apply_preset("Late 100ms").is_ok());
        assert_eq!(synchronizer.current_delay(), 100);
    }
    
    #[test]
    fn test_apply_preset_not_found() {
        let config = SyncConfig::default();
        let mut synchronizer = SubtitleSynchronizer::new(config);
        
        assert!(synchronizer.apply_preset("NonExistent").is_err());
    }
    
    #[test]
    fn test_preview_sync() {
        let config = SyncConfig::default();
        let synchronizer = SubtitleSynchronizer::new(config);
        
        let result = synchronizer.preview_sync(100);
        
        assert_eq!(result.applied_delay, 100);
        assert_eq!(result.quality, SyncQuality::Good);
    }
}