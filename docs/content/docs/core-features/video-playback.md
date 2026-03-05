---
sidebar_position: 1
---

# Video Playback

Vantis Media Player provides robust video playback capabilities with support for a wide range of formats and advanced features.

## Supported Formats

Vantis Media Player supports the following video formats:

### Container Formats

- **MP4** (MPEG-4 Part 14)
- **MKV** (Matroska)
- **WebM** (Web Media)
- **AVI** (Audio Video Interleave)
- **MOV** (QuickTime)
- **FLV** (Flash Video)
- **WMV** (Windows Media Video)
- **MPEG** (MPEG-1, MPEG-2)
- **TS** (MPEG Transport Stream)
- **M3U8** (HLS playlists)
- **MPD** (DASH manifests)

### Video Codecs

- **H.264/AVC** (Advanced Video Coding)
- **H.265/HEVC** (High Efficiency Video Coding)
- **VP8** (WebM Project)
- **VP9** (WebM Project)
- **AV1** (AOMedia Video 1)
- **MPEG-1** and **MPEG-2**
- **MJPEG**
- **Theora**

### HDR Support

- **HDR10**
- **HDR10+**
- **Dolby Vision**
- **HLG** (Hybrid Log-Gamma)

## Basic Video Playback

### Loading a Video

```javascript
const player = new Player('#container');
player.load('https://example.com/video.mp4');
player.play();
```

```rust
let mut player = Player::new()?;
player.load("video.mp4").await?;
player.play();
```

```bash
vantismedia video.mp4
```

## Hardware Acceleration

Vantis Media Player automatically uses hardware acceleration when available for optimal performance.

### Enabling Hardware Acceleration

```toml
# Configuration
[player]
hardware_acceleration = true
```

```javascript
const player = new Player('#container', {
    hardwareAcceleration: true
});
```

```bash
vantismedia --hwaccel video.mp4
```

### Supported Hardware Accelerators

- **Windows**: DXVA2, D3D11VA
- **macOS**: VideoToolbox (H.264, H.265, HEVC)
- **Linux**: VAAPI, VDPAU, NVDEC, V4L2

### Checking Hardware Acceleration

```bash
# Check available hardware decoders
vantismedia --check-hwaccel

# Show decoder information
vantismedia --info video.mp4
```

## Video Quality Control

### Resolution Selection

For adaptive streams (HLS/DASH), you can select quality:

```javascript
// Get available qualities
const qualities = player.getQualities();
console.log(qualities);

// Set quality
player.setQuality('1080p');
```

### Manual Quality Adjustment

```toml
[streaming]
max_bitrate = 10000  # 10 Mbps
adaptive_bitrate = true
```

## Frame Rate Control

### Frame Rate Adjustment

```javascript
// Set target frame rate
player.setFrameRate(60);

// Get current frame rate
const fps = player.getFrameRate();
```

### Motion Interpolation

```toml
[video]
interpolation = false  # Enable for smoother motion at cost of accuracy
```

## Aspect Ratio

### Setting Aspect Ratio

```javascript
// Set aspect ratio
player.setAspectRatio('16:9');

// Common ratios
// '16:9' - Widescreen
// '4:3'  - Standard
// '21:9' - Ultrawide
// 'auto' - Original
```

### Zoom and Pan

```javascript
// Set zoom level (1.0 = 100%)
player.setZoom(1.5);

// Pan the view
player.setPan(x, y);

// Reset to original
player.resetZoom();
```

## Deinterlacing

### Enabling Deinterlacing

```toml
[video]
deinterlace = true
```

```javascript
player.setDeinterlacing(true);
```

### Deinterlacing Methods

- **Yadif**: Yet Another Deinterlacing Filter
- **Bob**: Linear interpolation
- **Linear**: Simple linear interpolation

## Video Filters

### Brightness, Contrast, Saturation

```javascript
// Adjust brightness (-1.0 to 1.0)
player.setBrightness(0.2);

// Adjust contrast (0.0 to 2.0)
player.setContrast(1.2);

// Adjust saturation (0.0 to 2.0)
player.setSaturation(1.5);
```

```toml
[video]
brightness = 1.0
contrast = 1.0
saturation = 1.0
hue = 0.0
```

