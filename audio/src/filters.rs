//! Audio Filters and Effects
//! 
//! Comprehensive audio filtering including:
//! - IIR filters (low-pass, high-pass, band-pass, notch)
//! - FIR filters
//! - Audio effects (reverb, echo, chorus, flanger, phaser)
//! - Dynamic range compression
//! - Noise reduction

use std::f32::consts::PI;

// ============================================================================
// Filter Types
// ============================================================================

/// Filter type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    AllPass,
    Peaking,
    LowShelf,
    HighShelf,
}

/// Filter coefficients for biquad filter
#[derive(Debug, Clone, Default)]
pub struct BiquadCoefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
}

impl BiquadCoefficients {
    /// Create low-pass filter coefficients
    pub fn low_pass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let omega = 2.0 * PI * cutoff / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create high-pass filter coefficients
    pub fn high_pass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let omega = 2.0 * PI * cutoff / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 + cos_omega) / 2.0;
        let b1 = -(1.0 + cos_omega);
        let b2 = (1.0 + cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create band-pass filter coefficients
    pub fn band_pass(sample_rate: f32, center: f32, q: f32) -> Self {
        let omega = 2.0 * PI * center / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create notch filter coefficients
    pub fn notch(sample_rate: f32, center: f32, q: f32) -> Self {
        let omega = 2.0 * PI * center / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = 1.0;
        let b1 = -2.0 * cos_omega;
        let b2 = 1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create peaking filter coefficients
    pub fn peaking(sample_rate: f32, center: f32, q: f32, gain_db: f32) -> Self {
        let omega = 2.0 * PI * center / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_omega;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha / a;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create low-shelf filter coefficients
    pub fn low_shelf(sample_rate: f32, cutoff: f32, gain_db: f32) -> Self {
        let omega = 2.0 * PI * cutoff / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10.0_f32.powf(gain_db / 40.0);
        let beta = (a * a + 1.0).sqrt() / 2.0 - (a - 1.0) / 2.0;

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_omega + beta * sin_omega);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_omega - beta * sin_omega);
        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + beta * sin_omega;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) + (a - 1.0) * cos_omega - beta * sin_omega;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    /// Create high-shelf filter coefficients
    pub fn high_shelf(sample_rate: f32, cutoff: f32, gain_db: f32) -> Self {
        let omega = 2.0 * PI * cutoff / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10.0_f32.powf(gain_db / 40.0);
        let beta = (a * a + 1.0).sqrt() / 2.0 - (a - 1.0) / 2.0;

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_omega + beta * sin_omega);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_omega - beta * sin_omega);
        let a0 = (a + 1.0) - (a - 1.0) * cos_omega + beta * sin_omega;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) - (a - 1.0) * cos_omega - beta * sin_omega;

        Self::normalize(b0, b1, b2, a0, a1, a2)
    }

    fn normalize(b0: f32, b1: f32, b2: f32, a0: f32, a1: f32, a2: f32) -> Self {
        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a0: 1.0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }
}

/// Biquad filter state
#[derive(Debug, Clone, Default)]
pub struct BiquadState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

/// Biquad filter
#[derive(Debug, Clone)]
pub struct BiquadFilter {
    coeffs: BiquadCoefficients,
    state: BiquadState,
}

impl BiquadFilter {
    /// Create a new biquad filter
    pub fn new(coeffs: BiquadCoefficients) -> Self {
        Self {
            coeffs,
            state: BiquadState::default(),
        }
    }

    /// Create low-pass filter
    pub fn low_pass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        Self::new(BiquadCoefficients::low_pass(sample_rate, cutoff, q))
    }

    /// Create high-pass filter
    pub fn high_pass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        Self::new(BiquadCoefficients::high_pass(sample_rate, cutoff, q))
    }

    /// Create band-pass filter
    pub fn band_pass(sample_rate: f32, center: f32, q: f32) -> Self {
        Self::new(BiquadCoefficients::band_pass(sample_rate, center, q))
    }

    /// Create notch filter
    pub fn notch(sample_rate: f32, center: f32, q: f32) -> Self {
        Self::new(BiquadCoefficients::notch(sample_rate, center, q))
    }

    /// Create peaking filter
    pub fn peaking(sample_rate: f32, center: f32, q: f32, gain_db: f32) -> Self {
        Self::new(BiquadCoefficients::peaking(sample_rate, center, q, gain_db))
    }

    /// Process a single sample
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.coeffs.b0 * input
            + self.coeffs.b1 * self.state.x1
            + self.coeffs.b2 * self.state.x2
            - self.coeffs.a1 * self.state.y1
            - self.coeffs.a2 * self.state.y2;

        self.state.x2 = self.state.x1;
        self.state.x1 = input;
        self.state.y2 = self.state.y1;
        self.state.y1 = output;

        output
    }

    /// Process multiple samples
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.state = BiquadState::default();
    }

    /// Update coefficients
    pub fn update_coefficients(&mut self, coeffs: BiquadCoefficients) {
        self.coeffs = coeffs;
    }
}

