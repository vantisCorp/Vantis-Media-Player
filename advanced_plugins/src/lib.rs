//! Vantis Advanced Plugin System
//! 
//! This module provides advanced plugin management capabilities including:
//! - Plugin marketplace with search and installation
//! - Dependency management and resolution
//! - Enhanced sandbox with fine-grained permissions
//! - Hot-reload with state preservation
//! - Performance monitoring and profiling
//! - Plugin lifecycle management

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, debug, warn, error};

pub mod marketplace;
pub mod dependencies;
pub mod sandbox;
pub mod hot_reload;
pub mod monitoring;
pub mod lifecycle;
pub mod utils;

use marketplace::PluginMarketplace;
use dependencies::DependencyManager;
use sandbox::EnhancedSandbox;
use hot_reload::HotReloadManager;
use monitoring::PerformanceMonitor;
use lifecycle::LifecycleManager;

/// Advanced plugin manager
pub struct AdvancedPluginManager {
    /// Plugin marketplace
    marketplace: PluginMarketplace,
    
    /// Dependency manager
    dependency_manager: DependencyManager,
    
    /// Enhanced sandbox
    sandbox: EnhancedSandbox,
    
    /// Hot reload manager
    hot_reload: HotReloadManager,
    
    /// Performance monitor
    monitor: PerformanceMonitor,
    
    /// Lifecycle manager
    lifecycle: LifecycleManager,
    
    /// Plugin directory
    plugin_dir: PathBuf,
    
    /// Cache directory
    cache_dir: PathBuf,
    
    /// Configuration
    config: AdvancedPluginConfig,
}

/// Advanced plugin configuration
#[derive(Clone, Debug)]
pub struct AdvancedPluginConfig {
    /// Enable marketplace
    pub enable_marketplace: bool,
    
    /// Marketplace URL
    pub marketplace_url: String,
    
    /// Enable hot reload
    pub enable_hot_reload: bool,
    
    /// Hot reload interval in seconds
    pub hot_reload_interval: u64,
    
    /// Enable monitoring
    pub enable_monitoring: bool,
    
    /// Monitoring interval in seconds
    pub monitoring_interval: u64,
    
    /// Enable sandbox
    pub enable_sandbox: bool,
    
    /// Sandbox timeout in seconds
    pub sandbox_timeout: u64,
    
    /// Maximum memory per plugin (MB)
    pub max_plugin_memory: usize,
    
    /// Enable auto-update
    pub enable_auto_update: bool,
    
    /// Auto-update check interval in hours
    pub auto_update_interval: u64,
}

impl Default for AdvancedPluginConfig {
    fn default() -> Self {
        Self {
            enable_marketplace: true,
            marketplace_url: "https://plugins.vantis.io".to_string(),
            enable_hot_reload: true,
            hot_reload_interval: 5,
            enable_monitoring: true,
            monitoring_interval: 10,
            enable_sandbox: true,
            sandbox_timeout: 30,
            max_plugin_memory: 512,
            enable_auto_update: true,
            auto_update_interval: 24,
        }
    }
}

impl AdvancedPluginManager {
    /// Create a new advanced plugin manager
    pub fn new(plugin_dir: PathBuf, cache_dir: PathBuf) -> Result<Self> {
        Self::with_config(plugin_dir, cache_dir, AdvancedPluginConfig::default())
    }
    
    /// Create with custom configuration
    pub fn with_config(
        plugin_dir: PathBuf,
        cache_dir: PathBuf,
        config: AdvancedPluginConfig,
    ) -> Result<Self> {
        info!("🔧 Initializing Advanced Plugin System");
        
        // Create directories if they don't exist
        std::fs::create_dir_all(&plugin_dir)?;
        std::fs::create_dir_all(&cache_dir)?;
        
        // Initialize subsystems
        let marketplace = if config.enable_marketplace {
            PluginMarketplace::new(&config.marketplace_url)?
        } else {
            PluginMarketplace::disabled()
        };
        
        let dependency_manager = DependencyManager::new(cache_dir.clone())?;
        let sandbox = EnhancedSandbox::new(config.max_plugin_memory, config.sandbox_timeout)?;
        let hot_reload = HotReloadManager::new(plugin_dir.clone(), config.hot_reload_interval)?;
        let monitor = PerformanceMonitor::new(config.monitoring_interval)?;
        let lifecycle = LifecycleManager::new(plugin_dir.clone())?;
        
        info!("✅ Advanced plugin system initialized");
        info!("   - Marketplace: {}", if config.enable_marketplace { "Enabled" } else { "Disabled" });
        info!("   - Hot Reload: {}", if config.enable_hot_reload { "Enabled" } else { "Disabled" });
        info!("   - Monitoring: {}", if config.enable_monitoring { "Enabled" } else { "Disabled" });
        info!("   - Sandbox: {}", if config.enable_sandbox { "Enabled" } else { "Disabled" });
        info!("   - Auto-update: {}", if config.enable_auto_update { "Enabled" } else { "Disabled" });
        
        Ok(Self {
            marketplace,
            dependency_manager,
            sandbox,
            hot_reload,
            monitor,
            lifecycle,
            plugin_dir,
            cache_dir,
            config,
        })
    }
    
