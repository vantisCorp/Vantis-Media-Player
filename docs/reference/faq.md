---
sidebar_position: 3
title: Frequently Asked Questions
sidebar_label: FAQ
---

# Frequently Asked Questions

Common questions and answers about Vantis Media Player.

## General

### What is Vantis Media Player?

Vantis Media Player is a modern, high-performance media player built with cutting-edge web technologies. It supports multiple video and audio formats, adaptive streaming (HLS, DASH), and provides a rich plugin architecture for extensibility.

### Is Vantis Media Player free to use?

Yes, Vantis Media Player is open source and released under the MIT License. You can use it freely in both personal and commercial projects.

### What browsers are supported?

Vantis Media Player supports all modern browsers:

| Browser | Version | Notes |
|---------|---------|-------|
| Chrome | 90+ | Full support |
| Firefox | 88+ | Full support |
| Safari | 14+ | Full support, native HLS |
| Edge | 90+ | Full support |
| Opera | 76+ | Full support |

### What video formats are supported?

**Video Containers:**
- MP4 (H.264, H.265)
- WebM (VP8, VP9, AV1)
- Ogg (Theora)
- MKV (limited support)

**Audio Formats:**
- AAC
- MP3
- Opus
- Vorbis
- FLAC
- WAV

### What streaming protocols are supported?

- **HLS** (HTTP Live Streaming) - Native on Safari, via Hls.js elsewhere
- **DASH** (Dynamic Adaptive Streaming over HTTP)
- **Smooth Streaming** - Via plugins
- **Progressive Download** - Standard HTTP

## Installation

### How do I install Vantis Media Player?

```bash
# Using npm
npm install @vantis/player

# Using yarn
yarn add @vantis/player

# Using pnpm
pnpm add @vantis/player
```

### How do I add Vantis Media Player to my project?

```html
<!DOCTYPE html>
<html>
<head>
  <title>Vantis Media Player</title>
</head>
<body>
  <div id="player"></div>

  <script type="module">
    import { createPlayer } from '@vantis/player'
    
    const player = createPlayer({
      container: '#player',
      src: 'https://example.com/video.mp4',
      controls: true,
      autoplay: false
    })
  </script>
</body>
</html>
```

### Can I use Vantis Media Player via CDN?

Yes, you can include Vantis Media Player via CDN:

```html
<!-- Latest version -->
<script src="https://cdn.jsdelivr.net/npm/@vantis/player@latest/dist/vantis.min.js"></script>

<!-- Specific version -->
<script src="https://cdn.jsdelivr.net/npm/@vantis/player@1.0.0/dist/vantis.min.js"></script>
```

## Usage

### How do I play a video?

```typescript
import { createPlayer } from '@vantis/player'

const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  controls: true
})

// Play video
player.play()
```

### How do I pause a video?

```typescript
// Pause video
player.pause()

// Toggle play/pause
player.togglePlay()
```

### How do I seek to a specific time?

```typescript
// Seek to 30 seconds
player.seek(30)

// Seek forward 10 seconds
player.seek(player.currentTime + 10)

// Seek to percentage
player.seekPercentage(50) // Seek to 50% of video
```

### How do I change the volume?

```typescript
// Set volume (0-1)
player.volume = 0.5

// Mute/unmute
player.muted = true
player.muted = false

// Toggle mute
player.toggleMute()
```

### How do I enable fullscreen?

```typescript
// Enter fullscreen
player.requestFullscreen()

// Exit fullscreen
player.exitFullscreen()

// Toggle fullscreen
player.toggleFullscreen()
```

### How do I play an HLS stream?

```typescript
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/stream.m3u8',
  type: 'hls',
  controls: true
})
```

### How do I play a DASH stream?

```typescript
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/stream.mpd',
  type: 'dash',
  controls: true
})
```

## Integration

### How do I use Vantis Media Player with React?

