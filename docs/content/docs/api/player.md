---
sidebar_position: 2
---

# Player API

The Player API is the core of Vantis Media Player, providing methods for loading, controlling, and managing media playback.

## Constructor

### JavaScript/TypeScript

```javascript
const player = new Player(container, options?);
```

```typescript
interface PlayerOptions {
    // Container element
    container: string | HTMLElement;

    // Theme settings
    theme?: 'light' | 'dark' | 'auto';

    // Hardware acceleration
    hardwareAcceleration?: boolean;

    // Volume (0.0 - 1.0)
    volume?: number;

    // Auto-play
    autoplay?: boolean;

    // Muted state
    muted?: boolean;

    // Loop mode
    loop?: boolean;

    // Fullscreen on start
    fullscreen?: boolean;

    // Poster image
    poster?: string;

    // Loading indicator
    loading?: boolean;

    // Controls
    controls?: boolean | ControlsOptions;

    // Plugins
    plugins?: string[];

    // Additional options
    [key: string]: any;
}
```

#### Examples

```javascript
// Basic usage
const player = new Player('#video-container');

// With options
const player = new Player('#video-container', {
    theme: 'dark',
    hardwareAcceleration: true,
    volume: 0.8,
    autoplay: false,
    muted: false,
    loop: false,
    poster: 'poster.jpg',
    controls: true
});
```

### Rust

```rust
let player = Player::new(options)?;
```

```rust
let config = PlayerConfig {
    hardware_acceleration: true,
    volume: 0.8,
    autoplay: false,
    ..Default::default()
};

let player = Player::new(config)?;
```

## Loading Media

### load()

Load media from URL or file path.

```javascript
// Basic load
await player.load('video.mp4');

// With options
await player.load('video.mp4', {
    startTime: 30,        // Start at 30 seconds
    autoplay: true,       // Auto-play on load
    subtitles: 'subs.srt' // Load subtitles
});

// From URL
await player.load('https://example.com/stream.m3u8');
```

```rust
player.load("video.mp4").await?;

// With options
player.load_with_options("video.mp4", LoadOptions {
    start_time: Some(Duration::from_secs(30)),
    autoplay: true,
}).await?;
```

### loadPlaylist()

Load a playlist.

```javascript
await player.loadPlaylist('playlist.m3u');
await player.loadPlaylist(['video1.mp4', 'video2.mp4']);
```

### unload()

Unload current media.

```javascript
await player.unload();
```

## Playback Control

### play()

Start or resume playback.

```javascript
player.play();
```

```rust
player.play();
```

### pause()

Pause playback.

```javascript
player.pause();
```

```rust
player.pause();
```

### stop()

Stop playback and reset to beginning.

```javascript
player.stop();
```

```rust
player.stop();
```

### togglePlay()

Toggle between play and pause.

```javascript
player.togglePlay();
```

### seek()

Seek to a specific time.

```javascript
// Seek to position (seconds)
player.seek(120);  // Seek to 2 minutes

// Seek with time object
player.seek({
    seconds: 120,
    milliseconds: 500
});

// Relative seek
player.seek('+10');  // Forward 10 seconds
player.seek('-10');  // Backward 10 seconds
```

```rust
// Seek to duration
player.seek(Duration::from_secs(120));

// Relative seek
player.seek_relative(Duration::from_secs(10));
```

### seekToLive()

Seek to live edge for live streams.

```javascript
player.seekToLive();
```

## Playback Speed

### setSpeed()

Set playback speed.

```javascript
// Set speed (0.25x to 4.0x)
player.setSpeed(1.5);  // 1.5x speed
player.setSpeed(0.5);  // Half speed
player.setSpeed(2.0);  // Double speed
```

```rust
player.set_speed(1.5);
```

### getSpeed()

Get current playback speed.

```javascript
const speed = player.getSpeed();  // e.g., 1.5
```

### setPitchPreservation()

Preserve pitch when changing speed.

```javascript
player.setPitchPreservation(true);
```

## Volume Control

### setVolume()

Set volume level.

```javascript
// Set volume (0.0 to 1.0)
player.setVolume(0.8);  // 80%

// Set volume percentage
player.setVolumePercent(80);
```

```rust
player.set_volume(0.8);
```

### getVolume()

Get current volume.

```javascript
const volume = player.getVolume();  // e.g., 0.8
```

### setMuted()

Set muted state.

```javascript
player.setMuted(true);
```

### isMuted()

Check if muted.

```javascript
const isMuted = player.isMuted();
```

### toggleMute()

Toggle mute state.

```javascript
player.toggleMute();
```

## Fullscreen

### setFullscreen()

Enter or exit fullscreen.

```javascript
// Enter fullscreen
await player.setFullscreen(true);

// Exit fullscreen
await player.setFullscreen(false);

// Toggle fullscreen
await player.toggleFullscreen();
```

```rust
player.set_fullscreen(true)?;
player.toggle_fullscreen()?;
```

### isFullscreen()

Check fullscreen state.

```javascript
const isFullscreen = player.isFullscreen();
```

### isFullscreenEnabled()

Check if fullscreen is available.

```javascript
const enabled = player.isFullscreenEnabled();
```

## Playback State

### getState()

Get current playback state.

```javascript
const state = player.getState();
// 'idle' | 'loading' | 'ready' | 'playing' | 'paused' | 'ended' | 'error'
```

### isPlaying()

Check if currently playing.

```javascript
const playing = player.isPlaying();
```

### isPaused()

Check if paused.

```javascript
const paused = player.isPaused();
```

### isEnded()

