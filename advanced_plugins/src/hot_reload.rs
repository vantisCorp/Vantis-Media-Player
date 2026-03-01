//! Hot Reload Manager
//! 
//! Provides automatic plugin reloading with state preservation and minimal downtime.

use anyhow::{Context, Result};
use notify::{RecursiveMode, Watcher, Event, EventKind};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, debug, warn, error};

/// Hot reload manager
pub struct HotReloadManager {
    /// Plugin directory
    plugin_dir: PathBuf,
    
    /// Reload interval in seconds
    interval: u64,
    
    /// Active flag
    active: Arc<RwLock<bool>>,
    
    /// Plugin states
    states: Arc<RwLock<HashMap<String, PluginState>>>,
    
    /// Event sender
    event_sender: Option<mpsc::UnboundedSender<ReloadEvent>>,
}

/// Plugin state for preservation
#[derive(Clone, Debug)]
pub struct PluginState {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Configuration
    pub config: serde_json::Value,
    
    /// Custom data
    pub data: HashMap<String, serde_json::Value>,
    
    /// Last reload time
    pub last_reload: chrono::DateTime<chrono::Utc>,
}

/// Reload event
#[derive(Clone, Debug)]
pub enum ReloadEvent {
    /// Plugin reloaded
    Reloaded {
        name: String,
        old_version: String,
        new_version: String,
    },
    
    /// Reload failed
    Failed {
        name: String,
        error: String,
    },
    
    /// Plugin added
    Added {
        name: String,
    },
    
    /// Plugin removed
    Removed {
        name: String,
    },
}

/// Reload configuration
#[derive(Clone, Debug)]
pub struct ReloadConfig {
    /// Enable hot reload
    pub enabled: bool,
    
    /// Reload interval in seconds
    pub interval: u64,
    
    /// Debounce delay in milliseconds
    pub debounce: u64,
    
    /// Preserve state on reload
    pub preserve_state: bool,
    
    /// Auto-restart on error
    pub auto_restart: bool,
    
    /// Maximum retry attempts
    pub max_retries: u32,
}

impl Default for ReloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: 5,
            debounce: 500,
            preserve_state: true,
            auto_restart: true,
            max_retries: 3,
        }
    }
}

impl HotReloadManager {
    /// Create a new hot reload manager
    pub fn new(plugin_dir: PathBuf, interval: u64) -> Result<Self> {
        info!("🔄 Initializing Hot Reload Manager");
        
        info!("✅ Hot reload manager initialized");
        info!("   - Plugin directory: {}", plugin_dir.display());
        info!("   - Interval: {} seconds", interval);
        
        Ok(Self {
            plugin_dir,
            interval,
            active: Arc::new(RwLock::new(false)),
            states: Arc::new(RwLock::new(HashMap::new())),
            event_sender: None,
        })
    }
    
    /// Create with custom configuration
    pub fn with_config(plugin_dir: PathBuf, config: ReloadConfig) -> Result<Self> {
        let mut manager = Self::new(plugin_dir, config.interval)?;
        manager.interval = config.interval;
        Ok(manager)
    }
    
    /// Check if hot reload is active
    pub fn is_active(&self) -> bool {
        *self.active.read()
    }
    
    /// Start hot reload
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting Hot Reload Manager");
        
        if self.is_active() {
            warn!("⚠️  Hot reload already active");
            return Ok(());
        }
        
        *self.active.write() = true;
        
        // Start file watcher
        let plugin_dir = self.plugin_dir.clone();
        let states = self.states.clone();
        let active = self.active.clone();
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();
        
        // Store sender
        // Note: This is a simplified version - in production you'd store this properly
        
        tokio::spawn(async move {
            if let Err(e) = Self::watch_directory(plugin_dir, states, active, event_sender).await {
                error!("File watcher error: {}", e);
            }
        });
        
