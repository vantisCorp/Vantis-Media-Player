//! Animation Module
//! 
//! Provides animation primitives and utilities for interactive elements.

use std::time::{Duration, Instant};

/// Animation state
#[derive(Debug, Clone)]
pub struct Animation {
    /// Animation duration
    pub duration: Duration,
    /// Easing function
    pub easing: EasingFunction,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Animation start time
    start_time: Option<Instant>,
    /// Whether animation is playing
    pub is_playing: bool,
    /// Whether animation loops
    pub looping: bool,
    /// Animation direction
    pub direction: AnimationDirection,
    /// Reverse on completion (for ping-pong)
    pub auto_reverse: bool,
}

impl Animation {
    /// Create a new animation
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: EasingFunction::EaseOutCubic,
            progress: 0.0,
            start_time: None,
            is_playing: false,
            looping: false,
            direction: AnimationDirection::Forward,
            auto_reverse: false,
        }
    }
    
    /// Create a fast animation (200ms)
    pub fn fast() -> Self {
        Self::new(Duration::from_millis(200))
    }
    
    /// Create a normal animation (300ms)
    pub fn normal() -> Self {
        Self::new(Duration::from_millis(300))
    }
    
    /// Create a slow animation (500ms)
    pub fn slow() -> Self {
        Self::new(Duration::from_millis(500))
    }
    
    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
    
    /// Enable looping
    pub fn with_loop(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }
    
    /// Enable auto-reverse
    pub fn with_auto_reverse(mut self, auto_reverse: bool) -> Self {
        self.auto_reverse = auto_reverse;
        self
    }
    
    /// Start the animation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_playing = true;
        self.progress = 0.0;
        self.direction = AnimationDirection::Forward;
    }
    
    /// Start from current position (resume)
    pub fn resume(&mut self) {
        if !self.is_playing {
            self.start_time = Some(Instant::now());
            self.is_playing = true;
        }
    }
    
    /// Pause the animation
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    /// Stop and reset the animation
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.progress = 0.0;
        self.start_time = None;
    }
    
    /// Update animation state
    pub fn update(&mut self) {
        if !self.is_playing {
            return;
        }
        
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
            
            let adjusted_t = match self.direction {
                AnimationDirection::Forward => t,
                AnimationDirection::Backward => 1.0 - t,
            };
            
            self.progress = self.easing.apply(adjusted_t);
            
            if t >= 1.0 {
                if self.auto_reverse {
                    // Ping-pong: reverse direction
                    self.direction = match self.direction {
                        AnimationDirection::Forward => AnimationDirection::Backward,
                        AnimationDirection::Backward => AnimationDirection::Forward,
                    };
                    self.start_time = Some(Instant::now());
                } else if self.looping {
                    // Loop: restart
                    self.start_time = Some(Instant::now());
                    self.progress = 0.0;
                } else {
                    // Complete
                    self.is_playing = false;
                    self.progress = if matches!(self.direction, AnimationDirection::Forward) {
                        1.0
                    } else {
                        0.0
                    };
                }
            }
        }
    }
    
    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        !self.is_playing && self.progress >= 1.0
    }
    
    /// Get eased progress value
    pub fn eased_progress(&self) -> f32 {
        self.easing.apply(self.progress)
    }
}

/// Animation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    Forward,
    Backward,
}

