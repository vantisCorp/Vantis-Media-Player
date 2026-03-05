---
sidebar_position: 3
---

# Event System

Vantis Media Player uses a robust event system for communication between components and between the player and application code. This document describes the event system architecture and usage.

## Event System Overview

The event system follows the publish-subscribe pattern, allowing components to communicate without direct dependencies. This promotes loose coupling and makes the system more maintainable and extensible.

```
Event Emitter → Event Bus → Event Listeners
                     ↓
              Event Queue
                     ↓
              Event Handlers
```

## Core Concepts

### Event Types

Events in Vantis Media Player are categorized into several types:

1. **Lifecycle Events**: Player lifecycle events (ready, load, destroy)
2. **Playback Events**: Playback control events (play, pause, seek, ended)
3. **State Events**: State change events (volume, rate, mute)
4. **Media Events**: Media-specific events (video, audio, subtitles)
5. **Error Events**: Error events (player errors, media errors)
6. **Custom Events**: User-defined events for plugins

### Event Flow

```
Component A              Event System              Component B
    │                          │                        │
    │ emit('event', data)      │                        │
    ├─────────────────────────►│                        │
    │                          │                        │
    │                          │─► queue(event, data)   │
    │                          │                        │
    │                          │─► dispatch(event)      │
    │                          │   │                    │
    │                          │   ├─► listener1()      │
    │                          │   ├─► listener2() ◄────┤
    │                          │   └─► listener3()      │
```

## Event API

### Emitting Events

```typescript
// Basic emission
player.emit('player:play');

// With data
player.emit('player:seek', { time: 120 });

// Multiple listeners will all receive the event
```

### Listening to Events

```typescript
// Subscribe to event
const callback = (data) => {
  console.log('Event received:', data);
};

player.on('player:play', callback);

// Unsubscribe
player.off('player:play', callback);

// One-time listener
player.once('player:ended', () => {
  console.log('Playback ended');
});
```

### Event Listeners

Event listeners can be:
- **Regular listeners**: Called every time the event is emitted
- **One-time listeners**: Called once then automatically removed
- **Prioritized listeners**: Called in order of priority

```typescript
// Regular listener
player.on('player:timeupdate', (time) => {
  console.log('Current time:', time);
});

// One-time listener
player.once('player:ready', () => {
  console.log('Player is ready');
});

// Prioritized listener (higher priority = called first)
player.on('player:error', handleError, { priority: 10 });
```

## Standard Events

### Lifecycle Events

#### player:ready

Emitted when the player is fully initialized and ready to accept media.

```typescript
player.on('player:ready', () => {
  console.log('Player is ready');
});
```

#### player:load

Emitted when media is successfully loaded.

```typescript
player.on('player:load', (source) => {
  console.log('Loaded:', source);
  
  // source = {
  //   type: 'video' | 'audio' | 'hls' | 'dash',
  //   src: string,
  //   title?: string
  // }
});
```

#### player:destroy

Emitted when the player is being destroyed.

```typescript
player.on('player:destroy', () => {
  console.log('Player destroyed');
});
```

### Playback Events

#### player:play

Emitted when playback starts.

```typescript
player.on('player:play', () => {
  console.log('Playback started');
});
```

#### player:pause

Emitted when playback is paused.

```typescript
player.on('player:pause', () => {
  console.log('Playback paused');
});
```

#### player:ended

Emitted when media playback reaches the end.

```typescript
player.on('player:ended', () => {
  console.log('Playback ended');
});
```

#### player:seek

Emitted when the seek operation is performed.

```typescript
player.on('player:seek', (time) => {
  console.log('Seeked to:', time);
});
```

#### player:timeupdate

Emitted periodically during playback with current time.

```typescript
player.on('player:timeupdate', (time) => {
  console.log('Current time:', time);
  // Update progress bar
  updateProgressBar(time);
});
```

### State Events

#### player:volumechange

Emitted when volume changes.

