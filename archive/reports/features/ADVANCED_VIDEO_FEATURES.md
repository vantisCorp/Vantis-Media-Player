# Vantis Advanced Video Features

## Overview

The Vantis Advanced Video module provides cutting-edge video processing capabilities for the Vantis Media Player. This module implements professional-grade video features including stabilization, frame interpolation, denoising, color grading, and video comparison tools.

## Features

### 1. Video Stabilization Engine

The video stabilization system reduces camera shake and produces smooth, professional-looking video footage.

#### Capabilities

- **Motion Analysis**: Multiple motion estimation methods (Optical Flow, Feature Tracking, Global Motion, Hybrid)
- **Camera Path Smoothing**: Smooth camera movement while preserving intentional motion
- **Rolling Shutter Correction**: Compensates for rolling shutter artifacts
- **Crop-to-Stabilize**: Optional cropping to maintain full stabilization
- **Adjustable Strength**: Fine-tune stabilization intensity

#### Motion Analysis Methods

- **Optical Flow**: Pixel-level motion estimation using optical flow algorithms
- **Feature Tracking**: Track feature points across frames
- **Global Motion**: Estimate global camera motion
- **Hybrid**: Combine multiple methods for best results

#### Usage Example

```rust
use vantis_advanced_video::{AdvancedVideoEngine, AdvancedVideoConfig, StabilizationConfig};

// Create engine with stabilization enabled
let mut config = AdvancedVideoConfig::default();
config.stabilization.enabled = true;
config.stabilization.strength = 0.7;
config.stabilization.motion_analysis = MotionAnalysisMethod::OpticalFlow;

let engine = AdvancedVideoEngine::new(config)?;

// Stabilize video frames
for frame in video_frames {
    let stabilized = engine.stabilizer().stabilize(&frame).await?;
    // Use stabilized frame
}
```

#### Configuration Options

- `enabled`: Enable/disable stabilization
- `strength`: Stabilization strength (0.0 - 1.0)
- `motion_analysis`: Motion estimation method
- `rolling_shutter_correction`: Enable rolling shutter correction
- `crop_to_stabilize`: Crop to maintain full stabilization
- `max_crop`: Maximum crop percentage (0.0 - 1.0)

---

### 2. Frame Interpolation Engine

Generate intermediate frames to increase video frame rate for smoother playback.

#### Capabilities

- **Multiple Interpolation Methods**: Linear, Motion-Compensated, AI-based, Optical Flow
- **Target FPS Control**: Convert to any target frame rate
- **Quality vs Speed**: Adjustable quality/speed tradeoff
- **AI Models**: Support for AI-based interpolation models

#### Interpolation Methods

- **Linear**: Simple linear interpolation (fast, lower quality)
- **Motion-Compensated**: Motion-aware interpolation (better quality)
- **AI**: Deep learning-based interpolation (best quality, slower)
- **Optical Flow**: Optical flow-based interpolation (good balance)

#### Usage Example

```rust
// Set frame interpolation to 60 FPS
config.frame_interpolation.enabled = true;
config.frame_interpolation.target_fps = 60;
config.frame_interpolation.method = InterpolationMethod::AI;
config.frame_interpolation.quality = 0.8;

let engine = AdvancedVideoEngine::new(config)?;

// Interpolate between frames
let frame1 = load_frame(0)?;
let frame2 = load_frame(1)?;
let result = engine.interpolator().interpolate(&frame1, &frame2, 2).await?;

// result.frames contains 2 interpolated frames
```

#### Configuration Options

- `enabled`: Enable/disable frame interpolation
- `target_fps`: Target frame rate
- `method`: Interpolation method
- `ai_model`: AI model to use (for AI method)
- `quality`: Quality vs speed tradeoff (0.0 - 1.0)

---

### 3. Video Denoising Engine

Remove noise from video footage while preserving detail and edges.

#### Capabilities

