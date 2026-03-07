//! Plugin Auto-Update System
//!
//! Handles automatic plugin updates, version checking, background updates,
//! rollback support, and update notifications.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};

use super::marketplace::{PluginMarketplace, PluginListing};

// ============================================================================
// Update Types
// ============================================================================

/// Plugin update status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatus {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Current installed version
    pub current_version: String,
    
    /// Latest available version
    pub latest_version: String,
    
    /// Whether an update is available
    pub update_available: bool,
    
    /// Update type
    pub update_type: UpdateType,
    
    /// Changelog for the update
    pub changelog: Option<String>,
    
    /// Whether the update is security-related
    pub security_update: bool,
}

/// Type of update
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateType {
    /// Major version change (breaking changes)
    Major,
    
    /// Minor version change (new features)
    Minor,
    
    /// Patch version change (bug fixes)
    Patch,
    
    /// Pre-release version
    PreRelease,
    
    /// Build metadata change
    Build,
    
    /// No update available
    None,
}

/// Update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    /// Enable automatic updates
    pub auto_update_enabled: bool,
    
    /// Check for updates automatically
    pub auto_check_enabled: bool,
    
    /// Check interval in seconds
    pub check_interval_seconds: u64,
    
    /// Download updates automatically
    pub auto_download: bool,
    
    /// Install updates automatically
    pub auto_install: bool,
    
    /// Include pre-release versions
    pub include_prerelease: bool,
    
    /// Maximum rollback versions to keep
    pub max_rollback_versions: u32,
    
    /// Update channel
    pub channel: UpdateChannel,
}

/// Update channel
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateChannel {
    /// Stable releases only
    Stable,
    
    /// Beta releases
    Beta,
    
    /// Alpha/development releases
    Alpha,
    
    /// Nightly builds
    Nightly,
}

/// Update check result
#[derive(Debug, Clone)]
pub struct UpdateCheckResult {
    /// Time of check
    pub checked_at: chrono::DateTime<chrono::Utc>,
    
    /// Available updates
    pub updates: Vec<UpdateStatus>,
    
    /// Errors during check
    pub errors: Vec<UpdateCheckError>,
}

/// Update check error
#[derive(Debug, Clone)]
pub struct UpdateCheckError {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Error message
    pub error: String,
}

/// Rollback information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackInfo {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Version to rollback to
    pub version: String,
    
    /// Path to the backup
    pub backup_path: PathBuf,
    
    /// When the backup was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Update notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotification {
    /// Notification ID
    pub id: String,
    
    /// Plugin ID
    pub plugin_id: String,
    
    /// Plugin name
    pub plugin_name: String,
    
    /// Update type
    pub update_type: UpdateType,
    
    /// Current version
    pub current_version: String,
    
    /// New version
    pub new_version: String,
    
    /// Security update flag
    pub security_update: bool,
    
    /// Changelog summary
    pub changelog_summary: String,
    
    /// When the notification was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Whether the notification was read
    pub read: bool,
}

// ============================================================================
// Auto-Updater
// ============================================================================

/// Plugin auto-updater
pub struct PluginAutoUpdater {
    /// Marketplace client
    marketplace: Arc<PluginMarketplace>,
    
    /// Update configuration
    config: UpdateConfig,
    
    /// Installed plugins
    installed_plugins: HashMap<String, InstalledPlugin>,
    
    /// Rollback history
    rollbacks: HashMap<String, Vec<RollbackInfo>>,
    
    /// Pending notifications
    notifications: Arc<RwLock<Vec<UpdateNotification>>>,
    
    /// Plugin installation directory
    install_dir: PathBuf,
    
    /// Backup directory
    backup_dir: PathBuf,
}

/// Installed plugin info
#[derive(Debug, Clone)]
pub struct InstalledPlugin {
    /// Plugin ID
    pub id: String,
    
    /// Plugin name
    pub name: String,
    
    /// Installed version
    pub version: String,
    
    /// Installation path
    pub path: PathBuf,
    
    /// Last update check
    pub last_check: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Auto-update enabled for this plugin
    pub auto_update: bool,
}

// ============================================================================
// Implementation
// ============================================================================

impl PluginAutoUpdater {
    /// Create a new auto-updater
    pub fn new(
        marketplace: Arc<PluginMarketplace>,
        config: UpdateConfig,
        install_dir: &Path,
        backup_dir: &Path,
    ) -> Self {
        Self {
            marketplace,
            config,
            installed_plugins: HashMap::new(),
            rollbacks: HashMap::new(),
            notifications: Arc::new(RwLock::new(Vec::new())),
            install_dir: install_dir.to_path_buf(),
            backup_dir: backup_dir.to_path_buf(),
        }
    }
    
