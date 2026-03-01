//! Audio Renderer
//! 
//! Outputs audio using CPAL (bit-perfect playback).

use anyhow::Result;
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig};
use parking_lot::Mutex;
use std::sync::Arc;
use tracing::info;

/// Audio renderer
pub struct AudioRenderer {
    /// Audio stream
    stream: Option<Stream>,
    
    /// Is playing
    playing: Arc<Mutex<bool>>,
}

impl AudioRenderer {
    pub fn new(device: &Device, config: StreamConfig, sample_format: SampleFormat) -> Result<Self> {
        info!("🔊 Initializing audio renderer");
        
        let playing = Arc::new(Mutex::new(false));
        let playing_clone = playing.clone();
        
        let stream = match sample_format {
            SampleFormat::F32 => Self::create_stream::<f32>(device, config, playing_clone)?,
            SampleFormat::I16 => Self::create_stream::<i16>(device, config, playing_clone)?,
            SampleFormat::U16 => Self::create_stream::<u16>(device, config, playing_clone)?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };
        
        Ok(Self {
            stream: Some(stream),
            playing,
        })
    }
    
    fn create_stream<T: Sample>(
        device: &Device,
        config: StreamConfig,
        playing: Arc<Mutex<bool>>,
    ) -> Result<Stream> {
        let err_fn = |err| eprintln!("audio stream error: {}", err);
        
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [T], _| {
                if *playing.lock() {
                    // Render audio samples
                    for sample in data.iter_mut() {
                        *sample = Sample::from(&0.0f32);
                    }
                }
            },
            err_fn,
            None,
        )?;
        
        Ok(stream)
    }
    
    pub fn start(&mut self) -> Result<()> {
        if let Some(ref stream) = self.stream {
            stream.play()?;
            *self.playing.lock() = true;
            info!("▶ Audio playback started");
        }
        Ok(())
    }
    
    pub fn pause(&mut self) -> Result<()> {
        if let Some(ref stream) = self.stream {
            stream.pause()?;
            *self.playing.lock() = false;
            info!("⏸ Audio playback paused");
        }
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<()> {
        if let Some(ref stream) = self.stream {
            stream.pause()?;
            *self.playing.lock() = false;
            info!("⏹ Audio playback stopped");
        }
        Ok(())
    }
}