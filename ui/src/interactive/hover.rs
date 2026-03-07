//! Hover Effects Module
//! 
//! Provides hover state management and visual effects for interactive elements.

use iced::{Color, Point, Rectangle};
use std::time::{Duration, Instant};

/// Hover state tracker
#[derive(Debug, Clone)]
pub struct HoverState {
    /// Whether the element is currently hovered
    is_hovered: bool,
    /// Time when hover started
    hover_start: Option<Instant>,
    /// Hover animation progress (0.0 to 1.0)
    progress: f32,
    /// Last known cursor position
    cursor_position: Option<Point>,
}

impl Default for HoverState {
    fn default() -> Self {
        Self {
            is_hovered: false,
            hover_start: None,
            progress: 0.0,
            cursor_position: None,
        }
    }
}

impl HoverState {
    /// Create a new hover state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Start hovering
    pub fn start(&mut self) {
        if !self.is_hovered {
            self.is_hovered = true;
            self.hover_start = Some(Instant::now());
            self.progress = 0.0;
        }
    }
    
    /// Stop hovering
    pub fn stop(&mut self) {
        self.is_hovered = false;
        self.hover_start = None;
        self.progress = 0.0;
    }
    
    /// Update hover animation progress
    pub fn update(&mut self, duration: Duration) {
        if let Some(start) = self.hover_start {
            let elapsed = start.elapsed();
            self.progress = (elapsed.as_secs_f32() / duration.as_secs_f32()).min(1.0);
        }
    }
    
    /// Check if currently hovered
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }
    
    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    /// Update cursor position
    pub fn set_cursor_position(&mut self, position: Point) {
        self.cursor_position = Some(position);
    }
    
    /// Check if point is within bounds
    pub fn check_bounds(&mut self, cursor: Point, bounds: Rectangle) {
        if bounds.contains(cursor) {
            self.start();
        } else {
            self.stop();
        }
    }
}

/// Trait for hoverable elements
pub trait Hoverable {
    /// Get current hover state
    fn hover_state(&self) -> &HoverState;
    
    /// Get mutable hover state
    fn hover_state_mut(&mut self) -> &mut HoverState;
    
    /// Called when hover starts
    fn on_hover_start(&mut self) {}
    
    /// Called when hover ends
    fn on_hover_end(&mut self) {}
}

/// Hover effect types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverEffect {
    /// Scale up slightly
    Scale(f32),
    /// Brighten the color
    Brighten(f32),
    /// Add glow effect
    Glow {
        color: Color,
        intensity: f32,
        radius: f32,
    },
    /// Lift (shadow + slight translate up)
    Lift {
        shadow_blur: f32,
        translate_y: f32,
    },
    /// Color shift
    ColorShift(Color),
    /// Border highlight
    BorderHighlight {
        color: Color,
        width: f32,
    },
    /// Combination of effects
    Combined(Vec<HoverEffect>),
}

impl HoverEffect {
    /// Default hover effect (scale + glow)
    pub fn default_crimson() -> Self {
        HoverEffect::Glow {
            color: Color::from_rgb(0.863, 0.078, 0.235), // Crimson
            intensity: 0.5,
            radius: 8.0,
        }
    }
    
    /// Netflix-style hover effect
    pub fn netflix_style() -> Self {
        HoverEffect::Combined(vec![
            HoverEffect::Scale(1.05),
            HoverEffect::Glow {
                color: Color::from_rgb(0.863, 0.078, 0.235),
                intensity: 0.3,
                radius: 12.0,
            },
        ])
    }
    
    /// Card hover effect
    pub fn card_lift() -> Self {
        HoverEffect::Lift {
            shadow_blur: 20.0,
            translate_y: -4.0,
        }
    }
    
    /// Button hover effect
    pub fn button_glow() -> Self {
        HoverEffect::Glow {
            color: Color::from_rgb(0.863, 0.078, 0.235),
            intensity: 0.6,
            radius: 6.0,
        }
    }
    
    /// Icon hover effect
    pub fn icon_highlight() -> Self {
        HoverEffect::Brighten(0.2)
    }
}

/// Hover animation configuration
#[derive(Debug, Clone, Copy)]
pub struct HoverConfig {
    /// Animation duration in milliseconds
    pub duration_ms: u64,
    /// Easing function
    pub easing: EasingFn,
    /// Delay before effect starts
    pub delay_ms: u64,
}

impl Default for HoverConfig {
    fn default() -> Self {
        Self {
            duration_ms: 200,
            easing: EasingFn::EaseOutCubic,
            delay_ms: 0,
        }
    }
}

/// Easing functions for hover animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFn {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutElastic,
    EaseOutBounce,
}

impl EasingFn {
    /// Apply easing function to a value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFn::Linear => t,
            EasingFn::EaseInQuad => t * t,
            EasingFn::EaseOutQuad => t * (2.0 - t),
            EasingFn::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFn::EaseInCubic => t * t * t,
            EasingFn::EaseOutCubic => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFn::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = 2.0 * t - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
            EasingFn::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    (2.0f32).powf(-10.0 * t) * ((t - p / 4.0) * std::f32::consts::TAU / p).sin() + 1.0
                }
            }
            EasingFn::EaseOutBounce => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t1 = t - 1.5 / 2.75;
                    7.5625 * t1 * t1 + 0.75
                } else if t < 2.5 / 2.75 {
                    let t1 = t - 2.25 / 2.75;
                    7.5625 * t1 * t1 + 0.9375
                } else {
                    let t1 = t - 2.625 / 2.75;
                    7.5625 * t1 * t1 + 0.984375
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hover_state() {
        let mut state = HoverState::new();
        assert!(!state.is_hovered());
        
        state.start();
        assert!(state.is_hovered());
        
        state.stop();
        assert!(!state.is_hovered());
    }
    
    #[test]
    fn test_easing_functions() {
        assert_eq!(EasingFn::Linear.apply(0.5), 0.5);
        assert!(EasingFn::EaseOutCubic.apply(0.5) > 0.0);
        assert!(EasingFn::EaseOutCubic.apply(0.5) < 1.0);
    }
    
    #[test]
    fn test_default_effects() {
        let effect = HoverEffect::netflix_style();
        assert!(matches!(effect, HoverEffect::Combined(_)));
    }
}