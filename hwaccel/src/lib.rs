//! Vantis Hardware Acceleration Module
//! 
//! Provides hardware acceleration support for video decoding/encoding
//! across multiple platforms and GPU vendors.

pub mod common;
pub mod detection;
pub mod decoder;
pub mod encoder;
pub mod processor;
pub mod config;

// Platform-specific modules
#[cfg(windows)]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "android")]
pub mod android;

// Re-exports
pub use common::{
    HardwareAccelerationType,
    AccelerationBackend,
    CodecProfile,
    AccelerationCapabilities,
};

pub use detection::{
    HardwareDetector,
    detect_available_hardware,
    detect_capabilities,
};

pub use decoder::{
    HardwareDecoder,
    DecoderConfig,
    DecoderOutput,
};

pub use encoder::{
    HardwareEncoder,
    EncoderConfig,
    EncoderOutput,
};

pub use processor::{
    HardwareProcessor,
    ProcessorConfig,
};

pub use config::{
    HwAccelConfig,
    BackendPreference,
    DeviceSelection,
};

use anyhow::Result;
use tracing::info;

/// Hardware acceleration manager
pub struct HardwareAccelerationManager {
    config: HwAccelConfig,
    detector: HardwareDetector,
    decoder: Option<HardwareDecoder>,
    encoder: Option<HardwareEncoder>,
    processor: Option<HardwareProcessor>,
}

impl HardwareAccelerationManager {
    /// Create a new hardware acceleration manager
    pub fn new(config: HwAccelConfig) -> Result<Self> {
        info!("🚀 Initializing Hardware Acceleration Manager");
        
        let detector = HardwareDetector::new()?;
        
        Ok(Self {
            config,
            detector,
            decoder: None,
            encoder: None,
            processor: None,
        })
    }

    /// Detect available hardware acceleration
    pub fn detect(&self) -> Result<AccelerationCapabilities> {
        info!("🔍 Detecting hardware acceleration capabilities");
        self_detector.detect_capabilities(&self.config)
    }

    /// Initialize hardware decoder
    pub fn init_decoder(&mut self, config: DecoderConfig) -> Result<()> {
        info!("🎥 Initializing hardware decoder");
        let decoder = HardwareDecoder::new(config)?;
        self.decoder = Some(decoder);
        Ok(())
    }

    /// Initialize hardware encoder
    pub fn init_encoder(&mut self, config: EncoderConfig) -> Result<()> {
        info!("🎬 Initializing hardware encoder");
        let encoder = HardwareEncoder::new(config)?;
        self.encoder = Some(encoder);
        Ok(())
    }

    /// Initialize hardware processor
    pub fn init_processor(&mut self, config: ProcessorConfig) -> Result<()> {
        info!("⚙️  Initializing hardware processor");
        let processor = HardwareProcessor::new(config)?;
        self.processor = Some(processor);
        Ok(())
    }

    /// Get hardware decoder
    pub fn decoder(&self) -> Option<&HardwareDecoder> {
        self.decoder.as_ref()
    }

    /// Get hardware encoder
    pub fn encoder(&self) -> Option<&HardwareEncoder> {
        self.encoder.as_ref()
    }

    /// Get hardware processor
    pub fn processor(&self) -> Option<&HardwareProcessor> {
        self.processor.as_ref()
    }

    /// Check if hardware acceleration is available
    pub fn is_available(&self) -> bool {
        self.detector.is_available()
    }

    /// Get available backends
    pub fn available_backends(&self) -> Vec<AccelerationBackend> {
        self.detector.available_backends()
    }
}

impl Default for HardwareAccelerationManager {
    fn default() -> Self {
        Self::new(HwAccelConfig::default()).expect("Failed to create HardwareAccelerationManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwaccel_manager_creation() {
        let config = HwAccelConfig::default();
        let manager = HardwareAccelerationManager::new(config);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_detection() {
        let manager = HardwareAccelerationManager::default();
        let result = manager.detect();
        assert!(result.is_ok());
    }
}