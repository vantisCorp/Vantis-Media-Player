//! Room Correction Engine
//! 
//! Provides acoustic room correction using impulse response measurements
//! and digital signal processing to compensate for room acoustics.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use rustfft::{FftPlanner, num_complex::Complex};
use nalgebra::{DMatrix, DVector};
use rubato::{Resampler, SincFixedIn, InterpolationType};

/// Room correction engine
pub struct RoomCorrectionEngine {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Impulse response measurements
    impulse_responses: Arc<RwLock<Vec<ImpulseResponse>>>,
    
    /// Correction filters for each channel
    correction_filters: Arc<RwLock<Vec<CorrectionFilter>>>,
    
    /// Target frequency response curve
    target_curve: Arc<RwLock<Vec<f32>>>,
    
    /// Room acoustic parameters
    room_parameters: Arc<RwLock<RoomParameters>>,
}

/// Impulse response measurement
#[derive(Debug, Clone)]
pub struct ImpulseResponse {
    /// Channel index
    pub channel: usize,
    
    /// Impulse response samples
    pub samples: Vec<f32>,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Measurement timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Correction filter
#[derive(Debug, Clone)]
pub struct CorrectionFilter {
    /// Channel index
    pub channel: usize,
    
    /// Filter coefficients (FIR)
    pub coefficients: Vec<f32>,
    
    /// Filter length
    pub length: usize,
    
    /// Frequency response
    pub frequency_response: Vec<f32>,
}

/// Room acoustic parameters
#[derive(Debug, Clone)]
pub struct RoomParameters {
    /// Room dimensions (width, depth, height) in meters
    pub dimensions: (f32, f32, f32),
    
    /// Reverberation time (RT60) in seconds
    pub rt60: f32,
    
    /// Early decay time in seconds
    pub edt: f32,
    
    /// Bass ratio
    pub bass_ratio: f32,
    
    /// Brilliance ratio
    pub brilliance_ratio: f32,
    
    /// C50 (clarity for speech) in dB
    pub c50: f32,
    
    /// C80 (clarity for music) in dB
    pub c80: f32,
    
    /// STI (Speech Transmission Index)
    pub sti: f32,
}

impl Default for RoomParameters {
    fn default() -> Self {
        Self {
            dimensions: (5.0, 6.0, 3.0),
            rt60: 0.6,
            edt: 0.3,
            bass_ratio: 1.0,
            brilliance_ratio: 1.0,
            c50: 0.0,
            c80: 0.0,
            sti: 0.0,
        }
    }
}

/// Calibration measurement
#[derive(Debug, Clone)]
pub struct CalibrationMeasurement {
    /// Measurement position
    pub position: (f32, f32, f32),
    
    /// Frequency response
    pub frequency_response: Vec<f32>,
    
    /// Phase response
    pub phase_response: Vec<f32>,
    
    /// Impulse response
    pub impulse_response: Vec<f32>,
    
    /// Distortion measurements
    pub distortion: DistortionMeasurements,
}

/// Distortion measurements
#[derive(Debug, Clone)]
pub struct DistortionMeasurements {
    /// THD (Total Harmonic Distortion) in dB
    pub thd: f32,
    
    /// IMD (Intermodulation Distortion) in dB
    pub imd: f32,
    
    /// Noise floor in dB
    pub noise_floor: f32,
    
