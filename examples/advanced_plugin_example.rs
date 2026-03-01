// Example: Advanced plugin with complex functionality
//
// This example demonstrates an advanced plugin that provides
// audio visualization and real-time analysis capabilities.

use std::collections::HashMap;
use std::time::{Duration, Instant};

wit_bindgen::generate!({
    exports: {
        "vantis:plugin/plugin": Plugin
    }
});

/// Audio visualization data
struct AudioVisualization {
    spectrum: Vec<f32>,
    waveform: Vec<f32>,
    peak_level: f32,
    rms_level: f32,
}

/// Plugin state
struct PluginState {
    visualizations: HashMap<String, AudioVisualization>,
    analysis_enabled: bool,
    last_analysis_time: Instant,
    frame_count: u64,
}

impl PluginState {
    fn new() -> Self {
        Self {
            visualizations: HashMap::new(),
            analysis_enabled: true,
            last_analysis_time: Instant::now(),
            frame_count: 0,
        }
    }
}

/// Advanced plugin implementation
pub struct Plugin {
    state: PluginState,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            state: PluginState::new(),
        }
    }

    /// Analyze audio data
    fn analyze_audio(&mut self, audio_data: &[f32]) -> AudioVisualization {
        // Calculate spectrum (simplified FFT)
        let spectrum = self.calculate_spectrum(audio_data);
        
        // Calculate waveform
        let waveform = audio_data.to_vec();
        
        // Calculate peak level
        let peak_level = audio_data.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        
        // Calculate RMS level
        let rms_level = (audio_data.iter().map(|&x| x * x).sum::<f32>() / audio_data.len() as f32).sqrt();
        
        AudioVisualization {
            spectrum,
            waveform,
            peak_level,
            rms_level,
        }
    }

    /// Calculate audio spectrum (simplified)
    fn calculate_spectrum(&self, audio_data: &[f32]) -> Vec<f32> {
        // This is a simplified spectrum calculation
        // In a real implementation, you'd use FFT
        let bands = 32;
        let chunk_size = audio_data.len() / bands;
        
        let mut spectrum = Vec::with_capacity(bands);
        
        for i in 0..bands {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(audio_data.len());
            let chunk = &audio_data[start..end];
            
            let avg = chunk.iter().map(|&x| x.abs()).sum::<f32>() / chunk.len() as f32;
            spectrum.push(avg);
        }
        
        spectrum
    }

    /// Detect beats in audio
    fn detect_beat(&self, spectrum: &[f32]) -> bool {
        // Simple beat detection based on low-frequency energy
        let low_freq_energy: f32 = spectrum.iter().take(4).sum();
        let threshold = 0.5;
        
        low_freq_energy > threshold
    }

    /// Generate visualization data
    fn generate_visualization(&self, viz: &AudioVisualization) -> String {
        let mut output = String::new();
        
        output.push_str("Audio Visualization:\n");
        output.push_str(&format!("  Peak Level: {:.2} dB\n", 20.0 * viz.peak_level.log10()));
        output.push_str(&format!("  RMS Level: {:.2} dB\n", 20.0 * viz.rms_level.log10()));
        
        output.push_str("\nSpectrum:\n");
        for (i, &value) in viz.spectrum.iter().enumerate() {
            let bars = (value * 20.0) as usize;
            output.push_str(&format!("  {:2}: ", i));
            for _ in 0..bars {
                output.push('█');
            }
            output.push('\n');
        }
        
        output
    }
}

impl vantis::plugin::plugin::Plugin for Plugin {
    fn init(&mut self) -> Result<(), String> {
        log_info("Advanced Audio Visualization Plugin initialized!");
        log_info("Features:");
        log_info("  - Real-time spectrum analysis");
        log_info("  - Waveform visualization");
        log_info("  - Peak/RMS level monitoring");
        log_info("  - Beat detection");
        
        Ok(())
    }

    fn tick(&mut self, _delta_time: f64) {
        self.state.frame_count += 1;
        
        // Log statistics every 5 seconds
        let now = Instant::now();
        if now.duration_since(self.state.last_analysis_time) >= Duration::from_secs(5) {
            log_info(&format!("Processed {} frames", self.state.frame_count));
            log_info(&format!("Active visualizations: {}", self.state.visualizations.len()));
            
            self.state.last_analysis_time = now;
        }
    }

    fn handle_command(&mut self, command: String, args: Vec<String>) -> String {
        log_info(&format!("Received command: {}", command));
        
        match command.as_str() {
            "enable_analysis" => {
                self.state.analysis_enabled = true;
                log_info("Audio analysis enabled");
                "Analysis enabled".to_string()
            }
            
            "disable_analysis" => {
                self.state.analysis_enabled = false;
                log_info("Audio analysis disabled");
                "Analysis disabled".to_string()
            }
            
            "get_visualization" => {
                if let Some(viz) = self.state.visualizations.get("main") {
                    self.generate_visualization(viz)
                } else {
                    "No visualization data available".to_string()
                }
            }
            
            "clear_visualizations" => {
                self.state.visualizations.clear();
                log_info("Visualizations cleared");
                "Visualizations cleared".to_string()
            }
            
            "stats" => {
                let stats = format!(
                    "Stats:\n  Frames: {}\n  Visualizations: {}\n  Analysis: {}",
                    self.state.frame_count,
                    self.state.visualizations.len(),
                    if self.state.analysis_enabled { "enabled" } else { "disabled" }
                );
                log_info(&stats);
                stats
            }
            
            _ => {
                let error = format!("Unknown command: {}", command);
                log_error(&error);
                error
            }
        }
    }

    fn on_audio_data(&mut self, data: Vec<f32>, channels: u32, sample_rate: u32) {
        if !self.state.analysis_enabled {
            return;
        }
        
        // Analyze audio data
        let visualization = self.analyze_audio(&data);
        
        // Detect beat
        if self.detect_beat(&visualization.spectrum) {
            log_info("BEAT DETECTED!");
        }
        
        // Store visualization
        self.state.visualizations.insert(
            "main".to_string(),
            visualization
        );
    }

    fn on_media_loaded(&mut self, info: vantis::plugin::types::MediaInfo) {
        log_info(&format!("Media loaded: {}", info.title));
        log_info(&format!("  Audio: {} channels, {}Hz", info.audio_channels, info.audio_sample_rate));
        
        // Clear previous visualizations
        self.state.visualizations.clear();
    }

    fn on_playback_state_changed(&mut self, state: vantis::plugin::types::PlaybackState) {
        match state {
            vantis::plugin::types::PlaybackState::Playing => {
                log_info("Playback started - audio analysis active");
            }
            vantis::plugin::types::PlaybackState::Paused => {
                log_info("Playback paused - audio analysis paused");
            }
            vantis::plugin::types::PlaybackState::Stopped => {
                log_info("Playback stopped - audio analysis stopped");
                self.state.visualizations.clear();
            }
            _ => {}
        }
    }

    fn shutdown(&mut self) {
        log_info("Advanced Audio Visualization Plugin shutting down...");
        log_info(&format!("Final stats: {} frames processed", self.state.frame_count));
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
        vantis_log(1, message.as_ptr(), message.len() as u32);
    }
}

fn log_error(message: &str) {
    unsafe {
        vantis_log(3, message.as_ptr(), message.len() as u32);
    }
}

// External host functions
extern "C" {
    fn vantis_log(level: u32, message: *const u8, len: u32);
}