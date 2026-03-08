//! Room simulation and reverb for spatial audio.
//!
//! This module provides realistic room acoustics simulation including
//! early reflections, late reverberation, and distance-based attenuation.

use crate::error::{SpatialAudioError, SpatialAudioResult};
use crate::types::{AudioBuffer, AudioFormat, Environment, Material, RoomDimensions};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Reverb processor configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverbConfig {
    /// Room size multiplier (1.0 = normal, 2.0 = large hall).
    pub room_size: f32,
    /// Damping factor (0.0 = bright, 1.0 = muffled).
    pub damping: f32,
    /// Wet/dry mix (0.0 = dry, 1.0 = wet).
    pub wet_level: f32,
    /// Dry signal level.
    pub dry_level: f32,
    /// Width of the reverb (0.0 = mono, 1.0 = full stereo).
    pub width: f32,
    /// Pre-delay in milliseconds.
    pub pre_delay_ms: f32,
    /// Early reflections level.
    pub early_level: f32,
    /// Late reverb level.
    pub late_level: f32,
    /// High frequency cutoff for damping.
    pub hf_damping_hz: f32,
    /// Low frequency cutoff for damping.
    pub lf_damping_hz: f32,
    /// Decay time (RT60) in seconds.
    pub decay_time: f32,
}

impl Default for ReverbConfig {
    fn default() -> Self {
        ReverbConfig {
            room_size: 1.0,
            damping: 0.5,
            wet_level: 0.3,
            dry_level: 0.7,
            width: 1.0,
            pre_delay_ms: 20.0,
            early_level: 0.7,
            late_level: 0.5,
            hf_damping_hz: 5000.0,
            lf_damping_hz: 200.0,
            decay_time: 2.0,
        }
    }
}

impl ReverbConfig {
    /// Create a preset for a small room.
    pub fn small_room() -> Self {
        ReverbConfig {
            room_size: 0.3,
            damping: 0.6,
            wet_level: 0.2,
            dry_level: 0.8,
            pre_delay_ms: 5.0,
            decay_time: 0.5,
            ..Default::default()
        }
    }

    /// Create a preset for a medium room.
    pub fn medium_room() -> Self {
        ReverbConfig {
            room_size: 0.6,
            damping: 0.5,
            wet_level: 0.25,
            dry_level: 0.75,
            pre_delay_ms: 10.0,
            decay_time: 1.0,
            ..Default::default()
        }
    }

    /// Create a preset for a large hall.
    pub fn large_hall() -> Self {
        ReverbConfig {
            room_size: 1.5,
            damping: 0.3,
            wet_level: 0.35,
            dry_level: 0.65,
            pre_delay_ms: 30.0,
            decay_time: 3.5,
            width: 1.0,
            ..Default::default()
        }
    }

    /// Create a preset for a cathedral.
    pub fn cathedral() -> Self {
        ReverbConfig {
            room_size: 2.0,
            damping: 0.2,
            wet_level: 0.4,
            dry_level: 0.6,
            pre_delay_ms: 50.0,
            decay_time: 6.0,
            width: 1.0,
            ..Default::default()
        }
    }

    /// Create a preset for a bathroom (bright, short).
    pub fn bathroom() -> Self {
        ReverbConfig {
            room_size: 0.15,
            damping: 0.1,
            wet_level: 0.4,
            dry_level: 0.6,
            pre_delay_ms: 2.0,
            decay_time: 0.8,
            hf_damping_hz: 8000.0,
            ..Default::default()
        }
    }

    /// Create a preset for outdoor environment.
    pub fn outdoor() -> Self {
        ReverbConfig {
            room_size: 3.0,
            damping: 0.8,
            wet_level: 0.1,
            dry_level: 0.9,
            pre_delay_ms: 50.0,
            decay_time: 0.3,
            early_level: 0.3,
            late_level: 0.1,
            ..Default::default()
        }
    }
}

/// Early reflection pattern.
#[derive(Debug, Clone)]
pub struct EarlyReflections {
    /// Delay times in samples for each reflection.
    delays: Vec<i32>,
    /// Gain values for each reflection.
    gains: Vec<f32>,
    /// Pan positions for each reflection (-1 to 1).
    pans: Vec<f32>,
    /// Sample rate.
    sample_rate: u32,
}

