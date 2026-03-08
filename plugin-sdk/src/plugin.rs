//! Core plugin trait and types.

use crate::api::{HostApi, MediaApi, UiApi};
use crate::error::PluginResult;
use crate::manifest::PluginManifest;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Unique plugin identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PluginId(String);

impl PluginId {
    /// Create a new plugin ID from a string.
    pub fn new(id: impl Into<String>) -> Self {
        PluginId(id.into())
    }

    /// Create a plugin ID from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        PluginId(uuid.to_string())
    }

    /// Generate a random plugin ID.
    pub fn random() -> Self {
        PluginId(Uuid::new_v4().to_string())
    }

    /// Get the ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse a plugin ID from a string.
    pub fn parse(s: &str) -> Option<Self> {
        if s.is_empty() {
            None
        } else {
            Some(PluginId(s.to_string()))
        }
    }
}

impl Default for PluginId {
    fn default() -> Self {
        PluginId::random()
    }
}

impl std::fmt::Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for PluginId {
    fn from(s: &str) -> Self {
        PluginId(s.to_string())
    }
}

impl From<String> for PluginId {
    fn from(s: String) -> Self {
        PluginId(s)
    }
}

/// Plugin metadata and information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Unique plugin identifier (e.g., "com.example.my-plugin").
    pub id: PluginId,
    /// Human-readable plugin name.
    pub name: String,
    /// Plugin version (semver).
    pub version: String,
    /// Plugin description.
    pub description: String,
    /// Plugin author.
    pub author: String,
    /// Plugin homepage URL.
    pub homepage: Option<String>,
    /// Plugin license.
    pub license: String,
    /// Minimum host version required.
    pub min_host_version: String,
    /// Plugin tags for categorization.
    pub tags: Vec<String>,
    /// Plugin icon (base64 encoded or URL).
    pub icon: Option<String>,
}

impl Default for PluginInfo {
    fn default() -> Self {
        PluginInfo {
            id: PluginId::random(),
            name: "Unnamed Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: String::new(),
            homepage: None,
            license: "MIT".to_string(),
            min_host_version: "1.0.0".to_string(),
            tags: Vec::new(),
            icon: None,
        }
    }
}

impl PluginInfo {
    /// Create a new plugin info with the given ID and name.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        PluginInfo {
            id: PluginId::new(id),
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set the plugin version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Set the plugin author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Set the plugin description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Plugin capabilities flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PluginCapabilities {
    /// Plugin can handle media playback.
    pub media_playback: bool,
    /// Plugin can decode media formats.
    pub media_decoding: bool,
    /// Plugin can modify UI.
    pub ui_extension: bool,
    /// Plugin provides metadata providers.
    pub metadata_provider: bool,
    /// Plugin can handle network streams.
    pub network_handler: bool,
    /// Plugin provides subtitle support.
    pub subtitle_handler: bool,
    /// Plugin can process audio.
    pub audio_processor: bool,
    /// Plugin can process video.
    pub video_processor: bool,
    /// Plugin handles keyboard shortcuts.
    pub keyboard_shortcuts: bool,
    /// Plugin handles notifications.
    pub notifications: bool,
}

impl PluginCapabilities {
    /// Create capabilities for a media player plugin.
    pub fn media_player() -> Self {
        PluginCapabilities {
            media_playback: true,
            media_decoding: true,
            ..Default::default()
        }
    }

    /// Create capabilities for a UI plugin.
    pub fn ui_extension() -> Self {
        PluginCapabilities {
            ui_extension: true,
            ..Default::default()
        }
    }

    /// Create capabilities for a metadata plugin.
    pub fn metadata_provider() -> Self {
        PluginCapabilities {
            metadata_provider: true,
            ..Default::default()
        }
    }

    /// Create capabilities for an audio processor plugin.
    pub fn audio_processor() -> Self {
        PluginCapabilities {
            audio_processor: true,
            ..Default::default()
        }
    }

    /// Create capabilities for a video processor plugin.
    pub fn video_processor() -> Self {
        PluginCapabilities {
            video_processor: true,
            ..Default::default()
        }
    }

    /// Get all capabilities as a list of strings.
    pub fn to_list(&self) -> Vec<&'static str> {
        let mut caps = Vec::new();
        if self.media_playback { caps.push("media_playback"); }
        if self.media_decoding { caps.push("media_decoding"); }
        if self.ui_extension { caps.push("ui_extension"); }
        if self.metadata_provider { caps.push("metadata_provider"); }
        if self.network_handler { caps.push("network_handler"); }
        if self.subtitle_handler { caps.push("subtitle_handler"); }
        if self.audio_processor { caps.push("audio_processor"); }
        if self.video_processor { caps.push("video_processor"); }
        if self.keyboard_shortcuts { caps.push("keyboard_shortcuts"); }
        if self.notifications { caps.push("notifications"); }
        caps
    }

    /// Check if any capability is set.
    pub fn has_any(&self) -> bool {
        self.media_playback
            || self.media_decoding
            || self.ui_extension
            || self.metadata_provider
            || self.network_handler
            || self.subtitle_handler
            || self.audio_processor
            || self.video_processor
            || self.keyboard_shortcuts
            || self.notifications
    }
}

/// Configuration for a plugin.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Configuration values.
    pub values: HashMap<String, serde_json::Value>,
    /// Path to the config file.
    #[serde(skip)]
    pub config_path: Option<PathBuf>,
}

