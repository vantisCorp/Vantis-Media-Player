//! Utility Functions for Build & Deployment
//! 
//! Provides helper functions for:
//! - File operations
//! - Version management
//! - Git operations
//! - Build utilities
//! - Deployment utilities

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use semver::{Version, VersionReq};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn};

/// Calculate file checksum
pub fn calculate_checksum(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    
    let mut file = fs::File::open(path)
        .context("Failed to open file")?;
    
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)
        .context("Failed to read file")?;
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Get file size in bytes
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)
        .context("Failed to get file metadata")?;
    Ok(metadata.len())
}

/// Create directory if it doesn't exist
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .context("Failed to create directory")?;
        debug!("Created directory: {:?}", path);
    }
    Ok(())
}

/// Copy file with progress callback
pub fn copy_file_with_progress<F>(
    src: &Path,
    dst: &Path,
    mut progress_callback: F,
) -> Result<()>
where
    F: FnMut(u64, u64), // (bytes_copied, total_bytes)
{
    let total_bytes = get_file_size(src)?;
    
    let mut src_file = fs::File::open(src)
        .context("Failed to open source file")?;
    
    let mut dst_file = fs::File::create(dst)
        .context("Failed to create destination file")?;
    
    let mut buffer = [0u8; 8192];
    let mut copied = 0u64;
    
    loop {
        let bytes_read = std::io::Read::read(&mut src_file, &mut buffer)
            .context("Failed to read from source file")?;
        
        if bytes_read == 0 {
            break;
        }
        
        std::io::Write::write_all(&mut dst_file, &buffer[..bytes_read])
            .context("Failed to write to destination file")?;
        
        copied += bytes_read as u64;
        progress_callback(copied, total_bytes);
    }
    
    Ok(())
}

/// Parse version string
pub fn parse_version(version: &str) -> Result<Version> {
    Version::parse(version)
        .context("Failed to parse version string")
}

/// Compare versions
pub fn compare_versions(v1: &str, v2: &str) -> Result<std::cmp::Ordering> {
    let version1 = parse_version(v1)?;
    let version2 = parse_version(v2)?;
    Ok(version1.cmp(&version2))
}

/// Check if version satisfies requirement
pub fn satisfies_requirement(version: &str, requirement: &str) -> Result<bool> {
    let version = parse_version(version)?;
    let req = VersionReq::parse(requirement)
        .context("Failed to parse version requirement")?;
    Ok(req.matches(&version))
}

/// Bump version
pub fn bump_version(version: &str, bump_type: VersionBump) -> Result<String> {
    let mut version = parse_version(version)?;
    
    match bump_type {
        VersionBump::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        VersionBump::Minor => {
            version.minor += 1;
            version.patch = 0;
        }
        VersionBump::Patch => {
            version.patch += 1;
        }
    }
    
    Ok(version.to_string())
}

/// Version bump type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionBump {
    /// Major version bump
    Major,
    /// Minor version bump
    Minor,
    /// Patch version bump
    Patch,
}

/// Get current git commit hash
pub fn get_git_commit_hash() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .context("Failed to get git commit hash")?;

    if !output.status.success() {
        anyhow::bail!("Git command failed");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get current git branch
pub fn get_git_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to get git branch")?;

    if !output.status.success() {
        anyhow::bail!("Git command failed");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get git tags
pub fn get_git_tags() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["tag", "--sort=-version:refname"])
        .output()
        .context("Failed to get git tags")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let tags: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    Ok(tags)
}

/// Get git commits since tag
pub fn get_commits_since_tag(tag: &str) -> Result<Vec<GitCommit>> {
    let output = Command::new("git")
        .args([
            "log",
            &format!("{}..HEAD", tag),
            "--pretty=format:%H|%an|%ae|%ad|%s",
            "--date=iso",
        ])
        .output()
        .context("Failed to get git commits")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 5 {
            commits.push(GitCommit {
                hash: parts[0].to_string(),
                author: parts[1].to_string(),
                email: parts[2].to_string(),
                date: parts[3].to_string(),
                message: parts[4].to_string(),
            });
        }
    }

    Ok(commits)
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
}

/// Create git tag
pub fn create_git_tag(tag: &str, message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["tag", "-a", tag, "-m", message])
        .output()
        .context("Failed to create git tag")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create tag: {}", stderr);
    }

    info!("Created git tag: {}", tag);
    Ok(())
}

/// Push git tag to remote
pub fn push_git_tag(tag: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["push", "origin", tag])
        .output()
        .context("Failed to push git tag")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to push tag: {}", stderr);
    }

    info!("Pushed git tag: {}", tag);
    Ok(())
}

