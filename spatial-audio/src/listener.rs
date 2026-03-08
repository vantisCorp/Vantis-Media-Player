//! Listener and audio source management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Position, Orientation, AttenuationModel};

/// The listener represents the player's position and orientation in 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    /// Unique identifier
    pub id: String,
    
    /// Position in 3D space
    position: Position,
    
    /// Orientation (rotation)
    orientation: Orientation,
    
    /// Head radius in meters (for HRTF)
    head_radius: f32,
    
    /// Inter-aural time difference scale
    itd_scale: f32,
    
    /// Enable Doppler effect
    doppler_enabled: bool,
    
    /// Speed of sound (m/s)
    speed_of_sound: f32,
}

impl Listener {
    pub fn new(position: impl Into<[f32; 3]>) -> Self {
        let [x, y, z] = position.into();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            position: Position::new(x, y, z),
            orientation: Orientation::identity(),
            head_radius: 0.0875, // Average human head radius
            itd_scale: 1.0,
            doppler_enabled: true,
            speed_of_sound: 343.0, // m/s at 20°C
        }
    }
    
    pub fn position(&self) -> &Position {
        &self.position
    }
    
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
    
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
    
    pub fn look_at(&mut self, target: Position) {
        self.orientation = Orientation::look_at(target);
    }
    
    pub fn head_radius(&self) -> f32 {
        self.head_radius
    }
    
    pub fn set_head_radius(&mut self, radius: f32) {
        self.head_radius = radius;
    }
    
    pub fn speed_of_sound(&self) -> f32 {
        self.speed_of_sound
    }
    
    pub fn set_speed_of_sound(&mut self, speed: f32) {
        self.speed_of_sound = speed;
    }
    
    /// Calculate Doppler shift for a moving source
    pub fn calculate_doppler_factor(&self, source_velocity: f32, listener_velocity: f32) -> f32 {
        if !self.doppler_enabled {
            return 1.0;
        }
        
        let v_rel = source_velocity - listener_velocity;
        self.speed_of_sound / (self.speed_of_sound + v_rel)
    }
}

impl Default for Listener {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0])
    }
}

/// Audio source in 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    /// Unique identifier
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Position in 3D space
    position: Position,
    
    /// Source velocity (for Doppler effect)
    velocity: Position,
    
    /// Base volume
    volume: f32,
    
    /// Minimum distance (full volume)
    min_distance: f32,
    
    /// Maximum distance (inaudible)
    max_distance: f32,
    
    /// Attenuation model
    attenuation_model: AttenuationModel,
    
    /// Whether the source is active
    active: bool,
    
    /// Whether the source loops
    looping: bool,
    
    /// Source radius (for extended sources)
    radius: f32,
    
    /// Cone parameters (for directional sources)
    cone: Option<SoundCone>,
    
    /// Reverb send level
    reverb_send: f32,
    
    /// Occlusion factor (0 = fully occluded, 1 = not occluded)
    occlusion: f32,
}

/// Sound cone for directional sources
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoundCone {
    /// Inner cone angle (degrees) - full volume
    pub inner_angle: f32,
    
    /// Outer cone angle (degrees) - minimum volume
    pub outer_angle: f32,
    
    /// Volume gain outside outer cone
    pub outer_gain: f32,
    
    /// Cone direction (normalized)
    pub direction: Position,
}

impl AudioSource {
    pub fn new(id: impl Into<String>, position: impl Into<[f32; 3]>) -> Self {
        let [x, y, z] = position.into();
        Self {
            id: id.into(),
            name: String::new(),
            position: Position::new(x, y, z),
            velocity: Position::origin(),
            volume: 1.0,
            min_distance: 1.0,
            max_distance: 100.0,
            attenuation_model: AttenuationModel::InverseDistance,
            active: true,
            looping: false,
            radius: 0.0,
            cone: None,
            reverb_send: 0.3,
            occlusion: 1.0,
        }
    }
    
    pub fn named(name: impl Into<String>, id: impl Into<String>, position: impl Into<[f32; 3]>) -> Self {
        let mut source = Self::new(id, position);
        source.name = name.into();
        source
    }
    
    pub fn position(&self) -> &Position {
        &self.position
    }
    
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    
    pub fn velocity(&self) -> &Position {
        &self.velocity
    }
    
    pub fn set_velocity(&mut self, velocity: Position) {
        self.velocity = velocity;
    }
    
