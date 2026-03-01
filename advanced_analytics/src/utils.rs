//! Utility Functions for Analytics & Telemetry
//! 
//! Provides helper functions for:
//! - Data aggregation
//! - Statistical analysis
//! - Data export
//! - Privacy utilities

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Aggregate metrics by time period
pub fn aggregate_metrics_by_period<T>(
    metrics: &[T],
    period: TimePeriod,
    get_timestamp: &dyn Fn(&T) -> DateTime<Utc>,
) -> HashMap<String, Vec<T>>
where
    T: Clone,
{
    let mut aggregated: HashMap<String, Vec<T>> = HashMap::new();

    for metric in metrics {
        let timestamp = get_timestamp(metric);
        let key = format_timestamp_by_period(timestamp, period);
        
        aggregated.entry(key).or_insert_with(Vec::new).push(metric.clone());
    }

    aggregated
}

/// Format timestamp by time period
pub fn format_timestamp_by_period(timestamp: DateTime<Utc>, period: TimePeriod) -> String {
    match period {
        TimePeriod::Hour => timestamp.format("%Y-%m-%d %H:00").to_string(),
        TimePeriod::Day => timestamp.format("%Y-%m-%d").to_string(),
        TimePeriod::Week => {
            let week_start = timestamp - chrono::Duration::days(timestamp.weekday().num_days_from_monday() as i64);
            week_start.format("%Y-%m-%d").to_string()
        }
        TimePeriod::Month => timestamp.format("%Y-%m").to_string(),
        TimePeriod::Year => timestamp.format("%Y").to_string(),
    }
}

/// Time period for aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

/// Calculate statistics for numeric values
pub fn calculate_statistics(values: &[f64]) -> Statistics {
    if values.is_empty() {
        return Statistics {
            count: 0,
            sum: 0.0,
            mean: 0.0,
            median: 0.0,
            min: 0.0,
            max: 0.0,
            std_dev: 0.0,
            variance: 0.0,
            p25: 0.0,
            p75: 0.0,
            p90: 0.0,
            p95: 0.0,
            p99: 0.0,
        };
    }

    let count = values.len();
    let sum: f64 = values.iter().sum();
    let mean = sum / count as f64;

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = sorted[0];
    let max = sorted[count - 1];
    let median = if count % 2 == 0 {
        (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
    } else {
        sorted[count / 2]
    };

    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
    let std_dev = variance.sqrt();

    let p25 = percentile(&sorted, 25);
    let p75 = percentile(&sorted, 75);
    let p90 = percentile(&sorted, 90);
    let p95 = percentile(&sorted, 95);
    let p99 = percentile(&sorted, 99);

    Statistics {
        count,
        sum,
        mean,
        median,
        min,
        max,
        std_dev,
        variance,
        p25,
        p75,
        p90,
        p95,
        p99,
    }
}

/// Calculate percentile
fn percentile(sorted: &[f64], percentile: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }

    let index = (percentile as f64 / 100.0 * (sorted.len() - 1) as f64).round() as usize;
    sorted[index.min(sorted.len() - 1)]
}

/// Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    /// Count
    pub count: usize,
    /// Sum
    pub sum: f64,
    /// Mean
    pub mean: f64,
    /// Median
    pub median: f64,
    /// Minimum
    pub min: f64,
    /// Maximum
    pub max: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Variance
    pub variance: f64,
    /// 25th percentile
    pub p25: f64,
    /// 75th percentile
    pub p75: f64,
    /// 90th percentile
    pub p90: f64,
    /// 95th percentile
    pub p95: f64,
    /// 99th percentile
    pub p99: f64,
}

/// Anonymize user ID for privacy
pub fn anonymize_user_id(user_id: &str) -> String {
    use sha2::{Digest, Sha256};
    
    let mut hasher = Sha256::new();
    hasher.update(user_id.as_bytes());
    let result = hasher.finalize();
    
    format!("user_{}", &result[..8])
}

/// Hash email for privacy
pub fn hash_email(email: &str) -> String {
    use sha2::{Digest, Sha256};
    
    let mut hasher = Sha256::new();
    hasher.update(email.to_lowercase().as_bytes());
    let result = hasher.finalize();
    
    format!("{:x}", result)
}

/// Check if data should be sampled
pub fn should_sample(sample_rate: f64) -> bool {
    rand::random::<f64>() < sample_rate
}

/// Calculate conversion rate
pub fn calculate_conversion_rate(conversions: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        conversions as f64 / total as f64
    }
}

