//! Audio Equalizer
//! 
//! Multi-band parametric equalizer with:
//! - 10-band equalizer with adjustable frequencies
//! - Preset profiles (rock, pop, classical, jazz, etc.)
//! - Custom curve support
//! - Bass and treble enhancement

use crate::filters::{BiquadFilter, BiquadCoefficients};
use serde::{Deserialize, Serialize};

// ============================================================================
// Equalizer Band
// ============================================================================

/// Equalizer band
#[derive(Debug, Clone)]
pub struct EqBand {
    /// Center frequency in Hz
    pub frequency: f32,
    /// Gain in dB
    pub gain: f32,
    /// Q factor
    pub q: f32,
    /// Filter type
    pub filter_type: EqBandType,
    /// Whether the band is enabled
    pub enabled: bool,
    /// Filter instance
    filter: Option<BiquadFilter>,
}

impl EqBand {
    /// Create new EQ band
    pub fn new(frequency: f32, q: f32) -> Self {
        Self {
            frequency,
            gain: 0.0,
            q,
            filter_type: EqBandType::Peaking,
            enabled: true,
            filter: None,
        }
    }

    /// Initialize filter
    pub fn init_filter(&mut self, sample_rate: f32) {
        let coeffs = match self.filter_type {
            EqBandType::LowShelf => {
                BiquadCoefficients::low_shelf(sample_rate, self.frequency, self.gain)
            }
            EqBandType::HighShelf => {
                BiquadCoefficients::high_shelf(sample_rate, self.frequency, self.gain)
            }
            EqBandType::Peaking => {
                BiquadCoefficients::peaking(sample_rate, self.frequency, self.q, self.gain)
            }
            EqBandType::LowPass => {
                BiquadCoefficients::low_pass(sample_rate, self.frequency, self.q)
            }
            EqBandType::HighPass => {
                BiquadCoefficients::high_pass(sample_rate, self.frequency, self.q)
            }
            EqBandType::Notch => {
                BiquadCoefficients::notch(sample_rate, self.frequency, self.q)
            }
        };

        self.filter = Some(BiquadFilter::new(coeffs));
    }

    /// Process sample through band
    pub fn process(&mut self, sample: f32) -> f32 {
        if !self.enabled || self.gain.abs() < 0.01 {
            return sample;
        }

        if let Some(ref mut filter) = self.filter {
            filter.process(sample)
        } else {
            sample
        }
    }

    /// Set gain
    pub fn set_gain(&mut self, gain: f32, sample_rate: f32) {
        self.gain = gain.clamp(-24.0, 24.0);
        self.init_filter(sample_rate);
    }

    /// Set frequency
    pub fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.frequency = frequency.max(20.0).min(20000.0);
        self.init_filter(sample_rate);
    }

    /// Set Q
    pub fn set_q(&mut self, q: f32, sample_rate: f32) {
        self.q = q.max(0.1).min(20.0);
        self.init_filter(sample_rate);
    }
}

/// Equalizer band type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqBandType {
    Peaking,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
    Notch,
}

// ============================================================================
// Equalizer Presets
// ============================================================================

/// Equalizer preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqPreset {
    /// Preset name
    pub name: String,
    /// Band gains in dB
    pub gains: [f32; 10],
}

impl EqPreset {
    /// Flat preset
    pub fn flat() -> Self {
        Self {
            name: "Flat".to_string(),
            gains: [0.0; 10],
        }
    }

