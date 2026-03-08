//! Vantis Integrations
//! 
//! External service integrations for Vantis Media Player:
//! - **TMDB**: The Movie Database for movie/TV metadata
//! - **Filmweb**: Polish movie ratings and reviews
//! - **Trakt.tv**: Watch history and tracking
//! - **IPC Guard**: Antivirus integration
//! - **Spotify**: Music streaming and playlists
//! - **YouTube**: Video streaming and subscriptions
//! - **Cloud Storage**: Google Drive and Dropbox
//! - **Podcasts**: RSS feed parsing and iTunes search

use anyhow::Result;
use tracing::info;

pub mod tmdb;
pub mod filmweb;
pub mod trakt;
pub mod ipc;
pub mod spotify;
pub mod youtube;
pub mod cloud_storage;
pub mod podcast;

// Re-export main types
pub use spotify::{SpotifyClient, SpotifyConfig, SpotifyProfile, SpotifyTrack, SpotifyPlaylist, SpotifyCredentials};
pub use youtube::{YouTubeClient, YouTubeConfig, YouTubeVideo, YouTubePlaylist, YouTubeChannel, YouTubeCredentials};
pub use cloud_storage::{
    GoogleDriveClient, GoogleDriveConfig, GoogleDriveFile, GoogleDriveQuota,
    DropboxClient, DropboxConfig, DropboxFile, DropboxQuota,
};
pub use podcast::{PodcastClient, PodcastConfig, PodcastFeed, PodcastEpisode, PodcastCategory};

/// Integration provider enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrationProvider {
    /// The Movie Database
    TMDB,
    /// Filmweb (Polish ratings)
    Filmweb,
    /// Trakt.tv
    Trakt,
    /// Spotify
    Spotify,
    /// YouTube
    YouTube,
    /// Google Drive
    GoogleDrive,
    /// Dropbox
    Dropbox,
    /// iTunes/Podcasts
    ITunes,
}

/// Connection status for an integration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Not connected
    Disconnected,
    /// Successfully connected
    Connected,
    /// Connection expired
    Expired,
    /// Connection failed
    Failed,
}

/// Integration manager
pub struct IntegrationManager {
    /// TMDB client
    pub tmdb: Option<tmdb::TMDBClient>,
    
    /// Filmweb client
    pub filmweb: Option<filmweb::FilmwebClient>,
    
    /// Trakt client
    pub trakt: Option<trakt::TraktClient>,
    
    /// IPC Guard
    pub ipc_guard: Option<ipc::IPCGuard>,
    
    /// Spotify client
    pub spotify: Option<SpotifyClient>,
    
    /// YouTube client
    pub youtube: Option<YouTubeClient>,
    
    /// Google Drive client
    pub google_drive: Option<GoogleDriveClient>,
    
    /// Dropbox client
    pub dropbox: Option<DropboxClient>,
    
    /// Podcast client
    pub podcast: Option<PodcastClient>,
}

impl IntegrationManager {
    /// Create a new integration manager
    pub fn new() -> Result<Self> {
        info!("🌐 Initializing integrations...");
        
        Ok(Self {
            tmdb: None,
            filmweb: None,
            trakt: None,
            ipc_guard: None,
            spotify: None,
            youtube: None,
            google_drive: None,
            dropbox: None,
            podcast: None,
        })
    }
    
    /// Initialize TMDB client
    pub async fn init_tmdb(&mut self, api_key: &str) -> Result<()> {
        info!("🎬 Initializing TMDB integration");
        self.tmdb = Some(tmdb::TMDBClient::new(api_key)?);
        Ok(())
    }
    
    /// Initialize Filmweb client
    pub async fn init_filmweb(&mut self) -> Result<()> {
        info!("🎥 Initializing Filmweb integration");
        self.filmweb = Some(filmweb::FilmwebClient::new()?);
        Ok(())
    }
    
    /// Initialize Trakt client
    pub async fn init_trakt(&mut self, client_id: &str, client_secret: Option<&str>) -> Result<()> {
        info!("📺 Initializing Trakt.tv integration");
        self.trakt = Some(trakt::TraktClient::new(client_id, client_secret)?);
        Ok(())
    }
    
