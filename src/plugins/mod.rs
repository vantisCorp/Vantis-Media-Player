// Vantis Media Player - Modular Plugin System
// Inspired by VideoLAN's plugin architecture (400+ plugins in VLC)

pub mod api;
pub mod discovery;
pub mod loader;
pub mod registry;

pub use api::{Plugin, PluginCapability, PluginInfo, PluginMetadata, PluginState};
pub use discovery::PluginDiscovery;
pub use loader::PluginLoader;
pub use registry::PluginRegistry;

// Re-export common types for convenience
pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),

    #[error("Plugin load failed: {0}")]
    LoadFailed(String),

    #[error("Plugin unload failed: {0}")]
    UnloadFailed(String),

    #[error("Plugin API version mismatch: expected {expected}, got {actual}")]
    ApiVersionMismatch { expected: u32, actual: u32 },

    #[error("Plugin dependency not satisfied: {0}")]
    DependencyUnsatisfied(String),

    #[error("Plugin capability not supported: {0}")]
    CapabilityNotSupported(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// Plugin initialization configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Plugin directory path
    pub plugin_dir: std::path::PathBuf,

    /// Enable plugin hot-reloading
    pub hot_reload: bool,

    /// Enable plugin validation
    pub validate: bool,

    /// Maximum plugin load time in seconds
    pub max_load_time: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            plugin_dir: std::path::PathBuf::from("plugins"),
            hot_reload: false,
            validate: true,
            max_load_time: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_error_display() {
        let err = PluginError::NotFound("test_plugin".to_string());
        assert_eq!(err.to_string(), "Plugin not found: test_plugin");
    }

    #[test]
    fn test_plugin_config_default() {
        let config = PluginConfig::default();
        assert_eq!(config.plugin_dir, std::path::PathBuf::from("plugins"));
        assert_eq!(config.hot_reload, false);
        assert_eq!(config.validate, true);
        assert_eq!(config.max_load_time, 30);
    }
}