    /// Get marketplace reference
    pub fn marketplace(&self) -> &PluginMarketplace {
        &self.marketplace
    }
    
    /// Get dependency manager reference
    pub fn dependency_manager(&self) -> &DependencyManager {
        &self.dependency_manager
    }
    
    /// Get sandbox reference
    pub fn sandbox(&self) -> &EnhancedSandbox {
        &self.sandbox
    }
    
    /// Get hot reload manager reference
    pub fn hot_reload(&self) -> &HotReloadManager {
        &self.hot_reload
    }
    
    /// Get performance monitor reference
    pub fn monitor(&self) -> &PerformanceMonitor {
        &self.monitor
    }
    
    /// Get lifecycle manager reference
    pub fn lifecycle(&self) -> &LifecycleManager {
        &self.lifecycle
    }
    
    /// Get configuration
    pub fn config(&self) -> &AdvancedPluginConfig {
        &self.config
    }
    
    /// Get plugin directory
    pub fn plugin_dir(&self) -> &PathBuf {
        &self.plugin_dir
    }
    
    /// Get cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }
    
    /// Start all subsystems
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting Advanced Plugin System");
        
        // Start hot reload if enabled
        if self.config.enable_hot_reload {
            self.hot_reload.start().await?;
        }
        
        // Start monitoring if enabled
        if self.config.enable_monitoring {
            self.monitor.start().await?;
        }
        
        // Start auto-update if enabled
        if self.config.enable_auto_update {
            self.start_auto_update().await?;
        }
        
        info!("✅ Advanced plugin system started");
        
        Ok(())
    }
    
    /// Stop all subsystems
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Advanced Plugin System");
        
        // Stop hot reload
        self.hot_reload.stop().await?;
        
        // Stop monitoring
        self.monitor.stop().await?;
        
        info!("✅ Advanced plugin system stopped");
        
        Ok(())
    }
    
    /// Start auto-update task
    async fn start_auto_update(&self) -> Result<()> {
        let interval = self.config.auto_update_interval;
        let marketplace = self.marketplace.clone();
        let lifecycle = self.lifecycle.clone();
        
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(tokio::time::Duration::from_secs(interval * 3600));
            
            loop {
                timer.tick().await;
                
                if let Err(e) = marketplace.check_updates().await {
                    error!("Auto-update check failed: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Get system status
    pub fn get_status(&self) -> AdvancedPluginStatus {
        AdvancedPluginStatus {
            marketplace_enabled: self.config.enable_marketplace,
            hot_reload_enabled: self.config.enable_hot_reload,
            monitoring_enabled: self.config.enable_monitoring,
            sandbox_enabled: self.config.enable_sandbox,
            auto_update_enabled: self.config.enable_auto_update,
            hot_reload_active: self.hot_reload.is_active(),
            monitoring_active: self.monitor.is_active(),
            plugin_count: self.lifecycle.get_plugin_count(),
        }
    }
}

/// Advanced plugin system status
#[derive(Clone, Debug)]
pub struct AdvancedPluginStatus {
    /// Marketplace enabled
    pub marketplace_enabled: bool,
    
    /// Hot reload enabled
    pub hot_reload_enabled: bool,
    
    /// Monitoring enabled
    pub monitoring_enabled: bool,
    
    /// Sandbox enabled
    pub sandbox_enabled: bool,
    
    /// Auto-update enabled
    pub auto_update_enabled: bool,
    
    /// Hot reload active
    pub hot_reload_active: bool,
    
    /// Monitoring active
    pub monitoring_active: bool,
    
    /// Plugin count
    pub plugin_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_advanced_plugin_manager_creation() {
        let plugin_dir = TempDir::new().unwrap();
        let cache_dir = TempDir::new().unwrap();
        
        let manager = AdvancedPluginManager::new(
            plugin_dir.path().to_path_buf(),
            cache_dir.path().to_path_buf(),
        );
        
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_default_config() {
        let config = AdvancedPluginConfig::default();
        
        assert!(config.enable_marketplace);
        assert!(config.enable_hot_reload);
        assert!(config.enable_monitoring);
        assert!(config.enable_sandbox);
        assert!(config.enable_auto_update);
        assert_eq!(config.hot_reload_interval, 5);
        assert_eq!(config.max_plugin_memory, 512);
    }
    
    #[test]
    fn test_custom_config() {
        let config = AdvancedPluginConfig {
            enable_marketplace: false,
            enable_hot_reload: false,
            hot_reload_interval: 10,
            max_plugin_memory: 1024,
            ..Default::default()
        };
        
        assert!(!config.enable_marketplace);
        assert!(!config.enable_hot_reload);
        assert_eq!(config.hot_reload_interval, 10);
        assert_eq!(config.max_plugin_memory, 1024);
    }
}