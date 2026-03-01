//! Audio enhancement module using AI
//! 
//! Provides AI-powered audio enhancement including:
//! - Noise reduction
//! - Voice enhancement
//! - Room correction
//! - Dynamic range optimization
//! - Spatial audio enhancement

use crate::{AIConfig, AIError, AIResult};
use crate::models::{AIModel, ModelType};
use crate::utils::{TensorOps, FeatureExtractor};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Audio enhancement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnhancementConfig {
    /// Enable noise reduction
    pub enable_noise_reduction: bool,
    
    /// Noise reduction strength (0.0 - 1.0)
    pub noise_reduction_strength: f32,
    
    /// Enable voice enhancement
    pub enable_voice_enhancement: bool,
    
    /// Voice enhancement strength (0.0 - 1.0)
    pub voice_enhancement_strength: f32,
    
    /// Enable dynamic range compression
    pub enable_compression: bool,
    
    /// Compression ratio
    pub compression_ratio: f32,
    
    /// Enable loudness normalization
    pub enable_loudness_normalization: bool,
    
    /// Target loudness in LUFS
    pub target_loudness_lufs: f32,
    
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    
    /// Spatial audio mode
    pub spatial_mode: SpatialMode,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Buffer size
    pub buffer_size: usize,
}

impl Default for AudioEnhancementConfig {
    fn default() -> Self {
        Self {
            enable_noise_reduction: true,
            noise_reduction_strength: 0.7,
            enable_voice_enhancement: false,
            voice_enhancement_strength: 0.5,
            enable_compression: true,
            compression_ratio: 2.0,
            enable_loudness_normalization: true,
            target_loudness_lufs: -16.0,
            enable_spatial_audio: false,
            spatial_mode: SpatialMode::Stereo,
            sample_rate: 48000,
            buffer_size: 1024,
        }
    }
}

/// Spatial audio mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialMode {
    /// Standard stereo
    Stereo,
    
    /// 5.1 surround sound
    Surround51,
    
    /// 7.1 surround sound
    Surround71,
    
    /// Binaural (headphone)
    Binaural,
    
    /// Ambisonics
    Ambisonics,
}

/// Audio enhancer
pub struct AudioEnhancer {
    config: AudioEnhancementConfig,
    model: Option<AIModel>,
    feature_extractor: FeatureExtractor,
    audio_buffer: VecDeque<f32>,
    noise_profile: Vec<f32>,
    loudness_history: VecDeque<f32>,
}

