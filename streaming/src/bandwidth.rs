//! Bandwidth monitoring module
//! 
//! Provides real-time bandwidth monitoring and statistics for adaptive streaming.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Bandwidth statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthStats {
    /// Current bandwidth in bps
    pub current_bandwidth: u64,
    
    /// Average bandwidth in bps
    pub average_bandwidth: u64,
    
    /// Peak bandwidth in bps
    pub peak_bandwidth: u64,
    
    /// Minimum bandwidth in bps
    pub min_bandwidth: u64,
    
    /// Bandwidth variance
    pub variance: f64,
    
    /// Standard deviation
    pub std_deviation: f64,
    
    /// Number of samples
    pub sample_count: usize,
    
    /// Last update timestamp
    pub last_update: u64,
}

impl Default for BandwidthStats {
    fn default() -> Self {
        Self {
            current_bandwidth: 0,
            average_bandwidth: 0,
            peak_bandwidth: 0,
            min_bandwidth: u64::MAX,
            variance: 0.0,
            std_deviation: 0.0,
            sample_count: 0,
            last_update: 0,
        }
    }
}

/// Bandwidth monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthMonitorConfig {
    /// Measurement interval in milliseconds
    pub measurement_interval_ms: u64,
    
    /// Number of samples to keep for statistics
    pub sample_window_size: usize,
    
    /// Enable smoothing
    pub enable_smoothing: bool,
    
    /// Smoothing factor (0.0 - 1.0)
    pub smoothing_factor: f32,
    
    /// Enable prediction
    pub enable_prediction: bool,
    
    /// Prediction window size
    pub prediction_window_size: usize,
}

impl Default for BandwidthMonitorConfig {
    fn default() -> Self {
        Self {
            measurement_interval_ms: 1000,
            sample_window_size: 60,
            enable_smoothing: true,
            smoothing_factor: 0.3,
            enable_prediction: true,
            prediction_window_size: 10,
        }
    }
}

/// Bandwidth monitor
pub struct BandwidthMonitor {
    config: BandwidthMonitorConfig,
    samples: Vec<u64>,
    smoothed_bandwidth: u64,
    last_measurement: Option<Instant>,
    bytes_received: u64,
    last_bytes_received: u64,
    stats: BandwidthStats,
}

impl BandwidthMonitor {
    /// Create a new bandwidth monitor
    pub fn new() -> StreamingResult<Self> {
        Self::with_config(BandwidthMonitorConfig::default())
    }
    
    /// Create a new bandwidth monitor with custom configuration
    pub fn with_config(config: BandwidthMonitorConfig) -> StreamingResult<Self> {
        Ok(Self {
            config,
            samples: Vec::with_capacity(config.sample_window_size),
            smoothed_bandwidth: 0,
            last_measurement: None,
            bytes_received: 0,
            last_bytes_received: 0,
            stats: BandwidthStats::default(),
        })
    }
    
    /// Record bytes received
    pub fn record_bytes(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }
    
    /// Update bandwidth measurement
    pub fn update(&mut self) -> StreamingResult<BandwidthStats> {
        let now = Instant::now();
        
        // Check if enough time has passed
        if let Some(last) = self.last_measurement {
            let elapsed = now.duration_since(last);
            if elapsed < Duration::from_millis(self.config.measurement_interval_ms) {
                return Ok(self.stats.clone());
            }
        }
        
        // Calculate bandwidth
        let bytes_delta = self.bytes_received - self.last_bytes_received;
        let elapsed_secs = if let Some(last) = self.last_measurement {
            now.duration_since(last).as_secs_f64()
        } else {
            1.0
        };
        
        let bandwidth = if elapsed_secs > 0.0 {
            (bytes_delta as f64 * 8.0 / elapsed_secs) as u64
        } else {
            0
        };
        
        // Apply smoothing
        if self.config.enable_smoothing {
            self.smoothed_bandwidth = (self.smoothed_bandwidth as f32 * (1.0 - self.config.smoothing_factor)
                + bandwidth as f32 * self.config.smoothing_factor) as u64;
        } else {
            self.smoothed_bandwidth = bandwidth;
        }
        
        // Add sample
        self.samples.push(self.smoothed_bandwidth);
        
        // Maintain sample window
        if self.samples.len() > self.config.sample_window_size {
            self.samples.remove(0);
        }
        
        // Update statistics
        self.update_stats();
        
        // Update tracking
        self.last_measurement = Some(now);
        self.last_bytes_received = self.bytes_received;
        
        Ok(self.stats.clone())
    }
    
