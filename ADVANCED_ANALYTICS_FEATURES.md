# Advanced Analytics & Telemetry Features

## Overview

The Advanced Analytics & Telemetry system provides comprehensive capabilities for tracking usage, reporting crashes, monitoring performance, collecting user feedback, and running A/B tests. This system enables data-driven decision making and continuous improvement of the Vantis Media Player.

## Features

### 1. Usage Analytics

The usage analytics system tracks user behavior and application usage patterns.

#### Features

- Event tracking with categories
- User session management
- Usage statistics
- User behavior analysis
- Sampling support

#### Usage

```rust
use vant_is_advanced_analytics::{AdvancedAnalyticsEngine, AnalyticsConfig, UsageEvent, EventCategory};

let config = AnalyticsConfig::default();
let engine = AdvancedAnalyticsEngine::new(config).await?;

// Start a user session
let session_id = engine.start_session("user123").await?;

// Track events
let event = UsageEvent {
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
};

engine.track_event(event).await?;

// End session
engine.end_session(&session_id).await?;

// Get usage statistics
let stats = engine.get_usage_stats().await;
println!("Total sessions: {}", stats.total_sessions);
println!("Total events: {}", stats.total_events);
```

#### Event Categories

- **Interaction**: User interactions (clicks, scrolls, etc.)
- **Playback**: Media playback events
- **System**: System events
- **Error**: Error events
- **Custom**: Custom events

---

### 2. Crash Reporting

The crash reporting system automatically detects and reports application crashes.

#### Features

- Automatic crash detection
- Crash report collection
- Sentry integration
- Crash severity classification
- Crash statistics

#### Usage

```rust
use vant_is_advanced_analytics::{AdvancedAnalyticsEngine, CrashReport, CrashSeverity};

let config = AnalyticsConfig {
    sentry_dsn: Some("https://your-sentry-dsn".to_string()),
    ..Default::default()
};

let engine = AdvancedAnalyticsEngine::new(config).await?;

// Report a crash
let report = CrashReport {
    report_id: uuid::Uuid::new_v4().to_string(),
    timestamp: chrono::Utc::now(),
    user_id: Some("user123".to_string()),
    session_id: None,
    error_message: "Segmentation fault".to_string(),
    error_type: "Segfault".to_string(),
    stack_trace: "stack trace here".to_string(),
    severity: CrashSeverity::Fatal,
    app_version: "1.0.0".to_string(),
    os: "linux".to_string(),
    arch: "x86_64".to_string(),
    context: HashMap::new(),
};

engine.report_crash(report).await?;

// Or report an error directly
let error = anyhow::anyhow!("Something went wrong");
engine.report_crash(&error, CrashSeverity::Error).await?;
```

#### Crash Severity Levels

- **Fatal**: Application crash
- **Error**: Error condition
- **Warning**: Warning condition
- **Info**: Informational

---

### 3. Performance Monitoring

The performance monitoring system tracks application performance metrics.

#### Features

- Metric collection (Counter, Gauge, Histogram, Summary)
- Prometheus integration
- Real-time monitoring
- Performance alerts
- Performance dashboards

#### Usage

```rust
use vant_is_advanced_analytics::{AdvancedAnalyticsEngine, PerformanceMetric, MetricType};

let config = AnalyticsConfig::default();
let engine = AdvancedAnalyticsEngine::new(config).await?;

// Record a counter metric
engine.record_counter("requests_total", 1.0, HashMap::new()).await?;

// Record a gauge metric
engine.record_gauge("memory_usage", 1024.0, HashMap::new()).await?;

// Record a histogram metric
engine.record_histogram("request_duration", 0.5, HashMap::new()).await?;

// Record request duration
engine.record_request_duration("/api/video", 0.123).await?;

// Increment request counter
engine.increment_request_counter("/api/video", 200).await?;

// Update system metrics
engine.update_active_connections(10).await?;
engine.update_memory_usage(1024 * 1024 * 512).await?;
engine.update_cpu_usage(45.5).await?;

// Get performance metrics
let metrics = engine.get_performance_metrics().await;
for metric in metrics {
    println!("{}: {}", metric.name, metric.value);
}

// Get Prometheus metrics for export
let prometheus_metrics = engine.get_prometheus_metrics().await?;
println!("{}", prometheus_metrics);
```

#### Metric Types