- **Multiple Denoising Methods**: Bilateral, Non-Local Means, Wavelet, AI
- **Spatial Denoising**: Remove noise within individual frames
- **Temporal Denoising**: Remove noise across multiple frames
- **Adjustable Strength**: Fine-tune denoising intensity
- **Detail Preservation**: Preserve edges and fine details

#### Denoising Methods

- **Bilateral**: Edge-preserving smoothing
- **Non-Local Means**: Advanced denoising with self-similarity
- **Wavelet**: Frequency-domain denoising
- **AI**: Deep learning-based denoising (best quality)

#### Usage Example

```rust
// Enable video denoising
config.denoising.enabled = true;
config.denoising.method = DenoisingMethod::AI;
config.denoising.strength = 0.6;
config.denoising.spatial_denoising = true;
config.denoising.temporal_denoising = true;
config.denoising.detail_preservation = 0.8;

let engine = AdvancedVideoEngine::new(config)?;

// Denoise video frames
for frame in video_frames {
    let denoised = engine.denoiser().process(&frame).await?;
    // Use denoised frame
}
```

#### Configuration Options

- `enabled`: Enable/disable denoising
- `method`: Denoising method
- `strength`: Denoising strength (0.0 - 1.0)
- `spatial_denoising`: Enable spatial denoising
- `temporal_denoising`: Enable temporal denoising
- `detail_preservation`: Detail preservation level (0.0 - 1.0)

---

### 4. Color Grading Engine

Apply professional color grading with presets and custom adjustments.

#### Capabilities

- **Built-in Presets**: Neutral, Cinematic, Vivid, Warm, Cool, B&W, Vintage
- **Custom Adjustments**: Full control over all color parameters
- **LUT Support**: Load and apply custom LUTs (Look-Up Tables)
- **Real-time Preview**: See changes in real-time
- **Preset Management**: Save and load custom presets

#### Color Adjustments

- **Brightness**: Overall brightness adjustment
- **Contrast**: Contrast adjustment
- **Saturation**: Color saturation
- **Hue**: Hue shift
- **Temperature**: Color temperature (warm/cool)
- **Tint**: Green/magenta tint
- **Vibrance**: Vibrance (affects less saturated colors more)
- **Exposure**: Exposure adjustment
- **Highlights**: Highlight adjustment
- **Shadows**: Shadow adjustment
- **Whites**: White point adjustment
- **Blacks**: Black point adjustment

#### Built-in Presets

1. **Neutral**: No adjustments, original colors
2. **Cinematic**: Film-like color grading
3. **Vivid**: Enhanced saturation and contrast
4. **Warm**: Warm color temperature
5. **Cool**: Cool color temperature
6. **B&W**: Black and white conversion
7. **Vintage**: Vintage film look

#### Usage Example

```rust
// Apply cinematic preset
let graded = engine.color_grader().apply_preset(&frame, "cinematic").await?;

// Apply custom adjustments
let adjustments = ColorAdjustments {
    brightness: -0.05,
    contrast: 0.1,
    saturation: 0.9,
    temperature: -10.0,
    vibrance: 0.1,
    ..Default::default()
};

let graded = engine.color_grader().apply_adjustments(&frame, &adjustments).await?;

// Add custom preset
engine.color_grader().add_preset("my_preset".to_string(), adjustments).await?;
```

#### Configuration Options

- `enabled`: Enable/disable color grading
- `preset`: Default preset to apply
- `custom_adjustments`: Custom adjustments
- `lut_path`: Path to LUT file (optional)

---

### 5. Video Comparison Engine

Compare video frames for quality assessment and difference visualization.

#### Capabilities

- **Multiple Comparison Methods**: PSNR, SSIM, MSE, Visual Difference
- **Quality Metrics**: Calculate PSNR, SSIM, MSE
- **Difference Visualization**: Generate difference images
- **Similarity Scoring**: Overall similarity score
- **Batch Comparison**: Compare multiple frames

#### Comparison Methods

- **PSNR**: Peak Signal-to-Noise Ratio (dB)
- **SSIM**: Structural Similarity Index (0.0 - 1.0)
- **MSE**: Mean Squared Error
- **Visual Difference**: Visual difference score