/// Run command with timeout
pub fn run_command_with_timeout(
    command: &mut Command,
    timeout_secs: u64,
) -> Result<std::process::Output> {
    use std::time::Duration;

    let mut child = command.spawn()
        .context("Failed to spawn command")?;

    let timeout = Duration::from_secs(timeout_secs);
    
    // Wait for completion or timeout
    match std::thread::sleep(timeout) {
        _ => {
            // Timeout reached, kill the process
            let _ = child.kill();
            anyhow::bail!("Command timed out after {} seconds", timeout_secs);
        }
    }
}

/// Format duration as human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    
    if total_secs < 60 {
        format!("{}s", total_secs)
    } else if total_secs < 3600 {
        format!("{}m {}s", total_secs / 60, total_secs % 60)
    } else {
        format!("{}h {}m {}s", 
            total_secs / 3600,
            (total_secs % 3600) / 60,
            total_secs % 60)
    }
}

/// Format bytes as human-readable string
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

/// Extract version from Cargo.toml
pub fn extract_version_from_cargo_toml(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)
        .context("Failed to read Cargo.toml")?;
    
    let value: toml::Value = toml::from_str(&content)
        .context("Failed to parse Cargo.toml")?;
    
    let version = value
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            value.get("package")
                .and_then(|p| p.get("version"))
                .and_then(|v| v.as_str())
        })
        .context("Version not found in Cargo.toml")?;

    Ok(version.to_string())
}

/// Update version in Cargo.toml
pub fn update_version_in_cargo_toml(path: &Path, new_version: &str) -> Result<()> {
    let content = fs::read_to_string(path)
        .context("Failed to read Cargo.toml")?;
    
    let mut value: toml::Value = toml::from_str(&content)
        .context("Failed to parse Cargo.toml")?;
    
    // Update workspace version
    if let Some(workspace) = value.get_mut("workspace") {
        if let Some(package) = workspace.get_mut("package") {
            if let Some(version) = package.get_mut("version") {
                *version = toml::Value::String(new_version.to_string());
            }
        }
    }
    
    // Update package version
    if let Some(package) = value.get_mut("package") {
        if let Some(version) = package.get_mut("version") {
            *version = toml::Value::String(new_version.to_string());
        }
    }
    
    let new_content = toml::to_string_pretty(&value)
        .context("Failed to serialize Cargo.toml")?;
    
    fs::write(path, new_content)
        .context("Failed to write Cargo.toml")?;
    
    info!("Updated version in {:?} to {}", path, new_version);
    Ok(())
}

/// Validate semver version
pub fn validate_semver(version: &str) -> Result<()> {
    parse_version(version)?;
    Ok(())
}

/// Extract changelog entry
pub fn extract_changelog_entry(changelog_path: &Path, version: &str) -> Result<String> {
    let content = fs::read_to_string(changelog_path)
        .context("Failed to read CHANGELOG.md")?;
    
    // Look for version section
    let version_pattern = format!(r"## \[{}\]", regex::escape(version));
    let re = Regex::new(&version_pattern)
        .context("Failed to create regex")?;
    
    if let Some(start) = re.find(&content) {
        let start_pos = start.start();
        
        // Find next version section or end of file
        let next_version_re = Regex::new(r"## \[\d+\.\d+\.\d+\]")
            .context("Failed to create regex")?;
        
        let end_pos = next_version_re
            .find_at(&content, start_pos + version.len())
            .map(|m| m.start())
            .unwrap_or(content.len());
        
        let entry = content[start_pos..end_pos].trim().to_string();
        return Ok(entry);
    }
    
    anyhow::bail!("Changelog entry not found for version: {}", version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let version = parse_version("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_compare_versions() {
        assert_eq!(compare_versions("1.2.3", "1.2.4").unwrap(), std::cmp::Ordering::Less);
        assert_eq!(compare_versions("1.2.3", "1.2.3").unwrap(), std::cmp::Ordering::Equal);
        assert_eq!(compare_versions("1.2.4", "1.2.3").unwrap(), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_satisfies_requirement() {
        assert!(satisfies_requirement("1.2.3", "^1.2.0").unwrap());
        assert!(satisfies_requirement("1.3.0", "^1.2.0").unwrap());
        assert!(!satisfies_requirement("2.0.0", "^1.2.0").unwrap());
    }

    #[test]
    fn test_bump_version() {
        assert_eq!(bump_version("1.2.3", VersionBump::Patch).unwrap(), "1.2.4");
        assert_eq!(bump_version("1.2.3", VersionBump::Minor).unwrap(), "1.3.0");
        assert_eq!(bump_version("1.2.3", VersionBump::Major).unwrap(), "2.0.0");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_validate_semver() {
        assert!(validate_semver("1.2.3").is_ok());
        assert!(validate_semver("1.2.3-alpha").is_ok());
        assert!(validate_semver("1.2.3+build").is_ok());
        assert!(validate_semver("invalid").is_err());
    }
}