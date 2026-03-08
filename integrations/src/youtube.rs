//! YouTube integration service
//! 
//! Provides OAuth2 authentication and YouTube Data API v3 integration
//! for accessing playlists, liked videos, and channel subscriptions.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// YouTube OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl Default for YouTubeOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8080/callback/youtube".to_string(),
            scopes: vec![
                "https://www.googleapis.com/auth/youtube.readonly".to_string(),
                "https://www.googleapis.com/auth/youtubepartner".to_string(),
            ],
        }
    }
}

/// YouTube API client configuration
#[derive(Debug, Clone)]
pub struct YouTubeConfig {
    pub oauth: YouTubeOAuthConfig,
    pub api_key: Option<String>,
    pub api_base_url: String,
    pub timeout_seconds: u64,
}

impl Default for YouTubeConfig {
    fn default() -> Self {
        Self {
            oauth: YouTubeOAuthConfig::default(),
            api_key: None,
            api_base_url: "https://www.googleapis.com/youtube/v3".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// YouTube access credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
    pub scope: Option<String>,
}

impl YouTubeCredentials {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}

/// YouTube video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeVideo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub channel_title: Option<String>,
    pub channel_id: Option<String>,
    pub published_at: Option<String>,
    pub duration: Option<String>,
    pub duration_seconds: Option<u32>,
    pub view_count: Option<u64>,
    pub like_count: Option<u64>,
}

/// YouTube playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubePlaylist {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub channel_title: Option<String>,
    pub item_count: Option<u32>,
    pub privacy_status: Option<String>,
}

/// YouTube channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeChannel {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub subscriber_count: Option<u64>,
    pub video_count: Option<u64>,
}

/// Parse ISO 8601 duration format (PT1H2M3S)
fn parse_youtube_duration(duration: &str) -> Option<u32> {
    let mut total_seconds = 0u32;
    let mut current_number = String::new();
    
    for c in duration.chars() {
        match c {
            '0'..='9' => current_number.push(c),
            'H' => {
                if let Ok(hours) = current_number.parse::<u32>() {
                    total_seconds += hours * 3600;
                }
                current_number.clear();
            }
            'M' => {
                if let Ok(minutes) = current_number.parse::<u32>() {
                    total_seconds += minutes * 60;
                }
                current_number.clear();
            }
            'S' => {
                if let Ok(seconds) = current_number.parse::<u32>() {
                    total_seconds += seconds;
                }
                current_number.clear();
            }
            _ => {}
        }
    }
    
    if total_seconds > 0 { Some(total_seconds) } else { None }
}

/// YouTube client
pub struct YouTubeClient {
    config: YouTubeConfig,
    client: Client,
    credentials: Option<YouTubeCredentials>,
}

