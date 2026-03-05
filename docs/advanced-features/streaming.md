---
sidebar_position: 2
---

# Streaming

Vantis Media Player provides comprehensive streaming support with adaptive bitrate, low-latency modes, and multiple protocol support.

## Supported Protocols

### HTTP/HTTPS

Basic HTTP streaming for direct media files:

```bash
vantismedia https://example.com/video.mp4
vantismedia https://example.com/audio.mp3
```

### HLS (HTTP Live Streaming)

Apple's HTTP Live Streaming protocol:

```bash
vantismedia https://example.com/stream.m3u8
```

```javascript
// HLS with options
player.load('https://example.com/stream.m3u8', {
    hls: {
        maxBufferLength: 30,
        maxMaxBufferLength: 60,
        startLevel: -1  // Auto
    }
});
```

### DASH (Dynamic Adaptive Streaming over HTTP)

MPEG-DASH adaptive streaming:

```bash
vantismedia https://example.com/stream.mpd
```

```javascript
// DASH with options
player.load('https://example.com/stream.mpd', {
    dash: {
        updatePeriod: 5,
        bufferAhead: 60
    }
});
```

### RTMP

Real-Time Messaging Protocol for live streaming:

```bash
vantismedia rtmp://example.com/live/stream
```

### RTSP/RTP

Real-Time Streaming Protocol for IP cameras and surveillance:

```bash
vantismedia rtsp://camera.example.com/stream
```

### WebRTC

Real-time communication protocol:

```javascript
// WebRTC streaming
player.load('webrtc://example.com/stream', {
    webrtc: {
        iceServers: [
            { urls: 'stun:stun.example.com:3478' },
            { urls: 'turn:turn.example.com:3478', username: 'user', credential: 'pass' }
        ]
    }
});
```

## Adaptive Bitrate (ABR)

### ABR Algorithm

Vantis Media Player uses advanced ABR algorithms:

- **Buffer-based** - Adjusts quality based on buffer level
- **Throughput-based** - Adjusts based on measured bandwidth
- **Hybrid** - Combines both approaches
- **Learning-based** - Uses ML to predict optimal quality

### ABR Configuration

```javascript
// Configure ABR
player.setABR({
    algorithm: 'hybrid',
    minBitrate: 500000,     // 500 Kbps
    maxBitrate: 10000000,   // 10 Mbps
    startBitrate: 2000000,  // 2 Mbps
    maxBufferLength: 30,    // seconds
    minBufferLength: 5      // seconds
});
```

```toml
[streaming]
adaptive_bitrate = true
algorithm = "hybrid"
min_bitrate = 500000
max_bitrate = 10000000
start_bitrate = 2000000
buffer_length = 30
```

### Manual Quality Selection

```javascript
// Get available qualities
const qualities = player.getQualities();
console.log(qualities);
// [
//   { id: 0, bitrate: 500000, width: 640, height: 360, label: '360p' },
//   { id: 1, bitrate: 1000000, width: 854, height: 480, label: '480p' },
//   { id: 2, bitrate: 2000000, width: 1280, height: 720, label: '720p' },
//   { id: 3, bitrate: 5000000, width: 1920, height: 1080, label: '1080p' }
// ]

// Set quality
player.setQuality(3);  // 1080p

// Set to auto
player.setAutoQuality(true);
```

## Low-Latency Streaming

### Configuration

```toml
[streaming]
low_latency = true
buffer_size = 16384
target_latency = 3  # seconds
```

```javascript
// Enable low latency
player.setLowLatency(true, {
    targetLatency: 3,
    maxLatency: 5,
    catchupRate: 1.1
});
```

### LL-HLS Support

Low-Latency HLS for live streaming:

```javascript
player.load('https://example.com/ll-stream.m3u8', {
    llHls: {
        targetLatency: 3,
        maxLatency: 5,
        liveCatchUp: true
    }
});
```

## Buffering

### Buffer Configuration

```toml
[streaming]
buffer_size = 32768
max_buffer_length = 60
buffer_water_mark = 30
back_buffer_length = 30
```

```javascript
// Configure buffer
player.setBufferConfig({
    maxBufferLength: 60,      // Maximum buffer ahead (seconds)
    maxBufferSize: 52428800,  // Maximum buffer size (bytes) - 50 MB
    maxBufferHole: 0.5,       // Maximum gap in buffer (seconds)
    backBufferLength: 30      // Buffer behind current position (seconds)
});
```

### Buffer Events

```javascript
// Monitor buffer state
player.on('bufferUpdate', (info) => {
    console.log(`Buffer: ${info.buffered}s`);
    console.log(`Ahead: ${info.ahead}s`);
    console.log(`Behind: ${info.behind}s`);
});

// Buffer warning
player.on('bufferWarning', (info) => {
    console.log(`Low buffer: ${info.bufferLength}s`);
});
```

## Live Streaming

### Live Mode Detection

```javascript
// Check if stream is live
const isLive = player.isLive();

// Get live position
const livePosition = player.getLivePosition();

// Jump to live edge
player.seekToLive();
```

### Live Configuration

```toml
[streaming.live]
live_sync_duration = 30
live_max_latency = 60
live_duration = 86400
```

```javascript
// Configure live playback
player.setLiveConfig({
    syncDuration: 30,
    maxLatency: 60,
    liveCatchUp: {
        playbackRate: 1.1,
        threshold: 5
    }
});
```

### DVR Support

Digital Video Recorder for live streams:

