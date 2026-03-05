---
sidebar_position: 4
---

# Video API

The Video API provides methods for managing video playback, quality, and visual settings.

## Video Track Management

### getVideoTracks()

Get all available video tracks.

```javascript
const tracks = player.getVideoTracks();
// [
//   { id: 1, width: 1920, height: 1080, bitrate: 5000000, enabled: true },
//   { id: 2, width: 1280, height: 720, bitrate: 2500000, enabled: false }
// ]
```

### setVideoTrack()

Select a video track.

```javascript
player.setVideoTrack(1);
```

### getCurrentVideoTrack()

Get the currently selected video track.

```javascript
const track = player.getCurrentVideoTrack();
```

## Resolution and Quality

### getResolution()

Get current video resolution.

```javascript
const resolution = player.getResolution();
// { width: 1920, height: 1080 }
```

### setMaxResolution()

Set maximum resolution.

```javascript
player.setMaxResolution('4K');      // 3840x2160
player.setMaxResolution('1080p');  // 1920x1080
player.setMaxResolution('720p');   // 1280x720
player.setMaxResolution('480p');   // 854x480
```

### isAdaptiveResolution()

Check if adaptive resolution is enabled.

```javascript
const adaptive = player.isAdaptiveResolution();
```

## Aspect Ratio

### setAspectRatio()

Set aspect ratio.

```javascript
player.setAspectRatio('16:9');   // Widescreen
player.setAspectRatio('4:3');    // Standard
player.setAspectRatio('21:9');   // Ultrawide
player.setAspectRatio('auto');   // Original
```

### getAspectRatio()

Get current aspect ratio.

```javascript
const ratio = player.getAspectRatio();  // '16:9'
```

## Zoom and Pan

### setZoom()

Set zoom level.

```javascript
player.setZoom(1.5);   // 150% zoom
player.setZoom(1.0);   // 100% (reset)
```

### getZoom()

Get current zoom level.

```javascript
const zoom = player.getZoom();  // 1.5
```

### setPan()

Set pan offset.

```javascript
player.setPan(0.1, 0.2);  // x, y offset (-1 to 1)
```

### resetZoom()

Reset zoom and pan.

```javascript
player.resetZoom();
```

## Rotation

### setRotation()

Set video rotation.

```javascript
player.setRotation(90);   // Rotate 90 degrees
player.setRotation(180);  // Rotate 180 degrees
player.setRotation(270);  // Rotate 270 degrees
player.setRotation(0);    // No rotation
```

### getRotation()

Get current rotation.

```javascript
const rotation = player.getRotation();  // 90
```

## Flip

### setFlip()

Set flip mode.

```javascript
player.setFlip('horizontal');  // Flip horizontally
player.setFlip('vertical');    // Flip vertically
player.setFlip('none');        // No flip
```

### getFlip()

Get current flip mode.

```javascript
const flip = player.getFlip();  // 'horizontal'
```

## Video Filters

### Brightness

```javascript
// Set brightness (-1.0 to 1.0)
player.setBrightness(0.2);   // Brighter
player.setBrightness(-0.2);  // Darker
player.setBrightness(0);     // Normal

// Get brightness
const brightness = player.getBrightness();
```

### Contrast

```javascript
// Set contrast (0.0 to 2.0)
player.setContrast(1.2);  // Higher contrast
player.setContrast(0.8);  // Lower contrast
player.setContrast(1.0);  // Normal

// Get contrast
const contrast = player.getContrast();
```

### Saturation

```javascript
// Set saturation (0.0 to 2.0)
player.setSaturation(1.5);  // More saturated
player.setSaturation(0.5);  // Less saturated
player.setSaturation(1.0);  // Normal

// Get saturation
const saturation = player.getSaturation();
```

### Hue

```javascript
// Set hue (-180 to 180)
player.setHue(30);   // Shift hue
player.setHue(0);    // Normal

// Get hue
const hue = player.getHue();
```

### Gamma

```javascript
// Set gamma (0.1 to 10.0)
player.setGamma(1.2);
player.setGamma(1.0);  // Normal

// Get gamma
const gamma = player.getGamma();
```

### Reset Filters

```javascript
player.resetVideoFilters();
```

## Deinterlacing

### setDeinterlacing()

Enable or disable deinterlacing.

```javascript
player.setDeinterlacing(true);
player.setDeinterlacing(false);
```

### setDeinterlacingMode()

Set deinterlacing mode.

```javascript
player.setDeinterlacingMode('yadif');  // Yadif algorithm
player.setDeinterlacingMode('bob');    // Bob algorithm
player.setDeinterlacingMode('linear'); // Linear
```

### isDeinterlacingEnabled()

Check if deinterlacing is enabled.

```javascript
const enabled = player.isDeinterlacingEnabled();
```

## Frame Control

### getFrameRate()

Get video frame rate.

