//! Utility Functions
//! 
//! Common utility functions for the advanced documentation system.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn};

/// Escape HTML special characters
pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Format duration for display
pub fn format_duration(seconds: u64) -> String {
    if seconds >= 3600 {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    } else if seconds >= 60 {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", seconds)
    }
}

/// Format date for display
pub fn format_date(date: &str) -> Result<String> {
    let dt = chrono::DateTime::parse_from_rfc3339(date)?;
    Ok(dt.format("%B %d, %Y").to_string())
}

/// Generate slug from title
pub fn generate_slug(title: &str) -> String {
    title.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "-")
        .replace(' ', "-")
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Read JSON file
pub fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path)?;
    let value: T = serde_json::from_str(&content)?;
    Ok(value)
}

/// Write JSON file
pub fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(value)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Copy directory recursively
pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }
    
    std::fs::create_dir_all(dst)?;
    
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Minify HTML
pub fn minify_html(html: &str) -> String {
    html.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| !(w[0].is_whitespace() && w[1].is_whitespace()))
        .map(|w| w[0])
        .collect::<String>()
        + html.chars().last().unwrap_or(&'\0')
}

/// Generate table of contents from HTML
pub fn generate_toc(html: &str) -> String {
    let mut toc = String::new();
    let mut depth = 0;
    
    for line in html.lines() {
        if line.contains("<h1>") {
            depth = 1;
            if let Some(start) = line.find("<h1>") {
                if let Some(end) = line.find("</h1>") {
                    let title = &line[start + 4..end];
                    toc.push_str(&format!("<li><a href=&quot;#{}&quot;>{}</a></li>\n", 
                        generate_slug(title), title));
                }
            }
        } else if line.contains("<h2>") {
            depth = 2;
            if let Some(start) = line.find("<h2>") {
                if let Some(end) = line.find("</h2>") {
                    let title = &line[start + 4..end];
                    toc.push_str(&format!("<li><a href=&quot;#{}&quot;>{}</a></li>\n", 
                        generate_slug(title), title));
                }
            }
        } else if line.contains("<h3>") {
            depth = 3;
            if let Some(start) = line.find("<h3>") {
                if let Some(end) = line.find("</h3>") {
                    let title = &line[start + 4..end];
                    toc.push_str(&format!("<li><a href=&quot;#{}&quot;>{}</a></li>\n", 
                        generate_slug(title), title));
                }
            }
        }
    }
    
    format!("<ul>\n{}</ul>", toc)
}

/// Validate URL
pub fn is_valid_url(url: &str) -> bool {
    url.parse::<url::Url>().is_ok()
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> Option<String> {
    url.parse::<url::Url>()
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
}

/// Generate random ID
pub fn generate_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:x}", rng.gen::<u64>())
}

/// Create temporary directory
pub fn create_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let dir_path = temp_dir.join(format!("{}-{}", prefix, generate_id()));
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<div>"), "&lt;div&gt;");
        assert_eq!(html_escape("&"), "&amp;");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
    }
    
    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("Test 123"), "test-123");
    }
    
    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(!is_valid_url("not-a-url"));
    }
    
    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path"), Some("example.com".to_string()));
        assert_eq!(extract_domain("not-a-url"), None);
    }
}