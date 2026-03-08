//! GPU-accelerated video processing using wgpu
//!
//! This module provides GPU-accelerated implementations of video filters
//! for significantly improved performance on supported hardware.

use std::sync::Arc;
use wgpu::*;
use bytemuck::{Pod, Zeroable};
use crate::{Frame, FilterResult, FilterError, PixelFormat};

/// GPU device and queue wrapper
pub struct GpuContext {
    device: Device,
    queue: Queue,
}

impl GpuContext {
    /// Create a new GPU context
    pub async fn new() -> FilterResult<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| FilterError::GpuError(format!("Failed to get adapter: {:?}", e)))?;
        
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Vantis GPU Device"),
                features: Features::empty(),
                limits: Limits::default(),
            }, None)
            .await
            .map_err(|e| FilterError::GpuError(format!("Failed to get device: {:?}", e)))?;
        
        Ok(Self { device, queue })
    }
    
    /// Get the device
    pub fn device(&self) -> &Device {
        &self.device
    }
    
    /// Get the queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

/// GPU texture for frame data
pub struct GpuTexture {
    texture: Texture,
    view: TextureView,
    sampler: Sampler,
    width: u32,
    height: u32,
}

impl GpuTexture {
    /// Create a new GPU texture
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Frame Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        
        let view = texture.create_view(&TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        
        Self { texture, view, sampler, width, height }
    }
    
    /// Upload frame data to the texture
    pub fn upload(&self, queue: &Queue, frame: &Frame) {
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &frame.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.width),
                rows_per_image: Some(self.height),
            },
            Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
    }
    
    /// Download texture data to a frame
    pub async fn download(&self, device: &Device, queue: &Queue) -> FilterResult<Frame> {
        let bytes_per_pixel = 4u32;
        let unpadded_bytes_per_row = self.width * bytes_per_pixel;
        let align = COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        
        let buffer_size = (padded_bytes_per_row * self.height) as u64;
        
        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Output Buffer"),
            size: buffer_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Download Encoder"),
        });
        
        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            ImageCopyBuffer {
                buffer: &output_buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        
        queue.submit(Some(encoder.finish()));
        
        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = futures::channel::oneshot::channel();
        buffer_slice.map_async(MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(Maintain::Wait);
        
        rx.await
            .map_err(|e| FilterError::GpuError(format!("Failed to receive mapping result: {:?}", e)))?
            .map_err(|e| FilterError::GpuError(format!("Failed to map buffer: {:?}", e)))?;
        
        let data = buffer_slice.get_mapped_range();
        let mut frame_data = Vec::with_capacity((self.width * self.height * 4) as usize);
        
        for chunk in data.chunks(padded_bytes_per_row as usize) {
            frame_data.extend_from_slice(&chunk[..unpadded_bytes_per_row as usize]);
        }
        
        drop(data);
        output_buffer.unmap();
        
        Frame::from_data(self.width, self.height, PixelFormat::RGBA, frame_data)
            .ok_or_else(|| FilterError::GpuError("Failed to create frame from data".to_string()))
    }
    
    /// Get the texture view
    pub fn view(&self) -> &TextureView {
        &self.view
    }
    
    /// Get the sampler
    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

/// Compute shader for GPU filters
pub struct GpuFilter {
    pipeline: ComputePipeline,
    bind_group_layout: BindGroupLayout,
}

impl GpuFilter {
    /// Create a GPU filter from WGSL shader code
    pub fn new(device: &Device, shader_source: &str, entry_point: &str) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Filter Shader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });
        
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Filter Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Filter Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Filter Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some(entry_point),
            compilation_options: Default::default(),
            cache: None,
        });
        
        Self { pipeline, bind_group_layout }
    }
    
    /// Process a frame using the GPU
    pub async fn process(
        &self,
        device: &Device,
        queue: &Queue,
        input: &GpuTexture,
        output: &GpuTexture,
        params: &[f32],
    ) -> FilterResult<()> {
        let param_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Parameter Buffer"),
            contents: bytemuck::cast_slice(params),
            usage: BufferUsages::UNIFORM,
        });
        
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Filter Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(input.view()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(output.view()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(param_buffer.as_entire_buffer_binding()),
                },
            ],
        });
        
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Filter Encoder"),
        });
        
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Filter Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(
                (input.width + 15) / 16,
                (input.height + 15) / 16,
                1,
            );
        }
        
        queue.submit(Some(encoder.finish()));
        
        Ok(())
    }
}

