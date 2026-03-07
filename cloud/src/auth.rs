//! Authentication module for cloud services
//! 
//! Provides OAuth2 and JWT-based authentication for Vantis Cloud.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use async_trait::async_trait;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: Uuid,
    
    /// User email
    pub email: String,
    
    /// Display name
    pub display_name: Option<String>,
    
    /// Avatar URL
    pub avatar_url: Option<String>,
    
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last login timestamp
    pub last_login: DateTime<Utc>,
    
    /// Subscription tier
    pub subscription_tier: SubscriptionTier,
    
    /// Account preferences
    pub preferences: UserPreferences,
}

/// Subscription tier levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionTier {
    Free,
    Pro,
    Enterprise,
}

/// User preferences for cloud sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Enable automatic sync
    pub auto_sync: bool,
    
    /// Sync frequency in minutes
    pub sync_frequency_minutes: u32,
    
    /// Devices to sync
    pub synced_devices: Vec<Uuid>,
    
    /// Data types to sync
    pub sync_types: Vec<SyncType>,
    
    /// Privacy settings
    pub privacy: PrivacySettings,
}

/// Types of data that can be synced
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncType {
    Settings,
    Playlists,
    WatchHistory,
    Subtitles,
    Bookmarks,
    Themes,
}

/// Privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Share watch history for recommendations
    pub share_watch_history: bool,
    
    /// Allow analytics
    pub allow_analytics: bool,
    
    /// Public profile
    pub public_profile: bool,
}

/// Active session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: Uuid,
    
    /// User ID
    pub user_id: Uuid,
    
    /// Device ID
    pub device_id: Uuid,
    
    /// Access token
    pub access_token: String,
    
    /// Refresh token
    pub refresh_token: String,
    
    /// Token expiration
    pub expires_at: DateTime<Utc>,
    
    /// Session creation time
    pub created_at: DateTime<Utc>,
    
    /// Device information
    pub device_info: DeviceInfo,
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device name
    pub name: String,
    
    /// Device type
    pub device_type: DeviceType,
    
    /// Operating system
    pub os: String,
    
    /// Application version
    pub app_version: String,
    
    /// IP address (for security)
    pub ip_address: Option<String>,
}

/// Device types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    TV,
    Web,
    Unknown,
}

/// Authentication provider trait
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate with email and password
    async fn login(&self, email: &str, password: &str) -> Result<Session>;
    
    /// Authenticate with OAuth2
    async fn oauth_login(&self, provider: OAuthProvider, code: &str) -> Result<Session>;
    
    /// Refresh an expired session
    async fn refresh_session(&self, refresh_token: &str) -> Result<Session>;
    
    /// Logout and invalidate session
    async fn logout(&self, session: &Session) -> Result<()>;
    
    /// Get current user info
    async fn get_user(&self, session: &Session) -> Result<User>;
    
    /// Register new user
    async fn register(&self, email: &str, password: &str, display_name: Option<&str>) -> Result<User>;
    
    /// Request password reset
    async fn request_password_reset(&self, email: &str) -> Result<()>;
    
    /// Confirm password reset
    async fn confirm_password_reset(&self, token: &str, new_password: &str) -> Result<()>;
}

/// OAuth providers supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthProvider {
    Google,
    GitHub,
    Apple,
    Microsoft,
    Discord,
}

/// JWT token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Subject (user ID)
    pub sub: String,
    
    /// Issuer
    pub iss: String,
    
    /// Audience
    pub aud: String,
    
    /// Expiration timestamp
    pub exp: usize,
    
    /// Issued at timestamp
    pub iat: usize,
    
    /// Session ID
    pub sid: String,
    
    /// Device ID
    pub did: String,
}

/// Default user preferences
impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_frequency_minutes: 15,
            synced_devices: Vec::new(),
            sync_types: vec![
                SyncType::Settings,
                SyncType::Playlists,
                SyncType::WatchHistory,
            ],
            privacy: PrivacySettings::default(),
        }
    }
}

/// Default privacy settings
impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            share_watch_history: true,
            allow_analytics: false,
            public_profile: false,
        }
    }
}