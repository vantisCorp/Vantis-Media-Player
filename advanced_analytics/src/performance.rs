//! Performance Monitoring
//! 
//! Provides performance monitoring capabilities including:
//! - Metric collection
//! - Performance dashboards
//! - Real-time monitoring
//! - Performance alerts

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use prometheus::{Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramVec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::AnalyticsConfig;

/// Performance monitor
#[derive(Clone)]
pub struct PerformanceMonitor {
    /// Configuration
    config: AnalyticsConfig,
    /// Metrics storage
    metrics: Arc<RwLock<Vec<PerformanceMetric>>>,
    /// Prometheus metrics
    prometheus_metrics: PrometheusMetrics,
}

/// Prometheus metrics
#[derive(Clone)]
pub struct PrometheusMetrics {
    /// Request counter
    pub request_counter: CounterVec,
    /// Request duration histogram
    pub request_duration: HistogramVec,
    /// Active connections gauge
    pub active_connections: GaugeVec,
    /// Memory usage gauge
    pub memory_usage: Gauge,
    /// CPU usage gauge
    pub cpu_usage: Gauge,
}

/// Performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    /// Metric ID
    pub metric_id: String,
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Metric value
    pub value: f64,
    /// Metric labels
    pub labels: HashMap<String, String>,
    /// Metric timestamp
    pub timestamp: DateTime<Utc>,
    /// Metric unit
    pub unit: String,
}

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing Performance Monitor");

        let prometheus_metrics = PrometheusMetrics::new()?;

        Ok(Self {
            config,
            metrics: Arc::new(RwLock::new(Vec::new())),
            prometheus_metrics,
        })
    }

    /// Record a performance metric
    pub async fn record_metric(&self, metric: PerformanceMetric) -> Result<()> {
        debug!("Recording metric: {} = {}", metric.name, metric.value);

        // Add to metrics storage
        let mut metrics = self.metrics.write().await;
        metrics.push(metric.clone());

        // Update Prometheus metrics
        self.update_prometheus_metrics(&metric).await;

        info!("Metric recorded: {}", metric.name);
        Ok(())
    }

    /// Record a counter metric
    pub async fn record_counter(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let metric = PerformanceMetric {
            metric_id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            metric_type: MetricType::Counter,
            value,
            labels,
            timestamp: Utc::now(),
            unit: "count".to_string(),
        };

        self.record_metric(metric).await
    }

    /// Record a gauge metric
    pub async fn record_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let metric = PerformanceMetric {
            metric_id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            metric_type: MetricType::Gauge,
            value,
            labels,
            timestamp: Utc::now(),
            unit: "value".to_string(),
        };

        self.record_metric(metric).await
    }

    /// Record a histogram metric
    pub async fn record_histogram(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let metric = PerformanceMetric {
            metric_id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            metric_type: MetricType::Histogram,
            value,
            labels,
            timestamp: Utc::now(),
            unit: "seconds".to_string(),
        };

        self.record_metric(metric).await
    }

    /// Record request duration
    pub async fn record_request_duration(&self, endpoint: &str, duration: f64) -> Result<()> {
        let labels = vec![("endpoint".to_string(), endpoint.to_string())]
            .into_iter()
            .collect();

        self.prometheus_metrics
            .request_duration
            .with_label_values(&[endpoint])
            .observe(duration);

        self.record_histogram("request_duration", duration, labels).await
    }

    /// Increment request counter
    pub async fn increment_request_counter(&self, endpoint: &str, status: u16) -> Result<()> {
        let labels = vec![
            ("endpoint".to_string(), endpoint.to_string()),
            ("status".to_string(), status.to_string()),
        ]
        .into_iter()
        .collect();

        self.prometheus_metrics
            .request_counter
            .with_label_values(&[endpoint, &status.to_string()])
            .inc();

        self.record_counter("requests_total", 1.0, labels).await
    }

    /// Update active connections
    pub async fn update_active_connections(&self, count: u64) -> Result<()> {
        self.prometheus_metrics
            .active_connections
            .with_label_values(&["total"])
            .set(count as f64);

        self.record_gauge("active_connections", count as f64, HashMap::new())
            .await
    }

    /// Update memory usage
    pub async fn update_memory_usage(&self, bytes: u64) -> Result<()> {
        self.prometheus_metrics.memory_usage.set(bytes as f64);
        self.record_gauge("memory_usage_bytes", bytes as f64, HashMap::new())
            .await
    }

    /// Update CPU usage
    pub async fn update_cpu_usage(&self, percentage: f64) -> Result<()> {
        self.prometheus_metrics.cpu_usage.set(percentage);
        self.record_gauge("cpu_usage_percent", percentage, HashMap::new())
            .await
    }

    /// Get all metrics
    pub async fn get_metrics(&self) -> Vec<PerformanceMetric> {
        self.metrics.read().await.clone()
    }

    /// Get metrics by name
    pub async fn get_metrics_by_name(&self, name: &str) -> Vec<PerformanceMetric> {
        self.metrics
            .read()
            .await
            .iter()
            .filter(|m| m.name == name)
            .cloned()
            .collect()
    }

    /// Get metrics by type
    pub async fn get_metrics_by_type(&self, metric_type: MetricType) -> Vec<PerformanceMetric> {
        self.metrics
            .read()
            .await
            .iter()
            .filter(|m| m.metric_type == metric_type)
            .cloned()
            .collect()
    }

    /// Get Prometheus metrics for export
    pub async fn get_prometheus_metrics(&self) -> Result<String> {
        use prometheus::Encoder;
        
        let encoder = prometheus::TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        
        Ok(String::from_utf8(buffer)?)
    }

    /// Update Prometheus metrics
    async fn update_prometheus_metrics(&self, metric: &PerformanceMetric) {
        match metric.metric_type {
            MetricType::Counter => {
                if let Some(endpoint) = metric.labels.get("endpoint") {
                    let status = metric.labels.get("status").map(|s| s.as_str()).unwrap_or("200");
                    self.prometheus_metrics
                        .request_counter
                        .with_label_values(&[endpoint, status])
                        .inc_by(metric.value);
                }
            }
            MetricType::Gauge => {
                if metric.name == "active_connections" {
                    self.prometheus_metrics
                        .active_connections
                        .with_label_values(&["total"])
                        .set(metric.value);
                } else if metric.name == "memory_usage_bytes" {
                    self.prometheus_metrics.memory_usage.set(metric.value);
                } else if metric.name == "cpu_usage_percent" {
                    self.prometheus_metrics.cpu_usage.set(metric.value);
                }
            }
            MetricType::Histogram => {
                if let Some(endpoint) = metric.labels.get("endpoint") {
                    self.prometheus_metrics
                        .request_duration
                        .with_label_values(&[endpoint])
                        .observe(metric.value);
                }
            }
            MetricType::Summary => {
                // Summary metrics not yet implemented
            }
        }
    }

    /// Flush metrics
    pub async fn flush(&self) -> Result<()> {
        // In a real implementation, this would flush metrics to a monitoring system
        info!("Flushed {} metrics", self.metrics.read().await.len());
        Ok(())
    }

    /// Clear old metrics
    pub async fn clear_old_metrics(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = Utc::now() - older_than;
        
        let mut metrics = self.metrics.write().await;
        let initial_count = metrics.len();
        metrics.retain(|m| m.timestamp > cutoff);
        let removed = initial_count - metrics.len();

        info!("Cleared {} old metrics", removed);
        Ok(removed)
    }
}