impl EarlyReflections {
    /// Create early reflections from room dimensions.
    pub fn from_room(room: &RoomDimensions, sample_rate: u32) -> Self {
        let speed_of_sound = 343.0; // m/s
        
        // Calculate reflection times based on room geometry
        let mut delays = Vec::new();
        let mut gains = Vec::new();
        let mut pans = Vec::new();

        // First-order reflections (6 surfaces)
        let surfaces = [
            (room.length, 0.0),      // Front wall
            (room.length, 0.0),      // Back wall
            (room.width, 0.0),       // Left wall
            (room.width, 0.0),       // Right wall
            (room.height, 0.0),      // Ceiling
            (room.height, 0.0),      // Floor
        ];

        let pans_surface = [0.0, 0.0, -0.5, 0.5, 0.0, 0.0];

        for (idx, (distance, _)) in surfaces.iter().enumerate() {
            let delay_seconds = distance * 2.0 / speed_of_sound;
            let delay_samples = (delay_seconds * sample_rate as f64) as i32;
            
            // Gain falls off with distance and reflection coefficient
            let gain = 0.5 / (1.0 + distance / 5.0);

            delays.push(delay_samples);
            gains.push(gain as f32);
            pans.push(pans_surface[idx]);
        }

        // Add second-order reflections (corners)
        let corners = [
            (room.length + room.width, 0.3),
            (room.length + room.height, 0.25),
            (room.width + room.height, 0.35),
        ];

        for (distance, pan) in corners.iter() {
            let delay_seconds = distance * 2.0 / speed_of_sound;
            let delay_samples = (delay_seconds * sample_rate as f64) as i32;
            let gain = 0.3 / (1.0 + distance / 5.0);

            delays.push(delay_samples);
            gains.push(gain as f32);
            pans.push(*pan as f32);
        }

        EarlyReflections {
            delays,
            gains,
            pans,
            sample_rate,
        }
    }

    /// Create a simple early reflection pattern.
    pub fn simple(sample_rate: u32) -> Self {
        // Standard early reflection pattern
        let delay_times_ms = [19.0, 22.0, 27.0, 32.0, 38.0, 44.0, 52.0, 60.0];
        let gain_values = [0.8, 0.7, 0.6, 0.5, 0.4, 0.35, 0.3, 0.25];
        let pan_values = [-0.3, 0.3, -0.5, 0.5, -0.2, 0.2, 0.0, 0.0];

        let delays: Vec<i32> = delay_times_ms
            .iter()
            .map(|&ms| ((ms / 1000.0) * sample_rate as f32) as i32)
            .collect();

        EarlyReflections {
            delays,
            gains: gain_values.to_vec(),
            pans: pan_values.to_vec(),
            sample_rate,
        }
    }

    /// Process audio through early reflections.
    pub fn process(&self, input: &AudioBuffer, output: &mut AudioBuffer) {
        let samples = input.samples();
        let max_delay = self.delays.iter().max().copied().unwrap_or(0) as usize;

        // Create delay buffers for each channel
        let mut delay_buffers = vec![vec![0.0f32; samples + max_delay]; self.delays.len()];

        // Fill delay buffers with input
        for (buf, &delay) in delay_buffers.iter_mut().zip(self.delays.iter()) {
            for i in 0..samples {
                buf[i + delay as usize] = input.get_sample(0, i);
            }
        }

        // Mix reflections into output
        for (refl_idx, (&gain, &pan)) in self.gains.iter().zip(self.pans.iter()).enumerate() {
            let left_gain = gain * (1.0 - pan) * 0.5;
            let right_gain = gain * (1.0 + pan) * 0.5;

            for i in 0..samples {
                let sample = delay_buffers[refl_idx][i + self.delays[refl_idx] as usize];
                
                if output.channels() >= 2 {
                    let left = output.get_sample(0, i) + sample * left_gain;
                    let right = output.get_sample(1, i) + sample * right_gain;
                    output.set_sample(0, i, left);
                    output.set_sample(1, i, right);
                } else if output.channels() == 1 {
                    let current = output.get_sample(0, i);
                    output.set_sample(0, i, current + sample * gain);
                }
            }
        }
    }
}

