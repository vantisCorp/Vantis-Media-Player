//! Headphone Virtualization Engine
//! 
//! Provides binaural rendering and spatial audio for headphones
//! using HRTF (Head-Related Transfer Functions) and crossfeed.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use rustfft::{FftPlanner, num_complex::Complex};
use nalgebra::{DMatrix, DVector};

use crate::VirtualizationMode;

/// Headphone virtualizer
pub struct HeadphoneVirtualizer {
    is_initialized: Arc<RwLock<bool>>,
    
    /// HRTF dataset
    hrtf_dataset: Arc<RwLock<HRTFDataset>>,
    
    /// Current virtualization mode
    mode: Arc<RwLock<VirtualizationMode>>,
    
    /// Crossfeed settings
    crossfeed_enabled: Arc<RwLock<bool>>,
    crossfeed_strength: Arc<RwLock<f32>>,
    
    /// Convolution buffers
    left_buffer: Arc<RwLock<Vec<f32>>>,
    right_buffer: Arc<RwLock<Vec<f32>>>,
}

/// HRTF (Head-Related Transfer Function) dataset
#[derive(Debug, Clone)]
pub struct HRTFDataset {
    /// Dataset name
    pub name: String,
    
    /// HRTF filters for different azimuth/elevation angles
    pub filters: Vec<HRTFFilter>,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Filter length
    pub filter_length: usize,
}

/// Individual HRTF filter
#[derive(Debug, Clone)]
pub struct HRTFFilter {
    /// Azimuth angle in degrees (-180 to 180)
    pub azimuth: f32,
    
    /// Elevation angle in degrees (-90 to 90)
    pub elevation: f32,
    
    /// Left ear impulse response
    pub left_ir: Vec<f32>,
    
    /// Right ear impulse response
    pub right_ir: Vec<f32>,
    
    /// Distance in meters
    pub distance: f32,
}

/// Virtualization parameters
#[derive(Debug, Clone)]
pub struct VirtualizationParams {
    /// Virtualization mode
    pub mode: VirtualizationMode,
    
    /// Room size (0.0 - 1.0)
    pub room_size: f32,
    
    /// Reverb level (0.0 - 1.0)
    pub reverb_level: f32,
    
    /// Source position (azimuth, elevation, distance)
    pub source_position: (f32, f32, f32),
    
    /// Head tracking enabled
    pub head_tracking: bool,
    
    /// Head orientation (yaw, pitch, roll) in degrees
    pub head_orientation: (f32, f32, f32),
}

impl Default for VirtualizationParams {
    fn default() -> Self {
        Self {
            mode: VirtualizationMode::Stereo,
            room_size: 0.5,
            reverb_level: 0.3,
            source_position: (0.0, 0.0, 1.0),
            head_tracking: false,
            head_orientation: (0.0, 0.0, 0.0),
        }
    }
}

