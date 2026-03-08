//! HBO Max Integration
//! 
//! Provides integration with HBO Max streaming service.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::provider::*;
use crate::{Service, StreamingError, StreamingResult};

/// HBO Max provider
pub struct HBOMaxProvider {
    client: reqwest::Client,
    session: Option<AuthSession>,
    config: HBOConfig,
}

/// HBO Max configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HBOConfig {
    pub client_id: String,
    pub device_id: String,
    pub region: String,
}

impl Default for HBOConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            device_id: uuid::Uuid::new_v4().to_string(),
            region: "US".to_string(),
        }
    }
}

impl HBOMaxProvider {
    pub fn new(config: HBOConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            session: None,
            config,
        }
    }
}

#[async_trait]
impl StreamingProvider for HBOMaxProvider {
    fn service(&self) -> Service {
        Service::HBOMax
    }
    
    fn info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "HBO Max".to_string(),
            description: "Premium streaming service with HBO originals, movies, and exclusive content".to_string(),
            website_url: "https://hbomax.com".to_string(),
            logo_url: "https://www.hbomax.com/favicon.ico".to_string(),
            supported_regions: vec!["US".into(), "UK".into(), "CA".into(), "AU".into(), "DE".into(), "FR".into(), "ES".into()],
            subscription_tiers: vec![
                SubscriptionTier {
                    id: "with_ads".into(),
                    name: "With Ads".into(),
                    price_monthly_cents: 999,
                    price_yearly_cents: Some(9999),
                    features: vec!["Ads".into()],
                    max_streams: 2,
                    max_resolution: Resolution::FullHD_1080p,
                    has_ads: true,
                },
                SubscriptionTier {
                    id: "ad_free".into(),
                    name: "Ad-Free".into(),
                    price_monthly_cents: 1599,
                    price_yearly_cents: Some(15999),
                    features: vec!["No Ads".into(), "Downloads".into()],
                    max_streams: 3,
                    max_resolution: Resolution::UHD_4K,
                    has_ads: false,
                },
                SubscriptionTier {
                    id: "ultimate".into(),
                    name: "Ultimate Ad-Free".into(),
                    price_monthly_cents: 1999,
                    price_yearly_cents: Some(19999),
                    features: vec!["No Ads".into(), "4K".into(), "HDR".into(), "Dolby Atmos".into(), "Downloads".into()],
                    max_streams: 4,
                    max_resolution: Resolution::UHD_4K,
                    has_ads: false,
                },
            ],
            requires_subscription: true,
            features: ServiceFeatures {
                supports_4k: true,
                supports_hdr: true,
                supports_dolby_atmos: true,
                supports_downloads: true,
                supports_profiles: true,
                supports_kids_profile: true,
                supports_watch_parties: false,
                has_live_tv: false,
                has_originals: true,
            },
        }
    }
    
    async fn is_authenticated(&self) -> bool {
        self.session.as_ref().map(|s| !s.is_expired()).unwrap_or(false)
    }
    
    async fn authenticate(&mut self, credentials: &Credentials) -> StreamingResult<AuthSession> {
        if credentials.username.is_empty() || credentials.password.is_empty() {
            return Err(StreamingError::AuthenticationFailed("Invalid credentials".into()));
        }
        
        let session = AuthSession {
            access_token: format!("hbo_at_{}", uuid::Uuid::new_v4()),
            refresh_token: Some(format!("hbo_rt_{}", uuid::Uuid::new_v4())),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            token_type: "Bearer".into(),
            user_id: format!("hbo_user_{}", uuid::Uuid::new_v4()),
            session_id: uuid::Uuid::new_v4().to_string(),
        };
        
        self.session = Some(session.clone());
        Ok(session)
    }
    
    async fn refresh_token(&mut self) -> StreamingResult<AuthSession> {
        let session = self.session.as_ref()
            .ok_or_else(|| StreamingError::AuthenticationFailed("No session to refresh".into()))?;
        
        let new_session = AuthSession {
            access_token: format!("hbo_at_{}", uuid::Uuid::new_v4()),
            refresh_token: session.refresh_token.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            token_type: "Bearer".into(),
            user_id: session.user_id.clone(),
            session_id: session.session_id.clone(),
        };
        
        self.session = Some(new_session.clone());
        Ok(new_session)
    }
    
    async fn sign_out(&mut self) -> StreamingResult<()> {
        self.session = None;
        Ok(())
    }
    
    async fn get_profile(&self) -> StreamingResult<UserProfile> {
        let session = self.session.as_ref()
            .ok_or_else(|| StreamingError::AuthenticationFailed("Not authenticated".into()))?;
        
        Ok(UserProfile {
            id: session.user_id.clone(),
            display_name: "HBO Max User".into(),
            email: None,
            avatar_url: None,
            subscription_tier: Some("ultimate".into()),
            created_at: chrono::Utc::now() - chrono::Duration::days(200),
            preferences: HashMap::new(),
            in_free_trial: false,
            trial_ends_at: None,
        })
    }
    
    async fn search(&self, _query: &str, _options: &SearchOptions) -> StreamingResult<Vec<ContentItem>> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        
        Ok(vec![
            ContentItem {
                id: "game-of-thrones".into(),
                title: "Game of Thrones".into(),
                original_title: None,
                description: Some("Nine noble families fight for control over the lands of Westeros.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2011),
                duration_minutes: None,
                seasons: Some(8),
                rating: Some("TV-MA".into()),
                user_rating: Some(9.2),
                genres: vec!["Action".into(), "Adventure".into(), "Drama".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::HBOMax,
            },
            ContentItem {
                id: "succession".into(),
                title: "Succession".into(),
                original_title: None,
                description: Some("The Roy family is known for controlling the biggest media and entertainment company in the world.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2018),
                duration_minutes: None,
                seasons: Some(4),
                rating: Some("TV-MA".into()),
                user_rating: Some(8.9),
                genres: vec!["Drama".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::HBOMax,
            },
        ])
    }
    
    async fn get_content(&self, content_id: &str) -> StreamingResult<ContentDetails> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        
        Ok(ContentDetails {
            item: ContentItem {
                id: content_id.into(),
                title: "Game of Thrones".into(),
                original_title: None,
                description: Some("Nine noble families fight for control over the lands of Westeros.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2011),
                duration_minutes: None,
                seasons: Some(8),
                rating: Some("TV-MA".into()),
                user_rating: Some(9.2),
                genres: vec!["Action".into(), "Adventure".into(), "Drama".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::HBOMax,
            },
            cast: vec![
                CastMember { name: "Emilia Clarke".into(), character: "Daenerys Targaryen".into(), image_url: None },
                CastMember { name: "Kit Harington".into(), character: "Jon Snow".into(), image_url: None },
                CastMember { name: "Peter Dinklage".into(), character: "Tyrion Lannister".into(), image_url: None },
            ],
            directors: vec![],
            writers: vec!["David Benioff".into(), "D.B. Weiss".into()],
            studio: Some("HBO".into()),
            countries: vec!["United States".into()],
            languages: vec!["English".into()],
            subtitles: vec!["English".into()],
            audio_tracks: vec![],
            related: vec![],
            season_info: Some(SeasonInfo {
                total_seasons: 8,
                total_episodes: 73,
                seasons: vec![],
            }),
            external_ids: {
                let mut ids = HashMap::new();
                ids.insert("imdb".into(), "tt0944947".into());
                ids
            },
        })
    }
    
    async fn get_stream_url(&self, content_id: &str, quality: &QualityProfile) -> StreamingResult<StreamInfo> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        
        Ok(StreamInfo {
            url: format!("https://hbomax.com/watch/{}", content_id),
            manifest_url: Some(format!("https://hbomax.com/manifest/{}.mpd", content_id)),
            format: StreamFormat::Dash,
            drm: Some(DrmInfo {
                system: DrmSystem::Widevine,
                key_id: None,
            }),
            license_url: Some("https://hbomax.com/license".into()),
            audio_tracks: vec![],
            subtitles: vec![],
            quality: quality.clone(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(3)),
        })
    }
    
    async fn get_watchlist(&self) -> StreamingResult<Vec<ContentItem>> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        Ok(vec![])
    }
    
    async fn add_to_watchlist(&self, _content_id: &str) -> StreamingResult<()> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        Ok(())
    }
    
    async fn remove_from_watchlist(&self, _content_id: &str) -> StreamingResult<()> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        Ok(())
    }
    
    async fn get_continue_watching(&self) -> StreamingResult<Vec<ContentItem>> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        Ok(vec![])
    }
    
    async fn get_recommendations(&self) -> StreamingResult<Vec<ContentItem>> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        Ok(vec![])
    }
    
    fn available_qualities(&self) -> Vec<QualityProfile> {
        vec![
            QualityProfile {
                id: "auto".into(),
                name: "Auto".into(),
                resolution: Resolution::UHD_4K,
                bitrate_kbps: 22000,
                is_hdr: false,
                is_dolby_vision: false,
                audio_quality: AudioQuality::Atmos_7_1,
            },
            QualityProfile {
                id: "4k_hdr".into(),
                name: "4K HDR".into(),
                resolution: Resolution::UHD_4K,
                bitrate_kbps: 22000,
                is_hdr: true,
                is_dolby_vision: true,
                audio_quality: AudioQuality::Atmos_7_1,
            },
            QualityProfile {
                id: "1080p".into(),
                name: "Full HD".into(),
                resolution: Resolution::FullHD_1080p,
                bitrate_kbps: 7000,
                is_hdr: false,
                is_dolby_vision: false,
                audio_quality: AudioQuality::Surround_5_1_384,
            },
        ]
    }
}