```typescript
player.on('player:volumechange', (volume) => {
  console.log('Volume:', volume);
});
```

#### player:ratechange

Emitted when playback rate changes.

```typescript
player.on('player:ratechange', (rate) => {
  console.log('Playback rate:', rate);
});
```

#### player:mutechange

Emitted when mute state changes.

```typescript
player.on('player:mutechange', (muted) => {
  console.log('Muted:', muted);
});
```

#### player:fullscreenchange

Emitted when fullscreen state changes.

```typescript
player.on('player:fullscreenchange', (isFullscreen) => {
  console.log('Fullscreen:', isFullscreen);
});
```

### Video Events

#### video:ready

Emitted when video decoder is ready.

```typescript
player.on('video:ready', () => {
  console.log('Video decoder ready');
});
```

#### video:qualitychange

Emitted when video quality changes.

```typescript
player.on('video:qualitychange', (quality) => {
  console.log('Quality changed:', quality);
  
  // quality = {
  //   id: string,
  //   label: string,
  //   width: number,
  //   height: number,
  //   bitrate: number
  // }
});
```

### Audio Events

#### audio:ready

Emitted when audio decoder is ready.

```typescript
player.on('audio:ready', () => {
  console.log('Audio decoder ready');
});
```

#### audio:trackchange

Emitted when audio track changes.

```typescript
player.on('audio:trackchange', (track) => {
  console.log('Audio track changed:', track);
});
```

### Subtitle Events

#### subtitle:ready

Emitted when subtitle renderer is ready.

```typescript
player.on('subtitle:ready', () => {
  console.log('Subtitle renderer ready');
});
```

#### subtitle:trackchange

Emitted when subtitle track changes.

```typescript
player.on('subtitle:trackchange', (track) => {
  console.log('Subtitle track changed:', track);
});
```

### Error Events

#### player:error

Emitted when a player error occurs.

```typescript
player.on('player:error', (error) => {
  console.error('Player error:', error);
  
  // error = {
  //   code: string,
  //   message: string,
  //   fatal: boolean,
  //   details?: any
  // }
});
```

Error codes:
- `NETWORK_ERROR`: Network-related error
- `DECODE_ERROR`: Media decoding error
- `SOURCE_ERROR`: Source loading error
- `DRM_ERROR`: DRM-related error
- `PLUGIN_ERROR`: Plugin error

### Streaming Events

#### streaming:qualitychange

Emitted during adaptive streaming when quality changes.

```typescript
player.on('streaming:qualitychange', (data) => {
  console.log('Streaming quality changed:', data);
});
```

#### streaming:buffer

Emitted when buffering starts or ends.

```typescript
player.on('streaming:buffer', (data) => {
  console.log('Buffer status:', data);
  // data = { buffering: boolean, percent?: number }
});
```

## Custom Events

Plugins and applications can emit custom events for communication.

### Emitting Custom Events

```typescript
// Emit custom event
player.emit('myapp:custom', { 
  data: 'some data',
  timestamp: Date.now()
});

// Listen to custom event
player.on('myapp:custom', (data) => {
  console.log('Custom event:', data);
});
```

### Namespaced Events

Use namespacing to avoid event name collisions:

```typescript
// Plugin-specific events
player.emit('plugin:analytics:track', {
  event: 'play',
  timestamp: Date.now()
});

player.on('plugin:analytics:track', (data) => {
  console.log('Analytics event:', data);
});
```

## Event Propagation

Events can propagate through the component hierarchy:

```
VideoEngine.emit('video:ready')
    ↓
Player.emit('video:ready')
    ↓
Plugin.emit('video:ready')
    ↓
Application.emit('video:ready')
```

### Stopping Propagation

```typescript
player.on('video:qualitychange', (data, stopPropagation) => {
  console.log('Quality changed:', data);
  
  // Stop further propagation
  stopPropagation();
});
```

## Event Batching

For performance-critical scenarios, events can be batched:

