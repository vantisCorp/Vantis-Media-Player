//! Audio Visualization
//! 
//! This module provides real-time audio visualization including:
//! - Spectrum analyzer (FFT-based)
//! - Waveform display
//! - Volume metering (peak/RMS)
//! - Audio level history

use std::collections::VecDeque;

// ============================================================================
// FFT-based Spectrum Analyzer
// ============================================================================

/// Complex number for FFT
#[derive(Debug, Clone, Copy, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    pub fn phase(&self) -> f32 {
        self.im.atan2(self.re)
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { re: self.re + other.re, im: self.im + other.im }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { re: self.re - other.re, im: self.im - other.im }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// FFT processor
#[derive(Debug)]
pub struct FFT {
    /// FFT size (must be power of 2)
    size: usize,
    /// Bit-reversed indices
    bit_reverse: Vec<usize>,
    /// Twiddle factors
    twiddles: Vec<Complex>,
}

impl FFT {
    /// Create new FFT processor
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "FFT size must be power of 2");
        
        let log2_n = size.trailing_zeros() as usize;
        let mut bit_reverse = vec![0; size];
        
        for i in 0..size {
            let mut rev = 0;
            for j in 0..log2_n {
                if (i >> j) & 1 != 0 {
                    rev |= 1 << (log2_n - 1 - j);
                }
            }
            bit_reverse[i] = rev;
        }

        let mut twiddles = Vec::with_capacity(size / 2);
        let pi = std::f32::consts::PI;
        for i in 0..size / 2 {
            let angle = -2.0 * pi * i as f32 / size as f32;
            twiddles.push(Complex::new(angle.cos(), angle.sin()));
        }

        Self { size, bit_reverse, twiddles }
    }

    /// Perform FFT on real-valued input
    pub fn forward(&self, input: &[f32]) -> Vec<Complex> {
        assert_eq!(input.len(), self.size, "Input size must match FFT size");

        // Convert real input to complex
        let mut data: Vec<Complex> = input
            .iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();

        // Bit-reversal permutation
        for i in 0..self.size {
            let j = self.bit_reverse[i];
            if i < j {
                data.swap(i, j);
            }
        }

        // Cooley-Tukey FFT
        let mut m = 1;
        while m < self.size {
            let half_m = m;
            m *= 2;

            for k in (0..self.size).step_by(m) {
                for j in 0..half_m {
                    let t = self.twiddles[j * self.size / m];
                    let u = data[k + j];
                    let v = data[k + j + half_m] * t;
                    data[k + j] = u + v;
                    data[k + j + half_m] = u - v;
                }
            }
        }

        data
    }

    /// Get FFT size
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Spectrum analyzer
#[derive(Debug)]
pub struct SpectrumAnalyzer {
    /// FFT processor
    fft: FFT,
    /// Window function
    window: Vec<f32>,
    /// Sample buffer
    buffer: Vec<f32>,
    /// Buffer write position
    write_pos: usize,
    /// Spectrum magnitudes (dB)
    magnitudes: Vec<f32>,
    /// Number of frequency bands
    num_bands: usize,
    /// Sample rate
    sample_rate: f32,
}

impl SpectrumAnalyzer {
    /// Create new spectrum analyzer
    pub fn new(sample_rate: f32, fft_size: usize, num_bands: usize) -> Self {
        let fft = FFT::new(fft_size);
        let window = Self::hann_window(fft_size);
        let buffer = vec![0.0; fft_size];
        let magnitudes = vec![-100.0; num_bands];

        Self {
            fft,
            window,
            buffer,
            write_pos: 0,
            magnitudes,
            num_bands,
            sample_rate,
        }
    }

    /// Hann window function
    fn hann_window(size: usize) -> Vec<f32> {
        let pi = std::f32::consts::PI;
        (0..size)
            .map(|i| 0.5 * (1.0 - (2.0 * pi * i as f32 / (size - 1) as f32).cos()))
            .collect()
    }

    /// Add sample to buffer
    pub fn push_sample(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
    }