impl YouTubeClient {
    pub fn new(config: YouTubeConfig) -> Result<Self> {
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
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&access_type=offline&prompt=consent",
            urlencoding::encode(&self.config.oauth.client_id),
            urlencoding::encode(&self.config.oauth.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state),
        )
    }
    
    /// Exchange authorization code for access token
    pub async fn authenticate(&mut self, code: &str) -> Result<YouTubeCredentials> {
        let response = self.client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.config.oauth.client_id.as_str()),
                ("client_secret", self.config.oauth.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", self.config.oauth.redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = YouTubeCredentials {
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
    pub async fn refresh_token(&mut self, refresh_token: &str) -> Result<YouTubeCredentials> {
        let response = self.client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.config.oauth.client_id.as_str()),
                ("client_secret", self.config.oauth.client_secret.as_str()),
                ("refresh_token", refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?;
        
        let token: serde_json::Value = response.json().await?;
        
        let credentials = YouTubeCredentials {
            access_token: token["access_token"].as_str().unwrap_or_default().to_string(),
            refresh_token: Some(refresh_token.to_string()),
            expires_at: Utc::now() + chrono::Duration::seconds(token["expires_in"].as_i64().unwrap_or(3600)),
            token_type: "Bearer".to_string(),
            scope: token["scope"].as_str().map(|s| s.to_string()),
        };
        
        self.credentials = Some(credentials.clone());
        Ok(credentials)
    }
    
    /// Search for videos
    pub async fn search_videos(&self, query: &str, max_results: u32) -> Result<Vec<YouTubeVideo>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/search?part=snippet&q={}&type=video&maxResults={}",
            self.config.api_base_url,
            urlencoding::encode(query),
            max_results
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["items"].as_array()
            .cloned().unwrap_or_default();
        
        let videos: Vec<YouTubeVideo> = items.into_iter()
            .filter_map(|item| {
                let id = item["id"]["videoId"].as_str()?.to_string();
                let snippet = &item["snippet"];
                
                Some(YouTubeVideo {
                    id,
                    title: snippet["title"].as_str()?.to_string(),
                    description: snippet["description"].as_str().map(|s| s.to_string()),
                    thumbnail_url: snippet["thumbnails"]["high"]["url"].as_str().map(|s| s.to_string()),
                    channel_title: snippet["channelTitle"].as_str().map(|s| s.to_string()),
                    channel_id: snippet["channelId"].as_str().map(|s| s.to_string()),
                    published_at: snippet["publishedAt"].as_str().map(|s| s.to_string()),
                    duration: None,
                    duration_seconds: None,
                    view_count: None,
                    like_count: None,
                })
            })
            .collect();
        
        Ok(videos)
    }
    
    /// Get user's playlists
    pub async fn get_playlists(&self, max_results: u32) -> Result<Vec<YouTubePlaylist>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/playlists?part=snippet,contentDetails,status&mine=true&maxResults={}",
            self.config.api_base_url,
            max_results
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["items"].as_array()
            .cloned().unwrap_or_default();
        
        let playlists: Vec<YouTubePlaylist> = items.into_iter()
            .filter_map(|item| {
                let snippet = &item["snippet"];
                
                Some(YouTubePlaylist {
                    id: item["id"].as_str()?.to_string(),
                    title: snippet["title"].as_str()?.to_string(),
                    description: snippet["description"].as_str().map(|s| s.to_string()),
                    thumbnail_url: snippet["thumbnails"]["high"]["url"].as_str().map(|s| s.to_string()),
                    channel_title: snippet["channelTitle"].as_str().map(|s| s.to_string()),
                    item_count: item["contentDetails"]["itemCount"].as_u64().map(|n| n as u32),
                    privacy_status: item["status"]["privacyStatus"].as_str().map(|s| s.to_string()),
                })
            })
            .collect();
        
        Ok(playlists)
    }
    
    /// Get playlist items
    pub async fn get_playlist_items(&self, playlist_id: &str, max_results: u32) -> Result<Vec<YouTubeVideo>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/playlistItems?part=snippet,contentDetails&playlistId={}&maxResults={}",
            self.config.api_base_url,
            playlist_id,
            max_results
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["items"].as_array()
            .cloned().unwrap_or_default();
        
        let videos: Vec<YouTubeVideo> = items.into_iter()
            .filter_map(|item| {
                let content = &item["contentDetails"];
                let snippet = &item["snippet"];
                
                Some(YouTubeVideo {
                    id: content["videoId"].as_str()?.to_string(),
                    title: snippet["title"].as_str()?.to_string(),
                    description: snippet["description"].as_str().map(|s| s.to_string()),
                    thumbnail_url: snippet["thumbnails"]["high"]["url"].as_str().map(|s| s.to_string()),
                    channel_title: snippet["videoOwnerChannelTitle"].as_str().map(|s| s.to_string()),
                    channel_id: snippet["videoOwnerChannelId"].as_str().map(|s| s.to_string()),
                    published_at: content["videoPublishedAt"].as_str().map(|s| s.to_string()),
                    duration: None,
                    duration_seconds: None,
                    view_count: None,
                    like_count: None,
                })
            })
            .collect();
        
        Ok(videos)
    }
    
    /// Get subscriptions
    pub async fn get_subscriptions(&self, max_results: u32) -> Result<Vec<YouTubeChannel>> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!(
            "{}/subscriptions?part=snippet&mine=true&maxResults={}",
            self.config.api_base_url,
            max_results
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&token.access_token)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        let items: Vec<serde_json::Value> = result["items"].as_array()
            .cloned().unwrap_or_default();
        
        let channels: Vec<YouTubeChannel> = items.into_iter()
            .filter_map(|item| {
                let snippet = &item["snippet"];
                
                Some(YouTubeChannel {
                    id: snippet["resourceId"]["channelId"].as_str()?.to_string(),
                    title: snippet["title"].as_str()?.to_string(),
                    description: snippet["description"].as_str().map(|s| s.to_string()),
                    thumbnail_url: snippet["thumbnails"]["high"]["url"].as_str().map(|s| s.to_string()),
                    subscriber_count: None,
                    video_count: None,
                })
            })
            .collect();
        
        Ok(channels)
    }
    
    /// Revoke access
    pub async fn revoke_access(&self) -> Result<()> {
        let token = self.credentials.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        self.client
            .post("https://oauth2.googleapis.com/revoke")
            .form(&[("token", token.access_token.as_str())])
            .send()
            .await?;
        
        Ok(())
    }
    
    /// Get credentials
    pub fn get_credentials(&self) -> Option<&YouTubeCredentials> {
        self.credentials.as_ref()
    }
    
    /// Set credentials
    pub fn set_credentials(&mut self, credentials: YouTubeCredentials) {
        self.credentials = Some(credentials);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_youtube_duration() {
        assert_eq!(parse_youtube_duration("PT1H2M3S"), Some(3723));
        assert_eq!(parse_youtube_duration("PT5M30S"), Some(330));
        assert_eq!(parse_youtube_duration("PT45S"), Some(45));
        assert_eq!(parse_youtube_duration("PT1H"), Some(3600));
        assert_eq!(parse_youtube_duration("invalid"), None);
    }
}