- **Counter**: Monotonically increasing counter
- **Gauge**: Value that can go up or down
- **Histogram**: Distribution of values
- **Summary**: Summary statistics

---

### 4. User Feedback System

The user feedback system collects and manages user feedback.

#### Features

- Feedback collection
- Feedback categorization
- Rating system
- Screenshot support
- System information collection

#### Usage

```rust
use vant_is_advanced_analytics::{AdvancedAnalyticsEngine, FeedbackCategory};

let config = AnalyticsConfig::default();
let engine = AdvancedAnalyticsEngine::new(config).await?;

// Create a bug report
let feedback_id = engine.collect_feedback(
    Feedback {
        feedback_id: uuid::Uuid::new_v4().to_string(),
        user_id: Some("user123".to_string()),
        session_id: None,
        feedback_type: FeedbackType::BugReport,
        category: FeedbackCategory::UI,
        rating: None,
        message: "Button not working".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        screenshot: None,
        system_info: SystemInfo::default(),
    }
).await?;

// Create a rating
let rating_id = engine.collect_feedback(
    Feedback {
        feedback_id: uuid::Uuid::new_v4().to_string(),
        user_id: Some("user123".to_string()),
        session_id: None,
        feedback_type: FeedbackType::Rating,
        category: FeedbackCategory::Other,
        rating: Some(5),
        message: "Great app!".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        screenshot: None,
        system_info: SystemInfo::default(),
    }
).await?;

// Get feedback
let feedback = engine.get_feedback().await;
for item in feedback {
    println!("{}: {}", item.feedback_type, item.message);
}

// Get feedback statistics
let stats = engine.get_feedback_stats().await;
println!("Average rating: {}", stats.average_rating);
```

#### Feedback Types

- **BugReport**: Bug report
- **FeatureRequest**: Feature request
- **General**: General feedback
- **Rating**: Rating
- **Support**: Support request

#### Feedback Categories

- **UI**: User interface
- **Performance**: Performance
- **Audio**: Audio
- **Video**: Video
- **Subtitles**: Subtitles
- **Plugins**: Plugins
- **Other**: Other

---

### 5. A/B Testing Framework

The A/B testing framework enables controlled experiments to test new features.

#### Features

- Experiment management
- Variant assignment
- Conversion tracking
- Statistical analysis
- Traffic allocation

#### Usage

```rust
use vant_is_advanced_analytics::{AdvancedAnalyticsEngine, Experiment, Variant, ExperimentStatus};

let config = AnalyticsConfig::default();
let engine = AdvancedAnalyticsEngine::new(config).await?;

// Create an experiment
let experiment = Experiment {
    experiment_id: "new_ui_test".to_string(),
    name: "New UI Test".to_string(),
    description: "Test new UI design".to_string(),
    variants: vec![
        Variant {
            variant_id: "control".to_string(),
            name: "Control".to_string(),
            description: "Current UI".to_string(),
            traffic_allocation: 0.5,
            config: HashMap::new(),
        },
        Variant {
            variant_id: "treatment".to_string(),
            name: "Treatment".to_string(),
            description: "New UI".to_string(),
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

// Get variant for user
let variant = engine.get_variant("new_ui_test", "user123").await?;
if let Some(variant) = variant {
    println!("User assigned to variant: {}", variant.name);
    // Apply variant configuration
}

// Track conversion
engine.track_conversion("new_ui_test", "user123", "treatment").await?;

// Get experiment results
let results = engine.get_experiment_results("new_ui_test").await?;
if let Some(results) = results {
    println!("Total participants: {}", results.total_participants);
    println!("Conversion rate: {:.2}%", results.conversion_rate * 100.0);
    
    for (variant_id, variant_result) in results.variants {
        println!("  {}: {:.2}% conversion rate", 
            variant_id, variant_result.conversion_rate * 100.0);
    }
}
```

#### Experiment Status

- **Draft**: Experiment is being created
- **Running**: Experiment is active
- **Paused**: Experiment is paused
- **Completed**: Experiment is completed
- **Cancelled**: Experiment is cancelled

---

## Configuration

### Analytics Configuration

```rust
use vant_is_advanced_analytics::AnalyticsConfig;

let config = AnalyticsConfig {
    enabled: true,
    crash_reporting: true,
    performance_monitoring: true,
    user_feedback: true,
    ab_testing: true,
    analytics_endpoint: Some("https://analytics.example.com".to_string()),
    sentry_dsn: Some("https://your-sentry-dsn".to_string()),
    sample_rate: 1.0,
    user_id: Some("user123".to_string()),
    session_id: Some("session456".to_string()),
};
```