    /// Bass boost preset
    pub fn bass_boost() -> Self {
        Self {
            name: "Bass Boost".to_string(),
            gains: [6.0, 5.0, 4.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Treble boost preset
    pub fn treble_boost() -> Self {
        Self {
            name: "Treble Boost".to_string(),
            gains: [0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 4.0, 5.0, 6.0, 6.0],
        }
    }

    /// Rock preset
    pub fn rock() -> Self {
        Self {
            name: "Rock".to_string(),
            gains: [5.0, 4.0, 2.0, 0.0, -1.0, -1.0, 1.0, 3.0, 5.0, 6.0],
        }
    }

    /// Pop preset
    pub fn pop() -> Self {
        Self {
            name: "Pop".to_string(),
            gains: [-1.0, 1.0, 3.0, 4.0, 3.0, 0.0, -1.0, -1.0, -1.0, -1.0],
        }
    }

    /// Classical preset
    pub fn classical() -> Self {
        Self {
            name: "Classical".to_string(),
            gains: [4.0, 3.0, 2.0, 1.0, -1.0, -1.0, 0.0, 2.0, 3.0, 4.0],
        }
    }

    /// Jazz preset
    pub fn jazz() -> Self {
        Self {
            name: "Jazz".to_string(),
            gains: [3.0, 2.0, 1.0, 2.0, -2.0, -2.0, 0.0, 1.0, 2.0, 3.0],
        }
    }

    /// Electronic preset
    pub fn electronic() -> Self {
        Self {
            name: "Electronic".to_string(),
            gains: [4.0, 3.0, 1.0, 0.0, -1.0, 2.0, 1.0, 1.0, 4.0, 5.0],
        }
    }

    /// Hip-hop preset
    pub fn hip_hop() -> Self {
        Self {
            name: "Hip-Hop".to_string(),
            gains: [5.0, 4.0, 2.0, 1.0, -1.0, -1.0, 1.0, 0.0, 1.0, 3.0],
        }
    }

    /// Vocal preset
    pub fn vocal() -> Self {
        Self {
            name: "Vocal".to_string(),
            gains: [-2.0, -1.0, 0.0, 2.0, 4.0, 4.0, 3.0, 1.0, 0.0, -2.0],
        }
    }

    /// Live preset
    pub fn live() -> Self {
        Self {
            name: "Live".to_string(),
            gains: [-3.0, -1.0, 0.0, 1.0, 2.0, 2.0, 1.0, 0.0, -1.0, -2.0],
        }
    }

    /// Get all presets
    pub fn all() -> Vec<Self> {
        vec![
            Self::flat(),
            Self::bass_boost(),
            Self::treble_boost(),
            Self::rock(),
            Self::pop(),
            Self::classical(),
            Self::jazz(),
            Self::electronic(),
            Self::hip_hop(),
            Self::vocal(),
            Self::live(),
        ]
    }
}

// ============================================================================
// Parametric Equalizer
// ============================================================================

/// Standard frequencies for 10-band equalizer
pub const STANDARD_FREQUENCIES: [f32; 10] = [
    31.0,   // Sub-bass
    62.0,   // Bass
    125.0,  // Upper bass
    250.0,  // Lower midrange
    500.0,  // Midrange
    1000.0, // Upper midrange
    2000.0, // Presence
    4000.0, // Treble
    8000.0, // Upper treble
    16000.0, // Air
];

/// Multi-band parametric equalizer
#[derive(Debug, Clone)]
pub struct ParametricEqualizer {
    /// EQ bands
    bands: Vec<EqBand>,
    /// Sample rate
    sample_rate: f32,
    /// Global bypass
    bypass: bool,
    /// Preamp gain in dB
    preamp: f32,
    /// Current preset name
    current_preset: Option<String>,
}

impl ParametricEqualizer {
    /// Create new parametric equalizer
    pub fn new(sample_rate: f32) -> Self {
        let mut bands = Vec::new();
        
        for (i, &freq) in STANDARD_FREQUENCIES.iter().enumerate() {
            let mut band = EqBand::new(freq, 1.4);
            
            // Set band types for first and last bands
            if i == 0 {
                band.filter_type = EqBandType::LowShelf;
                band.q = 0.7;
            } else if i == STANDARD_FREQUENCIES.len() - 1 {
                band.filter_type = EqBandType::HighShelf;
                band.q = 0.7;
            } else {
                band.filter_type = EqBandType::Peaking;
            }
            
            band.init_filter(sample_rate);
            bands.push(band);
        }

        Self {
            bands,
            sample_rate,
            bypass: false,
            preamp: 0.0,
            current_preset: Some("Flat".to_string()),
        }
    }

    /// Process a single sample
    pub fn process(&mut self, sample: f32) -> f32 {
        if self.bypass {
            return sample;
        }

        // Apply preamp
        let preamp_linear = 10.0_f32.powf(self.preamp / 20.0);
        let mut output = sample * preamp_linear;

        // Process through each band
        for band in &mut self.bands {
            output = band.process(output);
        }

        output
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Get number of bands
    pub fn num_bands(&self) -> usize {
        self.bands.len()
    }

    /// Get band by index
    pub fn band(&self, index: usize) -> Option<&EqBand> {
        self.bands.get(index)
    }

    /// Get mutable band by index
    pub fn band_mut(&mut self, index: usize) -> Option<&mut EqBand> {
        self.bands.get_mut(index)
    }

    /// Set band gain
    pub fn set_band_gain(&mut self, band: usize, gain: f32) {
        if let Some(b) = self.bands.get_mut(band) {
            b.set_gain(gain, self.sample_rate);
            self.current_preset = None;
        }
    }

    /// Get band gain
    pub fn get_band_gain(&self, band: usize) -> f32 {
        self.bands.get(band).map(|b| b.gain).unwrap_or(0.0)
    }

    /// Set all band gains
    pub fn set_gains(&mut self, gains: &[f32]) {
        for (i, &gain) in gains.iter().enumerate() {
            if i < self.bands.len() {
                self.bands[i].set_gain(gain, self.sample_rate);
            }
        }
        self.current_preset = None;
    }

    /// Get all band gains
    pub fn get_gains(&self) -> [f32; 10] {
        let mut gains = [0.0; 10];
        for (i, band) in self.bands.iter().enumerate() {
            if i < 10 {
                gains[i] = band.gain;
            }
        }
        gains
    }

    /// Apply preset
    pub fn apply_preset(&mut self, preset: &EqPreset) {
        for (i, &gain) in preset.gains.iter().enumerate() {
            if i < self.bands.len() {
                self.bands[i].set_gain(gain, self.sample_rate);
            }
        }
        self.current_preset = Some(preset.name.clone());
    }

    /// Get current preset name
    pub fn current_preset(&self) -> Option<&str> {
        self.current_preset.as_deref()
    }

    /// Set preamp
    pub fn set_preamp(&mut self, gain_db: f32) {
        self.preamp = gain_db.clamp(-12.0, 12.0);
    }

    /// Get preamp
    pub fn preamp(&self) -> f32 {
        self.preamp
    }

    /// Set bypass
    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    /// Get bypass state
    pub fn bypass(&self) -> bool {
        self.bypass
    }

    /// Reset to flat
    pub fn reset(&mut self) {
        for band in &mut self.bands {
            band.gain = 0.0;
            band.init_filter(self.sample_rate);
        }
        self.preamp = 0.0;
        self.current_preset = Some("Flat".to_string());
    }

    /// Update sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        for band in &mut self.bands {
            band.init_filter(sample_rate);
        }
    }
}

// ============================================================================
// Bass and Treble Enhancement
// ============================================================================

/// Bass enhancement
#[derive(Debug, Clone)]
pub struct BassEnhancer {
    /// Bass gain in dB
    pub gain: f32,
    /// Cutoff frequency
    pub cutoff: f32,
    /// Harmonics amount
    pub harmonics: f32,
    /// Low-pass filter
    filter: BiquadFilter,
    /// Sample rate
    sample_rate: f32,
}

impl BassEnhancer {
    /// Create new bass enhancer
    pub fn new(sample_rate: f32) -> Self {
        let filter = BiquadFilter::low_pass(sample_rate, 150.0, 0.7);

        Self {
            gain: 0.0,
            cutoff: 150.0,
            harmonics: 0.5,
            filter,
            sample_rate,
        }
    }

    /// Process sample
    pub fn process(&mut self, input: f32) -> f32 {
        if self.gain.abs() < 0.01 {
            return input;
        }

        // Extract bass frequencies
        let bass = self.filter.process(input);
        
        // Add harmonics for perception of more bass
        let harmonic = bass.abs().powi(2) * bass.signum() * self.harmonics;
        
        // Mix with gain
        let gain_linear = 10.0_f32.powf(self.gain / 20.0);
        input + (bass + harmonic) * (gain_linear - 1.0)
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Set gain
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(0.0, 12.0);
    }

    /// Set cutoff
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(50.0).min(300.0);
        self.filter = BiquadFilter::low_pass(self.sample_rate, self.cutoff, 0.7);
    }
}

/// Treble enhancement
#[derive(Debug, Clone)]
pub struct TrebleEnhancer {
    /// Treble gain in dB
    pub gain: f32,
    /// Cutoff frequency
    pub cutoff: f32,
    /// High-pass filter
    filter: BiquadFilter,
    /// Sample rate
    sample_rate: f32,
}

impl TrebleEnhancer {
    /// Create new treble enhancer
    pub fn new(sample_rate: f32) -> Self {
        let filter = BiquadFilter::high_pass(sample_rate, 4000.0, 0.7);

        Self {
            gain: 0.0,
            cutoff: 4000.0,
            filter,
            sample_rate,
        }
    }

