//! Streaming Services Configuration
//! 
//! Provides configuration options for streaming service integrations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Service;

/// Main streaming services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingServicesConfig {
    /// Enable streaming services integration
    pub enabled: bool,
    
    /// Default region (ISO country code)
    pub default_region: String,
    
    /// Provider configurations
    pub providers: ProviderConfigs,
    
    /// Search settings
    pub search: SearchConfig,
    
    /// Cache settings
    pub cache: CacheConfig,
    
    /// Privacy settings
    pub privacy: PrivacyConfig,
}

impl Default for StreamingServicesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_region: "US".to_string(),
            providers: ProviderConfigs::default(),
            search: SearchConfig::default(),
            cache: CacheConfig::default(),
            privacy: PrivacyConfig::default(),
        }
    }
}

/// Provider-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfigs {
    /// Netflix configuration
    pub netflix: NetflixConfig,
    
    /// Hulu configuration
    pub hulu: HuluConfig,
    
    /// Disney+ configuration
    pub disney_plus: DisneyPlusConfig,
    
    /// Amazon Prime Video configuration
    pub prime_video: PrimeVideoConfig,
    
    /// HBO Max configuration
    pub hbo_max: HBOMaxConfig,
    
    /// Custom provider configurations
    pub custom: HashMap<String, CustomProviderConfig>,
}

impl Default for ProviderConfigs {
    fn default() -> Self {
        Self {
            netflix: NetflixConfig::default(),
            hulu: HuluConfig::default(),
            disney_plus: DisneyPlusConfig::default(),
            prime_video: PrimeVideoConfig::default(),
            hbo_max: HBOMaxConfig::default(),
            custom: HashMap::new(),
        }
    }
}

/// Netflix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetflixConfig {
    /// Enable Netflix integration
    pub enabled: bool,
    
    /// Client ID for authentication
    pub client_id: Option<String>,
    
    /// Auto-login on startup
    pub auto_login: bool,
    
    /// Preferred quality
    pub preferred_quality: PreferredQuality,
    
    /// Remember playback position
    pub remember_position: bool,
}

impl Default for NetflixConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: None,
            auto_login: false,
            preferred_quality: PreferredQuality::Auto,
            remember_position: true,
        }
    }
}

/// Hulu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuluConfig {
    /// Enable Hulu integration
    pub enabled: bool,
    
    /// Client ID for authentication
    pub client_id: Option<String>,
    
    /// Auto-login on startup
    pub auto_login: bool,
    
    /// Preferred quality
    pub preferred_quality: PreferredQuality,
    
    /// Include live TV
    pub include_live_tv: bool,
}

impl Default for HuluConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: None,
            auto_login: false,
            preferred_quality: PreferredQuality::Auto,
            include_live_tv: false,
        }
    }
}

/// Disney+ configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisneyPlusConfig {
    /// Enable Disney+ integration
    pub enabled: bool,
    
    /// Client ID for authentication
    pub client_id: Option<String>,
    
    /// Auto-login on startup
    pub auto_login: bool,
    
    /// Preferred quality
    pub preferred_quality: PreferredQuality,
    
    /// Include kids profile
    pub include_kids: bool,
}

impl Default for DisneyPlusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: None,
            auto_login: false,
            preferred_quality: PreferredQuality::Auto,
            include_kids: true,
        }
    }
}

/// Amazon Prime Video configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeVideoConfig {
    /// Enable Prime Video integration
    pub enabled: bool,
    
    /// Client ID for authentication
    pub client_id: Option<String>,
    
    /// Auto-login on startup
    pub auto_login: bool,
    
    /// Preferred quality
    pub preferred_quality: PreferredQuality,
    
    /// Enable watch parties
    pub enable_watch_parties: bool,
}

impl Default for PrimeVideoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: None,
            auto_login: false,
            preferred_quality: PreferredQuality::Auto,
            enable_watch_parties: false,
        }
    }
}

/// HBO Max configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HBOMaxConfig {
    /// Enable HBO Max integration
    pub enabled: bool,
    
    /// Client ID for authentication
    pub client_id: Option<String>,
    
    /// Auto-login on startup
    pub auto_login: bool,
    
    /// Preferred quality
    pub preferred_quality: PreferredQuality,
}

