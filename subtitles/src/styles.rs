//! Subtitle Style Customization Module
//!
//! This module provides comprehensive subtitle style customization features including:
//! - Style editor UI
//! - Font customization (family, size, weight, style)
//! - Color customization (text color, background color, outline color)
//! - Position adjustment (vertical position, alignment)
//! - Style presets
//! - Style export/import

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Subtitle style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleStyle {
    /// Font family (e.g., "Arial", "Roboto", "DejaVu Sans")
    pub font_family: String,
    
    /// Font size in pixels
    pub font_size: u32,
    
    /// Font weight (100-900, where 400 is normal, 700 is bold)
    pub font_weight: u32,
    
    /// Font style (normal, italic, oblique)
    pub font_style: FontStyle,
    
    /// Text color (hex color, e.g., "#FFFFFF")
    pub text_color: String,
    
    /// Background color (hex color, e.g., "#000000" for transparent/black)
    pub background_color: String,
    
    /// Background opacity (0.0-1.0)
    pub background_opacity: f32,
    
    /// Outline color (hex color)
    pub outline_color: String,
    
    /// Outline width in pixels
    pub outline_width: u32,
    
    /// Shadow color (hex color)
    pub shadow_color: String,
    
    /// Shadow blur in pixels
    pub shadow_blur: u32,
    
    /// Shadow offset x in pixels
    pub shadow_offset_x: i32,
    
    /// Shadow offset y in pixels
    pub shadow_offset_y: i32,
    
    /// Vertical position (0-100, where 0 is top, 50 is center, 100 is bottom)
    pub vertical_position: u8,
    
    /// Horizontal alignment
    pub horizontal_alignment: Alignment,
    
    /// Line spacing (1.0-3.0, where 1.0 is normal)
    pub line_spacing: f32,
    
    /// Character spacing (pixels)
    pub character_spacing: i32,
    
    /// Border radius (pixels)
    pub border_radius: u32,
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    /// Normal font style
    Normal,
    /// Italic font style
    Italic,
    /// Oblique font style
    Oblique,
}

/// Horizontal alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    /// Left alignment
    Left,
    /// Center alignment
    Center,
    /// Right alignment,
}

impl Alignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

impl Default for SubtitleStyle {
    fn default() -> Self {
        Self {
            font_family: "DejaVu Sans".to_string(),
            font_size: 24,
            font_weight: 400,
            font_style: FontStyle::Normal,
            text_color: "#FFFFFF".to_string(),
            background_color: "#000000".to_string(),
            background_opacity: 0.7,
            outline_color: "#000000".to_string(),
            outline_width: 2,
            shadow_color: "#000000".to_string(),
            shadow_blur: 2,
            shadow_offset_x: 1,
            shadow_offset_y: 1,
            vertical_position: 85,
            horizontal_alignment: Alignment::Center,
            line_spacing: 1.2,
            character_spacing: 0,
            border_radius: 4,
        }
    }
}

impl SubtitleStyle {
    /// Create a new subtitle style with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new subtitle style with custom values
    pub fn with_values(
        font_family: String,
        font_size: u32,
        text_color: String,
    ) -> Self {
        Self {
            font_family,
            font_size,
            text_color,
            ..Default::default()
        }
    }
    
    /// Get CSS representation of the style
    pub fn to_css(&self) -> String {
        format!(
            "font-family: '{}'; font-size: {}px; font-weight: {}; font-style: {}; \
            color: {}; background-color: {}; background-opacity: {}; \
            outline: {}px solid {}; text-shadow: {}px {}px {}px {}; \
            vertical-align: {}%; text-align: {}; line-height: {}; letter-spacing: {}px; border-radius: {}px;",
            self.font_family,
            self.font_size,
            self.font_weight,
            self.font_style.as_str(),
            self.text_color,
            self.background_color,
            self.background_opacity,
            self.outline_width,
            self.outline_color,
            self.shadow_offset_x,
            self.shadow_offset_y,
            self.shadow_blur,
            self.shadow_color,
            self.vertical_position,
            self.horizontal_alignment.as_str(),
            self.line_spacing,
            self.character_spacing,
            self.border_radius,
        )
    }
    
