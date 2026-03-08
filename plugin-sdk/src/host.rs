//! Plugin host implementation for loading and managing plugins.

use crate::error::{PluginError, PluginResult};
use crate::lifecycle::{PluginLifecycle, PluginState};
use crate::manifest::{PluginManifest, PluginDependency};
use crate::plugin::{LoadedPlugin, Plugin, PluginCapabilities, PluginContext, PluginId, PluginInfo};
use crate::ffi::PluginLibrary;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Plugin host for managing plugin lifecycle.
pub struct PluginHost {
    /// Registry of loaded plugins.
    registry: PluginRegistry,
    /// Plugin loader.
    loader: PluginLoader,
    /// Plugin directories to search.
    plugin_dirs: Vec<PathBuf>,
    /// Host version for compatibility checking.
    host_version: String,
    /// Configuration directory.
    config_dir: PathBuf,
    /// Data directory.
    data_dir: PathBuf,
    /// Event sender for plugin events.
    event_tx: mpsc::Sender<PluginEvent>,
    /// Event receiver (optional, for external handling).
    event_rx: Option<mpsc::Receiver<PluginEvent>>,
}

/// Plugin events.
#[derive(Debug, Clone)]
pub enum PluginEvent {
    /// Plugin was loaded.
    Loaded { id: PluginId },
    /// Plugin was initialized.
    Initialized { id: PluginId },
    /// Plugin was enabled.
    Enabled { id: PluginId },
    /// Plugin was disabled.
    Disabled { id: PluginId },
    /// Plugin was unloaded.
    Unloaded { id: PluginId },
    /// Plugin encountered an error.
    Error { id: PluginId, error: String },
}

impl PluginHost {
    /// Create a new plugin host.
    pub fn new(host_version: impl Into<String>) -> Self {
        let (event_tx, event_rx) = mpsc::channel(100);
        
        PluginHost {
            registry: PluginRegistry::new(),
            loader: PluginLoader::new(),
            plugin_dirs: Vec::new(),
            host_version: host_version.into(),
            config_dir: PathBuf::from("./plugins/config"),
            data_dir: PathBuf::from("./plugins/data"),
            event_tx,
            event_rx: Some(event_rx),
        }
    }

    /// Add a plugin search directory.
    pub fn add_plugin_dir(&mut self, dir: impl Into<PathBuf>) {
        self.plugin_dirs.push(dir.into());
    }

    /// Set the configuration directory.
    pub fn set_config_dir(&mut self, dir: impl Into<PathBuf>) {
        self.config_dir = dir.into();
    }

    /// Set the data directory.
    pub fn set_data_dir(&mut self, dir: impl Into<PathBuf>) {
        self.data_dir = dir.into();
    }

    /// Get the event receiver.
    pub fn take_event_receiver(&mut self) -> Option<mpsc::Receiver<PluginEvent>> {
        self.event_rx.take()
    }

    /// Scan plugin directories for available plugins.
    pub fn discover_plugins(&self) -> PluginResult<Vec<DiscoveredPlugin>> {
        let mut discovered = Vec::new();

        for dir in &self.plugin_dirs {
            if !dir.exists() {
                debug!("Plugin directory does not exist: {:?}", dir);
                continue;
            }

            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                // Check for plugin manifest or library
                if path.is_dir() {
                    let manifest_path = path.join("plugin.json");
                    if manifest_path.exists() {
                        if let Ok(manifest) = PluginManifest::load(&manifest_path) {
                            discovered.push(DiscoveredPlugin {
                                manifest,
                                path: path.join("plugin.so"), // or .dll on Windows
                                manifest_path,
                            });
                        }
                    }
                } else if path.extension().map_or(false, |ext| {
                    ext == "so" || ext == "dll" || ext == "dylib"
                }) {
                    // Try to load manifest from adjacent file
                    let manifest_path = path.with_extension("json");
                    if manifest_path.exists() {
                        if let Ok(manifest) = PluginManifest::load(&manifest_path) {
                            discovered.push(DiscoveredPlugin {
                                manifest,
                                path: path.clone(),
                                manifest_path,
                            });
                        }
                    }
                }
            }
        }

