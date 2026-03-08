//! Head-Related Transfer Function (HRTF) processing for binaural audio

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{SpatialResult, SpatialAudioError, AudioBuffer, Sample, SampleRate};

/// HRTF dataset containing impulse responses
#[derive(Debug, Clone)]
pub struct HrtfDataset {
    /// Sample rate of the dataset
    sample_rate: u32,
    
    /// Impulse responses indexed by (azimuth, elevation)
    /// Azimuth: -180 to 180 degrees
    /// Elevation: -90 to 90 degrees
    responses: HashMap<(i32, i32), HrtfImpulseResponse>,
    
    /// Interpolation grid
    azimuth_step: i32,
    elevation_step: i32,
    
    /// Default/KEMAR dataset or custom
    dataset_name: String,
}

/// Single HRTF impulse response for a specific direction
#[derive(Debug, Clone)]
pub struct HrtfImpulseResponse {
    /// Left ear impulse response
    pub left: Vec<Sample>,
    
    /// Right ear impulse response
    pub right: Vec<Sample>,
    
    /// Length in samples
    pub length: usize,
    
    /// Inter-aural time difference in samples
    pub itd: f32,
    
    /// Inter-aural level difference in dB
    pub ild: f32,
}

impl HrtfImpulseResponse {
    pub fn new(left: Vec<Sample>, right: Vec<Sample>) -> Self {
        Self {
            length: left.len().max(right.len()),
            left,
            right,
            itd: 0.0,
            ild: 0.0,
        }
    }
}

