//! Cross-Compilation Engine
//! 
//! Provides cross-compilation capabilities for multiple platforms including:
//! - Linux (x86_64, aarch64)
//! - Windows (x86_64)
//! - macOS (x86_64, aarch64)

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, info, warn};

use super::{BuildType, BuildStatus};

/// Target platform for cross-compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    /// Linux x86_64
    LinuxX86_64,
    /// Linux aarch64 (ARM64)
    LinuxAarch64,
    /// Windows x86_64
    WindowsX86_64,
    /// macOS x86_64 (Intel)
    MacOSX86_64,
    /// macOS aarch64 (Apple Silicon)
    MacOSAarch64,
}

impl Platform {
    /// Get the Rust target triple
    pub fn target_triple(&self) -> &'static str {
        match self {
            Platform::LinuxX86_64 => "x86_64-unknown-linux-gnu",
            Platform::LinuxAarch64 => "aarch64-unknown-linux-gnu",
            Platform::WindowsX86_64 => "x86_64-pc-windows-msvc",
            Platform::MacOSX86_64 => "x86_64-apple-darwin",
            Platform::MacOSAarch64 => "aarch64-apple-darwin",
        }
    }

    /// Get the platform name
    pub fn name(&self) -> &'static str {
        match self {
            Platform::LinuxX86_64 => "linux-x86_64",
            Platform::LinuxAarch64 => "linux-aarch64",
            Platform::WindowsX86_64 => "windows-x86_64",
            Platform::MacOSX86_64 => "macos-x86_64",
            Platform::MacOSAarch64 => "macos-aarch64",
        }
    }

    /// Get the output file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Platform::WindowsX86_64 => ".exe",
            _ => "",
        }
    }

    /// Check if this platform is the host platform
    pub fn is_host(&self) -> bool {
        let host = std::env::consts::ARCH;
        let os = std::env::consts::OS;

        match self {
            Platform::LinuxX86_64 => os == "linux" && host == "x86_64",
            Platform::LinuxAarch64 => os == "linux" && host == "aarch64",
            Platform::WindowsX86_64 => os == "windows" && host == "x86_64",
            Platform::MacOSX86_64 => os == "macos" && host == "x86_64",
            Platform::MacOSAarch64 => os == "macos" && host == "aarch64",
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Output directory
    pub output_dir: PathBuf,
    /// Enable optimizations
    pub optimize: bool,
    /// Enable debug symbols
    pub debug_symbols: bool,
    /// Number of parallel jobs
    pub jobs: Option<usize>,
    /// Custom cargo flags
    pub cargo_flags: Vec<String>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./target/releases"),
            optimize: true,
            debug_symbols: false,
            jobs: None,
            cargo_flags: Vec::new(),
            env_vars: HashMap::new(),
        }
    }
}

/// Cross-compilation engine
#[derive(Clone)]
pub struct CrossCompileEngine {
    config: BuildConfig,
    /// Build cache
    build_cache: HashMap<Platform, BuildCacheEntry>,
}

/// Build cache entry
#[derive(Debug, Clone)]
struct BuildCacheEntry {
    /// Last build time
    last_build: DateTime<Utc>,
    /// Build status
    status: BuildStatus,
    /// Output path
    output_path: Option<PathBuf>,
}

impl CrossCompileEngine {
    /// Create a new cross-compilation engine
    pub async fn new(config: BuildConfig) -> Result<Self> {
        info!("Initializing Cross-Compilation Engine");

        // Create output directory
        std::fs::create_dir_all(&config.output_dir)
            .context("Failed to create output directory")?;

        Ok(Self {
            config,
            build_cache: HashMap::new(),
        })
    }

    /// Build for a specific platform
    pub async fn build(&self, platform: Platform, build_type: BuildType) -> Result<PathBuf> {
        info!("Building for platform: {} ({:?})", platform.name(), build_type);

        let target_triple = platform.target_triple();
        let output_dir = self.config.output_dir.join(platform.name());

        // Create platform-specific output directory
        std::fs::create_dir_all(&output_dir)
            .context("Failed to create platform output directory")?;

        // Build cargo command
        let mut cmd = Command::new("cargo");
        cmd.arg("build");

        // Set target
        if !platform.is_host() {
            cmd.arg("--target").arg(target_triple);
        }

        // Set build type
        match build_type {
            BuildType::Debug => {
                // Debug is default
            }
            BuildType::Release => {
                cmd.arg("--release");
            }
            BuildType::Profile => {
                cmd.arg("--profile=release");
            }
            BuildType::Benchmark => {
                cmd.arg("--profile=bench");
            }
        }

        // Set output directory
        cmd.arg("--target-dir").arg(&output_dir);

        // Add custom flags
        for flag in &self.config.cargo_flags {
            cmd.arg(flag);
        }

        // Set environment variables
        for (key, value) in &self.config.env_vars {
            cmd.env(key, value);
        }

        // Set optimization level
        if self.config.optimize {
            cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "3");
        }

