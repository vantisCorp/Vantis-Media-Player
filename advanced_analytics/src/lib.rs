//! Advanced Analytics & Telemetry System
//! 
//! This module provides comprehensive analytics and telemetry capabilities including:
//! - Usage analytics
//! - Crash reporting
//! - Performance metrics dashboard
//! - User feedback system
//! - A/B testing framework

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub mod usage;
pub mod crash_reporting;
pub mod performance;
pub mod feedback;
pub mod ab_testing;
pub mod utils;

use crash_reporting::{CrashReporter, CrashReport, CrashSeverity};
use feedback::{FeedbackCollector, Feedback, FeedbackType};
use performance::{PerformanceMonitor, PerformanceMetric, MetricType};
use usage::{UsageAnalytics, UsageEvent, UserSession};
use ab_testing::{ABTestingEngine, Experiment, Variant, ExperimentStatus};

/// Advanced Analytics & Telemetry Engine
/// 
/// Coordinates all analytics and telemetry operations including usage tracking,
/// crash reporting, performance monitoring, user feedback, and A/B testing.
#[derive(Clone)]
pub struct AdvancedAnalyticsEngine {
    /// Usage analytics
    usage: Arc<UsageAnalytics>,
    /// Crash reporter
    crash_reporter: Arc<CrashReporter>,
    /// Performance monitor
    performance: Arc<PerformanceMonitor>,
    /// Feedback collector
    feedback: Arc<FeedbackCollector>,
    /// A/B testing engine
    ab_testing: Arc<ABTestingEngine>,
    /// Configuration
    config: AnalyticsConfig,
    /// Enabled state
    enabled: Arc<RwLock<bool>>,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Enable analytics
    pub enabled: bool,
    /// Enable crash reporting
    pub crash_reporting: bool,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
    /// Enable user feedback
    pub user_feedback: bool,
    /// Enable A/B testing
    pub ab_testing: bool,
    /// Analytics endpoint URL
    pub analytics_endpoint: Option<String>,
    /// Sentry DSN for crash reporting
    pub sentry_dsn: Option<String>,
    /// Sample rate for analytics (0.0 to 1.0)
    pub sample_rate: f64,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            crash_reporting: true,
            performance_monitoring: true,
            user_feedback: true,
            ab_testing: true,
            analytics_endpoint: None,
            sentry_dsn: None,
            sample_rate: 1.0,
            user_id: None,
            session_id: None,
        }
    }
}

impl AdvancedAnalyticsEngine {
    /// Create a new advanced analytics engine
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing Advanced Analytics Engine");

        let usage = Arc::new(UsageAnalytics::new(config.clone()).await?);
        let crash_reporter = Arc::new(CrashReporter::new(config.clone()).await?);
        let performance = Arc::new(PerformanceMonitor::new(config.clone()).await?);
        let feedback = Arc::new(FeedbackCollector::new(config.clone()).await?);
        let ab_testing = Arc::new(ABTestingEngine::new(config.clone()).await?);

        Ok(Self {
            usage,
            crash_reporter,
            performance,
            feedback,
            ab_testing,
            config,
            enabled: Arc::new(RwLock::new(config.enabled)),
        })
    }

    /// Track a usage event
    pub async fn track_event(&self, event: UsageEvent) -> Result<()> {
        if !*self.enabled.read().await {
            return Ok(());
        }

        self.usage.track_event(event).await
    }

    /// Start a user session
    pub async fn start_session(&self, user_id: &str) -> Result<String> {
        if !*self.enabled.read().await {
            return Ok(String::new());
        }

        self.usage.start_session(user_id).await
    }

    /// End a user session
    pub async fn end_session(&self, session_id: &str) -> Result<()> {
        if !*self.enabled.read().await {
            return Ok(());
        }

        self.usage.end_session(session_id).await
    }

    /// Report a crash
    pub async fn report_crash(&self, report: CrashReport) -> Result<()> {
        if !*self.enabled.read().await || !self.config.crash_reporting {
            return Ok(());
        }

        self.crash_reporter.report(report).await
    }

    /// Record a performance metric
    pub async fn record_metric(&self, metric: PerformanceMetric) -> Result<()> {
        if !*self.enabled.read().await || !self.config.performance_monitoring {
            return Ok(());
        }

        self.performance.record_metric(metric).await
    }

    /// Collect user feedback
    pub async fn collect_feedback(&self, feedback: Feedback) -> Result<()> {
        if !*self.enabled.read().await || !self.config.user_feedback {
            return Ok(());
        }

        self.feedback.collect(feedback).await
    }

    /// Get or create experiment variant for user
    pub async fn get_variant(&self, experiment_id: &str, user_id: &str) -> Result<Option<Variant>> {
        if !*self.enabled.read().await || !self.config.ab_testing {
            return Ok(None);
        }

        self.ab_testing.get_variant(experiment_id, user_id).await
    }

    /// Track experiment conversion
    pub async fn track_conversion(&self, experiment_id: &str, user_id: &str, variant_id: &str) -> Result<()> {
        if !*self.enabled.read().await || !self.config.ab_testing {
            return Ok(());
        }

        self.ab_testing.track_conversion(experiment_id, user_id, variant_id).await
    }

    /// Get usage statistics
    pub async fn get_usage_stats(&self) -> UsageStats {
        self.usage.get_stats().await
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> Vec<PerformanceMetric> {
        self.performance.get_metrics().await
    }

    /// Get feedback
    pub async fn get_feedback(&self) -> Vec<Feedback> {
        self.feedback.get_feedback().await
    }

    /// Get experiment results
    pub async fn get_experiment_results(&self, experiment_id: &str) -> Option<ExperimentResults> {
        self.ab_testing.get_results(experiment_id).await
    }

    /// Enable or disable analytics
    pub async fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().await = enabled;
        info!("Analytics {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Check if analytics is enabled
    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    /// Flush all pending data
    pub async fn flush(&self) -> Result<()> {
        self.usage.flush().await?;
        self.performance.flush().await?;
        self.feedback.flush().await?;
        self.ab_testing.flush().await?;
        Ok(())
    }
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
}

/// Experiment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    /// Experiment ID
    pub experiment_id: String,
    /// Total participants
    pub total_participants: usize,
    /// Total conversions
    pub total_conversions: usize,
    /// Conversion rate
    pub conversion_rate: f64,
    /// Variant results
    pub variants: HashMap<String, VariantResult>,
}

/// Variant result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResult {
    /// Variant ID
    pub variant_id: String,
    /// Participants
    pub participants: usize,
    /// Conversions
    pub conversions: usize,
    /// Conversion rate
    pub conversion_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_analytics_engine_creation() {
        let config = AnalyticsConfig::default();
        let engine = AdvancedAnalyticsEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_analytics_config_default() {
        let config = AnalyticsConfig::default();
        assert!(config.enabled);
        assert!(config.crash_reporting);
        assert!(config.performance_monitoring);
        assert_eq!(config.sample_rate, 1.0);
    }

    #[tokio::test]
    async fn test_enable_disable_analytics() {
        let engine = AdvancedAnalyticsEngine::new(AnalyticsConfig::default()).await.unwrap();
        
        assert!(engine.is_enabled().await);
        
        engine.set_enabled(false).await;
        assert!(!engine.is_enabled().await);
        
        engine.set_enabled(true).await;
        assert!(engine.is_enabled().await);
    }
}