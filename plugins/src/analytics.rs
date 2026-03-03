//! Plugin Analytics Dashboard
//!
//! Comprehensive analytics system for plugin developers to track plugin usage,
//! performance, and user engagement.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin analytics dashboard
pub struct PluginAnalyticsDashboard {
    installation_tracker: InstallationTracker,
    usage_monitor: UsageMonitor,
    performance_metrics: PerformanceMetricsCollector,
    error_tracker: ErrorTracker,
    download_stats: DownloadStatistics,
    config: AnalyticsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub real_time_tracking: bool,
    pub min_sample_size: usize,
    pub export_format: ExportFormat,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 90,
            real_time_tracking: true,
            min_sample_size: 100,
            export_format: ExportFormat::JSON,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    JSON,
    CSV,
    HTML,
}

#[derive(Debug, Clone)]
pub struct InstallationTracker {
    total_installations: HashMap<String, u64>,
    installation_history: HashMap<String, Vec<InstallationEvent>>,
    active_installations: HashMap<String, u64>,
    uninstallations: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationEvent {
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub platform: Platform,
    pub country: Option<String>,
    pub source: InstallationSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallationSource {
    Marketplace,
    Manual,
    Commandline,
    Update,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct UsageMonitor {
    usage_sessions: HashMap<String, Vec<UsageSession>>,
    avg_session_duration: HashMap<String, f64>,
    feature_usage: HashMap<String, HashMap<String, u64>>,
    daily_active_users: HashMap<String, u64>,
    monthly_active_users: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSession {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: u64,
    pub features_used: Vec<String>,
    pub actions_count: u64,
    pub errors_count: u64,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetricsCollector {
    performance_samples: HashMap<String, Vec<PerformanceSample>>,
    avg_metrics: HashMap<String, AverageMetrics>,
    performance_alerts: HashMap<String, Vec<PerformanceAlert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    pub timestamp: DateTime<Utc>,
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub response_time_ms: f64,
    pub init_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AverageMetrics {
    pub avg_memory_mb: f64,
    pub avg_cpu_percent: f64,
    pub avg_response_time_ms: f64,
    pub avg_init_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub timestamp: DateTime<Utc>,
    pub alert_type: PerformanceAlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub value: f64,
    pub threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceAlertType {
    HighMemoryUsage,
    HighCpuUsage,
    SlowResponse,
    SlowInitialization,
    MemoryLeak,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ErrorTracker {
    error_counts: HashMap<String, u64>,
    error_details: HashMap<String, Vec<ErrorEvent>>,
    error_rate_history: HashMap<String, Vec<ErrorRateSample>>,
    top_errors: HashMap<String, Vec<TopError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub timestamp: DateTime<Utc>,
    pub error_type: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub plugin_version: String,
    pub user_impact: UserImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateSample {
    pub timestamp: DateTime<Utc>,
    pub rate: f64,
    pub sample_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopError {
    pub error_type: String,
    pub count: u64,
    pub percentage: f64,
    pub last_occurrence: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DownloadStatistics {
    total_downloads: HashMap<String, u64>,
    download_history: HashMap<String, Vec<DownloadEvent>>,
    version_downloads: HashMap<String, HashMap<String, u64>>,
    platform_downloads: HashMap<String, HashMap<Platform, u64>>,
    download_trends: HashMap<String, DownloadTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEvent {
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub platform: Platform,
    pub country: Option<String>,
    pub source: InstallationSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTrend {
    pub daily: Vec<DailyCount>,
    pub weekly: Vec<WeeklyCount>,
    pub monthly: Vec<MonthlyCount>,
    pub direction: TrendDirection,
    pub growth_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCount {
    pub date: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyCount {
    pub week_start: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyCount {
    pub month: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

impl PluginAnalyticsDashboard {
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            installation_tracker: InstallationTracker::new(),
            usage_monitor: UsageMonitor::new(),
            performance_metrics: PerformanceMetricsCollector::new(),
            error_tracker: ErrorTracker::new(),
            download_stats: DownloadStatistics::new(),
            config,
        }
    }

    pub fn default_config() -> Self {
        Self::new(AnalyticsConfig::default())
    }

    pub fn track_installation(&mut self, plugin_id: &str, event: InstallationEvent) -> anyhow::Result<()> {
        if !self.config.enabled { return Ok(()); }
        self.installation_tracker.track_installation(plugin_id, event);
        Ok(())
    }

    pub fn track_uninstallation(&mut self, plugin_id: &str) -> anyhow::Result<()> {
        if !self.config.enabled { return Ok(()); }
        self.installation_tracker.track_uninstallation(plugin_id);
        Ok(())
    }

    pub fn start_session(&mut self, plugin_id: &str) -> String {
        self.usage_monitor.start_session(plugin_id)
    }

    pub fn end_session(&mut self, plugin_id: &str, session_id: &str, features_used: Vec<String>, actions_count: u64, errors_count: u64) -> anyhow::Result<()> {
        self.usage_monitor.end_session(plugin_id, session_id, features_used, actions_count, errors_count)
    }

    pub fn record_performance(&mut self, plugin_id: &str, sample: PerformanceSample) -> anyhow::Result<()> {
        if !self.config.enabled { return Ok(()); }
        self.performance_metrics.record_sample(plugin_id, sample)
    }

    pub fn track_error(&mut self, plugin_id: &str, error: ErrorEvent) -> anyhow::Result<()> {
        if !self.config.enabled { return Ok(()); }
        self.error_tracker.track_error(plugin_id, error);
        Ok(())
    }

    pub fn track_download(&mut self, plugin_id: &str, event: DownloadEvent) -> anyhow::Result<()> {
        if !self.config.enabled { return Ok(()); }
        self.download_stats.track_download(plugin_id, event);
        Ok(())
    }

    pub fn get_summary(&self, plugin_id: &str) -> PluginAnalyticsSummary {
        PluginAnalyticsSummary {
            plugin_id: plugin_id.to_string(),
            total_installations: self.installation_tracker.get_total_installations(plugin_id),
            active_installations: self.installation_tracker.get_active_installations(plugin_id),
            uninstallations: self.installation_tracker.get_uninstallations(plugin_id),
            total_downloads: self.download_stats.get_total_downloads(plugin_id),
            avg_session_duration: self.usage_monitor.get_avg_session_duration(plugin_id),
            daily_active_users: self.usage_monitor.get_daily_active_users(plugin_id),
            monthly_active_users: self.usage_monitor.get_monthly_active_users(plugin_id),
            error_count: self.error_tracker.get_error_count(plugin_id),
            error_rate: self.error_tracker.get_error_rate(plugin_id),
            avg_metrics: self.performance_metrics.get_avg_metrics(plugin_id),
        }
    }

    pub fn get_installation_stats(&self, plugin_id: &str) -> InstallationStats {
        self.installation_tracker.get_stats(plugin_id)
    }

    pub fn get_usage_stats(&self, plugin_id: &str) -> UsageStats {
        self.usage_monitor.get_stats(plugin_id)
    }

    pub fn get_performance_stats(&self, plugin_id: &str) -> PerformanceStats {
        self.performance_metrics.get_stats(plugin_id)
    }

    pub fn get_error_stats(&self, plugin_id: &str) -> ErrorStats {
        self.error_tracker.get_stats(plugin_id)
    }

    pub fn get_download_stats(&self, plugin_id: &str) -> DownloadStatsView {
        self.download_stats.get_stats(plugin_id)
    }

    pub fn export(&self, plugin_id: &str, format: ExportFormat) -> anyhow::Result<String> {
        match format {
            ExportFormat::JSON => self.export_json(plugin_id),
            ExportFormat::CSV => self.export_csv(plugin_id),
            ExportFormat::HTML => self.export_html(plugin_id),
        }
    }

    fn export_json(&self, plugin_id: &str) -> anyhow::Result<String> {
        let summary = self.get_summary(plugin_id);
        Ok(serde_json::to_string_pretty(&summary)?)
    }

    fn export_csv(&self, plugin_id: &str) -> anyhow::Result<String> {
        let summary = self.get_summary(plugin_id);
        Ok(format!("metric,value\nplugin_id,{}\ntotal_installations,{}\nactive_installations,{}\nuninstallations,{}\ntotal_downloads,{}\navg_session_duration,{:.2}\ndaily_active_users,{}\nmonthly_active_users,{}\nerror_count,{}\nerror_rate,{:.4}\n",
            summary.plugin_id, summary.total_installations, summary.active_installations,
            summary.uninstallations, summary.total_downloads, summary.avg_session_duration,
            summary.daily_active_users, summary.monthly_active_users, summary.error_count, summary.error_rate))
    }

    fn export_html(&self, plugin_id: &str) -> anyhow::Result<String> {
        let summary = self.get_summary(plugin_id);
        Ok(format!(r#"<!DOCTYPE html><html><head><title>Plugin Analytics - {}</title></head><body><h1>{}</h1><p>Total Installations: {}</p><p>Active Installations: {}</p><p>Total Downloads: {}</p></body></html>"#,
            plugin_id, plugin_id, summary.total_installations, summary.active_installations, summary.total_downloads))
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        let cutoff = Utc::now() - Duration::days(self.config.retention_days as i64);
        for (_, history) in &mut self.installation_tracker.installation_history {
            history.retain(|e| e.timestamp > cutoff);
        }
        Ok(())
    }
}

impl InstallationTracker {
    fn new() -> Self {
        Self { total_installations: HashMap::new(), installation_history: HashMap::new(), active_installations: HashMap::new(), uninstallations: HashMap::new() }
    }

    fn track_installation(&mut self, plugin_id: &str, event: InstallationEvent) {
        *self.total_installations.entry(plugin_id.to_string()).or_insert(0) += 1;
        self.installation_history.entry(plugin_id.to_string()).or_insert_with(Vec::new).push(event);
        let thirty_days_ago = Utc::now() - Duration::days(30);
        let active = self.installation_history.get(plugin_id).map(|h| h.iter().filter(|e| e.timestamp > thirty_days_ago).count() as u64).unwrap_or(0);
        self.active_installations.insert(plugin_id.to_string(), active);
    }

    fn track_uninstallation(&mut self, plugin_id: &str) {
        *self.uninstallations.entry(plugin_id.to_string()).or_insert(0) += 1;
    }

    fn get_total_installations(&self, plugin_id: &str) -> u64 { *self.total_installations.get(plugin_id).unwrap_or(&0) }
    fn get_active_installations(&self, plugin_id: &str) -> u64 { *self.active_installations.get(plugin_id).unwrap_or(&0) }
    fn get_uninstallations(&self, plugin_id: &str) -> u64 { *self.uninstallations.get(plugin_id).unwrap_or(&0) }

    fn get_stats(&self, plugin_id: &str) -> InstallationStats {
        InstallationStats { total: self.get_total_installations(plugin_id), active: self.get_active_installations(plugin_id), uninstallations: self.get_uninstallations(plugin_id), by_platform: HashMap::new(), by_source: HashMap::new() }
    }
}

impl UsageMonitor {
    fn new() -> Self {
        Self { usage_sessions: HashMap::new(), avg_session_duration: HashMap::new(), feature_usage: HashMap::new(), daily_active_users: HashMap::new(), monthly_active_users: HashMap::new() }
    }

    fn start_session(&mut self, plugin_id: &str) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        self.usage_sessions.entry(plugin_id.to_string()).or_insert_with(Vec::new).push(UsageSession {
            start_time: Utc::now(), end_time: None, duration_seconds: 0, features_used: Vec::new(), actions_count: 0, errors_count: 0,
        });
        *self.daily_active_users.entry(plugin_id.to_string()).or_insert(0) += 1;
        session_id
    }

    fn end_session(&mut self, plugin_id: &str, _session_id: &str, features_used: Vec<String>, actions_count: u64, errors_count: u64) -> anyhow::Result<()> {
        if let Some(sessions) = self.usage_sessions.get_mut(plugin_id) {
            if let Some(session) = sessions.last_mut() {
                session.end_time = Some(Utc::now());
                session.duration_seconds = (Utc::now() - session.start_time).num_seconds() as u64;
                session.features_used = features_used;
                session.actions_count = actions_count;
                session.errors_count = errors_count;
                let total_duration: u64 = sessions.iter().map(|s| s.duration_seconds).sum();
                self.avg_session_duration.insert(plugin_id.to_string(), total_duration as f64 / sessions.len() as f64);
            }
        }
        Ok(())
    }

    fn get_avg_session_duration(&self, plugin_id: &str) -> f64 { *self.avg_session_duration.get(plugin_id).unwrap_or(&0.0) }
    fn get_daily_active_users(&self, plugin_id: &str) -> u64 { *self.daily_active_users.get(plugin_id).unwrap_or(&0) }
    fn get_monthly_active_users(&self, plugin_id: &str) -> u64 { *self.monthly_active_users.get(plugin_id).unwrap_or(&0) }

    fn get_stats(&self, plugin_id: &str) -> UsageStats {
        UsageStats { avg_session_duration: self.get_avg_session_duration(plugin_id), daily_active_users: self.get_daily_active_users(plugin_id), monthly_active_users: self.get_monthly_active_users(plugin_id), total_sessions: self.usage_sessions.get(plugin_id).map(|s| s.len() as u64).unwrap_or(0), feature_usage: HashMap::new() }
    }
}

impl PerformanceMetricsCollector {
    fn new() -> Self { Self { performance_samples: HashMap::new(), avg_metrics: HashMap::new(), performance_alerts: HashMap::new() } }

    fn record_sample(&mut self, plugin_id: &str, sample: PerformanceSample) -> anyhow::Result<()> {
        self.performance_samples.entry(plugin_id.to_string()).or_insert_with(Vec::new).push(sample.clone());
        if let Some(samples) = self.performance_samples.get(plugin_id) {
            let count = samples.len();
            let sum_memory: f64 = samples.iter().map(|s| s.memory_mb).sum();
            let sum_cpu: f64 = samples.iter().map(|s| s.cpu_percent).sum();
            let sum_response: f64 = samples.iter().map(|s| s.response_time_ms).sum();
            self.avg_metrics.insert(plugin_id.to_string(), AverageMetrics {
                avg_memory_mb: sum_memory / count as f64,
                avg_cpu_percent: sum_cpu / count as f64,
                avg_response_time_ms: sum_response / count as f64,
                ..Default::default()
            });
        }
        Ok(())
    }

    fn get_avg_metrics(&self, plugin_id: &str) -> AverageMetrics { self.avg_metrics.get(plugin_id).cloned().unwrap_or_default() }

    fn get_stats(&self, plugin_id: &str) -> PerformanceStats {
        PerformanceStats { avg_metrics: self.get_avg_metrics(plugin_id), sample_count: self.performance_samples.get(plugin_id).map(|s| s.len() as u64).unwrap_or(0), alert_count: 0, alerts: Vec::new() }
    }
}

impl ErrorTracker {
    fn new() -> Self { Self { error_counts: HashMap::new(), error_details: HashMap::new(), error_rate_history: HashMap::new(), top_errors: HashMap::new() } }

    fn track_error(&mut self, plugin_id: &str, error: ErrorEvent) {
        *self.error_counts.entry(plugin_id.to_string()).or_insert(0) += 1;
        self.error_details.entry(plugin_id.to_string()).or_insert_with(Vec::new).push(error);
    }

    fn get_error_count(&self, plugin_id: &str) -> u64 { *self.error_counts.get(plugin_id).unwrap_or(&0) }
    fn get_error_rate(&self, _plugin_id: &str) -> f64 { 0.0 }

    fn get_stats(&self, plugin_id: &str) -> ErrorStats {
        ErrorStats { total_errors: self.get_error_count(plugin_id), error_rate: 0.0, top_errors: Vec::new(), by_impact: HashMap::new() }
    }
}

impl DownloadStatistics {
    fn new() -> Self { Self { total_downloads: HashMap::new(), download_history: HashMap::new(), version_downloads: HashMap::new(), platform_downloads: HashMap::new(), download_trends: HashMap::new() } }

    fn track_download(&mut self, plugin_id: &str, event: DownloadEvent) {
        *self.total_downloads.entry(plugin_id.to_string()).or_insert(0) += 1;
        self.download_history.entry(plugin_id.to_string()).or_insert_with(Vec::new).push(event);
    }

    fn get_total_downloads(&self, plugin_id: &str) -> u64 { *self.total_downloads.get(plugin_id).unwrap_or(&0) }

    fn get_stats(&self, plugin_id: &str) -> DownloadStatsView {
        DownloadStatsView { total: self.get_total_downloads(plugin_id), by_version: HashMap::new(), by_platform: HashMap::new(), trend: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAnalyticsSummary {
    pub plugin_id: String,
    pub total_installations: u64,
    pub active_installations: u64,
    pub uninstallations: u64,
    pub total_downloads: u64,
    pub avg_session_duration: f64,
    pub daily_active_users: u64,
    pub monthly_active_users: u64,
    pub error_count: u64,
    pub error_rate: f64,
    pub avg_metrics: AverageMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationStats { pub total: u64, pub active: u64, pub uninstallations: u64, pub by_platform: HashMap<Platform, u64>, pub by_source: HashMap<InstallationSource, u64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats { pub avg_session_duration: f64, pub daily_active_users: u64, pub monthly_active_users: u64, pub total_sessions: u64, pub feature_usage: HashMap<String, u64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats { pub avg_metrics: AverageMetrics, pub sample_count: u64, pub alert_count: u64, pub alerts: Vec<PerformanceAlert> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStats { pub total_errors: u64, pub error_rate: f64, pub top_errors: Vec<TopError>, pub by_impact: HashMap<UserImpact, u64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatsView { pub total: u64, pub by_version: HashMap<String, u64>, pub by_platform: HashMap<Platform, u64>, pub trend: Option<DownloadTrend> }