#### Usage Example

```rust
// Compare two frames
let result = engine.comparator().compare(&frame1, &frame2).await?;

println!("Similarity: {:.2}", result.similarity);
println!("PSNR: {:.2} dB", result.psnr);
println!("SSIM: {:.4}", result.ssim);
println!("MSE: {:.2}", result.mse);

// Get difference image
if let Some(diff_image) = result.difference_image {
    // Save or display difference image
}
```

#### Metrics Explained

- **PSNR**: Higher is better (typical range: 20-50 dB)
- **SSIM**: Closer to 1.0 is better (range: 0.0 - 1.0)
- **MSE**: Lower is better
- **Similarity**: Overall similarity score (0.0 - 1.0)

#### Configuration Options

- `method`: Comparison method to use
- `generate_difference_image`: Generate difference image

---

## Performance Considerations

### GPU Acceleration

The Advanced Video module supports GPU acceleration through WGPU for:
- Video stabilization (motion analysis)
- Frame interpolation (AI models)
- Video denoising (spatial/temporal filtering)
- Color grading (pixel operations)

Enable GPU acceleration in configuration for best performance.

### Memory Usage

- **Stabilization**: Requires storing previous frames and motion vectors
- **Frame Interpolation**: Requires storing multiple frames for interpolation
- **Denoising**: Temporal denoising requires frame buffer
- **Color Grading**: Minimal memory overhead

### Processing Speed

- **Stabilization**: 10-30 FPS (depending on method and resolution)
- **Frame Interpolation**: 5-20 FPS (AI method is slower)
- **Denoising**: 15-40 FPS (depending on method)
- **Color Grading**: 30-60 FPS (real-time capable)
- **Comparison**: 20-50 FPS (depending on metrics)

---

## Best Practices

### Video Stabilization

1. Start with moderate strength (0.5-0.7)
2. Enable crop-to-stabilize for best results
3. Use Optical Flow or Hybrid method for best quality
4. Adjust strength based on footage shake amount

### Frame Interpolation

1. Use AI method for best quality
2. Set quality to 0.8-1.0 for high-quality output
3. Consider target FPS carefully (60 FPS is common)
4. Test different methods for your content

### Video Denoising

1. Start with moderate strength (0.5-0.7)
2. Enable both spatial and temporal denoising
3. Use AI method for best results
4. Adjust detail preservation to avoid over-smoothing

### Color Grading

1. Start with presets and fine-tune
2. Adjust exposure before other adjustments
3. Use subtle adjustments for natural look
4. Save custom presets for consistent look

### Video Comparison

1. Use SSIM for perceptual similarity
2. Use PSNR for technical quality assessment
3. Generate difference images for visual analysis
4. Compare multiple frames for accurate assessment

---

## Troubleshooting

### Stabilization Issues

**Problem**: Video still shaky after stabilization
- **Solution**: Increase stabilization strength
- **Solution**: Try different motion analysis method
- **Solution**: Enable rolling shutter correction

**Problem**: Video cropped too much
- **Solution**: Reduce max_crop setting
- **Solution**: Disable crop-to-stabilize
- **Solution**: Reduce stabilization strength

### Frame Interpolation Issues

**Problem**: Interpolated frames look blurry
- **Solution**: Increase quality setting
- **Solution**: Use AI or Optical Flow method
- **Solution**: Check source frame quality

**Problem**: Interpolation is too slow
- **Solution**: Use Linear or Motion-Compensated method
- **Solution**: Reduce quality setting
- **Solution**: Enable GPU acceleration

### Denoising Issues

**Problem**: Video looks over-smoothed
- **Solution**: Reduce denoising strength
- **Solution**: Increase detail preservation
- **Solution**: Try different denoising method

**Problem**: Noise still visible
- **Solution**: Increase denoising strength
- **Solution**: Enable temporal denoising
- **Solution**: Use AI method

### Color Grading Issues

