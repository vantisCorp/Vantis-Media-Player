//! Application metrics collection
//!
//! Provides performance and usage metrics for monitoring.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use parking_lot::RwLock;

/// Application metrics
#[derive(Debug, Default)]
pub struct Metrics {
    /// Total media files played
    pub media_played: AtomicU64,
    /// Total playback time in seconds
    pub playback_time_seconds: AtomicU64,
    /// Errors encountered
    pub errors: AtomicU64,
    /// Plugins loaded
    pub plugins_loaded: AtomicU64,
    /// Crashes
    pub crashes: AtomicU64,
    /// Custom metrics
    custom: RwLock<HashMap<String, AtomicU64>>,
    /// Timing metrics
    timings: RwLock<HashMap<String, Vec<f64>>>,
}

impl Metrics {
    /// Create new metrics instance
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Increment a counter
    pub fn increment(&self, counter: &Counter) {
        match counter {
            Counter::MediaPlayed => self.media_played.fetch_add(1, Ordering::Relaxed),
            Counter::Errors => self.errors.fetch_add(1, Ordering::Relaxed),
            Counter::PluginsLoaded => self.plugins_loaded.fetch_add(1, Ordering::Relaxed),
            Counter::Crashes => self.crashes.fetch_add(1, Ordering::Relaxed),
        };
    }

    /// Add playback time
    pub fn add_playback_time(&self, seconds: u64) {
        self.playback_time_seconds.fetch_add(seconds, Ordering::Relaxed);
    }

    /// Record a timing
    pub fn record_timing(&self, operation: &str, duration_ms: f64) {
        let mut timings = self.timings.write();
        timings
            .entry(operation.to_string())
            .or_insert_with(Vec::new)
            .push(duration_ms);
    }

    /// Time an operation
    pub fn time<F, R>(&self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        self.record_timing(operation, duration);
        result
    }

    /// Set a custom metric
    pub fn set_custom(&self, name: &str, value: u64) {
        let mut custom = self.custom.write();
        custom
            .entry(name.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .store(value, Ordering::Relaxed);
    }

    /// Increment a custom metric
    pub fn increment_custom(&self, name: &str) {
        let mut custom = self.custom.write();
        custom
            .entry(name.to_string())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Get all metrics as a snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let custom: HashMap<String, u64> = self
            .custom
            .read()
            .iter()
            .map(|(k, v)| (k.clone(), v.load(Ordering::Relaxed)))
            .collect();

        let timings: HashMap<String, TimingStats> = self
            .timings
            .read()
            .iter()
            .map(|(k, v)| {
                let stats = if v.is_empty() {
                    TimingStats::default()
                } else {
                    let sum: f64 = v.iter().sum();
                    let count = v.len() as f64;
                    let avg = sum / count;
                    let min = v.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max = v.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    
                    TimingStats { avg, min, max, count: count as u64 }
                };
                (k.clone(), stats)
            })
            .collect();

        MetricsSnapshot {
            media_played: self.media_played.load(Ordering::Relaxed),
            playback_time_seconds: self.playback_time_seconds.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            plugins_loaded: self.plugins_loaded.load(Ordering::Relaxed),
            crashes: self.crashes.load(Ordering::Relaxed),
            custom,
            timings,
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.media_played.store(0, Ordering::Relaxed);
        self.playback_time_seconds.store(0, Ordering::Relaxed);
        self.errors.store(0, Ordering::Relaxed);
        self.plugins_loaded.store(0, Ordering::Relaxed);
        self.crashes.store(0, Ordering::Relaxed);
        self.custom.write().clear();
        self.timings.write().clear();
    }
}

/// Counter types
#[derive(Debug, Clone, Copy)]
pub enum Counter {
    MediaPlayed,
    Errors,
    PluginsLoaded,
    Crashes,
}

/// Snapshot of metrics
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsSnapshot {
    pub media_played: u64,
    pub playback_time_seconds: u64,
    pub errors: u64,
    pub plugins_loaded: u64,
    pub crashes: u64,
    pub custom: HashMap<String, u64>,
    pub timings: HashMap<String, TimingStats>,
}

/// Timing statistics
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TimingStats {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_increment() {
        let metrics = Metrics::new();
        metrics.increment(&Counter::MediaPlayed);
        metrics.increment(&Counter::MediaPlayed);
        
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.media_played, 2);
    }

    #[test]
    fn test_metrics_timing() {
        let metrics = Metrics::new();
        metrics.time("test_op", || {
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
        
        let snapshot = metrics.snapshot();
        assert!(snapshot.timings.contains_key("test_op"));
        assert!(snapshot.timings["test_op"].avg > 0.0);
    }

    #[test]
    fn test_metrics_custom() {
        let metrics = Metrics::new();
        metrics.increment_custom("custom_metric");
        metrics.increment_custom("custom_metric");
        
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.custom.get("custom_metric"), Some(&2));
    }
}