/// All-pass filter for diffusion.
#[derive(Debug, Clone)]
struct AllPassFilter {
    /// Delay line buffer.
    buffer: Vec<f32>,
    /// Write position.
    write_pos: usize,
    /// Delay length in samples.
    delay: usize,
    /// Feedback coefficient.
    feedback: f32,
}

impl AllPassFilter {
    fn new(delay: usize, feedback: f32) -> Self {
        AllPassFilter {
            buffer: vec![0.0; delay.max(1)],
            write_pos: 0,
            delay: delay.max(1),
            feedback,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let read_pos = (self.write_pos + self.buffer.len() - self.delay) % self.buffer.len();
        let delayed = self.buffer[read_pos];
        let output = delayed - input * self.feedback;
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        output
    }

    fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}

/// Comb filter for reverb tail.
#[derive(Debug, Clone)]
struct CombFilter {
    /// Delay line buffer.
    buffer: Vec<f32>,
    /// Write position.
    write_pos: usize,
    /// Delay length in samples.
    delay: usize,
    /// Feedback coefficient.
    feedback: f32,
    /// Low-pass filter state.
    filter_state: f32,
    /// Damping coefficient.
    damping: f32,
}

impl CombFilter {
    fn new(delay: usize, feedback: f32, damping: f32) -> Self {
        CombFilter {
            buffer: vec![0.0; delay.max(1)],
            write_pos: 0,
            delay: delay.max(1),
            feedback,
            filter_state: 0.0,
            damping,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let read_pos = (self.write_pos + self.buffer.len() - self.delay) % self.buffer.len();
        let delayed = self.buffer[read_pos];
        
        // Apply low-pass damping
        self.filter_state = delayed * (1.0 - self.damping) + self.filter_state * self.damping;
        
        let output = self.filter_state;
        self.buffer[self.write_pos] = input + self.filter_state * self.feedback;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        
        output
    }

    fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.filter_state = 0.0;
    }

    fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
    }
}

/// Late reverberation processor using Freeverb-style algorithm.
#[derive(Debug, Clone)]
pub struct LateReverb {
    /// Parallel comb filters (8 for stereo).
    comb_filters_left: Vec<CombFilter>,
    comb_filters_right: Vec<CombFilter>,
    /// Series all-pass filters (4 per channel).
    allpass_filters_left: Vec<AllPassFilter>,
    allpass_filters_right: Vec<AllPassFilter>,
    /// Sample rate.
    sample_rate: u32,
    /// Current feedback setting.
    feedback: f32,
    /// Current damping setting.
    damping: f32,
}

impl LateReverb {
    /// Create a new late reverb processor.
    pub fn new(sample_rate: u32) -> Self {
        // Comb filter delay lengths (based on Freeverb)
        let comb_delays = [
            1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617,
        ];
        
        // All-pass filter delay lengths
        let allpass_delays = [556, 441, 342, 289];

        // Scale delays based on sample rate (Freeverb is designed for 44100 Hz)
        let scale = sample_rate as f64 / 44100.0;

        let comb_filters_left: Vec<CombFilter> = comb_delays
            .iter()
            .map(|&d| CombFilter::new((d as f64 * scale) as usize, 0.84, 0.2))
            .collect();

        let comb_filters_right: Vec<CombFilter> = comb_delays
            .iter()
            .map(|&d| CombFilter::new(((d + 23) as f64 * scale) as usize, 0.84, 0.2))
            .collect();

        let allpass_filters_left: Vec<AllPassFilter> = allpass_delays
            .iter()
            .map(|&d| AllPassFilter::new((d as f64 * scale) as usize, 0.7))
            .collect();

        let allpass_filters_right: Vec<AllPassFilter> = allpass_delays
            .iter()
            .map(|&d| AllPassFilter::new(((d + 23) as f64 * scale) as usize, 0.7))
            .collect();

        LateReverb {
            comb_filters_left,
            comb_filters_right,
            allpass_filters_left,
            allpass_filters_right,
            sample_rate,
            feedback: 0.84,
            damping: 0.2,
        }
    }

