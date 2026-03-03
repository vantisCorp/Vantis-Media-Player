//! Customizable Theme System
//!
//! This module provides a flexible theme system allowing users to create,
//! customize, import, export, and share themes for Vantis Media Player.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

/// Theme specification format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSpecification {
    /// Theme name
    pub name: String,
    
    /// Theme version
    pub version: String,
    
    /// Theme author
    pub author: String,
    
    /// Theme description
    pub description: String,
    
    /// Theme type
    pub theme_type: ThemeType,
    
    /// Color palette
    pub colors: ColorPalette,
    
    /// Typography settings
    pub typography: Typography,
    
    /// Spacing settings
    pub spacing: Spacing,
    
    /// Border radius settings
    pub border_radius: BorderRadius,
    
    /// Shadow settings
    pub shadows: Shadows,
    
    /// Component styles
    pub components: ComponentStyles,
    
    /// Custom CSS
    pub custom_css: Option<String>,
}

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeType {
    /// Light theme
    Light,
    
    /// Dark theme
    Dark,
    
    /// Custom theme
    Custom,
}

impl ThemeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Custom => "Custom",
        }
    }
}

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary color
    pub primary: String,
    
    /// Secondary color
    pub secondary: String,
    
    /// Accent color
    pub accent: String,
    
    /// Background color
    pub background: String,
    
    /// Surface color
    pub surface: String,
    
    /// Text color
    pub text: String,
    
    /// Text secondary color
    pub text_secondary: String,
    
    /// Border color
    pub border: String,
    
    /// Success color
    pub success: String,
    
    /// Warning color
    pub warning: String,
    
    /// Error color
    pub error: String,
    
    /// Info color
    pub info: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: "#6366f1".to_string(),
            secondary: "#8b5cf6".to_string(),
            accent: "#ec4899".to_string(),
            background: "#0f172a".to_string(),
            surface: "#1e293b".to_string(),
            text: "#f8fafc".to_string(),
            text_secondary: "#94a3b8".to_string(),
            border: "#334155".to_string(),
            success: "#22c55e".to_string(),
            warning: "#f59e0b".to_string(),
            error: "#ef4444".to_string(),
            info: "#3b82f6".to_string(),
        }
    }
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Font family
    pub font_family: String,
    
    /// Font size base
    pub font_size_base: u32,
    
    /// Font size small
    pub font_size_small: u32,
    
    /// Font size large
    pub font_size_large: u32,
    
    /// Font weight normal
    pub font_weight_normal: u32,
    
    /// Font weight medium
    pub font_weight_medium: u32,
    
    /// Font weight bold
    pub font_weight_bold: u32,
    
    /// Line height
    pub line_height: f32,
    
    /// Letter spacing
    pub letter_spacing: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "Inter, system-ui, sans-serif".to_string(),
            font_size_base: 16,
            font_size_small: 14,
            font_size_large: 18,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
            line_height: 1.5,
            letter_spacing: 0.0,
        }
    }
}

/// Spacing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    /// Spacing unit
    pub unit: u32,
    
    /// Spacing small
    pub small: u32,
    
    /// Spacing medium
    pub medium: u32,
    
    /// Spacing large: u32,
    
    /// Spacing extra large
    pub extra_large: u32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            unit: 4,
            small: 8,
            medium: 16,
            large: 24,
            extra_large: 32,
        }
    }
}

/// Border radius settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderRadius {
    /// Small radius
    pub small: u32,
    
    /// Medium radius
    pub medium: u32,
    
    /// Large radius
    pub large: u32,
    
    /// Extra large radius
    pub extra_large: u32,
    
    /// Full radius
    pub full: u32,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            small: 4,
            medium: 8,
            large: 12,
            extra_large: 16,
            full: 9999,
        }
    }
}

/// Shadow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadows {
    /// Small shadow
    pub small: String,
    
    /// Medium shadow
    pub medium: String,
    
    /// Large shadow
    pub large: String,
    
    /// Extra large shadow
    pub extra_large: String,
}

impl Default for Shadows {
    fn default() -> Self {
        Self {
            small: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            medium: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
            large: "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string(),
            extra_large: "0 20px 25px -5px rgb(0 0 0 / 0.1)".to_string(),
        }
    }
}

/// Component styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyles {
    /// Button styles
    pub button: ButtonStyles,
    
    /// Input styles
    pub input: InputStyles,
    
    /// Card styles
    pub card: CardStyles,
    
    /// Navigation styles
    pub navigation: NavigationStyles,
}

/// Button styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyles {
    /// Primary button background
    pub primary_background: String,
    
    /// Primary button text
    pub primary_text: String,
    
    /// Secondary button background
    pub secondary_background: String,
    
    /// Secondary button text
    pub secondary_text: String,
    
    /// Border radius
    pub border_radius: u32,
    
    /// Padding
    pub padding: String,
}

