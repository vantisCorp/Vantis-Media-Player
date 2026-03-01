//! Utility Functions
//! 
//! Common utility functions for the advanced plugin system.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn};

/// Calculate SHA256 checksum of a file
pub fn calculate_checksum(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    let checksum = sha2::Sha256::digest(&bytes);
    Ok(hex::encode(checksum))
}

/// Verify checksum of a file
pub fn verify_checksum(path: &Path, expected: &str) -> Result<bool> {
    let actual = calculate_checksum(path)?;
    Ok(actual == expected)
}

/// Extract plugin metadata from WASM file
pub fn extract_metadata(path: &Path) -> Result<PluginMetadata> {
    let bytes = fs::read(path)?;
    
    // In a real implementation, you would parse WASM custom sections
    // For now, return default metadata
    Ok(PluginMetadata {
        name: path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        version: "1.0.0".to_string(),
        author: "Unknown".to_string(),
        description: String::new(),
        permissions: Vec::new(),
        dependencies: Vec::new(),
    })
}

/// Plugin metadata
#[derive(Clone, Debug)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin author
    pub author: String,
    
    /// Plugin description
    pub description: String,
    
    /// Required permissions
    pub permissions: Vec<String>,
    
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Validate plugin name
pub fn validate_plugin_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Plugin name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(anyhow::anyhow!("Plugin name too long (max 100 characters)"));
    }
    
    // Check for valid characters
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(anyhow::anyhow!("Plugin name contains invalid characters"));
    }
    
    Ok(())
}

/// Validate plugin version
pub fn validate_plugin_version(version: &str) -> Result<()> {
    if version.is_empty() {
        return Err(anyhow::anyhow!("Plugin version cannot be empty"));
    }
    
    // Try to parse as semver
    semver::Version::parse(version)
        .context("Invalid version format")?;
    
    Ok(())
}

/// Sanitize plugin name for file system
pub fn sanitize_plugin_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

/// Get plugin file path
pub fn get_plugin_path(plugin_dir: &Path, name: &str) -> PathBuf {
    plugin_dir.join(format!("{}.wasm", sanitize_plugin_name(name)))
}

/// Check if plugin exists
pub fn plugin_exists(plugin_dir: &Path, name: &str) -> bool {
    get_plugin_path(plugin_dir, name).exists()
}

/// List all plugins in directory
pub fn list_plugins(plugin_dir: &Path) -> Result<Vec<String>> {
    let mut plugins = Vec::new();
    
    if !plugin_dir.exists() {
        return Ok(plugins);
    }
    
    for entry in fs::read_dir(plugin_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "wasm") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                plugins.push(name.to_string());
            }
        }
    }
    
    plugins.sort();
    
    Ok(plugins)
}

/// Create plugin backup
pub fn create_backup(path: &Path) -> Result<PathBuf> {
    let backup_path = path.with_extension(format!("wasm.bak.{}", chrono::Utc::now().timestamp()));
    
    fs::copy(path, &backup_path)?;
    
    debug!("✅ Created backup: {}", backup_path.display());
    
    Ok(backup_path)
}

/// Restore plugin from backup
pub fn restore_backup(backup_path: &Path, target_path: &Path) -> Result<()> {
    if !backup_path.exists() {
        return Err(anyhow::anyhow!("Backup file not found"));
    }
    
    fs::copy(backup_path, target_path)?;
    
    debug!("✅ Restored from backup: {}", backup_path.display());
    
    Ok(())
}

/// Clean old backups
pub fn clean_old_backups(plugin_dir: &Path, max_age_days: i64) -> Result<usize> {
    let mut cleaned = 0;
    let cutoff = chrono::Utc::now() - chrono::Duration::days(max_age_days);
    
    for entry in fs::read_dir(plugin_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "bak") {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let modified_time = chrono::DateTime::<chrono::Utc>::from(modified);
                    
                    if modified_time < cutoff {
                        fs::remove_file(&path)?;
                        cleaned += 1;
                        debug!("🗑️  Removed old backup: {}", path.display());
                    }
                }
            }
        }
    }
    
    Ok(cleaned)
}

