//! Advanced Build & Deployment System
//! 
//! This module provides comprehensive build and deployment capabilities including:
//! - Cross-compilation matrix for multiple platforms
//! - Automated release notes generation
//! - Deployment dashboard
//! - Rollback automation
//! - Health monitoring

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub mod cross_compile;
pub mod release_notes;
pub mod deployment;
pub mod rollback;
pub mod monitoring;
pub mod utils;

use cross_compile::{CrossCompileEngine, Platform, BuildConfig};
use deployment::{DeploymentEngine, DeploymentTarget, DeploymentStatus};
use monitoring::{HealthMonitor, HealthStatus, Metric};
use release_notes::{ReleaseNotesGenerator, ReleaseInfo};
use rollback::{RollbackManager, RollbackPlan, RollbackStatus};

/// Advanced Build & Deployment Engine
/// 
/// Coordinates all build and deployment operations including cross-compilation,
/// release management, deployment automation, rollback capabilities, and health monitoring.
#[derive(Clone)]
pub struct AdvancedBuildEngine {
    /// Cross-compilation engine
    cross_compile: Arc<CrossCompileEngine>,
    /// Release notes generator
    release_notes: Arc<ReleaseNotesGenerator>,
    /// Deployment engine
    deployment: Arc<DeploymentEngine>,
    /// Rollback manager
    rollback: Arc<RollbackManager>,
    /// Health monitor
    monitoring: Arc<HealthMonitor>,
    /// Build history
    build_history: Arc<RwLock<Vec<BuildRecord>>>,
    /// Deployment history
    deployment_history: Arc<RwLock<Vec<DeploymentRecord>>>,
    /// Configuration
    config: BuildConfig,
}

/// Build record for tracking build operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRecord {
    /// Build ID
    pub id: String,
    /// Version
    pub version: String,
    /// Platform
    pub platform: Platform,
    /// Build type
    pub build_type: BuildType,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Status
    pub status: BuildStatus,
    /// Duration in seconds
    pub duration: Option<u64>,
    /// Output path
    pub output_path: Option<PathBuf>,
    /// Error message if failed
    pub error: Option<String>,
    /// Commit hash
    pub commit_hash: Option<String>,
    /// Branch name
    pub branch: Option<String>,
}

/// Deployment record for tracking deployment operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRecord {
    /// Deployment ID
    pub id: String,
    /// Version
    pub version: String,
    /// Target
    pub target: DeploymentTarget,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time: Option<DateTime<Utc>>,
    /// Status
    pub status: DeploymentStatus,
    /// Duration in seconds
    pub duration: Option<u64>,
    /// Error message if failed
    pub error: Option<String>,
    /// Rollback available
    pub rollback_available: bool,
}

/// Build type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildType {
    /// Debug build
    Debug,
    /// Release build
    Release,
    /// Profile-guided optimization build
    Profile,
    /// Benchmark build
    Benchmark,
}

/// Build status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildStatus {
    /// Build is pending
    Pending,
    /// Build is in progress
    InProgress,
    /// Build completed successfully
    Success,
    /// Build failed
    Failed,
    /// Build was cancelled
    Cancelled,
}

impl AdvancedBuildEngine {
    /// Create a new advanced build engine
    pub async fn new(config: BuildConfig) -> Result<Self> {
        info!("Initializing Advanced Build Engine");

        let cross_compile = Arc::new(CrossCompileEngine::new(config.clone()).await?);
        let release_notes = Arc::new(ReleaseNotesGenerator::new().await?);
        let deployment = Arc::new(DeploymentEngine::new().await?);
        let rollback = Arc::new(RollbackManager::new().await?);
        let monitoring = Arc::new(HealthMonitor::new().await?);

        Ok(Self {
            cross_compile,
            release_notes,
            deployment,
            rollback,
            monitoring,
            build_history: Arc::new(RwLock::new(Vec::new())),
            deployment_history: Arc::new(RwLock::new(Vec::new())),
            config,
        })
    }

    /// Build for a specific platform
    pub async fn build_platform(
        &self,
        platform: Platform,
        build_type: BuildType,
    ) -> Result<BuildRecord> {
        info!("Starting build for platform: {:?}", platform);

        let build_id = uuid::Uuid::new_v4().to_string();
        let version = self.get_version().await?;
        let commit_hash = self.get_commit_hash().await?;
        let branch = self.get_branch_name().await?;

        let mut record = BuildRecord {
            id: build_id.clone(),
            version: version.clone(),
            platform,
            build_type,
            start_time: Utc::now(),
            end_time: None,
            status: BuildStatus::InProgress,
            duration: None,
            output_path: None,
            error: None,
            commit_hash: Some(commit_hash),
            branch: Some(branch),
        };

        // Add to history
        self.build_history.write().await.push(record.clone());

        // Perform build
        let result = self.cross_compile.build(platform, build_type).await;

        let end_time = Utc::now();
        let duration = (end_time - record.start_time).num_seconds() as u64;

        match result {
            Ok(output_path) => {
                record.end_time = Some(end_time);
                record.duration = Some(duration);
                record.status = BuildStatus::Success;
                record.output_path = Some(output_path);

                info!("Build completed successfully in {}s", duration);
            }
            Err(e) => {
                record.end_time = Some(end_time);
                record.duration = Some(duration);
                record.status = BuildStatus::Failed;
                record.error = Some(e.to_string());

                error!("Build failed: {}", e);
            }
        }

        // Update history
        let mut history = self.build_history.write().await;
        if let Some(last) = history.last_mut() {
            *last = record.clone();
        }

        Ok(record)
    }

