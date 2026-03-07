//! Feedback Module
//! 
//! Provides user feedback mechanisms including haptic feedback,
//! sound effects, and visual notifications.

use iced::Color;
use std::time::Duration;

/// Feedback system for user interactions
#[derive(Debug, Clone)]
pub struct Feedback {
    /// Enable visual feedback
    pub visual_enabled: bool,
    /// Enable sound feedback
    pub sound_enabled: bool,
    /// Enable haptic feedback
    pub haptic_enabled: bool,
    /// Current visual feedback
    current_visual: Option<VisualFeedback>,
}

impl Default for Feedback {
    fn default() -> Self {
        Self {
            visual_enabled: true,
            sound_enabled: true,
            haptic_enabled: false,
            current_visual: None,
        }
    }
}

impl Feedback {
    /// Create a new feedback system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Trigger feedback of specified type
    pub fn trigger(&mut self, feedback_type: FeedbackType) {
        if self.visual_enabled {
            self.current_visual = Some(VisualFeedback::from(feedback_type.clone()));
        }
        
        if self.haptic_enabled {
            self.trigger_haptic(&feedback_type);
        }
    }
    
    /// Trigger haptic feedback
    fn trigger_haptic(&self, feedback_type: &FeedbackType) {
        #[cfg(target_os = "windows")]
        {
            // Windows haptic feedback
        }
        #[cfg(target_os = "macos")]
        {
            // macOS haptic feedback
        }
        #[cfg(target_os = "linux")]
        {
            // Linux haptic feedback
        }
        let _ = feedback_type; // Suppress unused warning
    }
    
    /// Get current visual feedback
    pub fn current_visual(&self) -> Option<&VisualFeedback> {
        self.current_visual.as_ref()
    }
    
    /// Clear visual feedback
    pub fn clear_visual(&mut self) {
        self.current_visual = None;
    }
}

/// Types of feedback
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackType {
    /// Light tap feedback
    Light,
    /// Medium impact feedback
    Medium,
    /// Heavy impact feedback
    Heavy,
    /// Success notification
    Success,
    /// Warning notification
    Warning,
    /// Error notification
    Error,
    /// Selection change
    Selection,
    /// Toggle on/off
    Toggle(bool),
    /// Custom feedback
    Custom {
        name: String,
        intensity: f32,
    },
}

/// Visual feedback representation
#[derive(Debug, Clone)]
pub struct VisualFeedback {
    /// Feedback type
    pub feedback_type: FeedbackType,
    /// Background color flash
    pub color: Color,
    /// Duration of the feedback
    pub duration: Duration,
    /// Opacity of the effect
    pub opacity: f32,
    /// Scale effect (if applicable)
    pub scale: f32,
}

impl From<FeedbackType> for VisualFeedback {
    fn from(feedback_type: FeedbackType) -> Self {
        match &feedback_type {
            FeedbackType::Light => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                duration: Duration::from_millis(100),
                opacity: 0.1,
                scale: 1.0,
            },
            FeedbackType::Medium => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                duration: Duration::from_millis(150),
                opacity: 0.2,
                scale: 1.02,
            },
            FeedbackType::Heavy => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                duration: Duration::from_millis(200),
                opacity: 0.3,
                scale: 1.05,
            },
            FeedbackType::Success => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgb(0.2, 0.8, 0.4), // Green
                duration: Duration::from_millis(300),
                opacity: 0.2,
                scale: 1.0,
            },
            FeedbackType::Warning => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgb(1.0, 0.8, 0.0), // Yellow
                duration: Duration::from_millis(400),
                opacity: 0.2,
                scale: 1.0,
            },
            FeedbackType::Error => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgb(1.0, 0.3, 0.3), // Red
                duration: Duration::from_millis(500),
                opacity: 0.3,
                scale: 1.0,
            },
            FeedbackType::Selection => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgb(0.863, 0.078, 0.235), // Crimson
                duration: Duration::from_millis(100),
                opacity: 0.15,
                scale: 1.0,
            },
            FeedbackType::Toggle(on) => Self {
                feedback_type: feedback_type.clone(),
                color: if *on {
                    Color::from_rgb(0.2, 0.8, 0.4)
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.2)
                },
                duration: Duration::from_millis(150),
                opacity: 0.2,
                scale: 1.0,
            },
            FeedbackType::Custom { name: _, intensity } => Self {
                feedback_type: feedback_type.clone(),
                color: Color::from_rgb(0.863, 0.078, 0.235),
                duration: Duration::from_millis(200),
                opacity: *intensity,
                scale: 1.0,
            },
        }
    }
}