/// Calculate statistical significance (simplified)
pub fn calculate_statistical_significance(
    control_conversions: usize,
    control_total: usize,
    treatment_conversions: usize,
    treatment_total: usize,
) -> StatisticalSignificance {
    let control_rate = calculate_conversion_rate(control_conversions, control_total);
    let treatment_rate = calculate_conversion_rate(treatment_conversions, treatment_total);

    let pooled_rate = (control_conversions + treatment_conversions) as f64
        / (control_total + treatment_total) as f64;

    let pooled_std_error = (pooled_rate * (1.0 - pooled_rate)
        * (1.0 / control_total as f64 + 1.0 / treatment_total as f64))
        .sqrt();

    let z_score = if pooled_std_error > 0.0 {
        (treatment_rate - control_rate) / pooled_std_error
    } else {
        0.0
    };

    let p_value = 2.0 * (1.0 - normal_cdf(z_score.abs()));

    let is_significant = p_value < 0.05;

    StatisticalSignificance {
        control_rate,
        treatment_rate,
        lift: if control_rate > 0.0 {
            (treatment_rate - control_rate) / control_rate * 100.0
        } else {
            0.0
        },
        z_score,
        p_value,
        is_significant,
        confidence_level: if is_significant { 95 } else { 0 },
    }
}

/// Normal cumulative distribution function (approximation)
fn normal_cdf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let x_abs = x.abs();
    let t = 1.0 / (1.0 + p * x_abs);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x_abs * x_abs / 2.0).exp();

    0.5 * (1.0 + sign * y)
}

/// Statistical significance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSignificance {
    /// Control conversion rate
    pub control_rate: f64,
    /// Treatment conversion rate
    pub treatment_rate: f64,
    /// Lift percentage
    pub lift: f64,
    /// Z-score
    pub z_score: f64,
    /// P-value
    pub p_value: f64,
    /// Is significant
    pub is_significant: bool,
    /// Confidence level
    pub confidence_level: usize,
}

/// Export data to CSV
pub fn export_to_csv<T>(data: &[T], headers: &[String]) -> Result<String>
where
    T: Serialize,
{
    let mut csv = String::new();

    // Write headers
    csv.push_str(&headers.join(","));
    csv.push('\n');

    // Write data
    for item in data {
        let json = serde_json::to_string(item)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;
        
        let row: Vec<String> = headers
            .iter()
            .map(|h| {
                value.get(h)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        
        csv.push_str(&row.join(","));
        csv.push('\n');
    }

    Ok(csv)
}

/// Export data to JSON
pub fn export_to_json<T>(data: &[T]) -> Result<String>
where
    T: Serialize,
{
    serde_json::to_string_pretty(data).map_err(Into::into)
}

/// Calculate moving average
pub fn calculate_moving_average(values: &[f64], window: usize) -> Vec<f64> {
    if values.is_empty() || window == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let window = window.min(values.len());

    for i in 0..=values.len() - window {
        let sum: f64 = values[i..i + window].iter().sum();
        result.push(sum / window as f64);
    }

    result
}

/// Calculate exponential moving average
pub fn calculate_exponential_moving_average(values: &[f64], alpha: f64) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(values.len());
    result.push(values[0]);

    for i in 1..values.len() {
        let ema = alpha * values[i] + (1.0 - alpha) * result[i - 1];
        result.push(ema);
    }

    result
}

/// Detect anomalies in data
pub fn detect_anomalies(values: &[f64], threshold: f64) -> Vec<usize> {
    if values.len() < 3 {
        return Vec::new();
    }

    let stats = calculate_statistics(values);
    let mean = stats.mean;
    let std_dev = stats.std_dev;

    values
        .iter()
        .enumerate()
        .filter(|(_, &v)| (v - mean).abs() > threshold * std_dev)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_statistics() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = calculate_statistics(&values);

        assert_eq!(stats.count, 5);
        assert_eq!(stats.sum, 15.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_anonymize_user_id() {
        let user_id = "user123@example.com";
        let anonymized = anonymize_user_id(user_id);
        assert!(!anonymized.contains(user_id));
        assert!(anonymized.starts_with("user_"));
    }

    #[test]
    fn test_hash_email() {
        let email = "test@example.com";
        let hashed = hash_email(email);
        assert!(!hashed.contains(email));
        assert_eq!(hashed.len(), 64); // SHA256 produces 64 hex characters
    }

    #[test]
    fn test_calculate_conversion_rate() {
        assert_eq!(calculate_conversion_rate(50, 100), 0.5);
        assert_eq!(calculate_conversion_rate(0, 100), 0.0);
        assert_eq!(calculate_conversion_rate(100, 0), 0.0);
    }

    #[test]
    fn test_calculate_moving_average() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = calculate_moving_average(&values, 3);
        assert_eq!(ma.len(), 3);
        assert_eq!(ma[0], 2.0);
        assert_eq!(ma[1], 3.0);
        assert_eq!(ma[2], 4.0);
    }

    #[test]
    fn test_detect_anomalies() {
        let values = vec![1.0, 2.0, 3.0, 100.0, 5.0];
        let anomalies = detect_anomalies(&values, 2.0);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0], 3);
    }
}