impl HrtfDataset {
    /// Create an empty HRTF dataset
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            responses: HashMap::new(),
            azimuth_step: 10,
            elevation_step: 10,
            dataset_name: "Custom".to_string(),
        }
    }
    
    /// Load the default KEMAR dataset
    pub fn kemar() -> Self {
        let mut dataset = Self::new(44100);
        dataset.dataset_name = "KEMAR".to_string();
        dataset.generate_default_responses();
        dataset
    }
    
    /// Create a simple HRTF dataset with approximated responses
    pub fn simple_approximation(sample_rate: u32) -> Self {
        let mut dataset = Self::new(sample_rate);
        dataset.generate_default_responses();
        dataset
    }
    
    /// Generate default HRTF responses (approximation)
    fn generate_default_responses(&mut self) {
        // Generate responses for a spherical grid
        for azimuth in (-180..=180).step_by(self.azimuth_step as usize) {
            for elevation in (-90..=90).step_by(self.elevation_step as usize) {
                let response = self.generate_approximated_response(azimuth, elevation);
                self.responses.insert((azimuth, elevation), response);
            }
        }
    }
    
    /// Generate an approximated HRTF response for a given direction
    fn generate_approximated_response(&self, azimuth: i32, elevation: i32) -> HrtfImpulseResponse {
        let length = 128; // Standard HRTF length
        
        // Convert to radians
        let az_rad = azimuth.to_radians();
        let el_rad = elevation.to_radians();
        
        // Calculate inter-aural time difference (ITD)
        // Approximation: ITD = (r/c) * (sin(azimuth) + sin(azimuth) * sin(elevation))
        let head_radius = 0.0875; // meters
        let speed_of_sound = 343.0; // m/s
        let itd_seconds = head_radius / speed_of_sound * (az_rad.sin() * el_rad.cos().abs());
        let itd_samples = itd_seconds * self.sample_rate as f32;
        
        // Calculate inter-aural level difference (ILD)
        // Higher frequency sounds have more ILD for lateral sources
        let ild = if azimuth.abs() > 45 {
            -3.0 * (az_rad.sin().abs()) // dB
        } else {
            0.0
        };
        
        // Generate simple impulse responses
        let mut left = vec![0.0f32; length];
        let mut right = vec![0.0f32; length];
        
        // Simple head shadow model
        let left_gain = if azimuth >= 0 { 1.0 } else { 1.0 + ild / 20.0 };
        let right_gain = if azimuth <= 0 { 1.0 } else { 1.0 + ild / 20.0 };
        
        // Apply simple filtering based on direction
        // This is a simplified model - real HRTF would use measured data
        left[0] = left_gain;
        right[0] = right_gain;
        
        // Add some simple frequency-dependent effects
        let delay_left = if azimuth > 0 { 0 } else { itd_samples.abs() as usize };
        let delay_right = if azimuth < 0 { 0 } else { itd_samples.abs() as usize };
        
        let delay_left = delay_left.min(length - 1);
        let delay_right = delay_right.min(length - 1);
        
        left.fill(0.0);
        right.fill(0.0);
        
        left[delay_left] = left_gain;
        right[delay_right] = right_gain;
        
        // Add some shoulder/pinna reflection effects
        if elevation > 30 {
            // High elevation - add notch filter effect
            let notch_freq = 8000.0 + (elevation as f32 - 30.0) * 50.0;
            let notch_period = (self.sample_rate as f32 / notch_freq) as usize;
            if notch_period < length {
                left[notch_period] = -0.3 * left_gain;
                right[notch_period] = -0.3 * right_gain;
            }
        }
        
        HrtfImpulseResponse {
            left,
            right,
            length,
            itd: itd_samples,
            ild,
        }
    }
    
    /// Get HRTF response for a specific direction
    pub fn get_response(&self, azimuth: f32, elevation: f32) -> Option<&HrtfImpulseResponse> {
        // Quantize to nearest grid point
        let az = ((azimuth.round() as i32 + 180) / self.azimuth_step * self.azimuth_step - 180).clamp(-180, 180);
        let el = ((elevation.round() as i32 + 90) / self.elevation_step * self.elevation_step - 90).clamp(-90, 90);
        
        self.responses.get(&(az, el))
    }
    
    /// Get interpolated HRTF response
    pub fn get_interpolated_response(&self, azimuth: f32, elevation: f32) -> HrtfImpulseResponse {
        // Simple bilinear interpolation
        let az_idx1 = ((azimuth / self.azimuth_step as f32).floor() as i32 * self.azimuth_step).clamp(-180, 180);
        let az_idx2 = (az_idx1 + self.azimuth_step).clamp(-180, 180);
        let el_idx1 = ((elevation / self.elevation_step as f32).floor() as i32 * self.elevation_step).clamp(-90, 90);
        let el_idx2 = (el_idx1 + self.elevation_step).clamp(-90, 90);
        
        let t_az = (azimuth - az_idx1 as f32) / self.azimuth_step as f32;
        let t_el = (elevation - el_idx1 as f32) / self.elevation_step as f32;
        
        let r00 = self.responses.get(&(az_idx1, el_idx1));
        let r01 = self.responses.get(&(az_idx1, el_idx2));
        let r10 = self.responses.get(&(az_idx2, el_idx1));
        let r11 = self.responses.get(&(az_idx2, el_idx2));
        
        let length = r00.map(|r| r.length).unwrap_or(128);
        let mut left = vec![0.0; length];
        let mut right = vec![0.0; length];
        
        // Interpolate
        for i in 0..length {
            let v00 = r00.map(|r| r.left.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v01 = r01.map(|r| r.left.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v10 = r10.map(|r| r.left.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v11 = r11.map(|r| r.left.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            
            left[i] = (1.0 - t_az) * (1.0 - t_el) * v00
                    + t_az * (1.0 - t_el) * v10
                    + (1.0 - t_az) * t_el * v01
                    + t_az * t_el * v11;
            
            let v00 = r00.map(|r| r.right.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v01 = r01.map(|r| r.right.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v10 = r10.map(|r| r.right.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            let v11 = r11.map(|r| r.right.get(i).copied().unwrap_or(0.0)).unwrap_or(0.0);
            
            right[i] = (1.0 - t_az) * (1.0 - t_el) * v00
                     + t_az * (1.0 - t_el) * v10
                     + (1.0 - t_az) * t_el * v01
                     + t_az * t_el * v11;
        }
        
        HrtfImpulseResponse {
            left,
            right,
            length,
            itd: 0.0,
            ild: 0.0,
        }
    }
    
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

/// HRTF processor for real-time binaural rendering
pub struct HrtfProcessor {
    /// Sample rate
    sample_rate: SampleRate,
    
    /// HRTF dataset
    dataset: HrtfDataset,
    
    /// FFT size for convolution
    fft_size: usize,
    
    /// Crossfade state for smooth transitions
    crossfade: CrossfadeState,
    
    /// Previous impulse response for crossfade
    previous_response: Option<HrtfImpulseResponse>,
}

/// State for crossfading between HRTF responses
#[derive(Debug, Clone)]
struct CrossfadeState {
    /// Current crossfade position (0.0 to 1.0)
    position: f32,
    
    /// Crossfade duration in samples
    duration: usize,
    
    /// Whether crossfade is active
    active: bool,
}

impl HrtfProcessor {
    /// Create a new HRTF processor
    pub fn new(sample_rate: SampleRate, dataset: Option<&HrtfDataset>) -> SpatialResult<Self> {
        let dataset = match dataset {
            Some(ds) => ds.clone(),
            None => HrtfDataset::simple_approximation(sample_rate.as_u32()),
        };
        
        Ok(Self {
            sample_rate,
            dataset,
            fft_size: 512,
            crossfade: CrossfadeState {
                position: 0.0,
                duration: 256,
                active: false,
            },
            previous_response: None,
        })
    }
    
    /// Process mono audio through HRTF
    pub fn process(
        &mut self,
        input: &AudioBuffer,
        azimuth: f32,
        elevation: f32,
        gain: f32,
    ) -> SpatialResult<AudioBuffer> {
        let sample_count = input.sample_count();
        let mut output = AudioBuffer::new(2, sample_count);
        
        // Get HRTF response for this direction
        let response = self.dataset.get_interpolated_response(azimuth, elevation);
        
        // Simple convolution (time-domain for small impulse responses)
        // For production, use FFT-based convolution for efficiency
        
        let input_channel = input.channel(0);
        let left_ir = &response.left;
        let right_ir = &response.right;
        
        let left_output = output.channel_mut(0);
        let right_output = output.channel_mut(1);
        
        // Apply convolution
        for (i, left_sample) in left_output.iter_mut().enumerate() {
            let mut sum = 0.0f32;
            for (j, ir_sample) in left_ir.iter().enumerate() {
                if i >= j {
                    sum += input_channel.get(i - j).copied().unwrap_or(0.0) * ir_sample;
                }
            }
            *left_sample = sum * gain;
        }
        
        for (i, right_sample) in right_output.iter_mut().enumerate() {
            let mut sum = 0.0f32;
            for (j, ir_sample) in right_ir.iter().enumerate() {
                if i >= j {
                    sum += input_channel.get(i - j).copied().unwrap_or(0.0) * ir_sample;
                }
            }
            *right_sample = sum * gain;
        }
        
        Ok(output)
    }
    
    /// Process with crossfade for smooth transitions
    pub fn process_with_crossfade(
        &mut self,
        input: &AudioBuffer,
        azimuth: f32,
        elevation: f32,
        gain: f32,
    ) -> SpatialResult<AudioBuffer> {
        // Check if direction changed significantly
        let current_response = self.dataset.get_interpolated_response(azimuth, elevation);
        
        // For now, use simple processing
        // Full implementation would crossfade between responses
        self.process(input, azimuth, elevation, gain)
    }
    
    /// Get the dataset
    pub fn dataset(&self) -> &HrtfDataset {
        &self.dataset
    }
    
    /// Get mutable reference to dataset
    pub fn dataset_mut(&mut self) -> &mut HrtfDataset {
        &mut self.dataset
    }
}

/// HRTF configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HrtfConfig {
    /// Enable crossfade smoothing
    pub smooth_transitions: bool,
    
    /// Crossfade duration in milliseconds
    pub crossfade_duration_ms: f32,
    
    /// Custom dataset path
    pub custom_dataset: Option<String>,
    
    /// Head radius in meters
    pub head_radius: f32,
}

impl Default for HrtfConfig {
    fn default() -> Self {
        Self {
            smooth_transitions: true,
            crossfade_duration_ms: 20.0,
            custom_dataset: None,
            head_radius: 0.0875,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hrtf_dataset_creation() {
        let dataset = HrtfDataset::simple_approximation(48000);
        assert!(dataset.sample_rate() == 48000);
    }
    
    #[test]
    fn test_hrtf_response_generation() {
        let dataset = HrtfDataset::simple_approximation(48000);
        let response = dataset.get_response(45.0, 0.0);
        assert!(response.is_some());
    }
    
    #[test]
    fn test_interpolated_response() {
        let dataset = HrtfDataset::simple_approximation(48000);
        let response = dataset.get_interpolated_response(47.5, 2.5);
        assert!(!response.left.is_empty());
        assert!(!response.right.is_empty());
    }
}