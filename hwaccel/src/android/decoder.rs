//! Android hardware decoder implementation

use anyhow::Result;
use tracing::info;

use crate::common::CodecProfile;
use crate::config::DecoderConfig;
use crate::decoder::DecoderBackend;

/// MediaCodec decoder
pub struct MediaCodecDecoder {
    config: DecoderConfig,
    stats: crate::decoder::DecoderStats,
}

impl MediaCodecDecoder {
    pub fn new(config: &DecoderConfig) -> Result<Self> {
        info!("Creating MediaCodec decoder");
        Ok(Self {
            config: config.clone(),
            stats: crate::decoder::DecoderStats::default(),
        })
    }
}

impl DecoderBackend for MediaCodecDecoder {
    fn initialize(&mut self) -> Result<()> {
        info!("Initializing MediaCodec decoder");
        // TODO: Initialize MediaCodec decoder
        Ok(())
    }
    
    fn supports_profile(&self, profile: CodecProfile) -> bool {
        matches!(profile, 
            CodecProfile::H264Main | 
            CodecProfile::H264High |
            CodecProfile::H265Main |
            CodecProfile::H265Main10 |
            CodecProfile::VP9Profile0 |
            CodecProfile::AV1Main
        )
    }
    
    fn supported_profiles(&self) -> Vec<CodecProfile> {
        vec![
            CodecProfile::H264Main,
            CodecProfile::H264High,
            CodecProfile::H265Main,
            CodecProfile::H265Main10,
            CodecProfile::VP9Profile0,
            CodecProfile::AV1Main,
        ]
    }
    
    fn decode(&mut self, _data: &[u8]) -> Result<crate::decoder::DecoderOutput> {
        // TODO: Implement MediaCodec decoding
        Ok(crate::decoder::DecoderOutput {
            data: vec![],
            width: 1920,
            height: 1080,
            format: "nv12".to_string(),
            timestamp: 0,
            hw_buffer: None,
        })
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn stats(&self) -> crate::decoder::DecoderStats {
        self.stats
    }
    
    fn reset(&mut self) -> Result<()> {
        self.stats = crate::decoder::DecoderStats::default();
        Ok(())
    }
}

/// OpenGL processor (for scaling, deinterlacing, etc.)
pub struct OpenGLProcessor {
    config: crate::config::ProcessorConfig,
    stats: crate::processor::ProcessorStats,
}

impl OpenGLProcessor {
    pub fn new(config: &crate::config::ProcessorConfig) -> Result<Self> {
        info!("Creating OpenGL processor");
        Ok(Self {
            config: config.clone(),
            stats: crate::processor::ProcessorStats::default(),
        })
    }
}

impl crate::processor::ProcessorBackend for OpenGLProcessor {
    fn initialize(&mut self) -> Result<()> {
        info!("Initializing OpenGL processor");
        Ok(())
    }
    
    fn scale(&mut self, _data: &[u8], _src_width: u32, _src_height: u32, _dst_width: u32, _dst_height: u32) -> Result<Vec<u8>> {
        // TODO: Implement OpenGL scaling
        Ok(vec![])
    }
    
    fn deinterlace(&mut self, _data: &[u8], _width: u32, _height: u32) -> Result<Vec<u8>> {
        // TODO: Implement OpenGL deinterlacing
        Ok(vec![])
    }
    
    fn convert_colorspace(&mut self, _data: &[u8], _src_format: &str, _dst_format: &str, _width: u32, _height: u32) -> Result<Vec<u8>> {
        // TODO: Implement OpenGL color space conversion
        Ok(vec![])
    }
    
    fn tone_map(&mut self, _data: &[u8], _src_transfer: &str, _dst_transfer: &str, _width: u32, _height: u32) -> Result<Vec<u8>> {
        // TODO: Implement OpenGL tone mapping
        Ok(vec![])
    }
    
    fn stats(&self) -> crate::processor::ProcessorStats {
        self.stats
    }
}