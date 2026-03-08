//! Streaming Provider Trait and Core Types
//! 
//! Defines the interface for streaming service providers.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::{Service, StreamingError, StreamingResult};

/// Streaming provider trait - implemented by each service
#[async_trait]
pub trait StreamingProvider: Send + Sync {
    /// Get the service type
    fn service(&self) -> Service;
    
    /// Get service information
    fn info(&self) -> ServiceInfo;
    
    /// Check if authenticated
    async fn is_authenticated(&self) -> bool;
    
    /// Authenticate with credentials
    async fn authenticate(&mut self, credentials: &Credentials) -> StreamingResult<AuthSession>;
    
    /// Refresh authentication token
    async fn refresh_token(&mut self) -> StreamingResult<AuthSession>;
    
    /// Sign out
    async fn sign_out(&mut self) -> StreamingResult<()>;
    
    /// Get user profile
    async fn get_profile(&self) -> StreamingResult<UserProfile>;
    
    /// Search for content
    async fn search(&self, query: &str, options: &SearchOptions) -> StreamingResult<Vec<ContentItem>>;
    
    /// Get content details
    async fn get_content(&self, content_id: &str) -> StreamingResult<ContentDetails>;
    
    /// Get stream URL for content
    async fn get_stream_url(&self, content_id: &str, quality: &QualityProfile) -> StreamingResult<StreamInfo>;
    
    /// Get watchlist
    async fn get_watchlist(&self) -> StreamingResult<Vec<ContentItem>>;
    
    /// Add to watchlist
    async fn add_to_watchlist(&self, content_id: &str) -> StreamingResult<()>;
    
    /// Remove from watchlist
    async fn remove_from_watchlist(&self, content_id: &str) -> StreamingResult<()>;
    
    /// Get continue watching list
    async fn get_continue_watching(&self) -> StreamingResult<Vec<ContentItem>>;
    
    /// Get recommendations
    async fn get_recommendations(&self) -> StreamingResult<Vec<ContentItem>>;
    
    /// Get available quality profiles
    fn available_qualities(&self) -> Vec<QualityProfile>;
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Username/email
    pub username: String,
    /// Password
    pub password: String,
    /// OAuth authorization code (for OAuth flows)
    pub auth_code: Option<String>,
    /// Device ID
    pub device_id: Option<String>,
}

/// Authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// User ID
    pub user_id: String,
    /// Session ID
    pub session_id: String,
}

impl AuthSession {
    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }
    
    /// Check if the session needs refresh (expires within 5 minutes)
    pub fn needs_refresh(&self) -> bool {
        let refresh_threshold = chrono::Duration::minutes(5);
        Utc::now() + refresh_threshold >= self.expires_at
    }
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service website URL
    pub website_url: String,
    /// Logo URL
    pub logo_url: String,
    /// Supported regions (ISO country codes)
    pub supported_regions: Vec<String>,
    /// Subscription tiers
    pub subscription_tiers: Vec<SubscriptionTier>,
    /// Whether the service requires subscription
    pub requires_subscription: bool,
    /// Features supported by this service
    pub features: ServiceFeatures,
}

/// Subscription tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTier {
    /// Tier ID
    pub id: String,
    /// Tier name
    pub name: String,
    /// Monthly price in cents
    pub price_monthly_cents: u32,
    /// Yearly price in cents (if available)
    pub price_yearly_cents: Option<u32>,
    /// Features included in this tier
    pub features: Vec<String>,
    /// Max streams
    pub max_streams: u32,
    /// Max resolution
    pub max_resolution: Resolution,
    /// Has ads
    pub has_ads: bool,
}

/// Service features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceFeatures {
    /// Supports 4K streaming
    pub supports_4k: bool,
    /// Supports HDR
    pub supports_hdr: bool,
    /// Supports Dolby Atmos
    pub supports_dolby_atmos: bool,
    /// Supports downloads
    pub supports_downloads: bool,
    /// Supports multiple profiles
    pub supports_profiles: bool,
    /// Supports kids profile
    pub supports_kids_profile: bool,
    /// Supports watch parties
    pub supports_watch_parties: bool,
    /// Has live TV
    pub has_live_tv: bool,
    /// Has original content
    pub has_originals: bool,
}

