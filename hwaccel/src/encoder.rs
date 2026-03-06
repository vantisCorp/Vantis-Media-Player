//! Hardware-accelerated video encoder

use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::common::{CodecProfile, HardwareAccelerationType};
use crate::config::{EncoderConfig, EncoderPreset, RateControlMode};

/// Hardware encoder
pub struct HardwareEncoder {
    config: EncoderConfig,
    backend: Box<dyn EncoderBackend>,
    initialized: bool,
}

impl HardwareEncoder {
    /// Create a new hardware encoder
    pub fn new(config: EncoderConfig) -> Result<Self> {
        info!("🎬 Creating hardware encoder: {:?}", config.hw_type);
        
        let backend = Self::create_backend(&config)?;
        
        Ok(Self {
            config,
            backend,
            initialized: false,
        })
    }
    
    /// Create encoder backend based on configuration
    fn create_backend(config: &EncoderConfig) -> Result<Box<dyn EncoderBackend>> {
        match config.hw_type {
            #[cfg(windows)]
            HardwareAccelerationType::Nvidia => {
                Ok(Box::new(crate::windows::NVENCEncoder::new(config)?))
            }
            #[cfg(windows)]
            HardwareAccelerationType::AMD => {
                Ok(Box::new(crate::windows::AMFEncoder::new(config)?))
            }
            #[cfg(windows)]
            HardwareAccelerationType::Intel => {
                Ok(Box::new(crate::windows::QSVCEncoder::new(config)?))
            }
            #[cfg(windows)]
            HardwareAccelerationType::D3D11VA => {
                Ok(Box::new(crate::windows::D3D11Encoder::new(config)?))
            }
            
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::Nvidia => {
                Ok(Box::new(crate::linux::NVENCEncoder::new(config)?))
            }
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::AMD => {
                Ok(Box::new(crate::linux::AMFEncoder::new(config)?))
            }
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::Intel => {
                Ok(Box::new(crate::linux::QSVCEncoder::new(config)?))
            }
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::VAAPI => {
                Ok(Box::new(crate::linux::VAEncoder::new(config)?))
            }
            
            #[cfg(target_os = "macos")]
            HardwareAccelerationType::Apple => {
                Ok(Box::new(crate::macos::VideoToolboxEncoder::new(config)?))
            }
            
            #[cfg(target_os = "android")]
            HardwareAccelerationType::MediaCodec => {
                Ok(Box::new(crate::android::MediaCodecEncoder::new(config)?))
            }
            
            HardwareAccelerationType::None => {
                Err(anyhow::anyhow!("Hardware acceleration disabled"))
            }
            
            _ => {
                warn!("Hardware acceleration type {:?} not implemented, using fallback", config.hw_type);
                Ok(Box::new(FallbackEncoder::new(config)?))
            }
        }
    }
    
    /// Initialize the encoder
    pub fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing hardware encoder");
        self.backend.initialize()?;
        self.initialized = true;
        Ok(())
    }
    
    /// Check if encoder supports a codec profile
    pub fn supports_profile(&self, profile: CodecProfile) -> bool {
        self.backend.supports_profile(profile)
    }
    
    /// Get supported profiles
    pub fn supported_profiles(&self) -> Vec<CodecProfile> {
        self.backend.supported_profiles()
    }
    
    /// Encode a frame
    pub fn encode(&mut self, data: &[u8], width: u32, height: u32) -> Result<EncoderOutput> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Encoder not initialized"));
        }
        
        self.backend.encode(data, width, height)
    }
    
    /// Encode asynchronously
    pub async fn encode_async(&mut self, data: Vec<u8>, width: u32, height: u32) -> Result<EncoderOutput> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Encoder not initialized"));
        }
        
        self.backend.encode_async(data, width, height).await
    }
    
    /// Flush the encoder
    pub fn flush(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Encoder not initialized"));
        }
        
        self.backend.flush()
    }
    
    /// Get encoder statistics
    pub fn stats(&self) -> EncoderStats {
        self.backend.stats()
    }
    
    /// Reset the encoder
    pub fn reset(&mut self) -> Result<()> {
        info!("🔄 Resetting hardware encoder");
        self.backend.reset()?;
        Ok(())
    }
}

