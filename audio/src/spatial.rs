//! Spatial Audio Processing
//! 
//! This module provides spatial audio capabilities including:
//! - Stereo to surround upmixing
//! - Binaural audio for headphones
//! - HRTF (Head-Related Transfer Function) support
//! - Virtual surround sound
//! - 3D audio positioning

use std::f32::consts::PI;

// ============================================================================
// Channel Configuration
// ============================================================================

/// Audio channel layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    Mono,
    Stereo,
    Surround51,
    Surround71,
}

impl ChannelLayout {
    pub fn channels(&self) -> usize {
        match self {
            ChannelLayout::Mono => 1,
            ChannelLayout::Stereo => 2,
            ChannelLayout::Surround51 => 6,
            ChannelLayout::Surround71 => 8,
        }
    }
}

/// Speaker positions for surround sound
#[derive(Debug, Clone, Copy)]
pub struct SpeakerPosition {
    /// Azimuth angle in radians (-π to π)
    pub azimuth: f32,
    /// Elevation angle in radians (-π/2 to π/2)
    pub elevation: f32,
    /// Distance (normalized, 1.0 = reference)
    pub distance: f32,
}

impl SpeakerPosition {
    /// Front left speaker
    pub fn front_left() -> Self {
        Self { azimuth: -30.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }

    /// Front right speaker
    pub fn front_right() -> Self {
        Self { azimuth: 30.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }

    /// Front center speaker
    pub fn front_center() -> Self {
        Self { azimuth: 0.0, elevation: 0.0, distance: 1.0 }
    }

    /// Low frequency effects (subwoofer)
    pub fn lfe() -> Self {
        Self { azimuth: 0.0, elevation: 0.0, distance: 0.0 }
    }

    /// Side left speaker
    pub fn side_left() -> Self {
        Self { azimuth: -110.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }

    /// Side right speaker
    pub fn side_right() -> Self {
        Self { azimuth: 110.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }

    /// Rear left speaker
    pub fn rear_left() -> Self {
        Self { azimuth: -150.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }

    /// Rear right speaker
    pub fn rear_right() -> Self {
        Self { azimuth: 150.0 * PI / 180.0, elevation: 0.0, distance: 1.0 }
    }
}

// ============================================================================
// Stereo to Surround Upmixer
// ============================================================================

/// Stereo to 5.1 surround upmixer
#[derive(Debug, Clone)]
pub struct SurroundUpmixer {
    /// Enable center channel extraction
    pub center_extract: bool,
    /// Enable LFE channel
    pub enable_lfe: bool,
    /// LFE crossover frequency
    pub lfe_crossover: f32,
    /// Surround width (0.0 - 1.0)
    pub surround_width: f32,
    /// Low-pass filter for LFE
    lfe_filter: super::filters::BiquadFilter,
    /// Sample rate
    sample_rate: f32,
}

impl SurroundUpmixer {
    /// Create new upmixer
    pub fn new(sample_rate: f32) -> Self {
        let lfe_filter = super::filters::BiquadFilter::low_pass(sample_rate, 120.0, 0.7);

        Self {
            center_extract: true,
            enable_lfe: true,
            lfe_crossover: 120.0,
            surround_width: 0.5,
            lfe_filter,
            sample_rate,
        }
    }

    /// Upmix stereo to 5.1 surround
    pub fn process(&mut self, left: f32, right: f32) -> [f32; 6] {
        // Front left and right pass through
        let fl = left;
        let fr = right;

        // Extract center (common signal)
        let center = if self.center_extract {
            (left + right) * 0.5
        } else {
            0.0
        };

        // Extract surround (difference signal)
        let width = self.surround_width;
        let sl = (left - right) * width * 0.5;
        let sr = (right - left) * width * 0.5;

        // LFE from low-frequency content
        let lfe = if self.enable_lfe {
            self.lfe_filter.process(left + right)
        } else {
            0.0
        };

        [fl, fr, center, lfe, sl, sr]
    }

    /// Set surround width
    pub fn set_width(&mut self, width: f32) {
        self.surround_width = width.clamp(0.0, 1.0);
    }

    /// Set LFE crossover
    pub fn set_lfe_crossover(&mut self, freq: f32) {
        self.lfe_crossover = freq.max(60.0).min(200.0);
        self.lfe_filter = super::filters::BiquadFilter::low_pass(
            self.sample_rate, self.lfe_crossover, 0.7
        );
    }
}

// ============================================================================
// Virtual Surround (Surround to Stereo Downmix with HRTF)
// ============================================================================

/// Virtual surround processor using HRTF
#[derive(Debug, Clone)]
pub struct VirtualSurround {
    /// HRTF data
    hrtf: HRTFData,
    /// Enable virtual surround
    pub enabled: bool,
    /// Room size simulation
    pub room_size: f32,
}

impl VirtualSurround {
    /// Create new virtual surround processor
    pub fn new() -> Self {
        Self {
            hrtf: HRTFData::default(),
            enabled: true,
            room_size: 0.5,
        }
    }

    /// Process 5.1 surround to virtual stereo
    pub fn process_51(&self, channels: &[f32; 6]) -> [f32; 2] {
        if !self.enabled {
            // Simple downmix
            return [
                channels[0] + channels[2] * 0.707 + channels[4] * 0.707,
                channels[1] + channels[2] * 0.707 + channels[5] * 0.707,
            ];
        }

        // Apply HRTF for each speaker
        let mut left = 0.0;
        let mut right = 0.0;

        // Front left
        let (fl_l, fl_r) = self.hrtf.apply(channels[0], SpeakerPosition::front_left());
        left += fl_l;
        right += fl_r;

        // Front right
        let (fr_l, fr_r) = self.hrtf.apply(channels[1], SpeakerPosition::front_right());
        left += fr_l;
        right += fr_r;

        // Center
        let center = channels[2];
        left += center * 0.707;
        right += center * 0.707;

        // LFE - distribute to both channels
        let lfe = channels[3] * 0.5;
        left += lfe;
        right += lfe;

        // Surround left
        let (sl_l, sl_r) = self.hrtf.apply(channels[4], SpeakerPosition::side_left());
        left += sl_l;
        right += sl_r;

        // Surround right
        let (sr_l, sr_r) = self.hrtf.apply(channels[5], SpeakerPosition::side_right());
        left += sr_l;
        right += sr_r;

        [left, right]
    }

    /// Process 7.1 surround to virtual stereo
    pub fn process_71(&self, channels: &[f32; 8]) -> [f32; 2] {
        if !self.enabled {
            return [
                channels[0] + channels[2] * 0.707 + channels[4] * 0.707 + channels[6] * 0.707,
                channels[1] + channels[2] * 0.707 + channels[5] * 0.707 + channels[7] * 0.707,
            ];
        }

        // Process as 5.1 first
        let surround51 = [
            channels[0], channels[1], channels[2], channels[3],
            channels[4], channels[5],
        ];
        let mut result = self.process_51(&surround51);

        // Add rear channels
        let (rl_l, rl_r) = self.hrtf.apply(channels[6], SpeakerPosition::rear_left());
        let (rr_l, rr_r) = self.hrtf.apply(channels[7], SpeakerPosition::rear_right());
        result[0] += rl_l + rr_l;
        result[1] += rl_r + rr_r;

        result
    }
}

impl Default for VirtualSurround {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HRTF (Head-Related Transfer Function)
// ============================================================================

/// Simplified HRTF data
#[derive(Debug, Clone)]
pub struct HRTFData {
    /// ITD (Interaural Time Difference) scale
    itd_scale: f32,
    /// ILD (Interaural Level Difference) scale
    ild_scale: f32,
    /// Head radius in meters
    head_radius: f32,
}

impl Default for HRTFData {
    fn default() -> Self {
        Self {
            itd_scale: 1.0,
            ild_scale: 1.0,
            head_radius: 0.0875, // Average human head radius
        }
    }
}

impl HRTFData {
    /// Apply HRTF for a virtual source
    pub fn apply(&self, sample: f32, position: SpeakerPosition) -> (f32, f32) {
        let azimuth = position.azimuth;
        
        // Calculate ITD (Interaural Time Difference)
        // Simplified model: ITD = (head_radius / speed_of_sound) * (azimuth + sin(azimuth))
        let speed_of_sound = 343.0; // m/s
        let itd_seconds = (self.head_radius / speed_of_sound) 
            * (azimuth + azimuth.sin());
        
        // Calculate ILD (Interaural Level Difference)
        // Simplified: more attenuation on the opposite side
        let ild = (azimuth.cos() + 1.0) / 2.0; // 0 to 1
        
        // Apply HRTF effects
        let left_gain = if azimuth <= 0.0 {
            1.0
        } else {
            1.0 - azimuth.abs() / PI * self.ild_scale
        };
        
        let right_gain = if azimuth >= 0.0 {
            1.0
        } else {
            1.0 - azimuth.abs() / PI * self.ild_scale
        };

        (sample * left_gain, sample * right_gain)
    }
}

// ============================================================================
// Binaural Audio Processor
// ============================================================================

/// Binaural audio processor for 3D sound over headphones
#[derive(Debug, Clone)]
pub struct BinauralProcessor {
    /// HRTF data
    hrtf: HRTFData,
    /// Enable binaural processing
    pub enabled: bool,
    /// Crossfeed amount (for speaker simulation on headphones)
    pub crossfeed: f32,
    /// Delay buffer for left channel
    left_delay: Vec<f32>,
    /// Delay buffer for right channel
    right_delay: Vec<f32>,
    /// Delay read indices
    left_idx: usize,
    right_idx: usize,
}

impl BinauralProcessor {
    /// Create new binaural processor
    pub fn new(sample_rate: f32) -> Self {
        // Max delay for ITD (about 0.7ms for average head)
        let max_delay = (0.001 * sample_rate) as usize + 1;
        
        Self {
            hrtf: HRTFData::default(),
            enabled: true,
            crossfeed: 0.2,
            left_delay: vec![0.0; max_delay],
            right_delay: vec![0.0; max_delay],
            left_idx: 0,
            right_idx: 0,
        }
    }

    /// Process stereo with binaural enhancement
    pub fn process(&mut self, left: f32, right: f32) -> [f32; 2] {
        if !self.enabled {
            return [left, right];
        }

        // Apply crossfeed (simulates sound from one speaker reaching the opposite ear)
        let crossfeed_l = right * self.crossfeed;
        let crossfeed_r = left * self.crossfeed;

        let out_left = left + crossfeed_l;
        let out_right = right + crossfeed_r;

        [out_left, out_right]
    }

    /// Process buffer
    pub fn process_buffer(&mut self, samples: &mut [f32]) {
        for i in (0..samples.len()).step_by(2) {
            if i + 1 < samples.len() {
                let [l, r] = self.process(samples[i], samples[i + 1]);
                samples[i] = l;
                samples[i + 1] = r;
            }
        }
    }

    /// Set crossfeed amount
    pub fn set_crossfeed(&mut self, amount: f32) {
        self.crossfeed = amount.clamp(0.0, 0.5);
    }
}

// ============================================================================
// 3D Audio Source
// ============================================================================

/// 3D audio source position
#[derive(Debug, Clone, Copy)]
pub struct AudioSource3D {
    /// X position (-1 to 1, left to right)
    pub x: f32,
    /// Y position (-1 to 1, back to front)
    pub y: f32,
    /// Z position (-1 to 1, down to up)
    pub z: f32,
    /// Source gain
    pub gain: f32,
    /// Distance attenuation enabled
    pub distance_attenuation: bool,
}

impl Default for AudioSource3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            gain: 1.0,
            distance_attenuation: true,
        }
    }
}

impl AudioSource3D {
    /// Create source at position
    pub fn at(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z,
            gain: 1.0,
            distance_attenuation: true,
        }
    }

    /// Calculate distance from listener
    pub fn distance(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculate azimuth angle
    pub fn azimuth(&self) -> f32 {
        self.x.atan2(self.y)
    }

    /// Calculate elevation angle
    pub fn elevation(&self) -> f32 {
        self.z.atan2((self.x * self.x + self.y * self.y).sqrt())
    }
}

/// 3D audio panner
#[derive(Debug, Clone)]
pub struct AudioPanner3D {
    /// HRTF data
    hrtf: HRTFData,
    /// Listener position (usually at origin)
    listener_position: AudioSource3D,
    /// Minimum distance for attenuation
    min_distance: f32,
    /// Maximum distance for attenuation
    max_distance: f32,
    /// Rolloff factor
    rolloff_factor: f32,
}

impl AudioPanner3D {
    /// Create new 3D panner
    pub fn new() -> Self {
        Self {
            hrtf: HRTFData::default(),
            listener_position: AudioSource3D::default(),
            min_distance: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
        }
    }

    /// Pan mono source to stereo based on 3D position
    pub fn pan(&self, sample: f32, source: &AudioSource3D) -> [f32; 2] {
        // Calculate distance attenuation
        let distance = if source.distance_attenuation {
            source.distance().max(self.min_distance)
        } else {
            1.0
        };

        let attenuation = if source.distance_attenuation {
            self.min_distance / (self.min_distance + self.rolloff_factor * (distance - self.min_distance))
        } else {
            1.0
        };

        // Apply gain and attenuation
        let attenuated = sample * source.gain * attenuation;

        // Calculate pan based on azimuth
        let azimuth = source.azimuth();
        
        // Constant power panning
        let angle = azimuth * 0.5;
        let left = attenuated * ((PI / 4.0) - angle).cos();
        let right = attenuated * ((PI / 4.0) + angle).cos();

        [left, right]
    }

    /// Set distance parameters
    pub fn set_distance_params(&mut self, min: f32, max: f32, rolloff: f32) {
        self.min_distance = min.max(0.1);
        self.max_distance = max.max(min);
        self.rolloff_factor = rolloff.max(0.0);
    }
}

impl Default for AudioPanner3D {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Room Simulation
// ============================================================================

/// Room simulation for reverb and early reflections
#[derive(Debug, Clone)]
pub struct RoomSimulator {
    /// Room dimensions (width, depth, height) in meters
    pub dimensions: [f32; 3],
    /// Wall absorption coefficients (0-1, higher = more absorptive)
    pub absorption: [f32; 6],
    /// Reverb amount
    pub reverb_amount: f32,
    /// Early reflections
    early_reflections: Vec<EarlyReflection>,
    /// Sample rate
    sample_rate: f32,
}

/// Early reflection
#[derive(Debug, Clone)]
struct EarlyReflection {
    delay_samples: usize,
    gain: f32,
    channel: usize,
    buffer: Vec<f32>,
    write_idx: usize,
}

impl RoomSimulator {
    /// Create new room simulator
    pub fn new(sample_rate: f32) -> Self {
        let mut simulator = Self {
            dimensions: [10.0, 8.0, 3.0], // Default room size
            absorption: [0.5; 6],
            reverb_amount: 0.3,
            early_reflections: Vec::new(),
            sample_rate,
        };
        
        simulator.calculate_reflections();
        simulator
    }

    /// Calculate early reflections based on room geometry
    fn calculate_reflections(&mut self) {
        self.early_reflections.clear();
        
        let speed_of_sound = 343.0;
        
        // First-order reflections from each wall
        let wall_positions = [
            (0, self.dimensions[0]), // Left, Right
            (1, self.dimensions[1]), // Front, Back
            (2, self.dimensions[2]), // Floor, Ceiling
        ];

        for (axis, _) in wall_positions.iter().enumerate() {
            for side in 0..2 {
                // Distance to wall and back
                let distance = if side == 0 {
                    self.dimensions[*axis]
                } else {
                    self.dimensions[*axis]
                };
                
                let delay = (distance * 2.0 / speed_of_sound * self.sample_rate) as usize;
                let absorption = self.absorption[axis * 2 + side];
                let gain = (1.0 - absorption) * 0.5 * self.reverb_amount;
                
                self.early_reflections.push(EarlyReflection {
                    delay_samples: delay.max(1),
                    gain,
                    channel: 0,
                    buffer: vec![0.0; delay.max(1) + 1],
                    write_idx: 0,
                });
            }
        }
    }

    /// Process stereo with room simulation
    pub fn process(&mut self, left: f32, right: f32) -> [f32; 2] {
        let mut out_left = left;
        let mut out_right = right;

        // Add early reflections
        for reflection in &mut self.early_reflections {
            let delayed = reflection.buffer[reflection.write_idx];
            reflection.buffer[reflection.write_idx] = (left + right) * 0.5;
            reflection.write_idx = (reflection.write_idx + 1) % reflection.delay_samples;
            
            if reflection.channel % 2 == 0 {
                out_left += delayed * reflection.gain;
            } else {
                out_right += delayed * reflection.gain;
            }
        }

        [out_left, out_right]
    }

    /// Set room dimensions
    pub fn set_dimensions(&mut self, width: f32, depth: f32, height: f32) {
        self.dimensions = [width, depth, height];
        self.calculate_reflections();
    }

    /// Set reverb amount
    pub fn set_reverb_amount(&mut self, amount: f32) {
        self.reverb_amount = amount.clamp(0.0, 1.0);
        self.calculate_reflections();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surround_upmixer() {
        let mut upmixer = SurroundUpmixer::new(44100.0);
        let result = upmixer.process(0.5, 0.5);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_virtual_surround() {
        let vs = VirtualSurround::new();
        let input = [0.5, 0.5, 0.3, 0.1, 0.2, 0.2];
        let output = vs.process_51(&input);
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_binaural_processor() {
        let mut processor = BinauralProcessor::new(44100.0);
        let output = processor.process(0.5, 0.5);
        assert!(output[0].is_finite());
        assert!(output[1].is_finite());
    }

    #[test]
    fn test_audio_panner_3d() {
        let panner = AudioPanner3D::new();
        let source = AudioSource3D::at(-1.0, 1.0, 0.0); // Left
        let output = panner.pan(0.5, &source);
        assert!(output[0] > output[1]); // Should be louder on left
    }

    #[test]
    fn test_room_simulator() {
        let mut room = RoomSimulator::new(44100.0);
        let output = room.process(0.5, 0.5);
        assert!(output[0].is_finite());
        assert!(output[1].is_finite());
    }
}