impl HeadphoneVirtualizer {
    /// Create a new headphone virtualizer
    pub fn new() -> Result<Self> {
        info!("Initializing headphone virtualizer");
        
        let hrtf_dataset = Self::load_default_hrtf()?;
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            hrtf_dataset: Arc::new(RwLock::new(hrtf_dataset)),
            mode: Arc::new(RwLock::new(VirtualizationMode::Stereo)),
            crossfeed_enabled: Arc::new(RwLock::new(false)),
            crossfeed_strength: Arc::new(RwLock::new(0.5)),
            left_buffer: Arc::new(RwLock::new(vec![0.0; 4096])),
            right_buffer: Arc::new(RwLock::new(vec![0.0; 4096])),
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
    
    /// Load default HRTF dataset
    fn load_default_hrtf() -> Result<HRTFDataset> {
        info!("Loading default HRTF dataset");
        
        // Generate simplified HRTF dataset
        let mut filters = Vec::new();
        
        // Generate HRTF filters for different azimuth angles
        for azimuth in (-180..=180).step_by(30) {
            for elevation in (-30..=30).step_by(30) {
                let filter = Self::generate_hrtf_filter(azimuth as f32, elevation as f32, 1.0);
                filters.push(filter);
            }
        }
        
        Ok(HRTFDataset {
            name: "default".to_string(),
            filters,
            sample_rate: 48000,
            filter_length: 256,
        })
    }
    
    /// Generate HRTF filter for given angles
    fn generate_hrtf_filter(azimuth: f32, elevation: f32, distance: f32) -> HRTFFilter {
        let filter_length = 256;
        let mut left_ir = vec![0.0; filter_length];
        let mut right_ir = vec![0.0; filter_length];
        
        // Calculate ITD (Interaural Time Difference)
        let itd_samples = ((azimuth.to_radians().sin() * 0.15) / 343.0 * 48000.0) as i32;
        
        // Calculate ILD (Interaural Level Difference)
        let ild_db = azimuth.to_radians().sin() * 6.0;
        let ild_linear = 10.0_f32.powf(ild_db / 20.0);
        
        // Generate impulse responses
        for i in 0..filter_length {
            let t = i as f32 / 48000.0;
            
            // Left ear response
            let left_delay = if itd_samples > 0 { itd_samples as usize } else { 0 };
            if i >= left_delay {
                left_ir[i] = (-(t - left_delay as f32 / 48000.0) * 1000.0).exp();
            }
            
            // Right ear response
            let right_delay = if itd_samples < 0 { (-itd_samples) as usize } else { 0 };
            if i >= right_delay {
                right_ir[i] = (-(t - right_delay as f32 / 48000.0) * 1000.0).exp() / ild_linear;
            }
        }
        
        HRTFFilter {
            azimuth,
            elevation,
            left_ir,
            right_ir,
            distance,
        }
    }
    
    /// Set virtualization mode
    pub async fn set_mode(&self, mode: VirtualizationMode) -> Result<()> {
        *self.mode.write().await = mode;
        info!("Virtualization mode set to {:?}", mode);
        Ok(())
    }
    
    /// Get current mode
    pub async fn get_mode(&self) -> VirtualizationMode {
        *self.mode.read().await
    }
    
    /// Enable/disable crossfeed
    pub async fn set_crossfeed(&self, enabled: bool, strength: f32) -> Result<()> {
        *self.crossfeed_enabled.write().await = enabled;
        *self.crossfeed_strength.write().await = strength.clamp(0.0, 1.0);
        info!("Crossfeed: enabled={}, strength={}", enabled, strength);
        Ok(())
    }
    
    /// Process audio with virtualization
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        let mode = *self.mode.read().await;
        let crossfeed_enabled = *self.crossfeed_enabled.read().await;
        let crossfeed_strength = *self.crossfeed_strength.read().await;
        
        match mode {
            VirtualizationMode::Stereo => {
                self.process_stereo(samples, crossfeed_enabled, crossfeed_strength).await
            }
            VirtualizationMode::Surround51 => {
                self.process_surround_51(samples, sample_rate).await
            }
            VirtualizationMode::Surround71 => {
                self.process_surround_71(samples, sample_rate).await
            }
            VirtualizationMode::Atmos => {
                self.process_atmos(samples, sample_rate).await
            }
            VirtualizationMode::Binaural => {
                self.process_binaural(samples, sample_rate).await
            }
            VirtualizationMode::Ambisonics => {
                self.process_ambisonics(samples, sample_rate).await
            }
        }
    }
    
    /// Process stereo audio with optional crossfeed
    async fn process_stereo(&self, samples: &[f32], crossfeed_enabled: bool, crossfeed_strength: f32) -> Result<Vec<f32>> {
        if samples.len() % 2 != 0 {
            return Err(anyhow::anyhow!("Stereo samples must have even length"));
        }
        
        let mut output = vec![0.0; samples.len()];
        
        for i in (0..samples.len()).step_by(2) {
            let left = samples[i];
            let right = samples[i + 1];
            
            if crossfeed_enabled {
                // Apply crossfeed
                output[i] = left * (1.0 - crossfeed_strength * 0.5) + right * crossfeed_strength * 0.3;
                output[i + 1] = right * (1.0 - crossfeed_strength * 0.5) + left * crossfeed_strength * 0.3;
            } else {
                output[i] = left;
                output[i + 1] = right;
            }
        }
        
        Ok(output)
    }
    