// ============================================================================
// Audio Effects
// ============================================================================

/// Reverb effect
#[derive(Debug, Clone)]
pub struct Reverb {
    /// Room size (0.0 - 1.0)
    pub room_size: f32,
    /// Damping (0.0 - 1.0)
    pub damping: f32,
    /// Wet level (0.0 - 1.0)
    pub wet_level: f32,
    /// Dry level (0.0 - 1.0)
    pub dry_level: f32,
    /// Width (0.0 - 1.0)
    pub width: f32,
    // Delay lines
    delays: Vec<Vec<f32>>,
    delay_indices: Vec<usize>,
    delay_lengths: Vec<usize>,
}

impl Reverb {
    /// Create new reverb
    pub fn new(sample_rate: f32) -> Self {
        // Prime number delay lengths for natural reverb
        let delay_times_ms = [43, 67, 89, 127, 151, 179, 211, 257];
        
        let mut delays = Vec::new();
        let mut delay_indices = Vec::new();
        let mut delay_lengths = Vec::new();

        for &time_ms in &delay_times_ms {
            let length = (time_ms as f32 * sample_rate / 1000.0) as usize;
            delays.push(vec![0.0; length]);
            delay_indices.push(0);
            delay_lengths.push(length);
        }

        Self {
            room_size: 0.5,
            damping: 0.5,
            wet_level: 0.3,
            dry_level: 0.7,
            width: 1.0,
            delays,
            delay_indices,
            delay_lengths,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = 0.0;
        let feedback = self.room_size * 0.9;

        for (i, delay) in self.delays.iter_mut().enumerate() {
            let idx = self.delay_indices[i];
            let delayed = delay[idx];
            
            // Apply damping
            let damped = delayed * (1.0 - self.damping);
            
            // Mix with input
            delay[idx] = input + damped * feedback;
            
            // Accumulate output
            output += delayed;
            
            // Advance index
            self.delay_indices[i] = (idx + 1) % self.delay_lengths[i];
        }

        // Normalize by number of delay lines
        output /= self.delays.len() as f32;

        // Mix dry and wet
        self.dry_level * input + self.wet_level * output
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset reverb state
    pub fn reset(&mut self) {
        for delay in &mut self.delays {
            delay.fill(0.0);
        }
        for idx in &mut self.delay_indices {
            *idx = 0;
        }
    }
}

/// Echo/Delay effect
#[derive(Debug, Clone)]
pub struct Echo {
    /// Delay time in seconds
    pub delay_time: f32,
    /// Feedback (0.0 - 1.0)
    pub feedback: f32,
    /// Wet level (0.0 - 1.0)
    pub wet_level: f32,
    /// Dry level (0.0 - 1.0)
    pub dry_level: f32,
    /// Delay buffer
    buffer: Vec<f32>,
    write_idx: usize,
    delay_samples: usize,
}

impl Echo {
    /// Create new echo effect
    pub fn new(sample_rate: f32, delay_time: f32, feedback: f32) -> Self {
        let delay_samples = (delay_time * sample_rate) as usize;
        let buffer = vec![0.0; delay_samples + 1];

        Self {
            delay_time,
            feedback: feedback.clamp(0.0, 0.99),
            wet_level: 0.5,
            dry_level: 1.0,
            buffer,
            write_idx: 0,
            delay_samples,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.write_idx];
        
        // Write to buffer
        self.buffer[self.write_idx] = input + delayed * self.feedback;
        
        // Advance write index
        self.write_idx = (self.write_idx + 1) % self.delay_samples.max(1);
        
        // Mix dry and wet
        self.dry_level * input + self.wet_level * delayed
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Set delay time
    pub fn set_delay_time(&mut self, sample_rate: f32, delay_time: f32) {
        self.delay_time = delay_time;
        self.delay_samples = (delay_time * sample_rate) as usize;
        self.buffer.resize(self.delay_samples + 1, 0.0);
        self.write_idx = 0;
    }

    /// Reset echo state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_idx = 0;
    }
}

/// Chorus effect
#[derive(Debug, Clone)]
pub struct Chorus {
    /// Rate in Hz
    pub rate: f32,
    /// Depth (0.0 - 1.0)
    pub depth: f32,
    /// Wet level (0.0 - 1.0)
    pub wet_level: f32,
    /// Dry level (0.0 - 1.0)
    pub dry_level: f32,
    /// LFO phase
    phase: f32,
    /// Delay buffer
    buffer: Vec<f32>,
    write_idx: usize,
    max_delay: usize,
}

impl Chorus {
    /// Create new chorus effect
    pub fn new(sample_rate: f32) -> Self {
        let max_delay = (0.05 * sample_rate) as usize; // 50ms max delay
        let buffer = vec![0.0; max_delay + 1];

        Self {
            rate: 1.5,
            depth: 0.5,
            wet_level: 0.5,
            dry_level: 1.0,
            phase: 0.0,
            buffer,
            write_idx: 0,
            max_delay,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Write to buffer
        self.buffer[self.write_idx] = input;
        
        // Calculate modulated delay
        let lfo = (self.phase * 2.0 * PI).sin();
        let mod_delay = (self.depth * 0.02 * sample_rate * (1.0 + lfo) / 2.0) as usize;
        
        // Read from buffer
        let read_idx = if self.write_idx >= mod_delay {
            self.write_idx - mod_delay
        } else {
            self.max_delay - (mod_delay - self.write_idx)
        };
        
        let delayed = self.buffer[read_idx.min(self.max_delay)];
        
        // Advance indices
        self.write_idx = (self.write_idx + 1) % (self.max_delay + 1);
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        // Mix dry and wet
        self.dry_level * input + self.wet_level * delayed
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32], sample_rate: f32) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample, sample_rate);
        }
    }

    /// Reset chorus state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_idx = 0;
        self.phase = 0.0;
    }
}