        info!("Discovered {} plugins", discovered.len());
        Ok(discovered)
    }

    /// Load a plugin from a path.
    pub async fn load_plugin(&mut self, path: &Path) -> PluginResult<PluginId> {
        // Load the plugin manifest
        let manifest_path = path.with_extension("json");
        let manifest = if manifest_path.exists() {
            Some(PluginManifest::load(&manifest_path)?)
        } else {
            None
        };

        // Check dependencies
        if let Some(ref m) = manifest {
            self.check_dependencies(&m.dependencies)?;
        }

        // Load the plugin library
        let library = self.loader.load(path)?;
        let info = library.info()?;
        let capabilities = library.capabilities()?;

        // Check version compatibility
        self.check_version_compatibility(&info)?;

        // Create loaded plugin record
        let plugin_id = info.id.clone();
        let loaded = LoadedPlugin::new(info, capabilities, path.to_path_buf());
        
        // Register the plugin
        self.registry.register(plugin_id.clone(), loaded, library);

        // Send event
        let _ = self.event_tx.send(PluginEvent::Loaded { id: plugin_id.clone() }).await;

        info!("Loaded plugin: {} ({})", plugin_id, path.display());
        Ok(plugin_id)
    }

    /// Initialize a loaded plugin.
    pub async fn initialize_plugin(&mut self, id: &PluginId) -> PluginResult<()> {
        let plugin_dir = self.data_dir.join(id.as_str());
        std::fs::create_dir_all(&plugin_dir)?;

        // Get host APIs
        let host_api = self.create_host_api();
        let media_api = self.create_media_api();
        let ui_api = self.create_ui_api();

        let context = PluginContext::new(host_api, media_api, ui_api, plugin_dir);
        context.ensure_directories()?;

        // Initialize through registry
        self.registry.initialize(id, &context).await?;

        let _ = self.event_tx.send(PluginEvent::Initialized { id: id.clone() }).await;
        info!("Initialized plugin: {}", id);
        Ok(())
    }

    /// Enable a plugin.
    pub async fn enable_plugin(&mut self, id: &PluginId) -> PluginResult<()> {
        self.registry.enable(id).await?;
        let _ = self.event_tx.send(PluginEvent::Enabled { id: id.clone() }).await;
        info!("Enabled plugin: {}", id);
        Ok(())
    }

    /// Disable a plugin.
    pub async fn disable_plugin(&mut self, id: &PluginId) -> PluginResult<()> {
        self.registry.disable(id).await?;
        let _ = self.event_tx.send(PluginEvent::Disabled { id: id.clone() }).await;
        info!("Disabled plugin: {}", id);
        Ok(())
    }

    /// Unload a plugin.
    pub async fn unload_plugin(&mut self, id: &PluginId) -> PluginResult<()> {
        self.registry.unregister(id).await?;
        let _ = self.event_tx.send(PluginEvent::Unloaded { id: id.clone() }).await;
        info!("Unloaded plugin: {}", id);
        Ok(())
    }

    /// Get all loaded plugins.
    pub fn loaded_plugins(&self) -> Vec<&LoadedPlugin> {
        self.registry.list()
    }

    /// Get a specific plugin.
    pub fn get_plugin(&self, id: &PluginId) -> Option<&LoadedPlugin> {
        self.registry.get(id)
    }

    /// Check if a plugin is loaded.
    pub fn is_loaded(&self, id: &PluginId) -> bool {
        self.registry.contains(id)
    }

    /// Get plugins with a specific capability.
    pub fn plugins_with_capability(&self, capability: &str) -> Vec<&LoadedPlugin> {
        self.registry.list()
            .into_iter()
            .filter(|p| p.has_capability(capability))
            .copied()
            .collect()
    }

    /// Check dependencies.
    fn check_dependencies(&self, dependencies: &[PluginDependency]) -> PluginResult<()> {
        for dep in dependencies {
            if dep.optional {
                continue;
            }
            if !self.registry.contains(&dep.plugin_id) {
                return Err(PluginError::dependency(
                    "host",
                    dep.plugin_id.as_str(),
                    "required dependency not loaded",
                ));
            }
        }
        Ok(())
    }

    /// Check version compatibility.
    fn check_version_compatibility(&self, info: &PluginInfo) -> PluginResult<()> {
        use semver::Version;

        let host_version = Version::parse(&self.host_version)
            .map_err(|_| PluginError::ConfigurationError("Invalid host version".into()))?;
        let min_required = Version::parse(&info.min_host_version)
            .map_err(|_| PluginError::ConfigurationError("Invalid plugin min_host_version".into()))?;

        if host_version < min_required {
            return Err(PluginError::version_mismatch(
                &info.id.to_string(),
                &info.min_host_version,
                &self.host_version,
            ));
        }

        Ok(())
    }

    /// Create host API instance.
    fn create_host_api(&self) -> crate::api::HostApi {
        crate::api::HostApi::new(self.host_version.clone())
    }

    /// Create media API instance.
    fn create_media_api(&self) -> crate::api::MediaApi {
        crate::api::MediaApi::new()
    }

    /// Create UI API instance.
    fn create_ui_api(&self) -> crate::api::UiApi {
        crate::api::UiApi::new()
    }
}

/// A discovered plugin that hasn't been loaded yet.
#[derive(Debug, Clone)]
pub struct DiscoveredPlugin {
    /// Plugin manifest.
    pub manifest: PluginManifest,
    /// Path to the plugin library.
    pub path: PathBuf,
    /// Path to the manifest file.
    pub manifest_path: PathBuf,
}

/// Plugin registry for tracking loaded plugins.
pub struct PluginRegistry {
    /// Loaded plugin instances.
    plugins: DashMap<PluginId, (LoadedPlugin, Option<Arc<RwLock<PluginLibrary>>>)>,
    /// Plugin states.
    states: DashMap<PluginId, PluginState>,
}

