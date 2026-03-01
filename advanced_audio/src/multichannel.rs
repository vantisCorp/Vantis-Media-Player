//! Multi-Channel Audio Processing
//! 
//! Provides channel upmixing, downmixing, bass management, and
//! multi-channel audio processing for various speaker configurations.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};

use crate::ChannelConfiguration;

/// Multi-channel processor
pub struct MultichannelProcessor {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Output channel configuration
    output_config: Arc<RwLock<ChannelConfiguration>>,
    
    /// Enable upmixing
    upmixing_enabled: Arc<RwLock<bool>>,
    
    /// Enable downmixing
    downmixing_enabled: Arc<RwLock<bool>>,
    
    /// Enable bass management
    bass_management_enabled: Arc<RwLock<bool>>,
    
    /// Crossover frequency for bass management
    crossover_frequency: Arc<RwLock<f32>>,
    
    /// Channel mapping
    channel_mapping: Arc<RwLock<ChannelMapping>>,
}

/// Channel mapping for different configurations
#[derive(Debug, Clone)]
pub struct ChannelMapping {
    /// Input channel count
    pub input_channels: usize,
    
    /// Output channel count
    pub output_channels: usize,
    
    /// Mapping matrix (output_channel -> [(input_channel, gain)])
    pub matrix: Vec<Vec<(usize, f32)>>,
}

/// Bass management settings
#[derive(Debug, Clone)]
pub struct BassManagementSettings {
    /// Enable bass management
    pub enabled: bool,
    
    /// Crossover frequency in Hz
    pub crossover_frequency: f32,
    
    /// LFE channel gain
    pub lfe_gain: f32,
    
    /// Main channel high-pass filter frequency
    pub high_pass_frequency: f32,
}

impl Default for BassManagementSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            crossover_frequency: 80.0,
            lfe_gain: 0.0,
            high_pass_frequency: 80.0,
        }
    }
}

/// Channel layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    Mono,
    Stereo,
    Stereo21,
    Surround51,
    Surround71,
    Atmos,
}

impl ChannelLayout {
    pub fn channel_count(&self) -> usize {
        match self {
            ChannelLayout::Mono => 1,
            ChannelLayout::Stereo => 2,
            ChannelLayout::Stereo21 => 3,
            ChannelLayout::Surround51 => 6,
            ChannelLayout::Surround71 => 8,
            ChannelLayout::Atmos => 8, // Simplified
        }
    }
}

