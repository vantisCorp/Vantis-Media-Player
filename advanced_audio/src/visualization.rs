//! Audio Visualization Engine
//! 
//! Provides real-time audio visualization including spectrum, waveform,
//! spectrogram, and frequency band displays.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use rustfft::{FftPlanner, num_complex::Complex};
use image::{Rgb, RgbImage};

use crate::{VisualizationType, ColorScheme};

/// Audio visualizer
pub struct AudioVisualizer {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Current visualization type
    visualization_type: Arc<RwLock<VisualizationType>>,
    
    /// Color scheme
    color_scheme: Arc<RwLock<ColorScheme>>,
    
    /// FFT size
    fft_size: Arc<RwLock<usize>>,
    
    /// Current audio buffer
    audio_buffer: Arc<RwLock<Vec<f32>>>,
    
    /// Sample rate
    sample_rate: Arc<RwLock<u32>>,
    
    /// Current spectrum data
    spectrum_data: Arc<RwLock<Vec<f32>>>,
    
    /// Current waveform data
    waveform_data: Arc<RwLock<Vec<f32>>>,
    
    /// Spectrogram data
    spectrogram_data: Arc<RwLock<Vec<Vec<f32>>>>,
}

/// Visualization frame
#[derive(Debug, Clone)]
pub struct VisualizationFrame {
    /// Frame data (RGB pixels)
    pub data: Vec<u8>,
    
    /// Frame width
    pub width: usize,
    
    /// Frame height
    pub height: usize,
    
    /// Frame timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Frequency band data
#[derive(Debug, Clone)]
pub struct FrequencyBands {
    /// Bass level (20-250 Hz)
    pub bass: f32,
    
    /// Low mid level (250-500 Hz)
    pub low_mid: f32,
    
    /// Mid level (500-2000 Hz)
    pub mid: f32,
    
    /// High mid level (2000-4000 Hz)
    pub high_mid: f32,
    
