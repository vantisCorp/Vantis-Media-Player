//! Trakt Integration
//! 
/// Synchronizes watch history with Trakt.tv.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Trakt client
pub struct TraktClient {
    /// Client ID
    client_id: String,
    
    /// Client secret
    client_secret: String,
    
    /// Access token
    access_token: Option<String>,
    
    /// HTTP client
    client: Client,
    
    /// Base URL
    base_url: String,
}

impl TraktClient {
    /// Create a new Trakt client
    pub fn new(client_id: &str, client_secret: &str) -> Result<Self> {
        Ok(Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            access_token: None,
            client: Client::new(),
            base_url: "https://api.trakt.tv".to_string(),
        })
    }
    
    /// Authenticate with Trakt
    pub async fn authenticate(&mut self, code: &str) -> Result<()> {
        let url = format!("{}/oauth/token", self.base_url);
        
        let body = serde_json::json!({
            "code": code,
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "redirect_uri": "urn:ietf:wg:oauth:2.0:oob",
            "grant_type": "authorization_code"
        });
        
        let response: AuthResponse = self.client.post(&url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        
        self.access_token = Some(response.access_token);
        
        Ok(())
    }
    
    /// Mark movie as watched
    pub async fn mark_watched(&self, movie_id: u32) -> Result<()> {
        let token = self.access_token.as_ref().ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!("{}/sync/history", self.base_url);
        
        let body = serde_json::json!({
            "movies": [{
                "ids": {
                    "trakt": movie_id
                },
                "watched_at": chrono::Utc::now().to_rfc3339()
            }]
        });
        
        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("trakt-api-version", "2")
            .header("trakt-api-key", &self.client_id)
            .json(&body)
            .send()
            .await?;
        
        Ok(())
    }
    
    /// Get watch history
    pub async fn get_watch_history(&self) -> Result<Vec<WatchedItem>> {
        let token = self.access_token.as_ref().ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        
        let url = format!("{}/sync/history", self.base_url);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("trakt-api-version", "2")
            .header("trakt-api-key", &self.client_id)
            .send()
            .await?;
        
        let items: Vec<WatchedItem> = response.json().await?;
        
        Ok(items)
    }
}

/// Authentication response
#[derive(Debug, Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
    created_at: u32,
}

/// Watched item
#[derive(Debug, Clone, Deserialize)]
pub struct WatchedItem {
    pub watched_at: String,
    pub action: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
}

/// Movie
#[derive(Debug, Clone, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: u32,
    pub ids: Ids,
}

/// TV Show
#[derive(Debug, Clone, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: u32,
    pub ids: Ids,
}

/// IDs
#[derive(Debug, Clone, Deserialize)]
pub struct Ids {
    pub trakt: u32,
    pub slug: String,
    pub imdb: Option<String>,
    pub tmdb: Option<u32>,
}