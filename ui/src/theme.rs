//! Theme System
//! 
//! Netflix-style design with deep black (#000000) and crimson (#DC143C).
//! Geometric separators: ► ════ ◄

use iced::{Color, Theme, application};

/// Vantis theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VantisTheme {
    #[default]
    NetflixDark,
    NetflixLight,
    CrimsonNight,
    MidnightCinema,
}

impl VantisTheme {
    /// Get the Iced theme
    pub fn to_iced_theme(&self) -> iced::Theme {
        match self {
            VantisTheme::NetflixDark => iced::Theme::custom(|_| application::Appearance {
                background_color: netflix::dark::BACKGROUND,
                text_color: netflix::dark::TEXT,
            }),
            VantisTheme::NetflixLight => iced::Theme::custom(|_| application::Appearance {
                background_color: netflix::light::BACKGROUND,
                text_color: netflix::light::TEXT,
            }),
            VantisTheme::CrimsonNight => iced::Theme::custom(|_| application::Appearance {
                background_color: crimson_night::BACKGROUND,
                text_color: crimson_night::TEXT,
            }),
            VantisTheme::MidnightCinema => iced::Theme::custom(|_| application::Appearance {
                background_color: midnight_cinema::BACKGROUND,
                text_color: midnight_cinema::TEXT,
            }),
        }
    }
    
    /// Get theme name for display
    pub fn name(&self) -> &'static str {
        match self {
            VantisTheme::NetflixDark => "Netflix Dark",
            VantisTheme::NetflixLight => "Netflix Light",
            VantisTheme::CrimsonNight => "Crimson Night",
            VantisTheme::MidnightCinema => "Midnight Cinema",
        }
    }
}

impl From<VantisTheme> for iced::Theme {
    fn from(theme: VantisTheme) -> Self {
        theme.to_iced_theme()
    }
}

/// Netflix-inspired color palette
pub mod netflix {
    /// Deep black (#000000) with crimson (#DC143C) accent
    pub mod dark {
        use iced::Color;
        
        /// Primary background - Pure deep black
        pub const BACKGROUND: Color = Color::from_rgb(0.0, 0.0, 0.0);
        /// Surface/Card background - Subtle dark gray
        pub const SURFACE: Color = Color::from_rgb(0.08, 0.08, 0.08);
        /// Elevated surface - Slightly lighter
        pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.12, 0.12, 0.12);
        /// Primary accent - Netflix Crimson
        pub const CRIMSON: Color = Color::from_rgb(0.863, 0.078, 0.235); // #DC143C
        /// Crimson hover state - Slightly brighter
        pub const CRIMSON_LIGHT: Color = Color::from_rgb(0.91, 0.14, 0.31); // #E8244F
        /// Crimson glow effect
        pub const CRIMSON_GLOW: Color = Color::from_rgba(0.863, 0.078, 0.235, 0.3);
        /// Primary text - Pure white
        pub const TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);
        /// Secondary text - Muted gray
        pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
        /// Muted/placeholder text
        pub const TEXT_MUTED: Color = Color::from_rgb(0.5, 0.5, 0.5);
        /// Border color - Subtle
        pub const BORDER: Color = Color::from_rgb(0.2, 0.2, 0.2);
        /// Border focus state
        pub const BORDER_FOCUS: Color = CRIMSON;
        /// Success state
        pub const SUCCESS: Color = Color::from_rgb(0.2, 0.8, 0.4); // #33CC66
        /// Warning state
        pub const WARNING: Color = Color::from_rgb(1.0, 0.8, 0.0); // #FFCC00
        /// Error state
        pub const ERROR: Color = Color::from_rgb(1.0, 0.3, 0.3); // #FF4D4D
        /// Overlay for modals
        pub const OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);
        /// Gradient start for banners
        pub const GRADIENT_START: Color = BACKGROUND;
        /// Gradient end with crimson tint
        pub const GRADIENT_END: Color = Color::from_rgb(0.15, 0.0, 0.05);
    }
    
    /// Light mode with crimson accents
    pub mod light {
        use iced::Color;
        
        /// Primary background
        pub const BACKGROUND: Color = Color::from_rgb(0.98, 0.98, 0.98);
        /// Surface/Card background
        pub const SURFACE: Color = Color::from_rgb(1.0, 1.0, 1.0);
        /// Elevated surface
        pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.95, 0.95, 0.95);
        /// Primary accent - Netflix Crimson
        pub const CRIMSON: Color = Color::from_rgb(0.863, 0.078, 0.235);
        /// Crimson hover state
        pub const CRIMSON_LIGHT: Color = Color::from_rgb(0.91, 0.14, 0.31);
        /// Primary text
        pub const TEXT: Color = Color::from_rgb(0.1, 0.1, 0.1);
        /// Secondary text
        pub const TEXT_SECONDARY: Color = Color::from_rgb(0.3, 0.3, 0.3);
        /// Muted text
        pub const TEXT_MUTED: Color = Color::from_rgb(0.5, 0.5, 0.5);
        /// Border
        pub const BORDER: Color = Color::from_rgb(0.85, 0.85, 0.85);
        /// Border focus
        pub const BORDER_FOCUS: Color = CRIMSON;
        /// Success
        pub const SUCCESS: Color = Color::from_rgb(0.15, 0.7, 0.35);
        /// Warning
        pub const WARNING: Color = Color::from_rgb(0.9, 0.7, 0.0);
        /// Error
        pub const ERROR: Color = Color::from_rgb(0.9, 0.2, 0.2);
    }
}

