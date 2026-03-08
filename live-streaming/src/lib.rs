//! Live Streaming Module for Vantis Media Player
//!
//! This module provides comprehensive live streaming capabilities similar to
//! Twitch and YouTube Live, including:
//! - Stream creation and management
//! - Real-time viewer engagement
//! - Live chat with moderation
//! - Interactive features (polls, predictions, hype trains)
//! - Stream discovery and recommendations
//! - Analytics and insights

pub mod types;
pub mod error;
pub mod broadcaster;
pub mod viewer;
pub mod chat;

// Re-export key types for convenience
pub use types::*;
pub use error::{LiveStreamingError, LiveStreamingResult};

use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::{broadcast, RwLock};
use broadcaster::{BroadcasterService, DefaultBroadcasterService};
use viewer::{ViewerService, DiscoveryService, DefaultViewerService, DefaultDiscoveryService};
use chat::{ChatService, DefaultChatService};

/// Configuration for the live streaming module
#[derive(Debug, Clone)]
pub struct LiveStreamingConfig {
    /// Maximum concurrent streams per broadcaster
    pub max_streams_per_broadcaster: usize,
    /// Maximum viewers per stream
    pub max_viewers_per_stream: usize,
    /// Chat message rate limit (messages per minute)
    pub chat_rate_limit: u32,
    /// Maximum chat message length
    pub max_chat_message_length: usize,
    /// Enable hype train feature
    pub enable_hype_train: bool,
    /// Enable predictions feature
    pub enable_predictions: bool,
    /// Enable polls feature
    pub enable_polls: bool,
    /// Default stream latency mode
    pub default_latency_mode: LatencyMode,
    /// VOD retention period in days
    pub vod_retention_days: u32,
}

impl Default for LiveStreamingConfig {
    fn default() -> Self {
        Self {
            max_streams_per_broadcaster: 1,
            max_viewers_per_stream: 100_000,
            chat_rate_limit: 100,
            max_chat_message_length: 500,
            enable_hype_train: true,
            enable_predictions: true,
            enable_polls: true,
            default_latency_mode: LatencyMode::LowLatency,
            vod_retention_days: 60,
        }
    }
}

/// Statistics for the live streaming service
#[derive(Debug, Clone, Default)]
pub struct StreamingStats {
    /// Total active streams
    pub active_streams: u64,
    /// Total active viewers across all streams
    pub total_viewers: u64,
    /// Total broadcasters
    pub total_broadcasters: u64,
    /// Total chat messages sent today
    pub chat_messages_today: u64,
    /// Peak concurrent viewers today
    pub peak_viewers_today: u64,
}

/// Container for all live streaming services
pub struct LiveStreamingServices {
    /// Broadcaster service for stream management
    pub broadcaster: Arc<dyn BroadcasterService>,
    /// Viewer service for viewer operations
    pub viewer: Arc<dyn ViewerService>,
    /// Discovery service for finding streams
    pub discovery: Arc<dyn DiscoveryService>,
    /// Chat service for live chat
    pub chat: Arc<dyn ChatService>,
    /// Configuration
    config: Arc<RwLock<LiveStreamingConfig>>,
    /// Streaming statistics
    stats: Arc<RwLock<StreamingStats>>,
    /// Event broadcaster for real-time notifications
    event_sender: broadcast::Sender<StreamEvent>,
}

impl LiveStreamingServices {
    /// Create a new instance of live streaming services
    pub fn new(config: LiveStreamingConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1024);
        let config = Arc::new(RwLock::new(config));
        let stats = Arc::new(RwLock::new(StreamingStats::default()));
        
        // Create service instances
        let broadcaster = Arc::new(DefaultBroadcasterService::new(
            config.clone(),
            event_sender.clone(),
        ));
        
        let viewer = Arc::new(DefaultViewerService::new(
            config.clone(),
            event_sender.clone(),
        ));
        
        let discovery = Arc::new(DefaultDiscoveryService::new(
            config.clone(),
        ));
        
        let chat = Arc::new(DefaultChatService::new(
            config.clone(),
            event_sender.clone(),
        ));
        
