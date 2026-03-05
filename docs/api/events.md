---
sidebar_position: 6
---

# Events API

Vantis Media Player uses an event-driven architecture to provide notifications about player state changes and media playback.

## Event System Overview

The event system allows you to:

- **React to state changes** - Play, pause, seek, etc.
- **Handle errors** - Catch and respond to errors
- **Track progress** - Monitor playback position
- **Customize behavior** - Add custom logic
- **Integrate with other systems** - Connect to external services

## Listening to Events

### JavaScript/TypeScript

```javascript
// Basic event listener
player.on('playing', () => {
    console.log('Player is now playing');
});

// Event listener with data
player.on('timeupdate', (data) => {
    console.log(`Current time: ${data.currentTime}s`);
});

// One-time event listener
player.once('ended', () => {
    console.log('Playback ended');
});

// Remove event listener
const handler = () => console.log('Playing');
player.on('playing', handler);
player.off('playing', handler);
```

### Rust

```rust
use vantismedia::Events;

// Register event handler
player.on(Events::Playing, || {
    println!("Player is now playing");
});

// With context
player.on(Events::TimeUpdate, |position: Duration| {
    println!("Current time: {:?}", position);
});

// Remove handler
player.off(Events::Playing);
```

## Playback Events

### loadstart

Fired when media loading starts.

```javascript
player.on('loadstart', () => {
    console.log('Media loading started');
});
```

### loadedmetadata

Fired when media metadata is loaded.

```javascript
player.on('loadedmetadata', (metadata) => {
    console.log('Duration:', metadata.duration);
    console.log('Dimensions:', metadata.width, 'x', metadata.height);
});
```

### loadeddata

Fired when media data is loaded and can be played.

```javascript
player.on('loadeddata', () => {
    console.log('Media data loaded');
});
```

### canplay

Fired when the media can be played.

```javascript
player.on('canplay', () => {
    console.log('Media can be played');
});
```

### canplaythrough

Fired when the media can be played through to end without stopping.

```javascript
player.on('canplaythrough', () => {
    console.log('Media can be played through');
});
```

### playing

Fired when playback starts.

```javascript
player.on('playing', () => {
    console.log('Playback started');
});
```

### pause

Fired when playback is paused.

```javascript
player.on('pause', () => {
    console.log('Playback paused');
});
```

### ended

Fired when playback ends.

```javascript
player.on('ended', () => {
    console.log('Playback ended');
});
```

### stop

Fired when playback is stopped.

```javascript
player.on('stop', () => {
    console.log('Playback stopped');
});
```

## Progress Events

### timeupdate

Fired periodically during playback.

```javascript
player.on('timeupdate', (data) => {
    console.log(`Current: ${data.currentTime}s`);
    console.log(`Duration: ${data.duration}s`);
    console.log(`Percent: ${data.percentage}%`);
});
```

### progress

Fired when the download progress updates.

```javascript
player.on('progress', (data) => {
    console.log(`Downloaded: ${data.loaded}%`);
    console.log(`Total: ${data.total} bytes`);
});
```

### bufferprogress

Fired when buffer progress updates.

```javascript
player.on('bufferprogress', (data) => {
    console.log(`Buffered: ${data.buffered}%`);
    console.log(`Buffer ahead: ${data.ahead}s`);
});
```

### seeking

Fired when seeking starts.

```javascript
player.on('seeking', (data) => {
    console.log('Seeking to:', data.time);
});
```

### seeked

Fired when seeking completes.

```javascript
player.on('seeked', (data) => {
    console.log('Seeked to:', data.time);
});
```

## Volume Events

### volumechange

Fired when volume changes.

```javascript
player.on('volumechange', (data) => {
    console.log(`Volume: ${Math.round(data.volume * 100)}%`);
});
```

### mute

Fired when audio is muted.

```javascript
player.on('mute', () => {
    console.log('Audio muted');
});
```

### unmute

Fired when audio is unmuted.

```javascript
player.on('unmute', () => {
    console.log('Audio unmuted');
});
```

## Quality Events

### qualitychange

Fired when quality level changes.

```javascript
player.on('qualitychange', (quality) => {
    console.log('Quality:', quality.label);
    console.log('Bitrate:', quality.bitrate);
});
```