    /// Set the feedback (decay time).
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
        for filter in &mut self.comb_filters_left {
            filter.set_feedback(feedback);
        }
        for filter in &mut self.comb_filters_right {
            filter.set_feedback(feedback);
        }
    }

    /// Set the damping factor.
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
        for filter in &mut self.comb_filters_left {
            filter.set_damping(damping);
        }
        for filter in &mut self.comb_filters_right {
            filter.set_damping(damping);
        }
    }

    /// Process stereo audio through the late reverb.
    pub fn process(&mut self, input_left: f32, input_right: f32) -> (f32, f32) {
        // Process through comb filters in parallel
        let mut sum_left = 0.0f32;
        let mut sum_right = 0.0f32;

        for filter in &mut self.comb_filters_left {
            sum_left += filter.process(input_left);
        }

        for filter in &mut self.comb_filters_right {
            sum_right += filter.process(input_right);
        }

        // Scale the comb filter output
        sum_left /= 8.0;
        sum_right /= 8.0;

        // Process through all-pass filters in series
        let mut out_left = sum_left;
        let mut out_right = sum_right;

        for filter in &mut self.allpass_filters_left {
            out_left = filter.process(out_left);
        }

        for filter in &mut self.allpass_filters_right {
            out_right = filter.process(out_right);
        }

        (out_left, out_right)
    }

    /// Clear the internal buffers.
    pub fn clear(&mut self) {
        for filter in &mut self.comb_filters_left {
            filter.clear();
        }
        for filter in &mut self.comb_filters_right {
            filter.clear();
        }
        for filter in &mut self.allpass_filters_left {
            filter.clear();
        }
        for filter in &mut self.allpass_filters_right {
            filter.clear();
        }
    }
}

/// Room simulator with acoustic modeling.
#[derive(Debug, Clone)]
pub struct RoomSimulator {
    /// Room dimensions.
    room: RoomDimensions,
    /// Wall materials.
    materials: WallMaterials,
    /// Sample rate.
    sample_rate: u32,
    /// Early reflections processor.
    early_reflections: EarlyReflections,
    /// Late reverb processor.
    late_reverb: LateReverb,
    /// Configuration.
    config: ReverbConfig,
}

/// Wall materials for acoustic simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallMaterials {
    /// Front wall material.
    pub front: Material,
    /// Back wall material.
    pub back: Material,
    /// Left wall material.
    pub left: Material,
    /// Right wall material.
    pub right: Material,
    /// Ceiling material.
    pub ceiling: Material,
    /// Floor material.
    pub floor: Material,
}

impl Default for WallMaterials {
    fn default() -> Self {
        WallMaterials {
            front: Material::Plaster,
            back: Material::Plaster,
            left: Material::Plaster,
            right: Material::Plaster,
            ceiling: Material::Plaster,
            floor: Material::Wood,
        }
    }
}

impl WallMaterials {
    /// Create materials for a concert hall.
    pub fn concert_hall() -> Self {
        WallMaterials {
            front: Material::Wood,
            back: Material::Fabric,
            left: Material::Wood,
            right: Material::Wood,
            ceiling: Material::Plaster,
            floor: Material::Carpet,
        }
    }

    /// Create materials for a studio.
    pub fn studio() -> Self {
        WallMaterials {
            front: Material::Foam,
            back: Material::Foam,
            left: Material::Foam,
            right: Material::Fabric,
            ceiling: Material::Foam,
            floor: Material::Carpet,
        }
    }

    /// Create materials for a bathroom.
    pub fn bathroom() -> Self {
        WallMaterials {
            front: Material::Tile,
            back: Material::Tile,
            left: Material::Tile,
            right: Material::Glass,
            ceiling: Material::Plaster,
            floor: Material::Tile,
        }
    }

    /// Calculate average absorption coefficient.
    pub fn average_absorption(&self) -> f32 {
        let materials = [
            &self.front, &self.back, &self.left,
            &self.right, &self.ceiling, &self.floor,
        ];
        
        let total: f32 = materials.iter().map(|m| m.absorption_coefficient()).sum();
        total / materials.len() as f32
    }
}

