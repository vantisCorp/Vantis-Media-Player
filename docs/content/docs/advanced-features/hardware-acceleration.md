---
sidebar_position: 1
---

# Hardware Acceleration

Vantis Media Player leverages hardware acceleration to provide optimal performance and minimal resource usage during media playback.

## Overview

Hardware acceleration offloads video decoding and processing from the CPU to specialized hardware, providing:

- **Lower CPU usage** - Free up system resources
- **Better battery life** - Essential for laptops and mobile devices
- **Higher quality playback** - Supports higher resolutions and frame rates
- **Smoother performance** - Reduce stuttering and frame drops

## Supported Hardware Accelerators

### Windows

| API | Description | Supported Codecs |
|-----|-------------|------------------|
| **D3D11VA** | DirectX 11 Video Acceleration | H.264, H.265, VP9, AV1 |
| **DXVA2** | DirectX Video Acceleration 2.0 | H.264, H.265 |
| **NVDEC** | NVIDIA Decoder | H.264, H.265, VP9, AV1 |
| **QSV** | Intel Quick Sync Video | H.264, H.265, VP9 |
| **AMF** | AMD Advanced Media Framework | H.264, H.265, VP9 |

### macOS

| API | Description | Supported Codecs |
|-----|-------------|------------------|
| **VideoToolbox** | Apple Hardware Acceleration | H.264, H.265, VP9, ProRes |
| **Metal** | GPU Compute | Post-processing, filters |

### Linux

| API | Description | Supported Codecs |
|-----|-------------|------------------|
| **VAAPI** | Video Acceleration API (Intel, AMD) | H.264, H.265, VP9, AV1 |
| **VDPAU** | Video Decode and Presentation API (NVIDIA) | H.264, H.265 |
| **NVDEC** | NVIDIA Decoder | H.264, H.265, VP9, AV1 |
| **V4L2** | Video4Linux2 (Hardware-dependent) | Various |

## Enabling Hardware Acceleration

### Configuration File

```toml
[player]
hardware_acceleration = true
video_decoder = "auto"
```

### Command Line

```bash
# Enable hardware acceleration
vantismedia --hwaccel video.mp4

# Use specific decoder
vantismedia --decoder d3d11va video.mp4
vantismedia --decoder vaapi video.mp4
vantismedia --decoder videotoolbox video.mp4

# Disable hardware acceleration
vantismedia --no-hwaccel video.mp4
```

### API

```javascript
const player = new Player('#container', {
    hardwareAcceleration: true,
    decoder: 'auto'
});
```

```rust
let config = PlayerConfig {
    hardware_acceleration: true,
    decoder: Decoder::Auto,
    ..Default::default()
};
let player = Player::new(config)?;
```

## Automatic Decoder Selection

Vantis Media Player automatically selects the best available hardware decoder:

1. **Query available decoders** based on system hardware
2. **Check codec support** for the video being played
3. **Select optimal decoder** based on performance and compatibility
4. **Fallback to software** if hardware decoding fails

```javascript
// Get available decoders
const decoders = player.getAvailableDecoders();
console.log(decoders);
// ['d3d11va', 'nvdec', 'qsv', 'software']

// Get current decoder
const current = player.getCurrentDecoder();
console.log(current); // 'd3d11va'
```

## Zero-Copy Rendering

Zero-copy rendering minimizes memory copies between GPU and CPU:

```toml
[performance]
zero_copy = true
gpu_memory = true
```

```javascript
player.setZeroCopyRendering(true);
```

## Hardware Decoding Errors

When hardware decoding fails, Vantis Media Player automatically falls back to software decoding:

```javascript
player.on('decoderFallback', (event) => {
    console.log(`Failed to use ${event.decoder}, falling back to software`);
    console.log(`Reason: ${event.reason}`);
});
```

### Common Issues

**Issue**: "Hardware decoder not found"  
**Solution**: Ensure your GPU supports the video codec

**Issue**: "Hardware acceleration disabled"  
**Solution**: Check GPU drivers are installed correctly

**Issue**: "Decoder initialization failed"  
**Solution**: Update GPU drivers to the latest version

## Performance Monitoring

### GPU Usage

```javascript
// Get GPU usage statistics
const gpuStats = player.getGPUStats();
console.log(gpuStats);
// {
//   decoder: 'd3d11va',
//   gpuUsage: 45,  // percentage
//   memoryUsage: 512,  // MB
//   temperature: 65  // Celsius
// }
```

