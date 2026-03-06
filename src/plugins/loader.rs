// Plugin Loader - Load and manage plugins
// Inspired by VideoLAN's plugin loading mechanism

use super::{api::*, PluginError, PluginResult};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

/// Plugin loader - handles loading and unloading of plugins
pub struct PluginLoader {
    loaded_plugins: HashMap<String, Arc<LoadedPlugin>>,
    config: super::PluginConfig,
}

/// Loaded plugin with runtime information
pub struct LoadedPlugin {
    pub metadata: PluginMetadata,
    pub plugin: Box<dyn Plugin>,
    pub load_time: std::time::Duration,
    pub error: Option<String>,
    pub state: PluginState,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new(config: super::PluginConfig) -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            config,
        }
    }

    /// Load a plugin by ID
    pub fn load_plugin(&mut self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        if self.loaded_plugins.contains_key(plugin_id) {
            return Err(PluginError::AlreadyLoaded(plugin_id.to_string()));
        }

        let start = Instant::now();

        // Discover plugin (simplified - in real implementation would search directories)
        let metadata = self.find_plugin_metadata(plugin_id)?;

        // Load plugin (simplified - in real implementation would load shared library/WASM)
        let plugin: Box<dyn Plugin> = self.instantiate_plugin(&metadata)?;

        // Initialize plugin
        plugin.initialize()
            .map_err(|e| PluginError::LoadFailed(format!("Initialization failed: {}", e)))?;

        let load_time = start.elapsed();

        // Check load time
        if load_time.as_secs() > self.config.max_load_time {
            return Err(PluginError::LoadFailed(format!(
                "Plugin load time exceeded maximum: {}s",
                load_time.as_secs()
            )));
        }

        let loaded = Arc::new(LoadedPlugin {
            metadata: metadata.clone(),
            plugin,
            load_time,
            error: None,
            state: PluginState::Loaded,
        });

        self.loaded_plugins.insert(plugin_id.to_string(), Arc::clone(&loaded));
        Ok(loaded)
    }

    /// Unload a plugin by ID
    pub fn unload_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        let loaded = self
            .loaded_plugins
            .remove(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        // Shutdown plugin
        let mut plugin = Arc::try_unwrap(loaded)
            .map_err(|_| PluginError::UnloadFailed("Plugin still in use".to_string()))?;

        plugin.plugin.shutdown()
            .map_err(|e| PluginError::UnloadFailed(format!("Shutdown failed: {}", e)))?;

        Ok(())
    }

    /// Get a loaded plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        self.loaded_plugins
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))
    }

    /// Get all loaded plugins
    pub fn get_all_plugins(&self) -> Vec<Arc<LoadedPlugin>> {
        self.loaded_plugins.values().cloned().collect()
    }

    /// Get plugins with specific capability
    pub fn get_plugins_by_capability(&self, capability: PluginCapability) -> Vec<Arc<LoadedPlugin>> {
        self.loaded_plugins
            .values()
            .filter(|p| p.metadata.capabilities.contains(&capability))
            .cloned()
            .collect()
    }

    /// Reload a plugin (if hot-reload is enabled)
    pub fn reload_plugin(&mut self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        if !self.config.hot_reload {
            return Err(PluginError::LoadFailed(
                "Hot-reload is not enabled".to_string(),
            ));
        }

        // Unload existing plugin
        if self.loaded_plugins.contains_key(plugin_id) {
            self.unload_plugin(plugin_id)?;
        }

        // Load plugin again
        self.load_plugin(plugin_id)
    }

    /// Activate a plugin
    pub fn activate_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        let mut loaded = self.get_plugin(plugin_id)?;

        // Note: This would require interior mutability in a real implementation
        // For now, we'll just update the state
        // loaded.state = PluginState::Active;

        Ok(())
    }

    /// Deactivate a plugin
    pub fn deactivate_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        let loaded = self.get_plugin(plugin_id)?;

        // Note: This would require interior mutability in a real implementation
        // loaded.state = PluginState::Loaded;

        Ok(())
    }

    /// Get plugin statistics
    pub fn get_plugin_stats(&self, plugin_id: &str) -> PluginResult<HashMap<String, serde_json::Value>> {
        let loaded = self.get_plugin(plugin_id)?;
        Ok(loaded.plugin.get_stats())
    }

    /// Get plugin configuration
    pub fn get_plugin_config(&self, plugin_id: &str, key: &str) -> PluginResult<Option<serde_json::Value>> {
        let loaded = self.get_plugin(plugin_id)?;
        Ok(loaded.plugin.get_config(key))
    }

    /// Set plugin configuration
    pub fn set_plugin_config(
        &mut self,
        plugin_id: &str,
        key: &str,
        value: serde_json::Value,
    ) -> PluginResult<()> {
        let mut loaded = self.get_plugin(plugin_id)?;

        // Note: This would require interior mutability in a real implementation
        // loaded.plugin.set_config(key, value)?;

        Err(PluginError::Other("Configuration not supported".to_string()))
    }

    /// Find plugin metadata by ID (simplified)
    fn find_plugin_metadata(&self, plugin_id: &str) -> PluginResult<PluginMetadata> {
        // In a real implementation, this would search plugin directories
        // For now, we'll return an error
        Err(PluginError::NotFound(format!(
            "Plugin metadata not found: {}",
            plugin_id
        )))
    }

    /// Instantiate plugin from metadata (simplified)
    fn instantiate_plugin(&self, metadata: &PluginMetadata) -> PluginResult<Box<dyn Plugin>> {
        // In a real implementation, this would:
        // 1. Load shared library (.so, .dylib, .dll)
        // 2. Load WASM module (.wasm)
        // 3. Call plugin factory function
        // 4. Validate API version

        // For now, return a default plugin
        Ok(Box::new(super::api::DefaultPlugin))
    }

    /// Get load statistics
    pub fn get_load_stats(&self) -> LoadStats {
        LoadStats {
            total_plugins: self.loaded_plugins.len(),
            total_load_time: self
                .loaded_plugins
                .values()
                .map(|p| p.load_time.as_millis())
                .sum::<u128>() as u64,
            average_load_time: if self.loaded_plugins.is_empty() {
                0
            } else {
                self.loaded_plugins
                    .values()
                    .map(|p| p.load_time.as_millis())
                    .sum::<u128>() as u64
                    / self.loaded_plugins.len() as u64
            },
        }
    }
}

/// Plugin load statistics
#[derive(Debug, Clone)]
pub struct LoadStats {
    pub total_plugins: usize,
    pub total_load_time: u64, // milliseconds
    pub average_load_time: u64, // milliseconds
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new(super::PluginConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_new() {
        let loader = PluginLoader::new(PluginConfig::default());
        assert!(loader.loaded_plugins.is_empty());
    }

    #[test]
    fn test_load_plugin_not_found() {
        let mut loader = PluginLoader::new(PluginConfig::default());
        let result = loader.load_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_plugin_not_found() {
        let mut loader = PluginLoader::new(PluginConfig::default());
        let result = loader.unload_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_plugin_not_found() {
        let loader = PluginLoader::new(PluginConfig::default());
        let result = loader.get_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_load_stats() {
        let loader = PluginLoader::new(PluginConfig::default());
        let stats = loader.get_load_stats();
        assert_eq!(stats.total_plugins, 0);
        assert_eq!(stats.total_load_time, 0);
        assert_eq!(stats.average_load_time, 0);
    }
}