```tsx
import { useRef, useEffect } from 'react'
import { createPlayer, VantisPlayer } from '@vantis/player/react'

export function VideoPlayer({ src }) {
  const playerRef = useRef(null)

  useEffect(() => {
    const player = createPlayer({
      container: playerRef.current,
      src,
      controls: true
    })

    return () => player.destroy()
  }, [src])

  return <div ref={playerRef} />
}
```

### How do I use Vantis Media Player with Vue?

```vue
<template>
  <div ref="playerContainer"></div>
</template>

<script>
import { createPlayer } from '@vantis/player'

export default {
  props: ['src'],
  mounted() {
    this.player = createPlayer({
      container: this.$refs.playerContainer,
      src: this.src,
      controls: true
    })
  },
  beforeUnmount() {
    this.player?.destroy()
  }
}
</script>
```

### How do I use Vantis Media Player with Angular?

```typescript
import { Component, OnInit, OnDestroy, ElementRef, ViewChild } from '@angular/core'
import { createPlayer } from '@vantis/player'

@Component({
  selector: 'app-video-player',
  template: '<div #playerContainer></div>'
})
export class VideoPlayerComponent implements OnInit, OnDestroy {
  @ViewChild('playerContainer') container!: ElementRef
  private player: any

  @Input() src!: string

  ngOnInit() {
    this.player = createPlayer({
      container: this.container.nativeElement,
      src: this.src,
      controls: true
    })
  }

  ngOnDestroy() {
    this.player?.destroy()
  }
}
```

## Plugins

### How do I create a plugin?

```typescript
import { Plugin } from '@vantis/player'

class MyPlugin extends Plugin {
  name = 'myPlugin'

  onInit(player) {
    console.log('Plugin initialized')
  }

  onReady() {
    console.log('Player ready')
  }

  onDestroy() {
    console.log('Plugin destroyed')
  }
}

// Register plugin
player.registerPlugin('myPlugin', MyPlugin)
```

### How do I use existing plugins?

```typescript
import { createPlayer } from '@vantis/player'
import { AnalyticsPlugin } from '@vantis/analytics-plugin'
import { AdsPlugin } from '@vantis/ads-plugin'

const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  plugins: [
    { name: 'analytics', plugin: AnalyticsPlugin, options: { trackingId: 'UA-XXXXX' } },
    { name: 'ads', plugin: AdsPlugin, options: { adTagUrl: '...' } }
  ]
})
```

### Where can I find available plugins?

