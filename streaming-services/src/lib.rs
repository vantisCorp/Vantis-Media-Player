//! Streaming Services Integration Module
//! 
//! Provides unified integration with popular streaming services
//! including Netflix, Hulu, Disney+, HBO Max, Amazon Prime Video, and more.

pub mod provider;
pub mod netflix;
pub mod hulu;
pub mod disney;
pub mod prime;
pub mod hbo;
pub mod search;
pub mod config;

pub use provider::{StreamingProvider, StreamingService, ServiceInfo};
pub use search::{UnifiedSearch, SearchResult};

/// Streaming services error types
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Content not found: {0}")]
    ContentNotFound(String),
    
    #[error("Region restricted: {0}")]
    RegionRestricted(String),
    
    #[error("Subscription required: {0}")]
    SubscriptionRequired(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Provider not configured: {0}")]
    ProviderNotConfigured(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type for streaming operations
pub type StreamingResult<T> = Result<T, StreamingError>;

/// Available streaming services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Service {
    Netflix,
    Hulu,
    DisneyPlus,
    AmazonPrime,
    HBOMax,
    Peacock,
    ParamountPlus,
    AppleTVPlus,
    Crunchyroll,
    YouTube,
    Twitch,
    Custom(u32),
}

impl Service {
    /// Get the display name of the service
    pub fn display_name(&self) -> &str {
        match self {
            Service::Netflix => "Netflix",
            Service::Hulu => "Hulu",
            Service::DisneyPlus => "Disney+",
            Service::AmazonPrime => "Amazon Prime Video",
            Service::HBOMax => "HBO Max",
            Service::Peacock => "Peacock",
            Service::ParamountPlus => "Paramount+",
            Service::AppleTVPlus => "Apple TV+",
            Service::Crunchyroll => "Crunchyroll",
            Service::YouTube => "YouTube",
            Service::Twitch => "Twitch",
            Service::Custom(id) => "Custom Service",
        }
    }
    
    /// Get the service identifier for API calls
    pub fn api_identifier(&self) -> &str {
        match self {
            Service::Netflix => "netflix",
            Service::Hulu => "hulu",
            Service::DisneyPlus => "disney",
            Service::AmazonPrime => "prime",
            Service::HBOMax => "hbo",
            Service::Peacock => "peacock",
            Service::ParamountPlus => "paramount",
            Service::AppleTVPlus => "apple",
            Service::Crunchyroll => "crunchyroll",
            Service::YouTube => "youtube",
            Service::Twitch => "twitch",
            Service::Custom(id) => "custom",
        }
    }
}