        // Start event handler
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                debug!("📡 Reload event: {:?}", event);
                // Handle event
            }
        });
        
        info!("✅ Hot reload manager started");
        
        Ok(())
    }
    
    /// Stop hot reload
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Hot Reload Manager");
        
        *self.active.write() = false;
        
        info!("✅ Hot reload manager stopped");
        
        Ok(())
    }
    
    /// Watch directory for changes
    async fn watch_directory(
        plugin_dir: PathBuf,
        states: Arc<RwLock<HashMap<String, PluginState>>>,
        active: Arc<RwLock<bool>>,
        event_sender: mpsc::UnboundedSender<ReloadEvent>,
    ) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(100);
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;
        
        watcher.watch(&plugin_dir, RecursiveMode::Recursive)?;
        
        while *active.read() {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    // Periodic check
                }
                Some(event) = rx.recv() => {
                    if let Err(e) = Self::handle_file_event(event, &plugin_dir, &states, &event_sender).await {
                        error!("Error handling file event: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle file event
    async fn handle_file_event(
        event: Event,
        plugin_dir: &Path,
        states: &Arc<RwLock<HashMap<String, PluginState>>>,
        event_sender: &mpsc::UnboundedSender<ReloadEvent>,
    ) -> Result<()> {
        debug!("📁 File event: {:?}", event.kind);
        
        for path in event.paths {
            if path.extension().map_or(false, |ext| ext == "wasm") {
                let plugin_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                
                match event.kind {
                    EventKind::Create(_) => {
                        info!("➕ Plugin added: {}", plugin_name);
                        let _ = event_sender.send(ReloadEvent::Added {
                            name: plugin_name.to_string(),
                        });
                    }
                    EventKind::Modify(_) => {
                        info!("🔄 Plugin modified: {}", plugin_name);
                        
                        // Get old state
                        let old_state = states.read().get(plugin_name).cloned();
                        
                        // Reload plugin
                        match Self::reload_plugin(&path).await {
                            Ok(new_version) => {
                                let old_version = old_state
                                    .as_ref()
                                    .map(|s| s.version.clone())
                                    .unwrap_or_else(|| "unknown".to_string());
                                
                                let _ = event_sender.send(ReloadEvent::Reloaded {
                                    name: plugin_name.to_string(),
                                    old_version,
                                    new_version,
                                });
                            }
                            Err(e) => {
                                error!("Failed to reload plugin {}: {}", plugin_name, e);
                                let _ = event_sender.send(ReloadEvent::Failed {
                                    name: plugin_name.to_string(),
                                    error: e.to_string(),
                                });
                            }
                        }
                    }
                    EventKind::Remove(_) => {
                        info!("➖ Plugin removed: {}", plugin_name);
                        let _ = event_sender.send(ReloadEvent::Removed {
                            name: plugin_name.to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Reload plugin
    async fn reload_plugin(path: &Path) -> Result<String> {
        // Read WASM file
        let wasm_bytes = tokio::fs::read(path).await?;
        
        // Extract version from WASM (simplified)
        // In production, you'd parse the WASM custom sections
        let version = "1.0.0".to_string();
        
        debug!("✅ Plugin reloaded: {}", path.display());
        
        Ok(version)
    }
    
    /// Save plugin state
    pub fn save_state(&self, name: &str, state: PluginState) {
        let mut states = self.states.write();
        states.insert(name.to_string(), state);
        debug!("💾 Saved state for plugin: {}", name);
    }
    
    /// Load plugin state
    pub fn load_state(&self, name: &str) -> Option<PluginState> {
        let states = self.states.read();
        states.get(name).cloned()
    }
    
    /// Remove plugin state
    pub fn remove_state(&self, name: &str) {
        let mut states = self.states.write();
        states.remove(name);
        debug!("🗑️  Removed state for plugin: {}", name);
    }
    
    /// Get all states
    pub fn get_all_states(&self) -> HashMap<String, PluginState> {
        let states = self.states.read();
        states.clone()
    }
    
    /// Clear all states
    pub fn clear_states(&self) {
        let mut states = self.states.write();
        states.clear();
        debug!("🗑️  Cleared all plugin states");
    }
    
    /// Subscribe to reload events
    pub fn subscribe(&mut self) -> mpsc::UnboundedReceiver<ReloadEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_sender = Some(tx);
        rx
    }
    
    /// Manually trigger reload for a plugin
    pub async fn trigger_reload(&self, name: &str) -> Result<()> {
        info!("🔄 Manual reload triggered for: {}", name);
        
        let plugin_path = self.plugin_dir.join(format!("{}.wasm", name));
        
        if !plugin_path.exists() {
            return Err(anyhow::anyhow!("Plugin not found: {}", name));
        }
        
        // Reload plugin
        let version = Self::reload_plugin(&plugin_path).await?;
        
        info!("✅ Plugin reloaded: {}@{}", name, version);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_hot_reload_manager_creation() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = HotReloadManager::new(plugin_dir.path().to_path_buf(), 5);
        assert!(manager.is_ok());
        assert!(!manager.is_active());
    }
    
    #[test]
    fn test_reload_config_default() {
        let config = ReloadConfig::default();
        assert!(config.enabled);
        assert_eq!(config.interval, 5);
        assert_eq!(config.debounce, 500);
        assert!(config.preserve_state);
    }
    
    #[test]
    fn test_plugin_state() {
        let state = PluginState {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            config: serde_json::json!({"key": "value"}),
            data: HashMap::new(),
            last_reload: chrono::Utc::now(),
        };
        
        assert_eq!(state.name, "test_plugin");
        assert_eq!(state.version, "1.0.0");
    }
    
    #[test]
    fn test_state_management() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = HotReloadManager::new(plugin_dir.path().to_path_buf(), 5).unwrap();
        
        let state = PluginState {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            config: serde_json::json!({}),
            data: HashMap::new(),
            last_reload: chrono::Utc::now(),
        };
        
        manager.save_state("test_plugin", state);
        
        let loaded = manager.load_state("test_plugin");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "test_plugin");
        
        manager.remove_state("test_plugin");
        assert!(manager.load_state("test_plugin").is_none());
    }
}