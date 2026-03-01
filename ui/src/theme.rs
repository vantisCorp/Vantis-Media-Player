//! Theme System
//! 
/// Liquid Glass theme with dark mode support.

use iced::{Color, Theme, application, color};

/// Vantis theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Custom,
}

impl Theme {
    /// Get the Iced theme
    pub fn to_iced_theme(&self) -> iced::Theme {
        match self {
            Theme::Dark => iced::Theme::Dark,
            Theme::Light => iced::Theme::Light,
            Theme::Custom => iced::Theme::custom(|_| application::Appearance {
                background_color: Color::from_rgb(0.1, 0.1, 0.15),
                text_color: Color::WHITE,
            }),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl From<Theme> for iced::Theme {
    fn from(theme: Theme) -> Self {
        theme.to_iced_theme()
    }
}

/// Dark theme colors
pub mod dark {
    use iced::Color;

    pub const BACKGROUND: Color = Color::from_rgb(0.08, 0.08, 0.12);
    pub const SURFACE: Color = Color::from_rgb(0.15, 0.15, 0.2);
    pub const PRIMARY: Color = Color::from_rgb(0.4, 0.6, 1.0);
    pub const SECONDARY: Color = Color::from_rgb(0.6, 0.4, 1.0);
    pub const ACCENT: Color = Color::from_rgb(1.0, 0.6, 0.4);
    pub const TEXT: Color = Color::from_rgb(0.95, 0.95, 0.95);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.6, 0.6, 0.65);
    pub const BORDER: Color = Color::from_rgb(0.2, 0.2, 0.25);
}

/// Light theme colors
pub mod light {
    use iced::Color;

    pub const BACKGROUND: Color = Color::from_rgb(0.98, 0.98, 1.0);
    pub const SURFACE: Color = Color::from_rgb(0.95, 0.95, 1.0);
    pub const PRIMARY: Color = Color::from_rgb(0.2, 0.4, 0.9);
    pub const SECONDARY: Color = Color::from_rgb(0.5, 0.3, 0.9);
    pub const ACCENT: Color = Color::from_rgb(0.9, 0.4, 0.2);
    pub const TEXT: Color = Color::from_rgb(0.1, 0.1, 0.12);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.5, 0.5, 0.55);
    pub const BORDER: Color = Color::from_rgb(0.85, 0.85, 0.9);
}