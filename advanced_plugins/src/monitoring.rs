//! Performance Monitoring
//! 
//! Provides performance metrics, profiling, and health monitoring for plugins.

use anyhow::Result;
use parking_lot::RwLock;
use prometheus::{Counter, Histogram, Gauge, Registry, TextEncoder};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn, error};

/// Performance monitor
pub struct PerformanceMonitor {
    /// Metrics registry
    registry: Registry,
    
    /// Plugin metrics
    metrics: Arc<RwLock<HashMap<String, PluginMetrics>>>,
    
    /// Monitoring interval in seconds
    interval: u64,
    
    /// Active flag
    active: Arc<RwLock<bool>>,
    
    /// Prometheus metrics
    prometheus: PrometheusMetrics,
}

/// Plugin metrics
#[derive(Clone, Debug)]
pub struct PluginMetrics {
    /// Plugin name
    pub name: String,
    
    /// Execution count
    pub execution_count: u64,
    
    /// Total execution time
    pub total_execution_time: Duration,
    
    /// Average execution time
    pub average_execution_time: Duration,
    
    /// Memory usage in bytes
    pub memory_usage: usize,
    
    /// CPU usage percentage
    pub cpu_usage: f64,
    
    /// Error count
    pub error_count: u64,
    
    /// Last execution time
    pub last_execution: Option<Instant>,
    
    /// Peak memory usage
    pub peak_memory: usize,
    
    /// Peak CPU usage
    pub peak_cpu: f64,
}

/// Prometheus metrics
struct PrometheusMetrics {
    /// Execution counter
    execution_counter: Counter,
    
    /// Execution time histogram
    execution_time: Histogram,
    
    /// Memory usage gauge
    memory_usage: Gauge,
    
    /// CPU usage gauge
    cpu_usage: Gauge,
    
    /// Error counter
    error_counter: Counter,
}

/// Performance snapshot
#[derive(Clone, Debug)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Plugin metrics
    pub metrics: HashMap<String, PluginMetrics>,
    
    /// System metrics
    pub system: SystemMetrics,
}

/// System metrics
#[derive(Clone, Debug)]
pub struct SystemMetrics {
    /// Total memory usage
    pub total_memory: usize,
    
    /// Total CPU usage
    pub total_cpu: f64,
    
    /// Active plugins
    pub active_plugins: usize,
    
    /// Total executions
    pub total_executions: u64,
    
