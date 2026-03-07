//! Debug Overlay Module
//! 
//! Provides an in-app debug overlay for development builds.

use iced::{Color, Point, Rectangle, Size};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Debug overlay display
#[derive(Debug, Clone)]
pub struct DebugOverlay {
    /// Is overlay visible
    pub visible: bool,
    /// Overlay position
    pub position: Point,
    /// Overlay size
    pub size: Size,
    /// Background opacity
    pub opacity: f32,
    /// Show FPS
    pub show_fps: bool,
    /// Show memory
    pub show_memory: bool,
    /// Show frame times
    pub show_frame_times: bool,
    /// Frame time history for graph
    frame_times: VecDeque<f32>,
    /// Max frame time history
    max_history: usize,
    /// Last update time
    last_update: Instant,
    /// FPS calculation
    fps_samples: VecDeque<Instant>,
    /// Current FPS
    current_fps: f32,
    /// Debug log messages
    log_messages: VecDeque<LogMessage>,
    /// Max log messages
    max_log_messages: usize,
}

/// Log message entry
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: Instant,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Get color for log level
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Error => Color::from_rgb(1.0, 0.3, 0.3),
            LogLevel::Warn => Color::from_rgb(1.0, 0.8, 0.0),
            LogLevel::Info => Color::from_rgb(0.3, 0.8, 1.0),
            LogLevel::Debug => Color::from_rgb(0.7, 0.7, 0.7),
            LogLevel::Trace => Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self {
            visible: false,
            position: Point::new(10.0, 10.0),
            size: Size::new(300.0, 200.0),
            opacity: 0.9,
            show_fps: true,
            show_memory: true,
            show_frame_times: true,
            frame_times: VecDeque::with_capacity(100),
            max_history: 100,
            last_update: Instant::now(),
            fps_samples: VecDeque::with_capacity(60),
            current_fps: 0.0,
            log_messages: VecDeque::with_capacity(50),
            max_log_messages: 50,
        }
    }
}

impl DebugOverlay {
    /// Create a new debug overlay
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    /// Show the overlay
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    /// Hide the overlay
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// Record frame time
    pub fn record_frame(&mut self, frame_time_ms: f32) {
        self.frame_times.push_back(frame_time_ms);
        if self.frame_times.len() > self.max_history {
            self.frame_times.pop_front();
        }
        
        // Update FPS
        let now = Instant::now();
        self.fps_samples.push_back(now);
        
        // Remove samples older than 1 second
        while let Some(front) = self.fps_samples.front() {
            if now.duration_since(*front) > Duration::from_secs(1) {
                self.fps_samples.pop_front();
            } else {
                break;
            }
        }
        
        self.current_fps = self.fps_samples.len() as f32;
    }
    
    /// Get current FPS
    pub fn fps(&self) -> f32 {
        self.current_fps
    }
    
    /// Get average frame time
    pub fn avg_frame_time(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
    }
    
    /// Get max frame time
    pub fn max_frame_time(&self) -> f32 {
        self.frame_times.iter().cloned().fold(0.0, f32::max)
    }
    
    /// Get min frame time
    pub fn min_frame_time(&self) -> f32 {
        self.frame_times.iter().cloned().fold(f32::MAX, f32::min)
    }
    
    /// Add log message
    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.log_messages.push_back(LogMessage {
            level,
            message: message.into(),
            timestamp: Instant::now(),
        });
        
        if self.log_messages.len() > self.max_log_messages {
            self.log_messages.pop_front();
        }
    }
    
    /// Clear log messages
    pub fn clear_logs(&mut self) {
        self.log_messages.clear();
    }
    
    /// Get memory usage (approximate)
    pub fn memory_usage_mb(&self) -> f64 {
        // This is a rough approximation
        // In production, use proper memory profiling
        #[cfg(target_os = "linux")]
        {
            if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(kb) = parts[1].parse::<f64>() {
                                return kb / 1024.0;
                            }
                        }
                    }
                }
            }
        }
        0.0
    }
    
    /// Generate debug info string
    pub fn debug_info(&self) -> String {
        format!(
            "FPS: {:.1}\nFrame Time: {:.2}ms (avg: {:.2}ms)\nMemory: {:.1} MB",
            self.current_fps,
            self.frame_times.back().unwrap_or(&0.0),
            self.avg_frame_time(),
            self.memory_usage_mb()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_overlay_default() {
        let overlay = DebugOverlay::new();
        assert!(!overlay.visible);
    }
    
    #[test]
    fn test_overlay_toggle() {
        let mut overlay = DebugOverlay::new();
        overlay.toggle();
        assert!(overlay.visible);
        overlay.toggle();
        assert!(!overlay.visible);
    }
    
    #[test]
    fn test_fps_calculation() {
        let mut overlay = DebugOverlay::new();
        overlay.record_frame(16.67);
        overlay.record_frame(16.67);
        overlay.record_frame(16.67);
        
        assert!(overlay.fps() > 0.0);
        assert!((overlay.avg_frame_time() - 16.67).abs() < 0.01);
    }
    
    #[test]
    fn test_log_messages() {
        let mut overlay = DebugOverlay::new();
        overlay.log(LogLevel::Info, "Test message");
        
        assert_eq!(overlay.log_messages.len(), 1);
    }
}