    /// Process sample
    pub fn process(&mut self, input: f32) -> f32 {
        if self.gain.abs() < 0.01 {
            return input;
        }

        // Extract treble frequencies
        let treble = self.filter.process(input);
        
        // Mix with gain
        let gain_linear = 10.0_f32.powf(self.gain / 20.0);
        input + treble * (gain_linear - 1.0)
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Set gain
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(0.0, 12.0);
    }

    /// Set cutoff
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(2000.0).min(10000.0);
        self.filter = BiquadFilter::high_pass(self.sample_rate, self.cutoff, 0.7);
    }
}

// ============================================================================
// Loudness Normalization
// ============================================================================

/// EBU R128 loudness normalizer
#[derive(Debug, Clone)]
pub struct LoudnessNormalizer {
    /// Target loudness in LUFS
    pub target_loudness: f32,
    /// Current integrated loudness
    integrated_loudness: f32,
    /// Running sum for loudness calculation
    sum: f64,
    /// Sample count
    sample_count: u64,
    /// Sample rate
    sample_rate: f32,
}

impl LoudnessNormalizer {
    /// Create new loudness normalizer
    pub fn new(sample_rate: f32) -> Self {
        Self {
            target_loudness: -16.0, // Standard for streaming
            integrated_loudness: -23.0,
            sum: 0.0,
            sample_count: 0,
            sample_rate,
        }
    }

