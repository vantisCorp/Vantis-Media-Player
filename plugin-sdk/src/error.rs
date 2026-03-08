//! Error types for the plugin system.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for plugin operations.
pub type PluginResult<T> = Result<T, PluginError>;

/// Plugin system errors.
#[derive(Error, Debug)]
pub enum PluginError {
    /// Plugin not found.
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// Plugin already loaded.
    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),

    /// Plugin initialization failed.
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),

    /// Plugin load failed.
    #[error("Failed to load plugin from '{path}': {reason}")]
    LoadFailed {
        /// Path to the plugin.
        path: PathBuf,
        /// Reason for failure.
        reason: String,
    },

    /// Plugin unload failed.
    #[error("Failed to unload plugin '{plugin_id}': {reason}")]
    UnloadFailed {
        /// Plugin identifier.
        plugin_id: String,
        /// Reason for failure.
        reason: String,
    },

    /// Invalid plugin manifest.
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),

    /// Missing plugin manifest.
    #[error("Missing plugin manifest in: {0}")]
    MissingManifest(PathBuf),

    /// Plugin dependency error.
    #[error("Dependency error for plugin '{plugin}': {dependency} {reason}")]
    DependencyError {
        /// Plugin that has the dependency.
        plugin: String,
        /// The dependency that caused the error.
        dependency: String,
        /// Reason for the error.
        reason: String,
    },

    /// Version mismatch.
    #[error("Version mismatch: plugin '{plugin}' requires {required}, but host has {actual}")]
    VersionMismatch {
        /// Plugin identifier.
        plugin: String,
        /// Required version.
        required: String,
        /// Actual version.
        actual: String,
    },

    /// ABI incompatibility.
    #[error("ABI incompatibility: plugin expects ABI {expected}, host provides {actual}")]
    AbiMismatch {
        /// Expected ABI version.
        expected: u32,
        /// Actual ABI version.
        actual: u32,
    },

    /// Symbol not found in plugin library.
    #[error("Symbol '{symbol}' not found in plugin '{plugin}'")]
    SymbolNotFound {
        /// Symbol name.
        symbol: String,
        /// Plugin identifier.
        plugin: String,
    },

    /// Plugin disabled.
    #[error("Plugin '{0}' is disabled")]
    Disabled(String),

    /// Plugin error during execution.
    #[error("Plugin '{plugin}' error: {message}")]
    ExecutionError {
        /// Plugin identifier.
        plugin: String,
        /// Error message.
        message: String,
    },

    /// Permission denied.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Dynamic library loading error.
    #[error("Dynamic library error: {0}")]
    LibraryLoad(String),

    /// Lock acquisition error.
    #[error("Lock acquisition error: {0}")]
    LockError(String),

    /// Invalid state transition.
    #[error("Invalid state transition: {from} -> {to}")]
    InvalidTransition {
        /// Source state.
        from: String,
        /// Target state.
        to: String,
    },

    /// Plugin capability not supported.
    #[error("Plugin '{plugin}' does not support capability '{capability}'")]
    CapabilityNotSupported {
        /// Plugin identifier.
        plugin: String,
        /// Capability name.
        capability: String,
    },

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Timeout error.
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Channel communication error.
    #[error("Channel error: {0}")]
    ChannelError(String),
}

impl PluginError {
    /// Create a load failed error.
    pub fn load_failed(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        PluginError::LoadFailed {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Create an unload failed error.
    pub fn unload_failed(plugin_id: impl Into<String>, reason: impl Into<String>) -> Self {
        PluginError::UnloadFailed {
            plugin_id: plugin_id.into(),
            reason: reason.into(),
        }
    }

    /// Create a dependency error.
    pub fn dependency(
        plugin: impl Into<String>,
        dependency: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        PluginError::DependencyError {
            plugin: plugin.into(),
            dependency: dependency.into(),
            reason: reason.into(),
        }
    }

    /// Create a version mismatch error.
    pub fn version_mismatch(
        plugin: impl Into<String>,
        required: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        PluginError::VersionMismatch {
            plugin: plugin.into(),
            required: required.into(),
            actual: actual.into(),
        }
    }

    /// Create an execution error.
    pub fn execution(plugin: impl Into<String>, message: impl Into<String>) -> Self {
        PluginError::ExecutionError {
            plugin: plugin.into(),
            message: message.into(),
        }
    }

    /// Check if this error is recoverable.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PluginError::Disabled(_)
                | PluginError::DependencyError { .. }
                | PluginError::Timeout(_)
                | PluginError::ConfigurationError(_)
        )
    }

    /// Check if this error indicates the plugin should be disabled.
    pub fn should_disable(&self) -> bool {
        matches!(
            self,
            PluginError::InitializationFailed(_)
                | PluginError::VersionMismatch { .. }
                | PluginError::AbiMismatch { .. }
                | PluginError::SymbolNotFound { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = PluginError::NotFound("test-plugin".to_string());
        assert!(err.to_string().contains("test-plugin"));
    }

    #[test]
    fn test_error_helpers() {
        let err = PluginError::load_failed("/path/to/plugin.so", "missing symbol");
        assert!(matches!(err, PluginError::LoadFailed { .. }));

        let err = PluginError::version_mismatch("test", "1.0.0", "0.9.0");
        assert!(matches!(err, PluginError::VersionMismatch { .. }));
    }

    #[test]
    fn test_error_classification() {
        let err = PluginError::Disabled("test".to_string());
        assert!(err.is_recoverable());
        assert!(!err.should_disable());

        let err = PluginError::InitializationFailed("test".to_string());
        assert!(!err.is_recoverable());
        assert!(err.should_disable());
    }
}