    /// Dynamic range in dB
    pub dynamic_range: f32,
}

impl RoomCorrectionEngine {
    /// Create a new room correction engine
    pub fn new() -> Result<Self> {
        info!("Initializing room correction engine");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            impulse_responses: Arc::new(RwLock::new(Vec::new())),
            correction_filters: Arc::new(RwLock::new(Vec::new())),
            target_curve: Arc::new(RwLock::new(vec![0.0; 64])),
            room_parameters: Arc::new(RwLock::new(RoomParameters::default())),
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
    
    /// Start room calibration
    pub async fn start_calibration(&self, measurement_duration: u32) -> Result<CalibrationMeasurement> {
        info!("Starting room calibration (duration: {}s)", measurement_duration);
        
        // Simulate calibration process
        let mut frequency_response = vec![0.0; 64];
        let mut phase_response = vec![0.0; 64];
        let mut impulse_response = vec![0.0; 4096];
        
        // Generate simulated measurements
        for i in 0..64 {
            let freq = 20.0 * (100.0_f32).powf(i as f32 / 63.0);
            frequency_response[i] = -3.0 * (freq / 1000.0).ln_1p(); // Simulated room response
            phase_response[i] = -freq * 0.001; // Simulated phase delay
        }
        
        // Generate impulse response with reverb
        impulse_response[0] = 1.0;
        for i in 1..impulse_response.len() {
            let t = i as f32 / 48000.0;
            impulse_response[i] = 0.1 * (-t / 0.6).exp() * (2.0 * std::f32::consts::PI * 1000.0 * t).sin();
        }
        
        let distortion = DistortionMeasurements {
            thd: -80.0,
            imd: -75.0,
            noise_floor: -90.0,
            dynamic_range: 95.0,
        };
        
        let measurement = CalibrationMeasurement {
            position: (0.0, 0.0, 0.0),
            frequency_response,
            phase_response,
            impulse_response,
            distortion,
        };
        
        // Calculate room parameters
        self.calculate_room_parameters(&measurement).await?;
        
        // Generate correction filters
        self.generate_correction_filters(&measurement).await?;
        
        info!("Room calibration completed");
        Ok(measurement)
    }
    
    /// Calculate room acoustic parameters
    async fn calculate_room_parameters(&self, measurement: &CalibrationMeasurement) -> Result<()> {
        debug!("Calculating room parameters");
        
        // Calculate RT60 from impulse response
        let rt60 = self.calculate_rt60(&measurement.impulse_response, 48000);
        
        // Calculate EDT (Early Decay Time)
        let edt = self.calculate_edt(&measurement.impulse_response, 48000);
        
        // Calculate bass ratio (125-250Hz / 500-1000Hz)
        let bass_ratio = self.calculate_bass_ratio(&measurement.frequency_response);
        
        // Calculate brilliance ratio (2000-4000Hz / 125-250Hz)
        let brilliance_ratio = self.calculate_brilliance_ratio(&measurement.frequency_response);
        
        let parameters = RoomParameters {
            dimensions: (5.0, 6.0, 3.0),
            rt60,
            edt,
            bass_ratio,
            brilliance_ratio,
            c50: 0.0,
            c80: 0.0,
            sti: 0.0,
        };
        
        *self.room_parameters.write().await = parameters;
        
        info!("Room parameters: RT60={:.2}s, EDT={:.2}s", rt60, edt);
        Ok(())
    }
    
    /// Calculate RT60 from impulse response
    fn calculate_rt60(&self, impulse: &[f32], sample_rate: u32) -> f32 {
        // Find the decay to -60dB
        let threshold = 0.001; // -60dB
        let mut decay_samples = 0;
        
        for i in 1..impulse.len() {
            if impulse[i].abs() < threshold {
                decay_samples = i;
                break;
            }
        }
        
        decay_samples as f32 / sample_rate as f32
    }
    
    /// Calculate EDT (Early Decay Time)
    fn calculate_edt(&self, impulse: &[f32], sample_rate: u32) -> f32 {
        // Calculate decay from 0dB to -10dB
        let start_threshold = 1.0;
        let end_threshold = 0.316; // -10dB
        
        let mut start_sample = 0;
        let mut end_sample = 0;
        
        for i in 0..impulse.len() {
            if impulse[i].abs() < start_threshold && start_sample == 0 {
                start_sample = i;
            }
            if impulse[i].abs() < end_threshold {
                end_sample = i;
                break;
            }
        }
        
        (end_sample - start_sample) as f32 * 6.0 / sample_rate as f32
    }
    
    /// Calculate bass ratio
    fn calculate_bass_ratio(&self, frequency_response: &[f32]) -> f32 {
        // Average response in 125-250Hz vs 500-1000Hz
        let bass_start = 0;
        let bass_end = 10;
        let mid_start = 20;
        let mid_end = 30;
        
        let bass_avg: f32 = frequency_response[bass_start..bass_end].iter().sum::<f32>() / (bass_end - bass_start) as f32;
        let mid_avg: f32 = frequency_response[mid_start..mid_end].iter().sum::<f32>() / (mid_end - mid_start) as f32;
        
        10.0_f32.powf((bass_avg - mid_avg) / 20.0)
    }
    
    /// Calculate brilliance ratio
    fn calculate_brilliance_ratio(&self, frequency_response: &[f32]) -> f32 {
        // Average response in 2000-4000Hz vs 125-250Hz
        let bass_start = 0;
        let bass_end = 10;
        let high_start = 45;
        let high_end = 55;
        
        let bass_avg: f32 = frequency_response[bass_start..bass_end].iter().sum::<f32>() / (bass_end - bass_start) as f32;
        let high_avg: f32 = frequency_response[high_start..high_end].iter().sum::<f32>() / (high_end - high_start) as f32;
        
        10.0_f32.powf((high_avg - bass_avg) / 20.0)
    }
    
    /// Generate correction filters
    async fn generate_correction_filters(&self, measurement: &CalibrationMeasurement) -> Result<()> {
        debug!("Generating correction filters");
        
        let target_curve = self.target_curve.read().await;
        let mut filters = self.correction_filters.write().await;
        
        // Create correction filter by inverting the measured response
        let mut coefficients = vec![0.0; 4096];
        
        for i in 0..measurement.frequency_response.len() {
            let correction = target_curve[i] - measurement.frequency_response[i];
            coefficients[i] = 10.0_f32.powf(correction / 20.0);
        }
        
        // Apply smoothing
        self.smooth_filter(&mut coefficients, 5);
        
        let filter = CorrectionFilter {
            channel: 0,
            coefficients,
            length: coefficients.len(),
            frequency_response: measurement.frequency_response.clone(),
        };
        
        filters.push(filter);
        
        info!("Correction filter generated");
        Ok(())
    }
    
    /// Smooth filter coefficients
    fn smooth_filter(&self, coefficients: &mut [f32], window_size: usize) {
        let mut smoothed = coefficients.to_vec();
        
        for i in window_size..coefficients.len() - window_size {
            let start = i - window_size;
            let end = i + window_size + 1;
            smoothed[i] = coefficients[start..end].iter().sum::<f32>() / (2 * window_size + 1) as f32;
        }
        
        coefficients.copy_from_slice(&smoothed);
    }
    
    /// Process audio with room correction
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        let filters = self.correction_filters.read().await;
        
        if filters.is_empty() {
            return Ok(samples.to_vec());
        }
        
        let filter = &filters[0];
        let mut output = vec![0.0; samples.len()];
        
        // Apply FIR filter using convolution
        for i in 0..samples.len() {
            for j in 0..filter.coefficients.len().min(i + 1) {
                output[i] += samples[i - j] * filter.coefficients[j];
            }
        }
        
        // Normalize output
        let max_val = output.iter().map(|&x| x.abs()).fold(0.0_f32, f32::max);
        if max_val > 0.0 {
            for sample in output.iter_mut() {
                *sample /= max_val;
            }
        }
        
        Ok(output)
    }
    
