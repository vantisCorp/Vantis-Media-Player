---
sidebar_position: 5
---

# Performance

Optimize Vantis Media Player for maximum performance across all platforms and use cases.

## Performance Overview

Vantis Media Player is designed for optimal performance with:

- **Hardware acceleration** for decoding and processing
- **Efficient memory management** with smart caching
- **Multi-threaded processing** for smooth playback
- **Adaptive quality** based on network and system conditions
- **Zero-copy rendering** where possible

## Hardware Acceleration

### Enable Hardware Decoding

```toml
[player]
hardware_acceleration = true
```

```javascript
const player = new Player('#container', {
    hardwareAcceleration: true
});
```

### GPU Acceleration

```toml
[performance]
gpu_acceleration = true
zero_copy = true
gpu_memory = true
```

```javascript
player.setGPUAcceleration(true);
player.setZeroCopyRendering(true);
```

See [Hardware Acceleration](./hardware-acceleration) for detailed information.

## Memory Management

### Cache Configuration

```toml
[performance]
cache_size = 512  # MB
video_cache = 256  # MB
audio_cache = 64  # MB
metadata_cache = 32  # MB
thumbnail_cache = 128  # MB
```

```javascript
player.setCacheConfig({
    video: 256 * 1024 * 1024,      // 256 MB
    audio: 64 * 1024 * 1024,       // 64 MB
    metadata: 32 * 1024 * 1024,    // 32 MB
    thumbnails: 128 * 1024 * 1024  // 128 MB
});
```

### Memory Limits

```toml
[performance]
max_memory_mb = 2048
memory_threshold = 0.9  # 90% of max
```

```javascript
player.setMaxMemory(2048 * 1024 * 1024);  // 2 GB
```

### Automatic Cache Management

```javascript
// Enable automatic cache clearing
player.setAutoCacheClear(true);

// Set cache clear threshold
player.setCacheThreshold(0.8);  // Clear when 80% full

// Clear cache manually
player.clearCache();
player.clearVideoCache();
player.clearAudioCache();
```

## Threading

### Thread Configuration

```toml
[performance]
thread_count = 0  # 0 = auto
decoder_threads = 0
processing_threads = 0
```

```javascript
player.setThreadConfig({
    decoder: 0,      // 0 = auto
    processing: 4,   // 4 threads
    total: 8         // 8 threads total
});
```

### Optimal Thread Counts

| System Type | CPU Cores | Recommended Threads |
|-------------|-----------|---------------------|
| Low-end | 2 | 2-3 |
| Mid-range | 4-6 | 4-8 |
| High-end | 8+ | 8-16 |

## Buffer Management

### Buffer Configuration

```toml
[performance]
buffer_size = 32768
max_buffer_length = 60
min_buffer_length = 5
back_buffer_length = 30
```

```javascript
player.setBufferConfig({
    maxSize: 50 * 1024 * 1024,    // 50 MB
    maxDuration: 60,               // 60 seconds
    minDuration: 5,                // 5 seconds
    backDuration: 30               // 30 seconds behind
});
```

### Buffer Strategy

```javascript
// Adaptive buffering
player.setBufferStrategy({
    algorithm: 'adaptive',
    targetBuffer: 30,      // seconds
    maxBuffer: 60,
    lowBuffer: 5
});

// Fixed buffering
player.setBufferStrategy({
    algorithm: 'fixed',
    bufferSize: 30
});
```

## Rendering Optimization

### Frame Rate Control

```javascript
// Set target frame rate
player.setFrameRate(60);

// Adaptive frame rate
player.setAdaptiveFrameRate(true);

// Limit frame rate
player.setMaxFrameRate(60);
```

### Resolution Scaling

```javascript
// Set max resolution
player.setMaxResolution('1080p');

// Adaptive resolution
player.setAdaptiveResolution(true);

// Resolution options
player.setMaxResolution('4K');      // 3840x2160
player.setMaxResolution('1080p');  // 1920x1080
player.setMaxResolution('720p');   // 1280x720
```

### Render Quality

```toml
[performance]
render_quality = "high"
antialiasing = true
vsync = true
```

```javascript
player.setRenderQuality('high');  // low, medium, high, ultra
player.setAntialiasing(true);
player.setVSync(true);
```

## Network Optimization

### Adaptive Bitrate

```toml
[streaming]
adaptive_bitrate = true
algorithm = "hybrid"
min_bitrate = 500000
max_bitrate = 10000000
```

See [Streaming](./streaming) for detailed ABR configuration.

### Connection Optimization

```toml
[network]
max_connections = 4
timeout = 30
keep_alive = true
```

```javascript
player.setNetworkConfig({
    maxConnections: 4,
    timeout: 30000,
    keepAlive: true
});
```

### Preloading

```javascript
// Preload next item in playlist
player.enablePreload(true);

// Preload buffer duration
player.setPreloadDuration(30);  // seconds

// Preload on hover (playlist)
player.setPreloadOnHover(true);
```

## Codecs and Formats

### Optimized Codecs

Prioritize efficient codecs:

| Codec | Quality | Efficiency | Hardware Support |
|-------|---------|------------|------------------|
| **H.264** | High | Good | Excellent |
| **H.265/HEVC** | Very High | Very Good | Good |
| **VP9** | High | Very Good | Good |
| **AV1** | Excellent | Excellent | Improving |
| **VP8** | Medium | Good | Good |

### Codec Selection

```javascript
// Set preferred codecs
player.setPreferredCodecs({
    video: ['av1', 'vp9', 'h265', 'h264'],
    audio: ['opus', 'aac', 'mp3']
});
```

## Disk I/O Optimization

### Fast Storage