- Official plugins: [GitHub Repository](https://github.com/vantisCorp/vantis-plugins)
- Community plugins: [npm @vantis scope](https://www.npmjs.com/search?q=%40vantis)
- Plugin marketplace: Coming soon

## Troubleshooting

### Why is my video not playing?

1. Check the video URL is correct and accessible
2. Verify the video format is supported
3. Check browser console for errors
4. Ensure CORS headers are set correctly
5. Try with a different video to isolate the issue

### Why is autoplay not working?

Most browsers block autoplay with sound. Solutions:

```typescript
// Mute to allow autoplay
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  autoplay: true,
  muted: true
})

// Or handle autoplay rejection
player.play().catch(error => {
  console.log('Autoplay blocked, user interaction required')
})
```

### Why is the video blurry or low quality?

1. Check if adaptive streaming is available
2. Force higher quality:

```typescript
// For HLS
player.hls.currentLevel = 0 // 0 = highest quality

// For DASH
player.dash.setQualityFor('video', 0)
```

### Why is there no sound?

1. Check volume is not muted
2. Check volume level
3. Verify audio track exists
4. Check browser audio permissions

```typescript
// Debug audio
console.log('Volume:', player.volume)
console.log('Muted:', player.muted)
console.log('Audio tracks:', player.audioTracks)
```

## Performance

### How can I improve video loading speed?

1. Use adaptive streaming (HLS/DASH)
2. Use CDN for video delivery
3. Enable preload:

```typescript
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  preload: 'auto' // 'none', 'metadata', or 'auto'
})
```

### How can I reduce memory usage?

```typescript
// Limit buffer size
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  buffer: {
    maxBufferLength: 30,
    maxMaxBufferLength: 60
  }
})

// Properly destroy player when done
player.destroy()
```

### How can I optimize for mobile?

```typescript
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  mobile: {
    playsInline: true,
    hardwareAcceleration: true
  }
})
```

## Security

### How do I protect my content with DRM?

```typescript
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/protected.mpd',
  drm: {
    widevine: {
      licenseUrl: 'https://license.example.com/widevine'
    },
    playready: {
      licenseUrl: 'https://license.example.com/playready'
    }
  }
})
```

### How do I use secure video URLs?

```typescript
// With token authentication
const player = createPlayer({
  container: '#player',
  src: 'https://example.com/video.mp4',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN'
  }
})
```

## Advanced

### How do I get the current video time?

```typescript
const currentTime = player.currentTime

// Listen for time updates
player.on('timeupdate', (time) => {
  console.log('Current time:', time)
})
```

### How do I get video metadata?

```typescript
player.on('loadedmetadata', () => {
  console.log('Duration:', player.duration)
  console.log('Width:', player.videoWidth)
  console.log('Height:', player.videoHeight)
})
```

### How do I add subtitles?

```typescript
// Add subtitle track
player.addTextTrack({
  kind: 'subtitles',
  src: 'https://example.com/subtitles.vtt',
  srclang: 'en',
  label: 'English'
})

// Multiple subtitles
const subtitles = [
  { src: '/subs/en.vtt', srclang: 'en', label: 'English' },
  { src: '/subs/es.vtt', srclang: 'es', label: 'Spanish' },
  { src: '/subs/fr.vtt', srclang: 'fr', label: 'French' }
]

player.loadSubtitles(subtitles)
```

### How do I capture video screenshots?

```typescript
// Capture current frame
const screenshot = player.captureFrame()

// Returns data URL
console.log(screenshot) // data:image/png;base64,...

// Download screenshot
const link = document.createElement('a')
link.download = 'screenshot.png'
link.href = screenshot
link.click()
```

## Support

### Where can I get help?

- **Documentation**: [docs.vantis.player](https://vantisplayer.dev/docs)
- **GitHub Issues**: [Report bugs](https://github.com/vantisCorp/Vantis-Media-Player/issues)
- **Discord**: [Join community](https://discord.gg/vantis)
- **Stack Overflow**: Tag questions with `vantis-player`

### How do I report a bug?

1. Check existing issues first
2. Create a minimal reproducible example
3. Include:
   - Browser and version
   - Operating system
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Console errors

### How do I request a feature?

1. Check existing feature requests
2. Open a GitHub issue with:
   - Use case description
   - Proposed solution
   - Alternative solutions considered
   - Additional context

## Miscellaneous

### What's the difference between Vantis Media Player and other players?

| Feature | Vantis | Video.js | Plyr | Player.js |
|---------|--------|----------|------|-----------|
| Bundle Size | Small | Medium | Small | Medium |
| HLS Support | Native | Plugin | Plugin | Native |
| DASH Support | Native | Plugin | Plugin | Native |
| Plugin System | Yes | Yes | Limited | Limited |
| TypeScript | Yes | Partial | Yes | Partial |
| React Integration | Native | Wrapper | Wrapper | Wrapper |
| Mobile Support | Excellent | Good | Good | Good |
| DRM Support | Yes | Plugin | No | Yes |

### Can I use Vantis Media Player for commercial projects?

Yes, Vantis Media Player is released under the MIT License, which allows commercial use without restrictions.

### How do I contribute to the project?

See our [Contributing Guide](/development/contributing) for details on how to contribute code, documentation, or report issues.

### Where can I find the changelog?

See our [Changelog](/reference/changelog) for version history and release notes.