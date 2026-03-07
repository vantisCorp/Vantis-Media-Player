//! Subtitle module for mobile

/// Subtitle configuration
#[derive(Debug, Clone)]
pub struct SubtitleConfig {
    /// Font family
    pub font_family: String,
    
    /// Font size
    pub font_size: u32,
    
    /// Text color (hex)
    pub text_color: String,
    
    /// Background color (hex)
    pub background_color: String,
    
    /// Position offset from bottom
    pub position_offset: i32,
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            font_family: "Arial".to_string(),
            font_size: 24,
            text_color: "#FFFFFF".to_string(),
            background_color: "#000000AA".to_string(),
            position_offset: 50,
        }
    }
}