/// Crimson Night theme - Dark with pronounced crimson accents
pub mod crimson_night {
    use iced::Color;
    
    pub const BACKGROUND: Color = Color::from_rgb(0.02, 0.0, 0.02);
    pub const SURFACE: Color = Color::from_rgb(0.06, 0.02, 0.04);
    pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.1, 0.04, 0.06);
    pub const CRIMSON: Color = Color::from_rgb(0.9, 0.1, 0.25);
    pub const CRIMSON_LIGHT: Color = Color::from_rgb(0.95, 0.2, 0.35);
    pub const TEXT: Color = Color::from_rgb(0.98, 0.96, 0.97);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.75, 0.7, 0.72);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.5, 0.45, 0.48);
    pub const BORDER: Color = Color::from_rgb(0.25, 0.15, 0.18);
}

/// Midnight Cinema theme - Classic cinema feel
pub mod midnight_cinema {
    use iced::Color;
    
    pub const BACKGROUND: Color = Color::from_rgb(0.03, 0.03, 0.05);
    pub const SURFACE: Color = Color::from_rgb(0.08, 0.08, 0.12);
    pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.12, 0.12, 0.18);
    pub const CRIMSON: Color = Color::from_rgb(0.8, 0.15, 0.2);
    pub const CRIMSON_LIGHT: Color = Color::from_rgb(0.85, 0.25, 0.3);
    pub const TEXT: Color = Color::from_rgb(0.95, 0.95, 0.97);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.75);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.45, 0.45, 0.5);
    pub const BORDER: Color = Color::from_rgb(0.18, 0.18, 0.22);
    /// Gold accent for premium feel
    pub const GOLD: Color = Color::from_rgb(0.85, 0.65, 0.13); // #DAA520
}

/// Geometric separator constants for UI elements
pub mod geometric {
    /// Arrow right symbol
    pub const ARROW_RIGHT: &str = "►";
    /// Arrow left symbol
    pub const ARROW_LEFT: &str = "◄";
    /// Double arrow right
    pub const ARROW_DOUBLE_RIGHT: &str = "»";
    /// Double arrow left
    pub const ARROW_DOUBLE_LEFT: &str = "«";
    /// Horizontal line
    pub const LINE: &str = "═";
    /// Diamond separator
    pub const DIAMOND: &str = "◆";
    /// Bullet point
    pub const BULLET: &str = "•";
    
    /// Create a geometric separator: ► ════ ◄
    pub fn separator(width: usize) -> String {
        format!("{} {} {}", ARROW_RIGHT, LINE.repeat(width), ARROW_LEFT)
    }
    
    /// Create a decorative header line
    pub fn header_line(text: &str) -> String {
        format!("{} {} {}", ARROW_RIGHT, text.to_uppercase(), ARROW_LEFT)
    }
}

/// Typography constants
pub mod typography {
    /// Font size for hero/large headings
    pub const HERO_SIZE: f32 = 48.0;
    /// Font size for section headings
    pub const HEADING_SIZE: f32 = 32.0;
    /// Font size for subheadings
    pub const SUBHEADING_SIZE: f32 = 24.0;
    /// Font size for body text
    pub const BODY_SIZE: f32 = 16.0;
    /// Font size for small/caption text
    pub const CAPTION_SIZE: f32 = 12.0;
    /// Font size for micro text
    pub const MICRO_SIZE: f32 = 10.0;
}

/// Animation timing constants
pub mod animation {
    /// Fast transition (100ms)
    pub const DURATION_FAST: u64 = 100;
    /// Normal transition (200ms)
    pub const DURATION_NORMAL: u64 = 200;
    /// Slow transition (300ms)
    pub const DURATION_SLOW: u64 = 300;
    /// Extra slow for dramatic effects (500ms)
    pub const DURATION_DRAMATIC: u64 = 500;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_default() {
        let theme = VantisTheme::default();
        assert_eq!(theme, VantisTheme::NetflixDark);
    }
    
    #[test]
    fn test_crimson_color() {
        let crimson = netflix::dark::CRIMSON;
        // #DC143C = rgb(220, 20, 60) = (0.863, 0.078, 0.235)
        assert!((crimson.r - 0.863).abs() < 0.01);
        assert!((crimson.g - 0.078).abs() < 0.01);
        assert!((crimson.b - 0.235).abs() < 0.01);
    }
    
    #[test]
    fn test_geometric_separator() {
        let sep = geometric::separator(4);
        assert_eq!(sep, "► ════ ◄");
    }
    
    #[test]
    fn test_geometric_header() {
        let header = geometric::header_line("Vantis");
        assert_eq!(header, "► VANTIS ◄");
    }
}