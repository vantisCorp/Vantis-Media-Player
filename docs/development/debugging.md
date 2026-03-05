---
sidebar_position: 4
title: Debugging
sidebar_label: Debugging
---

# Debugging

Effective debugging strategies and tools for troubleshooting issues in Vantis Media Player.

## Overview

Debugging is an essential skill for developers. This guide covers debugging techniques, tools, and best practices specific to Vantis Media Player development.

## Browser Debugging

### Chrome DevTools

**Opening DevTools:**
- `F12` or `Ctrl+Shift+I` (Windows/Linux)
- `Cmd+Option+I` (Mac)
- Right-click → Inspect

**Key Panels:**

1. **Elements Panel** - Inspect and modify DOM
2. **Console Panel** - View logs and run JavaScript
3. **Sources Panel** - Set breakpoints and debug code
4. **Network Panel** - Monitor network requests
5. **Performance Panel** - Analyze runtime performance
6. **Memory Panel** - Detect memory leaks

### Console Debugging

```typescript
// Basic logging
console.log('Player initialized')
console.info('Video loaded')
console.warn('Buffer low')
console.error('Playback failed')

// Grouped logging
console.group('Player State')
console.log('Playing:', isPlaying)
console.log('Current Time:', currentTime)
console.log('Duration:', duration)
console.groupEnd()

// Table logging for objects
console.table({ isPlaying, currentTime, duration, volume })

// Timing operations
console.time('videoLoad')
// ... code ...
console.timeEnd('videoLoad')

// Conditional logging
const DEBUG = process.env.NODE_ENV === 'development'
if (DEBUG) console.log('Debug info')

// Count function calls
console.count('playButtonClicked')
console.countReset('playButtonClicked')

// Stack traces
console.trace('Function call stack')

// Assert for debugging
console.assert(isPlaying, 'Player should be playing')
```

### Breakpoints

**Setting Breakpoints:**

```typescript
// Debugger statement
function play() {
  debugger // Execution pauses here
  // ...
}

// Conditional breakpoint in Sources panel
// Right-click line number → Add conditional breakpoint
// Condition: currentTime > 10
```

**Breakpoint Types:**
- Line breakpoint - Pauses at specific line
- Conditional breakpoint - Pauses when condition is true
- Logpoint - Logs message without pausing
- DOM breakpoint - Pauses on DOM changes
- XHR breakpoint - Pauses on network requests
- Exception breakpoint - Pauses on thrown errors

### Network Debugging

```typescript
// Monitor video loading
const videoElement = document.querySelector('video')
videoElement.addEventListener('loadstart', () => console.log('Loading started'))
videoElement.addEventListener('loadeddata', () => console.log('Data loaded'))
videoElement.addEventListener('canplay', () => console.log('Can play'))
videoElement.addEventListener('canplaythrough', () => console.log('Can play through'))
videoElement.addEventListener('error', (e) => console.error('Error:', e))

// Network monitoring in DevTools
// Network tab → Filter by "XHR/fetch" or "Media"
// Check Status, Size, Time, Waterfall

// Throttle network for testing
// DevTools → Network → No throttling → Select speed
```

## React Debugging

### React Developer Tools

**Installation:**
- Chrome: https://chrome.google.com/webstore
- Firefox: https://addons.mozilla.org

**Features:**
- Component tree inspection
- Props and state inspection
- Profiler for performance
- Highlight updates on render

```typescript
// Use React DevTools Profiler
import { Profiler } from 'react'

function onRenderCallback(
  id, phase, actualDuration, 
  baseDuration, startTime, commitTime
) {
  console.log(`${id} ${phase} took ${actualDuration}ms`)
}

<Profiler id="Player" onRender={onRenderCallback}>
  <Player />
</Profiler>
```

### State Debugging

```typescript
// Use useDebugValue for custom hooks
function usePlayerState() {
  const [state, setState] = useState(null)
  useDebugValue(state ? 'Playing' : 'Paused')
  return [state, setState]
}

// Debug with console.log in renders
useEffect(() => {
  console.log('State changed:', state)
}, [state])
```

## Performance Debugging

### Performance Profiling

