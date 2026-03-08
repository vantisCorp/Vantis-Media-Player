//! Core types for spatial audio processing

use serde::{Deserialize, Serialize};

/// Audio sample type
pub type Sample = f32;

/// Position in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    
    pub fn x(&self) -> f32 {
        self.x
    }
    
    pub fn y(&self) -> f32 {
        self.y
    }
    
    pub fn z(&self) -> f32 {
        self.z
    }
    
    /// Calculate distance to another position
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// Calculate relative position from this point
    pub fn relative_to(&self, origin: &Position) -> RelativePosition {
        let dx = self.x - origin.x;
        let dy = self.y - origin.y;
        let dz = self.z - origin.z;
        
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        let azimuth = dy.atan2(dx).to_degrees();
        let elevation = if distance > 0.0 {
            (dz / distance).asin().to_degrees()
        } else {
            0.0
        };
        
        RelativePosition {
            x: dx,
            y: dy,
            z: dz,
            distance,
            azimuth,
            elevation,
        }
    }
    
    /// Create position from spherical coordinates
    pub fn from_spherical(distance: f32, azimuth: f32, elevation: f32) -> Self {
        let azimuth_rad = azimuth.to_radians();
        let elevation_rad = elevation.to_radians();
        
        let cos_elev = elevation_rad.cos();
        let x = distance * azimuth_rad.cos() * cos_elev;
        let y = distance * azimuth_rad.sin() * cos_elev;
        let z = distance * elevation_rad.sin();
        
        Self::new(x, y, z)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::origin()
    }
}

/// Relative position from listener's perspective
#[derive(Debug, Clone, Copy)]
pub struct RelativePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub distance: f32,
    pub azimuth: f32,     // Horizontal angle in degrees (-180 to 180)
    pub elevation: f32,   // Vertical angle in degrees (-90 to 90)
}

impl RelativePosition {
    pub fn azimuth(&self) -> f32 {
        self.azimuth
    }
    
    pub fn elevation(&self) -> f32 {
        self.elevation
    }
    
    pub fn distance(&self) -> f32 {
        self.distance
    }
}

/// Orientation in 3D space using quaternion
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orientation {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Orientation {
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }
    
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
        let (sp, cp) = (pitch / 2.0).sin_cos();
        let (sy, cy) = (yaw / 2.0).sin_cos();
        let (sr, cr) = (roll / 2.0).sin_cos();
        
        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }
    
    pub fn look_at(direction: Position) -> Self {
        // Calculate yaw and pitch from direction
        let yaw = direction.y.atan2(direction.x).to_degrees();
        let pitch = (-direction.z).atan2((direction.x * direction.x + direction.y * direction.y).sqrt()).to_degrees();
        Self::from_euler(pitch.to_radians(), yaw.to_radians(), 0.0)
    }
    
    /// Rotate a position by this orientation
    pub fn rotate_position(&self, pos: &Position) -> Position {
        // Quaternion rotation: v' = q * v * q^-1
        let qv = Orientation {
            w: 0.0,
            x: pos.x,
            y: pos.y,
            z: pos.z,
        };
        
        let conj = Orientation {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
        
        // q * v
        let qv_mul = Orientation {
            w: self.w * qv.w - self.x * qv.x - self.y * qv.y - self.z * qv.z,
            x: self.w * qv.x + self.x * qv.w + self.y * qv.z - self.z * qv.y,
            y: self.w * qv.y - self.x * qv.z + self.y * qv.w + self.z * qv.x,
            z: self.w * qv.z + self.x * qv.y - self.y * qv.x + self.z * qv.w,
        };
        
        // (q * v) * q^-1
        Position::new(
            qv_mul.w * conj.x + qv_mul.x * conj.w + qv_mul.y * conj.z - qv_mul.z * conj.y,
            qv_mul.w * conj.y - qv_mul.x * conj.z + qv_mul.y * conj.w + qv_mul.z * conj.x,
            qv_mul.w * conj.z + qv_mul.x * conj.y - qv_mul.y * conj.x + qv_mul.z * conj.w,
        )
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::identity()
    }
}