        Self {
            broadcaster,
            viewer,
            discovery,
            chat,
            config,
            stats,
            event_sender,
        }
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(LiveStreamingConfig::default())
    }
    
    /// Get the current configuration
    pub async fn config(&self) -> LiveStreamingConfig {
        self.config.read().await.clone()
    }
    
    /// Update the configuration
    pub async fn update_config<F>(&self, f: F) 
    where
        F: FnOnce(&mut LiveStreamingConfig),
    {
        let mut config = self.config.write().await;
        f(&mut config);
    }
    
    /// Get current streaming statistics
    pub async fn stats(&self) -> StreamingStats {
        self.stats.read().await.clone()
    }
    
    /// Subscribe to stream events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<StreamEvent> {
        self.event_sender.subscribe()
    }
    
    /// Update streaming statistics
    pub async fn update_stats<F>(&self, f: F)
    where
        F: FnOnce(&mut StreamingStats),
    {
        let mut stats = self.stats.write().await;
        f(&mut stats);
    }
    
    /// Get health status of the live streaming services
    pub async fn health_check(&self) -> LiveStreamingResult<HealthStatus> {
        let stats = self.stats.read().await;
        let config = self.config.read().await;
        
        Ok(HealthStatus {
            healthy: true,
            active_streams: stats.active_streams,
            total_viewers: stats.total_viewers,
            features_enabled: FeaturesEnabled {
                hype_train: config.enable_hype_train,
                predictions: config.enable_predictions,
                polls: config.enable_polls,
            },
        })
    }
}

/// Health status information
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthStatus {
    /// Whether the service is healthy
    pub healthy: bool,
    /// Number of active streams
    pub active_streams: u64,
    /// Total viewers across all streams
    pub total_viewers: u64,
    /// Enabled features
    pub features_enabled: FeaturesEnabled,
}

/// Feature flags for the service
#[derive(Debug, Clone, serde::Serialize)]
pub struct FeaturesEnabled {
    pub hype_train: bool,
    pub predictions: bool,
    pub polls: bool,
}

/// Builder for creating custom live streaming service configurations
pub struct LiveStreamingServicesBuilder {
    config: LiveStreamingConfig,
}

impl LiveStreamingServicesBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: LiveStreamingConfig::default(),
        }
    }
    
    /// Set maximum streams per broadcaster
    pub fn max_streams_per_broadcaster(mut self, max: usize) -> Self {
        self.config.max_streams_per_broadcaster = max;
        self
    }
    
    /// Set maximum viewers per stream
    pub fn max_viewers_per_stream(mut self, max: usize) -> Self {
        self.config.max_viewers_per_stream = max;
        self
    }
    
    /// Set chat rate limit
    pub fn chat_rate_limit(mut self, limit: u32) -> Self {
        self.config.chat_rate_limit = limit;
        self
    }
    
    /// Enable or disable hype train
    pub fn enable_hype_train(mut self, enable: bool) -> Self {
        self.config.enable_hype_train = enable;
        self
    }
    
    /// Enable or disable predictions
    pub fn enable_predictions(mut self, enable: bool) -> Self {
        self.config.enable_predictions = enable;
        self
    }
    
    /// Enable or disable polls
    pub fn enable_polls(mut self, enable: bool) -> Self {
        self.config.enable_polls = enable;
        self
    }
    
    /// Set default latency mode
    pub fn default_latency_mode(mut self, mode: LatencyMode) -> Self {
        self.config.default_latency_mode = mode;
        self
    }
    
    /// Build the live streaming services
    pub fn build(self) -> LiveStreamingServices {
        LiveStreamingServices::new(self.config)
    }
}

impl Default for LiveStreamingServicesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = LiveStreamingConfig::default();
        assert_eq!(config.max_streams_per_broadcaster, 1);
        assert_eq!(config.max_viewers_per_stream, 100_000);
        assert!(config.enable_hype_train);
        assert!(config.enable_predictions);
        assert!(config.enable_polls);
    }
    
    #[test]
    fn test_builder() {
        let services = LiveStreamingServicesBuilder::new()
            .max_streams_per_broadcaster(3)
            .max_viewers_per_stream(50_000)
            .enable_hype_train(false)
            .build();
        
        assert!(services.broadcaster.as_ref() as *const _ != std::ptr::null());
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let services = LiveStreamingServices::with_defaults();
        let health = services.health_check().await.unwrap();
        
        assert!(health.healthy);
        assert_eq!(health.active_streams, 0);
    }
}