    /// Validate the style
    pub fn validate(&self) -> Result<()> {
        // Validate font size
        if self.font_size < 8 || self.font_size > 72 {
            return Err(anyhow::anyhow!("Font size must be between 8 and 72 pixels"));
        }
        
        // Validate font weight
        if self.font_weight < 100 || self.font_weight > 900 {
            return Err(anyhow::anyhow!("Font weight must be between 100 and 900"));
        }
        
        // Validate vertical position
        if self.vertical_position > 100 {
            return Err(anyhow::anyhow!("Vertical position must be between 0 and 100"));
        }
        
        // Validate line spacing
        if self.line_spacing < 1.0 || self.line_spacing > 3.0 {
            return Err(anyhow::anyhow!("Line spacing must be between 1.0 and 3.0"));
        }
        
        // Validate background opacity
        if self.background_opacity < 0.0 || self.background_opacity > 1.0 {
            return Err(anyhow::anyhow!("Background opacity must be between 0.0 and 1.0"));
        }
        
        Ok(())
    }
}

impl FontStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Italic => "italic",
            Self::Oblique => "oblique",
        }
    }
    
    pub fn all() -> Vec<FontStyle> {
        vec![
            FontStyle::Normal,
            FontStyle::Italic,
            FontStyle::Oblique,
        ]
    }
}

/// Style preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylePreset {
    /// Preset name
    pub name: String,
    
    /// Preset description
    pub description: String,
    
    /// Style configuration
    pub style: SubtitleStyle,
    
    /// Category (e.g., "Classic", "Modern", "Minimal")
    pub category: String,
}

/// Style editor configuration
#[derive(Debug, Clone)]
pub struct StyleEditorConfig {
    /// Enable style presets
    pub enable_presets: bool,
    
    /// Enable custom font selection
    pub enable_custom_fonts: bool,
    
    /// Available font families
    pub available_fonts: Vec<String>,
    
    /// Enable color picker
    pub enable_color_picker: bool,
    
    /// Enable background customization
    pub enable_background: bool,
    
    /// Enable outline customization
    pub enable_outline: bool,
    
    /// Enable shadow customization
    pub enable_shadow: bool,
    
    /// Enable position adjustment
    pub enable_position: bool,
}

impl Default for StyleEditorConfig {
    fn default() -> Self {
        Self {
            enable_presets: true,
            enable_custom_fonts: true,
            available_fonts: vec![
                "DejaVu Sans".to_string(),
                "Roboto".to_string(),
                "Arial".to_string(),
                "Open Sans".to_string(),
                "Lato".to_string(),
                "Source Sans Pro".to_string(),
                "Noto Sans".to_string(),
            ],
            enable_color_picker: true,
            enable_background: true,
            enable_outline: true,
            enable_shadow: true,
            enable_position: true,
        }
    }
}

/// Style editor UI state
#[derive(Debug, Clone)]
pub struct StyleEditorState {
    /// Current style being edited
    pub current_style: SubtitleStyle,
    
    /// Selected preset (if any)
    pub selected_preset: Option<String>,
    
    /// Editor configuration
    pub config: StyleEditorConfig,
    
    /// Has unsaved changes
    pub has_unsaved_changes: bool,
}

/// Style manager
pub struct StyleManager {
    /// Available presets
    presets: Arc<RwLock<HashMap<String, StylePreset>>>,
    
    /// Custom styles
    custom_styles: Arc<RwLock<HashMap<String, SubtitleStyle>>>,
    
    /// Current active style
    current_style: Arc<RwLock<SubtitleStyle>>,
    
    /// Editor state
    editor_state: Arc<RwLock<StyleEditorState>>,
}

impl StyleManager {
    /// Create a new style manager
    pub fn new() -> Self {
        info!("🎨 Initializing Style Manager");
        
        let presets = Self::create_default_presets();
        let current_style = SubtitleStyle::default();
        let config = StyleEditorConfig::default();
        
        Self {
            presets: Arc::new(RwLock::new(presets)),
            custom_styles: Arc::new(RwLock::new(HashMap::new())),
            current_style: Arc::new(RwLock::new(current_style)),
            editor_state: Arc::new(RwLock::new(StyleEditorState {
                current_style,
                selected_preset: None,
                config,
                has_unsaved_changes: false,
            })),
        }
    }
    
