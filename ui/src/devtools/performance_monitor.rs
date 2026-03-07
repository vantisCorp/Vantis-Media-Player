//! Performance Monitor Module
//! 
//! Tracks performance metrics for development builds.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance monitor for tracking frame times and resource usage
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Frame time history (milliseconds)
    frame_times: VecDeque<f32>,
    /// Maximum history size
    max_history: usize,
    /// FPS samples for calculation
    fps_samples: VecDeque<Instant>,
    /// Current calculated FPS
    current_fps: f32,
    /// Average FPS over time
    avg_fps: f32,
    /// Frame time percentiles
    percentiles: Percentiles,
    /// Memory samples
    memory_samples: VecDeque<MemorySample>,
    /// GPU timing (if available)
    gpu_times: VecDeque<f32>,
    /// Last update time
    last_update: Instant,
    /// Update interval
    update_interval: Duration,
}

/// Frame time percentiles
#[derive(Debug, Clone, Copy, Default)]
pub struct Percentiles {
    pub p50: f32,
    pub p90: f32,
    pub p95: f32,
    pub p99: f32,
}

/// Memory usage sample
#[derive(Debug, Clone)]
pub struct MemorySample {
    pub timestamp: Instant,
    pub used_mb: f64,
    pub available_mb: f64,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(1000),
            max_history: 1000,
            fps_samples: VecDeque::with_capacity(60),
            current_fps: 0.0,
            avg_fps: 0.0,
            percentiles: Percentiles::default(),
            memory_samples: VecDeque::with_capacity(100),
            gpu_times: VecDeque::with_capacity(100),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a frame time
    pub fn record_frame(&mut self, frame_time_ms: f32) {
        self.frame_times.push_back(frame_time_ms);
        if self.frame_times.len() > self.max_history {
            self.frame_times.pop_front();
        }
        
        // Update FPS
        let now = Instant::now();
        self.fps_samples.push_back(now);
        
        // Remove old samples
        while let Some(front) = self.fps_samples.front() {
            if now.duration_since(*front) > Duration::from_secs(1) {
                self.fps_samples.pop_front();
            } else {
                break;
            }
        }
        
        self.current_fps = self.fps_samples.len() as f32;
        
        // Update periodically
        if now.duration_since(self.last_update) >= self.update_interval {
            self.update_metrics();
            self.last_update = now;
        }
    }
    
    /// Update calculated metrics
    fn update_metrics(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }
        
        // Calculate average FPS
        let avg_frame_time: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        self.avg_fps = if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        };
        
        // Calculate percentiles
        let mut sorted: Vec<f32> = self.frame_times.iter().cloned().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let len = sorted.len();
        self.percentiles.p50 = sorted[len / 2];
        self.percentiles.p90 = sorted[(len as f32 * 0.9) as usize];
        self.percentiles.p95 = sorted[(len as f32 * 0.95) as usize];
        self.percentiles.p99 = sorted[(len as f32 * 0.99) as usize];
    }
    
    /// Record memory usage
    pub fn record_memory(&mut self, used_mb: f64, available_mb: f64) {
        self.memory_samples.push_back(MemorySample {
            timestamp: Instant::now(),
            used_mb,
            available_mb,
        });
        
        if self.memory_samples.len() > 100 {
            self.memory_samples.pop_front();
        }
    }
    
    /// Record GPU time
    pub fn record_gpu_time(&mut self, gpu_time_ms: f32) {
        self.gpu_times.push_back(gpu_time_ms);
        if self.gpu_times.len() > 100 {
            self.gpu_times.pop_front();
        }
    }
    
    /// Get current FPS
    pub fn fps(&self) -> f32 {
        self.current_fps
    }
    
    /// Get average FPS
    pub fn avg_fps(&self) -> f32 {
        self.avg_fps
    }
    
    /// Get current frame time
    pub fn current_frame_time(&self) -> f32 {
        *self.frame_times.back().unwrap_or(&0.0)
    }
    
    /// Get average frame time
    pub fn avg_frame_time(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
    }
    
    /// Get frame time percentiles
    pub fn percentiles(&self) -> &Percentiles {
        &self.percentiles
    }
    
    /// Get frame time history
    pub fn frame_time_history(&self) -> &VecDeque<f32> {
        &self.frame_times
    }
    
    /// Check if performance is good (60+ FPS)
    pub fn is_performance_good(&self) -> bool {
        self.current_fps >= 60.0
    }
    
    /// Check if performance is acceptable (30+ FPS)
    pub fn is_performance_acceptable(&self) -> bool {
        self.current_fps >= 30.0
    }
    
    /// Get performance rating
    pub fn performance_rating(&self) -> PerformanceRating {
        if self.current_fps >= 60.0 {
            PerformanceRating::Excellent
        } else if self.current_fps >= 45.0 {
            PerformanceRating::Good
        } else if self.current_fps >= 30.0 {
            PerformanceRating::Acceptable
        } else {
            PerformanceRating::Poor
        }
    }
    
    /// Clear all history
    pub fn clear(&mut self) {
        self.frame_times.clear();
        self.fps_samples.clear();
        self.memory_samples.clear();
        self.gpu_times.clear();
    }
}

/// Performance rating
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceRating {
    Excellent,  // 60+ FPS
    Good,       // 45-60 FPS
    Acceptable, // 30-45 FPS
    Poor,       // <30 FPS
}

impl PerformanceRating {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            PerformanceRating::Excellent => "Excellent",
            PerformanceRating::Good => "Good",
            PerformanceRating::Acceptable => "Acceptable",
            PerformanceRating::Poor => "Poor",
        }
    }
    
    /// Get color for rating
    pub fn color(&self) -> iced::Color {
        match self {
            PerformanceRating::Excellent => iced::Color::from_rgb(0.2, 0.8, 0.4),
            PerformanceRating::Good => iced::Color::from_rgb(0.5, 0.8, 0.2),
            PerformanceRating::Acceptable => iced::Color::from_rgb(1.0, 0.8, 0.2),
            PerformanceRating::Poor => iced::Color::from_rgb(1.0, 0.3, 0.3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        monitor.record_frame(16.67);
        assert!(monitor.current_frame_time() > 0.0);
    }
    
    #[test]
    fn test_fps_calculation() {
        let mut monitor = PerformanceMonitor::new();
        
        // Record 60 frames at 60 FPS
        for _ in 0..60 {
            monitor.record_frame(16.67);
        }
        
        assert!(monitor.fps() > 0.0);
    }
    
    #[test]
    fn test_performance_rating() {
        let mut monitor = PerformanceMonitor::new();
        
        // Simulate 60 FPS
        for _ in 0..60 {
            monitor.record_frame(16.67);
        }
        
        assert!(monitor.is_performance_good());
    }
}