// Plugin Registry - Central plugin management
// Inspired by VideoLAN's plugin registry

use super::{api::*, loader::*, PluginError, PluginResult};
use std::collections::HashMap;
use std::sync::Arc;

/// Plugin registry - central management for all plugins
pub struct PluginRegistry {
    loader: PluginLoader,
    discovery: PluginDiscovery,
    plugins: HashMap<String, Arc<LoadedPlugin>>,
    capability_index: HashMap<PluginCapability, Vec<String>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::default(),
            discovery: PluginDiscovery::new(),
            plugins: HashMap::new(),
            capability_index: HashMap::new(),
        }
    }

    /// Create a new plugin registry with configuration
    pub fn with_config(config: super::PluginConfig) -> Self {
        Self {
            loader: PluginLoader::new(config.clone()),
            discovery: PluginDiscovery::new(),
            plugins: HashMap::new(),
            capability_index: HashMap::new(),
        }
    }

    /// Add plugin directory for discovery
    pub fn add_plugin_dir(&mut self, dir: impl AsRef<std::path::Path>) {
        self.discovery.add_plugin_dir(dir);
    }

    /// Enable a specific capability
    pub fn enable_capability(&mut self, capability: PluginCapability) {
        self.discovery.enable_capability(capability);
    }

    /// Enable all capabilities
    pub fn enable_all_capabilities(&mut self) {
        self.discovery.enable_all_capabilities();
    }

    /// Discover all available plugins
    pub fn discover_plugins(&mut self) -> PluginResult<Vec<PluginMetadata>> {
        self.discovery.discover_all()
    }

    /// Load a plugin by ID
    pub fn load_plugin(&mut self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        let loaded = self.loader.load_plugin(plugin_id)?;

        // Index by capabilities
        for capability in &loaded.metadata.capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_insert_with(Vec::new)
                .push(plugin_id.to_string());
        }

        self.plugins.insert(plugin_id.to_string(), loaded.clone());
        Ok(loaded)
    }

    /// Unload a plugin by ID
    pub fn unload_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        // Remove from capability index
        if let Some(loaded) = self.plugins.get(plugin_id) {
            for capability in &loaded.metadata.capabilities {
                if let Some(ids) = self.capability_index.get_mut(capability) {
                    ids.retain(|id| id != plugin_id);
                }
            }
        }

        // Unload from loader
        self.loader.unload_plugin(plugin_id)?;

        // Remove from registry
        self.plugins.remove(plugin_id);

        Ok(())
    }

    /// Get a plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        self.plugins
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))
    }

    /// Get all plugins
    pub fn get_all_plugins(&self) -> Vec<Arc<LoadedPlugin>> {
        self.plugins.values().cloned().collect()
    }

    /// Get plugins with specific capability
    pub fn get_plugins_by_capability(&self, capability: PluginCapability) -> Vec<Arc<LoadedPlugin>> {
        self.plugins
            .values()
            .filter(|p| p.metadata.capabilities.contains(&capability))
            .cloned()
            .collect()
    }

    /// Reload a plugin (if hot-reload is enabled)
    pub fn reload_plugin(&mut self, plugin_id: &str) -> PluginResult<Arc<LoadedPlugin>> {
        self.unload_plugin(plugin_id)?;
        self.load_plugin(plugin_id)
    }

    /// Activate a plugin
    pub fn activate_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        self.loader.activate_plugin(plugin_id)
    }

    /// Deactivate a plugin
    pub fn deactivate_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        self.loader.deactivate_plugin(plugin_id)
    }

    /// Get plugin statistics
    pub fn get_plugin_stats(&self, plugin_id: &str) -> PluginResult<HashMap<String, serde_json::Value>> {
        self.loader.get_plugin_stats(plugin_id)
    }

    /// Get plugin configuration
    pub fn get_plugin_config(&self, plugin_id: &str, key: &str) -> PluginResult<Option<serde_json::Value>> {
        self.loader.get_plugin_config(plugin_id, key)
    }

    /// Set plugin configuration
    pub fn set_plugin_config(
        &mut self,
        plugin_id: &str,
        key: &str,
        value: serde_json::Value,
    ) -> PluginResult<()> {
        self.loader.set_plugin_config(plugin_id, key, value)
    }

    /// Get registry statistics
    pub fn get_registry_stats(&self) -> RegistryStats {
        let total_load_time = self
            .plugins
            .values()
            .map(|p| p.load_time.as_millis())
            .sum::<u128>() as u64;

        let average_load_time = if self.plugins.is_empty() {
            0
        } else {
            total_load_time / self.plugins.len() as u64
        };

        RegistryStats {
            total_plugins: self.plugins.len(),
            total_capabilities: self.capability_index.len(),
            total_load_time,
            average_load_time,
        }
    }

    /// Shutdown all plugins
    pub fn shutdown(&mut self) -> PluginResult<()> {
        let plugin_ids: Vec<String> = self.plugins.keys().cloned().collect();

        for plugin_id in plugin_ids {
            let _ = self.unload_plugin(&plugin_id);
        }

        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_plugins: usize,
    pub total_capabilities: usize,
    pub total_load_time: u64,
    pub average_load_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.plugins.len(), 0);
        assert_eq!(registry.capability_index.len(), 0);
    }

    #[test]
    fn test_registry_with_config() {
        let config = PluginConfig::default();
        let registry = PluginRegistry::with_config(config);
        assert_eq!(registry.plugins.len(), 0);
    }

    #[test]
    fn test_add_plugin_dir() {
        let mut registry = PluginRegistry::new();
        registry.add_plugin_dir("/path/to/plugins");
    }

    #[test]
    fn test_enable_capability() {
        let mut registry = PluginRegistry::new();
        registry.enable_capability(PluginCapability::AudioDecoder);
    }

    #[test]
    fn test_get_all_plugins() {
        let registry = PluginRegistry::new();
        let plugins = registry.get_all_plugins();
        assert_eq!(plugins.len(), 0);
    }

    #[test]
    fn test_get_registry_stats() {
        let registry = PluginRegistry::new();
        let stats = registry.get_registry_stats();
        assert_eq!(stats.total_plugins, 0);
        assert_eq!(stats.total_capabilities, 0);
    }

    #[test]
    fn test_shutdown() {
        let mut registry = PluginRegistry::new();
        let result = registry.shutdown();
        assert!(result.is_ok());
    }
}