impl Default for HBOMaxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: None,
            auto_login: false,
            preferred_quality: PreferredQuality::Auto,
        }
    }
}

/// Custom provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderConfig {
    /// Provider name
    pub name: String,
    
    /// API endpoint
    pub api_endpoint: String,
    
    /// Authentication type
    pub auth_type: AuthType,
    
    /// Client ID
    pub client_id: Option<String>,
    
    /// Client secret
    pub client_secret: Option<String>,
    
    /// Custom headers
    pub headers: HashMap<String, String>,
}

/// Authentication type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    OAuth2,
    ApiKey,
    BasicAuth,
    BearerToken,
    Custom,
}

/// Preferred quality setting
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PreferredQuality {
    Auto,
    Lowest,
    SD,
    HD,
    FullHD,
    UHD4K,
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Enable unified search across all services
    pub unified_search: bool,
    
    /// Search timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Maximum concurrent searches
    pub max_concurrent: usize,
    
    /// Enable search suggestions
    pub enable_suggestions: bool,
    
    /// Maximum suggestions to show
    pub max_suggestions: usize,
    
    /// Search history enabled
    pub history_enabled: bool,
    
    /// Maximum history entries
    pub max_history: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            unified_search: true,
            timeout_ms: 10000,
            max_concurrent: 5,
            enable_suggestions: true,
            max_suggestions: 10,
            history_enabled: true,
            max_history: 100,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache directory
    pub directory: String,
    
    /// Maximum cache size in MB
    pub max_size_mb: u32,
    
    /// Cache expiration time in seconds
    pub expiration_seconds: u64,
    
    /// Cache metadata only (not content)
    pub metadata_only: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: ".cache/streaming".to_string(),
            max_size_mb: 500,
            expiration_seconds: 86400, // 24 hours
            metadata_only: true,
        }
    }
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Store watch history
    pub store_watch_history: bool,
    
    /// Share viewing data with services
    pub share_viewing_data: bool,
    
    /// Enable recommendations
    pub enable_recommendations: bool,
    
    /// Clear credentials on exit
    pub clear_credentials_on_exit: bool,
    
    /// Use VPN/proxy for requests
    pub use_proxy: bool,
    
    /// Proxy URL
    pub proxy_url: Option<String>,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            store_watch_history: true,
            share_viewing_data: false,
            enable_recommendations: true,
            clear_credentials_on_exit: false,
            use_proxy: false,
            proxy_url: None,
        }
    }
}

impl StreamingServicesConfig {
    /// Load configuration from file
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        
        let config: StreamingServicesConfig = if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?
        };
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<(), ConfigError> {
        let content = if path.ends_with(".toml") {
            toml::to_string_pretty(self)
                .map_err(|e| ConfigError::SerializeError(e.to_string()))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| ConfigError::SerializeError(e.to_string()))?
        };
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get enabled services list
    pub fn enabled_services(&self) -> Vec<Service> {
        let mut services = Vec::new();
        
        if self.providers.netflix.enabled {
            services.push(Service::Netflix);
        }
        if self.providers.hulu.enabled {
            services.push(Service::Hulu);
        }
        if self.providers.disney_plus.enabled {
            services.push(Service::DisneyPlus);
        }
        if self.providers.prime_video.enabled {
            services.push(Service::AmazonPrime);
        }
        if self.providers.hbo_max.enabled {
            services.push(Service::HBOMax);
        }
        
        services
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.default_region.len() != 2 {
            return Err(ConfigError::ValidationError(
                "default_region must be a 2-letter ISO country code".to_string()
            ));
        }
        
        if self.cache.max_size_mb == 0 {
            return Err(ConfigError::ValidationError(
                "cache.max_size_mb must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Serialize error: {0}")]
    SerializeError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = StreamingServicesConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_region, "US");
    }
    
    #[test]
    fn test_enabled_services() {
        let config = StreamingServicesConfig::default();
        let services = config.enabled_services();
        assert!(services.contains(&Service::Netflix));
        assert!(services.contains(&Service::DisneyPlus));
    }
    
    #[test]
    fn test_config_validation() {
        let config = StreamingServicesConfig::default();
        assert!(config.validate().is_ok());
    }
}