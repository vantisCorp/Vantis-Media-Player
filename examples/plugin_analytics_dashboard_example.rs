//! Plugin Analytics Dashboard Example
//!
//! Demonstrates the complete analytics dashboard functionality for plugins.

use vanis_plugins::{
    PluginAnalyticsDashboard, AnalyticsConfig, ExportFormat,
    InstallationEvent, DownloadEvent, PerformanceSample, ErrorEvent, UserImpact,
    Platform, InstallationSource,
};
use chrono::Utc;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("📊 Plugin Analytics Dashboard Example\n");
    
    // Create analytics dashboard with custom configuration
    let config = AnalyticsConfig {
        enabled: true,
        retention_days: 90,
        real_time_tracking: true,
        min_sample_size: 100,
        export_format: ExportFormat::JSON,
    };
    
    let mut dashboard = PluginAnalyticsDashboard::new(config);
    
    println!("✅ Analytics dashboard created");
    println!("   - Real-time tracking: Enabled");
    println!("   - Data retention: 90 days");
    println!("   - Minimum sample size: 100\n");
    
    // Example 1: Track installations
    println!("📦 Example 1: Tracking Installations");
    let plugin_id = "example-video-filter";
    
    let installations = vec![
        InstallationEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::Linux,
            country: Some("US".to_string()),
            source: InstallationSource::Marketplace,
        },
        InstallationEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::Windows,
            country: Some("GB".to_string()),
            source: InstallationSource::Marketplace,
        },
        InstallationEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::MacOS,
            country: Some("DE".to_string()),
            source: InstallationSource::Manual,
        },
    ];
    
    for (i, event) in installations.iter().enumerate() {
        dashboard.track_installation(plugin_id, event.clone())?;
        println!("   Installation {} tracked for {}", i + 1, plugin_id);
    }
    
    let install_stats = dashboard.get_installation_stats(plugin_id);
    println!("   Total installations: {}", install_stats.total);
    println!("   Active installations: {}", install_stats.active);
    
    // Example 2: Track downloads
    println!("\n⬇️  Example 2: Tracking Downloads");
    let downloads = vec![
        DownloadEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::Linux,
            country: Some("FR".to_string()),
            source: InstallationSource::Marketplace,
        },
        DownloadEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::Windows,
            country: Some("CA".to_string()),
            source: InstallationSource::Marketplace,
        },
    ];
    
    for (i, event) in downloads.iter().enumerate() {
        dashboard.track_download(plugin_id, event.clone())?;
        println!("   Download {} tracked", i + 1);
    }
    
    let download_stats = dashboard.get_download_stats(plugin_id);
    println!("   Total downloads: {}", download_stats.total);
    
    // Example 3: Track usage sessions
    println!("\n⏱️  Example 3: Tracking Usage Sessions");
    let session_id = dashboard.start_session(plugin_id);
    println!("   Session started: {}", session_id);
    
    // Simulate session activity
    thread::sleep(Duration::from_millis(100));
    
    dashboard.end_session(
        plugin_id,
        &session_id,
        vec!["video_filter".to_string(), "brightness".to_string()],
        25,
        1,
    )?;
    println!("   Session ended after 25 actions");
    
    let usage_stats = dashboard.get_usage_stats(plugin_id);
    println!("   Average session duration: {:.2} seconds", usage_stats.avg_session_duration);
    println!("   Total sessions: {}", usage_stats.total_sessions);
    println!("   Daily active users: {}", usage_stats.daily_active_users);
    
    // Example 4: Track performance metrics
    println!("\n📈 Example 4: Tracking Performance Metrics");
    let performance_samples = vec![
        PerformanceSample {
            timestamp: Utc::now(),
            memory_mb: 85.5,
            cpu_percent: 15.3,
            response_time_ms: 25.4,
            init_time_ms: 180.0,
        },
        PerformanceSample {
            timestamp: Utc::now(),
            memory_mb: 92.1,
            cpu_percent: 18.7,
            response_time_ms: 28.9,
            init_time_ms: 185.0,
        },
        PerformanceSample {
            timestamp: Utc::now(),
            memory_mb: 88.3,
            cpu_percent: 16.2,
            response_time_ms: 26.1,
            init_time_ms: 175.0,
        },
    ];
    
    for (i, sample) in performance_samples.iter().enumerate() {
        dashboard.record_performance(plugin_id, sample.clone())?;
        println!("   Performance sample {} recorded", i + 1);
    }
    
    let perf_stats = dashboard.get_performance_stats(plugin_id);
    println!("   Average memory: {:.2} MB", perf_stats.avg_metrics.avg_memory_mb);
    println!("   Average CPU: {:.2}%", perf_stats.avg_metrics.avg_cpu_percent);
    println!("   Average response time: {:.2} ms", perf_stats.avg_metrics.avg_response_time_ms);
    println!("   P95 response time: {:.2} ms", perf_stats.avg_metrics.p95_response_time_ms);
    println!("   Performance alerts: {}", perf_stats.alert_count);
    
    // Example 5: Track errors
    println!("\n❌ Example 5: Tracking Errors");
    let errors = vec![
        ErrorEvent {
            timestamp: Utc::now(),
            error_type: "TimeoutError".to_string(),
            message: "Request timeout after 30 seconds".to_string(),
            stack_trace: Some("   at Plugin.process()".to_string()),
            plugin_version: "1.0.0".to_string(),
            user_impact: UserImpact::Low,
        },
        ErrorEvent {
            timestamp: Utc::now(),
            error_type: "ParseError".to_string(),
            message: "Failed to parse video format".to_string(),
            stack_trace: Some("   at VideoDecoder.parse()".to_string()),
            plugin_version: "1.0.0".to_string(),
            user_impact: UserImpact::Medium,
        },
    ];
    
    for (i, error) in errors.iter().enumerate() {
        dashboard.track_error(plugin_id, error.clone())?;
        println!("   Error {} tracked: {}", i + 1, error.error_type);
    }
    
    let error_stats = dashboard.get_error_stats(plugin_id);
    println!("   Total errors: {}", error_stats.total_errors);
    println!("   Error rate: {:.4}/hr", error_stats.error_rate);
    
    if !error_stats.top_errors.is_empty() {
        println!("   Top error: {} ({} occurrences)", 
            error_stats.top_errors[0].error_type,
            error_stats.top_errors[0].count
        );
    }
    
    // Example 6: Get analytics summary
    println!("\n📋 Example 6: Analytics Summary");
    let summary = dashboard.get_summary(plugin_id);
    println!("   Plugin: {}", summary.plugin_id);
    println!("   Total installations: {}", summary.total_installations);
    println!("   Active installations: {}", summary.active_installations);
    println!("   Total downloads: {}", summary.total_downloads);
    println!("   Average session duration: {:.2}s", summary.avg_session_duration);
    println!("   Error count: {}", summary.error_count);
    
    // Example 7: Export analytics data
    println!("\n📤 Example 7: Exporting Analytics Data");
    
    // Export as JSON
    let json_export = dashboard.export(plugin_id, ExportFormat::JSON)?;
    println!("   JSON export (first 200 chars):");
    println!("   {}", &json_export.chars().take(200).collect::<String>());
    println!("...");
    
    // Export as CSV
    let csv_export = dashboard.export(plugin_id, ExportFormat::CSV)?;
    println!("\n   CSV export (first 200 chars):");
    println!("   {}", &csv_export.chars().take(200).collect::<String>());
    
    // Export as HTML
    let html_export = dashboard.export(plugin_id, ExportFormat::HTML)?;
    println!("\n   HTML export (first 200 chars):");
    println!("   {}", &html_export.chars().take(200).collect::<String>());
    
    // Example 8: Performance alerts
    println!("\n⚠️  Example 8: Performance Alerts");
    let alert_sample = PerformanceSample {
        timestamp: Utc::now(),
        memory_mb: 550.0, // Above threshold
        cpu_percent: 85.0, // Above threshold
        response_time_ms: 120.0, // Above threshold
        init_time_ms: 1100.0, // Above threshold
    };
    
    dashboard.record_performance(plugin_id, alert_sample)?;
    
    let perf_stats_with_alerts = dashboard.get_performance_stats(plugin_id);
    println!("   Total performance alerts: {}", perf_stats_with_alerts.alert_count);
    
    for (i, alert) in perf_stats_with_alerts.alerts.iter().enumerate().take(3) {
        println!("   Alert {}: {}", i + 1, alert.message);
    }
    
    // Example 9: Data cleanup
    println!("\n🧹 Example 9: Data Cleanup (Retention Policy)");
    let old_event = InstallationEvent {
        timestamp: Utc::now() - chrono::Duration::days(100), // Old data
        version: "0.9.0".to_string(),
        platform: Platform::Linux,
        country: Some("JP".to_string()),
        source: InstallationSource::Manual,
    };
    
    dashboard.track_installation(plugin_id, old_event)?;
    println!("   Old installation tracked");
    
    dashboard.cleanup()?;
    println!("   Old data cleaned up based on retention policy");
    
    let install_stats_after = dashboard.get_installation_stats(plugin_id);
    println!("   Installations after cleanup: {}", install_stats_after.total);
    
    // Example 10: Multiple plugins
    println!("\n🔌 Example 10: Tracking Multiple Plugins");
    let plugins = vec![
        "example-video-filter",
        "example-audio-enhancer",
        "example-subtitle-sync",
    ];
    
    for plugin in &plugins {
        let event = InstallationEvent {
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
            platform: Platform::Linux,
            country: Some("US".to_string()),
            source: InstallationSource::Marketplace,
        };
        
        dashboard.track_installation(plugin, event)?;
        println!("   Tracked installation for {}", plugin);
    }
    
    println!("\n📊 Multi-Plugin Summary:");
    for plugin in &plugins {
        let summary = dashboard.get_summary(plugin);
        println!("   {}: {} installations", plugin, summary.total_installations);
    }
    
    println!("\n✅ All examples completed successfully!");
    println!("\nThe plugin analytics dashboard provides comprehensive insights into:");
    println!("   • Installation and download tracking");
    println!("   • Usage patterns and session statistics");
    println!("   • Performance metrics and alerts");
    println!("   • Error tracking and analysis");
    println!("   • Multi-format data export (JSON, CSV, HTML)");
    println!("   • Automatic data cleanup based on retention policy");
    println!("   • Support for multiple plugins simultaneously");
    
    Ok(())
}