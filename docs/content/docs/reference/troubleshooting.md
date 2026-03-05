---
sidebar_position: 2
title: Troubleshooting
sidebar_label: Troubleshooting
---

# Troubleshooting

Common issues and their solutions for Vantis Media Player.

## Video Playback Issues

### Video Won't Play

**Symptoms:**
- Video player loads but doesn't play
- Black screen with no error
- Player shows loading spinner indefinitely

**Possible Causes:**
1. Invalid video URL or format
2. Unsupported codec
3. CORS issues
4. Network problems

**Solutions:**

```typescript
// Check video URL validity
function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

// Check browser support for codec
const video = document.createElement('video')
const canPlayMP4 = video.canPlayType('video/mp4; codecs="avc1.64001F"')
const canPlayWebM = video.canPlayType('video/webm; codecs="vp9"')

console.log('MP4 Support:', canPlayMP4)
console.log('WebM Support:', canPlayWebM)
```

**Troubleshooting Steps:**
1. Verify video URL is accessible
2. Check browser console for errors
3. Test video in different browsers
4. Verify video format and codec
5. Check network tab for failed requests

### Audio Sync Issues

**Symptoms:**
- Audio and video are out of sync
- Audio plays ahead of video
- Video plays ahead of audio

**Possible Causes:**
1. Variable frame rate video
2. Encoding issues
3. Browser-specific problems
4. Hardware acceleration conflicts

**Solutions:**

```typescript
// Sync audio with video
function syncAudio(video: HTMLVideoElement) {
  video.currentTime = video.currentTime
}

// Reset video to fix sync
function resetVideo(video: HTMLVideoElement) {
  const currentTime = video.currentTime
  video.load()
  video.currentTime = currentTime
}

// Disable hardware acceleration (if needed)
const video = document.querySelector('video')
video.disableRemotePlayback = true
```

**Troubleshooting Steps:**
1. Reload the video
2. Disable hardware acceleration
3. Try different browser
4. Clear browser cache
5. Update browser to latest version

### Buffering Issues

**Symptoms:**
- Video stops to buffer frequently
- Long buffer times
- Playback is choppy

**Possible Causes:**
1. Slow internet connection
2. Insufficient buffer size
3. Server issues
4. High video quality

**Solutions:**

```typescript
// Configure buffer settings
const hls = new Hls({
  maxBufferLength: 30,
  maxMaxBufferLength: 60,
  maxBufferHole: 0.5,
  lowBufferWatchdogPeriod: 0.5,
})

// Monitor buffer status
video.addEventListener('progress', () => {
  const buffered = video.buffered
  if (buffered.length > 0) {
    const bufferedEnd = buffered.end(buffered.length - 1)
    const bufferPercent = (bufferedEnd / video.duration) * 100
    console.log(`Buffered: ${bufferPercent.toFixed(2)}%`)
  }
})

// Lower quality for slow connections
function adjustQuality(bufferPercent: number) {
  if (bufferPercent < 10) {
    // Switch to lower quality
    hls.currentLevel = hls.levels.length - 1
  }
}
```

**Troubleshooting Steps:**
1. Check internet connection speed
2. Lower video quality
3. Increase buffer size
4. Check server performance
5. Try different CDN

## Browser-Specific Issues

### Chrome

**Issue:** Autoplay blocked

```typescript
// Handle autoplay blocking
async function playWithUserInteraction(video: HTMLVideoElement) {
  try {
    await video.play()
  } catch (error) {
    // Show play button for user interaction
    console.log('Autoplay blocked, user interaction required')
  }
}
```

**Issue:** Memory leaks

```typescript
// Proper cleanup on unmount
useEffect(() => {
  return () => {
    // Cleanup resources
    hls.destroy()
    video.src = ''
    video.load()
  }
}, [])
```

### Firefox

**Issue:** Hardware acceleration problems

```typescript
// Check hardware acceleration support
const video = document.createElement('video')
if (video.canPlayType('video/mp4; codecs="avc1.64001F"')) {
  console.log('Hardware acceleration supported')
}
```

### Safari

**Issue:** HLS playback issues

```typescript
// Safari native HLS support
if (video.canPlayType('application/vnd.apple.mpegurl')) {
  video.src = videoUrl
} else {
  // Use Hls.js for other browsers
  const hls = new Hls()
  hls.loadSource(videoUrl)
  hls.attachMedia(video)
}
```

**Issue:** EME/DRM issues

```typescript
// Check for EME support
if (video.mediaKeys) {
  console.log('EME supported')
}
```

## Mobile Issues

### iOS

**Issue:** Autoplay not working

```typescript
// iOS requires user interaction for autoplay
const video = document.querySelector('video')
video.muted = true // Muted videos can autoplay on iOS
video.playsInline = true // Prevent fullscreen on mobile
video.autoplay = true
```

**Issue:** Picture-in-Picture not supported

```typescript
// Check for PiP support
if (document.pictureInPictureElement) {
  // Already in PiP mode
}
```

### Android

**Issue:** Fullscreen issues

```typescript
// Request fullscreen
async function requestFullscreen(video: HTMLVideoElement) {
  try {
    if (video.requestFullscreen) {
      await video.requestFullscreen()
    } else if ((video as any).webkitRequestFullscreen) {
      await (video as any).webkitRequestFullscreen()
    }
  } catch (error) {
    console.error('Fullscreen request failed:', error)
  }
}
```