    /// Treble level (4000-20000 Hz)
    pub treble: f32,
}

impl AudioVisualizer {
    /// Create a new audio visualizer
    pub fn new() -> Result<Self> {
        info!("Initializing audio visualizer");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            visualization_type: Arc::new(RwLock::new(VisualizationType::Spectrum)),
            color_scheme: Arc::new(RwLock::new(ColorScheme::default())),
            fft_size: Arc::new(RwLock::new(2048)),
            audio_buffer: Arc::new(RwLock::new(Vec::new())),
            sample_rate: Arc::new(RwLock::new(48000)),
            spectrum_data: Arc::new(RwLock::new(Vec::new())),
            waveform_data: Arc::new(RwLock::new(Vec::new())),
            spectrogram_data: Arc::new(RwLock::new(Vec::new())),
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
    
    /// Set visualization type
    pub async fn set_visualization_type(&self, viz_type: VisualizationType) -> Result<()> {
        *self.visualization_type.write().await = viz_type;
        info!("Visualization type set to {:?}", viz_type);
        Ok(())
    }
    
    /// Get current visualization type
    pub async fn get_visualization_type(&self) -> VisualizationType {
        *self.visualization_type.read().await
    }
    
    /// Set color scheme
    pub async fn set_color_scheme(&self, scheme: ColorScheme) -> Result<()> {
        *self.color_scheme.write().await = scheme;
        info!("Color scheme updated");
        Ok(())
    }
    
    /// Set FFT size
    pub async fn set_fft_size(&self, size: usize) -> Result<()> {
        *self.fft_size.write().await = size;
        info!("FFT size set to {}", size);
        Ok(())
    }
    
    /// Update audio buffer
    pub async fn update(&self, samples: &[f32], sample_rate: u32) -> Result<()> {
        *self.audio_buffer.write().await = samples.to_vec();
        *self.sample_rate.write().await = sample_rate;
        
        // Compute spectrum
        self.compute_spectrum().await?;
        
        // Compute waveform
        self.compute_waveform().await?;
        
        // Update spectrogram
        self.update_spectrogram().await?;
        
        Ok(())
    }
    
    /// Compute frequency spectrum
    async fn compute_spectrum(&self) -> Result<()> {
        let buffer = self.audio_buffer.read().await;
        let fft_size = *self.fft_size.read().await;
        
        if buffer.len() < fft_size {
            return Ok(());
        }
        
        // Compute FFT
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        let mut spectrum = vec![Complex::new(0.0, 0.0); fft_size];
        for (i, &sample) in buffer.iter().take(fft_size).enumerate() {
            spectrum[i] = Complex::new(sample as f64, 0.0);
        }
        
        fft.process(&mut spectrum);
        
        // Extract magnitudes
        let mut magnitudes = Vec::with_capacity(fft_size / 2);
        for complex in spectrum.iter().take(fft_size / 2) {
            let magnitude = complex.norm() as f32;
            magnitudes.push(magnitude);
        }
        
        // Apply logarithmic scaling
        for mag in magnitudes.iter_mut() {
            *mag = 20.0 * mag.log10().max(0.0);
        }
        
        *self.spectrum_data.write().await = magnitudes;
        Ok(())
    }
    
    /// Compute waveform
    async fn compute_waveform(&self) -> Result<()> {
        let buffer = self.audio_buffer.read().await;
        
        // Downsample waveform for display
        let downsample_factor = (buffer.len() / 1000).max(1);
        let mut waveform = Vec::new();
        
        for chunk in buffer.chunks(downsample_factor) {
            let avg: f32 = chunk.iter().sum::<f32>() / chunk.len() as f32;
            waveform.push(avg);
        }
        
        *self.waveform_data.write().await = waveform;
        Ok(())
    }
    
    /// Update spectrogram
    async fn update_spectrogram(&self) -> Result<()> {
        let spectrum = self.spectrum_data.read().await;
        
        if spectrum.is_empty() {
            return Ok(());
        }
        
        let mut spectrogram = self.spectrogram_data.write().await;
        
        // Add new spectrum line
        spectrogram.push(spectrum.clone());
        
        // Keep only last 100 lines
        if spectrogram.len() > 100 {
            spectrogram.remove(0);
        }
        
        Ok(())
    }
    
    /// Generate visualization frame
    pub async fn generate_frame(&self, width: usize, height: usize) -> Result<VisualizationFrame> {
        let viz_type = *self.visualization_type.read().await;
        let color_scheme = self.color_scheme.read().await.clone();
        
        match viz_type {
            VisualizationType::Spectrum => {
                self.generate_spectrum_frame(width, height, &color_scheme).await
            }
            VisualizationType::Waveform => {
                self.generate_waveform_frame(width, height, &color_scheme).await
            }
            VisualizationType::Spectrogram => {
                self.generate_spectrogram_frame(width, height, &color_scheme).await
            }
            VisualizationType::FrequencyBands => {
                self.generate_frequency_bands_frame(width, height, &color_scheme).await
            }
            VisualizationType::CircularSpectrum => {
                self.generate_circular_spectrum_frame(width, height, &color_scheme).await
            }
            VisualizationType::ThreeD => {
                self.generate_3d_frame(width, height, &color_scheme).await
            }
        }
    }
    
    /// Generate spectrum visualization
    async fn generate_spectrum_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        let spectrum = self.spectrum_data.read().await;
        let mut img = RgbImage::new(width as u32, height as u32);
        
        let bar_width = width / spectrum.len().max(1);
        
        for (i, &magnitude) in spectrum.iter().enumerate() {
            let bar_height = ((magnitude / 100.0) * height as f32).min(height as f32) as usize;
            
            let x = i * bar_width;
            
            for y in (height - bar_height)..height {
                let pixel = Rgb([
                    scheme.primary.0,
                    scheme.primary.1,
                    scheme.primary.2,
                ]);
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        
        Ok(VisualizationFrame {
            data: img.into_raw(),
            width,
            height,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Generate waveform visualization
    async fn generate_waveform_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        let waveform = self.waveform_data.read().await;
        let mut img = RgbImage::new(width as u32, height as u32);
        
        let center_y = height / 2;
        
        for (i, &sample) in waveform.iter().enumerate() {
            let x = (i * width / waveform.len().max(1)).min(width - 1);
            let y_offset = ((sample.abs() * height as f32 / 2.0) as usize).min(center_y);
            
            let y_start = center_y - y_offset;
            let y_end = center_y + y_offset;
            
            for y in y_start..=y_end.min(height - 1) {
                let pixel = Rgb([
                    scheme.secondary.0,
                    scheme.secondary.1,
                    scheme.secondary.2,
                ]);
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        
        Ok(VisualizationFrame {
            data: img.into_raw(),
            width,
            height,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Generate spectrogram visualization
    async fn generate_spectrogram_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        let spectrogram = self.spectrogram_data.read().await;
        let mut img = RgbImage::new(width as u32, height as u32);
        
        if spectrogram.is_empty() {
            return Ok(VisualizationFrame {
                data: img.into_raw(),
                width,
                height,
                timestamp: chrono::Utc::now(),
            });
        }
        
        let num_lines = spectrogram.len();
        let num_bins = spectrogram[0].len();
        
        for (line_idx, line) in spectrogram.iter().enumerate() {
            let y = (line_idx * height / num_lines).min(height - 1);
            
            for (bin_idx, &magnitude) in line.iter().enumerate() {
                let x = (bin_idx * width / num_bins).min(width - 1);
                
                // Map magnitude to color intensity
                let intensity = ((magnitude / 100.0) * 255.0).min(255.0) as u8;
                
                let pixel = Rgb([
                    (scheme.primary.0 as f32 * intensity as f32 / 255.0) as u8,
                    (scheme.primary.1 as f32 * intensity as f32 / 255.0) as u8,
                    (scheme.primary.2 as f32 * intensity as f32 / 255.0) as u8,
                ]);
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        
        Ok(VisualizationFrame {
            data: img.into_raw(),
            width,
            height,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Generate frequency bands visualization
    async fn generate_frequency_bands_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        let bands = self.get_frequency_bands().await?;
        
        let mut img = RgbImage::new(width as u32, height as u32);
        
        let band_width = width / 5;
        let bands_data = [bands.bass, bands.low_mid, bands.mid, bands.high_mid, bands.treble];
        
        for (i, &level) in bands_data.iter().enumerate() {
            let bar_height = ((level / 100.0) * height as f32).min(height as f32) as usize;
            let x = i * band_width;
            
            for y in (height - bar_height)..height {
                let pixel = Rgb([
                    scheme.primary.0,
                    scheme.primary.1,
                    scheme.primary.2,
                ]);
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        
        Ok(VisualizationFrame {
            data: img.into_raw(),
            width,
            height,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Generate circular spectrum visualization
    async fn generate_circular_spectrum_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        let spectrum = self.spectrum_data.read().await;
        let mut img = RgbImage::new(width as u32, height as u32);
        
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = width.min(height) / 3;
        
        for (i, &magnitude) in spectrum.iter().enumerate() {
            let angle = (i as f32 / spectrum.len() as f32) * 2.0 * std::f32::consts::PI;
            let bar_length = ((magnitude / 100.0) * radius as f32).min(radius as f32) as usize;
            
            for j in 0..bar_length {
                let r = radius - j;
                let x = (center_x as f32 + r as f32 * angle.cos()) as usize;
                let y = (center_y as f32 + r as f32 * angle.sin()) as usize;
                
                if x < width && y < height {
                    let pixel = Rgb([
                        scheme.primary.0,
                        scheme.primary.1,
                        scheme.primary.2,
                    ]);
                    img.put_pixel(x as u32, y as u32, pixel);
                }
            }
        }
        
        Ok(VisualizationFrame {
            data: img.into_raw(),
            width,
            height,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Generate 3D visualization
    async fn generate_3d_frame(&self, width: usize, height: usize, scheme: &ColorScheme) -> Result<VisualizationFrame> {
        // Simplified 3D visualization - use spectrum with perspective
        self.generate_spectrum_frame(width, height, scheme).await
    }
    
    /// Get frequency bands
    pub async fn get_frequency_bands(&self) -> Result<FrequencyBands> {
        let spectrum = self.spectrum_data.read().await;
        let sample_rate = *self.sample_rate.read().await;
        
        if spectrum.is_empty() {
            return Ok(FrequencyBands {
                bass: 0.0,
                low_mid: 0.0,
                mid: 0.0,
                high_mid: 0.0,
                treble: 0.0,
            });
        }
        
        let fft_size = spectrum.len();
        let bin_size = sample_rate as f32 / fft_size as f32;
        
        // Calculate band indices
        let bass_start = (20.0 / bin_size) as usize;
        let bass_end = (250.0 / bin_size) as usize;
        let low_mid_end = (500.0 / bin_size) as usize;
        let mid_end = (2000.0 / bin_size) as usize;
        let high_mid_end = (4000.0 / bin_size) as usize;
        let treble_end = (20000.0 / bin_size).min(fft_size as f32) as usize;
        
        // Calculate average levels for each band
        let bass = spectrum[bass_start..bass_end].iter().sum::<f32>() / (bass_end - bass_start).max(1) as f32;
        let low_mid = spectrum[bass_end..low_mid_end].iter().sum::<f32>() / (low_mid_end - bass_end).max(1) as f32;
        let mid = spectrum[low_mid_end..mid_end].iter().sum::<f32>() / (mid_end - low_mid_end).max(1) as f32;
        let high_mid = spectrum[mid_end..high_mid_end].iter().sum::<f32>() / (high_mid_end - mid_end).max(1) as f32;
        let treble = spectrum[high_mid_end..treble_end].iter().sum::<f32>() / (treble_end - high_mid_end).max(1) as f32;
        
        Ok(FrequencyBands {
            bass,
            low_mid,
            mid,
            high_mid,
            treble,
        })
    }
    
    /// Get current spectrum data
    pub async fn get_spectrum(&self) -> Vec<f32> {
        self.spectrum_data.read().await.clone()
    }
    
    /// Get current waveform data
    pub async fn get_waveform(&self) -> Vec<f32> {
        self.waveform_data.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_visualizer_creation() {
        let visualizer = AudioVisualizer::new().unwrap();
        assert!(visualizer.is_initialized());
    }
    
    #[tokio::test]
    async fn test_update_and_spectrum() {
        let visualizer = AudioVisualizer::new().unwrap();
        
        let samples: Vec<f32> = (0..48000).map(|i| (i as f32 / 48000.0 * 2.0 * std::f32::consts::PI * 440.0).sin() as f32).collect();
        visualizer.update(&samples, 48000).await.unwrap();
        
        let spectrum = visualizer.get_spectrum().await;
        assert!(!spectrum.is_empty());
    }
    
    #[tokio::test]
    async fn test_frequency_bands() {
        let visualizer = AudioVisualizer::new().unwrap();
        
        let samples: Vec<f32> = (0..48000).map(|i| (i as f32 / 48000.0 * 2.0 * std::f32::consts::PI * 440.0).sin() as f32).collect();
        visualizer.update(&samples, 48000).await.unwrap();
        
        let bands = visualizer.get_frequency_bands().await.unwrap();
        assert!(bands.mid > 0.0); // 440Hz should be in mid band
    }
    
    #[tokio::test]
    async fn test_frame_generation() {
        let visualizer = AudioVisualizer::new().unwrap();
        
        let samples: Vec<f32> = (0..48000).map(|i| (i as f32 / 48000.0 * 2.0 * std::f32::consts::PI * 440.0).sin() as f32).collect();
        visualizer.update(&samples, 48000).await.unwrap();
        
        let frame = visualizer.generate_frame(800, 600).await.unwrap();
        assert_eq!(frame.width, 800);
        assert_eq!(frame.height, 600);
        assert_eq!(frame.data.len(), 800 * 600 * 3);
    }
}