impl PrometheusMetrics {
    /// Create new Prometheus metrics
    fn new() -> Result<Self> {
        Ok(Self {
            request_counter: CounterVec::new(
                prometheus::Opts {
                    name: "requests_total".to_string(),
                    help: "Total number of requests".to_string(),
                },
                &["endpoint", "status"],
            )?,
            request_duration: HistogramVec::new(
                prometheus::HistogramOpts {
                    name: "request_duration_seconds".to_string(),
                    help: "Request duration in seconds".to_string(),
                    buckets: vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
                },
                &["endpoint"],
            )?,
            active_connections: GaugeVec::new(
                prometheus::Opts {
                    name: "active_connections".to_string(),
                    help: "Number of active connections".to_string(),
                },
                &["type"],
            )?,
            memory_usage: Gauge::new(
                prometheus::Opts {
                    name: "memory_usage_bytes".to_string(),
                    help: "Memory usage in bytes".to_string(),
                },
            )?,
            cpu_usage: Gauge::new(
                prometheus::Opts {
                    name: "cpu_usage_percent".to_string(),
                    help: "CPU usage percentage".to_string(),
                },
            )?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let config = AnalyticsConfig::default();
        let monitor = PerformanceMonitor::new(config).await;
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_record_counter() {
        let monitor = PerformanceMonitor::new(AnalyticsConfig::default()).await.unwrap();
        let result = monitor.record_counter("test_counter", 1.0, HashMap::new()).await;
        assert!(result.is_ok());
        
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.len(), 1);
    }

    #[tokio::test]
    async fn test_record_gauge() {
        let monitor = PerformanceMonitor::new(AnalyticsConfig::default()).await.unwrap();
        let result = monitor.record_gauge("test_gauge", 42.0, HashMap::new()).await;
        assert!(result.is_ok());
        
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.len(), 1);
    }

    #[tokio::test]
    async fn test_record_histogram() {
        let monitor = PerformanceMonitor::new(AnalyticsConfig::default()).await.unwrap();
        let result = monitor.record_histogram("test_histogram", 0.5, HashMap::new()).await;
        assert!(result.is_ok());
        
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.len(), 1);
    }
}