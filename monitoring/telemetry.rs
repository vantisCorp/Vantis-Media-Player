//! User telemetry module for Vantis Media Player
//!
//! Privacy-first telemetry for understanding usage patterns.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Telemetry event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryEvent {
    /// Application started
    AppStarted {
        version: String,
        platform: String,
        architecture: String,
    },
    /// Media file opened
    MediaOpened {
        format: String,
        duration_ms: Option<u64>,
        has_subtitles: bool,
    },
    /// Feature used
    FeatureUsed {
        feature: String,
        context: HashMap<String, String>,
    },
    /// Performance metric
    Performance {
        operation: String,
        duration_ms: u64,
        success: bool,
    },
    /// Error occurred
    ErrorOccurred {
        error_type: String,
        component: String,
    },
}

/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Whether telemetry is enabled
    pub enabled: bool,
    /// Anonymous user ID
    pub user_id: String,
    /// Session ID
    pub session_id: String,
    /// Endpoint URL
    pub endpoint: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            user_id: uuid::Uuid::new_v4().to_string(),
            session_id: uuid::Uuid::new_v4().to_string(),
            endpoint: String::new(),
        }
    }
}

/// Telemetry client
pub struct TelemetryClient {
    config: TelemetryConfig,
    http_client: reqwest::Client,
    event_queue: Vec<TelemetryEvent>,
    last_flush: Instant,
}

impl TelemetryClient {
    /// Create a new telemetry client
    pub fn new(config: TelemetryConfig) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        Ok(Self {
            config,
            http_client,
            event_queue: Vec::new(),
            last_flush: Instant::now(),
        })
    }

    /// Track a telemetry event
    pub fn track(&mut self, event: TelemetryEvent) {
        if !self.config.enabled {
            return;
        }
        
        self.event_queue.push(event);
        
        // Flush every 30 seconds or when queue has 100 events
        if self.last_flush.elapsed() > Duration::from_secs(30) 
            || self.event_queue.len() >= 100 {
            if let Err(e) = self.flush() {
                tracing::error!("Failed to flush telemetry: {}", e);
            }
        }
    }

    /// Flush queued events to the server
    pub fn flush(&mut self) -> Result<()> {
        if self.event_queue.is_empty() {
            return Ok(());
        }

        let events = std::mem::take(&mut self.event_queue);
        self.last_flush = Instant::now();

        let payload = serde_json::json!({
            "user_id": self.config.user_id,
            "session_id": self.config.session_id,
            "events": events,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        // Send asynchronously (fire and forget)
        let client = self.http_client.clone();
        let endpoint = self.config.endpoint.clone();
        
        tokio::spawn(async move {
            if let Err(e) = client.post(&endpoint).json(&payload).send().await {
                tracing::debug!("Telemetry send failed: {}", e);
            }
        });

        Ok(())
    }

    /// Check if telemetry is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Get the anonymous user ID
    pub fn user_id(&self) -> &str {
        &self.config.user_id
    }
}

impl Drop for TelemetryClient {
    fn drop(&mut self) {
        if !self.event_queue.is_empty() {
            let _ = self.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_disabled() {
        let config = TelemetryConfig::default();
        assert!(!config.enabled);
    }

    #[test]
    fn test_telemetry_event_serialization() {
        let event = TelemetryEvent::AppStarted {
            version: "1.0.0".to_string(),
            platform: "linux".to_string(),
            architecture: "x86_64".to_string(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("app_started"));
    }
}