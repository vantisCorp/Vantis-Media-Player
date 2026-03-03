//! Machine Translation Module
//!
//! This module provides machine translation features for subtitles including:
//! - Translation API integration (Google Translate, DeepL)
//! - Translation caching
//! - Language selection
//! - Batch translation
//! - Translation quality indicators

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Translation configuration
#[derive(Debug, Clone)]
pub struct TranslationConfig {
    /// Translation service
    pub service: TranslationService,
    
    /// API key for translation service
    pub api_key: Option<String>,
    
    /// Enable caching
    pub enable_cache: bool,
    
    /// Cache size limit
    pub cache_size_limit: usize,
    
    /// Default source language
    pub default_source_language: String,
    
    /// Default target language
    pub default_target_language: String,
    
    /// Enable quality indicators
    pub enable_quality_indicators: bool,
    
    /// Batch size for batch translation
    pub batch_size: usize,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            service: TranslationService::GoogleTranslate,
            api_key: None,
            enable_cache: true,
            cache_size_limit: 1000,
            default_source_language: "auto".to_string(),
            default_target_language: "en".to_string(),
            enable_quality_indicators: true,
            batch_size: 50,
        }
    }
}

/// Translation service
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationService {
    /// Google Translate
    GoogleTranslate,
    
    /// DeepL
    DeepL,
    
    /// LibreTranslate (free, self-hosted)
    LibreTranslate,
}

impl TranslationService {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GoogleTranslate => "Google Translate",
            Self::DeepL => "DeepL",
            Self::LibreTranslate => "LibreTranslate",
        }
    }
}

/// Language information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Language code (e.g., "en", "es", "fr")
    pub code: String,
    
    /// Language name (e.g., "English", "Spanish", "French")
    pub name: String,
    
    /// Native name
    pub native_name: String,
}

/// Translation result
#[derive(Debug, Clone)]
pub struct TranslationResult {
    /// Original text
    pub original_text: String,
    
    /// Translated text
    pub translated_text: String,
    
    /// Source language
    pub source_language: String,
    
    /// Target language
    pub target_language: String,
    
    /// Quality score (0.0-1.0)
    pub quality_score: f32,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    
    /// Translation service used
    pub service: TranslationService,
    
    /// Cached result
    pub cached: bool,
}

/// Batch translation result
#[derive(Debug, Clone)]
pub struct BatchTranslationResult {
    /// Individual translation results
    pub results: Vec<TranslationResult>,
    
    /// Total items
    pub total_items: usize,
    
    /// Successful translations
    pub successful: usize,
    
    /// Failed translations
    pub failed: usize,
    
    /// Average quality score
    pub average_quality: f32,
}

/// Translation cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Translated text
    translated_text: String,
    
    /// Quality score
    quality_score: f32,
    
    /// Timestamp
    timestamp: std::time::Instant,
}

/// Machine translator
pub struct MachineTranslator {
    /// Configuration
    config: TranslationConfig,
    
    /// Translation cache
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    
    /// Supported languages
    languages: Vec<Language>,
}