    /// Process buffer of samples
    pub fn process(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.push_sample(sample);
        }
    }

    /// Calculate spectrum
    pub fn calculate(&mut self) -> &[f32] {
        let fft_size = self.fft.size();
        
        // Apply window and prepare input
        let windowed: Vec<f32> = self.buffer
            .iter()
            .zip(self.window.iter())
            .map(|(&s, &w)| s * w)
            .collect();

        // Perform FFT
        let spectrum = self.fft.forward(&windowed);

        // Convert to magnitude and map to bands
        let bin_freq = self.sample_rate / fft_size as f32;
        let bins_per_band = (fft_size / 2 / self.num_bands).max(1);

        self.magnitudes.clear();
        self.magnitudes.resize(self.num_bands, -100.0);

        for band in 0..self.num_bands {
            let start_bin = band * bins_per_band;
            let end_bin = ((band + 1) * bins_per_band).min(fft_size / 2);
            
            if start_bin >= fft_size / 2 {
                break;
            }

            // Average magnitude in band
            let mut sum = 0.0;
            let mut count = 0;
            for bin in start_bin..end_bin {
                let mag = spectrum[bin].magnitude_squared();
                sum += mag;
                count += 1;
            }

            let avg_mag = sum / count as f32;
            let db = if avg_mag > 0.0 {
                10.0 * avg_mag.log10()
            } else {
                -100.0
            };
            self.magnitudes[band] = db.clamp(-100.0, 0.0);
        }

        &self.magnitudes
    }

    /// Get frequency for band
    pub fn band_frequency(&self, band: usize) -> f32 {
        let bin_freq = self.sample_rate / self.fft.size() as f32;
        let bins_per_band = (self.fft.size() / 2 / self.num_bands).max(1);
        band as f32 * bins_per_band as f32 * bin_freq
    }

    /// Get magnitudes
    pub fn magnitudes(&self) -> &[f32] {
        &self.magnitudes
    }
}

// ============================================================================
// Waveform Display
// ============================================================================

/// Waveform display data
#[derive(Debug, Clone, Default)]
pub struct WaveformData {
    /// Minimum values for each sample position
    pub min: Vec<f32>,
    /// Maximum values for each sample position
    pub max: Vec<f32>,
}

/// Waveform analyzer
#[derive(Debug)]
pub struct WaveformAnalyzer {
    /// Sample buffer
    buffer: VecDeque<f32>,
    /// Maximum buffer size
    max_size: usize,
    /// Downsample factor
    downsample: usize,
    /// Current waveform data
    data: WaveformData,
}

impl WaveformAnalyzer {
    /// Create new waveform analyzer
    pub fn new(max_size: usize, downsample: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
            downsample: downsample.max(1),
            data: WaveformData::default(),
        }
    }

    /// Push sample
    pub fn push(&mut self, sample: f32) {
        if self.buffer.len() >= self.max_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    /// Process buffer
    pub fn process(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.push(sample);
        }
    }

    /// Get waveform data
    pub fn get_waveform(&mut self, num_points: usize) -> &WaveformData {
        let samples_per_point = self.buffer.len() / num_points.max(1);
        
        self.data.min.clear();
        self.data.max.clear();
        self.data.min.reserve(num_points);
        self.data.max.reserve(num_points);

        for i in 0..num_points {
            let start = i * samples_per_point;
            let end = ((i + 1) * samples_per_point).min(self.buffer.len());
            
            if start >= self.buffer.len() {
                self.data.min.push(0.0);
                self.data.max.push(0.0);
                continue;
            }

            let mut min = f32::MAX;
            let mut max = f32::MIN;
            
            for j in start..end {
                let sample = self.buffer[j];
                min = min.min(sample);
                max = max.max(sample);
            }

            self.data.min.push(min);
            self.data.max.push(max);
        }

        &self.data
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.data.min.clear();
        self.data.max.clear();
    }
}

// ============================================================================
// Volume Metering
// ============================================================================

/// Volume meter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterType {
    Peak,
    RMS,
    VU,
    LUFS,
}

/// Volume meter
#[derive(Debug)]
pub struct VolumeMeter {
    /// Meter type
    meter_type: MeterType,
    /// Current level (linear 0-1)
    level: f32,
    /// Peak level
    peak: f32,
    /// Peak hold time in samples
    peak_hold_samples: usize,
    /// Peak hold counter
    peak_hold_counter: usize,
    /// RMS window
    rms_window: VecDeque<f32>,
    /// RMS window size
    rms_window_size: usize,
    /// Sample rate
    sample_rate: f32,
}

