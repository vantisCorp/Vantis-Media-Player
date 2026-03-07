//! Hardware acceleration detection

use anyhow::{Context, Result};
use tracing::{debug, info, warn};
use std::sync::Arc;

use crate::common::{
    AccelerationBackend,
    AccelerationCapabilities,
    GPUDeviceInfo,
    HardwareAccelerationType,
};
use crate::config::HwAccelConfig;

// Platform-specific detectors
#[cfg(windows)]
use crate::windows::WindowsDetector;

#[cfg(target_os = "linux")]
use crate::linux::LinuxDetector;

#[cfg(target_os = "macos")]
use crate::macos::MacOSDetector;

#[cfg(target_os = "android")]
use crate::android::AndroidDetector;

/// Hardware detector
pub struct HardwareDetector {
    config: Arc<HwAccelConfig>,
    platform_detector: Box<dyn PlatformDetector>,
}

impl HardwareDetector {
    /// Create a new hardware detector
    pub fn new() -> Result<Self> {
        let config = Arc::new(HwAccelConfig::default());
        let platform_detector = Self::create_platform_detector()?;
        
        Ok(Self {
            config,
            platform_detector,
        })
    }
    
    /// Create platform-specific detector
    fn create_platform_detector() -> Result<Box<dyn PlatformDetector>> {
        #[cfg(windows)]
        {
            Ok(Box::new(WindowsDetector::new()?))
        }
        
        #[cfg(target_os = "linux")]
        {
            Ok(Box::new(LinuxDetector::new()?))
        }
        
        #[cfg(target_os = "macos")]
        {
            Ok(Box::new(MacOSDetector::new()?))
        }
        
        #[cfg(target_os = "android")]
        {
            Ok(Box::new(AndroidDetector::new()?))
        }
        
        #[cfg(not(any(windows, target_os = "linux", target_os = "macos", target_os = "android")))]
        {
            Ok(Box::new(GenericDetector::new()?))
        }
    }
    
    /// Detect available hardware acceleration
    pub fn detect_capabilities(&self, config: &HwAccelConfig) -> Result<AccelerationCapabilities> {
        info!("🔍 Starting hardware acceleration detection");
        
        // Get GPU information
        let gpu_info = self.platform_detector.detect_gpu()
            .unwrap_or_else(|e| {
                warn!("Failed to detect GPU: {}", e);
                None
            });
        
        if let Some(ref gpu) = gpu_info {
            info!("🎮 Detected GPU:\n{}", gpu);
        }
        
        // Detect available acceleration types
        let mut capabilities = AccelerationCapabilities::default();
        
        capabilities.available_types = self.platform_detector.detect_acceleration_types()?;
        capabilities.available_backends = self.platform_detector.detect_backends()?;
        capabilities.decoder_profiles = self.platform_detector.detect_decoder_profiles()?;
        capabilities.encoder_profiles = self.platform_detector.detect_encoder_profiles()?;
        
        // Determine preferred acceleration type
        capabilities.preferred_type = self.select_preferred_type(&capabilities, config);
        
        // Detect capabilities
        capabilities.max_resolution = self.platform_detector.detect_max_resolution()?;
        capabilities.max_bitrate = self.platform_detector.detect_max_bitrate()?;
        capabilities.zero_copy = self.platform_detector.supports_zero_copy()?;
        capabilities.async_processing = self.platform_detector.supports_async()?;
        capabilities.frame_pool = self.platform_detector.supports_frame_pool()?;
        
        info!("✅ Detection complete");
        info!("   Available types: {:?}", capabilities.available_types);
        info!("   Preferred type: {:?}", capabilities.preferred_type);
        info!("   Available backends: {:?}", capabilities.available_backends);
        info!("   Zero-copy: {}", capabilities.zero_copy);
        info!("   Async processing: {}", capabilities.async_processing);
        
        Ok(capabilities)
    }
    