    /// Create default style presets
    fn create_default_presets() -> HashMap<String, StylePreset> {
        let mut presets = HashMap::new();
        
        // Classic preset
        presets.insert(
            "classic".to_string(),
            StylePreset {
                name: "Classic".to_string(),
                description: "Classic subtitle style with white text on black background".to_string(),
                category: "Classic".to_string(),
                style: SubtitleStyle {
                    font_family: "DejaVu Sans".to_string(),
                    font_size: 24,
                    font_weight: 400,
                    font_style: FontStyle::Normal,
                    text_color: "#FFFFFF".to_string(),
                    background_color: "#000000".to_string(),
                    background_opacity: 0.7,
                    outline_color: "#000000".to_string(),
                    outline_width: 2,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 2,
                    shadow_offset_x: 1,
                    shadow_offset_y: 1,
                    vertical_position: 85,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.2,
                    character_spacing: 0,
                    border_radius: 4,
                },
            },
        );
        
        // Modern preset
        presets.insert(
            "modern".to_string(),
            StylePreset {
                name: "Modern".to_string(),
                description: "Modern style with clean look and subtle shadow".to_string(),
                category: "Modern".to_string(),
                style: SubtitleStyle {
                    font_family: "Roboto".to_string(),
                    font_size: 28,
                    font_weight: 500,
                    font_style: FontStyle::Normal,
                    text_color: "#FFFFFF".to_string(),
                    background_color: "#1A1A1A".to_string(),
                    background_opacity: 0.8,
                    outline_color: "#FFFFFF".to_string(),
                    outline_width: 0,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 4,
                    shadow_offset_x: 0,
                    shadow_offset_y: 2,
                    vertical_position: 90,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.3,
                    character_spacing: 0,
                    border_radius: 8,
                },
            },
        );
        
        // Minimal preset
        presets.insert(
            "minimal".to_string(),
            StylePreset {
                name: "Minimal".to_string(),
                description: "Minimal style with no background and subtle outline".to_string(),
                category: "Minimal".to_string(),
                style: SubtitleStyle {
                    font_family: "Open Sans".to_string(),
                    font_size: 26,
                    font_weight: 300,
                    font_style: FontStyle::Normal,
                    text_color: "#FFFFFF".to_string(),
                    background_color: "#000000".to_string(),
                    background_opacity: 0.0,
                    outline_color: "#000000".to_string(),
                    outline_width: 3,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 0,
                    shadow_offset_x: 0,
                    shadow_offset_y: 0,
                    vertical_position: 88,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.2,
                    character_spacing: 0,
                    border_radius: 0,
                },
            },
        );
        
        // Bold preset
        presets.insert(
            "bold".to_string(),
            StylePreset {
                name: "Bold".to_string(),
                description: "Bold style with larger font and strong outline".to_string(),
                category: "Classic".to_string(),
                style: SubtitleStyle {
                    font_family: "Arial".to_string(),
                    font_size: 32,
                    font_weight: 700,
                    font_style: FontStyle::Normal,
                    text_color: "#FFFFFF".to_string(),
                    background_color: "#000000".to_string(),
                    background_opacity: 0.75,
                    outline_color: "#000000".to_string(),
                    outline_width: 4,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 2,
                    shadow_offset_x: 1,
                    shadow_offset_y: 1,
                    vertical_position: 87,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.2,
                    character_spacing: 0,
                    border_radius: 4,
                },
            },
        );
        
        // Cinematic preset
        presets.insert(
            "cinematic".to_string(),
            StylePreset {
                name: "Cinematic".to_string(),
                description: "Cinematic style with yellow text and film-like appearance".to_string(),
                category: "Cinematic".to_string(),
                style: SubtitleStyle {
                    font_family: "Lato".to_string(),
                    font_size: 30,
                    font_weight: 600,
                    font_style: FontStyle::Normal,
                    text_color: "#FFD700".to_string(),
                    background_color: "#000000".to_string(),
                    background_opacity: 0.6,
                    outline_color: "#000000".to_string(),
                    outline_width: 3,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 3,
                    shadow_offset_x: 1,
                    shadow_offset_y: 2,
                    vertical_position: 85,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.3,
                    character_spacing: 1,
                    border_radius: 2,
                },
            },
        );
        
        // Rounded preset
        presets.insert(
            "rounded".to_string(),
            StylePreset {
                name: "Rounded".to_string(),
                description: "Rounded style with soft corners and pastel colors".to_string(),
                category: "Modern".to_string(),
                style: SubtitleStyle {
                    font_family: "Noto Sans".to_string(),
                    font_size: 26,
                    font_weight: 400,
                    font_style: FontStyle::Normal,
                    text_color: "#FFFFFF".to_string(),
                    background_color: "#2C3E50".to_string(),
                    background_opacity: 0.85,
                    outline_color: "#2C3E50".to_string(),
                    outline_width: 0,
                    shadow_color: "#000000".to_string(),
                    shadow_blur: 4,
                    shadow_offset_x: 0,
                    shadow_offset_y: 2,
                    vertical_position: 89,
                    horizontal_alignment: Alignment::Center,
                    line_spacing: 1.2,
                    character_spacing: 0,
                    border_radius: 12,
                },
            },
        );
        
        presets
    }
    
