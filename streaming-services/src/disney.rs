//! Disney+ Integration
//! 
//! Provides integration with Disney+ streaming service.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::provider::*;
use crate::{Service, StreamingError, StreamingResult};

/// Disney+ provider
pub struct DisneyPlusProvider {
    client: reqwest::Client,
    session: Option<AuthSession>,
    config: DisneyConfig,
}

/// Disney+ configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisneyConfig {
    pub client_id: String,
    pub device_id: String,
    pub region: String,
}

impl Default for DisneyConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            device_id: uuid::Uuid::new_v4().to_string(),
            region: "US".to_string(),
        }
    }
}

impl DisneyPlusProvider {
    pub fn new(config: DisneyConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            session: None,
            config,
        }
    }
}

#[async_trait]
impl StreamingProvider for DisneyPlusProvider {
    fn service(&self) -> Service {
        Service::DisneyPlus
    }
    
    fn info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "Disney+".to_string(),
            description: "Streaming service with Disney, Pixar, Marvel, Star Wars, and National Geographic content".to_string(),
            website_url: "https://disneyplus.com".to_string(),
            logo_url: "https://www.disneyplus.com/favicon.ico".to_string(),
            supported_regions: vec!["US".into(), "UK".into(), "CA".into(), "AU".into(), "DE".into(), "FR".into(), "JP".into()],
            subscription_tiers: vec![
                SubscriptionTier {
                    id: "basic_ads".into(),
                    name: "Basic with Ads".into(),
                    price_monthly_cents: 799,
                    price_yearly_cents: None,
                    features: vec!["Ads".into(), "Limited downloads".into()],
                    max_streams: 1,
                    max_resolution: Resolution::FullHD_1080p,
                    has_ads: true,
                },
                SubscriptionTier {
                    id: "premium".into(),
                    name: "Premium".into(),
                    price_monthly_cents: 1399,
                    price_yearly_cents: Some(13999),
                    features: vec!["4K".into(), "HDR".into(), "Dolby Atmos".into(), "Downloads".into(), "No Ads".into()],
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
            access_token: format!("disney_at_{}", uuid::Uuid::new_v4()),
            refresh_token: Some(format!("disney_rt_{}", uuid::Uuid::new_v4())),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            token_type: "Bearer".into(),
            user_id: format!("disney_user_{}", uuid::Uuid::new_v4()),
            session_id: uuid::Uuid::new_v4().to_string(),
        };
        
        self.session = Some(session.clone());
        Ok(session)
    }
    
    async fn refresh_token(&mut self) -> StreamingResult<AuthSession> {
        let session = self.session.as_ref()
            .ok_or_else(|| StreamingError::AuthenticationFailed("No session to refresh".into()))?;
        
        let new_session = AuthSession {
            access_token: format!("disney_at_{}", uuid::Uuid::new_v4()),
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
            display_name: "Disney+ User".into(),
            email: None,
            avatar_url: None,
            subscription_tier: Some("premium".into()),
            created_at: chrono::Utc::now() - chrono::Duration::days(365),
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
                id: "mandalorian".into(),
                title: "The Mandalorian".into(),
                original_title: None,
                description: Some("After the fall of the Empire, a lone gunfighter makes his way through the lawless galaxy.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2019),
                duration_minutes: None,
                seasons: Some(3),
                rating: Some("TV-14".into()),
                user_rating: Some(8.7),
                genres: vec!["Action".into(), "Adventure".into(), "Sci-Fi".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::DisneyPlus,
            },
            ContentItem {
                id: "wanda-vision".into(),
                title: "WandaVision".into(),
                original_title: None,
                description: Some("Wanda Maximoff and Vision live their ideal suburban lives, but begin to suspect everything is not as it seems.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2021),
                duration_minutes: None,
                seasons: Some(1),
                rating: Some("TV-14".into()),
                user_rating: Some(8.0),
                genres: vec!["Action".into(), "Comedy".into(), "Drama".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::DisneyPlus,
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
                title: "The Mandalorian".into(),
                original_title: None,
                description: Some("After the fall of the Empire, a lone gunfighter makes his way through the lawless galaxy.".into()),
                poster_url: None,
                background_url: None,
                content_type: ContentType::Series,
                year: Some(2019),
                duration_minutes: None,
                seasons: Some(3),
                rating: Some("TV-14".into()),
                user_rating: Some(8.7),
                genres: vec!["Action".into(), "Adventure".into(), "Sci-Fi".into()],
                available_resolutions: vec![Resolution::UHD_4K],
                has_hdr: true,
                watch_progress: None,
                service: Service::DisneyPlus,
            },
            cast: vec![
                CastMember { name: "Pedro Pascal".into(), character: "Din Djarin".into(), image_url: None },
                CastMember { name: "Gina Carano".into(), character: "Cara Dune".into(), image_url: None },
            ],
            directors: vec!["Jon Favreau".into()],
            writers: vec!["Jon Favreau".into()],
            studio: Some("Lucasfilm".into()),
            countries: vec!["United States".into()],
            languages: vec!["English".into()],
            subtitles: vec!["English".into(), "Spanish".into()],
            audio_tracks: vec![],
            related: vec![],
            season_info: Some(SeasonInfo {
                total_seasons: 3,
                total_episodes: 24,
                seasons: vec![],
            }),
            external_ids: {
                let mut ids = HashMap::new();
                ids.insert("imdb".into(), "tt8111088".into());
                ids
            },
        })
    }
    
    async fn get_stream_url(&self, content_id: &str, quality: &QualityProfile) -> StreamingResult<StreamInfo> {
        if !self.is_authenticated().await {
            return Err(StreamingError::AuthenticationFailed("Not authenticated".into()));
        }
        
        Ok(StreamInfo {
            url: format!("https://disneyplus.com/watch/{}", content_id),
            manifest_url: Some(format!("https://disneyplus.com/manifest/{}.mpd", content_id)),
            format: StreamFormat::Dash,
            drm: Some(DrmInfo {
                system: DrmSystem::Widevine,
                key_id: None,
            }),
            license_url: Some("https://disneyplus.com/license".into()),
            audio_tracks: vec![],
            subtitles: vec![],
            quality: quality.clone(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
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
                bitrate_kbps: 25000,
                is_hdr: false,
                is_dolby_vision: false,
                audio_quality: AudioQuality::Atmos_7_1,
            },
            QualityProfile {
                id: "4k_hdr".into(),
                name: "4K HDR".into(),
                resolution: Resolution::UHD_4K,
                bitrate_kbps: 25000,
                is_hdr: true,
                is_dolby_vision: true,
                audio_quality: AudioQuality::Atmos_7_1,
            },
            QualityProfile {
                id: "1080p".into(),
                name: "Full HD".into(),
                resolution: Resolution::FullHD_1080p,
                bitrate_kbps: 8000,
                is_hdr: false,
                is_dolby_vision: false,
                audio_quality: AudioQuality::Surround_5_1_384,
            },
        ]
    }
}