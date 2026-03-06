// Plugin Discovery - Find and validate plugins
// Inspired by VideoLAN's plugin discovery mechanism

use super::{api::*, PluginError, PluginResult};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Plugin discovery engine
pub struct PluginDiscovery {
    plugin_dirs: Vec<PathBuf>,
    enabled_capabilities: HashSet<PluginCapability>,
}

impl PluginDiscovery {
    /// Create a new plugin discovery engine
    pub fn new() -> Self {
        Self {
            plugin_dirs: Vec::new(),
            enabled_capabilities: HashSet::new(),
        }
    }

    /// Add a plugin directory to search
    pub fn add_plugin_dir(&mut self, dir: impl AsRef<Path>) {
        self.plugin_dirs.push(dir.as_ref().to_path_buf());
    }

    /// Enable a specific capability
    pub fn enable_capability(&mut self, capability: PluginCapability) {
        self.enabled_capabilities.insert(capability);
    }

    /// Enable all capabilities
    pub fn enable_all_capabilities(&mut self) {
        for cap in [
            PluginCapability::AudioDecoder,
            PluginCapability::VideoDecoder,
            PluginCapability::SubtitleDecoder,
            PluginCapability::AudioEncoder,
            PluginCapability::VideoEncoder,
            PluginCapability::SubtitleEncoder,
            PluginCapability::AudioFilter,
            PluginCapability::VideoFilter,
            PluginCapability::SubtitleFilter,
            PluginCapability::Demuxer,
            PluginCapability::Muxer,
            PluginCapability::NetworkProtocol,
            PluginCapability::UIComponent,
            PluginCapability::Theme,
            PluginCapability::Visualizer,
            PluginCapability::Extension,
            PluginCapability::Script,
            PluginCapability::AIModel,
            PluginCapability::HardwareAcceleration,
        ] {
            self.enabled_capabilities.insert(cap);
        }
    }

    /// Discover all plugins in registered directories
    pub fn discover_all(&self) -> PluginResult<Vec<PluginMetadata>> {
        let mut plugins = Vec::new();

        for dir in &self.plugin_dirs {
            if dir.exists() && dir.is_dir() {
                let dir_plugins = self.discover_in_directory(dir)?;
                plugins.extend(dir_plugins);
            }
        }

        // Remove duplicates by ID
        let mut seen = HashSet::new();
        plugins.retain(|p| seen.insert(p.id.clone()));

        Ok(plugins)
    }

    /// Discover plugins in a specific directory
    pub fn discover_in_directory(&self, dir: &Path) -> PluginResult<Vec<PluginMetadata>> {
        let mut plugins = Vec::new();

        // Read directory entries
        let entries = fs::read_dir(dir)
            .map_err(|e| PluginError::Io(e))?;

        for entry in entries {
            let entry = entry.map_err(|e| PluginError::Io(e))?;
            let path = entry.path();

            // Try to discover plugin
            if let Some(metadata) = self.discover_plugin(&path)? {
                // Filter by capabilities if specified
                if self.enabled_capabilities.is_empty()
                    || metadata.capabilities.iter().any(|c| self.enabled_capabilities.contains(c))
                {
                    plugins.push(metadata);
                }
            }
        }

        Ok(plugins)
    }

    /// Discover a single plugin at the given path
    pub fn discover_plugin(&self, path: &Path) -> PluginResult<Option<PluginMetadata>> {
        // Try different plugin formats
        if path.is_file() {
            // Check for shared library (.so, .dylib, .dll)
            if path.extension().and_then(|s| s.to_str()).map_or(false, |ext| {
                matches!(ext, "so" | "dylib" | "dll")
            }) {
                return self.discover_native_plugin(path);
            }

            // Check for manifest file (.json, .toml)
            if path.extension().and_then(|s| s.to_str()).map_or(false, |ext| {
                matches!(ext, "json" | "toml")
            }) {
                return self.discover_manifest_plugin(path);
            }

            // Check for WASM plugin (.wasm)
            if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                return self.discover_wasm_plugin(path);
            }
        }

        // Check for directory plugin
        if path.is_dir() {
            return self.discover_directory_plugin(path);
        }

