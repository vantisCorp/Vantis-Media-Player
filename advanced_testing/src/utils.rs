//! Utility Functions
//! 
//! Common utility functions for the advanced testing suite.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};

/// Generate random bytes
pub fn generate_random_bytes(size: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

/// Generate random string
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Measure execution time of a function
pub fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Measure execution time of an async function
pub async fn measure_time_async<F, R>(f: F) -> (R, Duration)
where
    F: std::future::Future<Output = R>,
{
    let start = Instant::now();
    let result = f.await;
    let duration = start.elapsed();
    (result, duration)
}

/// Create temporary directory
pub fn create_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let dir_path = temp_dir.join(format!("{}-{}", prefix, generate_random_string(8)));
    std::fs::create_dir_all(&dir_path)?;
    Ok(dir_path)
}

/// Clean up temporary directory
pub fn cleanup_temp_dir(dir: &Path) -> Result<()> {
    if dir.exists() {
        std::fs::remove_dir_all(dir)?;
        debug!("✅ Cleaned up temporary directory: {}", dir.display());
    }
    Ok(())
}

/// Write test data to file
pub fn write_test_data(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, data)?;
    debug!("✅ Wrote test data to: {}", path.display());
    Ok(())
}

/// Read test data from file
pub fn read_test_data(path: &Path) -> Result<Vec<u8>> {
    let data = std::fs::read(path)?;
    debug!("✅ Read test data from: {}", path.display());
    Ok(data)
}

/// Compare two files
pub fn compare_files(path1: &Path, path2: &Path) -> Result<bool> {
    let data1 = std::fs::read(path1)?;
    let data2 = std::fs::read(path2)?;
    Ok(data1 == data2)
}

/// Calculate file checksum
pub fn calculate_file_checksum(path: &Path) -> Result<String> {
    let data = std::fs::read(path)?;
    let checksum = sha2::Sha256::digest(&data);
    Ok(hex::encode(checksum))
}

/// Format duration for display
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if secs > 60 {
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{}m {}s", minutes, seconds)
    } else if secs > 0 {
        format!("{}s {}ms", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Format bytes for display
pub fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;
    const GB: usize = 1024 * MB;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Parse duration from string
pub fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim().to_lowercase();
    
    if s.ends_with('s') {
        let secs: u64 = s[..s.len()-1].parse()?;
        Ok(Duration::from_secs(secs))
    } else if s.ends_with("ms") {
        let millis: u64 = s[..s.len()-2].parse()?;
        Ok(Duration::from_millis(millis))
    } else if s.ends_with("us") {
        let micros: u64 = s[..s.len()-2].parse()?;
        Ok(Duration::from_micros(micros))
    } else if s.ends_with("ns") {
        let nanos: u64 = s[..s.len()-2].parse()?;
        Ok(Duration::from_nanos(nanos))
    } else {
        // Default to seconds
        let secs: u64 = s.parse()?;
        Ok(Duration::from_secs(secs))
    }
}

/// Retry function with exponential backoff
pub async fn retry_async<F, R, E>(
    mut f: F,
    max_attempts: u32,
    initial_delay: Duration,
) -> Result<R, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, E>> + Send>>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;
    
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_attempts {
                    return Err(e);
                }
                warn!("Attempt {} failed: {}. Retrying in {:?}", attempt, e, delay);
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
        }
    }
    
    unreachable!()
}

/// Wait for condition with timeout
pub async fn wait_for_condition<F>(
    mut condition: F,
    timeout: Duration,
    check_interval: Duration,
) -> Result<bool>
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        if condition() {
            return Ok(true);
        }
        tokio::time::sleep(check_interval).await;
    }
    
    Ok(false)
}

/// Spawn multiple tasks and wait for all to complete
pub async fn spawn_and_wait<F, R>(tasks: Vec<F>) -> Vec<Result<R>>
where
    F: std::future::Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let handles: Vec<_> = tasks
        .into_iter()
        .map(|task| tokio::spawn(task))
        .collect();
    
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.map_err(|e| anyhow::anyhow!("Task failed: {}", e)));
    }
    
    results
}

/// Collect test results
pub fn collect_test_results<T>(results: Vec<Result<T>>) -> (Vec<T>, Vec<anyhow::Error>) {
    let mut successes = Vec::new();
    let mut failures = Vec::new();
    
    for result in results {
        match result {
            Ok(value) => successes.push(value),
            Err(e) => failures.push(e),
        }
    }
    
    (successes, failures)
}

/// Generate test report
pub fn generate_test_report(
    test_name: &str,
    total: usize,
    passed: usize,
    failed: usize,
    duration: Duration,
) -> String {
    let percentage = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    format!(
        "=== {} Test Report ===\n\
         Total: {}\n\
         Passed: {} ({:.1}%)\n\
         Failed: {}\n\
         Duration: {}\n\
         =======================",
        test_name,
        total,
        passed,
        percentage,
        failed,
        format_duration(duration)
    )
}

/// Save test report to file
pub fn save_test_report(path: &Path, report: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, report)?;
    info!("✅ Saved test report to: {}", path.display());
    Ok(())
}

/// Load test report from file
pub fn load_test_report(path: &Path) -> Result<String> {
    let report = std::fs::read_to_string(path)?;
    debug!("✅ Loaded test report from: {}", path.display());
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_random_bytes() {
        let bytes = generate_random_bytes(100);
        assert_eq!(bytes.len(), 100);
    }
    
    #[test]
    fn test_generate_random_string() {
        let s = generate_random_string(10);
        assert_eq!(s.len(), 10);
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(5)), "5s 0ms");
        assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("5s").unwrap(), Duration::from_secs(5));
        assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
        assert_eq!(parse_duration("1000us").unwrap(), Duration::from_micros(1000));
    }
}