    /// Get all presets
    pub async fn get_presets(&self) -> Vec<StylePreset> {
        let presets = self.presets.read().await;
        presets.values().cloned().collect()
    }
    
    /// Get preset by name
    pub async fn get_preset(&self, name: &str) -> Option<StylePreset> {
        let presets = self.presets.read().await;
        presets.get(name).cloned()
    }
    
    /// Apply preset
    pub async fn apply_preset(&self, name: &str) -> Result<()> {
        info!("🎨 Applying preset: {}", name);
        
        let preset = self.get_preset(name).await
            .context(format!("Preset '{}' not found", name))?;
        
        let mut current_style = self.current_style.write().await;
        *current_style = preset.style.clone();
        
        let mut editor_state = self.editor_state.write().await;
        editor_state.current_style = preset.style.clone();
        editor_state.selected_preset = Some(name.to_string());
        editor_state.has_unsaved_changes = false;
        
        debug!("✅ Preset applied: {}", name);
        Ok(())
    }
    
    /// Get current style
    pub async fn get_current_style(&self) -> SubtitleStyle {
        let current_style = self.current_style.read().await;
        current_style.clone()
    }
    
    /// Set current style
    pub async fn set_current_style(&self, style: SubtitleStyle) -> Result<()> {
        style.validate()?;
        
        let mut current_style = self.current_style.write().await;
        *current_style = style.clone();
        
        let mut editor_state = self.editor_state.write().await;
        editor_state.current_style = style;
        editor_state.has_unsaved_changes = true;
        editor_state.selected_preset = None;
        
        debug!("✅ Current style updated");
        Ok(())
    }
    
    /// Save custom style
    pub async fn save_custom_style(&self, name: String, style: SubtitleStyle) -> Result<()> {
        style.validate()?;
        
        info!("🎨 Saving custom style: {}", name);
        
        let mut custom_styles = self.custom_styles.write().await;
        custom_styles.insert(name.clone(), style);
        
        debug!("✅ Custom style saved: {}", name);
        Ok(())
    }
    
    /// Load custom style
    pub async fn load_custom_style(&self, name: &str) -> Result<SubtitleStyle> {
        let custom_styles = self.custom_styles.read().await;
        
        custom_styles
            .get(name)
            .cloned()
            .context(format!("Custom style '{}' not found", name))
    }
    
    /// Get all custom styles
    pub async fn get_custom_styles(&self) -> HashMap<String, SubtitleStyle> {
        let custom_styles = self.custom_styles.read().await;
        custom_styles.clone()
    }
    