/// Easing functions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseOutElastic,
    EaseOutBounce,
    // Custom cubic-bezier
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    /// Apply easing function to a value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            
            EasingFunction::EaseInQuad => t * t,
            EasingFunction::EaseOutQuad => t * (2.0 - t),
            EasingFunction::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
            }
            
            EasingFunction::EaseInCubic => t * t * t,
            EasingFunction::EaseOutCubic => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFunction::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = 2.0 * t - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
            
            EasingFunction::EaseInQuart => t * t * t * t,
            EasingFunction::EaseOutQuart => {
                let t1 = t - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
            EasingFunction::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
            
            EasingFunction::EaseInExpo => {
                if t == 0.0 { 0.0 } else { (2.0f32).powf(10.0 * (t - 1.0)) }
            }
            EasingFunction::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - (2.0f32).powf(-10.0 * t) }
            }
            EasingFunction::EaseInOutExpo => {
                if t == 0.0 || t == 1.0 {
                    t
                } else if t < 0.5 {
                    (2.0f32).powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - (2.0f32).powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            
            EasingFunction::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            EasingFunction::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                let t1 = t - 1.0;
                1.0 + c3 * t1 * t1 * t1 + c1 * t1 * t1
            }
            EasingFunction::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    let t1 = 2.0 * t - 2.0;
                    (t1.powi(2) * ((c2 + 1.0) * t1 + c2) + 2.0) / 2.0
                }
            }
            
            EasingFunction::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    (2.0f32).powf(-10.0 * t) * ((t - p / 4.0) * std::f32::consts::TAU / p).sin() + 1.0
                }
            }
            
            EasingFunction::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t1 = t - 1.5 / d1;
                    n1 * t1 * t1 + 0.75
                } else if t < 2.5 / d1 {
                    let t1 = t - 2.25 / d1;
                    n1 * t1 * t1 + 0.9375
                } else {
                    let t1 = t - 2.625 / d1;
                    n1 * t1 * t1 + 0.984375
                }
            }
            
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                // Simplified cubic-bezier approximation
                // For production, use a proper bezier solver
                let t2 = t * t;
                let t3 = t2 * t;
                3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t2 * y2 + t3
            }
        }
    }
}

/// Animation state trait for types that can be animated
pub trait Animatable: Clone {
    /// Interpolate between two values
    fn interpolate(&self, other: &Self, progress: f32) -> Self;
}

impl Animatable for f32 {
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        self + (other - self) * progress
    }
}

impl Animatable for iced::Color {
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        iced::Color::from_rgba(
            self.r + (other.r - self.r) * progress,
            self.g + (other.g - self.g) * progress,
            self.b + (other.b - self.b) * progress,
            self.a + (other.a - self.a) * progress,
        )
    }
}

/// Predefined animation presets
pub struct AnimationPresets;

impl AnimationPresets {
    /// Fade in animation
    pub fn fade_in() -> Animation {
        Animation::fast().with_easing(EasingFunction::EaseOutCubic)
    }
    
    /// Scale up animation
    pub fn scale_up() -> Animation {
        Animation::fast().with_easing(EasingFunction::EaseOutBack)
    }
    
    /// Slide in animation
    pub fn slide_in() -> Animation {
        Animation::normal().with_easing(EasingFunction::EaseOutCubic)
    }
    
    /// Bounce animation
    pub fn bounce() -> Animation {
        Animation::slow().with_easing(EasingFunction::EaseOutBounce)
    }
    
    /// Pulse animation (ping-pong)
    pub fn pulse() -> Animation {
        Animation::normal()
            .with_easing(EasingFunction::EaseInOutCubic)
            .with_loop(true)
            .with_auto_reverse(true)
    }
    
    /// Shimmer animation
    pub fn shimmer() -> Animation {
        Animation::slow()
            .with_easing(EasingFunction::EaseInOutCubic)
            .with_loop(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_lifecycle() {
        let mut anim = Animation::fast();
        assert!(!anim.is_playing);
        
        anim.start();
        assert!(anim.is_playing);
        
        anim.pause();
        assert!(!anim.is_playing);
        
        anim.stop();
        assert_eq!(anim.progress, 0.0);
    }
    
    #[test]
    fn test_easing_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(1.0), 1.0);
        assert_eq!(easing.apply(0.5), 0.5);
    }
    
    #[test]
    fn test_easing_out_cubic() {
        let easing = EasingFunction::EaseOutCubic;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(1.0), 1.0);
        // Should be above linear at midpoint
        assert!(easing.apply(0.5) > 0.5);
    }
    
    #[test]
    fn test_color_interpolation() {
        let start = iced::Color::from_rgb(0.0, 0.0, 0.0);
        let end = iced::Color::from_rgb(1.0, 1.0, 1.0);
        let mid = start.interpolate(&end, 0.5);
        
        assert!((mid.r - 0.5).abs() < 0.01);
        assert!((mid.g - 0.5).abs() < 0.01);
        assert!((mid.b - 0.5).abs() < 0.01);
    }
}