    /// Total errors
    pub total_errors: u64,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(interval: u64) -> Result<Self> {
        info!("📊 Initializing Performance Monitor");
        
        let registry = Registry::new();
        
        // Create Prometheus metrics
        let execution_counter = Counter::new(
            "plugin_executions_total",
            "Total number of plugin executions"
        )?;
        
        let execution_time = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "plugin_execution_duration_seconds",
                "Plugin execution duration in seconds"
            ).buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
        )?;
        
        let memory_usage = Gauge::new(
            "plugin_memory_usage_bytes",
            "Plugin memory usage in bytes"
        )?;
        
        let cpu_usage = Gauge::new(
            "plugin_cpu_usage_percent",
            "Plugin CPU usage percentage"
        )?;
        
        let error_counter = Counter::new(
            "plugin_errors_total",
            "Total number of plugin errors"
        )?;
        
        // Register metrics
        registry.register(Box::new(execution_counter.clone()))?;
        registry.register(Box::new(execution_time.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(error_counter.clone()))?;
        
        let prometheus = PrometheusMetrics {
            execution_counter,
            execution_time,
            memory_usage,
            cpu_usage,
            error_counter,
        };
        
        info!("✅ Performance monitor initialized");
        info!("   - Interval: {} seconds", interval);
        
        Ok(Self {
            registry,
            metrics: Arc::new(RwLock::new(HashMap::new())),
            interval,
            active: Arc::new(RwLock::new(false)),
            prometheus,
        })
    }
    
    /// Check if monitoring is active
    pub fn is_active(&self) -> bool {
        *self.active.read()
    }
    
    /// Start monitoring
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting Performance Monitor");
        
        if self.is_active() {
            warn!("⚠️  Monitoring already active");
            return Ok(());
        }
        
        *self.active.write() = true;
        
        // Start periodic collection
        let metrics = self.metrics.clone();
        let active = self.active.clone();
        let prometheus = self.prometheus.clone();
        
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(Duration::from_secs(5));
            
            while *active.read() {
                timer.tick().await;
                
                // Update Prometheus metrics
                let metrics_guard = metrics.read();
                for (name, plugin_metrics) in metrics_guard.iter() {
                    let labels = &[("plugin", name.as_str())];
                    
                    // Update gauges
                    prometheus.memory_usage.set(plugin_metrics.memory_usage as f64, labels);
                    prometheus.cpu_usage.set(plugin_metrics.cpu_usage, labels);
                }
            }
        });
        
        info!("✅ Performance monitor started");
        
        Ok(())
    }
    
    /// Stop monitoring
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Performance Monitor");
        
        *self.active.write() = false;
        
        info!("✅ Performance monitor stopped");
        
        Ok(())
    }
    
    /// Record execution
    pub fn record_execution(&self, name: &str, duration: Duration) {
        let mut metrics = self.metrics.write();
        
        let plugin_metrics = metrics.entry(name.to_string()).or_insert_with(|| {
            PluginMetrics {
                name: name.to_string(),
                execution_count: 0,
                total_execution_time: Duration::ZERO,
                average_execution_time: Duration::ZERO,
                memory_usage: 0,
                cpu_usage: 0.0,
                error_count: 0,
                last_execution: None,
                peak_memory: 0,
                peak_cpu: 0.0,
            }
        });
        
        plugin_metrics.execution_count += 1;
        plugin_metrics.total_execution_time += duration;
        plugin_metrics.average_execution_time = plugin_metrics.total_execution_time / plugin_metrics.execution_count as u32;
        plugin_metrics.last_execution = Some(Instant::now());
        
        // Update Prometheus
        let labels = &[("plugin", name)];
        let _ = self.prometheus.execution_counter.inc_by(1, labels);
        let _ = self.prometheus.execution_time.observe(duration.as_secs_f64(), labels);
        
        debug!("📊 Recorded execution for {}: {:?}", name, duration);
    }
    
    /// Record error
    pub fn record_error(&self, name: &str) {
        let mut metrics = self.metrics.write();
        
        if let Some(plugin_metrics) = metrics.get_mut(name) {
            plugin_metrics.error_count += 1;
        }
        
        // Update Prometheus
        let labels = &[("plugin", name)];
        let _ = self.prometheus.error_counter.inc_by(1, labels);
        
        debug!("📊 Recorded error for: {}", name);
    }
    
    /// Update memory usage
    pub fn update_memory(&self, name: &str, usage: usize) {
        let mut metrics = self.metrics.write();
        
        if let Some(plugin_metrics) = metrics.get_mut(name) {
            plugin_metrics.memory_usage = usage;
            plugin_metrics.peak_memory = plugin_metrics.peak_memory.max(usage);
        }
    }
    
    /// Update CPU usage
    pub fn update_cpu(&self, name: &str, usage: f64) {
        let mut metrics = self.metrics.write();
        
        if let Some(plugin_metrics) = metrics.get_mut(name) {
            plugin_metrics.cpu_usage = usage;
            plugin_metrics.peak_cpu = plugin_metrics.peak_cpu.max(usage);
        }
    }
    
    /// Get plugin metrics
    pub fn get_metrics(&self, name: &str) -> Option<PluginMetrics> {
        let metrics = self.metrics.read();
        metrics.get(name).cloned()
    }
    
    /// Get all metrics
    pub fn get_all_metrics(&self) -> HashMap<String, PluginMetrics> {
        let metrics = self.metrics.read();
        metrics.clone()
    }
    
    /// Take snapshot
    pub fn take_snapshot(&self) -> PerformanceSnapshot {
        let metrics = self.metrics.read();
        
        let total_memory: usize = metrics.values().map(|m| m.memory_usage).sum();
        let total_cpu: f64 = metrics.values().map(|m| m.cpu_usage).sum();
        let total_executions: u64 = metrics.values().map(|m| m.execution_count).sum();
        let total_errors: u64 = metrics.values().map(|m| m.error_count).sum();
        
        PerformanceSnapshot {
            timestamp: chrono::Utc::now(),
            metrics: metrics.clone(),
            system: SystemMetrics {
                total_memory,
                total_cpu,
                active_plugins: metrics.len(),
                total_executions,
                total_errors,
            },
        }
    }
    
    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let result = encoder.encode_to_string(&metric_families)?;
        Ok(result)
    }
    
    /// Reset metrics for a plugin
    pub fn reset_metrics(&self, name: &str) {
        let mut metrics = self.metrics.write();
        
        if let Some(plugin_metrics) = metrics.get_mut(name) {
            plugin_metrics.execution_count = 0;
            plugin_metrics.total_execution_time = Duration::ZERO;
            plugin_metrics.average_execution_time = Duration::ZERO;
            plugin_metrics.error_count = 0;
            plugin_metrics.last_execution = None;
        }
        
        debug!("📊 Reset metrics for: {}", name);
    }
    
    /// Reset all metrics
    pub fn reset_all_metrics(&self) {
        let mut metrics = self.metrics.write();
        metrics.clear();
        debug!("📊 Reset all metrics");
    }
    
    /// Get performance report
    pub fn get_report(&self) -> String {
        let metrics = self.metrics.read();
        
        let mut report = String::from("📊 Performance Report\n");
        report.push_str(&format!("{}\n", "=".repeat(50)));
        report.push_str(&format!("Active Plugins: {}\n\n", metrics.len()));
        
        for (name, plugin_metrics) in metrics.iter() {
            report.push_str(&format!("Plugin: {}\n", name));
            report.push_str(&format!("  Executions: {}\n", plugin_metrics.execution_count));
            report.push_str(&format!("  Avg Time: {:?}\n", plugin_metrics.average_execution_time));
            report.push_str(&format!("  Memory: {} MB\n", plugin_metrics.memory_usage / 1024 / 1024));
            report.push_str(&format!("  CPU: {:.2}%\n", plugin_metrics.cpu_usage));
            report.push_str(&format!("  Errors: {}\n", plugin_metrics.error_count));
            report.push_str(&format!("  Peak Memory: {} MB\n", plugin_metrics.peak_memory / 1024 / 1024));
            report.push_str(&format!("  Peak CPU: {:.2}%\n", plugin_metrics.peak_cpu));
            report.push_str("\n");
        }
        
        report
    }
}