impl VolumeMeter {
    /// Create new volume meter
    pub fn new(meter_type: MeterType, sample_rate: f32) -> Self {
        let rms_window_size = (0.3 * sample_rate) as usize; // 300ms window
        
        Self {
            meter_type,
            level: 0.0,
            peak: 0.0,
            peak_hold_samples: (1.5 * sample_rate) as usize, // 1.5s hold
            peak_hold_counter: 0,
            rms_window: VecDeque::with_capacity(rms_window_size),
            rms_window_size,
            sample_rate,
        }
    }

    /// Process sample
    pub fn process(&mut self, sample: f32) -> f32 {
        let abs_sample = sample.abs();

        // Update peak
        if abs_sample > self.peak {
            self.peak = abs_sample;
            self.peak_hold_counter = 0;
        } else {
            self.peak_hold_counter += 1;
            if self.peak_hold_counter > self.peak_hold_samples {
                self.peak = self.peak * 0.999; // Slow decay
            }
        }

        // Update level based on meter type
        self.level = match self.meter_type {
            MeterType::Peak => abs_sample,
            MeterType::RMS => {
                self.rms_window.push_back(abs_sample * abs_sample);
                if self.rms_window.len() > self.rms_window_size {
                    self.rms_window.pop_front();
                }
                let sum: f32 = self.rms_window.iter().sum();
                (sum / self.rms_window.len().max(1) as f32).sqrt()
            }
            MeterType::VU => {
                // VU meter approximates RMS with ballistics
                let target = abs_sample;
                let coeff = 0.99; // Slow response
                self.level = self.level * coeff + target * (1.0 - coeff);
                self.level
            }
            MeterType::LUFS => {
                // Simplified LUFS (would need K-weighting filter for proper implementation)
                self.rms_window.push_back(abs_sample * abs_sample);
                if self.rms_window.len() > self.rms_window_size {
                    self.rms_window.pop_front();
                }
                let sum: f32 = self.rms_window.iter().sum();
                (sum / self.rms_window.len().max(1) as f32).sqrt()
            }
        };

        self.level
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.process(sample);
        }
    }

    /// Get current level (linear)
    pub fn level(&self) -> f32 {
        self.level
    }

    /// Get current level in dB
    pub fn level_db(&self) -> f32 {
        if self.level > 0.0 {
            20.0 * self.level.log10()
        } else {
            -100.0
        }
    }

    /// Get peak level
    pub fn peak(&self) -> f32 {
        self.peak
    }

    /// Get peak level in dB
    pub fn peak_db(&self) -> f32 {
        if self.peak > 0.0 {
            20.0 * self.peak.log10()
        } else {
            -100.0
        }
    }

    /// Reset meter
    pub fn reset(&mut self) {
        self.level = 0.0;
        self.peak = 0.0;
        self.peak_hold_counter = 0;
        self.rms_window.clear();
    }
}

// ============================================================================
// Stereo Level Meter
// ============================================================================

/// Stereo level meter
#[derive(Debug)]
pub struct StereoLevelMeter {
    /// Left channel meter
    left: VolumeMeter,
    /// Right channel meter
    right: VolumeMeter,
    /// Correlation meter
    correlation: f32,
    /// Correlation accumulator
    corr_sum_l: f32,
    corr_sum_r: f32,
    corr_sum_lr: f32,
    corr_count: usize,
}

impl StereoLevelMeter {
    /// Create new stereo level meter
    pub fn new(meter_type: MeterType, sample_rate: f32) -> Self {
        Self {
            left: VolumeMeter::new(meter_type, sample_rate),
            right: VolumeMeter::new(meter_type, sample_rate),
            correlation: 1.0,
            corr_sum_l: 0.0,
            corr_sum_r: 0.0,
            corr_sum_lr: 0.0,
            corr_count: 0,
        }
    }

    /// Process stereo sample
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        self.left.process(left);
        self.right.process(right);

        // Update correlation
        self.corr_sum_l += left * left;
        self.corr_sum_r += right * right;
        self.corr_sum_lr += left * right;
        self.corr_count += 1;