impl PluginRegistry {
    /// Create a new plugin registry.
    pub fn new() -> Self {
        PluginRegistry {
            plugins: DashMap::new(),
            states: DashMap::new(),
        }
    }

    /// Register a new plugin.
    pub fn register(&self, id: PluginId, plugin: LoadedPlugin, library: PluginLibrary) {
        self.states.insert(id.clone(), PluginState::Loaded);
        self.plugins.insert(id, (plugin, Some(Arc::new(RwLock::new(library)))));
    }

    /// Unregister a plugin.
    pub async fn unregister(&self, id: &PluginId) -> PluginResult<()> {
        if let Some((_, (mut plugin, mut library))) = self.plugins.remove(id) {
            // Call shutdown
            if let Some(lib) = library.take() {
                let mut lib = lib.write();
                lib.shutdown().await?;
            }
            self.states.remove(id);
            Ok(())
        } else {
            Err(PluginError::NotFound(id.to_string()))
        }
    }

    /// Get a plugin by ID.
    pub fn get(&self, id: &PluginId) -> Option<&LoadedPlugin> {
        self.plugins.get(id).map(|entry| &entry.0)
    }

    /// Check if a plugin is registered.
    pub fn contains(&self, id: &PluginId) -> bool {
        self.plugins.contains_key(id)
    }

    /// List all loaded plugins.
    pub fn list(&self) -> Vec<&LoadedPlugin> {
        self.plugins.iter().map(|entry| &entry.0).collect()
    }

    /// Get the state of a plugin.
    pub fn state(&self, id: &PluginId) -> Option<PluginState> {
        self.states.get(id).map(|s| *s)
    }

    /// Initialize a plugin.
    pub async fn initialize(&self, id: &PluginId, context: &PluginContext) -> PluginResult<()> {
        let mut entry = self.plugins.get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?;
        
        if let Some(ref library) = entry.1 {
            let mut lib = library.write();
            lib.initialize(context).await?;
        }

        self.states.insert(id.clone(), PluginState::Initialized);
        Ok(())
    }

    /// Enable a plugin.
    pub async fn enable(&self, id: &PluginId) -> PluginResult<()> {
        let mut entry = self.plugins.get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?;
        
        entry.0.enabled = true;

        if let Some(ref library) = entry.1 {
            let mut lib = library.write();
            lib.enable().await?;
        }

        self.states.insert(id.clone(), PluginState::Enabled);
        Ok(())
    }

    /// Disable a plugin.
    pub async fn disable(&self, id: &PluginId) -> PluginResult<()> {
        let mut entry = self.plugins.get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?;
        
        entry.0.enabled = false;

        if let Some(ref library) = entry.1 {
            let mut lib = library.write();
            lib.disable().await?;
        }

        self.states.insert(id.clone(), PluginState::Disabled);
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin loader for loading dynamic libraries.
pub struct PluginLoader {
    /// Loaded libraries.
    libraries: HashMap<PathBuf, libloading::Library>,
}

impl PluginLoader {
    /// Create a new plugin loader.
    pub fn new() -> Self {
        PluginLoader {
            libraries: HashMap::new(),
        }
    }

    /// Load a plugin library.
    pub fn load(&mut self, path: &Path) -> PluginResult<PluginLibrary> {
        // Check if already loaded
        if self.libraries.contains_key(path) {
            return Err(PluginError::AlreadyLoaded(path.display().to_string()));
        }

        // Load the library
        let library = unsafe {
            libloading::Library::new(path)
                .map_err(|e| PluginError::load_failed(path, e.to_string()))?
        };

        self.libraries.insert(path.to_path_buf(), library);

        Ok(PluginLibrary::new(path.to_path_buf()))
    }

    /// Unload a plugin library.
    pub fn unload(&mut self, path: &Path) -> PluginResult<()> {
        if let Some(_) = self.libraries.remove(path) {
            // Library is dropped here
            Ok(())
        } else {
            Err(PluginError::NotFound(path.display().to_string()))
        }
    }

    /// Check if a library is loaded.
    pub fn is_loaded(&self, path: &Path) -> bool {
        self.libraries.contains_key(path)
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_host_creation() {
        let host = PluginHost::new("1.0.0");
        assert_eq!(host.host_version, "1.0.0");
    }

    #[test]
    fn test_plugin_registry() {
        let registry = PluginRegistry::new();
        let id = PluginId::new("test");
        let info = PluginInfo::new("test", "Test Plugin");
        let loaded = LoadedPlugin::new(info, PluginCapabilities::default(), PathBuf::from("test.so"));

        assert!(!registry.contains(&id));
    }

    #[test]
    fn test_discovered_plugin() {
        let manifest = PluginManifest {
            id: PluginId::new("test"),
            name: "Test".into(),
            version: "1.0.0".into(),
            ..Default::default()
        };

        let discovered = DiscoveredPlugin {
            manifest,
            path: PathBuf::from("test.so"),
            manifest_path: PathBuf::from("test.json"),
        };

        assert_eq!(discovered.manifest.id.as_str(), "test");
    }
}