    /// Delete custom style
    pub async fn delete_custom_style(&self, name: &str) -> Result<()> {
        info!("🎨 Deleting custom style: {}", name);
        
        let mut custom_styles = self.custom_styles.write().await;
        if custom_styles.remove(name).is_some() {
            debug!("✅ Custom style deleted: {}", name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Custom style '{}' not found", name))
        }
    }
    
    /// Export style to JSON
    pub async fn export_style(&self, style: &SubtitleStyle) -> Result<String> {
        serde_json::to_string_pretty(style)
            .context("Failed to serialize style to JSON")
    }
    
    /// Import style from JSON
    pub async fn import_style(&self, json: &str) -> Result<SubtitleStyle> {
        let style: SubtitleStyle = serde_json::from_str(json)
            .context("Failed to parse style JSON")?;
        
        style.validate()?;
        
        Ok(style)
    }
    
    /// Get editor state
    pub async fn get_editor_state(&self) -> StyleEditorState {
        let editor_state = self.editor_state.read().await;
        editor_state.clone()
    }
    
    /// Update editor state
    pub async fn update_editor_state(&self, state: StyleEditorState) -> Result<()> {
        state.current_style.validate()?;
        
        let mut editor_state = self.editor_state.write().await;
        *editor_state = state;
        
        let mut current_style = self.current_style.write().await;
        *current_style = state.current_style;
        
        debug!("✅ Editor state updated");
        Ok(())
    }
    
    /// Reset to default style
    pub async fn reset_to_default(&self) -> Result<()> {
        info!("🎨 Resetting to default style");
        
        let default_style = SubtitleStyle::default();
        self.set_current_style(default_style).await?;
        
        let mut editor_state = self.editor_state.write().await;
        editor_state.selected_preset = None;
        editor_state.has_unsaved_changes = false;
        
        debug!("✅ Reset to default style");
        Ok(())
    }
    
    /// Get available fonts
    pub async fn get_available_fonts(&self) -> Vec<String> {
        let editor_state = self.editor_state.read().await;
        editor_state.config.available_fonts.clone()
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_style_manager_creation() {
        let manager = StyleManager::new();
        let presets = tokio::runtime::Runtime::new().unwrap().block_on(manager.get_presets());
        
        assert_eq!(presets.len(), 6);
        assert!(presets.iter().any(|p| p.name == "classic"));
    }
    
    #[test]
    fn test_default_style() {
        let style = SubtitleStyle::default();
        
        assert_eq!(style.font_family, "DejaVu Sans");
        assert_eq!(style.font_size, 24);
        assert_eq!(style.text_color, "#FFFFFF");
        assert_eq!(style.vertical_position, 85);
    }
    
    #[test]
    fn test_style_validation() {
        let style = SubtitleStyle::default();
        assert!(style.validate().is_ok());
        
        let mut invalid_style = SubtitleStyle::default();
        invalid_style.font_size = 100; // Too large
        assert!(invalid_style.validate().is_err());
    }
    
    #[test]
    fn test_style_to_css() {
        let style = SubtitleStyle::default();
        let css = style.to_css();
        
        assert!(css.contains("font-family"));
        assert!(css.contains("font-size"));
        assert!(css.contains("color"));
    }
    
    #[tokio::test]
    async fn test_apply_preset() {
        let manager = StyleManager::new();
        
        assert!(manager.apply_preset("classic").await.is_ok());
        
        let style = manager.get_current_style().await;
        assert_eq!(style.font_family, "DejaVu Sans");
    }
    
    #[tokio::test]
    async fn test_save_custom_style() {
        let manager = StyleManager::new();
        
        let custom_style = SubtitleStyle::with_values(
            "Custom Font".to_string(),
            30,
            "#FF0000".to_string(),
        );
        
        assert!(manager.save_custom_style("my_style".to_string(), custom_style).await.is_ok());
        
        let loaded = manager.load_custom_style("my_style").await;
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap().font_family, "Custom Font");
    }
    
    #[tokio::test]
    async fn test_export_import_style() {
        let manager = StyleManager::new();
        
        let style = SubtitleStyle::default();
        let json = manager.export_style(&style).await;
        assert!(json.is_ok());
        
        let imported = manager.import_style(&json.unwrap()).await;
        assert!(imported.is_ok());
        assert_eq!(imported.unwrap().font_family, style.font_family);
    }
    
    #[tokio::test]
    async fn test_reset_to_default() {
        let manager = StyleManager::new();
        
        // Apply a preset
        manager.apply_preset("bold").await.unwrap();
        
        // Reset to default
        assert!(manager.reset_to_default().await.is_ok());
        
        let style = manager.get_current_style().await;
        assert_eq!(style.font_family, "DejaVu Sans");
    }
    
    #[test]
    fn test_font_style_all() {
        let styles = FontStyle::all();
        assert_eq!(styles.len(), 3);
        assert!(styles.contains(&FontStyle::Normal));
        assert!(styles.contains(&FontStyle::Italic));
        assert!(styles.contains(&FontStyle::Oblique));
    }
}