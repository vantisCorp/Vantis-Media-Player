//! Video Frame representation

use wgpu::Texture;

/// Video frame with metadata
#[derive(Debug, Clone)]
pub struct VideoFrame {
    /// Frame index
    pub index: u64,
    
    /// Presentation timestamp (in nanoseconds)
    pub pts: u64,
    
    /// Duration of the frame (in nanoseconds)
    pub duration: u64,
    
    /// Width in pixels
    pub width: u32,
    
    /// Height in pixels
    pub height: u32,
    
    /// Pixel format
    pub format: PixelFormat,
    
    /// Color space
    pub color_space: ColorSpace,
    
    /// Whether this is a key frame
    pub is_keyframe: bool,
}

/// Pixel format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 8-bit YUV 4:2:0
    YUV420P,
    
    /// 8-bit RGB
    RGB24,
    
    /// 8-bit RGBA
    RGBA,
    
    /// 10-bit YUV 4:2:0
    YUV420P10,
    
    /// HDR formats
    HDR10,
    HLG,
}

/// Color space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// BT.601 (SD content)
    BT601,
    
    /// BT.709 (HD content)
    BT709,
    
    /// BT.2020 (UHD content)
    BT2020,
    
    /// SMPTE 240M
    SMPTE240M,
}

impl VideoFrame {
    /// Create a new video frame
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            index: 0,
            pts: 0,
            duration: 0,
            width,
            height,
            format: PixelFormat::YUV420P,
            color_space: ColorSpace::BT709,
            is_keyframe: false,
        }
    }
    
    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}