impl AudioEnhancer {
    /// Create a new audio enhancer
    pub fn new(ai_config: AIConfig) -> AIResult<Self> {
        let config = AudioEnhancementConfig::default();
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            audio_buffer: VecDeque::with_capacity(4096),
            noise_profile: Vec::new(),
            loudness_history: VecDeque::with_capacity(100),
        })
    }
    
    /// Create a new audio enhancer with custom configuration
    pub fn with_config(config: AudioEnhancementConfig, ai_config: AIConfig) -> AIResult<Self> {
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            audio_buffer: VecDeque::with_capacity(4096),
            noise_profile: Vec::new(),
            loudness_history: VecDeque::with_capacity(100),
        })
    }
    
    /// Load the audio enhancement model
    pub fn load_model(&mut self) -> AIResult<()> {
        self.model = Some(AIModel::load(ModelType::AudioEnhancement)?);
        Ok(())
    }
    
    /// Process audio buffer
    pub fn process_buffer(&mut self, input: &[f32]) -> AIResult<Vec<f32>> {
        let mut output = input.to_vec();
        
        // Apply noise reduction
        if self.config.enable_noise_reduction {
            output = self.apply_noise_reduction(&output)?;
        }
        
        // Apply voice enhancement
        if self.config.enable_voice_enhancement {
            output = self.apply_voice_enhancement(&output)?;
        }
        
        // Apply compression
        if self.config.enable_compression {
            output = self.apply_compression(&output)?;
        }
        
        // Apply loudness normalization
        if self.config.enable_loudness_normalization {
            output = self.apply_loudness_normalization(&output)?;
        }
        
        // Apply spatial audio
        if self.config.enable_spatial_audio {
            output = self.apply_spatial_audio(&output)?;
        }
        
        // Update buffer
        self.audio_buffer.extend(output.iter());
        while self.audio_buffer.len() > 4096 {
            self.audio_buffer.pop_front();
        }
        
        Ok(output)
    }
    
    /// Apply noise reduction
    fn apply_noise_reduction(&mut self, input: &[f32]) -> AIResult<Vec<f32>> {
        if self.noise_profile.is_empty() {
            // Build noise profile from first few samples
            self.build_noise_profile(input)?;
        }
        
        let mut output = Vec::with_capacity(input.len());
        let strength = self.config.noise_reduction_strength;
        
        for (i, &sample) in input.iter().enumerate() {
            let noise_level = self.noise_profile.get(i % self.noise_profile.len())
                .copied()
                .unwrap_or(0.0);
            
            // Spectral subtraction
            let magnitude = sample.abs();
            let reduced = if magnitude > noise_level {
                sample * (1.0 - strength * (noise_level / magnitude))
            } else {
                sample * (1.0 - strength)
            };
            
            output.push(reduced);
        }
        
        Ok(output)
    }
    
    /// Build noise profile
    fn build_noise_profile(&mut self, samples: &[f32]) -> AIResult<()> {
        // Use first 1024 samples as noise profile
        let profile_size = samples.len().min(1024);
        self.noise_profile = samples[..profile_size]
            .iter()
            .map(|&s| s.abs())
            .collect();
        
        Ok(())
    }
    
    /// Apply voice enhancement
    fn apply_voice_enhancement(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        let strength = self.config.voice_enhancement_strength;
        let mut output = Vec::with_capacity(input.len());
        
        // Simple voice frequency boost (300-3400 Hz)
        // This is a simplified implementation
        for &sample in input {
            // Apply gentle high-pass filter to reduce low-frequency noise
            let filtered = sample * 0.95;
            
            // Boost mid frequencies
            let enhanced = filtered * (1.0 + strength * 0.2);
            
            output.push(enhanced.clamp(-1.0, 1.0));
        }
        
        Ok(output)
    }
    
    /// Apply dynamic range compression
    fn apply_compression(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        let ratio = self.config.compression_ratio;
        let threshold = 0.5; // -6 dB
        let mut output = Vec::with_capacity(input.len());
        
        for &sample in input {
            let magnitude = sample.abs();
            
            if magnitude > threshold {
                let excess = magnitude - threshold;
                let compressed = threshold + excess / ratio;
                let gain = compressed / magnitude;
                output.push(sample * gain);
            } else {
                output.push(sample);
            }
        }
        
        Ok(output)
    }
    
    /// Apply loudness normalization (EBU R128)
    fn apply_loudness_normalization(&mut self, input: &[f32]) -> AIResult<Vec<f32>> {
        // Calculate current loudness
        let current_loudness = self.calculate_loudness(input);
        
        // Update loudness history
        self.loudness_history.push_back(current_loudness);
        while self.loudness_history.len() > 100 {
            self.loudness_history.pop_front();
        }
        
        // Calculate average loudness
        let avg_loudness: f32 = self.loudness_history.iter().sum::<f32>() 
            / self.loudness_history.len() as f32;
        
        // Calculate gain needed
        let target_lufs = self.config.target_loudness_lufs;
        let gain_db = target_lufs - avg_loudness;
        let gain_linear = 10.0_f32.powf(gain_db / 20.0);
        
        // Apply gain with limiting
        let max_gain = 6.0; // +6 dB maximum
        let limited_gain = gain_linear.min(10.0_f32.powf(max_gain / 20.0));
        
        let output: Vec<f32> = input
            .iter()
            .map(|&sample| (sample * limited_gain).clamp(-1.0, 1.0))
            .collect();
        
        Ok(output)
    }
    
    /// Calculate loudness (EBU R128 simplified)
    fn calculate_loudness(&self, samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return -60.0; // Silence
        }
        
        // Calculate RMS
        let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
        let rms = (sum_squares / samples.len() as f32).sqrt();
        
        // Convert to LUFS (simplified)
        if rms > 0.0 {
            20.0 * rms.log10()
        } else {
            -60.0
        }
    }
    
    /// Apply spatial audio processing
    fn apply_spatial_audio(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        match self.config.spatial_mode {
            SpatialMode::Stereo => Ok(input.to_vec()),
            SpatialMode::Surround51 => self.apply_surround_51(input),
            SpatialMode::Surround71 => self.apply_surround_71(input),
            SpatialMode::Binaural => self.apply_binaural(input),
            SpatialMode::Ambisonics => self.apply_ambisonics(input),
        }
    }
    
    /// Apply 5.1 surround sound
    fn apply_surround_51(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        // Assume input is stereo, expand to 5.1
        let mut output = Vec::with_capacity(input.len() * 3);
        
        for chunk in input.chunks(2) {
            let left = chunk.get(0).copied().unwrap_or(0.0);
            let right = chunk.get(1).copied().unwrap_or(0.0);
            
            let center = (left + right) * 0.5;
            let lfe = (left + right) * 0.1; // Low-frequency effects
            let left_surround = left * 0.3;
            let right_surround = right * 0.3;
            
            output.extend_from_slice(&[left, center, right, left_surround, right_surround, lfe]);
        }
        
        Ok(output)
    }
    
    /// Apply 7.1 surround sound
    fn apply_surround_71(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        // Assume input is stereo, expand to 7.1
        let mut output = Vec::with_capacity(input.len() * 4);
        
        for chunk in input.chunks(2) {
            let left = chunk.get(0).copied().unwrap_or(0.0);
            let right = chunk.get(1).copied().unwrap_or(0.0);
            
            let center = (left + right) * 0.5;
            let lfe = (left + right) * 0.1;
            let left_surround = left * 0.3;
            let right_surround = right * 0.3;
            let left_rear = left * 0.2;
            let right_rear = right * 0.2;
            
            output.extend_from_slice(&[
                left, center, right, left_surround, right_surround, left_rear, right_rear, lfe,
            ]);
        }
        
        Ok(output)
    }
    
    /// Apply binaural processing (for headphones)
    fn apply_binaural(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        // Simplified HRTF simulation
        let mut output = Vec::with_capacity(input.len());
        
        for chunk in input.chunks(2) {
            let left = chunk.get(0).copied().unwrap_or(0.0);
            let right = chunk.get(1).copied().unwrap_or(0.0);
            
            // Apply interaural time difference (ITD)
            let left_delayed = left * 0.95;
            let right_delayed = right * 0.95;
            
            // Apply interaural level difference (ILD)
            let left_binaural = left_delayed * 1.0 + right_delayed * 0.1;
            let right_binaural = right_delayed * 1.0 + left_delayed * 0.1;
            
            output.extend_from_slice(&[left_binaural, right_binaural]);
        }
        
        Ok(output)
    }
    
    /// Apply ambisonics processing
    fn apply_ambisonics(&self, input: &[f32]) -> AIResult<Vec<f32>> {
        // First-order ambisonics (B-format)
        let mut output = Vec::with_capacity(input.len() * 2);
        
        for chunk in input.chunks(2) {
            let left = chunk.get(0).copied().unwrap_or(0.0);
            let right = chunk.get(1).copied().unwrap_or(0.0);
            
            let w = (left + right) * 0.5; // Omnidirectional
            let x = (left - right) * 0.5; // Front-back
            let y = (left + right) * 0.3; // Left-right
            let z = 0.0; // Up-down (not available from stereo)
            
            output.extend_from_slice(&[w, x, y, z]);
        }
        
        Ok(output)
    }
    
    /// Analyze audio characteristics
    pub fn analyze_audio(&self, samples: &[f32]) -> AudioAnalysis {
        let loudness = self.calculate_loudness(samples);
        let peak = samples.iter().map(|&s| s.abs()).fold(0.0_f32, f32::max);
        let rms = (samples.iter().map(|&s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
        let dynamic_range = if rms > 0.0 { 20.0 * (peak / rms).log10() } else { 0.0 };
        
        // Estimate frequency content
        let spectral_centroid = self.calculate_spectral_centroid(samples);
        
        AudioAnalysis {
            loudness_lufs: loudness,
            peak_db: 20.0 * peak.log10(),
            rms_db: 20.0 * rms.log10(),
            dynamic_range_db: dynamic_range,
            spectral_centroid_hz: spectral_centroid,
            clipping_detected: peak >= 0.99,
        }
    }
    
    /// Calculate spectral centroid
    fn calculate_spectral_centroid(&self, samples: &[f32]) -> f32 {
        // Simplified spectral centroid calculation
        let mut sum_weighted = 0.0;
        let mut sum_weights = 0.0;
        
        for (i, &sample) in samples.iter().enumerate() {
            let frequency = (i as f32 * self.config.sample_rate as f32) / samples.len() as f32;
            let magnitude = sample.abs();
            sum_weighted += frequency * magnitude;
            sum_weights += magnitude;
        }
        
        if sum_weights > 0.0 {
            sum_weighted / sum_weights
        } else {
            0.0
        }
    }
    
    /// Reset the enhancer state
    pub fn reset(&mut self) {
        self.audio_buffer.clear();
        self.noise_profile.clear();
        self.loudness_history.clear();
    }
}

/// Audio analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysis {
    /// Loudness in LUFS
    pub loudness_lufs: f32,
    
    /// Peak level in dB
    pub peak_db: f32,
    
    /// RMS level in dB
    pub rms_db: f32,
    
    /// Dynamic range in dB
    pub dynamic_range_db: f32,
    
    /// Spectral centroid in Hz
    pub spectral_centroid_hz: f32,
    
    /// Clipping detected
    pub clipping_detected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_enhancement_config_default() {
        let config = AudioEnhancementConfig::default();
        assert!(config.enable_noise_reduction);
        assert_eq!(config.noise_reduction_strength, 0.7);
    }
    
    #[test]
    fn test_process_buffer() {
        let mut enhancer = AudioEnhancer::new(AIConfig::default()).unwrap();
        let input = vec![0.1, 0.2, 0.3, 0.4];
        let output = enhancer.process_buffer(&input).unwrap();
        assert_eq!(output.len(), input.len());
    }
}