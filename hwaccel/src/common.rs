//! Common types and structures for hardware acceleration

use serde::{Deserialize, Serialize};
use std::fmt;

/// Hardware acceleration type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HardwareAccelerationType {
    /// No hardware acceleration
    None,
    
    /// NVIDIA NVENC/NVDEC
    Nvidia,
    
    /// AMD AMF
    AMD,
    
    /// Intel Quick Sync Video
    Intel,
    
    /// Apple VideoToolbox
    Apple,
    
    /// VAAPI (Video Acceleration API)
    VAAPI,
    
    /// VDPAU (Video Decode and Presentation API for Unix)
    VDPAU,
    
    /// Video4Linux2
    V4L2,
    
    /// OpenCL-based acceleration
    OpenCL,
    
    /// DirectX Video Acceleration
    DXVA,
    
    /// Direct3D 11 Video
    D3D11VA,
    
    /// Media Foundation
    MediaFoundation,
    
    /// Android MediaCodec
    MediaCodec,
    
    /// Vulkan-based acceleration
    Vulkan,
}

impl fmt::Display for HardwareAccelerationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareAccelerationType::None => write!(f, "None"),
            HardwareAccelerationType::Nvidia => write!(f, "NVIDIA NVENC/NVDEC"),
            HardwareAccelerationType::AMD => write!(f, "AMD AMF"),
            HardwareAccelerationType::Intel => write!(f, "Intel QSV"),
            HardwareAccelerationType::Apple => write!(f, "Apple VideoToolbox"),
            HardwareAccelerationType::VAAPI => write!(f, "VAAPI"),
            HardwareAccelerationType::VDPAU => write!(f, "VDPAU"),
            HardwareAccelerationType::V4L2 => write!(f, "Video4Linux2"),
            HardwareAccelerationType::OpenCL => write!(f, "OpenCL"),
            HardwareAccelerationType::DXVA => write!(f, "DXVA"),
            HardwareAccelerationType::D3D11VA => write!(f, "D3D11VA"),
            HardwareAccelerationType::MediaFoundation => write!(f, "Media Foundation"),
            HardwareAccelerationType::MediaCodec => write!(f, "MediaCodec"),
            HardwareAccelerationType::Vulkan => write!(f, "Vulkan"),
        }
    }
}

/// Acceleration backend
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccelerationBackend {
    /// FFmpeg hardware acceleration
    FFmpeg,
    
    /// Vulkan API
    Vulkan,
    
    /// DirectX 11
    DirectX11,
    
    /// DirectX 12
    DirectX12,
    
    /// Metal
    Metal,
    
    /// OpenCL
    OpenCL,
    
    /// CUDA
    CUDA,
    
    /// VideoToolbox (macOS)
    VideoToolbox,
    
    /// MediaCodec (Android)
    MediaCodec,
    
    /// VAAPI (Linux)
    VAAPI,
    
    /// VDPAU (Linux)
    VDPAU,
}

impl fmt::Display for AccelerationBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccelerationBackend::FFmpeg => write!(f, "FFmpeg"),
            AccelerationBackend::Vulkan => write!(f, "Vulkan"),
            AccelerationBackend::DirectX11 => write!(f, "DirectX 11"),
            AccelerationBackend::DirectX12 => write!(f, "DirectX 12"),
            AccelerationBackend::Metal => write!(f, "Metal"),
            AccelerationBackend::OpenCL => write!(f, "OpenCL"),
            AccelerationBackend::CUDA => write!(f, "CUDA"),
            AccelerationBackend::VideoToolbox => write!(f, "VideoToolbox"),
            AccelerationBackend::MediaCodec => write!(f, "MediaCodec"),
            AccelerationBackend::VAAPI => write!(f, "VAAPI"),
            AccelerationBackend::VDPAU => write!(f, "VDPAU"),
        }
    }
}

/// Codec profile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodecProfile {
    /// H.264/AVC Baseline
    H264Baseline,
    
    /// H.264/AVC Main
    H264Main,
    
    /// H.264/AVC High
    H264High,
    
    /// H.265/HEVC Main
    H265Main,
    
    /// H.265/HEVC Main 10
    H265Main10,
    
    /// H.265/HEVC Main 12
    H265Main12,
    
    /// VP9 Profile 0
    VP9Profile0,
    
    /// VP9 Profile 1
    VP9Profile1,
    
    /// VP9 Profile 2
    VP9Profile2,
    
    /// VP9 Profile 3
    VP9Profile3,
    
    /// AV1 Main
    AV1Main,
    
    /// AV1 High
    AV1High,
    
    /// AV1 Professional
    AV1Professional,
    
    /// MPEG-2 Simple
    MPEG2Simple,
    
    /// MPEG-2 Main
    MPEG2Main,
    
    /// VC-1 Simple
    VC1Simple,
    
    /// VC-1 Main
    VC1Main,
    
    /// VC-1 Advanced
    VC1Advanced,
}

