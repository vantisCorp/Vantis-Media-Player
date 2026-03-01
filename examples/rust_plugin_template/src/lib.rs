//! Vantis Media Player - Rust Plugin Template
//! 
//! This is a template for creating plugins in Rust for Vantis.
//! Plugins are compiled to WebAssembly and run in a sandbox.

use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

/// Plugin state
thread_local! {
    static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
    static PLUGIN_NAME: RefCell<String> = RefCell::new("Vantis Example Plugin".to_string());
    static PLUGIN_VERSION: RefCell<String> = RefCell::new("1.0.0".to_string());
}

/// Initialize the plugin
/// 
/// Called when the plugin is loaded by Vantis.
/// Returns 0 on success, non-zero on error.
#[wasm_bindgen]
pub fn init() -> i32 {
    web_sys::console::log_1(&"Plugin: init() called".into());
    
    IS_INITIALIZED.with(|init| {
        init.store(true, Ordering::SeqCst);
    });
    
    // Perform initialization tasks
    web_sys::console::log_1(&"Plugin initialized successfully!".into());
    
    0 // Success
}

/// Shutdown the plugin
/// 
/// Called when Vantis is shutting down or the plugin is being unloaded.
/// Returns 0 on success, non-zero on error.
#[wasm_bindgen]
pub fn shutdown() -> i32 {
    web_sys::console::log_1(&"Plugin: shutdown() called".into());
    
    IS_INITIALIZED.with(|init| {
        init.store(false, Ordering::SeqCst);
    });
    
    // Cleanup resources
    web_sys::console::log_1(&"Plugin shutdown complete!".into());
    
    0 // Success
}

/// Tick function
/// 
/// Called periodically by Vantis (e.g., every frame or second).
/// Use this for periodic tasks like updates or animations.
/// Returns 0 on success, non-zero on error.
#[wasm_bindgen]
pub fn tick() -> i32 {
    // This is called periodically
    // Perform periodic work here
    
    0 // Success
}

/// Get plugin name
#[wasm_bindgen]
pub fn get_name() -> String {
    PLUGIN_NAME.with(|name| name.borrow().clone())
}

/// Get plugin version
#[wasm_bindgen]
pub fn get_version() -> String {
    PLUGIN_VERSION.with(|version| version.borrow().clone())
}

/// Get plugin description
#[wasm_bindgen]
pub fn get_description() -> String {
    "Example plugin for Vantis Media Player".to_string()
}

/// Custom function: Process audio data
/// 
/// Example of a custom function that can be called by Vantis.
/// This could be used for audio visualization, effects, etc.
#[wasm_bindgen]
pub fn process_audio(samples: &[f32]) -> Vec<f32> {
    // Example: Apply a simple gain to audio samples
    let gain = 1.2;
    samples.iter().map(|s| (s * gain).min(1.0).max(-1.0)).collect()
}

/// Custom function: Get metadata
/// 
/// Returns plugin-specific metadata as JSON string.
#[wasm_bindgen]
pub fn get_metadata() -> String {
    use serde_json::json;
    
    let metadata = json!({
        "name": get_name(),
        "version": get_version(),
        "description": get_description(),
        "author": "Your Name",
        "license": "MIT",
        "capabilities": [
            "audio_processing",
            "visualization",
            "metadata_extraction"
        ]
    });
    
    metadata.to_string()
}

/// Custom function: Configure plugin
/// 
/// Accepts configuration as JSON string.
#[wasm_bindgen]
pub fn configure(config_json: String) -> bool {
    match serde_json::from_str::<serde_json::Value>(&config_json) {
        Ok(config) => {
            web_sys::console::log_2(
                &"Plugin configured with:".into(),
                &config.to_string().into()
            );
            true
        }
        Err(e) => {
            web_sys::console::error_2(
                &"Failed to parse config:".into(),
                &e.to_string().into()
            );
            false
        }
    }
}

/// Custom function: Get statistics
/// 
/// Returns plugin statistics as JSON string.
#[wasm_bindgen]
pub fn get_statistics() -> String {
    use serde_json::json;
    
    let stats = json!({
        "initialized": IS_INITIALIZED.with(|i| i.load(Ordering::SeqCst)),
        "tick_count": 0, // You can track this in your plugin
        "memory_used": 1024, // Example value
    });
    
    stats.to_string()
}

// Unit tests (run with wasm-bindgen-test)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialization() {
        assert_eq!(init(), 0);
        assert!(IS_INITIALIZED.with(|i| i.load(Ordering::SeqCst)));
    }
    
    #[test]
    fn test_shutdown() {
        init();
        assert_eq!(shutdown(), 0);
        assert!(!IS_INITIALIZED.with(|i| i.load(Ordering::SeqCst)));
    }
    
    #[test]
    fn test_audio_processing() {
        let input = vec![0.5, 0.0, -0.5];
        let output = process_audio(&input);
        
        // Output should be gain-adjusted
        assert_eq!(output.len(), input.len());
        assert!(output[0] > input[0]); // Increased by gain
    }
    
    #[test]
    fn test_metadata() {
        let metadata = get_metadata();
        
        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&metadata).unwrap();
        
        // Should have required fields
        assert!(parsed["name"].is_string());
        assert!(parsed["version"].is_string());
    }
}

// Export for use in JavaScript/Host
#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Vantis Example Plugin loaded!".into());
}