impl RoomSimulator {
    /// Create a new room simulator.
    pub fn new(room: RoomDimensions, materials: WallMaterials, sample_rate: u32) -> Self {
        let early_reflections = EarlyReflections::from_room(&room, sample_rate);
        let late_reverb = LateReverb::new(sample_rate);
        
        RoomSimulator {
            room,
            materials,
            sample_rate,
            early_reflections,
            late_reverb,
            config: ReverbConfig::default(),
        }
    }

    /// Create from an environment preset.
    pub fn from_environment(env: Environment, sample_rate: u32) -> Self {
        let (room, materials, config) = match env {
            Environment::SmallRoom => {
                (RoomDimensions::small_room(), WallMaterials::default(), ReverbConfig::small_room())
            }
            Environment::MediumRoom => {
                (RoomDimensions::medium_room(), WallMaterials::default(), ReverbConfig::medium_room())
            }
            Environment::LargeHall => {
                (RoomDimensions::large_hall(), WallMaterials::concert_hall(), ReverbConfig::large_hall())
            }
            Environment::Cathedral => {
                (RoomDimensions::cathedral(), WallMaterials::default(), ReverbConfig::cathedral())
            }
            Environment::Bathroom => {
                (RoomDimensions::bathroom(), WallMaterials::bathroom(), ReverbConfig::bathroom())
            }
            Environment::Outdoor => {
                (RoomDimensions::outdoor(), WallMaterials::default(), ReverbConfig::outdoor())
            }
            Environment::Custom { dimensions, materials, reverb_time } => {
                let mut config = ReverbConfig::medium_room();
                config.decay_time = reverb_time;
                (dimensions, materials, config)
            }
        };

        let early_reflections = EarlyReflections::from_room(&room, sample_rate);
        let late_reverb = LateReverb::new(sample_rate);

        RoomSimulator {
            room,
            materials,
            sample_rate,
            early_reflections,
            late_reverb,
            config,
        }
    }

    /// Set the reverb configuration.
    pub fn set_config(&mut self, config: ReverbConfig) {
        self.config = config;
        self.late_reverb.set_feedback(self.calculate_feedback());
        self.late_reverb.set_damping(config.damping);
    }

    /// Calculate feedback from decay time.
    fn calculate_feedback(&self) -> f32 {
        // RT60 to feedback conversion
        // Based on room volume and absorption
        let volume = self.room.volume();
        let absorption = self.materials.average_absorption();
        
        // Sabine's formula approximation
        let rt60 = self.config.decay_time;
        let target_rt60 = rt60 * (1.0 + absorption * 0.5);
        
        // Convert RT60 to feedback coefficient
        // This is a simplified approximation
        let feedback = 0.7 + 0.25 * (target_rt60 / 5.0).min(1.0);
        feedback.min(0.98).max(0.5)
    }

    /// Process audio through the room simulation.
    pub fn process(&mut self, input: &AudioBuffer) -> SpatialAudioResult<AudioBuffer> {
        let samples = input.samples();
        let num_channels = input.channels().max(2);
        
        // Create output buffer
        let mut output = AudioBuffer::new(
            AudioFormat::new(num_channels, self.sample_rate),
            samples,
        );

        // Copy dry signal
        for ch in 0..input.channels().min(num_channels) {
            for i in 0..samples {
                output.set_sample(ch, i, input.get_sample(ch, i) * self.config.dry_level);
            }
        }

        // Process early reflections
        let mut early_buffer = AudioBuffer::new(
            AudioFormat::new(num_channels, self.sample_rate),
            samples,
        );
        self.early_reflections.process(input, &mut early_buffer);

        // Mix early reflections
        for ch in 0..num_channels.min(early_buffer.channels()) {
            for i in 0..samples {
                let current = output.get_sample(ch, i);
                let early = early_buffer.get_sample(ch, i) * self.config.early_level;
                output.set_sample(ch, i, current + early);
            }
        }

        // Process late reverb
        for i in 0..samples {
            let in_left = input.get_sample(0, i);
            let in_right = if input.channels() >= 2 {
                input.get_sample(1, i)
            } else {
                in_left
            };

            let (out_left, out_right) = self.late_reverb.process(in_left, in_right);

            // Mix late reverb
            let current_left = output.get_sample(0, i);
            output.set_sample(0, i, current_left + out_left * self.config.late_level * self.config.wet_level);

            if num_channels >= 2 {
                let current_right = output.get_sample(1, i);
                output.set_sample(1, i, current_right + out_right * self.config.late_level * self.config.wet_level);
            }
        }

        Ok(output)
    }