```typescript
// Enable event batching
player.enableEventBatching({
  maxDelay: 100, // ms
  maxEvents: 10
});

// Events will be batched and emitted together
```

## Event Filtering

Events can be filtered based on conditions:

```typescript
// Only receive timeupdate events every 1 second
player.on('player:timeupdate', (time) => {
  console.log('Time:', time);
}, {
  filter: (time) => Math.floor(time) % 1 === 0
});
```

## Event History

The event system can maintain a history of events:

```typescript
// Enable event history
player.enableEventHistory(100); // Keep last 100 events

// Get event history
const history = player.getEventHistory();

// Filter history
const errors = history.filter(e => e.name === 'player:error');
```

## Event Debugging

Enable event debugging for development:

```typescript
// Log all events
player.enableEventDebugging(true);

// Log specific events
player.enableEventDebugging(true, ['player:play', 'player:pause']);

// Event handler will log:
// [Event] player:play at 2024-01-01T12:00:00.000Z
```

## Performance Considerations

### Event Listener Cleanup

Always clean up event listeners to prevent memory leaks:

```typescript
class MyComponent {
  private listeners: Function[] = [];
  
  setup() {
    const handler1 = (data) => console.log(data);
    const handler2 = (data) => console.log(data);
    
    this.player.on('player:play', handler1);
    this.player.on('player:pause', handler2);
    
    // Store references
    this.listeners.push(handler1, handler2);
  }
  
  cleanup() {
    // Remove all listeners
    this.listeners.forEach(listener => {
      this.player.off('player:play', listener);
      this.player.off('player:pause', listener);
    });
    this.listeners = [];
  }
}
```

### Debouncing timeupdate Events

The `timeupdate` event fires frequently. Debounce if needed:

```typescript
let lastUpdateTime = 0;
const updateInterval = 100; // ms

player.on('player:timeupdate', (time) => {
  const now = Date.now();
  if (now - lastUpdateTime >= updateInterval) {
    updateUI(time);
    lastUpdateTime = now;
  }
});
```

### Use once for One-Time Events

For events that should only be handled once:

```typescript
// Good - use once()
player.once('player:ended', () => {
  showEndScreen();
});

// Bad - manual cleanup needed
player.on('player:ended', () => {
  showEndScreen();
  player.off('player:ended', /* need reference */);
});
```

## Event System Best Practices

1. **Use namespacing**: Prevent event name collisions
2. **Clean up listeners**: Prevent memory leaks
3. **Use once()**: For one-time event handling
4. **Debounce frequent events**: Optimize performance
5. **Type-safe events**: Use TypeScript interfaces
6. **Document custom events**: Add JSDoc comments
7. **Handle errors gracefully**: Always catch errors in listeners

## Advanced Patterns

### Event Middleware

Add middleware to transform events:

```typescript
player.addEventMiddleware((event, data) => {
  // Transform data before listeners receive it
  if (event === 'player:timeupdate') {
    return Math.round(data);
  }
  return data;
});
```

### Event Aggregators

Aggregate multiple events:

```typescript
class EventAggregator {
  private events: Map<string, any> = new Map();
  
  add(event: string, data: any) {
    this.events.set(event, data);
    this.check();
  }
  
  check() {
    if (this.events.has('player:ready') && 
        this.events.has('video:ready') &&
        this.events.has('audio:ready')) {
      this.onAllReady();
    }
  }
  
  onAllReady() {
    console.log('All components ready!');
  }
}
```

### Event Replay

Replay events for testing or recovery:

```typescript
// Replay all timeupdate events
const history = player.getEventHistory()
  .filter(e => e.name === 'player:timeupdate');

history.forEach(event => {
  console.log('Replaying:', event);
});
```

## Related Documentation

- [Overview](./overview) - High-level architecture
- [Components](./components) - Component details
- [State Management](./state-management) - State management
- [Plugin System](./plugin-system) - Plugin architecture