//! Advanced Analytics & Telemetry Example
//! 
//! This example demonstrates all features of the Advanced Analytics & Telemetry system:
//! - Usage analytics (events, sessions)
//! - Crash reporting
//! - Performance monitoring
//! - User feedback collection
//! - A/B testing
//! - Statistical analysis
//! - Data export

use std::collections::HashMap;
use std::time::Duration;
use vant_is_advanced_analytics::{
    AdvancedAnalyticsEngine, AnalyticsConfig, UsageEvent, EventCategory,
    CrashReport, CrashSeverity, PerformanceMetric, MetricType,
    Feedback, FeedbackType, FeedbackCategory, Experiment, Variant, ExperimentStatus,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Vantis Media Player - Advanced Analytics & Telemetry Example ===\n");

    // Example 1: Usage Analytics
    println!("1. Usage Analytics Example");
    usage_analytics_example().await?;
    println!();

    // Example 2: Crash Reporting
    println!("2. Crash Reporting Example");
    crash_reporting_example().await?;
    println!();

    // Example 3: Performance Monitoring
    println!("3. Performance Monitoring Example");
    performance_monitoring_example().await?;
    println!();

    // Example 4: User Feedback
    println!("4. User Feedback Example");
    user_feedback_example().await?;
    println!();

    // Example 5: A/B Testing
    println!("5. A/B Testing Example");
    ab_testing_example().await?;
    println!();

    // Example 6: Statistical Analysis
    println!("6. Statistical Analysis Example");
    statistical_analysis_example().await?;
    println!();

    // Example 7: Data Export
    println!("7. Data Export Example");
    data_export_example().await?;
    println!();

    println!("=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Usage Analytics
async fn usage_analytics_example() -> anyhow::Result<()> {
    println!("Creating analytics engine...");
    
    let config = AnalyticsConfig {
        enabled: true,
        sample_rate: 1.0,
        ..Default::default()
    };

    let engine = AdvancedAnalyticsEngine::new(config).await?;
    println!("Analytics engine created successfully\n");

    // Start a user session
    println!("Starting user session...");
    let session_id = engine.start_session("user123").await?;
    println!("Session started: {}", session_id);
    println!();

    // Track events
    println!("Tracking events...");
    let events = vec![
        UsageEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_name: "video_played".to_string(),
            user_id: Some("user123".to_string()),
            session_id: Some(session_id.clone()),
            timestamp: chrono::Utc::now(),
            properties: {
                let mut props = HashMap::new();
                props.insert("video_id".to_string(), serde_json::json!("video123"));
                props.insert("duration".to_string(), serde_json::json!(120));
                props
            },
            category: EventCategory::Playback,
        },
        UsageEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_name: "button_clicked".to_string(),
            user_id: Some("user123".to_string()),
            session_id: Some(session_id.clone()),
            timestamp: chrono::Utc::now(),
            properties: {
                let mut props = HashMap::new();
                props.insert("button_id".to_string(), serde_json::json!("play_button"));
                props
            },
            category: EventCategory::Interaction,
        },
    ];

    for event in events {
        engine.track_event(event).await?;
    }

    println!("Events tracked\n");

    // Get usage statistics
    println!("Usage statistics:");
    let stats = engine.get_usage_stats().await;
    println!("  Total sessions: {}", stats.total_sessions);
    println!("  Active sessions: {}", stats.active_sessions);
    println!("  Total events: {}", stats.total_events);
    println!("  Unique users: {}", stats.unique_users);
    println!("  Average session duration: {:.2}s", stats.avg_session_duration);

    // End session
    println!("\nEnding session...");
    engine.end_session(&session_id).await?;
    println!("Session ended");

    Ok(())
}

