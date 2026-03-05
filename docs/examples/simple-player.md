---
sidebar_position: 1
---

# Simple Player Example

This example demonstrates how to create a basic video player with Vantis Media Player. It covers the essential setup, configuration, and playback controls needed to get started.

## Basic HTML Setup

Create a new HTML file with the following structure:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Simple Vantis Player</title>
  <link rel="stylesheet" href="https://cdn.vantis.media/player/latest/vantis.min.css">
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      margin: 0;
      padding: 20px;
      background: #1a1a2e;
      color: white;
    }
    
    .container {
      max-width: 900px;
      margin: 0 auto;
    }
    
    h1 {
      text-align: center;
      margin-bottom: 20px;
    }
    
    #player-container {
      width: 100%;
      aspect-ratio: 16 / 9;
      background: #000;
      border-radius: 8px;
      overflow: hidden;
    }
    
    .controls {
      margin-top: 20px;
      display: flex;
      gap: 10px;
      flex-wrap: wrap;
    }
    
    button {
      padding: 10px 20px;
      background: #6366f1;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
    }
    
    button:hover {
      background: #4f46e5;
    }
    
    input[type="text"] {
      flex: 1;
      padding: 10px;
      border: 1px solid #333;
      border-radius: 4px;
      background: #2d2d44;
      color: white;
      min-width: 200px;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Simple Vantis Player</h1>
    
    <div id="player-container"></div>
    
    <div class="controls">
      <input type="text" id="video-url" placeholder="Enter video URL...">
      <button id="load-btn">Load Video</button>
    </div>
  </div>

  <script src="https://cdn.vantis.media/player/latest/vantis.min.js"></script>
  <script src="app.js"></script>
</body>
</html>
```

## JavaScript Implementation

Create an `app.js` file with the following code:

```javascript
// app.js

// Wait for DOM to be ready
document.addEventListener('DOMContentLoaded', () => {
  // Initialize player
  const player = new VantisPlayer('#player-container', {
    // Basic configuration
    autoplay: false,
    muted: false,
    volume: 0.8,
    
    // UI settings
    controls: true,
    loading: {
      spinner: true,
      color: '#6366f1'
    }
  });
  
  // Get elements
  const videoUrlInput = document.getElementById('video-url');
  const loadBtn = document.getElementById('load-btn');
  
  // Load video on button click
  loadBtn.addEventListener('click', () => {
    const url = videoUrlInput.value.trim();
    if (url) {
      loadVideo(url);
    }
  });
  
  // Load video on Enter key
  videoUrlInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
      const url = videoUrlInput.value.trim();
      if (url) {
        loadVideo(url);
      }
    }
  });
  
  // Function to load video
  async function loadVideo(url) {
    try {
      // Show loading state
      loadBtn.textContent = 'Loading...';
      loadBtn.disabled = true;
      
      // Load video source
      await player.load({
        type: 'video',
        src: url,
        title: 'Loaded Video'
      });
      
      console.log('Video loaded successfully');
    } catch (error) {
      console.error('Failed to load video:', error);
      alert('Failed to load video: ' + error.message);
    } finally {
      // Reset button state
      loadBtn.textContent = 'Load Video';
      loadBtn.disabled = false;
    }
  }
  
  // Event listeners
  player.on('player:ready', () => {
    console.log('Player is ready');
  });
  
  player.on('player:play', () => {
    console.log('Playback started');
  });
  
  player.on('player:pause', () => {
    console.log('Playback paused');
  });
  
  player.on('player:ended', () => {
    console.log('Playback ended');
  });
  
  player.on('player:error', (error) => {
    console.error('Player error:', error);
  });
  
  // Load a default video
  loadVideo('https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4');
});
```

## Using ES Modules

For modern development, use ES modules:

```javascript
// app.mjs

import { VantisPlayer } from 'https://cdn.vantis.media/player/latest/vantis.esm.js';

