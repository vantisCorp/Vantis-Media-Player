//! Spotify integration service
//! 
//! Provides OAuth2 authentication and Spotify Web API integration
//! for accessing user's library, playlists, and playback features.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Spotify OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl Default for SpotifyOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8080/callback/spotify".to_string(),
            scopes: vec![
                "user-read-private".to_string(),
                "user-read-email".to_string(),
                "user-library-read".to_string(),
                "user-library-modify".to_string(),
                "playlist-read-private".to_string(),
                "playlist-read-collaborative".to_string(),
                "playlist-modify-public".to_string(),
                "playlist-modify-private".to_string(),
                "user-read-playback-state".to_string(),
                "user-modify-playback-state".to_string(),
                "user-read-currently-playing".to_string(),
            ],
        }
    }
}

/// Spotify API client configuration
#[derive(Debug, Clone)]
pub struct SpotifyConfig {
    pub oauth: SpotifyOAuthConfig,
    pub api_base_url: String,
    pub accounts_url: String,
    pub timeout_seconds: u64,
}

impl Default for SpotifyConfig {
    fn default() -> Self {
        Self {
            oauth: SpotifyOAuthConfig::default(),
            api_base_url: "https://api.spotify.com/v1".to_string(),
            accounts_url: "https://accounts.spotify.com".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// Spotify access credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
    pub scope: Option<String>,
}

impl SpotifyCredentials {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}

/// Spotify user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyProfile {
    pub id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub product: Option<String>,
    pub images: Vec<SpotifyImage>,
    pub followers: Option<SpotifyFollowers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyImage {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyFollowers {
    pub total: Option<u32>,
}

/// Spotify track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyTrack {
    pub id: String,
    pub name: String,
    pub artists: Vec<SpotifyArtist>,
    pub album: Option<SpotifyAlbum>,
    pub duration_ms: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub explicit: bool,
    pub preview_url: Option<String>,
    pub external_urls: Option<SpotifyExternalUrls>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyArtist {
    pub id: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
    pub images: Option<Vec<SpotifyImage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyAlbum {
    pub id: String,
    pub name: String,
    pub album_type: Option<String>,
    pub artists: Vec<SpotifyArtist>,
    pub images: Vec<SpotifyImage>,
    pub release_date: Option<String>,
    pub total_tracks: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: Option<String>,
}

/// Spotify playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyPlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: SpotifyOwner,
    pub images: Vec<SpotifyImage>,
    pub tracks: SpotifyPlaylistTracks,
    pub public: Option<bool>,
    pub collaborative: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyOwner {
    pub id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyPlaylistTracks {
    pub total: u32,
    pub href: Option<String>,
}

/// Spotify client
pub struct SpotifyClient {
    config: SpotifyConfig,
    client: Client,
    credentials: Option<SpotifyCredentials>,
}

impl SpotifyClient {
    pub fn new(config: SpotifyConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;
        
        Ok(Self {
            config,
            client,
            credentials: None,
        })
    }
    
    /// Generate OAuth authorization URL
    pub fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.oauth.scopes.join(" ");
        format!(
            "{}/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}",
            self.config.accounts_url,
            urlencoding::encode(&self.config.oauth.client_id),
            urlencoding::encode(&self.config.oauth.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state),
        )
    }
    
    /// Exchange authorization code for access token
    pub async fn authenticate(&mut self, code: &str) -> Result<SpotifyCredentials> {
        let response = self.client
            .post(format!("{}/api/token", self.config.accounts_url))
            .basic_auth(&self.config.oauth.client_id, Some(&self.config.oauth.client_secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", &self.config.oauth.redirect_uri),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = SpotifyCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: token["refresh_token"].as_str().map(|s| s.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(token["expires_in"].as_i64().unwrap_or(3600)),
            token_type: token["token_type"].as_str().unwrap_or("Bearer").to_string(),
            scope: token["scope"].as_str().map(|s| s.to_string()),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Refresh access token
    pub async fn refresh_token(&mut self, refresh_token: &str) -> Result<SpotifyCredentials> {
        let response = self.client
            .post(format!("{}/api/token", self.config.accounts_url))
            .basic_auth(&self.config.oauth.client_id, Some(&self.config.oauth.client_secret))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = SpotifyCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: Some(refresh_token.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(token["expires_in"].as_i64().unwrap_or(3600)),
            token_type: "Bearer".to_string(),
            scope: token["scope"].as_str().map(|s| s.to_string()),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Get current user's profile
    pub async fn get_profile(&self) -> Result<SpotifyProfile> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let response = self.client
            .get(format!("{}/me", self.config.api_base_url))
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let profile: SpotifyProfile = response.json().await?;
        Ok(profile)
    }
    
    /// Search for tracks
    pub async fn search_tracks(&self, query: &str, limit: u32, offset: u32) -> Result<Vec<SpotifyTrack>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/search?q={}&type=track&limit={}&offset={}",
            self.config.api_base_url,
            urlencoding::encode(query),
            limit,
            offset
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let tracks: Vec<SpotifyTrack> = serde_json::from_value(result["tracks"]["items"].clone())?;
        Ok(tracks)
    }
    
    /// Get user's playlists
    pub async fn get_playlists(&self, limit: u32, offset: u32) -> Result<Vec<SpotifyPlaylist>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/me/playlists?limit={}&offset={}",
            self.config.api_base_url,
            limit,
            offset
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let playlists: Vec<SpotifyPlaylist> = serde_json::from_value(result["items"].clone())?;
        Ok(playlists)
    }
    
    /// Get playlist tracks
    pub async fn get_playlist_tracks(&self, playlist_id: &str, limit: u32, offset: u32) -> Result<Vec<SpotifyTrack>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/playlists/{}/tracks?limit={}&offset={}",
            self.config.api_base_url,
            playlist_id,
            limit,
            offset
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = serde_json::from_value(result["items"].clone())?;
        
        let tracks: Vec<SpotifyTrack> = items
            .into_iter()
            .filter_map(|item| serde_json::from_value(item["track"].clone()).ok())
            .collect();
        
        Ok(tracks)
    }
    
    /// Get user's saved tracks
    pub async fn get_saved_tracks(&self, limit: u32, offset: u32) -> Result<Vec<SpotifyTrack>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/me/tracks?limit={}&offset={}",
            self.config.api_base_url,
            limit,
            offset
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = serde_json::from_value(result["items"].clone())?;
        
        let tracks: Vec<SpotifyTrack> = items
            .into_iter()
            .filter_map(|item| serde_json::from_value(item["track"].clone()).ok())
            .collect();
        
        Ok(tracks)
    }
    
    /// Get available devices
    pub async fn get_devices(&self) -> Result<Vec<SpotifyDevice>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let response = self.client
            .get(format!("{}/me/player/devices", self.config.api_base_url))
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let devices: Vec<SpotifyDevice> = serde_json::from_value(result["devices"].clone())?;
        Ok(devices)
    }
    
    /// Get credentials
    pub fn get_credentials(&self) -> Option<&SpotifyCredentials> {
        self.credentials.as_ref()
    }
    
    /// Set credentials
    pub fn set_credentials(&mut self, credentials: SpotifyCredentials) {
        self.credentials = Some(credentials);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyDevice {
    pub id: String,
    pub is_active: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub volume_percent: Option<u32>,
}