impl MultichannelProcessor {
    /// Create a new multi-channel processor
    pub fn new() -> Result<Self> {
        info!("Initializing multi-channel processor");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            output_config: Arc::new(RwLock::new(ChannelConfiguration::Stereo)),
            upmixing_enabled: Arc::new(RwLock::new(true)),
            downmixing_enabled: Arc::new(RwLock::new(true)),
            bass_management_enabled: Arc::new(RwLock::new(false)),
            crossover_frequency: Arc::new(RwLock::new(80.0)),
            channel_mapping: Arc::new(RwLock::new(ChannelMapping {
                input_channels: 2,
                output_channels: 2,
                matrix: vec![vec![(0, 1.0)], vec![(1, 1.0)]],
            })),
        })
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *self.is_initialized.read().await
            })
        })
    }
    
    /// Set output channel configuration
    pub async fn set_output_config(&self, config: ChannelConfiguration) -> Result<()> {
        *self.output_config.write().await = config;
        info!("Output channel configuration set to {:?}", config);
        Ok(())
    }
    
    /// Get output channel configuration
    pub async fn get_output_config(&self) -> ChannelConfiguration {
        *self.output_config.read().await
    }
    
    /// Enable/disable upmixing
    pub async fn set_upmixing(&self, enabled: bool) -> Result<()> {
        *self.upmixing_enabled.write().await = enabled;
        info!("Upmixing: {}", enabled);
        Ok(())
    }
    
    /// Enable/disable downmixing
    pub async fn set_downmixing(&self, enabled: bool) -> Result<()> {
        *self.downmixing_enabled.write().await = enabled;
        info!("Downmixing: {}", enabled);
        Ok(())
    }
    
    /// Enable/disable bass management
    pub async fn set_bass_management(&self, enabled: bool, crossover_freq: f32) -> Result<()> {
        *self.bass_management_enabled.write().await = enabled;
        *self.crossover_frequency.write().await = crossover_freq;
        info!("Bass management: enabled={}, crossover={}Hz", enabled, crossover_freq);
        Ok(())
    }
    
    /// Detect input channel layout
    pub fn detect_layout(&self, samples: &[f32]) -> ChannelLayout {
        let channel_count = samples.len();
        
        match channel_count {
            1 => ChannelLayout::Mono,
            2 => ChannelLayout::Stereo,
            3 => ChannelLayout::Stereo21,
            6 => ChannelLayout::Surround51,
            8 => ChannelLayout::Surround71,
            _ => ChannelLayout::Stereo, // Default to stereo
        }
    }
    
    /// Process audio with multi-channel processing
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        let input_layout = self.detect_layout(samples);
        let output_config = *self.output_config.read().await;
        let output_layout = self.config_to_layout(output_config);
        
        // If input and output are the same, just return
        if input_layout == output_layout {
            return Ok(samples.to_vec());
        }
        
        // Determine if we need upmixing or downmixing
        let input_channels = input_layout.channel_count();
        let output_channels = output_layout.channel_count();
        
        let mut processed = if output_channels > input_channels {
            // Upmixing
            if *self.upmixing_enabled.read().await {
                self.upmix(samples, input_layout, output_layout).await?
            } else {
                // Simple channel duplication
                self.simple_upmix(samples, input_channels, output_channels).await?
            }
        } else if output_channels < input_channels {
            // Downmixing
            if *self.downmixing_enabled.read().await {
                self.downmix(samples, input_layout, output_layout).await?
            } else {
                // Simple channel truncation
                self.simple_downmix(samples, output_channels).await?
            }
        } else {
            samples.to_vec()
        };
        
        // Apply bass management if enabled
        if *self.bass_management_enabled.read().await {
            processed = self.apply_bass_management(&processed, sample_rate).await?;
        }
        
        Ok(processed)
    }
    
    /// Convert configuration to layout
    fn config_to_layout(&self, config: ChannelConfiguration) -> ChannelLayout {
        match config {
            ChannelConfiguration::Mono => ChannelLayout::Mono,
            ChannelConfiguration::Stereo => ChannelLayout::Stereo,
            ChannelConfiguration::Stereo21 => ChannelLayout::Stereo21,
            ChannelConfiguration::Surround51 => ChannelLayout::Surround51,
            ChannelConfiguration::Surround71 => ChannelLayout::Surround71,
            ChannelConfiguration::Atmos => ChannelLayout::Atmos,
        }
    }
    
    /// Simple upmixing (channel duplication)
    async fn simple_upmix(&self, samples: &[f32], input_channels: usize, output_channels: usize) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * output_channels / input_channels];
        
        for (i, &sample) in samples.iter().enumerate() {
            let input_channel = i % input_channels;
            let output_idx = (i / input_channels) * output_channels + input_channel.min(output_channels - 1);
            output[output_idx] = sample;
        }
        
        Ok(output)
    }
    
    /// Simple downmixing (channel truncation)
    async fn simple_downmix(&self, samples: &[f32], output_channels: usize) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * output_channels / samples.len()];
        
        for i in 0..output_channels {
            if i < samples.len() {
                output[i] = samples[i];
            }
        }
        
        Ok(output)
    }
    
    /// Intelligent upmixing
    async fn upmix(&self, samples: &[f32], input_layout: ChannelLayout, output_layout: ChannelLayout) -> Result<Vec<f32>> {
        match (input_layout, output_layout) {
            (ChannelLayout::Stereo, ChannelLayout::Surround51) => {
                self.upmix_stereo_to_51(samples).await
            }
            (ChannelLayout::Stereo, ChannelLayout::Surround71) => {
                self.upmix_stereo_to_71(samples).await
            }
            (ChannelLayout::Surround51, ChannelLayout::Surround71) => {
                self.upmix_51_to_71(samples).await
            }
            _ => {
                // Fallback to simple upmixing
                self.simple_upmix(samples, input_layout.channel_count(), output_layout.channel_count()).await
            }
        }
    }
    
    /// Upmix stereo to 5.1
    async fn upmix_stereo_to_51(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * 3]; // 2 -> 6 channels
        
        for i in (0..samples.len()).step_by(2) {
            let left = samples[i];
            let right = samples[i + 1];
            
            let center = (left + right) * 0.707;
            let lfe = (left + right) * 0.5;
            let ls = left * 0.707;
            let rs = right * 0.707;
            
            let out_idx = i * 3;
            output[out_idx] = left;       // L
            output[out_idx + 1] = right;  // R
            output[out_idx + 2] = center; // C
            output[out_idx + 3] = lfe;    // LFE
            output[out_idx + 4] = ls;     // LS
            output[out_idx + 5] = rs;     // RS
        }
        
        Ok(output)
    }
    
    /// Upmix stereo to 7.1
    async fn upmix_stereo_to_71(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * 4]; // 2 -> 8 channels
        
        for i in (0..samples.len()).step_by(2) {
            let left = samples[i];
            let right = samples[i + 1];
            
            let center = (left + right) * 0.707;
            let lfe = (left + right) * 0.5;
            let ls = left * 0.707;
            let rs = right * 0.707;
            let lsr = left * 0.5;
            let rsr = right * 0.5;
            
            let out_idx = i * 4;
            output[out_idx] = left;       // L
            output[out_idx + 1] = right;  // R
            output[out_idx + 2] = center; // C
            output[out_idx + 3] = lfe;    // LFE
            output[out_idx + 4] = ls;     // LS
            output[out_idx + 5] = rs;     // RS
            output[out_idx + 6] = lsr;    // LSR
            output[out_idx + 7] = rsr;    // RSR
        }
        
        Ok(output)
    }
    
    /// Upmix 5.1 to 7.1
    async fn upmix_51_to_71(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * 4 / 3]; // 6 -> 8 channels
        
        for i in (0..samples.len()).step_by(6) {
            let left = samples[i];
            let right = samples[i + 1];
            let center = samples[i + 2];
            let lfe = samples[i + 3];
            let ls = samples[i + 4];
            let rs = samples[i + 5];
            
            let lsr = ls * 0.707;
            let rsr = rs * 0.707;
            
            let out_idx = i * 4 / 3;
            output[out_idx] = left;       // L
            output[out_idx + 1] = right;  // R
            output[out_idx + 2] = center; // C
            output[out_idx + 3] = lfe;    // LFE
            output[out_idx + 4] = ls;     // LS
            output[out_idx + 5] = rs;     // RS
            output[out_idx + 6] = lsr;    // LSR
            output[out_idx + 7] = rsr;    // RSR
        }
        
        Ok(output)
    }
    
    /// Intelligent downmixing
    async fn downmix(&self, samples: &[f32], input_layout: ChannelLayout, output_layout: ChannelLayout) -> Result<Vec<f32>> {
        match (input_layout, output_layout) {
            (ChannelLayout::Surround51, ChannelLayout::Stereo) => {
                self.downmix_51_to_stereo(samples).await
            }
            (ChannelLayout::Surround71, ChannelLayout::Stereo) => {
                self.downmix_71_to_stereo(samples).await
            }
            (ChannelLayout::Surround71, ChannelLayout::Surround51) => {
                self.downmix_71_to_51(samples).await
            }
            _ => {
                // Fallback to simple downmixing
                self.simple_downmix(samples, output_layout.channel_count()).await
            }
        }
    }
    
    /// Downmix 5.1 to stereo
    async fn downmix_51_to_stereo(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() / 3]; // 6 -> 2 channels
        
        for i in (0..samples.len()).step_by(6) {
            let left = samples[i];
            let right = samples[i + 1];
            let center = samples[i + 2];
            let lfe = samples[i + 3];
            let ls = samples[i + 4];
            let rs = samples[i + 5];
            
            // Lt/Rt downmix with center and surrounds
            let lt = left + center * 0.707 + ls * 0.707 + lfe * 0.5;
            let rt = right + center * 0.707 + rs * 0.707 + lfe * 0.5;
            
            let out_idx = i / 3;
            output[out_idx] = lt;
            output[out_idx + 1] = rt;
        }
        
        Ok(output)
    }
    
    /// Downmix 7.1 to stereo
    async fn downmix_71_to_stereo(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() / 4]; // 8 -> 2 channels
        
        for i in (0..samples.len()).step_by(8) {
            let left = samples[i];
            let right = samples[i + 1];
            let center = samples[i + 2];
            let lfe = samples[i + 3];
            let ls = samples[i + 4];
            let rs = samples[i + 5];
            let lsr = samples[i + 6];
            let rsr = samples[i + 7];
            
            // Lt/Rt downmix with all channels
            let lt = left + center * 0.707 + ls * 0.707 + lsr * 0.5 + lfe * 0.5;
            let rt = right + center * 0.707 + rs * 0.707 + rsr * 0.5 + lfe * 0.5;
            
            let out_idx = i / 4;
            output[out_idx] = lt;
            output[out_idx + 1] = rt;
        }
        
        Ok(output)
    }
    
    /// Downmix 7.1 to 5.1
    async fn downmix_71_to_51(&self, samples: &[f32]) -> Result<Vec<f32>> {
        let mut output = vec![0.0; samples.len() * 3 / 4]; // 8 -> 6 channels
        
        for i in (0..samples.len()).step_by(8) {
            let left = samples[i];
            let right = samples[i + 1];
            let center = samples[i + 2];
            let lfe = samples[i + 3];
            let ls = samples[i + 4];
            let rs = samples[i + 5];
            let lsr = samples[i + 6];
            let rsr = samples[i + 7];
            
            // Combine side surrounds with rear surrounds
            let ls_combined = ls + lsr * 0.707;
            let rs_combined = rs + rsr * 0.707;
            
            let out_idx = i * 3 / 4;
            output[out_idx] = left;           // L
            output[out_idx + 1] = right;      // R
            output[out_idx + 2] = center;     // C
            output[out_idx + 3] = lfe;        // LFE
            output[out_idx + 4] = ls_combined; // LS
            output[out_idx + 5] = rs_combined; // RS
        }
        
        Ok(output)
    }
    
    /// Apply bass management
    async fn apply_bass_management(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        let crossover_freq = *self.crossover_frequency.read().await;
        
        // Simplified bass management - just redirect low frequencies to LFE
        let mut output = samples.to_vec();
        
        // Apply simple high-pass to main channels and low-pass to LFE
        // This is a simplified implementation
        let crossover_bin = (crossover_freq / sample_rate as f32 * samples.len() as f32) as usize;
        
        // In a real implementation, this would use proper filters
        // For now, we just demonstrate the concept
        
        Ok(output)
    }
    
    /// Get channel mapping
    pub async fn get_channel_mapping(&self) -> ChannelMapping {
        self.channel_mapping.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_multichannel_processor_creation() {
        let processor = MultichannelProcessor::new().unwrap();
        assert!(processor.is_initialized());
    }
    
    #[tokio::test]
    async fn test_layout_detection() {
        let processor = MultichannelProcessor::new().unwrap();
        
        assert_eq!(processor.detect_layout(&[0.5]), ChannelLayout::Mono);
        assert_eq!(processor.detect_layout(&[0.5, 0.5]), ChannelLayout::Stereo);
        assert_eq!(processor.detect_layout(&[0.5; 6]), ChannelLayout::Surround51);
        assert_eq!(processor.detect_layout(&[0.5; 8]), ChannelLayout::Surround71);
    }
    
    #[tokio::test]
    async fn test_stereo_to_51_upmix() {
        let processor = MultichannelProcessor::new().unwrap();
        processor.set_output_config(ChannelConfiguration::Surround51).await.unwrap();
        
        let samples = vec![0.5, -0.5, 0.3, -0.3];
        let processed = processor.process(&samples, 48000).await.unwrap();
        
        assert_eq!(processed.len(), 12); // 4 samples * 3 (2 -> 6 channels)
    }
    
    #[tokio::test]
    async fn test_51_to_stereo_downmix() {
        let processor = MultichannelProcessor::new().unwrap();
        processor.set_output_config(ChannelConfiguration::Stereo).await.unwrap();
        
        let samples = vec![0.5; 12]; // 2 frames of 5.1
        let processed = processor.process(&samples, 48000).await.unwrap();
        
        assert_eq!(processed.len(), 4); // 12 samples / 3 (6 -> 2 channels)
    }
    
    #[tokio::test]
    async fn test_bass_management() {
        let processor = MultichannelProcessor::new().unwrap();
        processor.set_bass_management(true, 80.0).await.unwrap();
        
        let samples = vec![0.5; 6];
        let processed = processor.process(&samples, 48000).await.unwrap();
        
        assert_eq!(processed.len(), samples.len());
    }
}