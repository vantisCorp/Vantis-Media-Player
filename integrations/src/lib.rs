//! Vantis Integrations
//! 
/// External service integrations:
/// - TMDB (The Movie Database)
/// - Filmweb (Polish ratings)
/// - Trakt.tv (Watch history)
/// - IPC Guard (Antivirus integration)

use anyhow::Result;
use tracing::info;

pub mod tmdb;
pub mod filmweb;
pub mod trakt;
pub mod ipc;

/// Integration manager
pub struct IntegrationManager {
    /// TMDB client
    tmdb: Option<tmdb::TMDBClient>,
    
    /// Filmweb client
    filmweb: Option<filmweb::FilmwebClient>,
    
    /// Trakt client
    trakt: Option<trakt::TraktClient>,
    
    /// IPC Guard
    ipc_guard: Option<ipc::IPCGuard>,
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
        info!("🎭 Initializing Filmweb integration");
        self.filmweb = Some(filmweb::FilmwebClient::new()?);
        Ok(())
    }
    
    /// Initialize Trakt client
    pub async fn init_trakt(&mut self, client_id: &str, client_secret: &str) -> Result<()> {
        info!("📊 Initializing Trakt integration");
        self.trakt = Some(trakt::TraktClient::new(client_id, client_secret)?);
        Ok(())
    }
    
    /// Initialize IPC Guard
    pub async fn init_ipc_guard(&mut self) -> Result<()> {
        info!("🛡️ Initializing IPC Guard");
        self.ipc_guard = Some(ipc::IPCGuard::new()?);
        Ok(())
    }
}

impl Default for IntegrationManager {
    fn default() -> Self {
        Self::new().expect("Failed to create integration manager")
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
    fn test_integration_manager_default() {
        let manager = IntegrationManager::default();
        assert!(manager.tmdb.is_none());
        assert!(manager.filmweb.is_none());
        assert!(manager.trakt.is_none());
        assert!(manager.ipc_guard.is_none());
    }
}