/// Flanger effect
#[derive(Debug, Clone)]
pub struct Flanger {
    /// Rate in Hz
    pub rate: f32,
    /// Depth (0.0 - 1.0)
    pub depth: f32,
    /// Feedback (0.0 - 1.0)
    pub feedback: f32,
    /// Wet level (0.0 - 1.0)
    pub wet_level: f32,
    /// Dry level (0.0 - 1.0)
    pub dry_level: f32,
    /// LFO phase
    phase: f32,
    /// Delay buffer
    buffer: Vec<f32>,
    write_idx: usize,
    max_delay: usize,
}

impl Flanger {
    /// Create new flanger effect
    pub fn new(sample_rate: f32) -> Self {
        let max_delay = (0.01 * sample_rate) as usize; // 10ms max delay
        let buffer = vec![0.0; max_delay + 1];

        Self {
            rate: 0.5,
            depth: 0.5,
            feedback: 0.7,
            wet_level: 0.5,
            dry_level: 1.0,
            phase: 0.0,
            buffer,
            write_idx: 0,
            max_delay,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        // Calculate modulated delay
        let lfo = (self.phase * 2.0 * PI).sin();
        let mod_delay = (self.depth * 0.005 * sample_rate * (1.0 + lfo) / 2.0) as usize;
        
        // Read from buffer
        let read_idx = if self.write_idx >= mod_delay {
            self.write_idx - mod_delay
        } else {
            self.max_delay - (mod_delay - self.write_idx)
        };
        
        let delayed = self.buffer[read_idx.min(self.max_delay)];
        
        // Write to buffer with feedback
        self.buffer[self.write_idx] = input + delayed * self.feedback;
        
        // Advance indices
        self.write_idx = (self.write_idx + 1) % (self.max_delay + 1);
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        // Mix dry and wet
        self.dry_level * input + self.wet_level * delayed
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32], sample_rate: f32) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample, sample_rate);
        }
    }

    /// Reset flanger state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_idx = 0;
        self.phase = 0.0;
    }
}

// ============================================================================
// Dynamic Range Processing
// ============================================================================

