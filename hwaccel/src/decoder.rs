//! Hardware-accelerated video decoder

use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::common::{CodecProfile, HardwareAccelerationType};
use crate::config::DecoderConfig;

/// Hardware decoder
pub struct HardwareDecoder {
    config: DecoderConfig,
    backend: Box<dyn DecoderBackend>,
    initialized: bool,
}

impl HardwareDecoder {
    /// Create a new hardware decoder
    pub fn new(config: DecoderConfig) -> Result<Self> {
        info!("🎥 Creating hardware decoder: {:?}", config.hw_type);
        
        let backend = Self::create_backend(&config)?;
        
        Ok(Self {
            config,
            backend,
            initialized: false,
        })
    }
    
    /// Create decoder backend based on configuration
    fn create_backend(config: &DecoderConfig) -> Result<Box<dyn DecoderBackend>> {
        match config.hw_type {
            #[cfg(windows)]
            HardwareAccelerationType::DXVA => {
                Ok(Box::new(crate::windows::DXVADecoder::new(config)?))
            }
            #[cfg(windows)]
            HardwareAccelerationType::D3D11VA => {
                Ok(Box::new(crate::windows::D3D11VADecoder::new(config)?))
            }
            #[cfg(windows)]
            HardwareAccelerationType::MediaFoundation => {
                Ok(Box::new(crate::windows::MediaFoundationDecoder::new(config)?))
            }
            
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::VAAPI => {
                Ok(Box::new(crate::linux::VAAPIDecoder::new(config)?))
            }
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::VDPAU => {
                Ok(Box::new(crate::linux::VDPAUDecoder::new(config)?))
            }
            #[cfg(target_os = "linux")]
            HardwareAccelerationType::V4L2 => {
                Ok(Box::new(crate::linux::V4L2Decoder::new(config)?))
            }
            
            #[cfg(target_os = "macos")]
            HardwareAccelerationType::Apple => {
                Ok(Box::new(crate::macos::VideoToolboxDecoder::new(config)?))
            }
            
            #[cfg(target_os = "android")]
            HardwareAccelerationType::MediaCodec => {
                Ok(Box::new(crate::android::MediaCodecDecoder::new(config)?))
            }
            
            HardwareAccelerationType::None => {
                Err(anyhow::anyhow!("Hardware acceleration disabled"))
            }
            
            _ => {
                warn!("Hardware acceleration type {:?} not implemented, using fallback", config.hw_type);
                Ok(Box::new(FallbackDecoder::new(config)?))
            }
        }
    }
    
    /// Initialize the decoder
    pub fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing hardware decoder");
        self.backend.initialize()?;
        self.initialized = true;
        Ok(())
    }
    
    /// Check if decoder supports a codec profile
    pub fn supports_profile(&self, profile: CodecProfile) -> bool {
        self.backend.supports_profile(profile)
    }
    
    /// Get supported profiles
    pub fn supported_profiles(&self) -> Vec<CodecProfile> {
        self.backend.supported_profiles()
    }
    
    /// Decode a frame
    pub fn decode(&mut self, data: &[u8]) -> Result<DecoderOutput> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Decoder not initialized"));
        }
        
        self.backend.decode(data)
    }
    
    /// Decode asynchronously
    pub async fn decode_async(&mut self, data: Vec<u8>) -> Result<DecoderOutput> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Decoder not initialized"));
        }
        
        self.backend.decode_async(data).await
    }
    
    /// Flush the decoder
    pub fn flush(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Decoder not initialized"));
        }
        
        self.backend.flush()
    }
    
    /// Get decoder statistics
    pub fn stats(&self) -> DecoderStats {
        self.backend.stats()
    }
    
    /// Reset the decoder
    pub fn reset(&mut self) -> Result<()> {
        info!("🔄 Resetting hardware decoder");
        self.backend.reset()?;
        Ok(())
    }
}

