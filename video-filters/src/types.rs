//! Core types for video processing

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Video frame data
#[derive(Debug, Clone)]
pub struct Frame {
    /// Frame width in pixels
    pub width: u32,
    
    /// Frame height in pixels
    pub height: u32,
    
    /// Pixel format
    pub pixel_format: PixelFormat,
    
    /// Raw pixel data
    pub data: Vec<u8>,
    
    /// Frame metadata
    pub info: FrameInfo,
}

impl Frame {
    /// Create a new frame
    pub fn new(width: u32, height: u32, pixel_format: PixelFormat) -> Self {
        let data_size = width as usize * height as usize * pixel_format.bytes_per_pixel();
        Self {
            width,
            height,
            pixel_format,
            data: vec![0u8; data_size],
            info: FrameInfo::default(),
        }
    }
    
    /// Create a frame from existing data
    pub fn from_data(
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        data: Vec<u8>,
    ) -> Option<Self> {
        let expected_size = width as usize * height as usize * pixel_format.bytes_per_pixel();
        if data.len() != expected_size {
            return None;
        }
        Some(Self {
            width,
            height,
            pixel_format,
            data,
            info: FrameInfo::default(),
        })
    }
    
    /// Get pixel at coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        
        let bpp = self.pixel_format.bytes_per_pixel();
        let idx = (y as usize * self.width as usize + x as usize) * bpp;
        
        match self.pixel_format {
            PixelFormat::RGBA => {
                if idx + 3 < self.data.len() {
                    Some([self.data[idx], self.data[idx+1], self.data[idx+2], self.data[idx+3]])
                } else {
                    None
                }
            }
            PixelFormat::RGB => {
                if idx + 2 < self.data.len() {
                    Some([self.data[idx], self.data[idx+1], self.data[idx+2], 255])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// Set pixel at coordinates
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let bpp = self.pixel_format.bytes_per_pixel();
        let idx = (y as usize * self.width as usize + x as usize) * bpp;
        
        match self.pixel_format {
            PixelFormat::RGBA => {
                if idx + 3 < self.data.len() {
                    self.data[idx] = r;
                    self.data[idx+1] = g;
                    self.data[idx+2] = b;
                    self.data[idx+3] = a;
                }
            }
            PixelFormat::RGB => {
                if idx + 2 < self.data.len() {
                    self.data[idx] = r;
                    self.data[idx+1] = g;
                    self.data[idx+2] = b;
                }
            }
            _ => {}
        }
    }
    
    /// Get the raw data slice
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    /// Get mutable data slice
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
    
    /// Get frame dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    /// Convert to a different pixel format
    pub fn convert_to(&self, target_format: PixelFormat) -> Option<Self> {
        if self.pixel_format == target_format {
            return Some(self.clone());
        }
        
        // Simple conversion for common formats
        match (self.pixel_format, target_format) {
            (PixelFormat::RGB, PixelFormat::RGBA) => {
                let mut new_data = Vec::with_capacity(self.data.len() / 3 * 4);
                for chunk in self.data.chunks(3) {
                    new_data.push(chunk[0]);
                    new_data.push(chunk[1]);
                    new_data.push(chunk[2]);
                    new_data.push(255);
                }
                Some(Self {
                    width: self.width,
                    height: self.height,
                    pixel_format: PixelFormat::RGBA,
                    data: new_data,
                    info: self.info.clone(),
                })
            }
            (PixelFormat::RGBA, PixelFormat::RGB) => {
                let mut new_data = Vec::with_capacity(self.data.len() / 4 * 3);
                for chunk in self.data.chunks(4) {
                    new_data.push(chunk[0]);
                    new_data.push(chunk[1]);
                    new_data.push(chunk[2]);
                }
                Some(Self {
                    width: self.width,
                    height: self.height,
                    pixel_format: PixelFormat::RGB,
                    data: new_data,
                    info: self.info.clone(),
                })
            }
            _ => None,
        }
    }
}

/// Pixel format of the frame
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PixelFormat {
    /// RGB, 3 bytes per pixel
    RGB,
    
    /// RGBA, 4 bytes per pixel
    RGBA,
    
    /// BGR, 3 bytes per pixel
    BGR,
    
    /// BGRA, 4 bytes per pixel
    BGRA,
    
    /// Grayscale, 1 byte per pixel
    Gray,
    
    /// YUV420P (planar)
    YUV420P,
    
    /// NV12 (semi-planar)
    NV12,
    
    /// YUV422P (planar)
    YUV422P,
}

impl PixelFormat {
    /// Get bytes per pixel
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::RGB | Self::BGR => 3,
            Self::RGBA | Self::BGRA => 4,
            Self::Gray => 1,
            Self::YUV420P => 1, // Average (12 bits per pixel)
            Self::NV12 => 1,    // Average (12 bits per pixel)
            Self::YUV422P => 2, // Average (16 bits per pixel)
        }
    }
    
    /// Get number of channels
    pub fn channels(&self) -> usize {
        match self {
            Self::RGB | Self::BGR => 3,
            Self::RGBA | Self::BGRA => 4,
            Self::Gray => 1,
            Self::YUV420P | Self::NV12 | Self::YUV422P => 3,
        }
    }
    
    /// Check if this format has an alpha channel
    pub fn has_alpha(&self) -> bool {
        matches!(self, Self::RGBA | Self::BGRA)
    }
}

/// Frame metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameInfo {
    /// Frame number
    pub frame_number: u64,
    
    /// Presentation timestamp
    pub pts: Duration,
    
    /// Duration of the frame
    pub duration: Duration,
    
    /// Color space
    pub color_space: ColorSpace,
    
    /// Whether frame is a keyframe
    pub is_keyframe: bool,
    
    /// Whether frame is interlaced
    pub is_interlaced: bool,
}

impl Default for FrameInfo {
    fn default() -> Self {
        Self {
            frame_number: 0,
            pts: Duration::ZERO,
            duration: Duration::from_secs_f64(1.0 / 30.0),
            color_space: ColorSpace::BT709,
            is_keyframe: false,
            is_interlaced: false,
        }
    }
}

/// Color space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorSpace {
    /// BT.601 (SD)
    BT601,
    
    /// BT.709 (HD)
    BT709,
    
    /// BT.2020 (UHD/HDR)
    BT2020,
    
    /// sRGB
    SRGB,
    
    /// Display P3
    DisplayP3,
    
    /// Adobe RGB
    AdobeRGB,
}

impl ColorSpace {
    /// Get the primary colors for this color space
    pub fn primaries(&self) -> [(f32, f32); 3] {
        match self {
            Self::BT601 => [(0.640, 0.330), (0.290, 0.600), (0.150, 0.060)],
            Self::BT709 => [(0.640, 0.330), (0.300, 0.600), (0.150, 0.060)],
            Self::BT2020 => [(0.708, 0.292), (0.170, 0.797), (0.131, 0.046)],
            Self::SRGB => [(0.640, 0.330), (0.300, 0.600), (0.150, 0.060)],
            Self::DisplayP3 => [(0.680, 0.320), (0.265, 0.690), (0.150, 0.060)],
            Self::AdobeRGB => [(0.640, 0.330), (0.210, 0.710), (0.150, 0.060)],
        }
    }
    
    /// Get the white point
    pub fn white_point(&self) -> (f32, f32) {
        // D65 for all standard spaces
        (0.3127, 0.3290)
    }
}

/// Rectangle for region processing
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn full_frame(width: u32, height: u32) -> Self {
        Self { x: 0, y: 0, width, height }
    }
    
    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

/// Point for coordinate operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Size dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}