### qualityavailable

Fired when new quality levels become available.

```javascript
player.on('qualityavailable', (qualities) => {
    console.log('Available qualities:', qualities.map(q => q.label));
});
```

## Track Events

### audiotrackchange

Fired when audio track changes.

```javascript
player.on('audiotrackchange', (track) => {
    console.log('Audio track:', track.label);
});
```

### subtitletrackchange

Fired when subtitle track changes.

```javascript
player.on('subtitletrackchange', (track) => {
    console.log('Subtitle track:', track.label);
});
```

## Fullscreen Events

### fullscreenchange

Fired when fullscreen state changes.

```javascript
player.on('fullscreenchange', (isFullscreen) => {
    console.log('Fullscreen:', isFullscreen);
});
```

### fullscreenenter

Fired when entering fullscreen.

```javascript
player.on('fullscreenenter', () => {
    console.log('Entered fullscreen');
});
```

### fullscreenexit

Fired when exiting fullscreen.

```javascript
player.on('fullscreenexit', () => {
    console.log('Exited fullscreen');
});
```

## Error Events

### error

Fired when an error occurs.

```javascript
player.on('error', (error) => {
    console.error('Error:', error.message);
    console.error('Code:', error.code);
    console.error('Type:', error.type);
});
```

### Error Codes

| Code | Type | Description |
|------|------|-------------|
| 1 | `MEDIA_ERR_ABORTED` | Fetch aborted |
| 2 | `MEDIA_ERR_NETWORK` | Network error |
| 3 | `MEDIA_ERR_DECODE` | Decode error |
| 4 | `MEDIA_ERR_SRC_NOT_SUPPORTED` | Format not supported |
| 5 | `PLAYER_ERR_INIT` | Player initialization failed |
| 6 | `PLAYER_ERR_PLAYBACK` | Playback error |
| 7 | `PLAYER_ERR_STREAM` | Streaming error |
| 8 | `PLAYER_ERR_DRM` | DRM/encryption error |

### Warning Events

```javascript
player.on('warning', (warning) => {
    console.warn('Warning:', warning.message);
});
```

## Statistics Events

### stats

Fired with playback statistics.

```javascript
player.on('stats', (stats) => {
    console.log('CPU:', stats.cpuUsage + '%');
    console.log('Memory:', stats.memoryUsage + 'MB');
    console.log('FPS:', stats.frameRate);
    console.log('Dropped frames:', stats.droppedFrames);
});
```

### performanceupdate

Fired with performance metrics.

```javascript
player.on('performanceupdate', (metrics) => {
    console.log('Decode time:', metrics.decodeTime + 'ms');
    console.log('Render time:', metrics.renderTime + 'ms');
    console.log('Buffer health:', metrics.bufferHealth + '%');
});
```

## Network Events

### networkstatechange

Fired when network state changes.

```javascript
player.on('networkstatechange', (state) => {
    console.log('Network state:', state);
    // 'idle' | 'loading' | 'no_source'
});
```

### bitratechange

Fired when bitrate changes (ABR).

```javascript
player.on('bitratechange', (bitrate) => {
    console.log('Bitrate:', (bitrate / 1000000).toFixed(2) + ' Mbps');
});
```

### bandwidthestimate

Fired with bandwidth estimate.

```javascript
player.on('bandwidthestimate', (bandwidth) => {
    console.log('Estimated bandwidth:', (bandwidth / 1000000).toFixed(2) + ' Mbps');
});
```

## Buffer Events

### bufferfull

Fired when buffer is full.

```javascript
player.on('bufferfull', () => {
    console.log('Buffer is full');
});
```

### bufferempty

Fired when buffer is empty.

```javascript
player.on('bufferempty', () => {
    console.log('Buffer is empty');
});
```

### bufferwarning

Fired when buffer is running low.

```javascript
player.on('bufferwarning', (data) => {
    console.log('Buffer warning:', data.bufferLength + 's');
});
```

## Custom Events

### Emitting Custom Events

```javascript
// Emit custom event
player.emit('custom-event', {
    data: 'Hello from player',
    timestamp: Date.now()
});

// Listen to custom event
player.on('custom-event', (data) => {
    console.log('Custom event received:', data);
});
```

