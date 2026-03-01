//! Vantis Media Player - Example Rust Plugin
//!
//! This example demonstrates how to create a plugin for Vantis Media Player
//! using Rust and the Wasm Interface Types (WIT) format.
//!
//! The plugin provides:
//! - Audio visualization
//! - Playback statistics
//! - Custom commands

use std::time::{Duration, Instant};

wit_bindgen::generate!({
    exports: {
        "vantis:plugin/plugin": Plugin
    }
});

/// Plugin state
struct PluginState {
    frame_count: u64,
    start_time: Option<Instant>,
    last_log_time: Instant,
}

impl PluginState {
    fn new() -> Self {
        Self {
            frame_count: 0,
            start_time: None,
            last_log_time: Instant::now(),
        }
    }
}

/// Plugin implementation
pub struct Plugin {
    state: PluginState,
}

impl Plugin {
    /// Create a new plugin instance
    pub fn new() -> Self {
        Self {
            state: PluginState::new(),
        }
    }

    /// Calculate frames per second
    fn calculate_fps(&mut self) -> f64 {
        let now = Instant::now();
        
        if let Some(start_time) = self.state.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                self.state.frame_count as f64 / elapsed
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

/// Plugin trait implementation
impl vantis::plugin::plugin::Plugin for Plugin {
    /// Called when the plugin is loaded
    fn init(&mut self) -> Result<(), String> {
        self.state.start_time = Some(Instant::now());
        
        log_info("Example Rust Plugin initialized!");
        log_info("Version: 0.1.0");
        log_info("Author: Vantis Team");
        
        Ok(())
    }

    /// Called periodically (approximately 60 times per second)
    fn tick(&mut self, _delta_time: f64) {
        self.state.frame_count += 1;
        
        // Log FPS every 5 seconds
        let now = Instant::now();
        if now.duration_since(self.state.last_log_time) >= Duration::from_secs(5) {
            let fps = self.calculate_fps();
            log_info(&format!("Current FPS: {:.1}", fps));
            log_info(&format!("Total frames: {}", self.state.frame_count));
            
            self.state.last_log_time = now;
        }
    }

    /// Called when a custom command is received
    fn handle_command(&mut self, command: String, args: Vec<String>) -> String {
        log_info(&format!("Received command: {}", command));
        
        match command.as_str() {
            "ping" => {
                log_info("Pong!");
                "Pong!".to_string()
            }
            
            "stats" => {
                let fps = self.calculate_fps();
                let stats = format!(
                    "Stats:\n  FPS: {:.1}\n  Frames: {}\n  Uptime: {:.1}s",
                    fps,
                    self.state.frame_count,
                    self.state.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0)
                );
                log_info(&stats);
                stats
            }
            
            "echo" => {
                let message = args.join(" ");
                log_info(&format!("Echo: {}", message));
                format!("Echo: {}", message)
            }
            
            "reset" => {
                self.state.frame_count = 0;
                self.state.start_time = Some(Instant::now());
                log_info("Stats reset!");
                "Stats reset!".to_string()
            }
            
            _ => {
                let error = format!("Unknown command: {}", command);
                log_error(&error);
                error
            }
        }
    }

    /// Called when media info is available
    fn on_media_loaded(&mut self, info: vantis::plugin::types::MediaInfo) {
        log_info(&format!("Media loaded: {}", info.title));
        log_info(&format!("  Duration: {:.1}s", info.duration));
        log_info(&format!("  Video: {}x{}", info.width, info.height));
        log_info(&format!("  Audio: {} channels, {}Hz", info.audio_channels, info.audio_sample_rate));
    }

    /// Called when playback state changes
    fn on_playback_state_changed(&mut self, state: vantis::plugin::types::PlaybackState) {
        match state {
            vantis::plugin::types::PlaybackState::Playing => {
                log_info("Playback started");
            }
            vantis::plugin::types::PlaybackState::Paused => {
                log_info("Playback paused");
            }
            vantis::plugin::types::PlaybackState::Stopped => {
                log_info("Playback stopped");
            }
            vantis::plugin::types::PlaybackState::Seeking => {
                log_info("Seeking...");
            }
        }
    }

    /// Called when playback position changes
    fn on_position_changed(&mut self, position: f64) {
        // Log position every 10 seconds
        if position as u64 % 10 == 0 && position > 0.0 {
            log_info(&format!("Position: {:.1}s", position));
        }
    }

    /// Called when volume changes
    fn on_volume_changed(&mut self, volume: f64) {
        log_info(&format!("Volume: {:.0}%", volume * 100.0));
    }

    /// Called when the plugin is unloaded
    fn shutdown(&mut self) {
        log_info("Example Rust Plugin shutting down...");
        log_info(&format!("Final stats: {} frames, {:.1} FPS average", 
            self.state.frame_count,
            self.calculate_fps()
        ));
    }
}

/// Plugin factory function
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut Plugin {
    let plugin = Box::new(Plugin::new());
    Box::into_raw(plugin)
}

/// Plugin destroy function
#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut Plugin) {
    if !plugin.is_null() {
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}

// Logging helper functions
fn log_info(message: &str) {
    unsafe {
        vantis_log(0, message.as_ptr(), message.len() as u32);
    }
}

fn log_error(message: &str) {
    unsafe {
        vantis_log(3, message.as_ptr(), message.len() as u32);
    }
}

// External host functions (provided by Vantis Media Player)
extern "C" {
    fn vantis_log(level: u32, message: *const u8, len: u32);
}