class SimplePlayer {
  constructor(containerSelector) {
    this.container = document.querySelector(containerSelector);
    this.player = null;
    this.init();
  }
  
  async init() {
    // Create player instance
    this.player = new VantisPlayer(this.container, {
      controls: true,
      autoplay: false
    });
    
    // Setup event handlers
    this.setupEvents();
  }
  
  setupEvents() {
    this.player.on('player:ready', () => this.onReady());
    this.player.on('player:play', () => this.onPlay());
    this.player.on('player:pause', () => this.onPause());
    this.player.on('player:ended', () => this.onEnded());
    this.player.on('player:error', (e) => this.onError(e));
  }
  
  async load(source) {
    try {
      await this.player.load(source);
    } catch (error) {
      this.onError(error);
    }
  }
  
  play() {
    this.player.play();
  }
  
  pause() {
    this.player.pause();
  }
  
  seek(time) {
    this.player.seek(time);
  }
  
  setVolume(volume) {
    this.player.setVolume(volume);
  }
  
  // Event handlers
  onReady() {
    console.log('Player ready');
  }
  
  onPlay() {
    console.log('Playing');
  }
  
  onPause() {
    console.log('Paused');
  }
  
  onEnded() {
    console.log('Ended');
  }
  
  onError(error) {
    console.error('Error:', error);
  }
  
  destroy() {
    if (this.player) {
      this.player.destroy();
      this.player = null;
    }
  }
}

// Initialize
const simplePlayer = new SimplePlayer('#player-container');
await simplePlayer.load({
  type: 'video',
  src: 'https://example.com/video.mp4'
});
```

## React Integration

Here's how to integrate Vantis Player with React:

```jsx
// VantisPlayer.jsx

import React, { useEffect, useRef, useState } from 'react';
import VantisPlayer from '@vantis/player';

function VantisPlayerComponent({ 
  source, 
  options = {}, 
  onPlay, 
  onPause, 
  onEnded, 
  onError 
}) {
  const containerRef = useRef(null);
  const playerRef = useRef(null);
  const [isReady, setIsReady] = useState(false);
  
  useEffect(() => {
    // Initialize player
    playerRef.current = new VantisPlayer(containerRef.current, {
      controls: true,
      autoplay: false,
      ...options
    });
    
    // Event handlers
    playerRef.current.on('player:ready', () => setIsReady(true));
    
    return () => {
      if (playerRef.current) {
        playerRef.current.destroy();
      }
    };
  }, []);
  
  useEffect(() => {
    // Load source when it changes
    if (playerRef.current && source && isReady) {
      playerRef.current.load(source);
    }
  }, [source, isReady]);
  
  useEffect(() => {
    // Bind event handlers
    const player = playerRef.current;
    
    if (player) {
      if (onPlay) player.on('player:play', onPlay);
      if (onPause) player.on('player:pause', onPause);
      if (onEnded) player.on('player:ended', onEnded);
      if (onError) player.on('player:error', onError);
    }
    
    return () => {
      if (player) {
        if (onPlay) player.off('player:play', onPlay);
        if (onPause) player.off('player:pause', onPause);
        if (onEnded) player.off('player:ended', onEnded);
        if (onError) player.off('player:error', onError);
      }
    };
  }, [onPlay, onPause, onEnded, onError, isReady]);
  
  return (
    <div 
      ref={containerRef} 
      className="vantis-player-container"
      style={{ width: '100%', aspectRatio: '16/9' }}
    />
  );
}

// Usage
function App() {
  const [source] = useState({
    type: 'video',
    src: 'https://example.com/video.mp4'
  });
  
  return (
    <div className="App">
      <VantisPlayerComponent 
        source={source}
        options={{
          autoplay: false,
          volume: 0.8
        }}
        onPlay={() => console.log('Playing')}
        onPause={() => console.log('Paused')}
        onEnded={() => console.log('Ended')}
        onError={(e) => console.error('Error:', e)}
      />
    </div>
  );
}