## Performance Issues

### High CPU Usage

**Symptoms:**
- Player causes high CPU usage
- Computer fans run loud
- Other applications slow down

**Solutions:**

```typescript
// Enable hardware acceleration
const video = document.querySelector('video')
video.setAttribute('playsinline', '')

// Use lower quality
function reduceQuality() {
  hls.currentLevel = hls.levels.length - 1
}

// Optimize rendering
requestAnimationFrame(() => {
  // Update UI only when needed
})
```

### High Memory Usage

**Symptoms:**
- Browser uses excessive memory
- Player becomes sluggish over time
- Browser crashes

**Solutions:**

```typescript
// Proper cleanup
function cleanup() {
  hls.destroy()
  video.src = ''
  video.load()
}

// Limit buffer size
const hls = new Hls({
  maxBufferLength: 30,
  maxMaxBufferLength: 30,
})

// Monitor memory usage
setInterval(() => {
  if (performance.memory) {
    console.log('Memory usage:', performance.memory.usedJSHeapSize)
  }
}, 5000)
```

## Network Issues

### CORS Errors

**Symptoms:**
- Console shows CORS errors
- Video fails to load
- Network requests blocked

**Solutions:**

```typescript
// Server-side CORS configuration (Express.js)
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*')
  res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept')
  next()
})

// Or use proxy during development
// In package.json
{
  "proxy": "https://api.example.com"
}
```

### SSL/TLS Issues

**Symptoms:**
- Mixed content warnings
- Video fails to load on HTTPS
- Certificate errors

**Solutions:**

```typescript
// Ensure all resources use HTTPS
const secureUrl = videoUrl.replace('http://', 'https://')

// Check certificate validity
if (window.location.protocol === 'https:') {
  if (!videoUrl.startsWith('https://')) {
    console.warn('Mixed content: loading HTTP resource on HTTPS page')
  }
}
```

## Plugin Issues

### Plugin Not Loading

**Symptoms:**
- Plugin fails to initialize
- Console shows plugin errors
- Plugin functionality not available

**Solutions:**

```typescript
// Check plugin registration
const player = new VantisPlayer()
player.registerPlugin('myPlugin', MyPlugin)

// Verify plugin interface
if (!player.plugins.myPlugin) {
  console.error('Plugin not registered')
}

// Check plugin compatibility
if (typeof MyPlugin !== 'function') {
  console.error('Invalid plugin format')
}
```

### Plugin Conflicts

**Symptoms:**
- Multiple plugins causing issues
- Unexpected behavior
- Performance degradation

**Solutions:**

```typescript
// Disable conflicting plugins
player.unregisterPlugin('conflictingPlugin')

// Check plugin order
const plugins = ['analytics', 'controls', 'ads']
plugins.forEach(name => {
  player.registerPlugin(name, plugins[name])
})

// Use plugin options to avoid conflicts
player.registerPlugin('pluginA', PluginA, { enabled: true })
player.registerPlugin('pluginB', PluginB, { enabled: false })
```

## Debugging Tips

### Enable Debug Mode

```typescript
// Enable debug logging
const player = new VantisPlayer({
  debug: true,
  logLevel: 'debug'
})

// Console logging
player.on('log', (event) => {
  console.log('[Player]', event.message)
})
```

### Monitor Events

```typescript
// Monitor all player events
const events = [
  'ready', 'play', 'pause', 'ended',
  'error', 'buffer', 'seek', 'volume'
]

events.forEach(event => {
  player.on(event, (data) => {
    console.log(`Event: ${event}`, data)
  })
})
```

### Check Network Requests

```typescript
// Monitor video loading
const video = document.querySelector('video')
video.addEventListener('loadstart', () => console.log('Loading started'))
video.addEventListener('loadeddata', () => console.log('Data loaded'))
video.addEventListener('canplay', () => console.log('Can play'))
video.addEventListener('canplaythrough', () => console.log('Can play through'))
video.addEventListener('error', (e) => console.error('Error:', e))
```

## Getting Help

If you're still experiencing issues:

1. **Check the documentation** - Review relevant documentation sections
2. **Search existing issues** - Check GitHub issues for similar problems
3. **Enable debug mode** - Gather diagnostic information
4. **Create a minimal reproducible example** - Isolate the problem
5. **File a bug report** - Include:
   - Browser and version
   - Operating system
   - Video URL (if applicable)
   - Steps to reproduce
   - Expected vs actual behavior
   - Console errors

## Common Error Messages

### Error: MEDIA_ELEMENT_ERROR: Format error

**Cause:** Unsupported video format or codec

**Solution:** Convert video to supported format (H.264, H.265, VP9)

### Error: NETWORK_ERROR

**Cause:** Network connection failed or video URL is invalid

**Solution:** Check internet connection and verify video URL

### Error: MEDIA_ERR_DECODE

**Cause:** Video decoding failed

**Solution:** Try different video format or browser

### Error: HLS parser error

**Cause:** Invalid or malformed HLS manifest

**Solution:** Validate HLS manifest file format

## Resources

- [MDN Video API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)
- [Hls.js Troubleshooting](https://github.com/video-dev/hls.js/blob/master/docs/API.md#error-handling)
- [GitHub Issues](https://github.com/vantisCorp/Vantis-Media-Player/issues)