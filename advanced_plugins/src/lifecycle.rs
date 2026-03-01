//! Lifecycle Management
//! 
//! Manages plugin lifecycle including initialization, startup, shutdown, and state transitions.

use anyhow::{Context, Result};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, debug, warn, error};

/// Lifecycle manager
pub struct LifecycleManager {
    /// Plugin directory
    plugin_dir: PathBuf,
    
    /// Plugin states
    states: Arc<RwLock<HashMap<String, PluginLifecycleState>>>,
    
    /// Lifecycle hooks
    hooks: Arc<RwLock<HashMap<String, LifecycleHooks>>>,
}

/// Plugin lifecycle state
#[derive(Clone, Debug, PartialEq)]
pub enum PluginLifecycleState {
    /// Plugin not loaded
    Unloaded,
    
    /// Plugin loaded but not initialized
    Loaded,
    
    /// Plugin initialized
    Initialized,
    
    /// Plugin starting
    Starting,
    
    /// Plugin running
    Running,
    
    /// Plugin stopping
    Stopping,
    
    /// Plugin stopped
    Stopped,
    
    /// Plugin error
    Error(String),
}

/// Lifecycle hooks
#[derive(Clone, Debug)]
pub struct LifecycleHooks {
    /// Plugin name
    pub plugin_name: String,
    
    /// On load hook
    pub on_load: Option<String>,
    
    /// On init hook
    pub on_init: Option<String>,
    
    /// On start hook
    pub on_start: Option<String>,
    
    /// On stop hook
    pub on_stop: Option<String>,
    
    /// On unload hook
    pub on_unload: Option<String>,
    
    /// On error hook
    pub on_error: Option<String>,
}

/// Lifecycle event
#[derive(Clone, Debug)]
pub enum LifecycleEvent {
    /// Plugin loaded
    Loaded { name: String },
    
    /// Plugin initialized
    Initialized { name: String },
    
    /// Plugin started
    Started { name: String },
    
    /// Plugin stopped
    Stopped { name: String },
    
    /// Plugin unloaded
    Unloaded { name: String },
    
    /// Plugin error
    Error { name: String, error: String },
    
    /// State changed
    StateChanged { name: String, old_state: PluginLifecycleState, new_state: PluginLifecycleState },
}

/// Lifecycle configuration
#[derive(Clone, Debug)]
pub struct LifecycleConfig {
    /// Auto-start on load
    pub auto_start: bool,
    
    /// Auto-restart on error
    pub auto_restart: bool,
    
    /// Maximum restart attempts
    pub max_restart_attempts: u32,
    
    /// Shutdown timeout in seconds
    pub shutdown_timeout: u64,
    