impl Clone for PrometheusMetrics {
    fn clone(&self) -> Self {
        Self {
            execution_counter: self.execution_counter.clone(),
            execution_time: self.execution_time.clone(),
            memory_usage: self.memory_usage.clone(),
            cpu_usage: self.cpu_usage.clone(),
            error_counter: self.error_counter.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new(10);
        assert!(monitor.is_ok());
        assert!(!monitor.is_active());
    }
    
    #[test]
    fn test_record_execution() {
        let monitor = PerformanceMonitor::new(10).unwrap();
        
        monitor.record_execution("test_plugin", Duration::from_millis(100));
        monitor.record_execution("test_plugin", Duration::from_millis(200));
        
        let metrics = monitor.get_metrics("test_plugin");
        assert!(metrics.is_some());
        
        let metrics = metrics.unwrap();
        assert_eq!(metrics.execution_count, 2);
        assert_eq!(metrics.total_execution_time, Duration::from_millis(300));
    }
    
    #[test]
    fn test_record_error() {
        let monitor = PerformanceMonitor::new(10).unwrap();
        
        monitor.record_error("test_plugin");
        monitor.record_error("test_plugin");
        
        let metrics = monitor.get_metrics("test_plugin");
        assert!(metrics.is_some());
        assert_eq!(metrics.unwrap().error_count, 2);
    }
    
    #[test]
    fn test_update_memory() {
        let monitor = PerformanceMonitor::new(10).unwrap();
        
        monitor.update_memory("test_plugin", 1024 * 1024);
        monitor.update_memory("test_plugin", 2 * 1024 * 1024);
        
        let metrics = monitor.get_metrics("test_plugin");
        assert!(metrics.is_some());
        assert_eq!(metrics.unwrap().memory_usage, 2 * 1024 * 1024);
    }
    
    #[test]
    fn test_snapshot() {
        let monitor = PerformanceMonitor::new(10).unwrap();
        
        monitor.record_execution("test_plugin", Duration::from_millis(100));
        monitor.update_memory("test_plugin", 1024 * 1024);
        monitor.update_cpu("test_plugin", 50.0);
        
        let snapshot = monitor.take_snapshot();
        assert_eq!(snapshot.metrics.len(), 1);
        assert_eq!(snapshot.system.active_plugins, 1);
        assert_eq!(snapshot.system.total_executions, 1);
    }
}