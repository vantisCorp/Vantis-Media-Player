//! Vantis Plugin System - WASM Sandbox
//! 
/// All plugins run in isolated WebAssembly environment for safety.
/// Crashes in plugins won't affect the main application.

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, debug, error};
use wasmtime::{Engine, Module, Store, Linker, Config};

pub mod host;

/// Plugin manager
pub struct PluginManager {
    /// Wasmtime engine
    engine: Engine,
    
    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, Plugin>>>,
    
    /// Linker for host functions
    linker: Linker<HostState>,
}

/// Plugin instance
#[derive(Clone)]
pub struct Plugin {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,
    
    /// Is active
    pub active: bool,
}

/// Host state exposed to plugins
pub struct HostState {
    /// Plugin name
    plugin_name: String,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Result<Self> {
        info!("🔌 Initializing WASM Plugin System");
        
        // Configure Wasmtime
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_multi_memory(true);
        config.wasm_threads(true);
        
        let engine = Engine::new(&config)?;
        
        // Create linker with host functions
        let mut linker = Linker::new(&engine);
        host::register_host_functions(&mut linker)?;
        
        info!("✅ Plugin system initialized");
        info!("   - SIMD: Enabled");
        info!("   - Multi-memory: Enabled");
        info!("   - Threads: Enabled");
        
        Ok(Self {
            engine,
            plugins: Arc::new(RwLock::new(HashMap::new())),
            linker,
        })
    }
    
    /// Load a plugin from WASM file
    pub async fn load_plugin(&mut self, path: &str) -> Result<()> {
        info!("📦 Loading plugin: {}", path);
        
        // Read WASM file
        let wasm_bytes = tokio::fs::read(path).await?;
        
        // Compile module
        let module = Module::from_binary(&self.engine, &wasm_bytes)?;
        
        // Create store
        let mut store = Store::new(&self.engine, HostState {
            plugin_name: path.to_string(),
        });
        
        // Instantiate
        let instance = self.linker.instantiate(&mut store, &module)?;
        
        debug!("✅ Plugin loaded: {}", path);
        
        // Store plugin info
        let plugin = Plugin {
            name: path.to_string(),
            version: "1.0.0".to_string(),
            description: "WASM plugin".to_string(),
            active: true,
        };
        
        self.plugins.write().insert(path.to_string(), plugin);
        
        Ok(())
    }
    
    /// Unload a plugin
    pub async fn unload_plugin(&self, name: &str) -> Result<()> {
        info!("📤 Unloading plugin: {}", name);
        
        let mut plugins = self.plugins.write();
        if plugins.remove(name).is_some() {
            debug!("✅ Plugin unloaded: {}", name);
        } else {
            error!("⚠️ Plugin not found: {}", name);
        }
        
        Ok(())
    }
    
    /// Get all plugins
    pub fn get_plugins(&self) -> Vec<Plugin> {
        self.plugins.read().values().cloned().collect()
    }
    
    /// Activate a plugin
    pub fn activate_plugin(&self, name: &str) -> Result<()> {
        let mut plugins = self.plugins.write();
        if let Some(plugin) = plugins.get_mut(name) {
            plugin.active = true;
            info!("✅ Plugin activated: {}", name);
        }
        Ok(())
    }
    
    /// Deactivate a plugin
    pub fn deactivate_plugin(&self, name: &str) -> Result<()> {
        let mut plugins = self.plugins.write();
        if let Some(plugin) = plugins.get_mut(name) {
            plugin.active = false;
            info!("⭕ Plugin deactivated: {}", name);
        }
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new().expect("Failed to create plugin manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_plugin_manager_default() {
        let manager = PluginManager::default();
        assert_eq!(manager.plugins.read().len(), 0);
    }
    
    #[test]
    fn test_plugin_info_creation() {
        let info = PluginInfo {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            active: false,
        };
        
        assert_eq!(info.name, "test_plugin");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.active, false);
    }
}