/// Audio buffer for multi-channel audio
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    channels: Vec<Vec<Sample>>,
    sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(channels: usize, samples: usize) -> Self {
        Self {
            channels: vec![vec![0.0; samples]; channels],
            sample_rate: 48000,
        }
    }
    
    pub fn with_sample_rate(channels: usize, samples: usize, sample_rate: u32) -> Self {
        Self {
            channels: vec![vec![0.0; samples]; channels],
            sample_rate,
        }
    }
    
    pub fn from_interleaved(data: &[Sample], channels: usize, sample_rate: u32) -> Self {
        let samples_per_channel = data.len() / channels;
        let mut channel_data = vec![vec![0.0; samples_per_channel]; channels];
        
        for (i, &sample) in data.iter().enumerate() {
            let ch = i % channels;
            let idx = i / channels;
            if idx < samples_per_channel {
                channel_data[ch][idx] = sample;
            }
        }
        
        Self {
            channels: channel_data,
            sample_rate,
        }
    }
    
    pub fn to_interleaved(&self) -> Vec<Sample> {
        let total_samples = self.sample_count() * self.channel_count();
        let mut interleaved = vec![0.0; total_samples];
        
        for (ch, channel) in self.channels.iter().enumerate() {
            for (i, &sample) in channel.iter().enumerate() {
                interleaved[i * self.channel_count() + ch] = sample;
            }
        }
        
        interleaved
    }
    
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
    
    pub fn sample_count(&self) -> usize {
        self.channels.first().map(|c| c.len()).unwrap_or(0)
    }
    
    pub fn channel(&self, index: usize) -> &[Sample] {
        self.channels.get(index).map(|c| c.as_slice()).unwrap_or(&[])
    }
    
    pub fn channel_mut(&mut self, index: usize) -> &mut Vec<Sample> {
        static mut EMPTY: Vec<Sample> = Vec::new();
        self.channels.get_mut(index).unwrap_or(unsafe { &mut EMPTY })
    }
    
    pub fn channels(&self) -> &[Vec<Sample>] {
        &self.channels
    }
    
    pub fn channels_mut(&mut self) -> &mut [Vec<Sample>] {
        &mut self.channels
    }
    
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    pub fn duration_seconds(&self) -> f32 {
        self.sample_count() as f32 / self.sample_rate as f32
    }
    
    /// Apply gain to all channels
    pub fn apply_gain(&mut self, gain: f32) {
        for channel in &mut self.channels {
            for sample in channel.iter_mut() {
                *sample *= gain;
            }
        }
    }
    
    /// Mix another buffer into this one
    pub fn mix(&mut self, other: &AudioBuffer, gain: f32) {
        for (ch, channel) in self.channels.iter_mut().enumerate() {
            if let Some(other_channel) = other.channels.get(ch) {
                for (i, sample) in channel.iter_mut().enumerate() {
                    if i < other_channel.len() {
                        *sample += other_channel[i] * gain;
                    }
                }
            }
        }
    }
    
    /// Clear all samples
    pub fn clear(&mut self) {
        for channel in &mut self.channels {
            channel.fill(0.0);
        }
    }
}

/// Audio format specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u32,
    pub bits_per_sample: u32,
}

impl AudioFormat {
    pub fn new(sample_rate: u32, channels: u32, bits_per_sample: u32) -> Self {
        Self { sample_rate, channels, bits_per_sample }
    }
    
    pub fn cd_quality() -> Self {
        Self::new(44100, 2, 16)
    }
    
    pub fn dvd_quality() -> Self {
        Self::new(48000, 2, 16)
    }
    
    pub fn hd_quality() -> Self {
        Self::new(96000, 2, 24)
    }
    
    pub fn surround_5_1() -> Self {
        Self::new(48000, 6, 16)
    }
    
    pub fn surround_7_1() -> Self {
        Self::new(48000, 8, 16)
    }
    
    pub fn bytes_per_sample(&self) -> u32 {
        self.bits_per_sample / 8
    }
    
    pub fn byte_rate(&self) -> u32 {
        self.sample_rate * self.channels * self.bytes_per_sample()
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::dvd_quality()
    }
}

/// Spatial audio environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Room dimensions in meters
    pub dimensions: RoomDimensions,
    
    /// Wall materials
    pub wall_materials: Vec<Material>,
    
    /// Ambient temperature (affects speed of sound)
    pub temperature: f32,
    
    /// Air absorption factor
    pub air_absorption: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            dimensions: RoomDimensions::medium_room(),
            wall_materials: vec![Material::concrete(); 6],
            temperature: 20.0,
            air_absorption: 0.001,
        }
    }
}

/// Room dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RoomDimensions {
    pub width: f32,
    pub depth: f32,
    pub height: f32,
}

impl RoomDimensions {
    pub fn new(width: f32, depth: f32, height: f32) -> Self {
        Self { width, depth, height }
    }
    
    pub fn small_room() -> Self {
        Self::new(4.0, 4.0, 2.5)
    }
    
    pub fn medium_room() -> Self {
        Self::new(8.0, 6.0, 3.0)
    }
    
    pub fn large_room() -> Self {
        Self::new(20.0, 15.0, 5.0)
    }
    
    pub fn concert_hall() -> Self {
        Self::new(50.0, 30.0, 15.0)
    }
    
    pub fn cathedral() -> Self {
        Self::new(80.0, 40.0, 30.0)
    }
    
    pub fn volume(&self) -> f32 {
        self.width * self.depth * self.height
    }
    
    pub fn surface_area(&self) -> f32 {
        2.0 * (self.width * self.depth + self.width * self.height + self.depth * self.height)
    }
}

/// Material properties for acoustics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub absorption_coefficient: f32,
    pub scattering_coefficient: f32,
    pub transmission_coefficient: f32,
}

impl Material {
    pub fn new(name: &str, absorption: f32, scattering: f32, transmission: f32) -> Self {
        Self {
            name: name.to_string(),
            absorption_coefficient: absorption,
            scattering_coefficient: scattering,
            transmission_coefficient: transmission,
        }
    }
    
    pub fn concrete() -> Self {
        Self::new("concrete", 0.03, 0.1, 0.0)
    }
    
    pub fn wood() -> Self {
        Self::new("wood", 0.1, 0.2, 0.05)
    }
    
    pub fn carpet() -> Self {
        Self::new("carpet", 0.3, 0.5, 0.0)
    }
    
    pub fn glass() -> Self {
        Self::new("glass", 0.05, 0.05, 0.1)
    }
    
    pub fn fabric() -> Self {
        Self::new("fabric", 0.5, 0.8, 0.0)
    }
}