//! Health Monitoring System
//! 
//! Provides comprehensive health monitoring including:
//! - Application health checks
//! - Performance metrics
//! - Resource usage monitoring
//! - Alerting and notifications
//! - Health dashboards

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System is degraded but operational
    Degraded,
    /// System is unhealthy
    Unhealthy,
    /// System status is unknown
    Unknown,
}

/// Metric type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter metric
    Counter,
    /// Gauge metric
    Gauge,
    /// Histogram metric
    Histogram,
    /// Summary metric
    Summary,
}

/// Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Metric value
    pub value: f64,
    /// Metric labels
    pub labels: HashMap<String, String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Check name
    pub name: String,
    /// Check status
    pub status: HealthStatus,
    /// Check message
    pub message: String,
    /// Check duration in milliseconds
    pub duration_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Check name
    pub name: String,
    /// Check interval in seconds
    pub interval: u64,
    /// Timeout in seconds
    pub timeout: u64,
    /// Number of consecutive failures before alerting
    pub failure_threshold: usize,
    /// Enable check
    pub enabled: bool,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Alert name
    pub name: String,
    /// Alert condition
    pub condition: AlertCondition,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
    /// Cooldown period in seconds
    pub cooldown: u64,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    /// Metric exceeds threshold
    MetricAboveThreshold {
        metric_name: String,
        threshold: f64,
    },
    /// Metric falls below threshold
    MetricBelowThreshold {
        metric_name: String,
        threshold: f64,
    },
    /// Health check fails
    HealthCheckFailed {
        check_name: String,
    },
    /// Custom condition
    Custom(String),
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notification
    Email { address: String },
    /// Slack notification
    Slack { webhook_url: String },
    /// Webhook notification
    Webhook { url: String },
    /// Custom notification
    Custom { name: String, config: HashMap<String, String> },
}

/// Health monitor
#[derive(Clone)]
pub struct HealthMonitor {
    /// Health status
    status: Arc<RwLock<HealthStatus>>,
    /// Health check results
    health_checks: Arc<RwLock<HashMap<String, HealthCheckResult>>>,
    /// Metrics
    metrics: Arc<RwLock<HashMap<String, Metric>>>,
    /// Health check configurations
    check_configs: Arc<RwLock<HashMap<String, HealthCheckConfig>>>,
    /// Alert configurations
    alert_configs: Arc<RwLock<HashMap<String, AlertConfig>>>,
    /// Alert history
    alert_history: Arc<RwLock<Vec<Alert>>>,
    /// Start time
    start_time: Instant,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Alert name
    pub name: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Resolved
    pub resolved: bool,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub async fn new() -> Result<Self> {
        info!("Initializing Health Monitor");

        Ok(Self {
            status: Arc::new(RwLock::new(HealthStatus::Healthy)),
            health_checks: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            check_configs: Arc::new(RwLock::new(HashMap::new())),
            alert_configs: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
        })
    }

    /// Get current health status
    pub async fn get_status(&self) -> HealthStatus {
        *self.status.read().await
    }

    /// Update health status
    pub async fn update_status(&self, status: HealthStatus) {
        *self.status.write().await = status;
        info!("Health status updated: {:?}", status);
    }

    /// Record a health check result
    pub async fn record_health_check(&self, result: HealthCheckResult) {
        let mut checks = self.health_checks.write().await;
        checks.insert(result.name.clone(), result.clone());

        // Update overall status based on check results
        self.update_overall_status(&checks).await;
    }

    /// Update overall health status
    async fn update_overall_status(&self, checks: &HashMap<String, HealthCheckResult>) {
        let mut has_unhealthy = false;
        let mut has_degraded = false;

        for result in checks.values() {
            match result.status {
                HealthStatus::Unhealthy => has_unhealthy = true,
                HealthStatus::Degraded => has_degraded = true,
                _ => {}
            }
        }

        let new_status = if has_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        *self.status.write().await = new_status;
    }