impl Default for ButtonStyles {
    fn default() -> Self {
        Self {
            primary_background: "#6366f1".to_string(),
            primary_text: "#ffffff".to_string(),
            secondary_background: "#334155".to_string(),
            secondary_text: "#f8fafc".to_string(),
            border_radius: 8,
            padding: "12px 24px".to_string(),
        }
    }
}

/// Input styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputStyles {
    /// Background color
    pub background: String,
    
    /// Border color
    pub border_color: String,
    
    /// Border color focus
    pub border_color_focus: String,
    
    /// Text color
    pub text_color: String,
    
    /// Placeholder color
    pub placeholder_color: String,
    
    /// Border radius
    pub border_radius: u32,
    
    /// Padding
    pub padding: String,
}

impl Default for InputStyles {
    fn default() -> Self {
        Self {
            background: "#1e293b".to_string(),
            border_color: "#334155".to_string(),
            border_color_focus: "#6366f1".to_string(),
            text_color: "#f8fafc".to_string(),
            placeholder_color: "#94a3b8".to_string(),
            border_radius: 8,
            padding: "12px 16px".to_string(),
        }
    }
}

/// Card styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStyles {
    /// Background color
    pub background: String,
    
    /// Border color
    pub border_color: String,
    
    /// Border radius
    pub border_radius: u32,
    
    /// Padding
    pub padding: String,
    
    /// Shadow
    pub shadow: String,
}

impl Default for CardStyles {
    fn default() -> Self {
        Self {
            background: "#1e293b".to_string(),
            border_color: "#334155".to_string(),
            border_radius: 12,
            padding: "24px".to_string(),
            shadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
        }
    }
}

/// Navigation styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationStyles {
    /// Background color
    pub background: String,
    
    /// Text color
    pub text_color: String,
    
    /// Active text color
    pub active_text_color: String,
    
    /// Border color
    pub border_color: String,
    
    /// Padding
    pub padding: String,
}

impl Default for NavigationStyles {
    fn default() -> Self {
        Self {
            background: "#0f172a".to_string(),
            text_color: "#94a3b8".to_string(),
            active_text_color: "#6366f1".to_string(),
            border_color: "#334155".to_string(),
            padding: "16px 24px".to_string(),
        }
    }
}

impl Default for ComponentStyles {
    fn default() -> Self {
        Self {
            button: ButtonStyles::default(),
            input: InputStyles::default(),
            card: CardStyles::default(),
            navigation: NavigationStyles::default(),
        }
    }
}

impl Default for ThemeSpecification {
    fn default() -> Self {
        Self {
            name: "Default Dark".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Default dark theme for Vantis Media Player".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    /// Current theme
    current_theme: ThemeSpecification,
    
    /// Available themes
    available_themes: HashMap<String, ThemeSpecification>,
    
    /// Theme presets
    presets: Vec<ThemeSpecification>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let current_theme = ThemeSpecification::default();
        let presets = Self::create_presets();
        
        let mut available_themes = HashMap::new();
        for preset in &presets {
            available_themes.insert(preset.name.clone(), preset.clone());
        }
        
        Self {
            current_theme,
            available_themes,
            presets,
        }
    }
    
    /// Create theme presets
    fn create_presets() -> Vec<ThemeSpecification> {
        vec![
            Self::create_dark_theme(),
            Self::create_light_theme(),
            Self::create_midnight_theme(),
            Self::create_ocean_theme(),
            Self::create_forest_theme(),
            Self::create_sunset_theme(),
        ]
    }
    
    /// Create dark theme preset
    fn create_dark_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Dark".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Dark theme with high contrast".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
    
    /// Create light theme preset
    fn create_light_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Light".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Light theme for daytime use".to_string(),
            theme_type: ThemeType::Light,
            colors: ColorPalette {
                primary: "#6366f1".to_string(),
                secondary: "#8b5cf6".to_string(),
                accent: "#ec4899".to_string(),
                background: "#ffffff".to_string(),
                surface: "#f8fafc".to_string(),
                text: "#0f172a".to_string(),
                text_secondary: "#64748b".to_string(),
                border: "#e2e8f0".to_string(),
                success: "#22c55e".to_string(),
                warning: "#f59e0b".to_string(),
                error: "#ef4444".to_string(),
                info: "#3b82f6".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles {
                button: ButtonStyles {
                    primary_background: "#6366f1".to_string(),
                    primary_text: "#ffffff".to_string(),
                    secondary_background: "#e2e8f0".to_string(),
                    secondary_text: "#0f172a".to_string(),
                    border_radius: 8,
                    padding: "12px 24px".to_string(),
                },
                input: InputStyles {
                    background: "#ffffff".to_string(),
                    border_color: "#e2e8f0".to_string(),
                    border_color_focus: "#6366f1".to_string(),
                    text_color: "#0f172a".to_string(),
                    placeholder_color: "#94a3b8".to_string(),
                    border_radius: 8,
                    padding: "12px 16px".to_string(),
                },
                card: CardStyles {
                    background: "#ffffff".to_string(),
                    border_color: "#e2e8f0".to_string(),
                    border_radius: 12,
                    padding: "24px".to_string(),
                    shadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
                },
                navigation: NavigationStyles {
                    background: "#ffffff".to_string(),
                    text_color: "#64748b".to_string(),
                    active_text_color: "#6366f1".to_string(),
                    border_color: "#e2e8f0".to_string(),
                    padding: "16px 24px".to_string(),
                },
            },
            custom_css: None,
        }
    }
    