/// Example 2: Crash Reporting
async fn crash_reporting_example() -> anyhow::Result<()> {
    println!("Creating analytics engine with crash reporting...");
    
    let config = AnalyticsConfig {
        crash_reporting: true,
        ..Default::default()
    };

    let engine = AdvancedAnalyticsEngine::new(config).await?;
    println!("Analytics engine created successfully\n");

    // Report a crash
    println!("Reporting crash...");
    let report = CrashReport {
        report_id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now(),
        user_id: Some("user123".to_string()),
        session_id: None,
        error_message: "Segmentation fault at address 0x1234".to_string(),
        error_type: "Segfault".to_string(),
        stack_trace: "stack trace here".to_string(),
        severity: CrashSeverity::Fatal,
        app_version: "1.0.0".to_string(),
        os: "linux".to_string(),
        arch: "x86_64".to_string(),
        context: {
            let mut ctx = HashMap::new();
            ctx.insert("video_id".to_string(), "video123".to_string());
            ctx
        },
    };

    engine.report_crash(report).await?;
    println!("Crash reported\n");

    // Report an error
    println!("Reporting error...");
    let error = anyhow::anyhow!("Failed to load video: file not found");
    engine.report_crash(&error, CrashSeverity::Error).await?;
    println!("Error reported");

    Ok(())
}

/// Example 3: Performance Monitoring
async fn performance_monitoring_example() -> anyhow::Result<()> {
    println!("Creating analytics engine with performance monitoring...");
    
    let config = AnalyticsConfig {
        performance_monitoring: true,
        ..Default::default()
    };

    let engine = AdvancedAnalyticsEngine::new(config).await?;
    println!("Analytics engine created successfully\n");

    // Record counter metrics
    println!("Recording counter metrics...");
    engine.record_counter("requests_total", 1.0, HashMap::new()).await?;
    engine.record_counter("requests_total", 1.0, HashMap::new()).await?;
    engine.record_counter("requests_total", 1.0, HashMap::new()).await?;
    println!("Counter recorded: requests_total = 3\n");

    // Record gauge metrics
    println!("Recording gauge metrics...");
    engine.record_gauge("memory_usage", 1024.0, HashMap::new()).await?;
    engine.record_gauge("cpu_usage", 45.5, HashMap::new()).await?;
    engine.record_gauge("active_connections", 10, HashMap::new()).await?;
    println!("Gauges recorded\n");

    // Record histogram metrics
    println!("Recording histogram metrics...");
    for i in 1..=10 {
        engine.record_histogram("request_duration", i as f64 * 0.1, HashMap::new()).await?;
    }
    println!("Histogram recorded\n");

    // Record request duration
    println!("Recording request duration...");
    engine.record_request_duration("/api/video", 0.123).await?;
    engine.record_request_duration("/api/audio", 0.456).await?;
    println!("Request durations recorded\n");

    // Increment request counter
    println!("Incrementing request counter...");
    engine.increment_request_counter("/api/video", 200).await?;
    engine.increment_request_counter("/api/video", 200).await?;
    engine.increment_request_counter("/api/audio", 200).await?;
    println!("Request counters incremented\n");

    // Update system metrics
    println!("Updating system metrics...");
    engine.update_active_connections(15).await?;
    engine.update_memory_usage(1024 * 1024 * 512).await?;
    engine.update_cpu_usage(55.5).await?;
    println!("System metrics updated\n");

    // Get performance metrics
    println!("Performance metrics:");
    let metrics = engine.get_performance_metrics().await;
    for metric in metrics.iter().take(5) {
        println!("  {} ({:?}): {}", metric.name, metric.metric_type, metric.value);
    }

    // Get Prometheus metrics
    println!("\nPrometheus metrics:");
    let prometheus_metrics = engine.get_prometheus_metrics().await?;
    println!("{}", prometheus_metrics);

    Ok(())
}