/// Format file size
pub fn format_file_size(bytes: u64) -> String {
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

/// Format duration
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if secs >= 3600 {
        format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
    } else if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs > 0 {
        format!("{}s {}ms", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Parse duration string
pub fn parse_duration(s: &str) -> Result<std::time::Duration> {
    let s = s.trim().to_lowercase();
    
    if s.ends_with("ms") {
        let ms: u64 = s[..s.len()-2].parse()
            .context("Invalid millisecond value")?;
        Ok(std::time::Duration::from_millis(ms))
    } else if s.ends_with("s") {
        let secs: u64 = s[..s.len()-1].parse()
            .context("Invalid second value")?;
        Ok(std::time::Duration::from_secs(secs))
    } else if s.ends_with("m") {
        let mins: u64 = s[..s.len()-1].parse()
            .context("Invalid minute value")?;
        Ok(std::time::Duration::from_secs(mins * 60))
    } else if s.ends_with("h") {
        let hours: u64 = s[..s.len()-1].parse()
            .context("Invalid hour value")?;
        Ok(std::time::Duration::from_secs(hours * 3600))
    } else {
        // Default to seconds
        let secs: u64 = s.parse()
            .context("Invalid duration value")?;
        Ok(std::time::Duration::from_secs(secs))
    }
}

/// Create temporary directory
pub fn create_temp_dir() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let plugin_temp = temp_dir.join(format!("vantis-plugins-{}", chrono::Utc::now().timestamp()));
    
    fs::create_dir_all(&plugin_temp)?;
    
    Ok(plugin_temp)
}

/// Clean temporary directory
pub fn clean_temp_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
        debug!("🗑️  Cleaned temp directory: {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_validate_plugin_name() {
        assert!(validate_plugin_name("test_plugin").is_ok());
        assert!(validate_plugin_name("test-plugin").is_ok());
        assert!(validate_plugin_name("").is_err());
        assert!(validate_plugin_name("a".repeat(101).as_str()).is_err());
    }
    
    #[test]
    fn test_validate_plugin_version() {
        assert!(validate_plugin_version("1.0.0").is_ok());
        assert!(validate_plugin_version("1.2.3-beta").is_ok());
        assert!(validate_plugin_version("").is_err());
        assert!(validate_plugin_version("invalid").is_err());
    }
    
    #[test]
    fn test_sanitize_plugin_name() {
        assert_eq!(sanitize_plugin_name("test plugin"), "test_plugin");
        assert_eq!(sanitize_plugin_name("test@plugin"), "test_plugin");
        assert_eq!(sanitize_plugin_name("test-plugin"), "test_plugin");
    }
    
    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(std::time::Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(std::time::Duration::from_secs(5)), "5s 0ms");
        assert_eq!(format_duration(std::time::Duration::from_secs(65)), "1m 5s");
        assert_eq!(format_duration(std::time::Duration::from_secs(3665)), "1h 1m 5s");
    }
    
    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("500ms").unwrap(), std::time::Duration::from_millis(500));
        assert_eq!(parse_duration("5s").unwrap(), std::time::Duration::from_secs(5));
        assert_eq!(parse_duration("5m").unwrap(), std::time::Duration::from_secs(300));
        assert_eq!(parse_duration("1h").unwrap(), std::time::Duration::from_secs(3600));
    }
    
    #[test]
    fn test_list_plugins() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path();
        
        // Create some plugin files
        fs::write(plugin_dir.join("plugin1.wasm"), b"test").unwrap();
        fs::write(plugin_dir.join("plugin2.wasm"), b"test").unwrap();
        fs::write(plugin_dir.join("not_a_plugin.txt"), b"test").unwrap();
        
        let plugins = list_plugins(plugin_dir).unwrap();
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"plugin1".to_string()));
        assert!(plugins.contains(&"plugin2".to_string()));
    }
}