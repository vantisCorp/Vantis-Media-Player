//! Audio module for mobile

/// Audio configuration for mobile
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Sample rate
    pub sample_rate: u32,
    
    /// Number of channels
    pub channels: u32,
    
    /// Buffer size in frames
    pub buffer_size: u32,
    
    /// Enable audio effects
    pub effects_enabled: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            channels: 2,
            buffer_size: 1024,
            effects_enabled: false,
        }
    }
}