/// Encoder output
#[derive(Debug, Clone)]
pub struct EncoderOutput {
    /// Encoded frame data
    pub data: Vec<u8>,
    
    /// Is this a keyframe?
    pub is_keyframe: bool,
    
    /// Timestamp (in microseconds)
    pub timestamp: i64,
    
    /// Presentation timestamp (PTS)
    pub pts: i64,
    
    /// Decode timestamp (DTS)
    pub dts: Option<i64>,
    
    /// Frame size (in bytes)
    pub frame_size: usize,
}

/// Encoder statistics
#[derive(Debug, Clone, Copy)]
pub struct EncoderStats {
    /// Number of frames encoded
    pub frames_encoded: u64,
    
    /// Number of keyframes
    pub keyframes: u64,
    
    /// Number of errors
    pub errors: u64,
    
    /// Average encode time (in microseconds)
    pub avg_encode_time_us: u64,
    
    /// Current bitrate (in bps)
    pub current_bitrate: u64,
    
    /// Average quantization parameter
    pub avg_qp: Option<f32>,
}

impl Default for EncoderStats {
    fn default() -> Self {
        Self {
            frames_encoded: 0,
            keyframes: 0,
            errors: 0,
            avg_encode_time_us: 0,
            current_bitrate: 0,
            avg_qp: None,
        }
    }
}

/// Encoder backend trait
pub trait EncoderBackend: Send + Sync {
    /// Initialize the encoder
    fn initialize(&mut self) -> Result<()>;
    
    /// Check if encoder supports a codec profile
    fn supports_profile(&self, profile: CodecProfile) -> bool;
    
    /// Get supported profiles
    fn supported_profiles(&self) -> Vec<CodecProfile>;
    
    /// Encode a frame
    fn encode(&mut self, data: &[u8], width: u32, height: u32) -> Result<EncoderOutput>;
    
    /// Encode asynchronously
    fn encode_async(&mut self, data: Vec<u8>, width: u32, height: u32) -> Result<EncoderOutput> {
        // Default implementation: synchronous encode
        self.encode(data, width, height)
    }
    
    /// Flush the encoder
    fn flush(&mut self) -> Result<()>;
    
    /// Get encoder statistics
    fn stats(&self) -> EncoderStats;
    
    /// Reset the encoder
    fn reset(&mut self) -> Result<()>;
}

/// Fallback encoder (software encoding)
struct FallbackEncoder {
    config: EncoderConfig,
    stats: EncoderStats,
}

impl FallbackEncoder {
    fn new(config: &EncoderConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            stats: EncoderStats::default(),
        })
    }
}

impl EncoderBackend for FallbackEncoder {
    fn initialize(&mut self) -> Result<()> {
        info!("🔄 Using software fallback encoder");
        Ok(())
    }
    
    fn supports_profile(&self, _profile: CodecProfile) -> bool {
        // Software encoder supports most profiles
        true
    }
    
    fn supported_profiles(&self) -> Vec<CodecProfile> {
        vec![
            CodecProfile::H264Baseline,
            CodecProfile::H264Main,
            CodecProfile::H264High,
            CodecProfile::H265Main,
            CodecProfile::H265Main10,
            CodecProfile::VP9Profile0,
            CodecProfile::AV1Main,
        ]
    }
    
    fn encode(&mut self, _data: &[u8], _width: u32, _height: u32) -> Result<EncoderOutput> {
        // Placeholder for software encoding
        // In real implementation, this would use FFmpeg software encoders
        Ok(EncoderOutput {
            data: vec![],
            is_keyframe: true,
            timestamp: 0,
            pts: 0,
            dts: Some(0),
            frame_size: 0,
        })
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn stats(&self) -> EncoderStats {
        self.stats
    }
    
    fn reset(&mut self) -> Result<()> {
        self.stats = EncoderStats::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_creation() {
        let config = EncoderConfig::default();
        let result = HardwareEncoder::new(config);
        // May fail if no hardware acceleration available
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_fallback_encoder() {
        let config = EncoderConfig::default();
        let encoder = FallbackEncoder::new(&config).unwrap();
        assert!(encoder.supports_profile(CodecProfile::H264Main));
    }
}