/// Dynamic range compressor
#[derive(Debug, Clone)]
pub struct Compressor {
    /// Threshold in dB
    pub threshold: f32,
    /// Ratio (e.g., 4.0 = 4:1)
    pub ratio: f32,
    /// Attack time in seconds
    pub attack: f32,
    /// Release time in seconds
    pub release: f32,
    /// Knee width in dB
    pub knee: f32,
    /// Make-up gain in dB
    pub makeup_gain: f32,
    /// Current envelope
    envelope: f32,
    /// Sample rate
    sample_rate: f32,
}

impl Compressor {
    /// Create new compressor
    pub fn new(sample_rate: f32) -> Self {
        Self {
            threshold: -20.0,
            ratio: 4.0,
            attack: 0.003,
            release: 0.25,
            knee: 6.0,
            makeup_gain: 0.0,
            envelope: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Convert to dB
        let input_db = 20.0 * input.abs().log10().max(-60.0);
        
        // Calculate gain reduction
        let gain_reduction = if input_db > self.threshold + self.knee / 2.0 {
            // Above knee - full compression
            let over_db = input_db - self.threshold;
            over_db * (1.0 - 1.0 / self.ratio)
        } else if input_db > self.threshold - self.knee / 2.0 {
            // Within knee - soft transition
            let over_db = input_db - self.threshold + self.knee / 2.0;
            over_db * over_db / (2.0 * self.knee) * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        };
        
        // Smooth gain reduction with attack/release
        let target = -gain_reduction;
        let coeff = if target < self.envelope {
            (-1.0 / (self.attack * self.sample_rate)).exp()
        } else {
            (-1.0 / (self.release * self.sample_rate)).exp()
        };
        self.envelope = target + (self.envelope - target) * coeff;
        
        // Apply gain reduction
        let gain_db = self.envelope + self.makeup_gain;
        let gain = 10.0_f32.powf(gain_db / 20.0);
        
        input * gain
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset compressor state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}

/// Limiter
#[derive(Debug, Clone)]
pub struct Limiter {
    /// Threshold in dB
    pub threshold: f32,
    /// Release time in seconds
    pub release: f32,
    /// Current gain
    gain: f32,
    /// Sample rate
    sample_rate: f32,
}

impl Limiter {
    /// Create new limiter
    pub fn new(sample_rate: f32) -> Self {
        Self {
            threshold: -1.0,
            release: 0.1,
            gain: 1.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        let threshold_linear = 10.0_f32.powf(self.threshold / 20.0);
        
        // Calculate required gain
        let input_abs = input.abs();
        let required_gain = if input_abs > threshold_linear {
            threshold_linear / input_abs
        } else {
            1.0
        };
        
        // Smooth gain changes
        if required_gain < self.gain {
            self.gain = required_gain;
        } else {
            let release_coeff = (-1.0 / (self.release * self.sample_rate)).exp();
            self.gain = self.gain + (1.0 - self.gain) * (1.0 - release_coeff);
        }
        
        input * self.gain
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset limiter state
    pub fn reset(&mut self) {
        self.gain = 1.0;
    }
}

// ============================================================================
// Noise Reduction
// ============================================================================

/// Simple noise gate
#[derive(Debug, Clone)]
pub struct NoiseGate {
    /// Threshold in dB
    pub threshold: f32,
    /// Attack time in seconds
    pub attack: f32,
    /// Release time in seconds
    pub release: f32,
    /// Hold time in seconds
    pub hold: f32,
    /// Range in dB
    pub range: f32,
    /// Current envelope
    envelope: f32,
    /// Current gain
    gain: f32,
    /// Hold counter
    hold_counter: f32,
    /// Sample rate
    sample_rate: f32,
}

impl NoiseGate {
    /// Create new noise gate
    pub fn new(sample_rate: f32) -> Self {
        Self {
            threshold: -40.0,
            attack: 0.001,
            release: 0.1,
            hold: 0.05,
            range: -80.0,
            envelope: 0.0,
            gain: 0.0,
            hold_counter: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Track envelope
        let input_db = 20.0 * input.abs().log10().max(-60.0);
        
        let attack_coeff = (-1.0 / (self.attack * self.sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release * self.sample_rate)).exp();
        
        if input_db > self.envelope {
            self.envelope = input_db + (self.envelope - input_db) * attack_coeff;
        } else {
            self.envelope = input_db + (self.envelope - input_db) * release_coeff;
        }
        
        // Determine gate state
        if self.envelope >= self.threshold {
            self.hold_counter = self.hold * self.sample_rate;
        }
        
        // Calculate gain
        let target_gain = if self.hold_counter > 0.0 || self.envelope >= self.threshold {
            0.0 // Open gate (no reduction)
        } else {
            self.range // Closed gate
        };
        
        if self.hold_counter > 0.0 {
            self.hold_counter -= 1.0;
        }
        
        // Smooth gain changes
        let gain_coeff = if target_gain > self.gain {
            (-1.0 / (self.attack * self.sample_rate)).exp()
        } else {
            (-1.0 / (self.release * self.sample_rate)).exp()
        };
        self.gain = target_gain + (self.gain - target_gain) * gain_coeff;
        
        // Apply gain
        let gain_linear = 10.0_f32.powf(self.gain / 20.0);
        input * gain_linear
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Reset gate state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.gain = self.range;
        self.hold_counter = 0.0;
    }
}

// ============================================================================
// Pitch/Tempo Control
// ============================================================================

/// Simple pitch shifter using granular synthesis
#[derive(Debug, Clone)]
pub struct PitchShifter {
    /// Pitch shift in semitones
    pub shift: f32,
    /// Grain size in samples
    grain_size: usize,
    /// Overlap factor
    overlap: usize,
    /// Buffer for grains
    buffer: Vec<f32>,
    /// Write position
    write_pos: usize,
    /// Read position 1
    read_pos1: f32,
    /// Read position 2
    read_pos2: f32,
    /// Crossfade position
    crossfade: f32,
}

impl PitchShifter {
    /// Create new pitch shifter
    pub fn new(sample_rate: f32) -> Self {
        let grain_size = (0.05 * sample_rate) as usize; // 50ms grains
        let buffer = vec![0.0; grain_size * 4];

        Self {
            shift: 0.0,
            grain_size,
            overlap: 4,
            buffer,
            write_pos: 0,
            read_pos1: 0.0,
            read_pos2: 0.0,
            crossfade: 0.0,
        }
    }

    /// Process a single sample
    pub fn process(&mut self, input: f32) -> f32 {
        // Write to buffer
        let buf_len = self.buffer.len();
        self.buffer[self.write_pos % buf_len] = input;
        self.write_pos += 1;

        // Calculate read positions based on pitch shift
        let shift_factor = 2.0_f32.powf(self.shift / 12.0);
        
        // Read with pitch shift
        let grain_len = self.grain_size;
        let crossfade_len = grain_len / self.overlap;
        
        // Update crossfade
        self.crossfade += 1.0;
        if self.crossfade >= grain_len as f32 {
            self.crossfade = 0.0;
            self.read_pos1 = self.read_pos2;
            self.read_pos2 = self.write_pos as f32 - grain_len as f32;
        }

        // Calculate output with crossfade
        let idx1 = (self.read_pos1 as usize + self.crossfade as usize) % self.buffer.len();
        let idx2 = (self.read_pos2 as usize + self.crossfade as usize) % self.buffer.len();
        
        let sample1 = self.buffer[idx1];
        let sample2 = self.buffer[idx2];
        
        // Crossfade envelope
        let fade = self.crossfade / crossfade_len as f32;
        let fade1 = if self.crossfade < crossfade_len as f32 {
            fade
        } else if self.crossfade > (grain_len - crossfade_len) as f32 {
            1.0 - (self.crossfade - (grain_len - crossfade_len) as f32) / crossfade_len as f32
        } else {
            1.0
        };
        let fade2 = 1.0 - fade1;
        
        sample1 * fade1 + sample2 * fade2
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// Set pitch shift in semitones
    pub fn set_shift(&mut self, semitones: f32) {
        self.shift = semitones;
    }

    /// Reset pitch shifter state
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.read_pos1 = 0.0;
        self.read_pos2 = 0.0;
        self.crossfade = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_filter() {
        let mut filter = BiquadFilter::low_pass(44100.0, 1000.0, 0.707);
        let output = filter.process(1.0);
        assert!(output.is_finite());
    }

    #[test]
    fn test_compressor() {
        let mut compressor = Compressor::new(44100.0);
        let output = compressor.process(0.5);
        assert!(output.is_finite());
    }

    #[test]
    fn test_reverb() {
        let mut reverb = Reverb::new(44100.0);
        let output = reverb.process(0.5);
        assert!(output.is_finite());
    }

    #[test]
    fn test_echo() {
        let mut echo = Echo::new(44100.0, 0.5, 0.5);
        let output = echo.process(0.5);
        assert!(output.is_finite());
    }
}