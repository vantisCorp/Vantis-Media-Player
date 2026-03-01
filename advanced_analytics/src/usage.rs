//! Usage Analytics
//! 
//! Provides usage tracking and analytics capabilities including:
//! - Event tracking
//! - User session management
//! - Usage statistics
//! - User behavior analysis

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::AnalyticsConfig;

/// Usage analytics
#[derive(Clone)]
pub struct UsageAnalytics {
    /// Configuration
    config: AnalyticsConfig,
    /// Active sessions
    sessions: Arc<RwLock<HashMap<String, UserSession>>>,
    /// Event history
    events: Arc<RwLock<Vec<UsageEvent>>>,
    /// Statistics
    stats: Arc<RwLock<UsageStats>>,
}

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// Session ID
    pub session_id: String,
    /// User ID
    pub user_id: String,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Events tracked
    pub events: Vec<String>,
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

/// Usage event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEvent {
    /// Event ID
    pub event_id: String,
    /// Event name
    pub event_name: String,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Event category
    pub category: EventCategory,
}

/// Event category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCategory {
    /// User interaction
    Interaction,
    /// Media playback
    Playback,
    /// System event
    System,
    /// Error event
    Error,
    /// Custom event
    Custom,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// Total sessions
    pub total_sessions: usize,
    /// Active sessions
    pub active_sessions: usize,
    /// Total events
    pub total_events: usize,
    /// Unique users
    pub unique_users: usize,
    /// Average session duration in seconds
    pub avg_session_duration: f64,
    /// Events by category
    pub events_by_category: HashMap<EventCategory, usize>,
}

impl UsageAnalytics {
    /// Create a new usage analytics instance
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing Usage Analytics");

        Ok(Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(UsageStats {
                total_sessions: 0,
                active_sessions: 0,
                total_events: 0,
                unique_users: 0,
                avg_session_duration: 0.0,
                events_by_category: HashMap::new(),
            })),
        })
    }

    /// Track a usage event
    pub async fn track_event(&self, event: UsageEvent) -> Result<()> {
        debug!("Tracking event: {}", event.event_name);

        // Apply sampling
        if rand::random::<f64>() > self.config.sample_rate {
            return Ok(());
        }

        // Add to event history
        let mut events = self.events.write().await;
        events.push(event.clone());

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_events += 1;
        *stats.events_by_category.entry(event.category).or_insert(0) += 1;

        // Add to session if session ID is provided
        if let Some(session_id) = &event.session_id {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.events.push(event.event_id.clone());
            }
        }

        info!("Event tracked: {}", event.event_name);
        Ok(())
    }

    /// Start a user session
    pub async fn start_session(&self, user_id: &str) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let session = UserSession {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            start_time: Utc::now(),
            end_time: None,
            events: Vec::new(),
            metadata: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_sessions += 1;
        stats.active_sessions += 1;

        info!("Session started: {} for user: {}", session_id, user_id);
        Ok(session_id)
    }

    /// End a user session
    pub async fn end_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.end_time = Some(Utc::now());
            
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.active_sessions -= 1;
            
            // Calculate average session duration
            let duration = (session.end_time.unwrap() - session.start_time).num_seconds() as f64;
            let total_duration = stats.avg_session_duration * (stats.total_sessions - 1) as f64;
            stats.avg_session_duration = (total_duration + duration) / stats.total_sessions as f64;
            
            info!("Session ended: {}", session_id);
        }

        Ok(())
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<UserSession> {
        self.sessions.read().await.get(session_id).cloned()
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<UserSession> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|s| s.end_time.is_none())
            .cloned()
            .collect()
    }

    /// Get usage statistics
    pub async fn get_stats(&self) -> UsageStats {
        self.stats.read().await.clone()
    }

    /// Get events by category
    pub async fn get_events_by_category(&self, category: EventCategory) -> Vec<UsageEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.category == category)
            .cloned()
            .collect()
    }

    /// Get events for user
    pub async fn get_user_events(&self, user_id: &str) -> Vec<UsageEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.user_id.as_deref() == Some(user_id))
            .cloned()
            .collect()
    }

    /// Flush events to analytics endpoint
    pub async fn flush(&self) -> Result<()> {
        if let Some(endpoint) = &self.config.analytics_endpoint {
            let events = self.events.read().await.clone();
            
            if events.is_empty() {
                return Ok(());
            }

            debug!("Flushing {} events to {}", events.len(), endpoint);

            // In a real implementation, this would send events to the analytics endpoint
            // For now, we'll just clear the events
            
            self.events.write().await.clear();
            
            info!("Flushed {} events", events.len());
        }

        Ok(())
    }

    /// Clear old events
    pub async fn clear_old_events(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = Utc::now() - older_than;
        
        let mut events = self.events.write().await;
        let initial_count = events.len();
        events.retain(|e| e.timestamp > cutoff);
        let removed = initial_count - events.len();

        info!("Cleared {} old events", removed);
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_usage_analytics_creation() {
        let config = AnalyticsConfig::default();
        let analytics = UsageAnalytics::new(config).await;
        assert!(analytics.is_ok());
    }

    #[tokio::test]
    async fn test_start_session() {
        let analytics = UsageAnalytics::new(AnalyticsConfig::default()).await.unwrap();
        let session_id = analytics.start_session("user123").await;
        assert!(session_id.is_ok());
        assert!(!session_id.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_track_event() {
        let analytics = UsageAnalytics::new(AnalyticsConfig::default()).await.unwrap();
        
        let event = UsageEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_name: "test_event".to_string(),
            user_id: Some("user123".to_string()),
            session_id: None,
            timestamp: Utc::now(),
            properties: HashMap::new(),
            category: EventCategory::Custom,
        };
        
        let result = analytics.track_event(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let analytics = UsageAnalytics::new(AnalyticsConfig::default()).await.unwrap();
        
        let session_id = analytics.start_session("user123").await.unwrap();
        let session = analytics.get_session(&session_id).await;
        assert!(session.is_some());
        assert!(session.unwrap().end_time.is_none());
        
        analytics.end_session(&session_id).await.unwrap();
        let session = analytics.get_session(&session_id).await;
        assert!(session.unwrap().end_time.is_some());
    }
}