    /// Select preferred acceleration type based on config and availability
    fn select_preferred_type(
        &self,
        capabilities: &AccelerationCapabilities,
        config: &HwAccelConfig,
    ) -> Option<HardwareAccelerationType> {
        // If user specified a preference, try to use it
        if let Some(ref preference) = config.backend_preference {
            if capabilities.available_types.contains(preference) {
                return Some(*preference);
            }
            warn!("Preferred acceleration type {:?} not available", preference);
        }
        
        // Auto-select based on platform and capabilities
        #[cfg(windows)]
        {
            // Windows: Prefer DXVA/D3D11VA
            for hw_type in &[HardwareAccelerationType::D3D11VA, HardwareAccelerationType::DXVA] {
                if capabilities.available_types.contains(hw_type) {
                    return Some(*hw_type);
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux: Prefer VAAPI, then VDPAU
            for hw_type in &[HardwareAccelerationType::VAAPI, HardwareAccelerationType::VDPAU] {
                if capabilities.available_types.contains(hw_type) {
                    return Some(*hw_type);
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS: VideoToolbox
            if capabilities.available_types.contains(&HardwareAccelerationType::Apple) {
                return Some(HardwareAccelerationType::Apple);
            }
        }
        
        #[cfg(target_os = "android")]
        {
            // Android: MediaCodec
            if capabilities.available_types.contains(&HardwareAccelerationType::MediaCodec) {
                return Some(HardwareAccelerationType::MediaCodec);
            }
        }
        
        // Fallback to first available
        capabilities.available_types.first().copied()
    }
    
    /// Check if hardware acceleration is available
    pub fn is_available(&self) -> bool {
        !self.platform_detector.detect_acceleration_types().unwrap_or_default().is_empty()
    }
    
    /// Get available backends
    pub fn available_backends(&self) -> Vec<AccelerationBackend> {
        self.platform_detector.detect_backends().unwrap_or_default()
    }
    
    /// Get GPU information
    pub fn gpu_info(&self) -> Result<Option<GPUDeviceInfo>> {
        self.platform_detector.detect_gpu()
    }
}

impl Default for HardwareDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create HardwareDetector")
    }
}

/// Platform detector trait
pub trait PlatformDetector: Send + Sync {
    /// Detect GPU information
    fn detect_gpu(&self) -> Result<Option<GPUDeviceInfo>>;
    
    /// Detect available acceleration types
    fn detect_acceleration_types(&self) -> Result<Vec<HardwareAccelerationType>>;
    
    /// Detect available backends
    fn detect_backends(&self) -> Result<Vec<AccelerationBackend>>;
    
    /// Detect decoder profiles
    fn detect_decoder_profiles(&self) -> Result<Vec<crate::common::CodecProfile>>;
    
    /// Detect encoder profiles
    fn detect_encoder_profiles(&self) -> Result<Vec<crate::common::CodecProfile>>;
    
    /// Detect maximum resolution
    fn detect_max_resolution(&self) -> Result<(u32, u32)>;
    
    /// Detect maximum bitrate
    fn detect_max_bitrate(&self) -> Result<u32>;
    
    /// Check if zero-copy is supported
    fn supports_zero_copy(&self) -> Result<bool>;
    
    /// Check if async processing is supported
    fn supports_async(&self) -> Result<bool>;
    
    /// Check if frame pool is supported
    fn supports_frame_pool(&self) -> Result<bool>;
}

/// Generic detector for unsupported platforms
struct GenericDetector;

impl GenericDetector {
    fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl PlatformDetector for GenericDetector {
    fn detect_gpu(&self) -> Result<Option<GPUDeviceInfo>> {
        Ok(None)
    }
    
    fn detect_acceleration_types(&self) -> Result<Vec<HardwareAccelerationType>> {
        Ok(vec![])
    }
    
    fn detect_backends(&self) -> Result<Vec<AccelerationBackend>> {
        Ok(vec![])
    }
    
    fn detect_decoder_profiles(&self) -> Result<Vec<crate::common::CodecProfile>> {
        Ok(vec![])
    }
    
    fn detect_encoder_profiles(&self) -> Result<Vec<crate::common::CodecProfile>> {
        Ok(vec![])
    }
    
    fn detect_max_resolution(&self) -> Result<(u32, u32)> {
        Ok((1920, 1080))
    }
    
    fn detect_max_bitrate(&self) -> Result<u32> {
        Ok(50)
    }
    
    fn supports_zero_copy(&self) -> Result<bool> {
        Ok(false)
    }
    
    fn supports_async(&self) -> Result<bool> {
        Ok(false)
    }
    
    fn supports_frame_pool(&self) -> Result<bool> {
        Ok(false)
    }
}

/// Convenience function to detect available hardware
pub fn detect_available_hardware() -> Result<AccelerationCapabilities> {
    let detector = HardwareDetector::new()?;
    let config = HwAccelConfig::default();
    detector.detect_capabilities(&config)
}

/// Convenience function to detect capabilities
pub fn detect_capabilities(config: &HwAccelConfig) -> Result<AccelerationCapabilities> {
    let detector = HardwareDetector::new()?;
    detector.detect_capabilities(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = HardwareDetector::new();
        assert!(detector.is_ok());
    }

    #[test]
    fn test_detection() {
        let detector = HardwareDetector::new().unwrap();
        let result = detector.detect_capabilities(&HwAccelConfig::default());
        // Result may be empty on systems without hardware acceleration
        assert!(result.is_ok());
    }
}