//! Advanced Build & Deployment Example
//! 
//! This example demonstrates all features of the Advanced Build & Deployment system:
//! - Cross-compilation for multiple platforms
//! - Release notes generation
//! - Deployment to different targets
//! - Rollback operations
//! - Health monitoring
//! - Metric recording
//! - Backup management

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use vant_is_advanced_build::{
    AdvancedBuildEngine, BuildConfig, BuildType, Platform,
    DeploymentTarget, HealthMonitor, HealthStatus,
    Metric, MetricType, RollbackManager, BackupType,
    RollbackStatus, AlertSeverity,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Vantis Media Player - Advanced Build & Deployment Example ===\n");

    // Example 1: Cross-Compilation
    println!("1. Cross-Compilation Example");
    cross_compilation_example().await?;
    println!();

    // Example 2: Release Notes Generation
    println!("2. Release Notes Generation Example");
    release_notes_example().await?;
    println!();

    // Example 3: Deployment
    println!("3. Deployment Example");
    deployment_example().await?;
    println!();

    // Example 4: Rollback
    println!("4. Rollback Example");
    rollback_example().await?;
    println!();

    // Example 5: Health Monitoring
    println!("5. Health Monitoring Example");
    health_monitoring_example().await?;
    println!();

    // Example 6: Metrics
    println!("6. Metrics Example");
    metrics_example().await?;
    println!();

    // Example 7: Backup Management
    println!("7. Backup Management Example");
    backup_management_example().await?;
    println!();

    println!("=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Cross-Compilation
async fn cross_compilation_example() -> anyhow::Result<()> {
    println!("Creating build engine...");
    
    let config = BuildConfig {
        output_dir: PathBuf::from("./target/releases"),
        optimize: true,
        debug_symbols: false,
        jobs: Some(4),
        cargo_flags: vec![],
        env_vars: HashMap::new(),
    };

    let engine = AdvancedBuildEngine::new(config).await?;
    println!("Build engine created successfully\n");

    // Build for specific platform
    println!("Building for Linux x86_64...");
    let record = engine.build_platform(Platform::LinuxX86_64, BuildType::Release).await?;
    println!("Build status: {:?}", record.status);
    println!("Build duration: {:?}s", record.duration);
    if let Some(output_path) = &record.output_path {
        println!("Output path: {:?}", output_path);
    }
    println!();

    // Build for all platforms
    println!("Building for all platforms...");
    let records = engine.build_all_platforms(BuildType::Release).await?;
    println!("Built {} platforms:", records.len());
    for record in &records {
        println!("  - {:?}: {:?}", record.platform, record.status);
    }

    // Get build history
    println!("\nBuild history:");
    let history = engine.get_build_history().await;
    for record in history.iter().take(5) {
        println!("  - {} ({:?}): {:?}", record.version, record.platform, record.status);
    }

    Ok(())
}

/// Example 2: Release Notes Generation
async fn release_notes_example() -> anyhow::Result<()> {
    println!("Generating release notes for version 1.0.0...\n");

    let engine = AdvancedBuildEngine::new(BuildConfig::default()).await?;
    let notes = engine.generate_release_notes("1.0.0").await?;

    println!("{}", notes);

    Ok(())
}

/// Example 3: Deployment
async fn deployment_example() -> anyhow::Result<()> {
    println!("Creating deployment engine...");
    
    let engine = AdvancedBuildEngine::new(BuildConfig::default()).await?;
    println!("Deployment engine created successfully\n");

    // Deploy to Docker
    println!("Deploying to Docker...");
    let record = engine.deploy("1.0.0", DeploymentTarget::Docker).await?;
    println!("Deployment status: {:?}", record.status);
    println!("Deployment duration: {:?}s", record.duration);
    println!("Rollback available: {}", record.rollback_available);
    println!();

    // Deploy to staging
    println!("Deploying to staging...");
    let record = engine.deploy("1.0.0", DeploymentTarget::Staging).await?;
    println!("Deployment status: {:?}", record.status);
    println!();

    // Get deployment history
    println!("Deployment history:");
    let history = engine.get_deployment_history().await;
    for record in history.iter().take(5) {
        println!("  - {} to {:?}: {:?}", record.version, record.target, record.status);
    }

    Ok(())
}

/// Example 4: Rollback
async fn rollback_example() -> anyhow::Result<()> {
    println!("Creating rollback manager...");
    
    let mut manager = RollbackManager::new().await?;
    println!("Rollback manager created successfully\n");

    // Create backup
    println!("Creating backup for version 1.0.0...");
    let backup = manager.create_backup("1.0.0", BackupType::Full).await?;
    println!("Backup created: {}", backup.id);
    println!("Backup version: {}", backup.version);
    println!("Backup type: {:?}", backup.backup_type);
    println!();

    // Create rollback plan
    println!("Creating rollback plan...");
    let plan = manager.create_rollback_plan("deployment-123").await?;
    println!("Rollback plan created: {}", plan.id);
    println!("Previous version: {}", plan.previous_version);
    println!("Current version: {}", plan.current_version);
    println!("Rollback steps: {}", plan.steps.len());
    println!();

    // Execute rollback
    println!("Executing rollback...");
    let status = manager.execute_rollback(plan).await?;
    println!("Rollback status: {:?}", status);
    println!();

    // Restore backup
    println!("Restoring backup...");
    manager.restore_backup(&backup.id).await?;
    println!("Backup restored successfully\n");

    // Get rollback history
    println!("Rollback history:");
    let history = manager.get_history();
    for record in history.iter().take(5) {
        println!("  - {}: {:?}", record.id, record.status);
    }

    // Clean up old backups
    println!("\nCleaning up old backups (retention: 3)...");
    let removed = manager.cleanup_old_backups(3).await?;
    println!("Removed {} old backups", removed);

    Ok(())
}

/// Example 5: Health Monitoring
async fn health_monitoring_example() -> anyhow::Result<()> {
    println!("Creating health monitor...");
    
    let monitor = HealthMonitor::new().await?;
    println!("Health monitor created successfully\n");

    // Get initial health status
    println!("Initial health status: {:?}", monitor.get_status().await);
    println!("Uptime: {:?}", monitor.get_uptime());
    println!();

    // Update health status
    println!("Updating health status to Degraded...");
    monitor.update_status(HealthStatus::Degraded).await;
    println!("Current health status: {:?}", monitor.get_status().await);
    println!();

    // Record health checks
    println!("Recording health checks...");
    use vant_is_advanced_build::HealthCheckResult;
    
    let checks = vec![
        HealthCheckResult {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: "Database connection OK".to_string(),
            duration_ms: 50,
            timestamp: chrono::Utc::now(),
        },
        HealthCheckResult {
            name: "cache".to_string(),
            status: HealthStatus::Healthy,
            message: "Cache service OK".to_string(),
            duration_ms: 10,
            timestamp: chrono::Utc::now(),
        },
        HealthCheckResult {
            name: "api".to_string(),
            status: HealthStatus::Degraded,
            message: "API response time high".to_string(),
            duration_ms: 500,
            timestamp: chrono::Utc::now(),
        },
    ];

    for check in checks {
        monitor.record_health_check(check).await;
    }

    // Get health checks
    println!("\nHealth checks:");
    let checks = monitor.get_health_checks().await;
    for (name, check) in checks {
        println!("  - {}: {:?} ({}ms)", name, check.status, check.duration_ms);
    }

    // Get system metrics
    println!("\nSystem metrics:");
    let metrics = monitor.get_system_metrics().await;
    println!("  - CPU usage: {:.1}%", metrics.cpu_usage);
    println!("  - Memory usage: {:.1}%", metrics.memory_usage);
    println!("  - Disk usage: {:.1}%", metrics.disk_usage);
    println!("  - Network in: {:.2} MB/s", metrics.network_in / 1024.0 / 1024.0);
    println!("  - Network out: {:.2} MB/s", metrics.network_out / 1024.0 / 1024.0);
    println!("  - Uptime: {}s", metrics.uptime);

    Ok(())
}

/// Example 6: Metrics
async fn metrics_example() -> anyhow::Result<()> {
    println!("Creating health monitor for metrics...");
    
    let monitor = HealthMonitor::new().await?;
    println!("Health monitor created successfully\n");

    // Record counter metrics
    println!("Recording counter metrics...");
    monitor.increment_counter("requests_total", 1.0, HashMap::new()).await;
    monitor.increment_counter("requests_total", 1.0, HashMap::new()).await;
    monitor.increment_counter("requests_total", 1.0, HashMap::new()).await;
    println!("Counter recorded: requests_total = 3\n");

    // Record gauge metrics
    println!("Recording gauge metrics...");
    monitor.set_gauge("memory_usage", 1024.0, HashMap::new()).await;
    monitor.set_gauge("cpu_usage", 45.5, HashMap::new()).await;
    monitor.set_gauge("active_connections", 10, HashMap::new()).await;
    println!("Gauges recorded\n");

    // Record custom metrics
    println!("Recording custom metrics...");
    let metric = Metric {
        name: "custom_metric".to_string(),
        metric_type: MetricType::Gauge,
        value: 42.0,
        labels: {
            let mut labels = HashMap::new();
            labels.insert("environment".to_string(), "production".to_string());
            labels
        },
        timestamp: chrono::Utc::now(),
    };
    monitor.record_metric(metric).await;
    println!("Custom metric recorded\n");

    // Get all metrics
    println!("All metrics:");
    let metrics = monitor.get_metrics().await;
    for metric in metrics {
        println!("  - {} ({:?}): {}", metric.name, metric.metric_type, metric.value);
    }

    // Get specific metric
    println!("\nGetting specific metric:");
    if let Some(metric) = monitor.get_metric("requests_total").await {
        println!("  - requests_total: {}", metric.value);
    }

    // Create alerts
    println!("\nCreating alerts...");
    monitor.create_alert("high_memory", AlertSeverity::Warning, "Memory usage above 90%").await;
    monitor.create_alert("api_error", AlertSeverity::Error, "API returning errors").await;
    println!("Alerts created\n");

    // Get alert history
    println!("Alert history:");
    let alerts = monitor.get_alert_history().await;
    for alert in alerts {
        println!("  - {} ({:?}): {}", alert.name, alert.severity, alert.message);
    }

    Ok(())
}

/// Example 7: Backup Management
async fn backup_management_example() -> anyhow::Result<()> {
    println!("Creating rollback manager for backup management...");
    
    let mut manager = RollbackManager::new().await?;
    println!("Rollback manager created successfully\n");

    // Create multiple backups
    println!("Creating backups...");
    let backup_types = vec![
        BackupType::Full,
        BackupType::Incremental,
        BackupType::Config,
        BackupType::Database,
    ];

    for (i, backup_type) in backup_types.iter().enumerate() {
        let backup = manager.create_backup(&format!("1.0.{}", i), *backup_type).await?;
        println!("  - Created {} backup: {} ({:?})", backup_type, backup.id, backup.version);
    }
    println!();

    // List all backups
    println!("All backups:");
    let backups = manager.get_backups();
    for (id, backup) in backups {
        println!("  - {} ({}): {:?} - {} bytes", 
            id, backup.version, backup.backup_type, backup.size);
    }
    println!();

    // Restore a backup
    if let Some((id, backup)) = backups.iter().next() {
        println!("Restoring backup: {} ({})", id, backup.version);
        manager.restore_backup(id).await?;
        println!("Backup restored successfully\n");
    }

    // Clean up old backups
    println!("Cleaning up old backups (retention: 2)...");
    let removed = manager.cleanup_old_backups(2).await?;
    println!("Removed {} old backups\n");

    // List remaining backups
    println!("Remaining backups:");
    let backups = manager.get_backups();
    for (id, backup) in backups {
        println!("  - {} ({}): {:?}", id, backup.version, backup.backup_type);
    }

    Ok(())
}