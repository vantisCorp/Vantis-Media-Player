//! Tooltip Module
//! 
//! Provides tooltip components with positioning and styling.

use iced::{Color, Point, Rectangle, Size};
use std::time::{Duration, Instant};

/// Tooltip component
#[derive(Debug, Clone)]
pub struct Tooltip {
    /// Tooltip text content
    pub content: String,
    /// Position relative to target
    pub position: TooltipPosition,
    /// Styling
    pub style: TooltipStyle,
    /// Show delay
    pub delay: Duration,
    /// Hide delay
    pub hide_delay: Duration,
    /// Maximum width
    pub max_width: f32,
    /// Whether tooltip is visible
    pub visible: bool,
    /// Time when show was requested
    show_requested: Option<Instant>,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            position: TooltipPosition::Top,
            style: TooltipStyle::default(),
            delay: Duration::from_millis(500),
            hide_delay: Duration::from_millis(100),
            max_width: 250.0,
            visible: false,
            show_requested: None,
        }
    }
    
    /// Set tooltip position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }
    
    /// Set tooltip style
    pub fn with_style(mut self, style: TooltipStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set show delay
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
    
    /// Request to show tooltip (starts delay timer)
    pub fn request_show(&mut self) {
        if self.show_requested.is_none() {
            self.show_requested = Some(Instant::now());
        }
    }
    
    /// Cancel show request
    pub fn cancel_show(&mut self) {
        self.show_requested = None;
        self.visible = false;
    }
    
    /// Update tooltip state
    pub fn update(&mut self) {
        if let Some(requested) = self.show_requested {
            if requested.elapsed() >= self.delay {
                self.visible = true;
            }
        }
    }
    
    /// Calculate tooltip bounds given target bounds
    pub fn calculate_bounds(&self, target_bounds: Rectangle, tooltip_size: Size) -> Rectangle {
        let offset = 8.0; // Gap between tooltip and target
        
        let position = match self.position {
            TooltipPosition::Top => {
                Point::new(
                    target_bounds.center_x() - tooltip_size.width / 2.0,
                    target_bounds.y - tooltip_size.height - offset,
                )
            }
            TooltipPosition::Bottom => {
                Point::new(
                    target_bounds.center_x() - tooltip_size.width / 2.0,
                    target_bounds.y + target_bounds.height + offset,
                )
            }
            TooltipPosition::Left => {
                Point::new(
                    target_bounds.x - tooltip_size.width - offset,
                    target_bounds.center_y() - tooltip_size.height / 2.0,
                )
            }
            TooltipPosition::Right => {
                Point::new(
                    target_bounds.x + target_bounds.width + offset,
                    target_bounds.center_y() - tooltip_size.height / 2.0,
                )
            }
            TooltipPosition::Auto => {
                // Auto: prefer top, fall back to bottom
                Point::new(
                    target_bounds.center_x() - tooltip_size.width / 2.0,
                    target_bounds.y - tooltip_size.height - offset,
                )
            }
        };
        
        Rectangle::new(position, tooltip_size)
    }
}

/// Tooltip position relative to target element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Above the target
    Top,
    /// Below the target
    Bottom,
    /// Left of the target
    Left,
    /// Right of the target
    Right,
    /// Automatically choose best position
    Auto,
}

/// Tooltip visual style
#[derive(Debug, Clone, Copy)]
pub struct TooltipStyle {
    /// Background color
    pub background: Color,
    /// Text color
    pub text_color: Color,
    /// Border color
    pub border_color: Option<Color>,
    /// Border width
    pub border_width: f32,
    /// Border radius
    pub border_radius: f32,
    /// Padding
    pub padding: f32,
    /// Font size
    pub font_size: f32,
    /// Arrow/pointer style
    pub arrow: bool,
    /// Shadow blur
    pub shadow_blur: f32,
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            background: Color::from_rgb(0.12, 0.12, 0.12), // Dark surface
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            border_color: Some(Color::from_rgb(0.863, 0.078, 0.235)), // Crimson
            border_width: 1.0,
            border_radius: 6.0,
            padding: 8.0,
            font_size: 12.0,
            arrow: true,
            shadow_blur: 8.0,
        }
    }
}

impl TooltipStyle {
    /// Netflix-style tooltip
    pub fn netflix() -> Self {
        Self {
            background: Color::from_rgb(0.0, 0.0, 0.0), // Pure black
            text_color: Color::WHITE,
            border_color: Some(Color::from_rgb(0.863, 0.078, 0.235)),
            border_width: 1.0,
            border_radius: 4.0,
            padding: 10.0,
            font_size: 13.0,
            arrow: false,
            shadow_blur: 12.0,
        }
    }
    
    /// Light tooltip style
    pub fn light() -> Self {
        Self {
            background: Color::from_rgb(1.0, 1.0, 1.0),
            text_color: Color::from_rgb(0.1, 0.1, 0.1),
            border_color: Some(Color::from_rgb(0.85, 0.85, 0.85)),
            border_width: 1.0,
            border_radius: 6.0,
            padding: 8.0,
            font_size: 12.0,
            arrow: true,
            shadow_blur: 8.0,
        }
    }
    
    /// Minimal tooltip (no border, subtle shadow)
    pub fn minimal() -> Self {
        Self {
            background: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            text_color: Color::WHITE,
            border_color: None,
            border_width: 0.0,
            border_radius: 4.0,
            padding: 6.0,
            font_size: 11.0,
            arrow: false,
            shadow_blur: 4.0,
        }
    }
}

/// Rich tooltip with title and description
#[derive(Debug, Clone)]
pub struct RichTooltip {
    /// Title text (bold)
    pub title: String,
    /// Description text
    pub description: Option<String>,
    /// Optional icon
    pub icon: Option<String>,
    /// Base tooltip properties
    pub tooltip: Tooltip,
}

impl RichTooltip {
    /// Create a new rich tooltip
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            tooltip: Tooltip::new(""),
        }
    }
    
    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Add icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tooltip_creation() {
        let tooltip = Tooltip::new("Test tooltip");
        assert_eq!(tooltip.content, "Test tooltip");
        assert!(!tooltip.visible);
    }
    
    #[test]
    fn test_tooltip_delay() {
        let mut tooltip = Tooltip::new("Test");
        tooltip.delay = Duration::from_millis(10);
        
        tooltip.request_show();
        assert!(!tooltip.visible);
        
        std::thread::sleep(Duration::from_millis(15));
        tooltip.update();
        assert!(tooltip.visible);
    }
    
    #[test]
    fn test_tooltip_position_calculation() {
        let tooltip = Tooltip::new("Test");
        let target = Rectangle::new(Point::new(100.0, 100.0), Size::new(50.0, 30.0));
        let tooltip_size = Size::new(80.0, 24.0);
        
        let bounds = tooltip.calculate_bounds(target, tooltip_size);
        assert!(bounds.y < target.y); // Should be above target
    }
}