impl PluginConfig {
    /// Create a new empty config.
    pub fn new() -> Self {
        PluginConfig::default()
    }

    /// Get a configuration value.
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.values.get(key).and_then(|v| {
            serde_json::from_value(v.clone()).ok()
        })
    }

    /// Set a configuration value.
    pub fn set<T: Serialize>(&mut self, key: impl Into<String>, value: T) {
        self.values.insert(key.into(), serde_json::to_value(value).unwrap_or(serde_json::Value::Null));
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Remove a configuration value.
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.values.remove(key)
    }

    /// Load config from a file.
    pub fn load(path: &PathBuf) -> PluginResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: PluginConfig = serde_json::from_str(&content)?;
        config.config_path = Some(path.clone());
        Ok(config)
    }

    /// Save config to a file.
    pub fn save(&self) -> PluginResult<()> {
        if let Some(path) = &self.config_path {
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }
}

/// Context provided to a plugin during initialization.
pub struct PluginContext {
    /// Host API for interacting with the application.
    pub host: HostApi,
    /// Media API for media operations.
    pub media: MediaApi,
    /// UI API for UI operations.
    pub ui: UiApi,
    /// Plugin configuration.
    pub config: PluginConfig,
    /// Plugin data directory.
    pub data_dir: PathBuf,
    /// Plugin cache directory.
    pub cache_dir: PathBuf,
    /// Plugin log directory.
    pub log_dir: PathBuf,
}

impl PluginContext {
    /// Create a new plugin context.
    pub fn new(
        host: HostApi,
        media: MediaApi,
        ui: UiApi,
        data_dir: PathBuf,
    ) -> Self {
        PluginContext {
            host,
            media,
            ui,
            config: PluginConfig::default(),
            data_dir: data_dir.clone(),
            cache_dir: data_dir.join("cache"),
            log_dir: data_dir.join("logs"),
        }
    }

    /// Get a reference to the host API.
    pub fn host(&self) -> &HostApi {
        &self.host
    }

    /// Get a reference to the media API.
    pub fn media(&self) -> &MediaApi {
        &self.media
    }

    /// Get a reference to the UI API.
    pub fn ui(&self) -> &UiApi {
        &self.ui
    }

    /// Ensure plugin directories exist.
    pub fn ensure_directories(&self) -> PluginResult<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        std::fs::create_dir_all(&self.log_dir)?;
        Ok(())
    }
}

/// The core plugin trait.
///
/// All plugins must implement this trait to be loaded by the host.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin information.
    fn info(&self) -> PluginInfo;

    /// Get plugin capabilities.
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::default()
    }

    /// Create a new instance of the plugin.
    fn new() -> Self where Self: Sized;

    /// Initialize the plugin.
    /// 
    /// This is called after the plugin is loaded and before it's used.
    /// The plugin should set up its internal state and register any
    /// handlers or extensions.
    async fn initialize(&mut self, context: &PluginContext) -> PluginResult<()>;

    /// Shutdown the plugin.
    /// 
    /// This is called when the plugin is being unloaded.
    /// The plugin should clean up any resources.
    async fn shutdown(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Called when the plugin is enabled.
    async fn on_enable(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Called when the plugin is disabled.
    async fn on_disable(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Handle a custom message from the host.
    async fn handle_message(&mut self, _message: &str, _data: &[u8]) -> PluginResult<Vec<u8>> {
        Ok(Vec::new())
    }

    /// Get extension data.
    /// 
    /// This is used for type-erased plugin extensions.
    fn as_any(&self) -> &dyn Any {
        &()
    }

    /// Get mutable extension data.
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut ()
    }
}

/// A loaded plugin instance.
pub struct LoadedPlugin {
    /// Plugin information.
    pub info: PluginInfo,
    /// Plugin capabilities.
    pub capabilities: PluginCapabilities,
    /// Plugin manifest.
    pub manifest: Option<PluginManifest>,
    /// Path to the plugin library.
    pub library_path: PathBuf,
    /// Whether the plugin is enabled.
    pub enabled: bool,
}

impl LoadedPlugin {
    /// Create a new loaded plugin record.
    pub fn new(
        info: PluginInfo,
        capabilities: PluginCapabilities,
        library_path: PathBuf,
    ) -> Self {
        LoadedPlugin {
            info,
            capabilities,
            manifest: None,
            library_path,
            enabled: true,
        }
    }

    /// Check if the plugin has a specific capability.
    pub fn has_capability(&self, cap: &str) -> bool {
        self.capabilities.to_list().contains(&cap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_id() {
        let id = PluginId::new("com.example.test");
        assert_eq!(id.as_str(), "com.example.test");
    }

    #[test]
    fn test_plugin_info() {
        let info = PluginInfo::new("com.example.test", "Test Plugin")
            .with_version("1.0.0")
            .with_author("Test Author");

        assert_eq!(info.id.as_str(), "com.example.test");
        assert_eq!(info.name, "Test Plugin");
        assert_eq!(info.version, "1.0.0");
    }

    #[test]
    fn test_capabilities() {
        let caps = PluginCapabilities::media_player();
        assert!(caps.media_playback);
        assert!(caps.media_decoding);
        assert!(!caps.ui_extension);
    }

    #[test]
    fn test_config() {
        let mut config = PluginConfig::new();
        config.set("key", "value");
        config.set("number", 42);

        assert_eq!(config.get::<String>("key"), Some("value".to_string()));
        assert_eq!(config.get::<i32>("number"), Some(42));
    }
}