### Hardware Info

```bash
# Check hardware acceleration support
vantismedia --check-hwaccel

# Output example:
# Hardware Accelerators Available:
#   D3D11VA: NVIDIA GeForce RTX 3080
#     - H.264: ✓
#     - H.265: ✓
#     - VP9: ✓
#     - AV1: ✓
#   NVDEC: NVIDIA GeForce RTX 3080
#     - H.264: ✓
#     - H.265: ✓
#     - VP9: ✓
#     - AV1: ✓
#   QSV: Intel Quick Sync Video
#     - H.264: ✓
#     - H.265: ✓
```

## HDR and Color Management

### HDR Support

Hardware acceleration enables HDR playback:

```javascript
// Check HDR support
const hdrSupport = player.getHDRSupport();
console.log(hdrSupport);
// {
//   hdr10: true,
//   hdr10plus: true,
//   dolbyVision: false,
//   hlg: true
// }

// Enable HDR
player.setHDRMode('auto');
```

### Color Space Conversion

Hardware-accelerated color space conversion:

```javascript
// Set color space
player.setColorSpace('bt2020');
player.setTransferFunction('pq');  // HDR
player.setColorPrimaries('bt2020');
```

## Post-Processing

### Hardware-Accelerated Filters

```javascript
// Enable hardware-accelerated deinterlacing
player.setHardwareDeinterlace('yadif');

// Hardware scaling
player.setHardwareScaling('bicubic');

// Noise reduction
player.setHardwareNoiseReduction(10);
```

## Memory Management

### GPU Memory

```toml
[performance]
gpu_memory_limit = 2048  # MB
gpu_cache_size = 512  # MB
```

```javascript
// Get GPU memory info
const memInfo = player.getGPUMemoryInfo();
console.log(memInfo);
// {
//   total: 10240,  // MB
//   used: 2048,    // MB
//   available: 8192  // MB
// }
```

## Multi-GPU Systems

### GPU Selection

```javascript
// List available GPUs
const gpus = player.getAvailableGPUs();
console.log(gpus);
// [
//   { id: 0, name: 'NVIDIA RTX 3080', type: 'discrete' },
//   { id: 1, name: 'Intel UHD Graphics', type: 'integrated' }
// ]

// Select GPU for decoding
player.setGPU(0);  // Use RTX 3080
```

### Configuration

```toml
[performance]
preferred_gpu = 0
auto_gpu_selection = true
```

## Platform-Specific Configuration

### Windows (D3D11VA)

```toml
[decoder.d3d11va]
adapter = 0
feature_level = "11_0"
```

### macOS (VideoToolbox)

```toml
[decoder.videotoolbox]
realtime = true
allow_pixel_format_conversion = true
```

### Linux (VAAPI)

```toml
[decoder.vaapi]
device = "/dev/dri/renderD128"
driver = "i965"  # or "iHD" for newer Intel GPUs
```

### NVIDIA (NVDEC)

```toml
[decoder.nvdec]
gpu_index = 0
cuda_device = 0
```

## Troubleshooting

### Check Hardware Acceleration Status

```bash
# Verbose output with decoder info
vantismedia --verbose video.mp4

# Check decoder being used
vantismedia --info video.mp4 | grep "Decoder"
```

### Debug Mode

```bash
# Enable debug logging
vantismedia --debug video.mp4

# Check for hardware acceleration messages
vantismedia --debug video.mp4 2>&1 | grep -i hwaccel
```

### Force Specific Decoder

```bash
# Force D3D11VA on Windows
vantismedia --decoder d3d11va --verbose video.mp4

# Force VAAPI on Linux
vantismedia --decoder vaapi --verbose video.mp4

# Force VideoToolbox on macOS
vantismedia --decoder videotoolbox --verbose video.mp4
```

## Performance Comparison

| Scenario | Software Decoding | Hardware Decoding |
|----------|------------------|-------------------|
| 4K H.264 | 80% CPU | 5% CPU, 15% GPU |
| 4K H.265 | 95% CPU | 8% CPU, 25% GPU |
| 8K H.265 | 100% CPU (stuttering) | 15% CPU, 40% GPU |
| 4K AV1 | 100% CPU | 10% CPU, 35% GPU |

## Next Steps

- **[Streaming](./streaming)** - Streaming capabilities
- **[Plugins](./plugins)** - Plugin system
- **[Performance](./performance)** - Performance optimization

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)