    /// Get health check results
    pub async fn get_health_checks(&self) -> HashMap<String, HealthCheckResult> {
        self.health_checks.read().await.clone()
    }

    /// Record a metric
    pub async fn record_metric(&self, metric: Metric) {
        let mut metrics = self.metrics.write().await;
        metrics.insert(metric.name.clone(), metric.clone());
        debug!("Metric recorded: {} = {}", metric.name, metric.value);
    }

    /// Increment a counter metric
    pub async fn increment_counter(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: MetricType::Counter,
            value,
            labels,
            timestamp: Utc::now(),
        };
        self.record_metric(metric).await;
    }

    /// Set a gauge metric
    pub async fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: MetricType::Gauge,
            value,
            labels,
            timestamp: Utc::now(),
        };
        self.record_metric(metric).await;
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> Vec<Metric> {
        self.metrics.read().await.values().cloned().collect()
    }

    /// Get metric by name
    pub async fn get_metric(&self, name: &str) -> Option<Metric> {
        self.metrics.read().await.get(name).cloned()
    }

    /// Add health check configuration
    pub async fn add_check_config(&self, config: HealthCheckConfig) {
        let mut configs = self.check_configs.write().await;
        configs.insert(config.name.clone(), config);
    }

    /// Add alert configuration
    pub async fn add_alert_config(&self, config: AlertConfig) {
        let mut configs = self.alert_configs.write().await;
        configs.insert(config.name.clone(), config);
    }

    /// Get alert history
    pub async fn get_alert_history(&self) -> Vec<Alert> {
        self.alert_history.read().await.clone()
    }

    /// Create an alert
    pub async fn create_alert(&self, name: &str, severity: AlertSeverity, message: &str) {
        let alert = Alert {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            severity,
            message: message.to_string(),
            timestamp: Utc::now(),
            resolved: false,
        };

        let mut history = self.alert_history.write().await;
        history.push(alert.clone());

        warn!("Alert created: {} - {}", name, message);
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: &str) {
        let mut history = self.alert_history.write().await;
        if let Some(alert) = history.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            info!("Alert resolved: {}", alert_id);
        }
    }

    /// Get uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        // In a real implementation, this would gather actual system metrics
        // For now, we'll return placeholder values
        
        SystemMetrics {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_in: 0.0,
            network_out: 0.0,
            uptime: self.get_uptime().as_secs(),
        }
    }
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network in bytes per second
    pub network_in: f64,
    /// Network out bytes per second
    pub network_out: f64,
    /// Uptime in seconds
    pub uptime: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new().await;
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_health_status_update() {
        let monitor = HealthMonitor::new().await.unwrap();
        
        monitor.update_status(HealthStatus::Degraded).await;
        assert_eq!(monitor.get_status().await, HealthStatus::Degraded);
    }

    #[tokio::test]
    async fn test_metric_recording() {
        let monitor = HealthMonitor::new().await.unwrap();
        
        let metric = Metric {
            name: "test_metric".to_string(),
            metric_type: MetricType::Counter,
            value: 42.0,
            labels: HashMap::new(),
            timestamp: Utc::now(),
        };
        
        monitor.record_metric(metric).await;
        
        let retrieved = monitor.get_metric("test_metric").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 42.0);
    }

    #[tokio::test]
    async fn test_health_check_recording() {
        let monitor = HealthMonitor::new().await.unwrap();
        
        let result = HealthCheckResult {
            name: "test_check".to_string(),
            status: HealthStatus::Healthy,
            message: "OK".to_string(),
            duration_ms: 100,
            timestamp: Utc::now(),
        };
        
        monitor.record_health_check(result).await;
        
        let checks = monitor.get_health_checks().await;
        assert!(checks.contains_key("test_check"));
    }

    #[tokio::test]
    async fn test_alert_creation() {
        let monitor = HealthMonitor::new().await.unwrap();
        
        monitor.create_alert("test_alert", AlertSeverity::Warning, "Test message").await;
        
        let history = monitor.get_alert_history().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].name, "test_alert");
    }
}