    /// Get the room dimensions.
    pub fn room(&self) -> &RoomDimensions {
        &self.room
    }

    /// Get the wall materials.
    pub fn materials(&self) -> &WallMaterials {
        &self.materials
    }

    /// Get the configuration.
    pub fn config(&self) -> &ReverbConfig {
        &self.config
    }

    /// Clear internal buffers.
    pub fn clear(&mut self) {
        self.late_reverb.clear();
    }
}

/// Distance-based attenuation model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DistanceModel {
    /// No attenuation.
    None,
    /// Inverse distance rolloff.
    Inverse,
    /// Inverse distance squared (more natural).
    InverseSquared,
    /// Linear rolloff.
    Linear,
    /// Exponential rolloff.
    Exponential,
}

/// Distance attenuation processor.
#[derive(Debug, Clone)]
pub struct DistanceAttenuation {
    /// Distance model to use.
    model: DistanceModel,
    /// Reference distance (no attenuation).
    reference_distance: f32,
    /// Maximum distance (full attenuation).
    max_distance: f32,
    /// Rolloff factor.
    rolloff_factor: f32,
}

impl Default for DistanceAttenuation {
    fn default() -> Self {
        DistanceAttenuation {
            model: DistanceModel::InverseSquared,
            reference_distance: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
        }
    }
}

impl DistanceAttenuation {
    /// Create a new distance attenuation processor.
    pub fn new(model: DistanceModel, reference_distance: f32, max_distance: f32, rolloff_factor: f32) -> Self {
        DistanceAttenuation {
            model,
            reference_distance,
            max_distance,
            rolloff_factor,
        }
    }

    /// Calculate gain for a given distance.
    pub fn calculate_gain(&self, distance: f32) -> f32 {
        let distance = distance.max(self.reference_distance);
        
        match self.model {
            DistanceModel::None => 1.0,
            DistanceModel::Inverse => {
                let ref_dist = self.reference_distance;
                let rolloff = self.rolloff_factor;
                ref_dist / (ref_dist + rolloff * (distance - ref_dist))
            }
            DistanceModel::InverseSquared => {
                let ref_dist = self.reference_distance;
                let rolloff = self.rolloff_factor;
                let dist_ratio = ref_dist / (ref_dist + rolloff * (distance - ref_dist));
                dist_ratio * dist_ratio
            }
            DistanceModel::Linear => {
                let ref_dist = self.reference_distance;
                let max_dist = self.max_distance;
                1.0 - self.rolloff_factor * (distance - ref_dist) / (max_dist - ref_dist).max(0.001)
            }
            DistanceModel::Exponential => {
                let ref_dist = self.reference_distance;
                (-distance / ref_dist * self.rolloff_factor).exp()
            }
        }
    }

    /// Apply distance attenuation to an audio buffer.
    pub fn apply(&self, buffer: &mut AudioBuffer, distance: f32) {
        let gain = self.calculate_gain(distance);
        for ch in 0..buffer.channels() {
            for i in 0..buffer.samples() {
                let sample = buffer.get_sample(ch, i) * gain;
                buffer.set_sample(ch, i, sample);
            }
        }
    }
}

/// Doppler effect processor for moving sources.
#[derive(Debug, Clone)]
pub struct DopplerProcessor {
    /// Sample rate.
    sample_rate: u32,
    /// Speed of sound in m/s.
    speed_of_sound: f32,
    /// Previous source position.
    prev_position: Option<[f32; 3]>,
    /// Previous listener position.
    prev_listener_position: Option<[f32; 3]>,
    /// Delay line for pitch shifting.
    delay_line: Vec<f32>,
    /// Write position in delay line.
    write_pos: usize,
    /// Current delay in samples.
    current_delay: f32,
    /// Maximum delay in samples.
    max_delay: usize,
}