```typescript
// Measure render performance
import { Profiler } from 'react'

<Profiler id="Player" onRender={(id, phase, actualDuration) => {
  if (actualDuration > 16) {
    console.warn(`${id} ${phase} took ${actualDuration}ms`)
  }
}}>
  <Player />
</Profiler>
```

### Memory Profiling

```typescript
// Detect memory leaks
const players = []

function createPlayer() {
  const player = new Player()
  players.push(player)
  return player
}

// Check memory in DevTools
// Memory tab → Take heap snapshot
// Compare snapshots to find leaks

// Force garbage collection (Chrome)
// DevTools → Memory → Collect garbage
```

### Network Performance

```typescript
// Monitor resource timing
const resources = performance.getEntriesByType('resource')
resources.forEach(resource => {
  console.log(`${resource.name}: ${resource.duration}ms`)
})

// Measure video loading time
const video = document.querySelector('video')
const perfObserver = new PerformanceObserver((list) => {
  for (const entry of list.getEntries()) {
    console.log(`${entry.name}: ${entry.duration}ms`)
  }
})
perfObserver.observe({ entryTypes: ['resource'] })
```

## Video-Specific Debugging

### Video Element Debugging

```typescript
// Check video state
const video = document.querySelector('video')

console.log('Ready State:', video.readyState)
// 0 = HAVE_NOTHING
// 1 = HAVE_METADATA
// 2 = HAVE_CURRENT_DATA
// 3 = HAVE_FUTURE_DATA
// 4 = HAVE_ENOUGH_DATA

console.log('Network State:', video.networkState)
// 0 = NETWORK_EMPTY
// 1 = NETWORK_IDLE
// 2 = NETWORK_LOADING
// 3 = NETWORK_NO_SOURCE

console.log('Error:', video.error)
// Check error.code for specific error type

console.log('Video Width:', video.videoWidth)
console.log('Video Height:', video.videoHeight)
console.log('Duration:', video.duration)
console.log('Current Time:', video.currentTime)
```

### Codec Debugging

```typescript
// Check supported codecs
function getSupportedCodecs() {
  const video = document.createElement('video')
  const codecs = [
    'avc1.64001F',
    'avc1.640028',
    'avc1.640029',
    'vp9',
    'vp09.00.50.08',
    'av01.0.01M.08',
    'mp4a.40.2',
    'mp4a.40.5',
    'opus'
  ]
  
  return codecs.filter(codec => video.canPlayType(`video/mp4; codecs="${codec}"`))
}

console.log('Supported codecs:', getSupportedCodecs())
```

### HLS Debugging

```typescript
// Hls.js debugging
import Hls from 'hls.js'

const hls = new Hls({
  debug: true, // Enable debug logs
  enableWorker: true,
})

hls.on(Hls.Events.ERROR, (event, data) => {
  console.error('HLS Error:', data)
  if (data.fatal) {
    switch (data.type) {
      case Hls.ErrorTypes.NETWORK_ERROR:
        console.error('Network error:', data)
        break
      case Hls.ErrorTypes.MEDIA_ERROR:
        console.error('Media error:', data)
        break
      default:
        console.error('Fatal error:', data)
        break
    }
  }
})

hls.on(Hls.Events.MANIFEST_PARSED, (event, data) => {
  console.log('Manifest parsed:', data.levels)
})

hls.on(Hls.Events.LEVEL_SWITCHED, (event, data) => {
  console.log('Level switched:', data.level)
})
```

## Server-Side Debugging

### Node.js Debugging

```bash
# Debug with inspect
node --inspect app.js

# Debug with break on start
node --inspect-brk app.js

# Connect Chrome DevTools
# chrome://inspect
```

### VSCode Debugging

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "node",
      "request": "launch",
      "name": "Debug Server",
      "runtimeExecutable": "pnpm",
      "runtimeArgs": ["dev"],
      "skipFiles": ["<node_internals>/**"],
      "env": {
        "NODE_ENV": "development"
      }
    },
    {
      "type": "chrome",
      "request": "launch",
      "name": "Debug Chrome",
      "url": "http://localhost:3000",
      "webRoot": "${workspaceFolder}"
    },
    {
      "type": "node",
      "request": "attach",
      "name": "Attach to Process",
      "port": 9229
    }
  ]
}
```

## Error Handling

### Try-Catch Patterns

```typescript
// Basic error handling
try {
  await player.play()
} catch (error) {
  console.error('Play failed:', error)
  // Handle error appropriately
}

