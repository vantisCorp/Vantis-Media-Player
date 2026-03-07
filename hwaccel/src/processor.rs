//! Hardware-accelerated video processor (scaling, deinterlacing, etc.)

use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::common::HardwareAccelerationType;
use crate::config::ProcessorConfig;

/// Hardware processor
pub struct HardwareProcessor {
    config: ProcessorConfig,
    backend: Box<dyn ProcessorBackend>,
    initialized: bool,
}

impl HardwareProcessor {
    /// Create a new hardware processor
    pub fn new(config: ProcessorConfig) -> Result<Self> {
        info!("⚙️  Creating hardware processor");
        
        let backend = Self::create_backend(&config)?;
        
        Ok(Self {
            config,
            backend,
            initialized: false,
        })
    }
    
    /// Create processor backend based on configuration
    fn create_backend(config: &ProcessorConfig) -> Result<Box<dyn ProcessorBackend>> {
        #[cfg(windows)]
        {
            Ok(Box::new(crate::windows::DirectXProcessor::new(config)?))
        }
        
        #[cfg(target_os = "linux")]
        {
            Ok(Box::new(crate::linux::VAProcessor::new(config)?))
        }
        
        #[cfg(target_os = "macos")]
        {
            Ok(Box::new(crate::macos::MetalProcessor::new(config)?))
        }
        
        #[cfg(target_os = "android")]
        {
            Ok(Box::new(crate::android::OpenGLProcessor::new(config)?))
        }
        
        #[cfg(not(any(windows, target_os = "linux", target_os = "macos", target_os = "android")))]
        {
            Ok(Box::new(FallbackProcessor::new(config)?))
        }
    }
    
    /// Initialize the processor
    pub fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing hardware processor");
        self.backend.initialize()?;
        self.initialized = true;
        Ok(())
    }
    
    /// Scale a frame
    pub fn scale(&mut self, data: &[u8], src_width: u32, src_height: u32, dst_width: u32, dst_height: u32) -> Result<Vec<u8>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Processor not initialized"));
        }
        
        self.backend.scale(data, src_width, src_height, dst_width, dst_height)
    }
    
    /// Deinterlace a frame
    pub fn deinterlace(&mut self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Processor not initialized"));
        }
        
        self.backend.deinterlace(data, width, height)
    }
    
    /// Convert color space
    pub fn convert_colorspace(&mut self, data: &[u8], src_format: &str, dst_format: &str, width: u32, height: u32) -> Result<Vec<u8>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Processor not initialized"));
        }
        
        self.backend.convert_colorspace(data, src_format, dst_format, width, height)
    }
    
    /// Apply tone mapping (HDR to SDR)
    pub fn tone_map(&mut self, data: &[u8], src_transfer: &str, dst_transfer: &str, width: u32, height: u32) -> Result<Vec<u8>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Processor not initialized"));
        }
        
        self.backend.tone_map(data, src_transfer, dst_transfer, width, height)
    }
    
    /// Get processor statistics
    pub fn stats(&self) -> ProcessorStats {
        self.backend.stats()
    }
}

/// Processor statistics
#[derive(Debug, Clone, Copy)]
pub struct ProcessorStats {
    /// Number of frames processed
    pub frames_processed: u64,
    
    /// Number of scaling operations
    pub scaling_ops: u64,
    
    /// Number of deinterlacing operations
    pub deinterlace_ops: u64,
    
    /// Number of color space conversions
    pub colorspace_ops: u64,
    
    /// Number of tone mapping operations
    pub tone_map_ops: u64,
    
    /// Average processing time (in microseconds)
    pub avg_process_time_us: u64,
}

impl Default for ProcessorStats {
    fn default() -> Self {
        Self {
            frames_processed: 0,
            scaling_ops: 0,
            deinterlace_ops: 0,
            colorspace_ops: 0,
            tone_map_ops: 0,
            avg_process_time_us: 0,
        }
    }
}

/// Processor backend trait
pub trait ProcessorBackend: Send + Sync {
    /// Initialize the processor
    fn initialize(&mut self) -> Result<()>;
    
    /// Scale a frame
    fn scale(&mut self, data: &[u8], src_width: u32, src_height: u32, dst_width: u32, dst_height: u32) -> Result<Vec<u8>>;
    
    /// Deinterlace a frame
    fn deinterlace(&mut self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>>;
    
    /// Convert color space
    fn convert_colorspace(&mut self, data: &[u8], src_format: &str, dst_format: &str, width: u32, height: u32) -> Result<Vec<u8>>;
    
    /// Apply tone mapping
    fn tone_map(&mut self, data: &[u8], src_transfer: &str, dst_transfer: &str, width: u32, height: u32) -> Result<Vec<u8>>;
    
    /// Get processor statistics
    fn stats(&self) -> ProcessorStats;
}

/// Fallback processor (software processing)
struct FallbackProcessor {
    config: ProcessorConfig,
    stats: ProcessorStats,
}

impl FallbackProcessor {
    fn new(config: &ProcessorConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            stats: ProcessorStats::default(),
        })
    }
}

impl ProcessorBackend for FallbackProcessor {
    fn initialize(&mut self) -> Result<()> {
        info!("🔄 Using software fallback processor");
        Ok(())
    }
    
    fn scale(&mut self, _data: &[u8], _src_width: u32, _src_height: u32, _dst_width: u32, _dst_height: u32) -> Result<Vec<u8>> {
        // Placeholder for software scaling
        // In real implementation, this would use FFmpeg or similar
        Ok(vec![])
    }
    
    fn deinterlace(&mut self, _data: &[u8], _width: u32, _height: u32) -> Result<Vec<u8>> {
        // Placeholder for software deinterlacing
        Ok(vec![])
    }
    
    fn convert_colorspace(&mut self, _data: &[u8], _src_format: &str, _dst_format: &str, _width: u32, _height: u32) -> Result<Vec<u8>> {
        // Placeholder for software color space conversion
        Ok(vec![])
    }
    
    fn tone_map(&mut self, _data: &[u8], _src_transfer: &str, _dst_transfer: &str, _width: u32, _height: u32) -> Result<Vec<u8>> {
        // Placeholder for software tone mapping
        Ok(vec![])
    }
    
    fn stats(&self) -> ProcessorStats {
        self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let config = ProcessorConfig::default();
        let result = HardwareProcessor::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fallback_processor() {
        let config = ProcessorConfig::default();
        let processor = FallbackProcessor::new(&config).unwrap();
        assert!(processor.initialize().is_ok());
    }
}