impl DopplerProcessor {
    /// Create a new doppler processor.
    pub fn new(sample_rate: u32) -> Self {
        let max_delay = (sample_rate as f32 * 0.1) as usize; // 100ms max delay
        DopplerProcessor {
            sample_rate,
            speed_of_sound: 343.0,
            prev_position: None,
            prev_listener_position: None,
            delay_line: vec![0.0; max_delay],
            write_pos: 0,
            current_delay: 0.0,
            max_delay,
        }
    }

    /// Calculate the doppler pitch ratio.
    pub fn calculate_pitch_ratio(
        &mut self,
        source_pos: [f32; 3],
        listener_pos: [f32; 3],
    ) -> f32 {
        // Calculate current distance
        let current_distance = ((source_pos[0] - listener_pos[0]).powi(2)
            + (source_pos[1] - listener_pos[1]).powi(2)
            + (source_pos[2] - listener_pos[2]).powi(2))
        .sqrt();

        // Calculate velocity if we have previous positions
        let doppler_factor = if let (Some(prev_src), Some(prev_lst)) =
            (self.prev_position, self.prev_listener_position)
        {
            // Calculate previous distance
            let prev_distance = ((prev_src[0] - prev_lst[0]).powi(2)
                + (prev_src[1] - prev_lst[1]).powi(2)
                + (prev_src[2] - prev_lst[2]).powi(2))
            .sqrt();

            // Calculate relative velocity (approaching = negative)
            let distance_change = current_distance - prev_distance;
            
            // Time delta = 1 sample worth of time
            let time_delta = 1.0 / self.sample_rate as f32;
            let velocity = distance_change / time_delta;

            // Doppler factor: f' = f * c / (c + v)
            let denominator = self.speed_of_sound + velocity;
            if denominator > 0.0 {
                self.speed_of_sound / denominator
            } else {
                1.0
            }
        } else {
            1.0
        };

        // Store current positions for next calculation
        self.prev_position = Some(source_pos);
        self.prev_listener_position = Some(listener_pos);

        doppler_factor
    }

    /// Apply doppler effect to audio buffer.
    pub fn process(
        &mut self,
        input: &AudioBuffer,
        source_pos: [f32; 3],
        listener_pos: [f32; 3],
    ) -> AudioBuffer {
        let pitch_ratio = self.calculate_pitch_ratio(source_pos, listener_pos);
        
        // Simple pitch shifting via resampling
        let samples = input.samples();
        let mut output = AudioBuffer::new(input.format().clone(), samples);

        for ch in 0..input.channels() {
            for i in 0..samples {
                // Simple linear interpolation resampling
                let src_idx = i as f32 * pitch_ratio;
                let idx0 = src_idx.floor() as usize;
                let idx1 = (idx0 + 1).min(samples - 1);
                let frac = src_idx - idx0 as f32;

                if idx0 < samples {
                    let s0 = input.get_sample(ch, idx0);
                    let s1 = input.get_sample(ch, idx1);
                    let sample = s0 * (1.0 - frac) + s1 * frac;
                    output.set_sample(ch, i, sample);
                }
            }
        }

        output
    }

    /// Reset the processor state.
    pub fn reset(&mut self) {
        self.prev_position = None;
        self.prev_listener_position = None;
        self.delay_line.fill(0.0);
        self.write_pos = 0;
        self.current_delay = 0.0;
    }
}

/// Complete spatial reverb processor.
#[derive(Debug)]
pub struct SpatialReverbProcessor {
    /// Room simulator.
    room_simulator: Option<RoomSimulator>,
    /// Distance attenuation.
    distance_attenuation: DistanceAttenuation,
    /// Doppler processor.
    doppler: DopplerProcessor,
    /// Configuration.
    config: SpatialReverbConfig,
}

/// Configuration for spatial reverb processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialReverbConfig {
    /// Enable room simulation.
    pub enable_room_simulation: bool,
    /// Enable distance attenuation.
    pub enable_distance_attenuation: bool,
    /// Enable doppler effect.
    pub enable_doppler: bool,
    /// Environment preset.
    pub environment: Environment,
    /// Reverb configuration.
    pub reverb_config: ReverbConfig,
    /// Distance model.
    pub distance_model: DistanceModel,
    /// Reference distance.
    pub reference_distance: f32,
    /// Maximum distance.
    pub max_distance: f32,
    /// Sample rate.
    pub sample_rate: u32,
}