    /// Register an installed plugin for updates
    pub fn register_plugin(&mut self, plugin: InstalledPlugin) {
        info!("Registering plugin for updates: {}@{}", plugin.id, plugin.version);
        self.installed_plugins.insert(plugin.id.clone(), plugin);
    }
    
    /// Unregister a plugin from updates
    pub fn unregister_plugin(&mut self, plugin_id: &str) {
        info!("Unregistering plugin from updates: {}", plugin_id);
        self.installed_plugins.remove(plugin_id);
    }
    
    /// Check for updates for all registered plugins
    pub fn check_for_updates(&self) -> Result<UpdateCheckResult> {
        info!("Checking for plugin updates...");
        
        let mut result = UpdateCheckResult {
            checked_at: chrono::Utc::now(),
            updates: Vec::new(),
            errors: Vec::new(),
        };
        
        for (plugin_id, plugin) in &self.installed_plugins {
            match self.check_plugin_update(plugin_id, &plugin.version) {
                Ok(Some(status)) => {
                    if status.update_available {
                        result.updates.push(status);
                    }
                }
                Ok(None) => {
                    debug!("No update available for {}", plugin_id);
                }
                Err(e) => {
                    warn!("Failed to check update for {}: {}", plugin_id, e);
                    result.errors.push(UpdateCheckError {
                        plugin_id: plugin_id.clone(),
                        error: e.to_string(),
                    });
                }
            }
        }
        
        info!("Update check complete: {} updates available", result.updates.len());
        Ok(result)
    }
    
    /// Check for update for a specific plugin
    fn check_plugin_update(&self, plugin_id: &str, current_version: &str) -> Result<Option<UpdateStatus>> {
        let listing = self.marketplace.get_plugin(plugin_id)?;
        
        let latest_version = &listing.latest_version;
        
        if latest_version == current_version {
            return Ok(None);
        }
        
        // Determine update type
        let update_type = Self::compare_versions(current_version, latest_version)?;
        
        // Get changelog
        let changelog = listing.versions.iter()
            .find(|v| &v.version == latest_version)
            .and_then(|v| v.changelog.clone());
        
        Ok(Some(UpdateStatus {
            plugin_id: plugin_id.to_string(),
            current_version: current_version.to_string(),
            latest_version: latest_version.clone(),
            update_available: true,
            update_type,
            changelog,
            security_update: false, // Would be determined by metadata
        }))
    }
    
    /// Compare two versions to determine update type
    fn compare_versions(current: &str, latest: &str) -> Result<UpdateType> {
        let current_parts: Vec<u32> = current
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let latest_parts: Vec<u32> = latest
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if current_parts.len() < 2 || latest_parts.len() < 2 {
            return Ok(UpdateType::None);
        }
        
        // Major version changed
        if latest_parts[0] > current_parts[0] {
            return Ok(UpdateType::Major);
        }
        
        // Minor version changed
        if latest_parts[1] > current_parts.get(1).unwrap_or(&0) {
            return Ok(UpdateType::Minor);
        }
        
        // Patch version changed
        if latest_parts.len() > 2 && current_parts.len() > 2 {
            if latest_parts[2] > current_parts[2] {
                return Ok(UpdateType::Patch);
            }
        }
        
        Ok(UpdateType::None)
    }
    
    /// Update a plugin to the latest version
    pub fn update_plugin(&mut self, plugin_id: &str) -> Result<()> {
        info!("Updating plugin: {}", plugin_id);
        
        let plugin = self.installed_plugins.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin {} not found", plugin_id))?;
        
        // Create backup for rollback
        self.create_backup(plugin_id, &plugin.version)?;
        
        // Install update
        self.marketplace.install_plugin(plugin_id, None, Some(Box::new(|progress| {
            debug!("Update progress: {:?} - {}", progress.phase, progress.message);
        })))?;
        
        // Update installed version
        if let Some(plugin) = self.installed_plugins.get_mut(plugin_id) {
            // Get new version from marketplace
            if let Ok(listing) = self.marketplace.get_plugin(plugin_id) {
                plugin.version = listing.latest_version;
            }
        }
        
        info!("Plugin {} updated successfully", plugin_id);
        Ok(())
    }
    
    /// Update all plugins with available updates
    pub fn update_all(&mut self) -> Result<Vec<(String, Result<()>)>> {
        info!("Updating all plugins...");
        
        let check_result = self.check_for_updates()?;
        let mut results = Vec::new();
        
        for update in check_result.updates {
            let plugin_id = update.plugin_id.clone();
            let result = self.update_plugin(&plugin_id);
            results.push((plugin_id, result));
        }
        
        Ok(results)
    }
    
    /// Rollback a plugin to a previous version
    pub fn rollback(&mut self, plugin_id: &str) -> Result<()> {
        info!("Rolling back plugin: {}", plugin_id);
        
        let rollbacks = self.rollbacks.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("No rollback available for {}", plugin_id))?;
        
