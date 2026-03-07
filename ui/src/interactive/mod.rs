//! Interactive Elements Module
//! 
//! Provides interactive UI components with animations, hover effects,
//! tooltips, progress indicators, and gesture support.

mod hover;
mod tooltip;
mod progress;
mod gesture;
mod animation;
mod feedback;

pub use hover::{HoverState, Hoverable, HoverEffect};
pub use tooltip::{Tooltip, TooltipPosition, TooltipStyle};
pub use progress::{ProgressIndicator, ProgressState, CircularProgress};
pub use gesture::{Gesture, GestureRecognizer, SwipeDirection};
pub use animation::{Animation, AnimationState, EasingFunction};
pub use feedback::{Feedback, FeedbackType, HapticFeedback};

/// Interactive component configuration
#[derive(Debug, Clone)]
pub struct InteractiveConfig {
    /// Enable hover effects
    pub hover_enabled: bool,
    /// Enable tooltips
    pub tooltips_enabled: bool,
    /// Enable animations
    pub animations_enabled: bool,
    /// Animation duration in milliseconds
    pub animation_duration: u64,
    /// Enable gesture recognition
    pub gestures_enabled: bool,
    /// Enable haptic feedback
    pub haptic_feedback: bool,
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            hover_enabled: true,
            tooltips_enabled: true,
            animations_enabled: true,
            animation_duration: 200,
            gestures_enabled: true,
            haptic_feedback: false,
        }
    }
}

/// Interactive element state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionState {
    /// Default state
    Normal,
    /// Mouse hovering over element
    Hovered,
    /// Element is pressed/clicked
    Pressed,
    /// Element is focused (keyboard)
    Focused,
    /// Element is disabled
    Disabled,
}

impl InteractionState {
    /// Check if element is interactive (can be clicked)
    pub fn is_interactive(&self) -> bool {
        !matches!(self, InteractionState::Disabled)
    }
    
    /// Check if element should show hover effects
    pub fn show_hover_effects(&self) -> bool {
        matches!(self, InteractionState::Hovered | InteractionState::Focused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = InteractiveConfig::default();
        assert!(config.hover_enabled);
        assert!(config.tooltips_enabled);
        assert!(config.animations_enabled);
    }
    
    #[test]
    fn test_interaction_state() {
        assert!(InteractionState::Normal.is_interactive());
        assert!(InteractionState::Hovered.is_interactive());
        assert!(!InteractionState::Disabled.is_interactive());
    }
}