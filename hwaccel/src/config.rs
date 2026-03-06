//! Hardware acceleration configuration

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::common::HardwareAccelerationType;

/// Hardware acceleration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HwAccelConfig {
    /// Preferred backend
    pub backend_preference: Option<HardwareAccelerationType>,
    
    /// Device selection strategy
    pub device_selection: DeviceSelection,
    
    /// Enable zero-copy if available
    pub enable_zero_copy: bool,
    
    /// Enable async processing if available
    pub enable_async: bool,
    
    /// Enable frame pool if available
    pub enable_frame_pool: bool,
    
    /// Fallback to software decoding
    pub fallback_to_software: bool,
    
    /// Maximum concurrent decode operations
    pub max_concurrent_decodes: usize,
    
    /// Maximum concurrent encode operations
    pub max_concurrent_encodes: usize,
    
    /// Memory pool size (in MB)
    pub memory_pool_size_mb: u64,
    
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for HwAccelConfig {
    fn default() -> Self {
        Self {
            backend_preference: None,
            device_selection: DeviceSelection::Auto,
            enable_zero_copy: true,
            enable_async: true,
            enable_frame_pool: true,
            fallback_to_software: true,
            max_concurrent_decodes: 2,
            max_concurrent_encodes: 1,
            memory_pool_size_mb: 512,
            enable_monitoring: true,
        }
    }
}

impl fmt::Display for HwAccelConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Hardware Acceleration Configuration:")?;
        writeln!(f, "  Backend preference: {:?}", self.backend_preference)?;
        writeln!(f, "  Device selection: {:?}", self.device_selection)?;
        writeln!(f, "  Zero-copy: {}", self.enable_zero_copy)?;
        writeln!(f, "  Async processing: {}", self.enable_async)?;
        writeln!(f, "  Frame pool: {}", self.enable_frame_pool)?;
        writeln!(f, "  Software fallback: {}", self.fallback_to_software)?;
        writeln!(f, "  Max concurrent decodes: {}", self.max_concurrent_decodes)?;
        writeln!(f, "  Max concurrent encodes: {}", self.max_concurrent_encodes)?;
        writeln!(f, "  Memory pool: {} MB", self.memory_pool_size_mb)?;
        writeln!(f, "  Monitoring: {}", self.enable_monitoring)
    }
}

/// Device selection strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceSelection {
    /// Automatically select the best device
    Auto,
    
    /// Select the device with most memory
    MostMemory,
    
    /// Select the device with highest performance
    HighestPerformance,
    
    /// Select the device with lowest power consumption
    LowestPower,
    
    /// Select a specific device by index
    SpecificDevice(usize),
    
    /// Select a device by name
    DeviceByName(String),
}

impl fmt::Display for DeviceSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceSelection::Auto => write!(f, "Auto"),
            DeviceSelection::MostMemory => write!(f, "Most Memory"),
            DeviceSelection::HighestPerformance => write!(f, "Highest Performance"),
            DeviceSelection::LowestPower => write!(f, "Lowest Power"),
            DeviceSelection::SpecificDevice(idx) => write!(f, "Device #{}", idx),
            DeviceSelection::DeviceByName(name) => write!(f, "Device: {}", name),
        }
    }
}

/// Decoder configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoderConfig {
    /// Hardware acceleration type
    pub hw_type: HardwareAccelerationType,
    
    /// Enable zero-copy
    pub enable_zero_copy: bool,
    
    /// Enable async processing
    pub enable_async: bool,
    
    /// Number of output buffers
    pub num_buffers: usize,
    
    /// Maximum frame dimensions
    pub max_width: u32,
    pub max_height: u32,
    
    /// Enable frame pool
    pub enable_frame_pool: bool,
    
    /// Fallback to software decoding
    pub fallback_to_software: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            hw_type: HardwareAccelerationType::None,
            enable_zero_copy: true,
            enable_async: true,
            num_buffers: 10,
            max_width: 4096,
            max_height: 4096,
            enable_frame_pool: true,
            fallback_to_software: true,
        }
    }
}

/// Encoder configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncoderConfig {
    /// Hardware acceleration type
    pub hw_type: HardwareAccelerationType,
    
    /// Codec profile
    pub profile: String,
    
    /// Preset (quality vs speed tradeoff)
    pub preset: EncoderPreset,
    
    /// Rate control mode
    pub rate_control: RateControlMode,
    
    /// Target bitrate (in bps)
    pub bitrate: u64,
    
    /// Maximum bitrate (in bps)
    pub max_bitrate: Option<u64>,
    
    /// Constant rate factor (CRF)
    pub crf: Option<u32>,
    
    /// Keyframe interval (in frames)
    pub keyframe_interval: u32,
    
    /// Enable async processing
    pub enable_async: bool,
    
    /// Enable multi-pass encoding
    pub enable_multipass: bool,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            hw_type: HardwareAccelerationType::None,
            profile: "high".to_string(),
            preset: EncoderPreset::Default,
            rate_control: RateControlMode::CBR,
            bitrate: 5_000_000, // 5 Mbps
            max_bitrate: None,
            crf: None,
            keyframe_interval: 250,
            enable_async: true,
            enable_multipass: false,
        }
    }
}

/// Encoder preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EncoderPreset {
    /// Fastest encoding, lowest quality
    UltraFast,
    /// Very fast encoding
    VeryFast,
    /// Faster encoding
    Faster,
    /// Fast encoding
    Fast,
    /// Default balanced preset
    Default,
    /// Slow encoding, better quality
    Slow,
    /// Slower encoding, even better quality
    Slower,
    /// Slowest encoding, best quality
    VerySlow,
}

impl fmt::Display for EncoderPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncoderPreset::UltraFast => write!(f, "UltraFast"),
            EncoderPreset::VeryFast => write!(f, "VeryFast"),
            EncoderPreset::Faster => write!(f, "Faster"),
            EncoderPreset::Fast => write!(f, "Fast"),
            EncoderPreset::Default => write!(f, "Default"),
            EncoderPreset::Slow => write!(f, "Slow"),
            EncoderPreset::Slower => write!(f, "Slower"),
            EncoderPreset::VerySlow => write!(f, "VerySlow"),
        }
    }
}

/// Rate control mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RateControlMode {
    /// Constant Bitrate
    CBR,
    /// Variable Bitrate
    VBR,
    /// Constant Rate Factor (quality-based)
    CRF,
    /// Constant Quantization Parameter
    CQP,
}

impl fmt::Display for RateControlMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateControlMode::CBR => write!(f, "CBR"),
            RateControlMode::VBR => write!(f, "VBR"),
            RateControlMode::CRF => write!(f, "CRF"),
            RateControlMode::CQP => write!(f, "CQP"),
        }
    }
}

/// Processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorConfig {
    /// Enable hardware scaling
    pub enable_scaling: bool,
    
    /// Enable hardware deinterlacing
    pub enable_deinterlacing: bool,
    
    /// Enable hardware color space conversion
    pub enable_colorspace_conversion: bool,
    
    /// Enable hardware tone mapping
    pub enable_tone_mapping: bool,
    
    /// Target format
    pub target_format: String,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            enable_scaling: true,
            enable_deinterlacing: true,
            enable_colorspace_conversion: true,
            enable_tone_mapping: true,
            target_format: "rgba".to_string(),
        }
    }
}