/// Example 4: User Feedback
async fn user_feedback_example() -> anyhow::Result<()> {
    println!("Creating analytics engine with user feedback...");
    
    let config = AnalyticsConfig {
        user_feedback: true,
        ..Default::default()
    };

    let engine = AdvancedAnalyticsEngine::new(config).await?;
    println!("Analytics engine created successfully\n");

    // Create a bug report
    println!("Creating bug report...");
    let bug_report = Feedback {
        feedback_id: uuid::Uuid::new_v4().to_string(),
        user_id: Some("user123".to_string()),
        session_id: None,
        feedback_type: FeedbackType::BugReport,
        category: FeedbackCategory::UI,
        rating: None,
        message: "Play button not responding when clicked".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("page".to_string(), "player".to_string());
            meta
        },
        screenshot: None,
        system_info: vant_is_advanced_analytics::SystemInfo {
            app_version: "1.0.0".to_string(),
            os: "linux".to_string(),
            os_version: "5.15.0".to_string(),
            arch: "x86_64".to_string(),
            cpu_cores: 8,
            total_memory: 16 * 1024 * 1024 * 1024,
            available_memory: 8 * 1024 * 1024 * 1024,
        },
    };

    engine.collect_feedback(bug_report).await?;
    println!("Bug report created\n");

    // Create a feature request
    println!("Creating feature request...");
    let feature_request = Feedback {
        feedback_id: uuid::Uuid::new_v4().to_string(),
        user_id: Some("user123".to_string()),
        session_id: None,
        feedback_type: FeedbackType::FeatureRequest,
        category: FeedbackCategory::Video,
        rating: None,
        message: "Add support for 8K video playback".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        screenshot: None,
        system_info: vant_is_advanced_analytics::SystemInfo {
            app_version: "1.0.0".to_string(),
            os: "linux".to_string(),
            os_version: "5.15.0".to_string(),
            arch: "x86_64".to_string(),
            cpu_cores: 8,
            total_memory: 16 * 1024 * 1024 * 1024,
            available_memory: 8 * 1024 * 1024 * 1024,
        },
    };

    engine.collect_feedback(feature_request).await?;
    println!("Feature request created\n");

    // Create a rating
    println!("Creating rating...");
    let rating = Feedback {
        feedback_id: uuid::Uuid::new_v4().to_string(),
        user_id: Some("user123".to_string()),
        session_id: None,
        feedback_type: FeedbackType::Rating,
        category: FeedbackCategory::Other,
        rating: Some(5),
        message: "Great app! Love the new features.".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        screenshot: None,
        system_info: vant_is_advanced_analytics::SystemInfo {
            app_version: "1.0.0".to_string(),
            os: "linux".to_string(),
            os_version: "5.15.0".to_string(),
            arch: "x86_64".to_string(),
            cpu_cores: 8,
            total_memory: 16 * 1024 * 1024 * 1024,
            available_memory: 8 * 1024 * 1024 * 1024,
        },
    };

    engine.collect_feedback(rating).await?;
    println!("Rating created\n");

    // Get feedback
    println!("User feedback:");
    let feedback = engine.get_feedback().await;
    for item in feedback {
        println!("  {} ({:?}): {}", item.feedback_type, item.category, item.message);
    }

    Ok(())
}

/// Example 5: A/B Testing
async fn ab_testing_example() -> anyhow::Result<()> {
    println!("Creating analytics engine with A/B testing...");
    
    let config = AnalyticsConfig {
        ab_testing: true,
        ..Default::default()
    };

    let engine = AdvancedAnalyticsEngine::new(config).await?;
    println!("Analytics engine created successfully\n");

    // Create an experiment
    println!("Creating experiment...");
    let experiment = Experiment {
        experiment_id: "new_ui_test".to_string(),
        name: "New UI Test".to_string(),
        description: "Test new UI design with dark theme".to_string(),
        variants: vec![
            Variant {
                variant_id: "control".to_string(),
                name: "Control".to_string(),
                description: "Current UI with light theme".to_string(),
                traffic_allocation: 0.5,
                config: {
                    let mut config = HashMap::new();
                    config.insert("theme".to_string(), serde_json::json!("light"));
                    config
                },
            },
            Variant {
                variant_id: "treatment".to_string(),
                name: "Treatment".to_string(),
                description: "New UI with dark theme".to_string(),
                traffic_allocation: 0.5,
                config: {
                    let mut config = HashMap::new();
                    config.insert("theme".to_string(), serde_json::json!("dark"));
                    config
                },
            },
        ],
        status: ExperimentStatus::Running,
        start_time: chrono::Utc::now(),
        end_time: None,
        traffic_allocation: 1.0,
        target_criteria: None,
        metadata: HashMap::new(),
    };

    engine.create_experiment(experiment).await?;
    println!("Experiment created\n");

    // Get variants for multiple users
    println!("Assigning variants to users...");
    let users = vec!["user1", "user2", "user3", "user4", "user5"];
    
    for user in users {
        let variant = engine.get_variant("new_ui_test", user).await?;
        if let Some(variant) = variant {
            println!("  {} assigned to: {} ({})", user, variant.name, variant.variant_id);
        }
    }
    println!();

    // Track conversions
    println!("Tracking conversions...");
    engine.track_conversion("new_ui_test", "user1", "control").await?;
    engine.track_conversion("new_ui_test", "user2", "treatment").await?;
    engine.track_conversion("new_ui_test", "user3", "treatment").await?;
    println!("Conversions tracked\n");

    // Get experiment results
    println!("Experiment results:");
    let results = engine.get_experiment_results("new_ui_test").await;
    if let Some(results) = results {
        println!("  Total participants: {}", results.total_participants);
        println!("  Total conversions: {}", results.total_conversions);
        println!("  Conversion rate: {:.2}%", results.conversion_rate * 100.0);
        println!();
        
        for (variant_id, variant_result) in results.variants {
            println!("  Variant {}:", variant_id);
            println!("    Participants: {}", variant_result.participants);
            println!("    Conversions: {}", variant_result.conversions);
            println!("    Conversion rate: {:.2}%", variant_result.conversion_rate * 100.0);
        }
    }

    Ok(())
}

