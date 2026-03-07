//! Language definitions and utilities
//! 
//! Supported languages: EN, PL, DE, ZH, RU, KO, ES, FR

use std::str::FromStr;

/// Supported languages with native names and flag emojis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    English,    // 🇬🇧 English
    Polish,     // 🇵🇱 Polski
    German,     // 🇩🇪 Deutsch
    Chinese,    // 🇨🇳 中文
    Russian,    // 🇷🇺 Русский
    Korean,     // 🇰🇷 한국어
    Spanish,    // 🇪🇸 Español
    French,     // 🇫🇷 Français
}

impl Language {
    /// Get all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Polish,
            Language::German,
            Language::Chinese,
            Language::Russian,
            Language::Korean,
            Language::Spanish,
            Language::French,
        ]
    }
    
    /// Get the ISO 639-1 language code
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Polish => "pl",
            Language::German => "de",
            Language::Chinese => "zh",
            Language::Russian => "ru",
            Language::Korean => "ko",
            Language::Spanish => "es",
            Language::French => "fr",
        }
    }
    
    /// Get the native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Polish => "Polski",
            Language::German => "Deutsch",
            Language::Chinese => "中文",
            Language::Russian => "Русский",
            Language::Korean => "한국어",
            Language::Spanish => "Español",
            Language::French => "Français",
        }
    }
    
    /// Get the English name of the language
    pub fn english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Polish => "Polish",
            Language::German => "German",
            Language::Chinese => "Chinese",
            Language::Russian => "Russian",
            Language::Korean => "Korean",
            Language::Spanish => "Spanish",
            Language::French => "French",
        }
    }
    
    /// Get the flag emoji for the language
    pub fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::Polish => "🇵🇱",
            Language::German => "🇩🇪",
            Language::Chinese => "🇨🇳",
            Language::Russian => "🇷🇺",
            Language::Korean => "🇰🇷",
            Language::Spanish => "🇪🇸",
            Language::French => "🇫🇷",
        }
    }
    
    /// Get display string with flag and native name
    pub fn display(&self) -> String {
        format!("{} {}", self.flag(), self.native_name())
    }
    
    /// Parse from ISO 639-1 code (case-insensitive)
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "en" | "eng" => Some(Language::English),
            "pl" | "pol" => Some(Language::Polish),
            "de" | "deu" | "ger" => Some(Language::German),
            "zh" | "zho" | "chi" => Some(Language::Chinese),
            "ru" | "rus" => Some(Language::Russian),
            "ko" | "kor" => Some(Language::Korean),
            "es" | "spa" => Some(Language::Spanish),
            "fr" | "fra" | "fre" => Some(Language::French),
            _ => None,
        }
    }
    
    /// Detect system language from environment
    pub fn from_system() -> Language {
        // Try LANG environment variable (Linux/macOS)
        if let Ok(lang) = std::env::var("LANG") {
            let code = lang.split('.')
                .next()
                .unwrap_or("en")
                .split('_')
                .next()
                .unwrap_or("en");
            
            if let Some(language) = Self::from_code(code) {
                return language;
            }
        }
        
        // Try LANGUAGE environment variable
        if let Ok(languages) = std::env::var("LANGUAGE") {
            if let Some(first) = languages.split(':').next() {
                if let Some(language) = Self::from_code(first) {
                    return language;
                }
            }
        }
        
        // Default to English
        Language::English
    }
    
    /// Check if the language uses RTL (Right-to-Left) text direction
    pub fn is_rtl(&self) -> bool {
        // None of our supported languages are RTL
        // If Arabic or Hebrew were added, they would be RTL
        false
    }
    
    /// Get the text direction for this language
    pub fn text_direction(&self) -> TextDirection {
        if self.is_rtl() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        }
    }
}

/// Text direction for layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

impl FromStr for Language {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s)
            .ok_or_else(|| format!("Unknown language code: {}", s))
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.native_name(), self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_language_codes() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Polish.code(), "pl");
        assert_eq!(Language::Chinese.code(), "zh");
    }
    
    #[test]
    fn test_native_names() {
        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::Polish.native_name(), "Polski");
        assert_eq!(Language::Chinese.native_name(), "中文");
    }
    
    #[test]
    fn test_flags() {
        assert_eq!(Language::English.flag(), "🇬🇧");
        assert_eq!(Language::Polish.flag(), "🇵🇱");
        assert_eq!(Language::Korean.flag(), "🇰🇷");
    }
    
    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("PL"), Some(Language::Polish));
        assert_eq!(Language::from_code("invalid"), None);
    }
    
    #[test]
    fn test_all_languages() {
        let all = Language::all();
        assert_eq!(all.len(), 8);
    }
    
    #[test]
    fn test_display() {
        let display = Language::Polish.display();
        assert!(display.contains("🇵🇱"));
        assert!(display.contains("Polski"));
    }
}