        Ok(None)
    }

    /// Discover native plugin (shared library)
    fn discover_native_plugin(&self, path: &Path) -> PluginResult<Option<PluginMetadata>> {
        // In a real implementation, this would load the shared library
        // and call a function to get the plugin metadata
        // For now, we'll look for a metadata file alongside the library

        let metadata_path = path.with_extension("json");
        if metadata_path.exists() {
            self.discover_manifest_plugin(&metadata_path)
        } else {
            Ok(None)
        }
    }

    /// Discover manifest plugin (JSON/TOML)
    fn discover_manifest_plugin(&self, path: &Path) -> PluginResult<Option<PluginMetadata>> {
        let content = fs::read_to_string(path)
            .map_err(|e| PluginError::Io(e))?;

        let metadata: PluginMetadata = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            // Parse TOML
            let value: toml::Value = toml::from_str(&content)
                .map_err(|e| PluginError::Other(format!("TOML parse error: {}", e)))?;
            serde_json::from_value(serde_json::to_value(value)?)
                .map_err(|e| PluginError::Other(format!("Metadata conversion error: {}", e)))?
        } else {
            // Parse JSON
            serde_json::from_str(&content)
                .map_err(|e| PluginError::Other(format!("JSON parse error: {}", e)))?
        };

        // Validate metadata
        metadata.validate()
            .map_err(|e| PluginError::LoadFailed(format!("Invalid metadata: {}", e)))?;

        Ok(Some(metadata))
    }

    /// Discover WASM plugin
    fn discover_wasm_plugin(&self, path: &Path) -> PluginResult<Option<PluginMetadata>> {
        // In a real implementation, this would read the WASM module's custom sections
        // to extract metadata
        // For now, we'll look for a metadata file alongside the WASM file

        let metadata_path = path.with_extension("json");
        if metadata_path.exists() {
            self.discover_manifest_plugin(&metadata_path)
        } else {
            Ok(None)
        }
    }

    /// Discover directory plugin
    fn discover_directory_plugin(&self, path: &Path) -> PluginResult<Option<PluginMetadata>> {
        // Look for plugin manifest in directory
        let manifest_path = path.join("plugin.json");
        if manifest_path.exists() {
            self.discover_manifest_plugin(&manifest_path)
        } else {
            let manifest_path = path.join("plugin.toml");
            if manifest_path.exists() {
                self.discover_manifest_plugin(&manifest_path)
            } else {
                Ok(None)
            }
        }
    }

    /// Validate plugin metadata
    pub fn validate_metadata(&self, metadata: &PluginMetadata) -> PluginResult<()> {
        metadata.validate()
            .map_err(|e| PluginError::LoadFailed(format!("Invalid metadata: {}", e)))?;

        // Check for circular dependencies
        self.check_circular_dependencies(metadata, &mut HashSet::new())?;

        Ok(())
    }

    /// Check for circular dependencies
    fn check_circular_dependencies(
        &self,
        metadata: &PluginMetadata,
        visited: &mut HashSet<String>,
    ) -> PluginResult<()> {
        if visited.contains(&metadata.id) {
            return Err(PluginError::DependencyUnsatisfied(format!(
                "Circular dependency detected for plugin {}",
                metadata.id
            )));
        }

        visited.insert(metadata.id.clone());

        for dep in &metadata.dependencies {
            // In a real implementation, we would check if the dependency exists
            // For now, we just skip the check
        }

        visited.remove(&metadata.id);
        Ok(())
    }
}

impl Default for PluginDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_new() {
        let discovery = PluginDiscovery::new();
        assert!(discovery.plugin_dirs.is_empty());
        assert!(discovery.enabled_capabilities.is_empty());
    }

    #[test]
    fn test_add_plugin_dir() {
        let mut discovery = PluginDiscovery::new();
        discovery.add_plugin_dir("/path/to/plugins");
        assert_eq!(discovery.plugin_dirs.len(), 1);
    }

    #[test]
    fn test_enable_capability() {
        let mut discovery = PluginDiscovery::new();
        discovery.enable_capability(PluginCapability::AudioDecoder);
        assert!(discovery.enabled_capabilities.contains(&PluginCapability::AudioDecoder));
    }

    #[test]
    fn test_enable_all_capabilities() {
        let mut discovery = PluginDiscovery::new();
        discovery.enable_all_capabilities();
        assert!(!discovery.enabled_capabilities.is_empty());
    }
}