    /// Process 5.1 surround audio
    async fn process_surround_51(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // 5.1: L, R, C, LFE, LS, RS
        if samples.len() % 6 != 0 {
            return Err(anyhow::anyhow!("5.1 samples must have length divisible by 6"));
        }
        
        let hrtf = self.hrtf_dataset.read().await;
        let mut output = vec![0.0; samples.len() / 3]; // Convert to stereo
        
        // Speaker positions (azimuth, elevation)
        let speaker_positions = [
            (-30.0, 0.0),  // L
            (30.0, 0.0),   // R
            (0.0, 0.0),    // C
            (0.0, 0.0),    // LFE (same as C)
            (-110.0, 0.0), // LS
            (110.0, 0.0),  // RS
        ];
        
        for i in (0..samples.len()).step_by(6) {
            let channels = [
                samples[i],     // L
                samples[i + 1], // R
                samples[i + 2], // C
                samples[i + 3], // LFE
                samples[i + 4], // LS
                samples[i + 5], // RS
            ];
            
            let mut left_sum = 0.0;
            let mut right_sum = 0.0;
            
            // Apply HRTF for each channel
            for (ch_idx, &(azimuth, elevation)) in speaker_positions.iter().enumerate() {
                let hrtf_filter = self.find_hrtf_filter(&hrtf, azimuth, elevation);
                
                // Simplified convolution (just use first sample for demo)
                left_sum += channels[ch_idx] * hrtf_filter.left_ir[0];
                right_sum += channels[ch_idx] * hrtf_filter.right_ir[0];
            }
            
            let out_idx = i / 3;
            output[out_idx] = left_sum;
            output[out_idx + 1] = right_sum;
        }
        
        Ok(output)
    }
    
    /// Process 7.1 surround audio
    async fn process_surround_71(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // 7.1: L, R, C, LFE, LS, RS, LSR, RSR
        if samples.len() % 8 != 0 {
            return Err(anyhow::anyhow!("7.1 samples must have length divisible by 8"));
        }
        
        let hrtf = self.hrtf_dataset.read().await;
        let mut output = vec![0.0; samples.len() / 4]; // Convert to stereo
        
        // Speaker positions (azimuth, elevation)
        let speaker_positions = [
            (-30.0, 0.0),  // L
            (30.0, 0.0),   // R
            (0.0, 0.0),    // C
            (0.0, 0.0),    // LFE
            (-110.0, 0.0), // LS
            (110.0, 0.0),  // RS
            (-150.0, 0.0), // LSR
            (150.0, 0.0),  // RSR
        ];
        
        for i in (0..samples.len()).step_by(8) {
            let channels = [
                samples[i],     // L
                samples[i + 1], // R
                samples[i + 2], // C
                samples[i + 3], // LFE
                samples[i + 4], // LS
                samples[i + 5], // RS
                samples[i + 6], // LSR
                samples[i + 7], // RSR
            ];
            
            let mut left_sum = 0.0;
            let mut right_sum = 0.0;
            
            for (ch_idx, &(azimuth, elevation)) in speaker_positions.iter().enumerate() {
                let hrtf_filter = self.find_hrtf_filter(&hrtf, azimuth, elevation);
                left_sum += channels[ch_idx] * hrtf_filter.left_ir[0];
                right_sum += channels[ch_idx] * hrtf_filter.right_ir[0];
            }
            
            let out_idx = i / 4;
            output[out_idx] = left_sum;
            output[out_idx + 1] = right_sum;
        }
        
        Ok(output)
    }
    
    /// Process Atmos (object-based) audio
    async fn process_atmos(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // Simplified Atmos processing - treat as 7.1 for now
        self.process_surround_71(samples, sample_rate).await
    }
    
    /// Process binaural audio
    async fn process_binaural(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // Binaural is already stereo, just apply crossfeed
        let crossfeed_enabled = *self.crossfeed_enabled.read().await;
        let crossfeed_strength = *self.crossfeed_strength.read().await;
        self.process_stereo(samples, crossfeed_enabled, crossfeed_strength).await
    }
    