### Rotation

```javascript
// Rotate video
player.setRotation(90);  // 0, 90, 180, 270 degrees
```

### Flip

```javascript
// Flip horizontally
player.setFlip('horizontal');

// Flip vertically
player.setFlip('vertical');

// Reset flip
player.setFlip('none');
```

## Picture-in-Picture (PiP)

### Enabling Picture-in-Picture

```javascript
// Enter PiP mode
await player.enterPictureInPicture();

// Exit PiP mode
await player.exitPictureInPicture();

// Check PiP status
const isPiP = player.isInPictureInPicture();
```

### PiP Configuration

```javascript
const player = new Player('#container', {
    pip: {
        enabled: true,
        position: 'bottom-right',
        width: 320,
        height: 180
    }
});
```

## Screenshots

### Capturing Screenshots

```javascript
// Capture screenshot
const screenshot = await player.captureScreenshot();

// Save to file
screenshot.save('screenshot.png');

// Get base64 data URL
const dataUrl = screenshot.toDataURL();
```

### Screenshot Quality

```javascript
const screenshot = await player.captureScreenshot({
    quality: 0.9,  // 0.0 to 1.0
    format: 'png'  // 'png' or 'jpeg'
});
```

## Video Metadata

### Reading Video Information

```javascript
// Get video metadata
const metadata = player.getVideoMetadata();
console.log(metadata);

// Example output:
{
  width: 1920,
  height: 1080,
  duration: 3600,
  codec: 'h264',
  bitrate: 5000000,
  frameRate: 30,
  aspectRatio: '16:9'
}
```

### Stream Information

```javascript
// Get available streams
const streams = player.getStreams();
console.log(streams);

// Example output:
[
  { type: 'video', codec: 'h264', index: 0 },
  { type: 'audio', codec: 'aac', language: 'en' },
  { type: 'subtitle', codec: 'srt', language: 'en' }
]
```

## Streaming Video

### HLS (HTTP Live Streaming)

```javascript
player.load('https://example.com/stream.m3u8');
```

```bash
vantismedia https://example.com/stream.m3u8
```

### DASH (Dynamic Adaptive Streaming)

```javascript
player.load('https://example.com/stream.mpd');
```

### Low Latency Streaming

```toml
[streaming]
low_latency = true
buffer_size = 16384
```

```bash
vantismedia --low-latency stream.m3u8
```

## 3D Video

### 3D Video Formats

Vantis Media Player supports:

- **Side-by-Side (SBS)**
- **Top-and-Bottom (TAB)**
- **Anaglyph** (red-cyan)
- **Frame Sequential**

### Enabling 3D Mode

```javascript
// Set 3D mode
player.set3DMode('sbs');  // 'sbs', 'tab', 'anaglyph'

// Disable 3D
player.set3DMode('none');
```

## Video Chapters

### Navigation by Chapters

```javascript
// Get chapter list
const chapters = player.getChapters();
console.log(chapters);

// Jump to chapter
player.goToChapter(2);

// Get current chapter
const currentChapter = player.getCurrentChapter();
```

### Chapter Markers

```javascript
// Add chapter marker
player.addChapterMarker({
    time: 60,
    title: 'Chapter 1: Introduction'
});
```

## Performance Optimization

### Caching

```toml
[performance]
cache_size = 512  # MB
```

### Threading

```toml
[performance]
thread_count = 4  # 0 = auto
```

### Memory Management

```javascript
// Clear cache
player.clearCache();

// Preload video
await player.preload('video.mp4');
```

## Troubleshooting

### Video Won't Play

```bash
# Check codec support
vantismedia --info video.mp4

# Try software decoding
vantismedia --no-hwaccel video.mp4
```

### Stuttering Playback

```toml
[performance]
cache_size = 1024
gpu_acceleration = true
```

### No Video, Only Audio

```bash
# Check video stream
vantismedia --info video.mp4

# Try different decoder
vantismedia --decoder software video.mp4
```

## Next Steps

- **[Audio Playback](./audio-playback)** - Learn about audio features
- **[Advanced Features](../advanced-features/)** - Explore advanced functionality
- **[API Reference](../api/)** - Complete API documentation

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)