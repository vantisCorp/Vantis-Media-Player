//! Internationalization (I18n) Module
//! 
//! Multi-language support for Vantis Media Player.
//! Supports 8 languages: EN, PL, DE, ZH, RU, KO, ES, FR
//!
//! Usage:
//! ```
//! use ui::i18n::{I18n, Language};
//! 
//! let i18n = I18n::new(Language::English);
//! let text = i18n.t("common.play"); // "Play"
//! ```

mod language;
mod translator;
mod loader;

pub use language::Language;
pub use translator::Translator;
pub use loader::Loader;

use std::sync::Arc;
use parking_lot::RwLock;

/// Global i18n instance
static I18N: once_cell::sync::Lazy<Arc<RwLock<I18n>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(I18n::default())));

/// Main I18n struct
#[derive(Debug, Clone)]
pub struct I18n {
    /// Current language
    language: Language,
    /// Translator instance
    translator: Translator,
    /// Fallback language (default: English)
    fallback: Language,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            language: Language::default(),
            translator: Translator::new(Language::default()),
            fallback: Language::English,
        }
    }
}

impl I18n {
    /// Create a new I18n instance with the specified language
    pub fn new(language: Language) -> Self {
        Self {
            language,
            translator: Translator::new(language),
            fallback: Language::English,
        }
    }
    
    /// Get the current language
    pub fn language(&self) -> Language {
        self.language
    }
    
    /// Set the current language
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.translator = Translator::new(language);
    }
    
    /// Translate a key to the current language
    pub fn t(&self, key: &str) -> String {
        self.translator.translate(key)
    }
    
    /// Translate with interpolation
    /// 
    /// Example:
    /// ```
    /// i18n.t_with("errors.file_not_found", &[("file", "video.mp4")])
    /// // Returns: "File 'video.mp4' not found"
    /// ```
    pub fn t_with(&self, key: &str, params: &[(&str, &str)]) -> String {
        let mut text = self.t(key);
        for (key, value) in params {
            text = text.replace(&format!("{{{}}}", key), value);
        }
        text
    }
    
    /// Get available languages
    pub fn available_languages() -> Vec<Language> {
        Language::all().to_vec()
    }
    
    /// Detect system language
    pub fn detect_system_language() -> Language {
        Language::from_system()
    }
}

/// Get the global I18n instance
pub fn i18n() -> Arc<RwLock<I18n>> {
    I18N.clone()
}

/// Quick translate function using global instance
pub fn t(key: &str) -> String {
    i18n().read().t(key)
}

/// Translate with parameters
pub fn t_with(key: &str, params: &[(&str, &str)]) -> String {
    i18n().read().t_with(key, params)
}

/// Set the global language
pub fn set_language(language: Language) {
    i18n().write().set_language(language);
}

/// Get the current global language
pub fn current_language() -> Language {
    i18n().read().language()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_language() {
        let i18n = I18n::default();
        assert_eq!(i18n.language(), Language::English);
    }
    
    #[test]
    fn test_language_change() {
        let mut i18n = I18n::new(Language::English);
        assert_eq!(i18n.language(), Language::English);
        
        i18n.set_language(Language::Polish);
        assert_eq!(i18n.language(), Language::Polish);
    }
    
    #[test]
    fn test_available_languages() {
        let languages = I18n::available_languages();
        assert_eq!(languages.len(), 8);
        assert!(languages.contains(&Language::English));
        assert!(languages.contains(&Language::Polish));
    }
}