impl Default for ServiceFeatures {
    fn default() -> Self {
        Self {
            supports_4k: true,
            supports_hdr: true,
            supports_dolby_atmos: false,
            supports_downloads: true,
            supports_profiles: true,
            supports_kids_profile: true,
            supports_watch_parties: false,
            has_live_tv: false,
            has_originals: true,
        }
    }
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub id: String,
    /// Display name
    pub display_name: String,
    /// Email
    pub email: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Current subscription tier
    pub subscription_tier: Option<String>,
    /// Account created date
    pub created_at: DateTime<Utc>,
    /// Profile preferences
    pub preferences: HashMap<String, serde_json::Value>,
    /// Is the user in a free trial
    pub in_free_trial: bool,
    /// Free trial end date
    pub trial_ends_at: Option<DateTime<Utc>>,
}

/// Content item (basic info)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    /// Content ID
    pub id: String,
    /// Title
    pub title: String,
    /// Original title
    pub original_title: Option<String>,
    /// Description/summary
    pub description: Option<String>,
    /// Poster image URL
    pub poster_url: Option<String>,
    /// Background image URL
    pub background_url: Option<String>,
    /// Content type
    pub content_type: ContentType,
    /// Release year
    pub year: Option<u16>,
    /// Duration in minutes (for movies)
    pub duration_minutes: Option<u32>,
    /// Number of seasons (for series)
    pub seasons: Option<u32>,
    /// Rating (e.g., "PG-13")
    pub rating: Option<String>,
    /// User rating (1-10)
    pub user_rating: Option<f32>,
    /// Genres
    pub genres: Vec<String>,
    /// Available resolutions
    pub available_resolutions: Vec<Resolution>,
    /// Has HDR
    pub has_hdr: bool,
    /// Watch progress (0.0 - 1.0)
    pub watch_progress: Option<f32>,
    /// Service this content belongs to
    pub service: Service,
}

/// Content type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Movie,
    Series,
    Episode,
    Documentary,
    Anime,
    LiveEvent,
    Short,
}

/// Content details (full information)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDetails {
    /// Basic content item
    #[serde(flatten)]
    pub item: ContentItem,
    /// Cast members
    pub cast: Vec<CastMember>,
    /// Directors
    pub directors: Vec<String>,
    /// Writers
    pub writers: Vec<String>,
    /// Studio/production company
    pub studio: Option<String>,
    /// Countries
    pub countries: Vec<String>,
    /// Languages
    pub languages: Vec<String>,
    /// Subtitles available
    pub subtitles: Vec<String>,
    /// Audio tracks available
    pub audio_tracks: Vec<AudioTrack>,
    /// Related content
    pub related: Vec<ContentItem>,
    /// Seasons (for series)
    pub season_info: Option<SeasonInfo>,
    /// External IDs (IMDb, etc.)
    pub external_ids: HashMap<String, String>,
}

/// Cast member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastMember {
    /// Actor name
    pub name: String,
    /// Character played
    pub character: String,
    /// Profile image URL
    pub image_url: Option<String>,
}

/// Audio track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    /// Track ID
    pub id: String,
    /// Language name
    pub language: String,
    /// Language code
    pub language_code: String,
    /// Is surround sound
    pub is_surround: bool,
    /// Audio codec
    pub codec: Option<String>,
}

/// Season information for series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    /// Total seasons
    pub total_seasons: u32,
    /// Total episodes
    pub total_episodes: u32,
    /// Seasons
    pub seasons: Vec<Season>,
}

/// Season details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    /// Season number
    pub number: u32,
    /// Season title
    pub title: String,
    /// Number of episodes
    pub episode_count: u32,
    /// Episodes
    pub episodes: Vec<Episode>,
}

/// Episode details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Episode ID
    pub id: String,
    /// Episode number
    pub number: u32,
    /// Title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Duration in minutes
    pub duration_minutes: u32,
    /// Thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Air date
    pub air_date: Option<DateTime<Utc>>,
    /// Watch progress (0.0 - 1.0)
    pub watch_progress: Option<f32>,
}

/// Search options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Content type filter
    pub content_type: Option<ContentType>,
    /// Genre filter
    pub genre: Option<String>,
    /// Year filter
    pub year: Option<u16>,
    /// Minimum rating
    pub min_rating: Option<f32>,
    /// Maximum results
    pub max_results: Option<u32>,
    /// Offset for pagination
    pub offset: Option<u32>,
    /// Sort by
    pub sort_by: Option<SortBy>,
    /// Sort order
    pub sort_order: Option<SortOrder>,
}