```toml
[performance]
cache_location = "/fast/ssd/cache"
temp_location = "/fast/ssd/temp"
```

```javascript
player.setStoragePaths({
    cache: '/fast/ssd/cache',
    temp: '/fast/ssd/temp'
});
```

### Async I/O

```toml
[performance]
async_io = true
io_threads = 4
```

```javascript
player.setAsyncIO(true, 4);
```

## GPU Memory Management

### GPU Memory Limits

```toml
[performance]
gpu_memory_limit = 2048  # MB
gpu_cache_size = 512  # MB
```

```javascript
player.setGPUMemoryLimit(2048 * 1024 * 1024);  // 2 GB
```

### Texture Management

```javascript
// Reduce texture resolution
player.setTextureResolution('half');  // full, half, quarter

// Compress textures
player.setTextureCompression(true);

// Clear unused textures
player.clearUnusedTextures();
```

## Battery Optimization

### Low Power Mode

```javascript
// Enable low power mode
player.setLowPowerMode(true);
```

Low power mode:
- Reduces frame rate
- Disables effects
- Uses software decoding
- Reduces buffer size

### Power-aware Playback

```javascript
// Monitor battery level
player.on('batteryChange', (level) => {
    if (level < 20) {
        player.setLowPowerMode(true);
    } else if (level > 50) {
        player.setLowPowerMode(false);
    }
});
```

## Performance Monitoring

### Metrics

```javascript
// Get performance metrics
const metrics = player.getPerformanceMetrics();
console.log(metrics);
// {
//   cpuUsage: 25.5,  // percentage
//   memoryUsage: 512,  // MB
//   gpuUsage: 30,  // percentage
//   gpuMemory: 256,  // MB
//   frameRate: 60,
//   droppedFrames: 0,
//   decodeTime: 8.5,  // ms
//   renderTime: 4.2,  // ms
//   bufferHealth: 85  // percentage
// }
```

### Performance Events

```javascript
// Monitor performance
player.on('performanceUpdate', (metrics) => {
    console.log('CPU:', metrics.cpuUsage + '%');
    console.log('Memory:', metrics.memoryUsage + 'MB');
    console.log('FPS:', metrics.frameRate);
});

// Warn on high CPU
player.on('highCpuUsage', (usage) => {
    console.warn(`High CPU usage: ${usage}%`);
});

// Warn on dropped frames
player.on('droppedFrames', (count) => {
    console.warn(`Dropped ${count} frames`);
});
```

### Profiling

```bash
# Enable profiling
vantismedia --profile performance.mp4

# Profile with statistics
vantismedia --profile-stats performance.mp4

# Output performance report
vantismedia --profile-output report.json performance.mp4
```

## Troubleshooting Performance Issues

### High CPU Usage

```toml
[performance]
hardware_acceleration = true
thread_count = 4
```

```javascript
// Enable hardware acceleration
player.setHardwareAcceleration(true);

// Reduce thread count
player.setThreadCount(4);

// Disable effects
player.disableEffects();
```

### Stuttering/Frame Drops

```toml
[performance]
buffer_size = 65536
cache_size = 1024
```

```javascript
// Increase buffer
player.setBufferConfig({
    maxDuration: 120,
    maxSize: 100 * 1024 * 1024
});

// Enable hardware acceleration
player.setHardwareAcceleration(true);
```

### High Memory Usage

```javascript
// Clear cache
player.clearCache();

// Reduce cache size
player.setCacheConfig({
    video: 128 * 1024 * 1024,  // 128 MB
    audio: 32 * 1024 * 1024    // 32 MB
});

// Disable preloading
player.enablePreload(false);
```

### Slow Startup

```javascript
// Lazy load plugins
player.setLazyPluginLoading(true);

// Disable auto-scan
player.setAutoScan(false);

// Preload on demand
player.preloadOnDemand();
```

## Performance Best Practices

### For Desktop

1. **Enable hardware acceleration** whenever possible
2. **Use SSD for cache** for faster I/O
3. **Configure appropriate thread count** based on CPU
4. **Enable GPU memory** for faster rendering

### For Mobile

1. **Enable low power mode** to save battery
2. **Reduce cache size** to save memory
3. **Use adaptive resolution** for network conditions
4. **Limit frame rate** to 30-60 FPS

### For Web

1. **Use CDN for streaming** for better performance
2. **Enable adaptive bitrate** for smooth playback
3. **Preload strategically** to balance speed and memory
4. **Use Web Workers** for off-main-thread processing

### For Streaming

1. **Use appropriate buffer size** based on network
2. **Enable adaptive bitrate** for quality adaptation
3. **Use hardware acceleration** for decoding
4. **Monitor network conditions** and adjust accordingly

## Performance Benchmarks

### System Requirements

| Resolution | CPU | RAM | GPU |
|------------|-----|-----|-----|
| 720p | Dual-core 2GHz | 2GB | Integrated |
| 1080p | Quad-core 2.5GHz | 4GB | Dedicated 1GB |
| 4K | Quad-core 3GHz+ | 8GB | Dedicated 2GB+ |
| 8K | Octa-core 3.5GHz+ | 16GB | Dedicated 4GB+ |

### Expected Performance

| Resolution | CPU Usage | GPU Usage | Memory |
|------------|-----------|-----------|--------|
| 720p | 5-10% | 10-20% | 256MB |
| 1080p | 10-20% | 20-40% | 512MB |
| 4K | 20-40% | 40-60% | 1GB |
| 8K | 40-60% | 60-80% | 2GB |

## Next Steps

- **[Hardware Acceleration](./hardware-acceleration)** - Detailed GPU optimization
- **[Streaming](./streaming)** - Network performance
- **[Custom UI](./custom-ui)** - UI performance tips

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)