        if self.corr_count >= 1024 {
            let var_l = self.corr_sum_l / self.corr_count as f32;
            let var_r = self.corr_sum_r / self.corr_count as f32;
            let cov_lr = self.corr_sum_lr / self.corr_count as f32;

            self.correlation = if var_l > 0.0 && var_r > 0.0 {
                (cov_lr / (var_l * var_r).sqrt()).clamp(-1.0, 1.0)
            } else {
                1.0
            };

            self.corr_sum_l = 0.0;
            self.corr_sum_r = 0.0;
            self.corr_sum_lr = 0.0;
            self.corr_count = 0;
        }

        (self.left.level(), self.right.level())
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &[f32]) {
        for chunk in samples.chunks(2) {
            let left = chunk.get(0).copied().unwrap_or(0.0);
            let right = chunk.get(1).copied().unwrap_or(0.0);
            self.process(left, right);
        }
    }

    /// Get left level
    pub fn left(&self) -> &VolumeMeter {
        &self.left
    }

    /// Get right level
    pub fn right(&self) -> &VolumeMeter {
        &self.right
    }

    /// Get correlation (-1 to 1)
    pub fn correlation(&self) -> f32 {
        self.correlation
    }

    /// Reset meters
    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.correlation = 1.0;
        self.corr_sum_l = 0.0;
        self.corr_sum_r = 0.0;
        self.corr_sum_lr = 0.0;
        self.corr_count = 0;
    }
}

// ============================================================================
// Level History
// ============================================================================

/// Audio level history for graphing
#[derive(Debug)]
pub struct LevelHistory {
    /// History buffer
    buffer: VecDeque<f32>,
    /// Maximum history size
    max_size: usize,
    /// Sample accumulator
    accumulator: f32,
    /// Sample count
    sample_count: usize,
    /// Samples per history point
    samples_per_point: usize,
}

impl LevelHistory {
    /// Create new level history
    pub fn new(max_size: usize, samples_per_point: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
            accumulator: 0.0,
            sample_count: 0,
            samples_per_point,
        }
    }

    /// Add sample
    pub fn push(&mut self, sample: f32) {
        self.accumulator += sample * sample;
        self.sample_count += 1;

        if self.sample_count >= self.samples_per_point {
            let rms = (self.accumulator / self.sample_count as f32).sqrt();
            let db = if rms > 0.0 { 20.0 * rms.log10() } else { -100.0 };
            
            if self.buffer.len() >= self.max_size {
                self.buffer.pop_front();
            }
            self.buffer.push_back(db.clamp(-100.0, 0.0));

            self.accumulator = 0.0;
            self.sample_count = 0;
        }
    }

    /// Process buffer
    pub fn process(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.push(sample);
        }
    }

    /// Get history
    pub fn history(&self) -> &VecDeque<f32> {
        &self.buffer
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.accumulator = 0.0;
        self.sample_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_creation() {
        let fft = FFT::new(1024);
        assert_eq!(fft.size(), 1024);
    }

    #[test]
    fn test_fft_forward() {
        let fft = FFT::new(64);
        let input = vec![1.0; 64];
        let output = fft.forward(&input);
        assert_eq!(output.len(), 64);
        // DC component should be large for constant input
        assert!(output[0].magnitude() > 60.0);
    }

    #[test]
    fn test_spectrum_analyzer() {
        let mut analyzer = SpectrumAnalyzer::new(44100.0, 1024, 32);
        for _ in 0..1024 {
            analyzer.push_sample(0.5);
        }
        let mags = analyzer.calculate();
        assert_eq!(mags.len(), 32);
    }

    #[test]
    fn test_volume_meter() {
        let mut meter = VolumeMeter::new(MeterType::Peak, 44100.0);
        meter.process(0.5);
        assert!(meter.level() > 0.0);
        assert!(meter.peak() > 0.0);
    }

    #[test]
    fn test_stereo_level_meter() {
        let mut meter = StereoLevelMeter::new(MeterType::Peak, 44100.0);
        let (l, r) = meter.process(0.5, 0.5);
        assert!(l > 0.0);
        assert!(r > 0.0);
    }

    #[test]
    fn test_waveform_analyzer() {
        let mut analyzer = WaveformAnalyzer::new(1024, 1);
        for i in 0..1024 {
            analyzer.push((i as f32 / 1024.0 - 0.5) * 2.0);
        }
        let waveform = analyzer.get_waveform(100);
        assert_eq!(waveform.min.len(), 100);
        assert_eq!(waveform.max.len(), 100);
    }
}