/// Haptic feedback controller
#[derive(Debug, Clone)]
pub struct HapticFeedback {
    /// Enable light haptics
    pub light_enabled: bool,
    /// Enable medium haptics
    pub medium_enabled: bool,
    /// Enable heavy haptics
    pub heavy_enabled: bool,
    /// Custom intensity multiplier
    pub intensity: f32,
}

impl Default for HapticFeedback {
    fn default() -> Self {
        Self {
            light_enabled: true,
            medium_enabled: true,
            heavy_enabled: true,
            intensity: 1.0,
        }
    }
}

impl HapticFeedback {
    /// Create a new haptic feedback controller
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Perform a haptic vibration
    pub fn vibrate(&self, pattern: VibrationPattern) {
        let _ = pattern; // Suppress unused warning
        
        #[cfg(target_os = "android")]
        {
            // Android vibration
        }
        #[cfg(target_os = "ios")]
        {
            // iOS haptic feedback
        }
    }
}

/// Vibration patterns for haptic feedback
#[derive(Debug, Clone)]
pub enum VibrationPattern {
    /// Single short vibration
    Short,
    /// Single medium vibration
    Medium,
    /// Single long vibration
    Long,
    /// Double tap pattern
    DoubleTap,
    /// Triple tap pattern
    TripleTap,
    /// Heartbeat pattern
    Heartbeat,
    /// Success pattern (two short)
    Success,
    /// Error pattern (long-short-long)
    Error,
    /// Custom pattern (durations in ms)
    Custom(Vec<Duration>),
}

/// Notification banner for feedback
#[derive(Debug, Clone)]
pub struct NotificationBanner {
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: Option<String>,
    /// Notification type
    pub notification_type: NotificationType,
    /// Duration to show
    pub duration: Duration,
    /// Whether it can be dismissed
    pub dismissible: bool,
    /// Action button text
    pub action_text: Option<String>,
}

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationBanner {
    /// Create a new notification
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: None,
            notification_type: NotificationType::Info,
            duration: Duration::from_secs(5),
            dismissible: true,
            action_text: None,
        }
    }
    
    /// Set notification type
    pub fn with_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = notification_type;
        self
    }
    
    /// Set message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    
    /// Set duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
    
    /// Set action button
    pub fn with_action(mut self, action_text: impl Into<String>) -> Self {
        self.action_text = Some(action_text.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feedback_default() {
        let feedback = Feedback::new();
        assert!(feedback.visual_enabled);
        assert!(feedback.sound_enabled);
        assert!(!feedback.haptic_enabled);
    }
    
    #[test]
    fn test_feedback_trigger() {
        let mut feedback = Feedback::new();
        feedback.trigger(FeedbackType::Success);
        
        assert!(feedback.current_visual.is_some());
    }
    
    #[test]
    fn test_visual_feedback_from_type() {
        let visual = VisualFeedback::from(FeedbackType::Success);
        assert!(matches!(visual.feedback_type, FeedbackType::Success));
        assert!((visual.color.g - 0.8).abs() < 0.1); // Green-ish
    }
    
    #[test]
    fn test_notification_banner() {
        let banner = NotificationBanner::new("Test")
            .with_type(NotificationType::Success)
            .with_message("Operation completed");
        
        assert_eq!(banner.title, "Test");
        assert!(banner.message.is_some());
    }
}