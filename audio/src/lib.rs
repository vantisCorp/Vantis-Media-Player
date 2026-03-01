//! Vantis Audio Engine
//! 
//! Bit-perfect audio playback with exclusive mode,
/// loudness normalization, and advanced audio processing.

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig, SampleFormat, SupportedStreamConfig,
};
use parking_lot::RwLock;
use std::sync::Arc;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use tracing::{info, debug, error};

pub mod decoder;
pub mod renderer;
pub mod effects;

/// Audio engine - manages all audio processing
pub struct AudioEngine {
    /// Audio host
    host: Host,
    
    /// Output device
    device: Option<Device>,
    
    /// Stream configuration
    config: Option<StreamConfig>,
    
    /// Sample format
    sample_format: Option<SampleFormat>,
    
    /// Audio decoder
    decoder: Option<decoder::AudioDecoder>,
    
    /// Audio renderer
    renderer: Option<renderer::AudioRenderer>,
    
    /// Audio effects
    effects: Arc<RwLock<effects::AudioEffects>>,
    
    /// Volume (0.0 - 1.0)
    volume: Arc<RwLock<f32>>,
    
    /// Muted state
    muted: Arc<RwLock<bool>>,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Result<Self> {
        info!("🔊 Initializing Vantis Audio Engine");
        
        // Get default host
        let host = cpal::default_host();
        debug!("🎧 Audio Host: {:?}", host.id());
        
        // List output devices
        let devices = host
            .output_devices()
            .map_err(|e| anyhow::anyhow!("Failed to enumerate audio devices: {}", e))?;
        
        info!("📡 Available audio output devices:");
        for (i, device) in devices.enumerate() {
            if let Ok(name) = device.name() {
                info!("   [{}] {}", i, name);
            }
        }
        
        Ok(Self {
            host,
            device: None,
            config: None,
            sample_format: None,
            decoder: None,
            renderer: None,
            effects: Arc::new(RwLock::new(effects::AudioEffects::new())),
            volume: Arc::new(RwLock::new(1.0)),
            muted: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Initialize output device
    pub fn initialize_device(&mut self, device_name: Option<&str>) -> Result<()> {
        info!("🎧 Initializing audio device...");
        
        // Get default or specific device
        let device = if let Some(name) = device_name {
            self.find_device_by_name(name)?
        } else {
            self.host
                .default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No default output device found"))?
        };
        
        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        info!("   Selected device: {}", device_name);
        
        // Get default config
        let default_config = device
            .default_output_config()
            .map_err(|e| anyhow::anyhow!("Failed to get default output config: {}", e))?;
        
        debug!("   Config: {:?}", default_config);
        
        let config = StreamConfig {
            channels: default_config.channels(),
            sample_rate: default_config.sample_rate(),
            buffer_size: default_config.buffer_size(),
        };
        
        let sample_format = default_config.sample_format();
        
        self.device = Some(device);
        self.config = Some(config);
        self.sample_format = Some(sample_format);
        
        // Initialize renderer
        let device = self.device.as_ref().unwrap();
        self.renderer = Some(renderer::AudioRenderer::new(device, config, sample_format)?);
        
        info!("✅ Audio device initialized");
        info!("   - Sample Rate: {} Hz", config.sample_rate.0);
        info!("   - Channels: {}", config.channels);
        info!("   - Format: {:?}", sample_format);
        
        Ok(())
    }
    
    /// Load an audio file
    pub fn load_audio(&mut self, path: &str) -> Result<()> {
        info!("📂 Loading audio: {}", path);
        
        let source = Box::new(std::fs::File::open(path)?);
        let mss = MediaSourceStream::new(source, Default::default());
        
        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)?;
        
        let format = probed.format;
        
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No valid audio track found"))?;
        
        let decoder_opts = DecoderOptions {
            verify: false,
            ..Default::default()
        };
        
        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)?;
        
        self.decoder = Some(decoder::AudioDecoder::new(format, decoder, track.id));
        
        info!("✅ Audio loaded successfully");
        
        Ok(())
    }
    
    /// Start playback
    pub fn play(&mut self) -> Result<()> {
        info!("▶ Starting audio playback");
        if let Some(ref mut renderer) = self.renderer {
            renderer.start()?;
        }
        Ok(())
    }
    
    /// Pause playback
    pub fn pause(&mut self) -> Result<()> {
        info!("⏸ Pausing audio playback");
        if let Some(ref mut renderer) = self.renderer {
            renderer.pause()?;
        }
        Ok(())
    }
    
    /// Stop playback
    pub fn stop(&mut self) -> Result<()> {
        info!("⏹ Stopping audio playback");
        if let Some(ref mut renderer) = self.renderer {
            renderer.stop()?;
        }
        Ok(())
    }
    
    /// Set volume
    pub fn set_volume(&self, volume: f32) {
        let mut vol = self.volume.write();
        *vol = volume.clamp(0.0, 1.0);
        debug!("🔊 Volume set to {:.0}%", *vol * 100.0);
    }
    
    /// Get volume
    pub fn volume(&self) -> f32 {
        *self.volume.read()
    }
    
    /// Set muted state
    pub fn set_muted(&self, muted: bool) {
        let mut m = self.muted.write();
        *m = muted;
        debug!("🔇 Muted: {}", *m);
    }
    
    /// Get muted state
    pub fn is_muted(&self) -> bool {
        *self.muted.read()
    }
    
    /// Find device by name
    fn find_device_by_name(&self, name: &str) -> Result<Device> {
        let devices = self.host.output_devices()?;
        
        for device in devices {
            if let Ok(device_name) = device.name() {
                if device_name.contains(name) {
                    return Ok(device);
                }
            }
        }
        
        Err(anyhow::anyhow!("Audio device '{}' not found", name))
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create audio engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_engine_creation() {
        let engine = AudioEngine::new();
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_audio_engine_default() {
        let engine = AudioEngine::default();
        assert_eq!(engine.volume(), 1.0);
        assert_eq!(engine.is_muted(), false);
    }
    
    #[test]
    fn test_set_volume() {
        let engine = AudioEngine::new().unwrap();
        engine.set_volume(0.5);
        assert_eq!(engine.volume(), 0.5);
        
        // Test clamping
        engine.set_volume(1.5);
        assert_eq!(engine.volume(), 1.0);
        
        engine.set_volume(-0.5);
        assert_eq!(engine.volume(), 0.0);
    }
    
    #[test]
    fn test_set_muted() {
        let engine = AudioEngine::new().unwrap();
        assert_eq!(engine.is_muted(), false);
        
        engine.set_muted(true);
        assert_eq!(engine.is_muted(), true);
        
        engine.set_muted(false);
        assert_eq!(engine.is_muted(), false);
    }
}