    /// Process sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Update running sum for loudness estimation
        self.sum += (input as f64) * (input as f64);
        self.sample_count += 1;

        // Periodically update loudness estimate
        if self.sample_count % (self.sample_rate as u64 * 5) == 0 {
            self.update_loudness();
        }

        // Apply gain
        let loudness_diff = self.target_loudness - self.integrated_loudness;
        let gain = 10.0_f32.powf(loudness_diff / 20.0);
        input * gain
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Update loudness estimate
    fn update_loudness(&mut self) {
        if self.sample_count > 0 {
            let mean_square = self.sum / self.sample_count as f64;
            if mean_square > 0.0 {
                // Simplified loudness calculation (real EBU R128 is more complex)
                self.integrated_loudness = -0.691 + 10.0 * mean_square.log10() as f32;
            }
        }
    }

    /// Set target loudness
    pub fn set_target(&mut self, target: f32) {
        self.target_loudness = target.clamp(-30.0, -10.0);
    }

    /// Reset normalizer
    pub fn reset(&mut self) {
        self.sum = 0.0;
        self.sample_count = 0;
        self.integrated_loudness = -23.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equalizer_creation() {
        let eq = ParametricEqualizer::new(44100.0);
        assert_eq!(eq.num_bands(), 10);
    }

    #[test]
    fn test_equalizer_process() {
        let mut eq = ParametricEqualizer::new(44100.0);
        let output = eq.process(0.5);
        assert!(output.is_finite());
    }

    #[test]
    fn test_eq_preset() {
        let mut eq = ParametricEqualizer::new(44100.0);
        eq.apply_preset(&EqPreset::rock());
        assert_eq!(eq.current_preset(), Some("Rock"));
    }

    #[test]
    fn test_bass_enhancer() {
        let mut enhancer = BassEnhancer::new(44100.0);
        enhancer.set_gain(6.0);
        let output = enhancer.process(0.5);
        assert!(output.is_finite());
    }

    #[test]
    fn test_loudness_normalizer() {
        let mut normalizer = LoudnessNormalizer::new(44100.0);
        let output = normalizer.process(0.5);
        assert!(output.is_finite());
    }
}