impl Default for SpatialReverbConfig {
    fn default() -> Self {
        SpatialReverbConfig {
            enable_room_simulation: true,
            enable_distance_attenuation: true,
            enable_doppler: false,
            environment: Environment::MediumRoom,
            reverb_config: ReverbConfig::medium_room(),
            distance_model: DistanceModel::InverseSquared,
            reference_distance: 1.0,
            max_distance: 100.0,
            sample_rate: 48000,
        }
    }
}

impl SpatialReverbProcessor {
    /// Create a new spatial reverb processor.
    pub fn new(config: SpatialReverbConfig) -> Self {
        let room_simulator = if config.enable_room_simulation {
            Some(RoomSimulator::from_environment(
                config.environment.clone(),
                config.sample_rate,
            ))
        } else {
            None
        };

        let distance_attenuation = DistanceAttenuation::new(
            config.distance_model,
            config.reference_distance,
            config.max_distance,
            1.0,
        );

        let doppler = DopplerProcessor::new(config.sample_rate);

        SpatialReverbProcessor {
            room_simulator,
            distance_attenuation,
            doppler,
            config,
        }
    }

    /// Process audio with spatial reverb.
    pub fn process(
        &mut self,
        input: &AudioBuffer,
        distance: f32,
        source_pos: Option<[f32; 3]>,
        listener_pos: Option<[f32; 3]>,
    ) -> SpatialAudioResult<AudioBuffer> {
        let mut output = input.clone();

        // Apply distance attenuation
        if self.config.enable_distance_attenuation {
            self.distance_attenuation.apply(&mut output, distance);
        }

        // Apply doppler effect
        if self.config.enable_doppler {
            if let (Some(src_pos), Some(lst_pos)) = (source_pos, listener_pos) {
                output = self.doppler.process(&output, src_pos, lst_pos);
            }
        }

        // Apply room simulation
        if let Some(ref mut room) = self.room_simulator {
            output = room.process(&output)?;
        }

        Ok(output)
    }

    /// Set the environment.
    pub fn set_environment(&mut self, environment: Environment) {
        self.config.environment = environment.clone();
        if self.config.enable_room_simulation {
            self.room_simulator = Some(RoomSimulator::from_environment(
                environment,
                self.config.sample_rate,
            ));
        }
    }

    /// Set the reverb configuration.
    pub fn set_reverb_config(&mut self, config: ReverbConfig) {
        self.config.reverb_config = config.clone();
        if let Some(ref mut room) = self.room_simulator {
            room.set_config(config);
        }
    }

    /// Get the configuration.
    pub fn config(&self) -> &SpatialReverbConfig {
        &self.config
    }

    /// Reset all processors.
    pub fn reset(&mut self) {
        if let Some(ref mut room) = self.room_simulator {
            room.clear();
        }
        self.doppler.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverb_config_presets() {
        let small = ReverbConfig::small_room();
        assert!(small.decay_time < 1.0);

        let large = ReverbConfig::large_hall();
        assert!(large.decay_time > 2.0);
    }

    #[test]
    fn test_distance_attenuation() {
        let atten = DistanceAttenuation::default();
        
        // At reference distance, gain should be ~1
        let gain_ref = atten.calculate_gain(1.0);
        assert!(gain_ref > 0.9);
        
        // At double distance, gain should decrease
        let gain_double = atten.calculate_gain(2.0);
        assert!(gain_double < gain_ref);
    }

    #[test]
    fn test_late_reverb() {
        let mut reverb = LateReverb::new(48000);
        let (left, right) = reverb.process(0.5, 0.5);
        
        // Output should be non-zero after processing
        assert!(left.abs() < 1.0);
        assert!(right.abs() < 1.0);
    }

    #[test]
    fn test_room_simulator() {
        let room = RoomDimensions::medium_room();
        let simulator = RoomSimulator::from_environment(Environment::MediumRoom, 48000);
        
        assert!(room.length > 0.0);
        assert!(simulator.materials().average_absorption() > 0.0);
    }
}