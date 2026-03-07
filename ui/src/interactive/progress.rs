//! Progress Indicators Module
//! 
//! Provides linear and circular progress indicators with animations.

use iced::{Color, Point, Rectangle, Size};
use std::time::{Duration, Instant};

/// Progress indicator state
#[derive(Debug, Clone)]
pub struct ProgressIndicator {
    /// Current progress (0.0 to 1.0)
    progress: f32,
    /// Target progress for animations
    target_progress: f32,
    /// Animation start time
    animation_start: Option<Instant>,
    /// Animation duration
    animation_duration: Duration,
    /// Indeterminate mode
    indeterminate: bool,
    /// Indeterminate animation position
    indeterminate_position: f32,
    /// Visual state
    pub state: ProgressState,
}

impl Default for ProgressIndicator {
    fn default() -> Self {
        Self {
            progress: 0.0,
            target_progress: 0.0,
            animation_start: None,
            animation_duration: Duration::from_millis(300),
            indeterminate: false,
            indeterminate_position: 0.0,
            state: ProgressState::Normal,
        }
    }
}

impl ProgressIndicator {
    /// Create a new progress indicator
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set progress value (0.0 to 1.0)
    pub fn set_progress(&mut self, value: f32) {
        self.target_progress = value.clamp(0.0, 1.0);
        self.animation_start = Some(Instant::now());
    }
    
    /// Set progress immediately without animation
    pub fn set_progress_immediate(&mut self, value: f32) {
        self.progress = value.clamp(0.0, 1.0);
        self.target_progress = self.progress;
        self.animation_start = None;
    }
    
    /// Get current progress
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    /// Enable indeterminate mode
    pub fn set_indeterminate(&mut self, indeterminate: bool) {
        self.indeterminate = indeterminate;
        if indeterminate {
            self.progress = 0.0;
        }
    }
    
    /// Update animation state
    pub fn update(&mut self, delta: Duration) {
        // Animate progress changes
        if let Some(start) = self.animation_start {
            let elapsed = start.elapsed();
            let t = (elapsed.as_secs_f32() / self.animation_duration.as_secs_f32()).min(1.0);
            let eased = ease_out_cubic(t);
            
            let start_progress = self.progress;
            self.progress = start_progress + (self.target_progress - start_progress) * eased;
            
            if t >= 1.0 {
                self.progress = self.target_progress;
                self.animation_start = None;
            }
        }
        
        // Update indeterminate animation
        if self.indeterminate {
            self.indeterminate_position += delta.as_secs_f32() * 2.0;
            if self.indeterminate_position > 2.0 {
                self.indeterminate_position = 0.0;
            }
        }
    }
    
    /// Check if complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

/// Progress state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// Normal progress
    Normal,
    /// Loading/buffering
    Loading,
    /// Success state
    Success,
    /// Error state
    Error,
    /// Paused
    Paused,
}

/// Circular progress indicator
#[derive(Debug, Clone)]
pub struct CircularProgress {
    /// Base progress indicator
    pub progress: ProgressIndicator,
    /// Circle radius
    pub radius: f32,
    /// Stroke width
    pub stroke_width: f32,
    /// Background color
    pub background_color: Color,
    /// Progress color
    pub progress_color: Color,
    /// Success color
    pub success_color: Color,
    /// Error color
    pub error_color: Color,
    /// Show percentage text
    pub show_percentage: bool,
}

impl CircularProgress {
    /// Create a new circular progress
    pub fn new(radius: f32) -> Self {
        Self {
            progress: ProgressIndicator::new(),
            radius,
            stroke_width: 4.0,
            background_color: Color::from_rgb(0.2, 0.2, 0.2),
            progress_color: Color::from_rgb(0.863, 0.078, 0.235), // Crimson
            success_color: Color::from_rgb(0.2, 0.8, 0.4),
            error_color: Color::from_rgb(1.0, 0.3, 0.3),
            show_percentage: true,
        }
    }
    
    /// Get the current color based on state
    pub fn current_color(&self) -> Color {
        match self.progress.state {
            ProgressState::Success => self.success_color,
            ProgressState::Error => self.error_color,
            _ => self.progress_color,
        }
    }
    
    /// Calculate the arc end point
    pub fn arc_end_point(&self) -> Point {
        let angle = self.progress.progress() * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        Point::new(
            self.radius * angle.cos(),
            self.radius * angle.sin(),
        )
    }
}

/// Linear progress bar
#[derive(Debug, Clone)]
pub struct LinearProgress {
    /// Base progress indicator
    pub progress: ProgressIndicator,
    /// Bar height
    pub height: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Background color
    pub background_color: Color,
    /// Progress color
    pub progress_color: Color,
    /// Buffer color (for streaming)
    pub buffer_color: Option<Color>,
    /// Buffer progress (0.0 to 1.0)
    pub buffer_progress: f32,
    /// Animated shimmer for loading
    pub shimmer: bool,
}

impl Default for LinearProgress {
    fn default() -> Self {
        Self {
            progress: ProgressIndicator::new(),
            height: 4.0,
            corner_radius: 2.0,
            background_color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
            progress_color: Color::from_rgb(0.863, 0.078, 0.235), // Crimson
            buffer_color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            buffer_progress: 0.0,
            shimmer: false,
        }
    }
}

impl LinearProgress {
    /// Create a new linear progress bar
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set buffer progress
    pub fn set_buffer(&mut self, progress: f32) {
        self.buffer_progress = progress.clamp(0.0, 1.0);
    }
    
    /// Enable shimmer animation
    pub fn with_shimmer(mut self, shimmer: bool) -> Self {
        self.shimmer = shimmer;
        self
    }
    
    /// Netflix-style progress bar
    pub fn netflix_style() -> Self {
        Self {
            progress: ProgressIndicator::new(),
            height: 3.0,
            corner_radius: 1.5,
            background_color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
            progress_color: Color::from_rgb(0.863, 0.078, 0.235),
            buffer_color: Some(Color::from_rgba(0.863, 0.078, 0.235, 0.4)),
            buffer_progress: 0.0,
            shimmer: false,
        }
    }
}

/// Ease out cubic function
fn ease_out_cubic(t: f32) -> f32 {
    let t1 = t - 1.0;
    t1 * t1 * t1 + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_indicator() {
        let mut progress = ProgressIndicator::new();
        assert_eq!(progress.progress(), 0.0);
        
        progress.set_progress(0.5);
        assert_eq!(progress.target_progress, 0.5);
    }
    
    #[test]
    fn test_progress_clamping() {
        let mut progress = ProgressIndicator::new();
        progress.set_progress(1.5);
        assert_eq!(progress.target_progress, 1.0);
        
        progress.set_progress(-0.5);
        assert_eq!(progress.target_progress, 0.0);
    }
    
    #[test]
    fn test_circular_progress() {
        let circular = CircularProgress::new(20.0);
        assert_eq!(circular.radius, 20.0);
    }
    
    #[test]
    fn test_linear_progress_netflix() {
        let progress = LinearProgress::netflix_style();
        assert_eq!(progress.height, 3.0);
    }
}