    /// Update statistics
    fn update_stats(&mut self) {
        if self.samples.is_empty() {
            return;
        }
        
        let current = *self.samples.last().unwrap();
        let sum: u64 = self.samples.iter().sum();
        let count = self.samples.len();
        
        self.stats.current_bandwidth = current;
        self.stats.average_bandwidth = sum / count as u64;
        self.stats.peak_bandwidth = *self.samples.iter().max().unwrap();
        self.stats.min_bandwidth = *self.samples.iter().min().unwrap();
        self.stats.sample_count = count;
        self.stats.last_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Calculate variance and standard deviation
        if count > 1 {
            let mean = self.stats.average_bandwidth as f64;
            let variance_sum: f64 = self.samples
                .iter()
                .map(|&x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum();
            
            self.stats.variance = variance_sum / (count - 1) as f64;
            self.stats.std_deviation = self.stats.variance.sqrt();
        }
    }
    
    /// Get current statistics
    pub fn stats(&self) -> &BandwidthStats {
        &self.stats
    }
    
    /// Get current bandwidth
    pub fn current_bandwidth(&self) -> u64 {
        self.stats.current_bandwidth
    }
    
    /// Get average bandwidth
    pub fn average_bandwidth(&self) -> u64 {
        self.stats.average_bandwidth
    }
    
    /// Predict future bandwidth
    pub fn predict_bandwidth(&self) -> u64 {
        if !self.config.enable_prediction || self.samples.len() < 3 {
            return self.stats.current_bandwidth;
        }
        
        let window_size = self.config.prediction_window_size.min(self.samples.len());
        let recent: Vec<_> = self.samples.iter().rev().take(window_size).collect();
        
        // Simple linear prediction
        let trend: i64 = recent.iter()
            .zip(recent.iter().skip(1))
            .map(|(a, b)| *a as i64 - *b as i64)
            .sum::<i64>() / (window_size - 1) as i64;
        
        let predicted = self.stats.current_bandwidth as i64 + trend;
        predicted.max(0) as u64
    }
    
    /// Get bandwidth stability score (0.0 - 1.0)
    pub fn stability_score(&self) -> f32 {
        if self.stats.sample_count < 2 {
            return 1.0;
        }
        
        let cv = if self.stats.average_bandwidth > 0 {
            self.stats.std_deviation / self.stats.average_bandwidth as f64
        } else {
            0.0
        };
        
        // Lower coefficient of variation = higher stability
        (1.0 - cv.min(1.0)) as f32
    }
    
    /// Reset monitor
    pub fn reset(&mut self) {
        self.samples.clear();
        self.smoothed_bandwidth = 0;
        self.last_measurement = None;
        self.bytes_received = 0;
        self.last_bytes_received = 0;
        self.stats = BandwidthStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bandwidth_monitor_creation() {
        let monitor = BandwidthMonitor::new();
        assert!(monitor.is_ok());
    }
    
    #[test]
    fn test_record_bytes() {
        let mut monitor = BandwidthMonitor::new().unwrap();
        monitor.record_bytes(1024);
        assert_eq!(monitor.bytes_received, 1024);
    }
    
    #[test]
    fn test_update() {
        let mut monitor = BandwidthMonitor::new().unwrap();
        monitor.record_bytes(1024 * 1024); // 1 MB
        let stats = monitor.update().unwrap();
        assert!(stats.current_bandwidth > 0);
    }
    
    #[test]
    fn test_stability_score() {
        let monitor = BandwidthMonitor::new().unwrap();
        let score = monitor.stability_score();
        assert!(score >= 0.0 && score <= 1.0);
    }
}