### Configuration Options

- **enabled**: Enable/disable all analytics
- **crash_reporting**: Enable crash reporting
- **performance_monitoring**: Enable performance monitoring
- **user_feedback**: Enable user feedback collection
- **ab_testing**: Enable A/B testing
- **analytics_endpoint**: Analytics endpoint URL
- **sentry_dsn**: Sentry DSN for crash reporting
- **sample_rate**: Sampling rate (0.0 to 1.0)
- **user_id**: User ID
- **session_id**: Session ID

---

## Privacy

### Data Anonymization

The analytics system includes privacy features to protect user data:

```rust
use vant_is_advanced_analytics::utils;

// Anonymize user ID
let anonymized = utils::anonymize_user_id("user123@example.com");
// Output: user_1a2b3c4d

// Hash email
let hashed = utils::hash_email("user@example.com");
// Output: 5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8
```

### Sampling

Reduce data collection with sampling:

```rust
let config = AnalyticsConfig {
    sample_rate: 0.1, // Only collect 10% of data
    ..Default::default()
};
```

---

## Best Practices

### 1. Event Tracking

- Use descriptive event names
- Include relevant properties
- Categorize events appropriately
- Apply sampling for high-volume events

### 2. Crash Reporting

- Report all errors, not just crashes
- Include context information
- Use appropriate severity levels
- Monitor crash trends

### 3. Performance Monitoring

- Track key performance indicators
- Use appropriate metric types
- Set up alerts for anomalies
- Monitor trends over time

### 4. User Feedback

- Make feedback collection easy
- Categorize feedback properly
- Respond to feedback promptly
- Track feedback trends

### 5. A/B Testing

- Start with small traffic allocation
- Monitor results closely
- Use statistical significance
- Document experiment results

---

## Troubleshooting

### Analytics Not Working

**Problem**: Events are not being tracked

**Solution**:
1. Check if analytics is enabled
2. Verify configuration
3. Check network connectivity
4. Review logs for errors

### Crash Reports Not Sending

**Problem**: Crash reports are not being sent to Sentry

**Solution**:
1. Verify Sentry DSN is correct
2. Check network connectivity
3. Review Sentry dashboard
4. Check error logs

### Performance Metrics Not Updating

**Problem**: Performance metrics are not being recorded

**Solution**:
1. Check if performance monitoring is enabled
2. Verify metric names are correct
3. Check Prometheus configuration
4. Review metric collection code

### A/B Testing Not Assigning Variants

**Problem**: Users are not being assigned to variants

**Solution**:
1. Check if experiment is running
2. Verify traffic allocation
3. Check user assignment logic
4. Review experiment configuration

---

## API Reference

### AdvancedAnalyticsEngine

Main engine coordinating all analytics operations.

#### Methods

- `new(config: AnalyticsConfig) -> Result<Self>` - Create new engine
- `track_event(event: UsageEvent) -> Result<()>` - Track usage event
- `start_session(user_id: &str) -> Result<String>` - Start user session
- `end_session(session_id: &str) -> Result<()>` - End user session
- `report_crash(report: CrashReport) -> Result<()>` - Report crash
- `record_metric(metric: PerformanceMetric) -> Result<()>` - Record performance metric
- `collect_feedback(feedback: Feedback) -> Result<()>` - Collect user feedback
- `get_variant(experiment_id: &str, user_id: &str) -> Result<Option<Variant>>` - Get experiment variant
- `track_conversion(experiment_id: &str, user_id: &str, variant_id: &str) -> Result<()>` - Track conversion
- `get_usage_stats() -> UsageStats` - Get usage statistics
- `get_performance_metrics() -> Vec<PerformanceMetric>` - Get performance metrics
- `get_feedback() -> Vec<Feedback>` - Get user feedback
- `get_experiment_results(experiment_id: &str) -> Option<ExperimentResults>` - Get experiment results
- `set_enabled(enabled: bool)` - Enable/disable analytics
- `is_enabled() -> bool` - Check if analytics is enabled
- `flush() -> Result<()>` - Flush all pending data

---

## Examples

See `examples/advanced_analytics_example.rs` for complete examples of all features.