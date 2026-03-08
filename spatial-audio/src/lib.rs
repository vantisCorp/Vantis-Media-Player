//! Spatial Audio Processing for Vantis Media Player
//!
//! This module provides immersive 3D audio capabilities including:
//! - Head-Related Transfer Function (HRTF) for binaural audio
//! - Ambisonics for surround sound decoding/encoding
//! - Room simulation and reverberation
//! - Real-time listener and source positioning
//!
//! # Example
//!
//! ```
//! use vantis_spatial_audio::{SpatialAudioProcessor, Listener, AudioSource};
//!
//! let mut processor = SpatialAudioProcessor::new(48000)?;
//! processor.set_listener(Listener::new([0.0, 0.0, 0.0]));
//! processor.add_source(AudioSource::new("music", [0.0, 0.0, -1.0]));
//!
//! let output = processor.process(&input_samples)?;
//! ```

pub mod hrtf;
pub mod ambisonics;
pub mod reverb;
pub mod listener;
pub mod types;
pub mod error;

pub use error::SpatialAudioError;
pub use types::{AudioBuffer, Sample, Position, Orientation, AudioFormat};
pub use listener::{Listener, AudioSource, AudioScene};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result type for spatial audio operations
pub type SpatialResult<T> = Result<T, SpatialAudioError>;

/// Sample rate for audio processing
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SampleRate(pub u32);

impl Default for SampleRate {
    fn default() -> Self {
        Self(48000)
    }
}

impl SampleRate {
    pub fn new(rate: u32) -> Self {
        Self(rate)
    }
    
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

/// Main spatial audio processor
pub struct SpatialAudioProcessor {
    /// Sample rate
    sample_rate: SampleRate,
    
    /// Audio scene with listener and sources
    scene: AudioScene,
    
    /// HRTF processor for binaural rendering
    hrtf_processor: Option<hrtf::HrtfProcessor>,
    
    /// Ambisonics decoder (if configured)
    ambisonics_decoder: Option<ambisonics::AmbisonicsDecoder>,
    
    /// Reverb processor
    reverb_processor: Option<reverb::ReverbProcessor>,
    
    /// Output channel count
    output_channels: usize,
    
    /// Processing statistics
    stats: ProcessingStats,
}

impl SpatialAudioProcessor {
    /// Create a new spatial audio processor
    pub fn new(sample_rate: u32) -> SpatialResult<Self> {
        Ok(Self {
            sample_rate: SampleRate::new(sample_rate),
            scene: AudioScene::new(),
            hrtf_processor: None,
            ambisonics_decoder: None,
            reverb_processor: None,
            output_channels: 2,
            stats: ProcessingStats::default(),
        })
    }
    
    /// Enable HRTF processing for binaural audio
    pub fn enable_hrtf(&mut self, hrtf_dataset: Option<&hrtf::HrtfDataset>) -> SpatialResult<()> {
        self.hrtf_processor = Some(hrtf::HrtfProcessor::new(self.sample_rate, hrtf_dataset)?);
        self.output_channels = 2; // Binaural output
        Ok(())
    }
    
    /// Enable ambisonics decoding
    pub fn enable_ambisonics(&mut self, order: ambisonics::AmbisonicsOrder) -> SpatialResult<()> {
        self.ambisonics_decoder = Some(ambisonics::AmbisonicsDecoder::new(order, self.output_channels)?);
        Ok(())
    }
    
    /// Enable reverb processing
    pub fn enable_reverb(&mut self, config: reverb::ReverbConfig) {
        self.reverb_processor = Some(reverb::ReverbProcessor::new(self.sample_rate, config));
    }
    
    /// Set the listener
    pub fn set_listener(&mut self, listener: Listener) {
        self.scene.set_listener(listener);
    }
    
    /// Get mutable reference to listener
    pub fn listener_mut(&mut self) -> &mut Listener {
        self.scene.listener_mut()
    }
    
    /// Add an audio source
    pub fn add_source(&mut self, source: AudioSource) {
        self.scene.add_source(source);
    }
    
    /// Remove an audio source
    pub fn remove_source(&mut self, id: &str) -> Option<AudioSource> {
        self.scene.remove_source(id)
    }
    
    /// Get an audio source by ID
    pub fn get_source(&self, id: &str) -> Option<&AudioSource> {
        self.scene.get_source(id)
    }
    
    /// Get mutable reference to audio source
    pub fn get_source_mut(&mut self, id: &str) -> Option<&mut AudioSource> {
        self.scene.get_source_mut(id)
    }
    
    /// Set output channel count
    pub fn set_output_channels(&mut self, channels: usize) {
        self.output_channels = channels;
    }
    