**Problem**: Colors look unnatural
- **Solution**: Use smaller adjustment values
- **Solution**: Start with presets
- **Solution**: Reset to neutral and adjust gradually

**Problem**: Preset not found
- **Solution**: Check preset name spelling
- **Solution**: Use get_presets() to list available presets
- **Solution**: Create custom preset

---

## API Reference

### AdvancedVideoEngine

Main engine coordinating all advanced video features.

```rust
pub struct AdvancedVideoEngine {
    stabilizer: Arc<VideoStabilizer>,
    interpolator: Arc<FrameInterpolator>,
    denoiser: Arc<VideoDenoiser>,
    color_grader: Arc<ColorGrader>,
    comparator: Arc<VideoComparator>,
}
```

#### Methods

- `new(config: AdvancedVideoConfig) -> Result<Self>`
- `stabilizer(&self) -> &VideoStabilizer`
- `interpolator(&self) -> &FrameInterpolator`
- `denoiser(&self) -> &VideoDenoiser`
- `color_grader(&self) -> &ColorGrader`
- `comparator(&self) -> &VideoComparator`

### VideoStabilizer

Video stabilization engine.

```rust
pub struct VideoStabilizer {
    config: Arc<RwLock<StabilizationConfig>>,
    // ...
}
```

#### Methods

- `new() -> Result<Self>`
- `set_config(&self, config: StabilizationConfig) -> Result<()>`
- `get_config(&self) -> StabilizationConfig`
- `stabilize(&self, frame: &RgbImage) -> Result<StabilizationResult>`

### FrameInterpolator

Frame interpolation engine.

```rust
pub struct FrameInterpolator {
    config: Arc<RwLock<FrameInterpolationConfig>>,
    // ...
}
```

#### Methods

- `new() -> Result<Self>`
- `set_config(&self, config: FrameInterpolationConfig) -> Result<()>`
- `get_config(&self) -> FrameInterpolationConfig`
- `interpolate(&self, frame1: &RgbImage, frame2: &RgbImage, num_frames: usize) -> Result<InterpolationResult>`

### VideoDenoiser

Video denoising engine.

```rust
pub struct VideoDenoiser {
    config: Arc<RwLock<DenoisingConfig>>,
    // ...
}
```

#### Methods

- `new() -> Result<Self>`
- `set_config(&self, config: DenoisingConfig) -> Result<()>`
- `get_config(&self) -> DenoisingConfig`
- `process(&self, frame: &RgbImage) -> Result<RgbImage>`

### ColorGrader

Color grading engine.

```rust
pub struct ColorGrader {
    config: Arc<RwLock<ColorGradingConfig>>,
    presets: Arc<RwLock<HashMap<String, ColorAdjustments>>>,
    // ...
}
```

#### Methods

- `new() -> Result<Self>`
- `set_config(&self, config: ColorGradingConfig) -> Result<()>`
- `get_config(&self) -> ColorGradingConfig`
- `apply_preset(&self, frame: &RgbImage, preset_name: &str) -> Result<RgbImage>`
- `apply_adjustments(&self, frame: &RgbImage, adjustments: &ColorAdjustments) -> Result<RgbImage>`
- `add_preset(&self, name: String, adjustments: ColorAdjustments) -> Result<()>`
- `remove_preset(&self, name: &str) -> Result<bool>`
- `get_presets(&self) -> Vec<String>`
- `load_lut(&self, path: &str) -> Result<()>`

### VideoComparator

Video comparison engine.

```rust
pub struct VideoComparator {
    config: Arc<RwLock<ComparisonConfig>>,
    // ...
}
```

#### Methods

- `new() -> Result<Self>`
- `set_config(&self, config: ComparisonConfig) -> Result<()>`
- `get_config(&self) -> ComparisonConfig`
- `compare(&self, frame1: &RgbImage, frame2: &RgbImage) -> Result<ComparisonResult>`

---

## Examples

See `examples/advanced_video_example.rs` for comprehensive examples of all advanced video features.

---

## License

MIT License - See LICENSE file for details.