/// Common GPU filter shaders
pub mod shaders {
    /// Brightness/Contrast shader
    pub const BRIGHTNESS_CONTRAST: &str = r#"
@group(0) @binding(0) var input_tex: texture_2d<f32>;
@group(0) @binding(1) var output_tex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> params: vec4<f32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_tex);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    let color = textureLoad(input_tex, vec2<i32>(global_id.xy), 0);
    
    let brightness = params.x;
    let contrast = params.y;
    
    var adjusted = (color.rgb - 0.5) * contrast + 0.5 + brightness;
    adjusted = clamp(adjusted, vec3<f32>(0.0), vec3<f32>(1.0));
    
    textureStore(output_tex, vec2<i32>(global_id.xy), vec4<f32>(adjusted, color.a));
}
"#;
    
    /// Saturation shader
    pub const SATURATION: &str = r#"
@group(0) @binding(0) var input_tex: texture_2d<f32>;
@group(0) @binding(1) var output_tex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> params: vec4<f32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_tex);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    let color = textureLoad(input_tex, vec2<i32>(global_id.xy), 0);
    let saturation = params.x;
    
    let gray = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let adjusted = mix(vec3<f32>(gray), color.rgb, saturation);
    
    textureStore(output_tex, vec2<i32>(global_id.xy), vec4<f32>(adjusted, color.a));
}
"#;
    
    /// Gaussian blur shader (horizontal pass)
    pub const GAUSSIAN_BLUR_H: &str = r#"
@group(0) @binding(0) var input_tex: texture_2d<f32>;
@group(0) @binding(1) var output_tex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> params: vec4<f32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_tex);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    let radius = int(params.x);
    let sigma = params.y;
    
    var sum = vec3<f32>(0.0);
    var weight_sum = 0.0;
    
    for (var i = -radius; i <= radius; i++) {
        let x = clamp(int(global_id.x) + i, 0, int(dims.x) - 1);
        let weight = exp(-f32(i * i) / (2.0 * sigma * sigma));
        let sample = textureLoad(input_tex, vec2<i32>(x, int(global_id.y)), 0);
        sum += sample.rgb * weight;
        weight_sum += weight;
    }
    
    let result = sum / weight_sum;
    let alpha = textureLoad(input_tex, vec2<i32>(global_id.xy), 0).a;
    
    textureStore(output_tex, vec2<i32>(global_id.xy), vec4<f32>(result, alpha));
}
"#;
    
    /// Gaussian blur shader (vertical pass)
    pub const GAUSSIAN_BLUR_V: &str = r#"
@group(0) @binding(0) var input_tex: texture_2d<f32>;
@group(0) @binding(1) var output_tex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> params: vec4<f32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_tex);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    let radius = int(params.x);
    let sigma = params.y;
    
    var sum = vec3<f32>(0.0);
    var weight_sum = 0.0;
    
    for (var i = -radius; i <= radius; i++) {
        let y = clamp(int(global_id.y) + i, 0, int(dims.y) - 1);
        let weight = exp(-f32(i * i) / (2.0 * sigma * sigma));
        let sample = textureLoad(input_tex, vec2<i32>(int(global_id.x), y), 0);
        sum += sample.rgb * weight;
        weight_sum += weight;
    }
    
    let result = sum / weight_sum;
    let alpha = textureLoad(input_tex, vec2<i32>(global_id.xy), 0).a;
    
    textureStore(output_tex, vec2<i32>(global_id.xy), vec4<f32>(result, alpha));
}
"#;
}

/// GPU-accelerated filter processor
pub struct GpuProcessor {
    context: GpuContext,
    filters: Vec<GpuFilter>,
}

impl GpuProcessor {
    /// Create a new GPU processor
    pub async fn new() -> FilterResult<Self> {
        let context = GpuContext::new().await?;
        Ok(Self {
            context,
            filters: Vec::new(),
        })
    }
    
    /// Add a filter
    pub fn add_filter(&mut self, shader: &str, entry_point: &str) {
        let filter = GpuFilter::new(self.context.device(), shader, entry_point);
        self.filters.push(filter);
    }
    
    /// Process a frame
    pub async fn process(&self, frame: &Frame) -> FilterResult<Frame> {
        let device = self.context.device();
        let queue = self.context.queue();
        
        let input_texture = GpuTexture::new(device, frame.width, frame.height);
        let output_texture = GpuTexture::new(device, frame.width, frame.height);
        
        input_texture.upload(queue, frame);
        
        // Process through filter chain
        let mut current_input = &input_texture;
        let mut current_output = &output_texture;
        
        for filter in &self.filters {
            filter.process(device, queue, current_input, current_output, &[0.0]).await?;
            std::mem::swap(&mut current_input, &mut current_output);
        }
        
        current_input.download(device, queue).await
    }
}