    /// Graceful shutdown
    pub graceful_shutdown: bool,
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            auto_restart: true,
            max_restart_attempts: 3,
            shutdown_timeout: 30,
            graceful_shutdown: true,
        }
    }
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new(plugin_dir: PathBuf) -> Result<Self> {
        info!("🔄 Initializing Lifecycle Manager");
        
        info!("✅ Lifecycle manager initialized");
        info!("   - Plugin directory: {}", plugin_dir.display());
        
        Ok(Self {
            plugin_dir,
            states: Arc::new(RwLock::new(HashMap::new())),
            hooks: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Get plugin state
    pub fn get_state(&self, name: &str) -> Option<PluginLifecycleState> {
        let states = self.states.read();
        states.get(name).cloned()
    }
    
    /// Set plugin state
    pub fn set_state(&self, name: &str, new_state: PluginLifecycleState) {
        let mut states = self.states.write();
        
        let old_state = states.get(name).cloned();
        states.insert(name.to_string(), new_state.clone());
        
        debug!("🔄 State transition for {}: {:?} -> {:?}", name, old_state, new_state);
    }
    
    /// Load plugin
    pub async fn load_plugin(&self, name: &str, path: &Path) -> Result<()> {
        info!("📦 Loading plugin: {}", name);
        
        // Check current state
        if let Some(state) = self.get_state(name) {
            if state != PluginLifecycleState::Unloaded {
                return Err(anyhow::anyhow!("Plugin is not unloaded: {:?}", state));
            }
        }
        
        // Load plugin
        if !path.exists() {
            return Err(anyhow::anyhow!("Plugin file not found: {}", path.display()));
        }
        
        // Set state to loaded
        self.set_state(name, PluginLifecycleState::Loaded);
        
        info!("✅ Plugin loaded: {}", name);
        
        Ok(())
    }
    
    /// Initialize plugin
    pub async fn initialize_plugin(&self, name: &str) -> Result<()> {
        info!("🔧 Initializing plugin: {}", name);
        
        // Check current state
        let state = self.get_state(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", name))?;
        
        if state != PluginLifecycleState::Loaded {
            return Err(anyhow::anyhow!("Plugin is not loaded: {:?}", state));
        }
        
        // Call init hook if exists
        if let Some(hooks) = self.get_hooks(name) {
            if let Some(_hook) = hooks.on_init {
                // Execute hook
                debug!("🔧 Executing init hook for: {}", name);
            }
        }
        
        // Set state to initialized
        self.set_state(name, PluginLifecycleState::Initialized);
        
        info!("✅ Plugin initialized: {}", name);
        
        Ok(())
    }
    
    /// Start plugin
    pub async fn start_plugin(&self, name: &str, config: &LifecycleConfig) -> Result<()> {
        info!("🚀 Starting plugin: {}", name);
        
        // Check current state
        let state = self.get_state(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", name))?;
        
        if state != PluginLifecycleState::Initialized && state != PluginLifecycleState::Stopped {
            return Err(anyhow::anyhow!("Plugin cannot be started: {:?}", state));
        }
        
        // Set state to starting
        self.set_state(name, PluginLifecycleState::Starting);
        
        // Call start hook if exists
        if let Some(hooks) = self.get_hooks(name) {
            if let Some(_hook) = hooks.on_start {
                // Execute hook
                debug!("🚀 Executing start hook for: {}", name);
            }
        }
        
        // Set state to running
        self.set_state(name, PluginLifecycleState::Running);
        
        info!("✅ Plugin started: {}", name);
        
        Ok(())
    }
    
    /// Stop plugin
    pub async fn stop_plugin(&self, name: &str, config: &LifecycleConfig) -> Result<()> {
        info!("🛑 Stopping plugin: {}", name);
        
        // Check current state
        let state = self.get_state(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", name))?;
        
        if state != PluginLifecycleState::Running {
            return Err(anyhow::anyhow!("Plugin is not running: {:?}", state));
        }
        
        // Set state to stopping
        self.set_state(name, PluginLifecycleState::Stopping);
        
        // Call stop hook if exists
        if let Some(hooks) = self.get_hooks(name) {
            if let Some(_hook) = hooks.on_stop {
                // Execute hook
                debug!("🛑 Executing stop hook for: {}", name);
            }
        }
        
        // Set state to stopped
        self.set_state(name, PluginLifecycleState::Stopped);
        
        info!("✅ Plugin stopped: {}", name);
        
        Ok(())
    }
    
    /// Unload plugin
    pub async fn unload_plugin(&self, name: &str) -> Result<()> {
        info!("📤 Unloading plugin: {}", name);
        
        // Check current state
        let state = self.get_state(name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", name))?;
        
        if state == PluginLifecycleState::Running {
            return Err(anyhow::anyhow!("Plugin is running, stop it first"));
        }
        
        // Call unload hook if exists
        if let Some(hooks) = self.get_hooks(name) {
            if let Some(_hook) = hooks.on_unload {
                // Execute hook
                debug!("📤 Executing unload hook for: {}", name);
            }
        }
        
        // Remove state
        let mut states = self.states.write();
        states.remove(name);
        
        // Remove hooks
        let mut hooks = self.hooks.write();
        hooks.remove(name);
        
        info!("✅ Plugin unloaded: {}", name);
        
        Ok(())
    }
    
    /// Handle plugin error
    pub async fn handle_error(&self, name: &str, error: String, config: &LifecycleConfig) -> Result<()> {
        error!("❌ Plugin error: {} - {}", name, error);
        
        // Set state to error
        self.set_state(name, PluginLifecycleState::Error(error.clone()));
        
        // Call error hook if exists
        if let Some(hooks) = self.get_hooks(name) {
            if let Some(_hook) = hooks.on_error {
                // Execute hook
                debug!("❌ Executing error hook for: {}", name);
            }
        }
        
        // Auto-restart if configured
        if config.auto_restart {
            warn!("🔄 Attempting to restart plugin: {}", name);
            
            // Reset state to stopped
            self.set_state(name, PluginLifecycleState::Stopped);
            
            // Try to start again
            if let Err(e) = self.start_plugin(name, config).await {
                error!("❌ Failed to restart plugin {}: {}", name, e);
            }
        }
        
        Ok(())
    }
    
    /// Get lifecycle hooks
    pub fn get_hooks(&self, name: &str) -> Option<LifecycleHooks> {
        let hooks = self.hooks.read();
        hooks.get(name).cloned()
    }
    
    /// Set lifecycle hooks
    pub fn set_hooks(&self, hooks: LifecycleHooks) {
        let mut hooks_map = self.hooks.write();
        hooks_map.insert(hooks.plugin_name.clone(), hooks);
        debug!("✅ Set lifecycle hooks for: {}", hooks.plugin_name);
    }
    
    /// Get all plugin states
    pub fn get_all_states(&self) -> HashMap<String, PluginLifecycleState> {
        let states = self.states.read();
        states.clone()
    }
    
    /// Get plugin count
    pub fn get_plugin_count(&self) -> usize {
        let states = self.states.read();
        states.len()
    }
    
    /// Get running plugins
    pub fn get_running_plugins(&self) -> Vec<String> {
        let states = self.states.read();
        states
            .iter()
            .filter(|(_, state)| *state == &PluginLifecycleState::Running)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get stopped plugins
    pub fn get_stopped_plugins(&self) -> Vec<String> {
        let states = self.states.read();
        states
            .iter()
            .filter(|(_, state)| *state == &PluginLifecycleState::Stopped)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get errored plugins
    pub fn get_errored_plugins(&self) -> Vec<(String, String)> {
        let states = self.states.read();
        states
            .iter()
            .filter_map(|(name, state)| {
                if let PluginLifecycleState::Error(error) = state {
                    Some((name.clone(), error.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Stop all running plugins
    pub async fn stop_all(&self, config: &LifecycleConfig) -> Result<()> {
        info!("🛑 Stopping all plugins");
        
        let running = self.get_running_plugins();
        
        for name in running {
            if let Err(e) = self.stop_plugin(&name, config).await {
                error!("Failed to stop plugin {}: {}", name, e);
            }
        }
        
        info!("✅ All plugins stopped");
        
        Ok(())
    }
    
    /// Unload all plugins
    pub async fn unload_all(&self) -> Result<()> {
        info!("📤 Unloading all plugins");
        
        let states = self.states.read();
        let names: Vec<String> = states.keys().cloned().collect();
        drop(states);
        
        for name in names {
            if let Err(e) = self.unload_plugin(&name).await {
                error!("Failed to unload plugin {}: {}", name, e);
            }
        }
        
        info!("✅ All plugins unloaded");
        
        Ok(())
    }
    
    /// Get lifecycle status
    pub fn get_status(&self) -> LifecycleStatus {
        let states = self.states.read();
        
        let running = states.values().filter(|s| *s == &PluginLifecycleState::Running).count();
        let stopped = states.values().filter(|s| *s == &PluginLifecycleState::Stopped).count();
        let errored = states.values().filter(|s| matches!(s, PluginLifecycleState::Error(_))).count();
        let loaded = states.values().filter(|s| *s == &PluginLifecycleState::Loaded).count();
        let initialized = states.values().filter(|s| *s == &PluginLifecycleState::Initialized).count();
        
        LifecycleStatus {
            total: states.len(),
            running,
            stopped,
            errored,
            loaded,
            initialized,
        }
    }
}

/// Lifecycle status
#[derive(Clone, Debug)]
pub struct LifecycleStatus {
    /// Total plugins
    pub total: usize,
    
    /// Running plugins
    pub running: usize,
    
    /// Stopped plugins
    pub stopped: usize,
    
    /// Errored plugins
    pub errored: usize,
    
    /// Loaded plugins
    pub loaded: usize,
    
    /// Initialized plugins
    pub initialized: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_lifecycle_manager_creation() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = LifecycleManager::new(plugin_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_state_transitions() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = LifecycleManager::new(plugin_dir.path().to_path_buf()).unwrap();
        
        assert_eq!(manager.get_state("test"), None);
        
        manager.set_state("test", PluginLifecycleState::Loaded);
        assert_eq!(manager.get_state("test"), Some(PluginLifecycleState::Loaded));
        
        manager.set_state("test", PluginLifecycleState::Running);
        assert_eq!(manager.get_state("test"), Some(PluginLifecycleState::Running));
    }
    
    #[test]
    fn test_lifecycle_config_default() {
        let config = LifecycleConfig::default();
        assert!(config.auto_start);
        assert!(config.auto_restart);
        assert_eq!(config.max_restart_attempts, 3);
    }
    
    #[test]
    fn test_lifecycle_hooks() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = LifecycleManager::new(plugin_dir.path().to_path_buf()).unwrap();
        
        let hooks = LifecycleHooks {
            plugin_name: "test_plugin".to_string(),
            on_load: Some("load_hook".to_string()),
            on_init: Some("init_hook".to_string()),
            on_start: Some("start_hook".to_string()),
            on_stop: Some("stop_hook".to_string()),
            on_unload: Some("unload_hook".to_string()),
            on_error: Some("error_hook".to_string()),
        };
        
        manager.set_hooks(hooks);
        
        let retrieved = manager.get_hooks("test_plugin");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().on_load, Some("load_hook".to_string()));
    }
    
    #[test]
    fn test_get_status() {
        let plugin_dir = TempDir::new().unwrap();
        let manager = LifecycleManager::new(plugin_dir.path().to_path_buf()).unwrap();
        
        manager.set_state("plugin1", PluginLifecycleState::Running);
        manager.set_state("plugin2", PluginLifecycleState::Stopped);
        manager.set_state("plugin3", PluginLifecycleState::Error("test".to_string()));
        
        let status = manager.get_status();
        assert_eq!(status.total, 3);
        assert_eq!(status.running, 1);
        assert_eq!(status.stopped, 1);
        assert_eq!(status.errored, 1);
    }
}