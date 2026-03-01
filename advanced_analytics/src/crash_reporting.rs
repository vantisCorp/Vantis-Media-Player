//! Crash Reporting
//! 
//! Provides crash reporting capabilities including:
//! - Automatic crash detection
//! - Crash report collection
//! - Sentry integration
//! - Crash severity classification
//! - Crash statistics

use anyhow::{Context, Result};
use backtrace::Backtrace;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use super::AnalyticsConfig;

/// Crash reporter
#[derive(Clone)]
pub struct CrashReporter {
    /// Configuration
    config: AnalyticsConfig,
    /// Crash reports
    reports: Arc<RwLock<Vec<CrashReport>>>,
    /// Crash statistics
    stats: Arc<RwLock<CrashStats>>,
}

/// Crash report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Report ID
    pub report_id: String,
    /// Crash timestamp
    pub timestamp: DateTime<Utc>,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Error message
    pub error_message: String,
    /// Error type
    pub error_type: String,
    /// Stack trace
    pub stack_trace: String,
    /// Crash severity
    pub severity: CrashSeverity,
    /// Application version
    pub app_version: String,
    /// Operating system
    pub os: String,
    /// Architecture
    pub arch: String,
    /// Additional context
    pub context: HashMap<String, String>,
}

/// Crash severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrashSeverity {
    /// Fatal crash
    Fatal,
    /// Error
    Error,
    /// Warning
    Warning,
    /// Info
    Info,
}

/// Crash statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashStats {
    /// Total crashes
    pub total_crashes: usize,
    /// Crashes by severity
    pub crashes_by_severity: HashMap<CrashSeverity, usize>,
    /// Crashes by error type
    pub crashes_by_error_type: HashMap<String, usize>,
    /// Most recent crash
    pub most_recent_crash: Option<DateTime<Utc>>,
}

impl CrashReporter {
    /// Create a new crash reporter
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing Crash Reporter");

        // Initialize Sentry if DSN is provided
        if let Some(dsn) = &config.sentry_dsn {
            let _guard = sentry::init((
                dsn.clone(),
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ));
            info!("Sentry initialized");
        }

        Ok(Self {
            config,
            reports: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(CrashStats {
                total_crashes: 0,
                crashes_by_severity: HashMap::new(),
                crashes_by_error_type: HashMap::new(),
                most_recent_crash: None,
            })),
        })
    }

    /// Report a crash
    pub async fn report(&self, report: CrashReport) -> Result<()> {
        error!("Crash reported: {} - {}", report.error_type, report.error_message);

        // Add to reports
        let mut reports = self.reports.write().await;
        reports.push(report.clone());

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_crashes += 1;
        *stats.crashes_by_severity.entry(report.severity).or_insert(0) += 1;
        *stats.crashes_by_error_type
            .entry(report.error_type.clone())
            .or_insert(0) += 1;
        stats.most_recent_crash = Some(report.timestamp);

        // Send to Sentry if configured
        if self.config.sentry_dsn.is_some() {
            self.send_to_sentry(&report).await;
        }

        info!("Crash report saved: {}", report.report_id);
        Ok(())
    }

    /// Report an error with automatic context collection
    pub async fn report_error(
        &self,
        error: &dyn std::error::Error,
        severity: CrashSeverity,
    ) -> Result<()> {
        let backtrace = Backtrace::new();
        
        let report = CrashReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user_id: self.config.user_id.clone(),
            session_id: self.config.session_id.clone(),
            error_message: error.to_string(),
            error_type: std::any::type_name::<dyn std::error::Error>().to_string(),
            stack_trace: format!("{:?}", backtrace),
            severity,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            context: HashMap::new(),
        };

        self.report(report).await
    }

    /// Send crash report to Sentry
    async fn send_to_sentry(&self, report: &CrashReport) {
        let event_id = sentry::capture_event(
            sentry::protocol::Event {
                message: Some(report.error_message.clone()),
                level: match report.severity {
                    CrashSeverity::Fatal => sentry::Level::Fatal,
                    CrashSeverity::Error => sentry::Level::Error,
                    CrashSeverity::Warning => sentry::Level::Warning,
                    CrashSeverity::Info => sentry::Level::Info,
                },
                exception: vec![sentry::protocol::Exception {
                    ty: report.error_type.clone(),
                    value: Some(report.error_message.clone()),
                    stacktrace: Some(sentry::protocol::Stacktrace::from_frames(
                        report
                            .stack_trace
                            .lines()
                            .map(|line| sentry::protocol::Frame {
                                function: Some(line.to_string()),
                                ..Default::default()
                            })
                            .collect(),
                    )),
                    ..Default::default()
                }],
                extra: report
                    .context
                    .iter()
                    .map(|(k, v)| (k.clone(), sentry::protocol::Value::String(v.clone())))
                    .collect(),
                ..Default::default()
            },
            None,
        );

        info!("Crash report sent to Sentry: {}", event_id);
    }

    /// Get crash reports
    pub async fn get_reports(&self) -> Vec<CrashReport> {
        self.reports.read().await.clone()
    }

    /// Get crash statistics
    pub async fn get_stats(&self) -> CrashStats {
        self.stats.read().await.clone()
    }

    /// Get crashes by severity
    pub async fn get_crashes_by_severity(&self, severity: CrashSeverity) -> Vec<CrashReport> {
        self.reports
            .read()
            .await
            .iter()
            .filter(|r| r.severity == severity)
            .cloned()
            .collect()
    }

    /// Get crashes by error type
    pub async fn get_crashes_by_error_type(&self, error_type: &str) -> Vec<CrashReport> {
        self.reports
            .read()
            .await
            .iter()
            .filter(|r| r.error_type == error_type)
            .cloned()
            .collect()
    }

    /// Clear old crash reports
    pub async fn clear_old_reports(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = Utc::now() - older_than;
        
        let mut reports = self.reports.write().await;
        let initial_count = reports.len();
        reports.retain(|r| r.timestamp > cutoff);
        let removed = initial_count - reports.len();

        info!("Cleared {} old crash reports", removed);
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crash_reporter_creation() {
        let config = AnalyticsConfig::default();
        let reporter = CrashReporter::new(config).await;
        assert!(reporter.is_ok());
    }

    #[tokio::test]
    async fn test_report_crash() {
        let reporter = CrashReporter::new(AnalyticsConfig::default()).await.unwrap();
        
        let report = CrashReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user_id: Some("user123".to_string()),
            session_id: None,
            error_message: "Test error".to_string(),
            error_type: "TestError".to_string(),
            stack_trace: "stack trace here".to_string(),
            severity: CrashSeverity::Error,
            app_version: "1.0.0".to_string(),
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            context: HashMap::new(),
        };
        
        let result = reporter.report(report).await;
        assert!(result.is_ok());
        
        let stats = reporter.get_stats().await;
        assert_eq!(stats.total_crashes, 1);
    }

    #[tokio::test]
    async fn test_report_error() {
        let reporter = CrashReporter::new(AnalyticsConfig::default()).await.unwrap();
        
        let error = anyhow::anyhow!("Test error");
        let result = reporter.report_error(&error, CrashSeverity::Error).await;
        assert!(result.is_ok());
        
        let stats = reporter.get_stats().await;
        assert_eq!(stats.total_crashes, 1);
    }
}