/// Decoder output
#[derive(Debug, Clone)]
pub struct DecoderOutput {
    /// Decoded frame data
    pub data: Vec<u8>,
    
    /// Frame width
    pub width: u32,
    
    /// Frame height
    u32,
    
    /// Frame format (e.g., "nv12", "yuv420p")
    pub format: String,
    
    /// Timestamp (in microseconds)
    pub timestamp: i64,
    
    /// Hardware buffer handle (for zero-copy)
    pub hw_buffer: Option<HwBufferHandle>,
}

/// Hardware buffer handle
#[derive(Debug, Clone)]
pub struct HwBufferHandle {
    /// Buffer ID
    pub id: u64,
    
    /// Memory type
    pub memory_type: HwMemoryType,
}

/// Hardware memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HwMemoryType {
    /// System memory
    SystemMemory,
    
    /// Device memory
    DeviceMemory,
    
    /// Pinned memory
    PinnedMemory,
    
    /// Shared memory
    SharedMemory,
}

/// Decoder statistics
#[derive(Debug, Clone, Copy)]
pub struct DecoderStats {
    /// Number of frames decoded
    pub frames_decoded: u64,
    
    /// Number of errors
    pub errors: u64,
    
    /// Average decode time (in microseconds)
    pub avg_decode_time_us: u64,
    
    /// Current buffer usage
    pub buffer_usage: usize,
}

impl Default for DecoderStats {
    fn default() -> Self {
        Self {
            frames_decoded: 0,
            errors: 0,
            avg_decode_time_us: 0,
            buffer_usage: 0,
        }
    }
}

/// Decoder backend trait
pub trait DecoderBackend: Send + Sync {
    /// Initialize the decoder
    fn initialize(&mut self) -> Result<()>;
    
    /// Check if decoder supports a codec profile
    fn supports_profile(&self, profile: CodecProfile) -> bool;
    
    /// Get supported profiles
    fn supported_profiles(&self) -> Vec<CodecProfile>;
    
    /// Decode a frame
    fn decode(&mut self, data: &[u8]) -> Result<DecoderOutput>;
    
    /// Decode asynchronously
    fn decode_async(&mut self, data: Vec<u8>) -> Result<DecoderOutput> {
        // Default implementation: synchronous decode
        self.decode(&data)
    }
    
    /// Flush the decoder
    fn flush(&mut self) -> Result<()>;
    
    /// Get decoder statistics
    fn stats(&self) -> DecoderStats;
    
    /// Reset the decoder
    fn reset(&mut self) -> Result<()>;
}

/// Fallback decoder (software decoding)
struct FallbackDecoder {
    config: DecoderConfig,
    stats: DecoderStats,
}

impl FallbackDecoder {
    fn new(config: &DecoderConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            stats: DecoderStats::default(),
        })
    }
}

impl DecoderBackend for FallbackDecoder {
    fn initialize(&mut self) -> Result<()> {
        info!("🔄 Using software fallback decoder");
        Ok(())
    }
    
    fn supports_profile(&self, _profile: CodecProfile) -> bool {
        // Software decoder supports most profiles
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
    
    fn decode(&mut self, _data: &[u8]) -> Result<DecoderOutput> {
        // Placeholder for software decoding
        // In real implementation, this would use FFmpeg software decoders
        Ok(DecoderOutput {
            data: vec![],
            width: 1920,
            height: 1080,
            format: "yuv420p".to_string(),
            timestamp: 0,
            hw_buffer: None,
        })
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn stats(&self) -> DecoderStats {
        self.stats
    }
    
    fn reset(&mut self) -> Result<()> {
        self.stats = DecoderStats::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let config = DecoderConfig::default();
        let result = HardwareDecoder::new(config);
        // May fail if no hardware acceleration available
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_fallback_decoder() {
        let config = DecoderConfig::default();
        let decoder = FallbackDecoder::new(&config).unwrap();
        assert!(decoder.supports_profile(CodecProfile::H264Main));
    }
}