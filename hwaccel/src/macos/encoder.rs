//! macOS hardware encoder implementation

use anyhow::Result;
use tracing::info;

use crate::common::CodecProfile;
use crate::config::EncoderConfig;
use crate::encoder::EncoderBackend;

/// VideoToolbox encoder
pub struct VideoToolboxEncoder {
    config: EncoderConfig,
    stats: crate::encoder::EncoderStats,
}

impl VideoToolboxEncoder {
    pub fn new(config: &EncoderConfig) -> Result<Self> {
        info!("Creating VideoToolbox encoder");
        Ok(Self {
            config: config.clone(),
            stats: crate::encoder::EncoderStats::default(),
        })
    }
}

impl EncoderBackend for VideoToolboxEncoder {
    fn initialize(&mut self) -> Result<()> {
        info!("Initializing VideoToolbox encoder");
        // TODO: Initialize VideoToolbox encoder
        Ok(())
    }
    
    fn supports_profile(&self, profile: CodecProfile) -> bool {
        matches!(profile, 
            CodecProfile::H264Main | 
            CodecProfile::H264High |
            CodecProfile::H265Main |
            CodecProfile::H265Main10
        )
    }
    
    fn supported_profiles(&self) -> Vec<CodecProfile> {
        vec![
            CodecProfile::H264Main,
            CodecProfile::H264High,
            CodecProfile::H265Main,
            CodecProfile::H265Main10,
        ]
    }
    
    fn encode(&mut self, _data: &[u8], _width: u32, _height: u32) -> Result<crate::encoder::EncoderOutput> {
        // TODO: Implement VideoToolbox encoding
        Ok(crate::encoder::EncoderOutput {
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
    
    fn stats(&self) -> crate::encoder::EncoderStats {
        self.stats
    }
    
    fn reset(&mut self) -> Result<()> {
        self.stats = crate::encoder::EncoderStats::default();
        Ok(())
    }
}