impl MachineTranslator {
    /// Create a new machine translator
    pub fn new(config: TranslationConfig) -> Self {
        let languages = Self::create_supported_languages();
        
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            languages,
        }
    }
    
    /// Create supported languages
    fn create_supported_languages() -> Vec<Language> {
        vec![
            Language {
                code: "auto".to_string(),
                name: "Auto Detect".to_string(),
                native_name: "Auto Detect".to_string(),
            },
            Language {
                code: "en".to_string(),
                name: "English".to_string(),
                native_name: "English".to_string(),
            },
            Language {
                code: "es".to_string(),
                name: "Spanish".to_string(),
                native_name: "Español".to_string(),
            },
            Language {
                code: "fr".to_string(),
                name: "French".to_string(),
                native_name: "Français".to_string(),
            },
            Language {
                code: "de".to_string(),
                name: "German".to_string(),
                native_name: "Deutsch".to_string(),
            },
            Language {
                code: "it".to_string(),
                name: "Italian".to_string(),
                native_name: "Italiano".to_string(),
            },
            Language {
                code: "pt".to_string(),
                name: "Portuguese".to_string(),
                native_name: "Português".to_string(),
            },
            Language {
                code: "ru".to_string(),
                name: "Russian".to_string(),
                native_name: "Русский".to_string(),
            },
            Language {
                code: "zh".to_string(),
                name: "Chinese".to_string(),
                native_name: "中文".to_string(),
            },
            Language {
                code: "ja".to_string(),
                name: "Japanese".to_string(),
                native_name: "日本語".to_string(),
            },
            Language {
                code: "ko".to_string(),
                name: "Korean".to_string(),
                native_name: "한국어".to_string(),
            },
            Language {
                code: "ar".to_string(),
                name: "Arabic".to_string(),
                native_name: "العربية".to_string(),
            },
            Language {
                code: "hi".to_string(),
                name: "Hindi".to_string(),
                native_name: "हिन्दी".to_string(),
            },
            Language {
                code: "pl".to_string(),
                name: "Polish".to_string(),
                native_name: "Polski".to_string(),
            },
            Language {
                code: "nl".to_string(),
                name: "Dutch".to_string(),
                native_name: "Nederlands".to_string(),
            },
            Language {
                code: "tr".to_string(),
                name: "Turkish".to_string(),
                native_name: "Türkçe".to_string(),
            },
            Language {
                code: "vi".to_string(),
                name: "Vietnamese".to_string(),
                native_name: "Tiếng Việt".to_string(),
            },
            Language {
                code: "th".to_string(),
                name: "Thai".to_string(),
                native_name: "ไทย".to_string(),
            },
            Language {
                code: "id".to_string(),
                name: "Indonesian".to_string(),
                native_name: "Bahasa Indonesia".to_string(),
            },
            Language {
                code: "sv".to_string(),
                name: "Swedish".to_string(),
                native_name: "Svenska".to_string(),
            },
        ]
    }
    
    /// Get supported languages
    pub fn supported_languages(&self) -> &[Language] {
        &self.languages
    }
    
    /// Get language by code
    pub fn get_language(&self, code: &str) -> Option<&Language> {
        self.languages.iter().find(|l| l.code == code)
    }
    
    /// Translate text
    pub async fn translate(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<TranslationResult> {
        info!("Translating text from '{}' to '{}'", source_language, target_language);
        
        // Check cache first
        if self.config.enable_cache {
            if let Some(cached) = self.check_cache(text, source_language, target_language).await {
                debug!("Translation found in cache");
                return Ok(cached);
            }
        }
        
        // Perform translation
        let result = match self.config.service {
            TranslationService::GoogleTranslate => {
                self.translate_google(text, source_language, target_language).await?
            }
            TranslationService::DeepL => {
                self.translate_deepl(text, source_language, target_language).await?
            }
            TranslationService::LibreTranslate => {
                self.translate_libre(text, source_language, target_language).await?
            }
        };
        
        // Cache result
        if self.config.enable_cache {
            self.cache_result(text, source_language, target_language, &result).await;
        }
        
        Ok(result)
    }
    
    /// Translate using Google Translate
    async fn translate_google(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<TranslationResult> {
        // In a real implementation, this would call the Google Translate API
        // For now, simulate translation
        let translated_text = self.simulate_translation(text, target_language);
        
        Ok(TranslationResult {
            original_text: text.to_string(),
            translated_text,
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
            quality_score: 0.95,
            confidence: 0.98,
            service: TranslationService::GoogleTranslate,
            cached: false,
        })
    }
    
    /// Translate using DeepL
    async fn translate_deepl(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<TranslationResult> {
        // In a real implementation, this would call the DeepL API
        // For now, simulate translation
        let translated_text = self.simulate_translation(text, target_language);
        
        Ok(TranslationResult {
            original_text: text.to_string(),
            translated_text,
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
            quality_score: 0.97,
            confidence: 0.99,
            service: TranslationService::DeepL,
            cached: false,
        })
    }
    
    /// Translate using LibreTranslate
    async fn translate_libre(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<TranslationResult> {
        // In a real implementation, this would call the LibreTranslate API
        // For now, simulate translation
        let translated_text = self.simulate_translation(text, target_language);
        
        Ok(TranslationResult {
            original_text: text.to_string(),
            translated_text,
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
            quality_score: 0.85,
            confidence: 0.90,
            service: TranslationService::LibreTranslate,
            cached: false,
        })
    }
    
    /// Simulate translation (for testing)
    fn simulate_translation(&self, text: &str, target_language: &str) -> String {
        if target_language == "en" {
            text.to_string()
        } else {
            format!("[{}] {}", target_language, text)
        }
    }
    
    /// Batch translate
    pub async fn batch_translate(
        &self,
        texts: &[String],
        source_language: &str,
        target_language: &str,
    ) -> Result<BatchTranslationResult> {
        info!("Batch translating {} texts from '{}' to '{}'", texts.len(), source_language, target_language);
        
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;
        let mut total_quality = 0.0;
        
        for text in texts {
            match self.translate(text, source_language, target_language).await {
                Ok(result) => {
                    total_quality += result.quality_score;
                    results.push(result);
                    successful += 1;
                }
                Err(e) => {
                    warn!("Translation failed for text: {}", e);
                    failed += 1;
                }
            }
        }
        
        let average_quality = if successful > 0 {
            total_quality / successful as f32
        } else {
            0.0
        };
        
        Ok(BatchTranslationResult {
            results,
            total_items: texts.len(),
            successful,
            failed,
            average_quality,
        })
    }
    
    /// Check cache
    async fn check_cache(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Option<TranslationResult> {
        let cache_key = format!("{}:{}:{}", source_language, target_language, text);
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(&cache_key) {
            Some(TranslationResult {
                original_text: text.to_string(),
                translated_text: entry.translated_text.clone(),
                source_language: source_language.to_string(),
                target_language: target_language.to_string(),
                quality_score: entry.quality_score,
                confidence: 1.0,
                service: self.config.service,
                cached: true,
            })
        } else {
            None
        }
    }
    
    /// Cache result
    async fn cache_result(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
        result: &TranslationResult,
    ) {
        let cache_key = format!("{}:{}:{}", source_language, target_language, text);
        let mut cache = self.cache.write().await;
        
        // Check cache size limit
        if cache.len() >= self.config.cache_size_limit {
            // Remove oldest entry (simplified)
            if let Some(key) = cache.keys().next() {
                cache.remove(key);
            }
        }
        
        cache.insert(
            cache_key,
            CacheEntry {
                translated_text: result.translated_text.clone(),
                quality_score: result.quality_score,
                timestamp: std::time::Instant::now(),
            },
        );
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) {
        info!("Clearing translation cache");
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// Get cache size
    pub async fn cache_size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }
    
    /// Get configuration
    pub fn config(&self) -> &TranslationConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: TranslationConfig) {
        info!("Updating translation configuration");
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_translator_creation() {
        let config = TranslationConfig::default();
        let translator = MachineTranslator::new(config);
        
        assert_eq!(translator.supported_languages().len(), 20);
        assert!(translator.get_language("en").is_some());
    }
    
    #[tokio::test]
    async fn test_translate() {
        let config = TranslationConfig::default();
        let translator = MachineTranslator::new(config);
        
        let result = translator.translate("Hello", "en", "es").await.unwrap();
        
        assert_eq!(result.original_text, "Hello");
        assert_eq!(result.source_language, "en");
        assert_eq!(result.target_language, "es");
        assert!(!result.cached);
    }
    
    #[tokio::test]
    async fn test_translate_with_cache() {
        let mut config = TranslationConfig::default();
        config.enable_cache = true;
        let translator = MachineTranslator::new(config);
        
        // First translation
        let result1 = translator.translate("Hello", "en", "es").await.unwrap();
        assert!(!result1.cached);
        
        // Second translation (should be cached)
        let result2 = translator.translate("Hello", "en", "es").await.unwrap();
        assert!(result2.cached);
    }
    
    #[tokio::test]
    async fn test_batch_translate() {
        let config = TranslationConfig::default();
        let translator = MachineTranslator::new(config);
        
        let texts = vec![
            "Hello".to_string(),
            "World".to_string(),
            "Test".to_string(),
        ];
        
        let result = translator.batch_translate(&texts, "en", "es").await.unwrap();
        
        assert_eq!(result.total_items, 3);
        assert_eq!(result.successful, 3);
        assert_eq!(result.failed, 0);
    }
    
    #[tokio::test]
    async fn test_clear_cache() {
        let mut config = TranslationConfig::default();
        config.enable_cache = true;
        let translator = MachineTranslator::new(config);
        
        // Add to cache
        translator.translate("Hello", "en", "es").await.unwrap();
        assert_eq!(translator.cache_size().await, 1);
        
        // Clear cache
        translator.clear_cache().await;
        assert_eq!(translator.cache_size().await, 0);
    }
    
    #[test]
    fn test_get_language() {
        let config = TranslationConfig::default();
        let translator = MachineTranslator::new(config);
        
        let lang = translator.get_language("en");
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "English");
        
        let lang = translator.get_language("invalid");
        assert!(lang.is_none());
    }
}