//! Streaming utility functions
//! 
//! Helper functions for streaming operations including URL parsing,
//! data processing, and format conversion.

use crate::{StreamingError, StreamingResult};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Parse URL and extract components
pub fn parse_url(url: &str) -> StreamingResult<UrlComponents> {
    let parsed = url::Url::parse(url)
        .map_err(|e| StreamingError::InvalidUrl(e.to_string()))?;
    
    Ok(UrlComponents {
        scheme: parsed.scheme().to_string(),
        host: parsed.host_str().map(|s| s.to_string()),
        port: parsed.port(),
        path: parsed.path().to_string(),
        query: parsed.query().map(|s| s.to_string()),
        fragment: parsed.fragment().map(|s| s.to_string()),
    })
}

/// URL components
#[derive(Debug, Clone)]
pub struct UrlComponents {
    pub scheme: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// Parse query string into key-value pairs
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    for pair in query.split('&') {
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].to_string();
            let value = urlencoding::decode(parts[1])
                .unwrap_or(std::borrow::Cow::Borrowed(parts[1]))
                .to_string();
            params.insert(key, value);
        }
    }
    
    params
}

/// Build query string from key-value pairs
pub fn build_query_string(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Format bytes to human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Format bitrate to human-readable string
pub fn format_bitrate(bps: u64) -> String {
    const UNITS: &[&str] = &["bps", "Kbps", "Mbps", "Gbps"];
    
    if bps == 0 {
        return "0 bps".to_string();
    }
    
    let mut bitrate = bps as f64;
    let mut unit_index = 0;
    
    while bitrate >= 1000.0 && unit_index < UNITS.len() - 1 {
        bitrate /= 1000.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", bitrate, UNITS[unit_index])
}

/// Format duration to human-readable string
pub fn format_duration(secs: f64) -> String {
    let hours = (secs / 3600.0) as u64;
    let minutes = ((secs % 3600.0) / 60.0) as u64;
    let seconds = (secs % 60.0) as u64;
    let millis = ((secs % 1.0) * 1000.0) as u64;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
    } else {
        format!("{:02}.{:03}s", seconds, millis)
    }
}

/// Calculate bitrate from bytes and duration
pub fn calculate_bitrate(bytes: u64, duration_secs: f64) -> u64 {
    if duration_secs > 0.0 {
        ((bytes as f64 * 8.0) / duration_secs) as u64
    } else {
        0
    }
}

/// Calculate duration from bytes and bitrate
pub fn calculate_duration(bytes: u64, bitrate_bps: u64) -> f64 {
    if bitrate_bps > 0 {
        (bytes as f64 * 8.0) / bitrate_bps as f64
    } else {
        0.0
    }
}

/// Get current timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Get current timestamp in milliseconds
pub fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Sleep for specified duration
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Retry operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_attempts: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;
    
    for attempt in 1..=max_attempts {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_attempts {
                    return Err(e);
                }
                
                tracing::warn!("Attempt {} failed: {}, retrying in {:?}", attempt, e, delay);
                sleep(delay).await;
                delay *= 2;
            }
        }
    }
    
    unreachable!()
}

/// Validate URL
pub fn validate_url(url: &str) -> StreamingResult<()> {
    let parsed = url::Url::parse(url)
        .map_err(|e| StreamingError::InvalidUrl(e.to_string()))?;
    
    if parsed.scheme().is_empty() {
        return Err(StreamingError::InvalidUrl("Missing scheme".to_string()));
    }
    
    if parsed.host_str().is_none() {
        return Err(StreamingError::InvalidUrl("Missing host".to_string()));
    }
    
    Ok(())
}

/// Extract file extension from URL
pub fn extract_extension(url: &str) -> Option<String> {
    let parsed = url::Url::parse(url).ok()?;
    let path = parsed.path();
    
    path.rfind('.')
        .map(|i| path[i + 1..].to_string())
}

/// Generate unique ID
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Calculate hash of data
pub fn calculate_hash(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Chunk data into specified size
pub fn chunk_data(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    data.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Merge chunks into single buffer
pub fn merge_chunks(chunks: &[Vec<u8>]) -> Vec<u8> {
    let total_size: usize = chunks.iter().map(|c| c.len()).sum();
    let mut merged = Vec::with_capacity(total_size);
    
    for chunk in chunks {
        merged.extend_from_slice(chunk);
    }
    
    merged
}

/// Calculate moving average
pub fn moving_average(values: &[f64], window_size: usize) -> Vec<f64> {
    if values.is_empty() || window_size == 0 {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    let mut window: Vec<f64> = Vec::new();
    
    for &value in values {
        window.push(value);
        
        if window.len() > window_size {
            window.remove(0);
        }
        
        let avg = window.iter().sum::<f64>() / window.len() as f64;
        result.push(avg);
    }
    
    result
}

/// Calculate percentile
pub fn calculate_percentile(values: &[f64], percentile: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let index = ((percentile / 100.0) * (sorted.len() - 1) as f64) as usize;
    sorted[index]
}

/// Clamp value between min and max
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Map value from one range to another
pub fn map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_url() {
        let url = "https://example.com:8080/path?query=value#fragment";
        let components = parse_url(url).unwrap();
        assert_eq!(components.scheme, "https");
        assert_eq!(components.host, Some("example.com".to_string()));
        assert_eq!(components.port, Some(8080));
        assert_eq!(components.path, "/path");
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    }
    
    #[test]
    fn test_format_bitrate() {
        assert_eq!(format_bitrate(0), "0 bps");
        assert_eq!(format_bitrate(1000), "1.00 Kbps");
        assert_eq!(format_bitrate(1_000_000), "1.00 Mbps");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "00.000s");
        assert_eq!(format_duration(65.5), "01:05.500");
        assert_eq!(format_duration(3665.5), "01:01:05");
    }
    
    #[test]
    fn test_calculate_bitrate() {
        let bitrate = calculate_bitrate(1_000_000, 2.0);
        assert_eq!(bitrate, 4_000_000);
    }
    
    #[test]
    fn test_extract_extension() {
        assert_eq!(extract_extension("https://example.com/video.mp4"), Some("mp4".to_string()));
        assert_eq!(extract_extension("https://example.com/video"), None);
    }
    
    #[test]
    fn test_moving_average() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let avg = moving_average(&values, 3);
        assert_eq!(avg.len(), 5);
    }
    
    #[test]
    fn test_calculate_percentile() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let p50 = calculate_percentile(&values, 50.0);
        assert_eq!(p50, 3.0);
    }
}