//! Translation file loader
//! 
//! Supports loading translations from JSON/YAML files.
//! Future: Hot-reload for development

use super::Language;
use std::collections::HashMap;
use std::path::Path;

/// Translation file loader
pub struct Loader {
    /// Base path for translation files
    base_path: std::path::PathBuf,
}

impl Loader {
    /// Create a new loader with the specified base path
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
    
    /// Get the default loader (uses ./i18n/locales)
    pub fn default_loader() -> Self {
        Self::new("i18n/locales")
    }
    
    /// Load translations for a language from file
    /// 
    /// Supports JSON and YAML formats.
    /// File naming convention: `en.json`, `pl.json`, etc.
    pub fn load(&self, language: Language) -> Result<HashMap<String, String>, LoadError> {
        let code = language.code();
        
        // Try JSON first
        let json_path = self.base_path.join(format!("{}.json", code));
        if json_path.exists() {
            return self.load_json(&json_path);
        }
        
        // Try YAML
        let yaml_path = self.base_path.join(format!("{}.yaml", code));
        if yaml_path.exists() {
            return self.load_yaml(&yaml_path);
        }
        
        // Try yml extension
        let yml_path = self.base_path.join(format!("{}.yml", code));
        if yml_path.exists() {
            return self.load_yaml(&yml_path);
        }
        
        Err(LoadError::FileNotFound(code.to_string()))
    }
    
    /// Load translations from a JSON file
    fn load_json(&self, path: &Path) -> Result<HashMap<String, String>, LoadError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LoadError::IoError(e.to_string()))?;
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| LoadError::ParseError(e.to_string()))?;
        
        Ok(flatten_json(&json, ""))
    }
    
    /// Load translations from a YAML file
    #[cfg(feature = "yaml")]
    fn load_yaml(&self, path: &Path) -> Result<HashMap<String, String>, LoadError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LoadError::IoError(e.to_string()))?;
        
        let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|e| LoadError::ParseError(e.to_string()))?;
        
        Ok(flatten_yaml(&yaml, ""))
    }
    
    #[cfg(not(feature = "yaml"))]
    fn load_yaml(&self, path: &Path) -> Result<HashMap<String, String>, LoadError> {
        Err(LoadError::FeatureNotEnabled("yaml".to_string()))
    }
    
    /// Check if translation file exists for a language
    pub fn exists(&self, language: Language) -> bool {
        let code = language.code();
        self.base_path.join(format!("{}.json", code)).exists()
            || self.base_path.join(format!("{}.yaml", code)).exists()
            || self.base_path.join(format!("{}.yml", code)).exists()
    }
    
    /// List available languages from translation files
    pub fn available_languages(&self) -> Vec<Language> {
        Language::all()
            .iter()
            .filter(|lang| self.exists(**lang))
            .copied()
            .collect()
    }
}

/// Flatten a JSON object into dot-notation keys
fn flatten_json(value: &serde_json::Value, prefix: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                result.extend(flatten_json(val, &new_prefix));
            }
        }
        serde_json::Value::String(s) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), s.clone());
            }
        }
        serde_json::Value::Number(n) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), n.to_string());
            }
        }
        serde_json::Value::Bool(b) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), b.to_string());
            }
        }
        serde_json::Value::Null => {}
        serde_json::Value::Array(_) => {
            // Arrays are not supported for translations
        }
    }
    
    result
}

#[cfg(feature = "yaml")]
fn flatten_yaml(value: &serde_yaml::Value, prefix: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (key, val) in map {
                if let Some(key_str) = key.as_str() {
                    let new_prefix = if prefix.is_empty() {
                        key_str.to_string()
                    } else {
                        format!("{}.{}", prefix, key_str)
                    };
                    result.extend(flatten_yaml(val, &new_prefix));
                }
            }
        }
        serde_yaml::Value::String(s) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), s.clone());
            }
        }
        serde_yaml::Value::Number(n) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), n.to_string());
            }
        }
        serde_yaml::Value::Bool(b) => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), b.to_string());
            }
        }
        serde_yaml::Value::Null => {}
        serde_yaml::Value::Sequence(_) => {}
    }
    
    result
}

/// Load error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum LoadError {
    #[error("Translation file not found for language: {0}")]
    FileNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Feature not enabled: {0}")]
    FeatureNotEnabled(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_loader() {
        let loader = Loader::default_loader();
        assert!(loader.base_path.ends_with("i18n/locales"));
    }
    
    #[test]
    fn test_flatten_json() {
        let json = serde_json::json!({
            "common": {
                "play": "Play",
                "pause": "Pause"
            },
            "nav": {
                "home": "Home"
            }
        });
        
        let flat = flatten_json(&json, "");
        
        assert_eq!(flat.get("common.play"), Some(&"Play".to_string()));
        assert_eq!(flat.get("common.pause"), Some(&"Pause".to_string()));
        assert_eq!(flat.get("nav.home"), Some(&"Home".to_string()));
    }
}