        let rollback_info = rollbacks.last()
            .ok_or_else(|| anyhow::anyhow!("No rollback versions available"))?;
        
        // Restore from backup
        let plugin_dir = self.install_dir.join(plugin_id);
        
        // Remove current version
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)?;
        }
        
        // Restore backup
        std::fs::rename(&rollback_info.backup_path, &plugin_dir)?;
        
        // Update installed version
        if let Some(plugin) = self.installed_plugins.get_mut(plugin_id) {
            plugin.version = rollback_info.version.clone();
        }
        
        // Remove used rollback info
        self.rollbacks.get_mut(plugin_id).unwrap().pop();
        
        info!("Plugin {} rolled back to {}", plugin_id, rollback_info.version);
        Ok(())
    }
    
    /// Create a backup for rollback
    fn create_backup(&mut self, plugin_id: &str, version: &str) -> Result<()> {
        let plugin_dir = self.install_dir.join(plugin_id);
        
        if !plugin_dir.exists() {
            return Ok(());
        }
        
        std::fs::create_dir_all(&self.backup_dir)?;
        
        let backup_path = self.backup_dir.join(format!("{}-{}-{}", 
            plugin_id, 
            version, 
            chrono::Utc::now().format("%Y%m%d%H%M%S")
        ));
        
        // Copy plugin directory to backup
        self.copy_dir_all(&plugin_dir, &backup_path)?;
        
        let rollback_info = RollbackInfo {
            plugin_id: plugin_id.to_string(),
            version: version.to_string(),
            backup_path: backup_path.clone(),
            created_at: chrono::Utc::now(),
        };
        
        self.rollbacks
            .entry(plugin_id.to_string())
            .or_default()
            .push(rollback_info);
        
        // Enforce max rollback versions
        if let Some(rollbacks) = self.rollbacks.get_mut(plugin_id) {
            while rollbacks.len() > self.config.max_rollback_versions as usize {
                let old = rollbacks.remove(0);
                if old.backup_path.exists() {
                    let _ = std::fs::remove_dir_all(&old.backup_path);
                }
            }
        }
        
        debug!("Created backup at {:?}", backup_path);
        Ok(())
    }
    
    /// Recursively copy a directory
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;
        
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            
            if ty.is_dir() {
                self.copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
        
        Ok(())
    }
    
    /// Get pending notifications
    pub fn get_notifications(&self) -> Vec<UpdateNotification> {
        self.notifications.read().clone()
    }
    
    /// Mark notification as read
    pub fn mark_notification_read(&self, notification_id: &str) {
        let mut notifications = self.notifications.write();
        if let Some(n) = notifications.iter_mut().find(|n| n.id == notification_id) {
            n.read = true;
        }
    }
    
    /// Clear all notifications
    pub fn clear_notifications(&self) {
        self.notifications.write().clear();
    }
    
    /// Get available rollbacks for a plugin
    pub fn get_rollbacks(&self, plugin_id: &str) -> Vec<&RollbackInfo> {
        self.rollbacks
            .get(plugin_id)
            .map(|r| r.iter().collect())
            .unwrap_or_default()
    }
    
    /// Enable/disable auto-update for a specific plugin
    pub fn set_plugin_auto_update(&mut self, plugin_id: &str, enabled: bool) {
        if let Some(plugin) = self.installed_plugins.get_mut(plugin_id) {
            plugin.auto_update = enabled;
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            auto_update_enabled: true,
            auto_check_enabled: true,
            check_interval_seconds: 3600, // 1 hour
            auto_download: false,
            auto_install: false,
            include_prerelease: false,
            max_rollback_versions: 3,
            channel: UpdateChannel::Stable,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_update_config_default() {
        let config = UpdateConfig::default();
        assert!(config.auto_update_enabled);
        assert!(config.auto_check_enabled);
        assert_eq!(config.check_interval_seconds, 3600);
    }
    
    #[test]
    fn test_version_comparison() {
        assert_eq!(
            PluginAutoUpdater::compare_versions("1.0.0", "2.0.0").unwrap(),
            UpdateType::Major
        );
        
        assert_eq!(
            PluginAutoUpdater::compare_versions("1.0.0", "1.1.0").unwrap(),
            UpdateType::Minor
        );
        
        assert_eq!(
            PluginAutoUpdater::compare_versions("1.0.0", "1.0.1").unwrap(),
            UpdateType::Patch
        );
        
        assert_eq!(
            PluginAutoUpdater::compare_versions("1.0.0", "1.0.0").unwrap(),
            UpdateType::None
        );
    }
    
    #[test]
    fn test_update_type_ordering() {
        assert!(UpdateType::Major != UpdateType::Minor);
        assert!(UpdateType::Minor != UpdateType::Patch);
    }
}