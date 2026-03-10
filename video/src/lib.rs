//! Vantis Video Engine
//! 
//! GPU-accelerated video processing with AI upscaling,
/// HDR tone mapping, and motion interpolation.

use anyhow::Result;
use tracing::{info, debug};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use decoding_optimization::{DecodingOptimizationConfig, VideoDecodingOptimizer};

pub mod decoder;
pub mod renderer;
pub mod upscaler;
pub mod tonemap;
pub mod frame;
pub mod decoding_optimization;

/// Video engine - manages all video processing
pub struct VideoEngine {
    /// WGPU device
    device: Device,
    
    /// WGPU queue
    queue: Queue,
    
    /// Surface
    surface: Surface<'static>,
    
    /// Configuration
    config: SurfaceConfiguration,
    
    /// Video decoder
    decoder: decoder::VideoDecoder,
    
    /// Video renderer
    renderer: renderer::VideoRenderer,
    
    /// AI upscaler
    upscaler: Option<upscaler::AIUpscaler>,
    
    /// HDR tone mapper
    tonemap: tonemap::ToneMapper,
    
    /// Decoding optimizer
    decoding_optimizer: Option<decoding_optimization::VideoDecodingOptimizer>,
}

impl VideoEngine {
    /// Create a new video engine (without surface)
    pub async fn new() -> Result<Self> {
        info!("🎬 Initializing Vantis Video Engine");
        
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable GPU adapter"))?;
        
        debug!("🔧 GPU Adapter: {:?}", adapter.get_info());
        
        // Request device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Vantis GPU Device"),
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;
        
        // Initialize components
        let decoder = decoder::VideoDecoder::new()?;
        let tonemap = tonemap::ToneMapper::new(&device)?;
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: 1920,
            height: 1080,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        
        let renderer = renderer::VideoRenderer::new(&device, &config)?;
        
        info!("✅ Video Engine initialized");
        info!("   - Hardware acceleration: Enabled");
        info!("   - HDR support: Enabled");
        info!("   - AI Upscaling: Available");
        
        Ok(Self {
            device,
            queue,
            surface: unsafe { std::mem::zeroed() }, // Placeholder, will be set later
            config,
            decoder,
            renderer,
            upscaler: None,
            tonemap,
            decoding_optimizer: None,
        })
    }
    
    /// Create a new video engine with surface
    pub async fn new_with_surface(surface: Surface<'static>) -> Result<Self> {
        info!("🎬 Initializing Vantis Video Engine with surface");
        
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable GPU adapter"))?;
        
        debug!("🔧 GPU Adapter: {:?}", adapter.get_info());
        
        // Request device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Vantis GPU Device"),
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;
        
        // Configure surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: 1920,
            height: 1080,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        
        surface.configure(&device, &config);
        
        // Initialize components
        let decoder = decoder::VideoDecoder::new()?;
        let renderer = renderer::VideoRenderer::new(&device, &config)?;
        let tonemap = tonemap::ToneMapper::new(&device)?;
        
        info!("✅ Video Engine initialized");
        info!("   - Hardware acceleration: Enabled");
        info!("   - HDR support: Enabled");
        info!("   - AI Upscaling: Available");
        
        Ok(Self {
            device,
            queue,
            surface,
            config,
            decoder,
            renderer,
            upscaler: None,
            tonemap,
            decoding_optimizer: None,
        })
    }
    
    /// Load a video file
    pub async fn load_video(&mut self, path: &str) -> Result<()> {
        info!("📂 Loading video: {}", path);
        self.decoder.load(path).await?;
        Ok(())
    }
    
    /// Get next frame
    pub async fn next_frame(&mut self) -> Result<Option<frame::VideoFrame>> {
        self.decoder.next_frame().await
    }
    
    /// Render a frame to the surface
    pub fn render_frame(&mut self, frame: &frame::VideoFrame) -> Result<()> {
        self.renderer.render(&self.device, &self.queue, &mut self.surface, frame)
    }
    
    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
    
    /// Initialize decoding optimizer
    pub fn init_decoding_optimizer(&mut self, config: DecodingOptimizationConfig) -> Result<()> {
        info!("Initializing decoding optimizer");
        let optimizer = VideoDecodingOptimizer::new(config);
        self.decoding_optimizer = Some(optimizer);
        Ok(())
    }
    
    /// Get decoding optimizer
    pub fn decoding_optimizer(&self) -> Option<&VideoDecodingOptimizer> {
        self.decoding_optimizer.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_video_engine_creation() {
        let engine = VideoEngine::new().await;
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_video_frame_creation() {
        let frame = frame::VideoFrame::new(1920, 1080);
        assert_eq!(frame.width(), 1920);
        assert_eq!(frame.height(), 1080);
    }
}