    /// Initialize IPC Guard
    pub async fn init_ipc_guard(&mut self) -> Result<()> {
        info!("🛡️ Initializing IPC Guard");
        self.ipc_guard = Some(ipc::IPCGuard::new()?);
        Ok(())
    }
    
    /// Initialize Spotify client
    pub async fn init_spotify(&mut self, config: SpotifyConfig) -> Result<()> {
        info!("🎧 Initializing Spotify integration");
        self.spotify = Some(SpotifyClient::new(config)?);
        Ok(())
    }
    
    /// Initialize YouTube client
    pub async fn init_youtube(&mut self, config: YouTubeConfig) -> Result<()> {
        info!("▶️ Initializing YouTube integration");
        self.youtube = Some(YouTubeClient::new(config)?);
        Ok(())
    }
    
    /// Initialize Google Drive client
    pub async fn init_google_drive(&mut self, config: GoogleDriveConfig) -> Result<()> {
        info!("☁️ Initializing Google Drive integration");
        self.google_drive = Some(GoogleDriveClient::new(config)?);
        Ok(())
    }
    
    /// Initialize Dropbox client
    pub async fn init_dropbox(&mut self, config: DropboxConfig) -> Result<()> {
        info!("📦 Initializing Dropbox integration");
        self.dropbox = Some(DropboxClient::new(config)?);
        Ok(())
    }
    
    /// Initialize Podcast client
    pub async fn init_podcast(&mut self, config: PodcastConfig) -> Result<()> {
        info!("🎙️ Initializing Podcast integration");
        self.podcast = Some(PodcastClient::new(config)?);
        Ok(())
    }
    
    /// Check if a provider is connected
    pub fn is_connected(&self, provider: IntegrationProvider) -> bool {
        match provider {
            IntegrationProvider::TMDB => self.tmdb.is_some(),
            IntegrationProvider::Filmweb => self.filmweb.is_some(),
            IntegrationProvider::Trakt => self.trakt.is_some(),
            IntegrationProvider::Spotify => self.spotify.as_ref()
                .map(|c| c.get_credentials().is_some())
                .unwrap_or(false),
            IntegrationProvider::YouTube => self.youtube.as_ref()
                .map(|c| c.get_credentials().is_some())
                .unwrap_or(false),
            IntegrationProvider::GoogleDrive => self.google_drive.as_ref()
                .map(|c| c.get_credentials().is_some())
                .unwrap_or(false),
            IntegrationProvider::Dropbox => self.dropbox.as_ref()
                .map(|c| c.get_credentials().is_some())
                .unwrap_or(false),
            IntegrationProvider::ITunes => self.podcast.is_some(),
        }
    }
    
    /// Get connection status for a provider
    pub fn get_status(&self, provider: IntegrationProvider) -> ConnectionStatus {
        match provider {
            IntegrationProvider::TMDB => if self.tmdb.is_some() { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected },
            IntegrationProvider::Filmweb => if self.filmweb.is_some() { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected },
            IntegrationProvider::Trakt => if self.trakt.is_some() { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected },
            IntegrationProvider::Spotify => {
                if let Some(client) = &self.spotify {
                    if let Some(creds) = client.get_credentials() {
                        if creds.is_expired() { ConnectionStatus::Expired } else { ConnectionStatus::Connected }
                    } else {
                        ConnectionStatus::Disconnected
                    }
                } else {
                    ConnectionStatus::Disconnected
                }
            }
            IntegrationProvider::YouTube => {
                if let Some(client) = &self.youtube {
                    if let Some(creds) = client.get_credentials() {
                        if creds.is_expired() { ConnectionStatus::Expired } else { ConnectionStatus::Connected }
                    } else {
                        ConnectionStatus::Disconnected
                    }
                } else {
                    ConnectionStatus::Disconnected
                }
            }
            IntegrationProvider::GoogleDrive => {
                if let Some(client) = &self.google_drive {
                    if let Some(creds) = client.get_credentials() {
                        if creds.is_expired() { ConnectionStatus::Expired } else { ConnectionStatus::Connected }
                    } else {
                        ConnectionStatus::Disconnected
                    }
                } else {
                    ConnectionStatus::Disconnected
                }
            }
            IntegrationProvider::Dropbox => {
                if let Some(client) = &self.dropbox {
                    if client.get_credentials().is_some() { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected }
                } else {
                    ConnectionStatus::Disconnected
                }
            }
            IntegrationProvider::ITunes => if self.podcast.is_some() { ConnectionStatus::Connected } else { ConnectionStatus::Disconnected },
        }
    }
}