export default VantisPlayerComponent;
```

## Vue.js Integration

Integration with Vue 3:

```vue
<!-- VantisPlayer.vue -->

<template>
  <div ref="containerRef" class="vantis-player"></div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue';
import VantisPlayer from '@vantis/player';

const props = defineProps({
  source: {
    type: Object,
    required: true
  },
  options: {
    type: Object,
    default: () => ({})
  }
});

const emit = defineEmits(['play', 'pause', 'ended', 'error', 'ready']);

const containerRef = ref(null);
const playerRef = ref(null);

onMounted(() => {
  // Initialize player
  playerRef.value = new VantisPlayer(containerRef.value, {
    controls: true,
    autoplay: false,
    ...props.options
  });
  
  // Setup event handlers
  playerRef.value.on('player:ready', () => emit('ready'));
  playerRef.value.on('player:play', () => emit('play'));
  playerRef.value.on('player:pause', () => emit('pause'));
  playerRef.value.on('player:ended', () => emit('ended'));
  playerRef.value.on('player:error', (e) => emit('error', e));
  
  // Load initial source
  if (props.source) {
    playerRef.value.load(props.source);
  }
});

onUnmounted(() => {
  if (playerRef.value) {
    playerRef.value.destroy();
  }
});

watch(() => props.source, (newSource) => {
  if (playerRef.value && newSource) {
    playerRef.value.load(newSource);
  }
});

// Expose player methods
defineExpose({
  play: () => playerRef.value?.play(),
  pause: () => playerRef.value?.pause(),
  seek: (time) => playerRef.value?.seek(time),
  setVolume: (vol) => playerRef.value?.setVolume(vol)
});
</script>

<style scoped>
.vantis-player {
  width: 100%;
  aspect-ratio: 16 / 9;
  background: #000;
}
</style>
```

## TypeScript Usage

For TypeScript projects:

```typescript
// player.ts

import VantisPlayer, { 
  PlayerOptions, 
  MediaSource, 
  PlayerState,
  PlayerEvents 
} from '@vantis/player';

interface MyPlayerConfig extends PlayerOptions {
  analytics?: boolean;
  debug?: boolean;
}

class MyPlayer {
  private player: VantisPlayer | null = null;
  private container: HTMLElement;
  private config: MyPlayerConfig;
  
  constructor(container: string | HTMLElement, config: MyPlayerConfig = {}) {
    this.container = typeof container === 'string' 
      ? document.querySelector(container)! 
      : container;
    this.config = config;
    this.init();
  }
  
  private init(): void {
    this.player = new VantisPlayer(this.container, {
      controls: true,
      autoplay: false,
      volume: 0.8,
      ...this.config
    });
    
    this.setupEventListeners();
  }
  
  private setupEventListeners(): void {
    if (!this.player) return;
    
    const events: (keyof PlayerEvents)[] = [
      'player:ready',
      'player:play',
      'player:pause',
      'player:ended',
      'player:error'
    ];
    
    events.forEach(event => {
      this.player!.on(event, (data) => {
        if (this.config.debug) {
          console.log(`[Player] ${event}:`, data);
        }
      });
    });
  }
  
  async load(source: MediaSource): Promise<void> {
    if (!this.player) {
      throw new Error('Player not initialized');
    }
    
    return this.player.load(source);
  }
  
  play(): void {
    this.player?.play();
  }
  
  pause(): void {
    this.player?.pause();
  }
  
  seek(time: number): void {
    this.player?.seek(time);
  }
  
  getState(): PlayerState {
    return this.player?.getState() ?? 'idle';
  }
  
  destroy(): void {
    this.player?.destroy();
    this.player = null;
  }
}

export default MyPlayer;
```

## Next Steps

- [Advanced Player Example](./advanced-player) - Learn about advanced features
- [Custom Controls](./custom-controls) - Create your own player UI
- [API Reference](../api/overview) - Explore the full API