```javascript
const fps = player.getFrameRate();  // 30, 60, etc.
```

### setFrameRate()

Set target frame rate.

```javascript
player.setFrameRate(60);
```

### stepFrame()

Step forward or backward by frame.

```javascript
player.stepFrame(1);   // Forward one frame
player.stepFrame(-1);  // Backward one frame
player.stepFrame(10);  // Forward 10 frames
```

### getCurrentFrame()

Get current frame number.

```javascript
const frame = player.getCurrentFrame();
```

## Picture-in-Picture

### enterPictureInPicture()

Enter Picture-in-Picture mode.

```javascript
try {
    await player.enterPictureInPicture();
    console.log('PiP mode activated');
} catch (error) {
    console.error('PiP not supported:', error);
}
```

### exitPictureInPicture()

Exit Picture-in-Picture mode.

```javascript
await player.exitPictureInPicture();
```

### isPictureInPicture()

Check if in Picture-in-Picture mode.

```javascript
const isPiP = player.isPictureInPicture();
```

### setPictureInPictureConfig()

Configure Picture-in-Picture.

```javascript
player.setPictureInPictureConfig({
    width: 320,
    height: 180
});
```

## Screenshots

### captureScreenshot()

Capture a screenshot.

```javascript
// Capture at current resolution
const screenshot = await player.captureScreenshot();

// Capture at specific resolution
const screenshot = await player.captureScreenshot({
    width: 1920,
    height: 1080,
    format: 'png',      // 'png' or 'jpeg'
    quality: 0.9        // For JPEG
});

// Save to file
await screenshot.save('screenshot.png');

// Get as base64 data URL
const dataUrl = screenshot.toDataURL();

// Get as Blob
const blob = screenshot.toBlob();

// Get as ArrayBuffer
const buffer = screenshot.toArrayBuffer();
```

### captureFrame()

Capture a specific frame.

```javascript
// Capture frame at specific time
const frame = await player.captureFrame(120);  // At 2 minutes

// Get frame data
const imageData = frame.getImageData();
```

## Video Statistics

### getVideoStats()

Get video playback statistics.

```javascript
const stats = player.getVideoStats();
// {
//   width: 1920,
//   height: 1080,
//   codec: 'h264',
//   bitrate: 5000000,
//   frameRate: 30,
//   droppedFrames: 0,
//   decodeTime: 8.5,
//   renderTime: 4.2
// }
```

### getDroppedFrames()

Get dropped frame count.

```javascript
const dropped = player.getDroppedFrames();
```

### resetStats()

Reset video statistics.

```javascript
player.resetVideoStats();
```

## HDR Support

### getHDRSupport()

Check HDR support.

```javascript
const hdrSupport = player.getHDRSupport();
// {
//   hdr10: true,
//   hdr10plus: true,
//   dolbyVision: false,
//   hlg: true
// }
```

### isHDR()

Check if current video is HDR.

```javascript
const isHDR = player.isHDR();
```

### setHDRMode()

Set HDR mode.

```javascript
player.setHDRMode('auto');    // Auto-detect
player.setHDRMode('sdr');     // Force SDR
player.setHDRMode('hdr10');   // Force HDR10
```

## Video Events

```javascript
// Video frame decoded
player.on('videoframe', (frame) => {
    console.log('Frame:', frame.number);
});

// Resolution changed
player.on('resolutionchange', (res) => {
    console.log(`Resolution: ${res.width}x${res.height}`);
});

// Aspect ratio changed
player.on('aspectratiochange', (ratio) => {
    console.log('Aspect ratio:', ratio);
});

// Dropped frames
player.on('droppedframes', (count) => {
    console.log('Dropped frames:', count);
});

// HDR mode changed
player.on('hdrmodechange', (mode) => {
    console.log('HDR mode:', mode);
});
```

## Rust API

```rust
use vantismedia::video::{VideoConfig, AspectRatio};

// Get resolution
let resolution = player.resolution();

// Set aspect ratio
player.set_aspect_ratio(AspectRatio::Wide16x9)?;

// Zoom and pan
player.set_zoom(1.5)?;
player.set_pan(0.1, 0.2)?;

// Video filters
player.set_brightness(0.2)?;
player.set_contrast(1.2)?;
player.set_saturation(1.5)?;

// Deinterlacing
player.set_deinterlacing(true);

// Picture-in-Picture
player.enter_picture_in_picture()?;
player.exit_picture_in_picture()?;

// Screenshot
let screenshot = player.capture_screenshot(ScreenshotOptions {
    format: ImageFormat::Png,
    width: 1920,
    height: 1080,
})?;
screenshot.save("screenshot.png")?;
```

## Next Steps

- **[Audio API](./audio)** - Audio features
- **[Subtitles API](./subtitles)** - Subtitle management
- **[Events API](./events)** - Event system

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)