impl Default for IntegrationManager {
    fn default() -> Self {
        Self::new().expect("Failed to create IntegrationManager")
    }
}

/// Builder for IntegrationManager
pub struct IntegrationManagerBuilder {
    tmdb_api_key: Option<String>,
    trakt_client_id: Option<String>,
    trakt_client_secret: Option<String>,
    spotify_config: Option<SpotifyConfig>,
    youtube_config: Option<YouTubeConfig>,
    google_drive_config: Option<GoogleDriveConfig>,
    dropbox_config: Option<DropboxConfig>,
    podcast_config: Option<PodcastConfig>,
    init_filmweb: bool,
    init_ipc: bool,
}

impl IntegrationManagerBuilder {
    pub fn new() -> Self {
        Self {
            tmdb_api_key: None,
            trakt_client_id: None,
            trakt_client_secret: None,
            spotify_config: None,
            youtube_config: None,
            google_drive_config: None,
            dropbox_config: None,
            podcast_config: None,
            init_filmweb: false,
            init_ipc: false,
        }
    }
    
    pub fn with_tmdb(mut self, api_key: String) -> Self {
        self.tmdb_api_key = Some(api_key);
        self
    }
    
    pub fn with_trakt(mut self, client_id: String, client_secret: Option<String>) -> Self {
        self.trakt_client_id = Some(client_id);
        self.trakt_client_secret = client_secret;
        self
    }
    
    pub fn with_spotify(mut self, config: SpotifyConfig) -> Self {
        self.spotify_config = Some(config);
        self
    }
    
    pub fn with_youtube(mut self, config: YouTubeConfig) -> Self {
        self.youtube_config = Some(config);
        self
    }
    
    pub fn with_google_drive(mut self, config: GoogleDriveConfig) -> Self {
        self.google_drive_config = Some(config);
        self
    }
    
    pub fn with_dropbox(mut self, config: DropboxConfig) -> Self {
        self.dropbox_config = Some(config);
        self
    }
    
    pub fn with_podcast(mut self, config: PodcastConfig) -> Self {
        self.podcast_config = Some(config);
        self
    }
    
    pub fn with_filmweb(mut self) -> Self {
        self.init_filmweb = true;
        self
    }
    
    pub fn with_ipc(mut self) -> Self {
        self.init_ipc = true;
        self
    }
    
    pub async fn build(self) -> Result<IntegrationManager> {
        let mut manager = IntegrationManager::new()?;
        
        if let Some(api_key) = self.tmdb_api_key {
            manager.init_tmdb(&api_key).await?;
        }
        
        if self.init_filmweb {
            manager.init_filmweb().await?;
        }
        
        if let Some(client_id) = self.trakt_client_id {
            manager.init_trakt(&client_id, self.trakt_client_secret.as_deref()).await?;
        }
        
        if self.init_ipc {
            manager.init_ipc_guard().await?;
        }
        
        if let Some(config) = self.spotify_config {
            manager.init_spotify(config).await?;
        }
        
        if let Some(config) = self.youtube_config {
            manager.init_youtube(config).await?;
        }
        
        if let Some(config) = self.google_drive_config {
            manager.init_google_drive(config).await?;
        }
        
        if let Some(config) = self.dropbox_config {
            manager.init_dropbox(config).await?;
        }
        
        if let Some(config) = self.podcast_config {
            manager.init_podcast(config).await?;
        }
        
        Ok(manager)
    }
}

impl Default for IntegrationManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_manager_creation() {
        let manager = IntegrationManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_connection_status() {
        let manager = IntegrationManager::new().unwrap();
        assert_eq!(manager.get_status(IntegrationProvider::Spotify), ConnectionStatus::Disconnected);
    }
}