    /// Process Ambisonics audio
    async fn process_ambisonics(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // Ambisonics: ACN format (W, X, Y, Z, ...)
        // First-order Ambisonics: 4 channels
        if samples.len() % 4 != 0 {
            return Err(anyhow::anyhow!("Ambisonics samples must have length divisible by 4"));
        }
        
        let hrtf = self.hrtf_dataset.read().await;
        let mut output = vec![0.0; samples.len() / 2]; // Convert to stereo
        
        for i in (0..samples.len()).step_by(4) {
            let w = samples[i];     // Omnidirectional
            let x = samples[i + 1]; // Front-back
            let y = samples[i + 2]; // Left-right
            let z = samples[i + 3]; // Up-down
            
            // Decode to binaural using HRTF
            let left = w + x * 0.707 + y * 0.707;
            let right = w + x * 0.707 - y * 0.707;
            
            let out_idx = i / 2;
            output[out_idx] = left;
            output[out_idx + 1] = right;
        }
        
        Ok(output)
    }
    
    /// Find HRTF filter for given angles
    fn find_hrtf_filter(&self, hrtf: &HRTFDataset, azimuth: f32, elevation: f32) -> HRTFFilter {
        // Find closest filter
        let mut closest_filter = &hrtf.filters[0];
        let mut min_distance = f32::MAX;
        
        for filter in &hrtf.filters {
            let azimuth_diff = (filter.azimuth - azimuth).abs();
            let elevation_diff = (filter.elevation - elevation).abs();
            let distance = (azimuth_diff.powi(2) + elevation_diff.powi(2)).sqrt();
            
            if distance < min_distance {
                min_distance = distance;
                closest_filter = filter;
            }
        }
        
        closest_filter.clone()
    }
    
    /// Update head orientation (for head tracking)
    pub async fn update_head_orientation(&self, yaw: f32, pitch: f32, roll: f32) -> Result<()> {
        debug!("Head orientation updated: yaw={}, pitch={}, roll={}", yaw, pitch, roll);
        // In a real implementation, this would update the HRTF rendering
        Ok(())
    }
    
    /// Get HRTF dataset info
    pub async fn get_hrtf_info(&self) -> (String, u32, usize) {
        let hrtf = self.hrtf_dataset.read().await;
        (hrtf.name.clone(), hrtf.sample_rate, hrtf.filters.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_headphone_virtualizer_creation() {
        let virtualizer = HeadphoneVirtualizer::new().unwrap();
        assert!(virtualizer.is_initialized());
    }
    
    #[tokio::test]
    async fn test_stereo_processing() {
        let virtualizer = HeadphoneVirtualizer::new().unwrap();
        let samples = vec![0.5, -0.5, 0.3, -0.3];
        let processed = virtualizer.process(&samples, 48000).await.unwrap();
        assert_eq!(processed.len(), samples.len());
    }
    
    #[tokio::test]
    async fn test_crossfeed() {
        let virtualizer = HeadphoneVirtualizer::new().unwrap();
        virtualizer.set_crossfeed(true, 0.5).await.unwrap();
        
        let samples = vec![1.0, 0.0, 1.0, 0.0];
        let processed = virtualizer.process(&samples, 48000).await.unwrap();
        
        // With crossfeed, right channel should have some left signal
        assert!(processed[1] > 0.0);
    }
    
    #[tokio::test]
    async fn test_surround_51_processing() {
        let virtualizer = HeadphoneVirtualizer::new().unwrap();
        virtualizer.set_mode(VirtualizationMode::Surround51).await.unwrap();
        
        let samples = vec![0.5; 12]; // 2 frames of 5.1
        let processed = virtualizer.process(&samples, 48000).await.unwrap();
        
        // Should convert to stereo
        assert_eq!(processed.len(), 4);
    }
    
    #[tokio::test]
    async fn test_mode_switching() {
        let virtualizer = HeadphoneVirtualizer::new().unwrap();
        
        virtualizer.set_mode(VirtualizationMode::Binaural).await.unwrap();
        assert_eq!(virtualizer.get_mode().await, VirtualizationMode::Binaural);
        
        virtualizer.set_mode(VirtualizationMode::Surround71).await.unwrap();
        assert_eq!(virtualizer.get_mode().await, VirtualizationMode::Surround71);
    }
}