        // Set debug symbols
        if self.config.debug_symbols {
            cmd.env("CARGO_PROFILE_RELEASE_DEBUG", "1");
        }

        // Set parallel jobs
        if let Some(jobs) = self.config.jobs {
            cmd.arg(format!("-j{}", jobs));
        }

        debug!("Executing: {:?}", cmd);

        // Execute build
        let output = cmd.output()
            .context("Failed to execute cargo build")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Build failed: {}", stderr);
        }

        // Find output binary
        let binary_name = format!("vantis-player{}", platform.extension());
        let binary_path = output_dir
            .join(target_triple)
            .join(match build_type {
                BuildType::Debug => "debug",
                _ => "release",
            })
            .join(&binary_name);

        if !binary_path.exists() {
            anyhow::bail!("Binary not found at: {:?}", binary_path);
        }

        info!("Build completed successfully: {:?}", binary_path);

        Ok(binary_path)
    }

    /// Build for all platforms
    pub async fn build_all(&self, build_type: BuildType) -> Result<HashMap<Platform, PathBuf>> {
        info!("Building for all platforms");

        let platforms = vec![
            Platform::LinuxX86_64,
            Platform::LinuxAarch64,
            Platform::WindowsX86_64,
            Platform::MacOSX86_64,
            Platform::MacOSAarch64,
        ];

        let mut results = HashMap::new();

        for platform in platforms {
            let path = self.build(platform, build_type).await?;
            results.insert(platform, path);
        }

        Ok(results)
    }

    /// Check if a platform can be built on the host
    pub fn can_build(&self, platform: Platform) -> bool {
        // Check if cross-compilation tools are available
        match platform {
            Platform::LinuxX86_64 | Platform::WindowsX86_64 => {
                // Can always build these on Linux/Windows
                true
            }
            Platform::LinuxAarch64 => {
                // Need aarch64-linux-gnu toolchain
                self.check_toolchain("aarch64-linux-gnu-gcc")
            }
            Platform::MacOSX86_64 | Platform::MacOSAarch64 => {
                // Can only build on macOS
                std::env::consts::OS == "macos"
            }
        }
    }

    /// Check if a toolchain is available
    fn check_toolchain(&name: &str) -> bool {
        Command::new(name)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get build cache
    pub fn get_cache(&self) -> &HashMap<Platform, BuildCacheEntry> {
        &self.build_cache
    }

    /// Clear build cache
    pub fn clear_cache(&mut self) {
        self.build_cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_target_triple() {
        assert_eq!(Platform::LinuxX86_64.target_triple(), "x86_64-unknown-linux-gnu");
        assert_eq!(Platform::WindowsX86_64.target_triple(), "x86_64-pc-windows-msvc");
        assert_eq!(Platform::MacOSAarch64.target_triple(), "aarch64-apple-darwin");
    }

    #[test]
    fn test_platform_name() {
        assert_eq!(Platform::LinuxX86_64.name(), "linux-x86_64");
        assert_eq!(Platform::WindowsX86_64.name(), "windows-x86_64");
    }

    #[test]
    fn test_platform_extension() {
        assert_eq!(Platform::WindowsX86_64.extension(), ".exe");
        assert_eq!(Platform::LinuxX86_64.extension(), "");
    }

    #[test]
    fn test_build_config_default() {
        let config = BuildConfig::default();
        assert!(config.optimize);
        assert!(!config.debug_symbols);
        assert_eq!(config.output_dir, PathBuf::from("./target/releases"));
    }

    #[tokio::test]
    async fn test_cross_compile_engine_creation() {
        let config = BuildConfig::default();
        let engine = CrossCompileEngine::new(config).await;
        assert!(engine.is_ok());
    }
}