impl fmt::Display for CodecProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecProfile::H264Baseline => write!(f, "H.264 Baseline"),
            CodecProfile::H264Main => write!(f, "H.264 Main"),
            CodecProfile::H264High => write!(f, "H.264 High"),
            CodecProfile::H265Main => write!(f, "H.265 Main"),
            CodecProfile::H265Main10 => write!(f, "H.265 Main 10"),
            CodecProfile::H265Main12 => write!(f, "H.265 Main 12"),
            CodecProfile::VP9Profile0 => write!(f, "VP9 Profile 0"),
            CodecProfile::VP9Profile1 => write!(f, "VP9 Profile 1"),
            CodecProfile::VP9Profile2 => write!(f, "VP9 Profile 2"),
            CodecProfile::VP9Profile3 => write!(f, "VP9 Profile 3"),
            CodecProfile::AV1Main => write!(f, "AV1 Main"),
            CodecProfile::AV1High => write!(f, "AV1 High"),
            CodecProfile::AV1Professional => write!(f, "AV1 Professional"),
            CodecProfile::MPEG2Simple => write!(f, "MPEG-2 Simple"),
            CodecProfile::MPEG2Main => write!(f, "MPEG-2 Main"),
            CodecProfile::VC1Simple => write!(f, "VC-1 Simple"),
            CodecProfile::VC1Main => write!(f, "VC-1 Main"),
            CodecProfile::VC1Advanced => write!(f, "VC-1 Advanced"),
        }
    }
}

/// Acceleration capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationCapabilities {
    /// Available acceleration types
    pub available_types: Vec<HardwareAccelerationType>,
    
    /// Preferred acceleration type
    pub preferred_type: Option<HardwareAccelerationType>,
    
    /// Available backends
    pub available_backends: Vec<AccelerationBackend>,
    
    /// Supported decoder profiles
    pub decoder_profiles: Vec<CodecProfile>,
    
    /// Supported encoder profiles
    pub encoder_profiles: Vec<CodecProfile>,
    
    /// Maximum resolution
    pub max_resolution: (u32, u32),
    
    /// Maximum bitrate (in Mbps)
    pub max_bitrate: u32,
    
    /// Zero-copy support
    pub zero_copy: bool,
    
    /// Async processing support
    pub async_processing: bool,
    
    /// Hardware frame pool support
    pub frame_pool: bool,
}

impl AccelerationCapabilities {
    /// Check if a specific acceleration type is available
    pub fn has_type(&self, hw_type: HardwareAccelerationType) -> bool {
        self.available_types.contains(&hw_type)
    }
    
    /// Check if a specific backend is available
    pub fn has_backend(&self, backend: AccelerationBackend) -> bool {
        self.available_backends.contains(&backend)
    }
    
    /// Check if a codec profile is supported for decoding
    pub fn supports_decode(&self, profile: CodecProfile) -> bool {
        self.decoder_profiles.contains(&profile)
    }
    
    /// Check if a codec profile is supported for encoding
    pub fn supports_encode(&self, profile: CodecProfile) -> bool {
        self.encoder_profiles.contains(&profile)
    }
}

impl Default for AccelerationCapabilities {
    fn default() -> Self {
        Self {
            available_types: vec![],
            preferred_type: None,
            available_backends: vec![],
            decoder_profiles: vec![],
            encoder_profiles: vec![],
            max_resolution: (3840, 2160),
            max_bitrate: 100,
            zero_copy: false,
            async_processing: false,
            frame_pool: false,
        }
    }
}

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUDeviceInfo {
    /// Device name
    pub name: String,
    
    /// Vendor
    pub vendor: String,
    
    /// Vendor ID
    pub vendor_id: u32,
    
    /// Device ID
    pub device_id: u32,
    
    /// Total memory (in bytes)
    pub total_memory: u64,
    
    /// Available memory (in bytes)
    pub available_memory: u64,
    
    /// Compute capability
    pub compute_capability: Option<(u32, u32)>,
    
    /// Driver version
    pub driver_version: String,
}

impl fmt::Display for GPUDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GPU Device: {}", self.name)?;
        writeln!(f, "  Vendor: {} (0x{:04X})", self.vendor, self.vendor_id)?;
        writeln!(f, "  Device ID: 0x{:04X}", self.device_id)?;
        writeln!(f, "  Memory: {} / {} MB", 
            self.available_memory / 1024 / 1024,
            self.total_memory / 1024 / 1024
        )?;
        if let Some((major, minor)) = self.compute_capability {
            writeln!(f, "  Compute Capability: {}.{}", major, minor)?;
        }
        writeln!(f, "  Driver: {}", self.driver_version)
    }
}