/// Example 6: Statistical Analysis
async fn statistical_analysis_example() -> anyhow::Result<()> {
    println!("Statistical Analysis Example\n");

    use vant_is_advanced_analytics::utils;

    // Calculate statistics
    println!("Calculating statistics...");
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = utils::calculate_statistics(&values);
    
    println!("  Count: {}", stats.count);
    println!("  Sum: {}", stats.sum);
    println!("  Mean: {:.2}", stats.mean);
    println!("  Median: {:.2}", stats.median);
    println!("  Min: {}", stats.min);
    println!("  Max: {}", stats.max);
    println!("  Std Dev: {:.2}", stats.std_dev);
    println!("  Variance: {:.2}", stats.variance);
    println!("  P25: {:.2}", stats.p25);
    println!("  P75: {:.2}", stats.p75);
    println!("  P90: {:.2}", stats.p90);
    println!("  P95: {:.2}", stats.p95);
    println!("  P99: {:.2}", stats.p99);
    println!();

    // Calculate moving average
    println!("Calculating moving average...");
    let ma = utils::calculate_moving_average(&values, 3);
    println!("  Moving average (window=3): {:?}", ma);
    println!();

    // Detect anomalies
    println!("Detecting anomalies...");
    let values_with_anomaly = vec![1.0, 2.0, 3.0, 100.0, 5.0, 6.0, 7.0];
    let anomalies = utils::detect_anomalies(&values_with_anomaly, 2.0);
    println!("  Anomalies detected at indices: {:?}", anomalies);
    println!();

    // Calculate statistical significance
    println!("Calculating statistical significance...");
    let significance = utils::calculate_statistical_significance(
        50,  // control conversions
        100, // control total
        60,  // treatment conversions
        100, // treatment total
    );
    
    println!("  Control rate: {:.2}%", significance.control_rate * 100.0);
    println!("  Treatment rate: {:.2}%", significance.treatment_rate * 100.0);
    println!("  Lift: {:.2}%", significance.lift);
    println!("  Z-score: {:.2}", significance.z_score);
    println!("  P-value: {:.4}", significance.p_value);
    println!("  Significant: {}", significance.is_significant);
    println!("  Confidence level: {}%", significance.confidence_level);

    Ok(())
}

/// Example 7: Data Export
async fn data_export_example() -> anyhow::Result<()> {
    println!("Data Export Example\n");

    use vant_is_advanced_analytics::utils;

    // Create sample data
    let data = vec![
        serde_json::json!({
            "id": 1,
            "name": "Item 1",
            "value": 100.0,
            "category": "A"
        }),
        serde_json::json!({
            "id": 2,
            "name": "Item 2",
            "value": 200.0,
            "category": "B"
        }),
        serde_json::json!({
            "id": 3,
            "name": "Item 3",
            "value": 300.0,
            "category": "A"
        }),
    ];

    // Export to CSV
    println!("Exporting to CSV...");
    let headers = vec!["id".to_string(), "name".to_string(), "value".to_string(), "category".to_string()];
    let csv = utils::export_to_csv(&data, &headers)?;
    println!("CSV output:");
    println!("{}", csv);
    println!();

    // Export to JSON
    println!("Exporting to JSON...");
    let json = utils::export_to_json(&data)?;
    println!("JSON output:");
    println!("{}", json);

    Ok(())
}