    pub fn volume(&self) -> f32 {
        self.volume
    }
    
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 2.0);
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    
    pub fn min_distance(&self) -> f32 {
        self.min_distance
    }
    
    pub fn set_min_distance(&mut self, distance: f32) {
        self.min_distance = distance.max(0.1);
    }
    
    pub fn max_distance(&self) -> f32 {
        self.max_distance
    }
    
    pub fn set_max_distance(&mut self, distance: f32) {
        self.max_distance = distance.max(self.min_distance);
    }
    
    pub fn set_distance_range(&mut self, min: f32, max: f32) {
        self.min_distance = min.max(0.1);
        self.max_distance = max.max(self.min_distance);
    }
    
    pub fn attenuation_model(&self) -> AttenuationModel {
        self.attenuation_model
    }
    
    pub fn set_attenuation_model(&mut self, model: AttenuationModel) {
        self.attenuation_model = model;
    }
    
    pub fn reverb_send(&self) -> f32 {
        self.reverb_send
    }
    
    pub fn set_reverb_send(&mut self, level: f32) {
        self.reverb_send = level.clamp(0.0, 1.0);
    }
    
    pub fn occlusion(&self) -> f32 {
        self.occlusion
    }
    
    pub fn set_occlusion(&mut self, factor: f32) {
        self.occlusion = factor.clamp(0.0, 1.0);
    }
    
    /// Calculate distance attenuation
    pub fn calculate_distance_attenuation(&self, distance: f32) -> f32 {
        let attenuation = self.attenuation_model.calculate(
            distance,
            self.min_distance,
            self.max_distance,
        );
        attenuation * self.volume * self.occlusion
    }
    
    /// Calculate cone attenuation for a listener position
    pub fn calculate_cone_attenuation(&self, listener_position: &Position) -> f32 {
        let cone = match &self.cone {
            Some(c) => c,
            None => return 1.0,
        };
        
        // Calculate angle to listener
        let to_listener = Position::new(
            listener_position.x - self.position.x,
            listener_position.y - self.position.y,
            listener_position.z - self.position.z,
        );
        
        let distance = (to_listener.x * to_listener.x + to_listener.y * to_listener.y + to_listener.z * to_listener.z).sqrt();
        if distance < 0.001 {
            return 1.0;
        }
        
        // Calculate dot product with cone direction
        let cos_angle = (to_listener.x * cone.direction.x + to_listener.y * cone.direction.y + to_listener.z * cone.direction.z) / distance;
        let angle = cos_angle.acos().to_degrees();
        
        if angle <= cone.inner_angle {
            1.0
        } else if angle >= cone.outer_angle {
            cone.outer_gain
        } else {
            // Linear interpolation between inner and outer
            let t = (angle - cone.inner_angle) / (cone.outer_angle - cone.inner_angle);
            1.0 - t * (1.0 - cone.outer_gain)
        }
    }
}

impl Default for AudioSource {
    fn default() -> Self {
        Self::new("default", [0.0, 0.0, -1.0])
    }
}

/// Audio scene containing listener and sources
#[derive(Debug, Clone)]
pub struct AudioScene {
    listener: Listener,
    sources: HashMap<String, AudioSource>,
    max_sources: usize,
}

impl AudioScene {
    pub fn new() -> Self {
        Self {
            listener: Listener::default(),
            sources: HashMap::new(),
            max_sources: 32,
        }
    }
    
    pub fn with_max_sources(max: usize) -> Self {
        Self {
            listener: Listener::default(),
            sources: HashMap::new(),
            max_sources: max,
        }
    }
    
    pub fn listener(&self) -> &Listener {
        &self.listener
    }
    
    pub fn listener_mut(&mut self) -> &mut Listener {
        &mut self.listener
    }
    
    pub fn set_listener(&mut self, listener: Listener) {
        self.listener = listener;
    }
    
    pub fn add_source(&mut self, source: AudioSource) {
        if self.sources.len() < self.max_sources {
            self.sources.insert(source.id.clone(), source);
        }
    }
    
    pub fn remove_source(&mut self, id: &str) -> Option<AudioSource> {
        self.sources.remove(id)
    }
    
    pub fn get_source(&self, id: &str) -> Option<&AudioSource> {
        self.sources.get(id)
    }
    
    pub fn get_source_mut(&mut self, id: &str) -> Option<&mut AudioSource> {
        self.sources.get_mut(id)
    }
    
    pub fn sources(&self) -> impl Iterator<Item = &AudioSource> {
        self.sources.values()
    }
    
    pub fn sources_mut(&mut self) -> impl Iterator<Item = &mut AudioSource> {
        self.sources.values_mut()
    }
    
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
    
    pub fn clear_sources(&mut self) {
        self.sources.clear();
    }
    
    /// Update all source positions based on velocity (for Doppler)
    pub fn update(&mut self, delta_time: f32) {
        for source in self.sources.values_mut() {
            // Update position based on velocity
            let new_pos = Position::new(
                source.position.x + source.velocity.x * delta_time,
                source.position.y + source.velocity.y * delta_time,
                source.position.z + source.velocity.z * delta_time,
            );
            source.position = new_pos;
        }
    }
}

impl Default for AudioScene {
    fn default() -> Self {
        Self::new()
    }
}