/// Sort options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SortBy {
    Relevance,
    Title,
    Year,
    Rating,
    Popularity,
    ReleaseDate,
}

/// Sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Quality profile for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityProfile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Resolution
    pub resolution: Resolution,
    /// Bitrate in kbps
    pub bitrate_kbps: u32,
    /// Is HDR
    pub is_hdr: bool,
    /// Is Dolby Vision
    pub is_dolby_vision: bool,
    /// Audio quality
    pub audio_quality: AudioQuality,
}

/// Resolution options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resolution {
    SD_480p,
    HD_720p,
    FullHD_1080p,
    UHD_4K,
    UHD_8K,
}

impl Resolution {
    pub fn height(&self) -> u32 {
        match self {
            Resolution::SD_480p => 480,
            Resolution::HD_720p => 720,
            Resolution::FullHD_1080p => 1080,
            Resolution::UHD_4K => 2160,
            Resolution::UHD_8K => 4320,
        }
    }
    
    pub fn width(&self, aspect_ratio: f32) -> u32 {
        (self.height() as f32 * aspect_ratio) as u32
    }
}

/// Audio quality options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AudioQuality {
    Stereo_128,
    Stereo_256,
    Stereo_320,
    Surround_5_1_384,
    Surround_5_1_640,
    Atmos_7_1,
    Lossless,
}

/// Stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Stream URL
    pub url: String,
    /// Manifest URL (for adaptive streaming)
    pub manifest_url: Option<String>,
    /// Stream format
    pub format: StreamFormat,
    /// DRM system (if protected)
    pub drm: Option<DrmInfo>,
    /// License URL (for DRM)
    pub license_url: Option<String>,
    /// Available audio tracks
    pub audio_tracks: Vec<AudioTrack>,
    /// Available subtitles
    pub subtitles: Vec<SubtitleTrack>,
    /// Quality profile used
    pub quality: QualityProfile,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
}

/// Stream format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StreamFormat {
    Hls,
    Dash,
    SmoothStreaming,
    Progressive,
    WebRTC,
}

/// DRM information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrmInfo {
    /// DRM system type
    pub system: DrmSystem,
    /// Key ID (if applicable)
    pub key_id: Option<String>,
}

/// DRM system types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DrmSystem {
    Widevine,
    FairPlay,
    PlayReady,
    ClearKey,
}

/// Subtitle track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    /// Track ID
    pub id: String,
    /// Language name
    pub language: String,
    /// Language code
    pub language_code: String,
    /// Is forced (for foreign language parts)
    pub is_forced: bool,
    /// Is SDH (hearing impaired)
    pub is_sdh: bool,
    /// URL to subtitle file
    pub url: Option<String>,
}

/// Streaming service manager
pub struct StreamingService {
    providers: Vec<Box<dyn StreamingProvider>>,
    sessions: HashMap<Service, AuthSession>,
}

impl StreamingService {
    /// Create a new streaming service manager
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            sessions: HashMap::new(),
        }
    }
    
    /// Add a provider
    pub fn add_provider(&mut self, provider: Box<dyn StreamingProvider>) {
        self.providers.push(provider);
    }
    
    /// Get provider by service
    pub fn get_provider(&self, service: Service) -> Option<&dyn StreamingProvider> {
        self.providers.iter()
            .find(|p| p.service() == service)
            .map(|p| p.as_ref())
    }
    
    /// Get mutable provider by service
    pub fn get_provider_mut(&mut self, service: Service) -> Option<&mut Box<dyn StreamingProvider>> {
        self.providers.iter_mut()
            .find(|p| p.service() == service)
    }
    
    /// Get all available services
    pub fn available_services(&self) -> Vec<ServiceInfo> {
        self.providers.iter().map(|p| p.info()).collect()
    }
    
    /// Check if a service is authenticated
    pub fn is_authenticated(&self, service: Service) -> bool {
        self.sessions.get(&service).map(|s| !s.is_expired()).unwrap_or(false)
    }
    
    /// Get session for service
    pub fn get_session(&self, service: Service) -> Option<&AuthSession> {
        self.sessions.get(&service)
    }
}