    /// Build for all platforms
    pub async fn build_all_platforms(&self, build_type: BuildType) -> Result<Vec<BuildRecord>> {
        info!("Starting build for all platforms");

        let platforms = vec![
            Platform::LinuxX86_64,
            Platform::LinuxAarch64,
            Platform::WindowsX86_64,
            Platform::MacOSX86_64,
            Platform::MacOSAarch64,
        ];

        let mut results = Vec::new();

        for platform in platforms {
            let record = self.build_platform(platform, build_type).await?;
            results.push(record);
        }

        Ok(results)
    }

    /// Deploy to a target
    pub async fn deploy(
        &self,
        version: &str,
        target: DeploymentTarget,
    ) -> Result<DeploymentRecord> {
        info!("Deploying version {} to {:?}", version, target);

        let deployment_id = uuid::Uuid::new_v4().to_string();

        let mut record = DeploymentRecord {
            id: deployment_id.clone(),
            version: version.to_string(),
            target: target.clone(),
            start_time: Utc::now(),
            end_time: None,
            status: DeploymentStatus::InProgress,
            duration: None,
            error: None,
            rollback_available: false,
        };

        // Add to history
        self.deployment_history.write().await.push(record.clone());

        // Perform deployment
        let result = self.deployment.deploy(version, target).await;

        let end_time = Utc::now();
        let duration = (end_time - record.start_time).num_seconds() as u64;

        match result {
            Ok(_) => {
                record.end_time = Some(end_time);
                record.duration = Some(duration);
                record.status = DeploymentStatus::Success;
                record.rollback_available = true;

                info!("Deployment completed successfully in {}s", duration);
            }
            Err(e) => {
                record.end_time = Some(end_time);
                record.duration = Some(duration);
                record.status = DeploymentStatus::Failed;
                record.error = Some(e.to_string());

                error!("Deployment failed: {}", e);
            }
        }

        // Update history
        let mut history = self.deployment_history.write().await;
        if let Some(last) = history.last_mut() {
            *last = record.clone();
        }

        Ok(record)
    }

    /// Rollback a deployment
    pub async fn rollback(&self, deployment_id: &str) -> Result<RollbackStatus> {
        info!("Rolling back deployment: {}", deployment_id);

        let plan = self.rollback.create_rollback_plan(deployment_id).await?;
        self.rollback.execute_rollback(plan).await
    }

    /// Generate release notes
    pub async fn generate_release_notes(&self, version: &str) -> Result<String> {
        info!("Generating release notes for version: {}", version);

        let release_info = ReleaseInfo {
            version: version.to_string(),
            release_date: Utc::now(),
            commit_hash: self.get_commit_hash().await?,
            branch: self.get_branch_name().await?,
        };

        self.release_notes.generate(release_info).await
    }

    /// Get build history
    pub async fn get_build_history(&self) -> Vec<BuildRecord> {
        self.build_history.read().await.clone()
    }

    /// Get deployment history
    pub async fn get_deployment_history(&self) -> Vec<DeploymentRecord> {
        self.deployment_history.read().await.clone()
    }

    /// Get health status
    pub async fn get_health_status(&self) -> HealthStatus {
        self.monitoring.get_status().await
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> Vec<Metric> {
        self.monitoring.get_metrics().await
    }

    /// Get current version
    async fn get_version(&self) -> Result<String> {
        // Read from Cargo.toml
        let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
        let value: toml::Value = toml::from_str(&cargo_toml)?;
        
        Ok(value
            .get("workspace")
            .and_then(|w| w.get("package"))
            .and_then(|p| p.get("version"))
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0")
            .to_string())
    }

    /// Get current commit hash
    async fn get_commit_hash(&self) -> Result<String> {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// Get current branch name
    async fn get_branch_name(&self) -> Result<String> {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok("unknown".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_build_engine_creation() {
        let config = BuildConfig::default();
        let engine = AdvancedBuildEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_build_record_serialization() {
        let record = BuildRecord {
            id: "test-id".to_string(),
            version: "1.0.0".to_string(),
            platform: Platform::LinuxX86_64,
            build_type: BuildType::Release,
            start_time: Utc::now(),
            end_time: None,
            status: BuildStatus::Pending,
            duration: None,
            output_path: None,
            error: None,
            commit_hash: None,
            branch: None,
        };

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: BuildRecord = serde_json::from_str(&serialized).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.version, deserialized.version);
    }

    #[tokio::test]
    async fn test_deployment_record_serialization() {
        let record = DeploymentRecord {
            id: "test-id".to_string(),
            version: "1.0.0".to_string(),
            target: DeploymentTarget::Production,
            start_time: Utc::now(),
            end_time: None,
            status: DeploymentStatus::InProgress,
            duration: None,
            error: None,
            rollback_available: false,
        };

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: DeploymentRecord = serde_json::from_str(&serialized).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.version, deserialized.version);
    }
}