    /// Process audio samples
    pub fn process(&mut self, input: &AudioBuffer) -> SpatialResult<AudioBuffer> {
        let start = std::time::Instant::now();
        
        let mut output = AudioBuffer::new(self.output_channels, input.sample_count());
        
        // Process each source
        for source in self.scene.sources_mut() {
            if !source.is_active() {
                continue;
            }
            
            // Calculate distance attenuation
            let distance = source.position().distance_to(self.scene.listener().position());
            let gain = source.calculate_distance_attenuation(distance);
            
            // Apply HRTF if enabled
            if let Some(hrtf) = &mut self.hrtf_processor {
                let relative_pos = source.position().relative_to(self.scene.listener());
                let azimuth = relative_pos.azimuth();
                let elevation = relative_pos.elevation();
                
                let source_output = hrtf.process(input, azimuth, elevation, gain)?;
                
                // Mix into output
                for (ch, samples) in output.channels_mut().iter_mut().enumerate() {
                    for (i, sample) in samples.iter_mut().enumerate() {
                        *sample += source_output.channel(ch).get(i).copied().unwrap_or(0.0);
                    }
                }
            } else {
                // Simple stereo panning without HRTF
                let pan = source.position().x() / 10.0; // Simple left-right pan
                let left_gain = (1.0 - pan.max(0.0)).min(1.0) * gain;
                let right_gain = (1.0 + pan.min(0.0)).min(1.0) * gain;
                
                for (i, sample) in input.channel(0).iter().enumerate() {
                    if let Some(left) = output.channel_mut(0).get_mut(i) {
                        *left += sample * left_gain;
                    }
                    if let Some(right) = output.channel_mut(1).get_mut(i) {
                        *right += sample * right_gain;
                    }
                }
            }
        }
        
        // Apply reverb if enabled
        if let Some(reverb) = &mut self.reverb_processor {
            output = reverb.process(&output)?;
        }
        
        self.stats.frames_processed += 1;
        self.stats.total_process_time += start.elapsed();
        
        Ok(output)
    }
    
    /// Get processing statistics
    pub fn stats(&self) -> &ProcessingStats {
        &self.stats
    }
    
    /// Reset processing statistics
    pub fn reset_stats(&mut self) {
        self.stats = ProcessingStats::default();
    }
    
    /// Get the sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate.as_u32()
    }
}

/// Processing statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessingStats {
    /// Total frames processed
    pub frames_processed: u64,
    
    /// Total processing time
    pub total_process_time: std::time::Duration,
    
    /// Average processing time per frame
    pub avg_process_time_us: f64,
    
    /// Peak CPU usage percentage
    pub peak_cpu_usage: f32,
}

impl ProcessingStats {
    pub fn update(&mut self, process_time: std::time::Duration) {
        self.frames_processed += 1;
        self.total_process_time += process_time;
        self.avg_process_time_us = self.total_process_time.as_micros() as f64 / self.frames_processed as f64;
    }
}

/// Spatial audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioConfig {
    /// Enable HRTF processing
    pub enable_hrtf: bool,
    
    /// Enable ambisonics
    pub enable_ambisonics: bool,
    
    /// Ambisonics order
    pub ambisonics_order: u8,
    
    /// Enable reverb
    pub enable_reverb: bool,
    
    /// Reverb preset
    pub reverb_preset: Option<String>,
    
    /// Output channels
    pub output_channels: usize,
    
    /// Maximum simultaneous sources
    pub max_sources: usize,
    
    /// Distance attenuation model
    pub attenuation_model: AttenuationModel,
}

impl Default for SpatialAudioConfig {
    fn default() -> Self {
        Self {
            enable_hrtf: true,
            enable_ambisonics: false,
            ambisonics_order: 1,
            enable_reverb: true,
            reverb_preset: Some("room".to_string()),
            output_channels: 2,
            max_sources: 32,
            attenuation_model: AttenuationModel::InverseDistance,
        }
    }
}

/// Distance attenuation models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AttenuationModel {
    /// No attenuation
    None,
    
    /// Inverse distance attenuation (1/r)
    InverseDistance,
    
    /// Inverse distance squared (1/r²)
    InverseDistanceSquared,
    
    /// Linear attenuation
    Linear,
    
    /// Exponential attenuation
    Exponential { exponent: f32 },
}

impl AttenuationModel {
    /// Calculate attenuation factor for a given distance
    pub fn calculate(&self, distance: f32, min_distance: f32, max_distance: f32) -> f32 {
        if distance <= min_distance {
            return 1.0;
        }
        if distance >= max_distance {
            return 0.0;
        }
        
        match self {
            Self::None => 1.0,
            Self::InverseDistance => min_distance / distance,
            Self::InverseDistanceSquared => (min_distance / distance).powi(2),
            Self::Linear => 1.0 - (distance - min_distance) / (max_distance - min_distance),
            Self::Exponential { exponent } => (-distance / max_distance).exp() * exponent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_processor_creation() {
        let processor = SpatialAudioProcessor::new(48000);
        assert!(processor.is_ok());
    }
    
    #[test]
    fn test_attenuation_model() {
        let model = AttenuationModel::InverseDistance;
        let attenuation = model.calculate(10.0, 1.0, 100.0);
        assert!((attenuation - 0.1).abs() < 0.01);
    }
}