```javascript
// Check if DVR is available
const dvrAvailable = player.isDVRAvailable();

// Get DVR window
const dvrWindow = player.getDVRWindow();
console.log(dvrWindow);
// { start: 0, end: 3600, current: 3550 }

// Seek within DVR window
player.seek(1800);  // Seek to 30 minutes ago
```

## HTTP Configuration

### Headers and Authentication

```javascript
// Custom headers
player.load('https://example.com/stream.m3u8', {
    headers: {
        'Authorization': 'Bearer token123',
        'X-Custom-Header': 'value'
    }
});
```

```toml
[network]
user_agent = "VantisMedia/1.0"
custom_headers = [
    ["Authorization", "Bearer token123"],
    ["X-Custom-Header", "value"]
]
```

### Proxy Configuration

```toml
[network]
proxy = "http://proxy.example.com:8080"
proxy_auth = "user:password"
```

```javascript
// Set proxy
player.setProxy({
    host: 'proxy.example.com',
    port: 8080,
    auth: {
        username: 'user',
        password: 'password'
    }
});
```

### Cookies

```javascript
// Set cookies for streaming
player.load('https://example.com/stream.m3u8', {
    cookies: {
        'session_id': 'abc123',
        'user_token': 'xyz789'
    }
});
```

## Multi-Audio Tracks

### Audio Track Selection

```javascript
// Get available audio tracks
const audioTracks = player.getAudioTracks();
console.log(audioTracks);
// [
//   { id: 'audio-1', language: 'en', label: 'English' },
//   { id: 'audio-2', language: 'es', label: 'Spanish' },
//   { id: 'audio-3', language: 'fr', label: 'French' }
// ]

// Select audio track
player.setAudioTrack('audio-2');

// Select by language
player.setAudioTrackByLanguage('es');
```

## Multiple Subtitle Tracks

### Subtitle Track Selection

```javascript
// Get available subtitle tracks
const subtitleTracks = player.getSubtitleTracks();

// Select subtitle track
player.setSubtitleTrack(1);

// Select by language
player.setSubtitleTrackByLanguage('en');
```

## Segment Downloads

### Download Control

```javascript
// Control segment downloads
player.setDownloadConfig({
    maxConcurrent: 4,
    segmentTimeout: 15000,
    retryCount: 3,
    retryDelay: 1000
});
```

### Progress Tracking

```javascript
// Track download progress
player.on('downloadProgress', (info) => {
    console.log(`Downloaded: ${info.downloaded}`);
    console.log(`Speed: ${info.speed} Mbps`);
    console.log(`Remaining: ${info.remaining}`);
});
```

## Bandwidth Estimation

### Manual Bandwidth Check

```bash
# Test bandwidth before streaming
vantismedia --test-bandwidth https://example.com
```

```javascript
// Get estimated bandwidth
const bandwidth = player.getEstimatedBandwidth();
console.log(`Estimated bandwidth: ${bandwidth} bps`);

// Force bandwidth estimation
await player.estimateBandwidth();
```

## Stream Recording

### Recording Live Streams

```javascript
// Start recording
const recording = await player.startRecording({
    format: 'mp4',
    path: '/path/to/recording.mp4',
    maxDuration: 3600  // 1 hour max
});

// Stop recording
await recording.stop();
console.log(`Recording saved: ${recording.path}`);
```

## Stream Information

### Getting Stream Info

```javascript
// Get stream information
const info = await player.getStreamInfo();
console.log(info);
// {
//   protocol: 'hls',
//   duration: 3600,
//   isLive: false,
//   qualities: [...],
//   audioTracks: [...],
//   subtitleTracks: [...],
//   codecs: {
//     video: 'h264',
//     audio: 'aac'
//   }
// }
```

## Error Handling

### Stream Errors

```javascript
// Handle stream errors
player.on('error', (error) => {
    switch (error.code) {
        case 'NETWORK_ERROR':
            console.log('Network error. Check your connection.');
            player.retry();
            break;
        case 'MANIFEST_ERROR':
            console.log('Invalid playlist.');
            break;
        case 'FRAG_ERROR':
            console.log('Segment download failed.');
            player.skipSegment();
            break;
        case 'BUFFER_ERROR':
            console.log('Buffer error.');
            player.clearBuffer();
            break;
    }
});
```

### Recovery Options

```javascript
// Automatic recovery
player.setErrorRecovery({
    retryCount: 3,
    retryDelay: 2000,
    skipSegments: true,
    fallbackQuality: true
});
```

## Troubleshooting

### Stream Won't Play

```bash
# Check stream URL
vantismedia --info https://example.com/stream.m3u8

# Verbose output
vantismedia --verbose https://example.com/stream.m3u8

# Test with different protocol
vantismedia --protocol hls https://example.com/stream
```

### Buffering Issues

```toml
[streaming]
buffer_size = 65536
max_buffer_length = 120
```

```bash
# Increase buffer
vantismedia --buffer 120 https://example.com/stream.m3u8
```

### Latency Issues

```bash
# Enable low latency
vantismedia --low-latency stream.m3u8
```

## Performance Tips

1. **Enable hardware acceleration** for decoding
2. **Use appropriate buffer size** based on bandwidth
3. **Enable adaptive bitrate** for best quality
4. **Use CDN-hosted streams** for better performance
5. **Monitor bandwidth** and adjust quality accordingly

## Next Steps

- **[Plugins](./plugins)** - Plugin system
- **[Custom UI](./custom-ui)** - Customization options
- **[Performance](./performance)** - Performance optimization

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)