## Event Filtering

### Filtering Events

```javascript
// Filter timeupdate events (fire every second instead of every frame)
player.on('timeupdate', (data) => {
    console.log('Time:', data.currentTime);
}, { throttle: 1000 });  // Throttle to once per second

// Only fire when percentage is a multiple of 10
player.on('timeupdate', (data) => {
    console.log('Progress:', data.percentage + '%');
}, { filter: (data) => data.percentage % 10 === 0 });
```

## Event Priority

Events are fired in this order:

1. **loadstart**
2. **progress**
3. **suspend**
4. **abort**
5. **error**
6. **emptied**
7. **stalled**
8. **loadedmetadata**
9. **loadeddata**
10. **canplay**
11. **playing**
12. **waiting**
13. **seeking**
14. **seeked**
15. **timeupdate** (repeated during playback)
16. **ended**
17. **pause**

## Event Data

All events include common properties:

```javascript
player.on('any-event', (data) => {
    console.log('Event type:', data.type);
    console.log('Timestamp:', data.timestamp);
    console.log('Player state:', data.playerState);
    console.log('Media info:', data.mediaInfo);
});
```

## Removing Event Listeners

### Remove Specific Listener

```javascript
const handler = (data) => console.log('Playing');
player.on('playing', handler);
player.off('playing', handler);
```

### Remove All Listeners

```javascript
// Remove all listeners for an event
player.off('playing');

// Remove all listeners
player.removeAllListeners();
```

### Remove Once Listener

```javascript
player.once('ended', handler);
player.off('ended', handler);
```

## Event Debugging

### Enable Event Logging

```javascript
// Enable verbose event logging
player.setEventLogging(true);

// Log specific events
player.setEventLogging(['playing', 'pause', 'error']);
```

### Event Statistics

```javascript
// Get event statistics
const stats = player.getEventStats();
console.log(stats);
// {
//   totalEvents: 1250,
//   eventCounts: {
//     playing: 10,
//     pause: 8,
//     timeupdate: 1200,
//     ...
//   }
// }
```

## Best Practices

### Memory Management

```javascript
// Always remove listeners when no longer needed
function setupPlayer() {
    const handler = () => console.log('Playing');
    player.on('playing', handler);
    
    return () => {
        player.off('playing', handler);
    };
}

const cleanup = setupPlayer();
// Later
cleanup();
```

### Debouncing/Throttling

```javascript
// Debounce frequent events
let timeout;
player.on('timeupdate', (data) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
        console.log('Time:', data.currentTime);
    }, 100);
});

// Or use built-in throttling
player.on('timeupdate', (data) => {
    console.log('Time:', data.currentTime);
}, { throttle: 1000 });
```

### Error Handling

```javascript
player.on('error', (error) => {
    switch (error.code) {
        case 2: // Network error
            console.log('Network error. Retrying...');
            player.retry();
            break;
        case 4: // Format not supported
            console.log('Unsupported format:', error.url);
            break;
        default:
            console.log('Error:', error.message);
    }
});
```

## Complete Example

```javascript
const player = new Player('#container');

// Setup event listeners
player.on('loadstart', () => {
    showLoadingIndicator();
});

player.on('canplay', () => {
    hideLoadingIndicator();
});

player.on('playing', () => {
    updatePlayButton('pause');
});

player.on('pause', () => {
    updatePlayButton('play');
});

player.on('timeupdate', (data) => {
    updateProgress(data.currentTime, data.duration);
    updateTimeDisplay(data.currentTime);
}, { throttle: 100 });

player.on('volumechange', (data) => {
    updateVolumeSlider(data.volume);
});

player.on('qualitychange', (quality) => {
    updateQualityButton(quality.label);
});

player.on('fullscreenchange', (isFullscreen) => {
    updateFullscreenButton(isFullscreen);
});

player.on('error', (error) => {
    showErrorMessage(error.message);
});

player.on('ended', () => {
    showRecommendations();
});

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    player.removeAllListeners();
});
```

## Next Steps

- **[Player API](./player)** - Core player methods
- **[Audio API](./audio)** - Audio events
- **[Video API](./video)** - Video events

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)