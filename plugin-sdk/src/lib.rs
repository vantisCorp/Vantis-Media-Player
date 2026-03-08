//! Plugin SDK for Vantis Media Player.
//!
//! This crate provides a comprehensive plugin system for extending
//! Vantis Media Player functionality. It supports both host-side
//! plugin loading/management and plugin-side development.
//!
//! # Architecture
//!
//! The plugin system is built around several key concepts:
//!
//! - **Plugin**: A dynamically loaded library that implements the `Plugin` trait
//! - **Host**: The application that loads and manages plugins
//! - **Lifecycle**: The stages a plugin goes through (load, initialize, run, shutdown)
//! - **API**: Interfaces that plugins can use to interact with the host
//!
//! # Example Plugin
//!
//! ```rust,ignore
//! use vantis_plugin_sdk::{Plugin, PluginContext, PluginInfo, PluginResult};
//!
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn info(&self) -> PluginInfo {
//!         PluginInfo {
//!             id: "com.example.my-plugin".into(),
//!             name: "My Plugin".into(),
//!             version: "1.0.0".into(),
//!             ..Default::default()
//!         }
//!     }
//!
//!     fn initialize(&mut self, context: &PluginContext) -> PluginResult<()> {
//!         // Initialize plugin
//!         Ok(())
//!     }
//! }
//!
//! vantis_plugin_sdk::export_plugin!(MyPlugin);
//! ```

pub mod error;
pub mod plugin;
pub mod host;
pub mod lifecycle;
pub mod api;
pub mod ffi;
pub mod manifest;
pub mod types;
pub mod debugging;

// Re-exports for convenience
pub use error::{PluginError, PluginResult};
pub use plugin::{Plugin, PluginInfo, PluginContext, PluginCapabilities};
pub use host::{PluginHost, PluginRegistry, PluginLoader};
pub use lifecycle::{PluginLifecycle, PluginState, StateTransition};
pub use api::{PluginApi, HostApi, MediaApi, UiApi};
pub use manifest::{PluginManifest, PluginDependency};
pub use types::*;
pub use debugging::{
    DebugLogLevel, DebugLogMessage, Breakpoint, BreakpointHit,
    PluginState as DebugPluginState, PluginStatus, StackFrame, MemoryRegion,
    PluginDebugController, DebugConfig, DebugSession, PluginDebugger,
    PluginDebugLogger,
};

/// Current SDK version.
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Minimum supported host version.
pub const MIN_HOST_VERSION: &str = "1.0.0";

/// Plugin ABI version for compatibility checking.
pub const ABI_VERSION: u32 = 1;

/// Declare a plugin entry point.
///
/// This macro creates the required FFI functions that the host
/// uses to load and interact with the plugin.
///
/// # Example
///
/// ```rust,ignore
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     // ...
/// }
///
/// vantis_plugin_sdk::export_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! export_plugin {
    ($plugin_type:ty) => {
        /// Plugin creation entry point.
        #[no_mangle]
        pub extern "C" fn vantis_plugin_create() -> *mut dyn $crate::Plugin {
            let plugin: Box<dyn $crate::Plugin> = Box::new(<$plugin_type>::new());
            Box::into_raw(plugin)
        }

        /// Plugin destruction entry point.
        #[no_mangle]
        pub extern "C" fn vantis_plugin_destroy(plugin: *mut dyn $crate::Plugin) {
            if !plugin.is_null() {
                unsafe {
                    drop(Box::from_raw(plugin));
                }
            }
        }

        /// Get SDK version.
        #[no_mangle]
        pub extern "C" fn vantis_plugin_sdk_version() -> *const std::os::raw::c_char {
            $crate::SDK_VERSION.as_ptr() as *const std::os::raw::c_char
        }

        /// Get ABI version.
        #[no_mangle]
        pub extern "C" fn vantis_plugin_abi_version() -> u32 {
            $crate::ABI_VERSION
        }
    };
}

/// Check if the SDK version is compatible with the host.
pub fn is_compatible(host_version: &str) -> bool {
    use semver::Version;
    
    if let (Ok(host), Ok(sdk)) = (Version::parse(host_version), Version::parse(SDK_VERSION)) {
        host.major == sdk.major && host.minor >= sdk.minor.saturating_sub(1)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatibility() {
        assert!(is_compatible("1.0.0"));
        assert!(is_compatible("1.1.0"));
        assert!(!is_compatible("0.9.0"));
        assert!(!is_compatible("2.0.0"));
    }
}