Check if playback ended.

```javascript
const ended = player.isEnded();
```

### isLive()

Check if playing live stream.

```javascript
const live = player.isLive();
```

### isSeeking()

Check if seeking.

```javascript
const seeking = player.isSeeking();
```

## Time Information

### getCurrentTime()

Get current playback position.

```javascript
const time = player.getCurrentTime();  // seconds
```

```rust
let position = player.current_time();
```

### setCurrentTime()

Set playback position (alias for seek).

```javascript
player.setCurrentTime(120);  // Seek to 2 minutes
```

### getDuration()

Get media duration.

```javascript
const duration = player.getDuration();  // seconds
```

```rust
let duration = player.duration();
```

### getRemainingTime()

Get remaining playback time.

```javascript
const remaining = player.getRemainingTime();
```

### getBuffered()

Get buffered time ranges.

```javascript
const buffered = player.getBuffered();
// [{ start: 0, end: 120 }, { start: 150, end: 300 }]
```

### getBufferedPercentage()

Get buffered percentage.

```javascript
const percentage = player.getBufferedPercentage();  // e.g., 75
```

## Media Information

### getMediaInfo()

Get comprehensive media information.

```javascript
const info = await player.getMediaInfo();
console.log(info);
// {
//   duration: 3600,
//   width: 1920,
//   height: 1080,
//   videoCodec: 'h264',
//   audioCodec: 'aac',
//   bitrate: 5000000,
//   frameRate: 30,
//   aspectRatio: '16:9',
//   isLive: false,
//   title: 'Video Title',
//   artist: 'Artist Name',
//   album: 'Album Name'
// }
```

### getVideoInfo()

Get video-specific information.

```javascript
const videoInfo = player.getVideoInfo();
// {
//   width: 1920,
//   height: 1080,
//   codec: 'h264',
//   frameRate: 30,
//   aspectRatio: '16:9',
//   bitrate: 4500000
// }
```

### getAudioInfo()

Get audio-specific information.

```javascript
const audioInfo = player.getAudioInfo();
// {
//   codec: 'aac',
//   sampleRate: 44100,
//   channels: 2,
//   bitrate: 128000,
//   language: 'en'
// }
```

### getMetadata()

Get media metadata.

```javascript
const metadata = player.getMetadata();
// {
//   title: 'Video Title',
//   artist: 'Artist Name',
//   album: 'Album Name',
//   year: 2024,
//   genre: 'Genre',
//   track: 1,
//   custom: { ... }
// }
```

## Track Selection

### getAudioTracks()

Get available audio tracks.

```javascript
const tracks = player.getAudioTracks();
// [
//   { id: 1, language: 'en', label: 'English', enabled: true },
//   { id: 2, language: 'es', label: 'Spanish', enabled: false }
// ]
```

### setAudioTrack()

Select audio track.

```javascript
player.setAudioTrack(2);  // Select by ID
player.setAudioTrackByLanguage('es');  // Select by language
```

### getSubtitleTracks()

Get available subtitle tracks.

```javascript
const tracks = player.getSubtitleTracks();
```

### setSubtitleTrack()

Select subtitle track.

```javascript
player.setSubtitleTrack(1);
player.setSubtitleTrackByLanguage('en');
```

### disableSubtitles()

Disable subtitles.

```javascript
player.disableSubtitles();
```

## Quality Selection

### getQualities()

Get available quality levels.

```javascript
const qualities = player.getQualities();
// [
//   { id: 0, label: 'Auto', bitrate: 0 },
//   { id: 1, label: '1080p', bitrate: 5000000, width: 1920, height: 1080 },
//   { id: 2, label: '720p', bitrate: 2500000, width: 1280, height: 720 }
// ]
```

### setQuality()

Set quality level.

```javascript
player.setQuality(1);  // Set by ID
player.setQualityByLabel('1080p');  // Set by label
player.setAutoQuality(true);  // Enable auto quality
```

### getQuality()

Get current quality.

```javascript
const quality = player.getQuality();
// { id: 1, label: '1080p', bitrate: 5000000 }
```

## Utility Methods

### screenshot()

Capture a screenshot.

```javascript
const screenshot = await player.screenshot({
    format: 'png',  // png, jpeg
    quality: 0.9    // for jpeg
});

// Save to file
screenshot.save('screenshot.png');

// Get base64 data URL
const dataUrl = screenshot.toDataURL();

// Get blob
const blob = screenshot.toBlob();
```

### download()

Download current media.

```javascript
await player.download({
    filename: 'video.mp4'
});
```

### destroy()

Destroy the player instance and free resources.

```javascript
player.destroy();
```

## Configuration

### configure()

Update player configuration.

```javascript
player.configure({
    volume: 0.9,
    loop: true,
    autoplay: false
});
```

### getConfiguration()

Get current configuration.

```javascript
const config = player.getConfiguration();
```

## Error Handling

### getError()

Get last error.

```javascript
const error = player.getError();
// { code: 'NETWORK_ERROR', message: 'Network request failed' }
```

### Error Codes

| Code | Description |
|------|-------------|
| `NETWORK_ERROR` | Network request failed |
| `MEDIA_ERROR` | Media decoding error |
| `FORMAT_ERROR` | Unsupported format |
| `SOURCE_ERROR` | Invalid media source |
| `ENCRYPTION_ERROR` | DRM/encryption error |
| `PLAYER_ERROR` | General player error |

## Next Steps

- **[Audio API](./audio)** - Audio-specific features
- **[Video API](./video)** - Video-specific features
- **[Events API](./events)** - Event system

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)