// Async error handling
async function loadVideo(url: string) {
  try {
    await player.load(url)
    return { success: true }
  } catch (error) {
    console.error('Load failed:', error)
    return { success: false, error }
  }
}

// Error boundaries
class ErrorBoundary extends React.Component {
  state = { hasError: false }

  static getDerivedStateFromError(error: Error) {
    console.error('Error boundary caught:', error)
    return { hasError: true }
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error('Error details:', info)
  }

  render() {
    if (this.state.hasError) {
      return <div>Something went wrong.</div>
    }
    return this.props.children
  }
}
```

### Error Logging

```typescript
// Custom error logger
class Logger {
  error(error: Error, context?: any) {
    console.error('Error:', error)
    console.error('Context:', context)
    
    // Send to error tracking service
    if (typeof window !== 'undefined' && (window as any).Sentry) {
      (window as any).Sentry.captureException(error, { extra: context })
    }
  }
}

const logger = new Logger()

try {
  // ...
} catch (error) {
  logger.error(error, { action: 'playVideo', url })
}
```

## Common Issues and Solutions

### Video Won't Play

**Problem:** Video doesn't start playing

**Solutions:**
```typescript
// Check if video is ready
if (video.readyState < 3) {
  console.log('Video not ready yet')
  await new Promise(resolve => video.addEventListener('canplay', resolve))
}

// Check if autoplay is blocked
if (video.paused && video.autoplay) {
  console.log('Autoplay blocked, manual play required')
  await video.play()
}

// Check browser support
if (!video.canPlayType('video/mp4')) {
  console.error('MP4 not supported')
}
```

### Audio Sync Issues

**Problem:** Audio and video are out of sync

**Solutions:**
```typescript
// Check audio track
const audioTracks = video.audioTracks
console.log('Audio tracks:', audioTracks?.length)

// Enable audio track
if (audioTracks && audioTracks.length > 0) {
  audioTracks[0].enabled = true
}

// Reset video
video.currentTime = 0
video.load()
```

### Buffering Issues

**Problem:** Video keeps buffering

**Solutions:**
```typescript
// Monitor buffer
video.addEventListener('progress', () => {
  const buffered = video.buffered
  if (buffered.length > 0) {
    const bufferedEnd = buffered.end(buffered.length - 1)
    const bufferPercent = (bufferedEnd / video.duration) * 100
    console.log(`Buffered: ${bufferPercent.toFixed(2)}%`)
  }
})

// Adjust buffer size
const hls = new Hls({
  maxBufferLength: 30,
  maxMaxBufferLength: 60,
})
```

## Debugging Tools

### React DevTools

```bash
# Install
npm install -D @welldone-software/why-did-you-render

# Setup
import whyDidYouRender from '@welldone-software/why-did-you-render'
whyDidYouRender(React, { trackAllPureComponents: true })
```

### Redux DevTools

```typescript
import { configureStore } from '@reduxjs/toolkit'

const store = configureStore({
  reducer: rootReducer,
  devTools: process.env.NODE_ENV !== 'production'
})
```

### Performance Monitoring

```typescript
// Web Vitals
import { getCLS, getFID, getFCP, getLCP, getTTFB } from 'web-vitals'

getCLS(console.log)
getFID(console.log)
getFCP(console.log)
getLCP(console.log)
getTTFB(console.log)
```

## Best Practices

1. **Use console.log strategically** - not too much, not too little
2. **Set breakpoints** at critical points
3. **Use debugger statements** for complex logic
4. **Monitor network requests** - especially for video loading
5. **Profile performance** regularly
6. **Check console errors** first
7. **Use TypeScript** for compile-time error detection
8. **Implement error boundaries** for graceful error handling
9. **Log errors** to external services
10. **Document common issues** and their solutions

## Next Steps

- [ ] Install and configure debugging tools
- [ ] Learn browser DevTools shortcuts
- [ ] Set up VSCode debugging configuration
- [ ] Implement error tracking
- [ ] Document debugging procedures