    /// Create midnight theme preset
    fn create_midnight_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Midnight".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Deep blue midnight theme".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette {
                primary: "#3b82f6".to_string(),
                secondary: "#1e40af".to_string(),
                accent: "#60a5fa".to_string(),
                background: "#020617".to_string(),
                surface: "#0f172a".to_string(),
                text: "#f1f5f9".to_string(),
                text_secondary: "#94a3b8".to_string(),
                border: "#1e293b".to_string(),
                success: "#10b981".to_string(),
                warning: "#f59e0b".to_string(),
                error: "#ef4444".to_string(),
                info: "#3b82f6".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
    
    /// Create ocean theme preset
    fn create_ocean_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Ocean".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Calm ocean blue theme".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette {
                primary: "#0ea5e9".to_string(),
                secondary: "#06b6d4".to_string(),
                accent: "#38bdf8".to_string(),
                background: "#0c4a6e".to_string(),
                surface: "#075985".to_string(),
                text: "#f0f9ff".to_string(),
                text_secondary: "#bae6fd".to_string(),
                border: "#0369a1".to_string(),
                success: "#22c55e".to_string(),
                warning: "#f59e0b".to_string(),
                error: "#ef4444".to_string(),
                info: "#0ea5e9".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
    
    /// Create forest theme preset
    fn create_forest_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Forest".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Natural forest green theme".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette {
                primary: "#22c55e".to_string(),
                secondary: "#16a34a".to_string(),
                accent: "#4ade80".to_string(),
                background: "#14532d".to_string(),
                surface: "#166534".to_string(),
                text: "#f0fdf4".to_string(),
                text_secondary: "#bbf7d0".to_string(),
                border: "#15803d".to_string(),
                success: "#22c55e".to_string(),
                warning: "#f59e0b".to_string(),
                error: "#ef4444".to_string(),
                info: "#3b82f6".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
    
    /// Create sunset theme preset
    fn create_sunset_theme() -> ThemeSpecification {
        ThemeSpecification {
            name: "Sunset".to_string(),
            version: "1.0.0".to_string(),
            author: "Vantis Team".to_string(),
            description: "Warm sunset orange theme".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette {
                primary: "#f97316".to_string(),
                secondary: "#ea580c".to_string(),
                accent: "#fb923c".to_string(),
                background: "#431407".to_string(),
                surface: "#7c2d12".to_string(),
                text: "#fff7ed".to_string(),
                text_secondary: "#fed7aa".to_string(),
                border: "#9a3412".to_string(),
                success: "#22c55e".to_string(),
                warning: "#f59e0b".to_string(),
                error: "#ef4444".to_string(),
                info: "#3b82f6".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            border_radius: BorderRadius::default(),
            shadows: Shadows::default(),
            components: ComponentStyles::default(),
            custom_css: None,
        }
    }
    
    /// Get current theme
    pub fn current_theme(&self) -> &ThemeSpecification {
        &self.current_theme
    }
    
    /// Set current theme
    pub fn set_theme(&mut self, theme: ThemeSpecification) {
        info!("Setting theme: {}", theme.name);
        self.current_theme = theme;
    }
    
    /// Set theme by name
    pub fn set_theme_by_name(&mut self, name: &str) -> Result<()> {
        if let Some(theme) = self.available_themes.get(name) {
            self.set_theme(theme.clone());
            Ok(())
        } else {
            anyhow::bail!("Theme '{}' not found", name)
        }
    }
    
    /// Get available themes
    pub fn available_themes(&self) -> &HashMap<String, ThemeSpecification> {
        &self.available_themes
    }
    
    /// Get theme presets
    pub fn presets(&self) -> &[ThemeSpecification] {
        &self.presets
    }
    
    /// Add custom theme
    pub fn add_theme(&mut self, theme: ThemeSpecification) {
        info!("Adding custom theme: {}", theme.name);
        self.available_themes.insert(theme.name.clone(), theme);
    }
    
    /// Remove theme
    pub fn remove_theme(&mut self, name: &str) -> Result<()> {
        if self.presets.iter().any(|p| p.name == name) {
            anyhow::bail!("Cannot remove preset theme '{}'", name)
        }
        
        if self.available_themes.remove(name).is_some() {
            info!("Removed theme: {}", name);
            Ok(())
        } else {
            anyhow::bail!("Theme '{}' not found", name)
        }
    }
    
    /// Export theme to file
    pub fn export_theme(&self, name: &str, path: &Path) -> Result<()> {
        let theme = self.available_themes.get(name)
            .context(format!("Theme '{}' not found", name))?;
        
        let json = serde_json::to_string_pretty(theme)
            .context("Failed to serialize theme")?;
        
        fs::write(path, json)
            .context(format!("Failed to write theme to {:?}", path))?;
        
        info!("Exported theme '{}' to {:?}", name, path);
        Ok(())
    }
    
    /// Import theme from file
    pub fn import_theme(&mut self, path: &Path) -> Result<()> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read theme from {:?}", path))?;
        
        let theme: ThemeSpecification = serde_json::from_str(&json)
            .context("Failed to parse theme")?;
        
        info!("Imported theme '{}' from {:?}", theme.name, path);
        self.add_theme(theme);
        Ok(())
    }
    
    /// Validate theme
    pub fn validate_theme(&self, theme: &ThemeSpecification) -> Result<()> {
        // Validate theme name
        if theme.name.is_empty() {
            anyhow::bail!("Theme name cannot be empty");
        }
        
        // Validate colors (basic hex format check)
        Self::validate_color(&theme.colors.primary)?;
        Self::validate_color(&theme.colors.secondary)?;
        Self::validate_color(&theme.colors.accent)?;
        Self::validate_color(&theme.colors.background)?;
        Self::validate_color(&theme.colors.surface)?;
        Self::validate_color(&theme.colors.text)?;
        Self::validate_color(&theme.colors.text_secondary)?;
        Self::validate_color(&theme.colors.border)?;
        
        Ok(())
    }
    
    /// Validate color format
    fn validate_color(color: &str) -> Result<()> {
        if !color.starts_with('#') || color.len() != 7 {
            anyhow::bail!("Invalid color format: '{}'. Expected hex format (#RRGGBB)", color);
        }
        
        // Check if it's valid hex
        color[1..].chars().all(|c| c.is_ascii_hexdigit())
            .then_some(())
            .context(format!("Invalid hex color: '{}'", color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert!(!manager.available_themes().is_empty());
        assert_eq!(manager.presets().len(), 6);
    }
    
    #[test]
    fn test_set_theme_by_name() {
        let mut manager = ThemeManager::new();
        assert!(manager.set_theme_by_name("Dark").is_ok());
        assert!(manager.set_theme_by_name("Light").is_ok());
        assert!(manager.set_theme_by_name("NonExistent").is_err());
    }
    
    #[test]
    fn test_add_custom_theme() {
        let mut manager = ThemeManager::new();
        let custom_theme = ThemeSpecification {
            name: "Custom".to_string(),
            ..Default::default()
        };
        
        manager.add_theme(custom_theme);
        assert!(manager.available_themes().contains_key("Custom"));
    }
    
    #[test]
    fn test_remove_theme() {
        let mut manager = ThemeManager::new();
        let custom_theme = ThemeSpecification {
            name: "Custom".to_string(),
            ..Default::default()
        };
        
        manager.add_theme(custom_theme);
        assert!(manager.remove_theme("Custom").is_ok());
        assert!(!manager.available_themes().contains_key("Custom"));
    }
    
    #[test]
    fn test_remove_preset_theme() {
        let mut manager = ThemeManager::new();
        assert!(manager.remove_theme("Dark").is_err());
    }
    
    #[test]
    fn test_validate_theme() {
        let manager = ThemeManager::new();
        let theme = ThemeSpecification::default();
        assert!(manager.validate_theme(&theme).is_ok());
    }
    
    #[test]
    fn test_validate_invalid_color() {
        let manager = ThemeManager::new();
        let mut theme = ThemeSpecification::default();
        theme.colors.primary = "invalid".to_string();
        assert!(manager.validate_theme(&theme).is_err());
    }
    
    #[test]
    fn test_theme_presets() {
        let manager = ThemeManager::new();
        let presets = manager.presets();
        
        assert_eq!(presets.len(), 6);
        assert!(presets.iter().any(|p| p.name == "Dark"));
        assert!(presets.iter().any(|p| p.name == "Light"));
        assert!(presets.iter().any(|p| p.name == "Midnight"));
        assert!(presets.iter().any(|p| p.name == "Ocean"));
        assert!(presets.iter().any(|p| p.name == "Forest"));
        assert!(presets.iter().any(|p| p.name == "Sunset"));
    }
}