    /// Set target frequency response curve
    pub async fn set_target_curve(&self, curve: Vec<f32>) -> Result<()> {
        *self.target_curve.write().await = curve;
        info!("Target curve updated");
        Ok(())
    }
    
    /// Get room parameters
    pub async fn get_room_parameters(&self) -> RoomParameters {
        self.room_parameters.read().await.clone()
    }
    
    /// Get correction filters
    pub async fn get_correction_filters(&self) -> Vec<CorrectionFilter> {
        self.correction_filters.read().await.clone()
    }
    
    /// Export calibration data
    pub async fn export_calibration(&self) -> Result<String> {
        let parameters = self.get_room_parameters().await;
        let filters = self.get_correction_filters().await;
        
        let data = serde_json::to_string_pretty(&serde_json::json!({
            "room_parameters": parameters,
            "correction_filters": filters,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))?;
        
        Ok(data)
    }
    
    /// Import calibration data
    pub async fn import_calibration(&self, data: &str) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(data)?;
        
        if let Some(params) = json.get("room_parameters") {
            let parameters: RoomParameters = serde_json::from_value(params.clone())?;
            *self.room_parameters.write().await = parameters;
        }
        
        if let Some(filters) = json.get("correction_filters") {
            let correction_filters: Vec<CorrectionFilter> = serde_json::from_value(filters.clone())?;
            *self.correction_filters.write().await = correction_filters;
        }
        
        info!("Calibration data imported");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_room_correction_creation() {
        let engine = RoomCorrectionEngine::new().unwrap();
        assert!(engine.is_initialized());
    }
    
    #[tokio::test]
    async fn test_calibration() {
        let engine = RoomCorrectionEngine::new().unwrap();
        let measurement = engine.start_calibration(10).await.unwrap();
        
        assert!(!measurement.frequency_response.is_empty());
        assert!(!measurement.impulse_response.is_empty());
    }
    
    #[tokio::test]
    async fn test_room_parameters() {
        let engine = RoomCorrectionEngine::new().unwrap();
        engine.start_calibration(10).await.unwrap();
        
        let params = engine.get_room_parameters().await;
        assert!(params.rt60 > 0.0);
        assert!(params.edt > 0.0);
    }
    
    #[tokio::test]
    async fn test_audio_processing() {
        let engine = RoomCorrectionEngine::new().unwrap();
        engine.start_calibration(10).await.unwrap();
        
        let samples = vec![0.5; 1000];
        let processed = engine.process(&samples, 48000).await.unwrap();
        
        assert_eq!(processed.len(), samples.len());
    }
}