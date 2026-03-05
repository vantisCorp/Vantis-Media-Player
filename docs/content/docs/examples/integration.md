---
sidebar_position: 5
---

# Integration Examples

This section provides complete integration examples for Vantis Media Player with popular frameworks and libraries.

## React Integration

### Basic React Component

```jsx
// VantisPlayer.jsx

import React, { useEffect, useRef, useState, useCallback } from 'react';
import VantisPlayer from '@vantis/player';
import '@vantis/player/dist/vantis.min.css';

function VantisPlayer({ 
  source,
  options = {},
  onPlay,
  onPause,
  onEnded,
  onError,
  onReady,
  onTimeUpdate,
  className = ''
}) {
  const containerRef = useRef(null);
  const playerRef = useRef(null);
  const [isReady, setIsReady] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [volume, setVolume] = useState(0.8);
  
  // Initialize player
  useEffect(() => {
    if (!containerRef.current) return;
    
    playerRef.current = new VantisPlayer(containerRef.current, {
      controls: true,
      autoplay: false,
      volume: 0.8,
      ...options
    });
    
    const player = playerRef.current;
    
    // Setup event handlers
    player.on('player:ready', () => {
      setIsReady(true);
      onReady?.();
    });
    
    player.on('player:play', () => {
      setIsPlaying(true);
      onPlay?.();
    });
    
    player.on('player:pause', () => {
      setIsPlaying(false);
      onPause?.();
    });
    
    player.on('player:ended', () => {
      setIsPlaying(false);
      onEnded?.();
    });
    
    player.on('player:timeupdate', (time) => {
      setCurrentTime(time);
      onTimeUpdate?.(time);
    });
    
    player.on('player:duration', (dur) => {
      setDuration(dur);
    });
    
    player.on('player:volumechange', (vol) => {
      setVolume(vol);
    });
    
    player.on('player:error', (error) => {
      onError?.(error);
    });
    
    return () => {
      if (playerRef.current) {
        playerRef.current.destroy();
        playerRef.current = null;
      }
    };
  }, []);
  
  // Load source when it changes
  useEffect(() => {
    if (playerRef.current && source && isReady) {
      playerRef.current.load(source);
    }
  }, [source, isReady]);
  
  // Expose player methods via ref
  const controls = useMemo(() => ({
    play: () => playerRef.current?.play(),
    pause: () => playerRef.current?.pause(),
    seek: (time) => playerRef.current?.seek(time),
    setVolume: (vol) => playerRef.current?.setVolume(vol),
    setMuted: (muted) => playerRef.current?.setMuted(muted),
    getCurrentTime: () => playerRef.current?.getCurrentTime(),
    getDuration: () => playerRef.current?.getDuration(),
    getState: () => playerRef.current?.getState()
  }), []);
  
  useImperativeHandle(forwardedRef, () => controls);
  
  return (
    <div 
      ref={containerRef} 
      className={`vantis-player ${className}`}
      style={{ width: '100%', aspectRatio: '16/9' }}
    />
  );
}

// Forward ref version
const ForwardedVantisPlayer = React.forwardRef((props, ref) => (
  <VantisPlayer {...props} forwardedRef={ref} />
));

export default ForwardedVantisPlayer;
```

### Advanced React Hook

```jsx
// useVantisPlayer.js

import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import VantisPlayer from '@vantis/player';

export function useVantisPlayer(options = {}) {
  const containerRef = useRef(null);
  const playerRef = useRef(null);
  const [state, setState] = useState({
    isReady: false,
    isPlaying: false,
    currentTime: 0,
    duration: 0,
    volume: 0.8,
    muted: false,
    buffered: 0,
    error: null
  });
  
  // Initialize player
  useEffect(() => {
    if (!containerRef.current) return;
    
    const player = new VantisPlayer(containerRef.current, {
      controls: true,
      autoplay: false,
      volume: 0.8,
      ...options
    });
    
    playerRef.current = player;
    
    // Event handlers
    const handleReady = () => setState(s => ({ ...s, isReady: true }));
    const handlePlay = () => setState(s => ({ ...s, isPlaying: true }));
    const handlePause = () => setState(s => ({ ...s, isPlaying: false }));
    const handleEnded = () => setState(s => ({ ...s, isPlaying: false }));
    const handleTimeUpdate = (time) => setState(s => ({ ...s, currentTime: time }));
    const handleDuration = (dur) => setState(s => ({ ...s, duration: dur }));
    const handleVolumeChange = (vol) => setState(s => ({ ...s, volume: vol }));
    const handleMuteChange = (muted) => setState(s => ({ ...s, muted }));
    const handleError = (error) => setState(s => ({ ...s, error }));
    
    player.on('player:ready', handleReady);
    player.on('player:play', handlePlay);
    player.on('player:pause', handlePause);
    player.on('player:ended', handleEnded);
    player.on('player:timeupdate', handleTimeUpdate);
    player.on('player:duration', handleDuration);
    player.on('player:volumechange', handleVolumeChange);
    player.on('player:mutechange', handleMuteChange);
    player.on('player:error', handleError);
    
    return () => {
      player.destroy();
      playerRef.current = null;
    };
  }, []);
  
  // Control methods
  const play = useCallback(() => playerRef.current?.play(), []);
  const pause = useCallback(() => playerRef.current?.pause(), []);
  const seek = useCallback((time) => playerRef.current?.seek(time), []);
  const setVolume = useCallback((vol) => playerRef.current?.setVolume(vol), []);
  const setMuted = useCallback((muted) => playerRef.current?.setMuted(muted), []);
  const load = useCallback((source) => playerRef.current?.load(source), []);
  const getState = useCallback(() => playerRef.current?.getState(), []);
  
  return {
    containerRef,
    state,
    controls: {
      play,
      pause,
      seek,
      setVolume,
      setMuted,
      load,
      getState
    }
  };
}

// Usage
function VideoPlayer({ source }) {
  const { containerRef, state, controls } = useVantisPlayer({
    autoplay: false,
    volume: 0.8
  });
  
  useEffect(() => {
    if (state.isReady && source) {
      controls.load(source);
    }
  }, [state.isReady, source]);
  
  return (
    <div>
      <div ref={containerRef} style={{ width: '100%', aspectRatio: '16/9' }} />
      <div className="controls">
        <button onClick={controls.play}>Play</button>
        <button onClick={controls.pause}>Pause</button>
        <span>{state.currentTime} / {state.duration}</span>
      </div>
    </div>
  );
}
```

## Vue.js Integration

### Vue 3 Composition API

```vue
<!-- VantisPlayer.vue -->

<template>
  <div 
    ref="containerRef" 
    class="vantis-player"
    :style="{ width: width, aspectRatio: aspectRatio }"
  />
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch, computed } from 'vue';
import VantisPlayer from '@vantis/player';
import '@vantis/player/dist/vantis.min.css';

const props = defineProps({
  source: {
    type: Object,
    required: true
  },
  options: {
    type: Object,
    default: () => ({})
  },
  width: {
    type: String,
    default: '100%'
  },
  aspectRatio: {
    type: String,
    default: '16 / 9'
  },
  autoplay: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits([
  'ready', 'play', 'pause', 'ended', 
  'error', 'timeupdate', 'duration', 'volumechange'
]);

const containerRef = ref(null);
const playerRef = ref(null);
const state = ref({
  isReady: false,
  isPlaying: false,
  currentTime: 0,
  duration: 0,
  volume: 0.8,
  muted: false
});

const controls = computed(() => ({
  play: () => playerRef.value?.play(),
  pause: () => playerRef.value?.pause(),
  seek: (time) => playerRef.value?.seek(time),
  setVolume: (vol) => playerRef.value?.setVolume(vol),
  setMuted: (muted) => playerRef.value?.setMuted(muted),
  getCurrentTime: () => playerRef.value?.getCurrentTime(),
  getDuration: () => playerRef.value?.getDuration(),
  getState: () => playerRef.value?.getState()
}));

onMounted(() => {
  if (containerRef.value) {
    playerRef.value = new VantisPlayer(containerRef.value, {
      controls: true,
      autoplay: props.autoplay,
      ...props.options
    });
    
    const player = playerRef.value;
    
    player.on('player:ready', () => {
      state.value.isReady = true;
      emit('ready');
    });
    
    player.on('player:play', () => {
      state.value.isPlaying = true;
      emit('play');
    });
    
    player.on('player:pause', () => {
      state.value.isPlaying = false;
      emit('pause');
    });
    
    player.on('player:ended', () => {
      state.value.isPlaying = false;
      emit('ended');
    });
    
    player.on('player:timeupdate', (time) => {
      state.value.currentTime = time;
      emit('timeupdate', time);
    });
    
    player.on('player:duration', (duration) => {
      state.value.duration = duration;
      emit('duration', duration);
    });
    
    player.on('player:volumechange', (volume) => {
      state.value.volume = volume;
      emit('volumechange', volume);
    });
    
    player.on('player:mutechange', (muted) => {
      state.value.muted = muted;
    });
    
    player.on('player:error', (error) => {
      emit('error', error);
    });
  }
});

onUnmounted(() => {
  if (playerRef.value) {
    playerRef.value.destroy();
    playerRef.value = null;
  }
});

watch(() => props.source, async (newSource) => {
  if (playerRef.value && newSource && state.value.isReady) {
    await playerRef.value.load(newSource);
  }
}, { deep: true });

// Expose controls
defineExpose({
  controls,
  state
});
</script>
```

### Vue 2 Options API

```vue
<!-- VantisPlayer.vue -->

<template>
  <div 
    ref="containerRef" 
    class="vantis-player"
    :style="{ width: width }"
  />
</template>

<script>
import VantisPlayer from '@vantis/player';
import '@vantis/player/dist/vantis.min.css';

export default {
  name: 'VantisPlayer',
  
  props: {
    source: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      default: () => ({})
    },
    width: {
      type: String,
      default: '100%'
    },
    autoplay: {
      type: Boolean,
      default: false
    }
  },
  
  data() {
    return {
      player: null,
      isReady: false,
      isPlaying: false,
      currentTime: 0,
      duration: 0,
      volume: 0.8,
      muted: false
    };
  },
  
  watch: {
    source: {
      deep: true,
      handler(newSource) {
        if (this.player && newSource && this.isReady) {
          this.player.load(newSource);
        }
      }
    }
  },
  
  mounted() {
    this.initPlayer();
  },
  
  beforeUnmount() {
    this.destroyPlayer();
  },
  
  methods: {
    initPlayer() {
      this.player = new VantisPlayer(this.$refs.containerRef, {
        controls: true,
        autoplay: this.autoplay,
        ...this.options
      });
      
      this.player.on('player:ready', () => {
        this.isReady = true;
        this.$emit('ready');
      });
      
      this.player.on('player:play', () => {
        this.isPlaying = true;
        this.$emit('play');
      });
      
      this.player.on('player:pause', () => {
        this.isPlaying = false;
        this.$emit('pause');
      });
      
      this.player.on('player:ended', () => {
        this.isPlaying = false;
        this.$emit('ended');
      });
      
      this.player.on('player:timeupdate', (time) => {
        this.currentTime = time;
        this.$emit('timeupdate', time);
      });
      
      this.player.on('player:duration', (duration) => {
        this.duration = duration;
        this.$emit('duration', duration);
      });
      
      this.player.on('player:volumechange', (volume) => {
        this.volume = volume;
        this.$emit('volumechange', volume);
      });
      
      this.player.on('player:error', (error) => {
        this.$emit('error', error);
      });
    },
    
    destroyPlayer() {
      if (this.player) {
        this.player.destroy();
        this.player = null;
      }
    },
    
    play() {
      this.player?.play();
    },
    
    pause() {
      this.player?.pause();
    },
    
    seek(time) {
      this.player?.seek(time);
    },
    
    setVolume(volume) {
      this.player?.setVolume(volume);
    }
  }
};
</script>
```

## Angular Integration

```typescript
// vantis-player.component.ts

import { Component, AfterViewInit, OnDestroy, Input, Output, EventEmitter, ElementRef } from '@angular/core';
import VantisPlayer from '@vantis/player';

@Component({
  selector: 'app-vantis-player',
  template: `
    <div #playerContainer class="vantis-player-container"></div>
  `,
  styles: [`
    .vantis-player-container {
      width: 100%;
      aspect-ratio: 16 / 9;
    }
  `]
})
export class VantisPlayerComponent implements AfterViewInit, OnDestroy {
  @Input() source: MediaSource;
  @Input() options: PlayerOptions = {};
  @Input() autoplay = false;
  
  @Output() ready = new EventEmitter<void>();
  @Output() play = new EventEmitter<void>();
  @Output() pause = new EventEmitter<void>();
  @Output() ended = new EventEmitter<void>();
  @Output() error = new EventEmitter<any>();
  @Output() timeUpdate = new EventEmitter<number>();
  
  @ViewChild('playerContainer', { static: true }) playerContainer: ElementRef;
  
  private player: VantisPlayer;
  private isReady = false;
  
  ngAfterViewInit() {
    this.initPlayer();
  }
  
  ngOnDestroy() {
    this.destroyPlayer();
  }
  
  private initPlayer() {
    this.player = new VantisPlayer(this.playerContainer.nativeElement, {
      controls: true,
      autoplay: this.autoplay,
      ...this.options
    });
    
    this.player.on('player:ready', () => {
      this.isReady = true;
      this.ready.emit();
      
      if (this.source) {
        this.player.load(this.source);
      }
    });
    
    this.player.on('player:play', () => this.play.emit());
    this.player.on('player:pause', () => this.pause.emit());
    this.player.on('player:ended', () => this.ended.emit());
    this.player.on('player:error', (err) => this.error.emit(err));
    this.player.on('player:timeupdate', (time) => this.timeUpdate.emit(time));
  }
  
  private destroyPlayer() {
    if (this.player) {
      this.player.destroy();
      this.player = null;
    }
  }
  
  // Public methods
  public play(): void {
    this.player?.play();
  }
  
  public pause(): void {
    this.player?.pause();
  }
  
  public seek(time: number): void {
    this.player?.seek(time);
  }
  
  public setVolume(volume: number): void {
    this.player?.setVolume(volume);
  }
  
  public load(source: MediaSource): void {
    if (this.player && this.isReady) {
      this.player.load(source);
    }
  }
}
```

## Svelte Integration

```svelte
<!-- VantisPlayer.svelte -->

<script>
  import { onMount, onDestroy, afterUpdate } from 'svelte';
  import VantisPlayer from '@vantis/player';
  import '@vantis/player/dist/vantis.min.css';
  
  export let source;
  export let options = {};
  export let width = '100%';
  export let autoplay = false;
  
  export let onReady;
  export let onPlay;
  export let onPause;
  export let onEnded;
  export let onError;
  
  let container;
  let player;
  let isReady = false;
  let isPlaying = false;
  let currentTime = 0;
  let duration = 0;
  let volume = 0.8;
  
  onMount(() => {
    player = new VantisPlayer(container, {
      controls: true,
      autoplay,
      ...options
    });
    
    player.on('player:ready', () => {
      isReady = true;
      onReady?.();
    });
    
    player.on('player:play', () => {
      isPlaying = true;
      onPlay?.();
    });
    
    player.on('player:pause', () => {
      isPlaying = false;
      onPause?.();
    });
    
    player.on('player:ended', () => {
      isPlaying = false;
      onEnded?.();
    });
    
    player.on('player:timeupdate', (time) => {
      currentTime = time;
    });
    
    player.on('player:duration', (dur) => {
      duration = dur;
    });
    
    player.on('player:volumechange', (vol) => {
      volume = vol;
    });
    
    player.on('player:error', (err) => {
      onError?.(err);
    });
  });
  
  onDestroy(() => {
    if (player) {
      player.destroy();
    }
  });
  
  // Load source when it changes
  $: if (isReady && source) {
    player.load(source);
  }
  
  // Expose controls
  function play() {
    player?.play();
  }
  
  function pause() {
    player?.pause();
  }
  
  function seek(time) {
    player?.seek(time);
  }
  
  function setVolume(vol) {
    player?.setVolume(vol);
  }
  
  // Bind controls
  const controls = {
    play,
    pause,
    seek,
    setVolume
  };
</script>

<div 
  bind:this={container} 
  class="vantis-player"
  style:width={width}
  style:aspect-ratio="16 / 9"
/>

<slot {controls} {state} />
```

## Next.js Integration

```jsx
// pages/video-player.js

import dynamic from 'next/dynamic';
import { useState, useEffect } from 'react';

// Dynamically import Vantis Player to avoid SSR issues
const VantisPlayer = dynamic(
  () => import('@/components/VantisPlayer'),
  { ssr: false }
);

export default function VideoPlayerPage() {
  const [source, setSource] = useState(null);
  
  useEffect(() => {
    // Set source after client-side hydration
    setSource({
      type: 'video',
      src: 'https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4'
    });
  }, []);
  
  return (
    <div>
      <h1>Video Player</h1>
      {source && (
        <VantisPlayer
          source={source}
          options={{
            autoplay: false,
            volume: 0.8
          }}
          onPlay={() => console.log('Playing')}
          onPause={() => console.log('Paused')}
        />
      )}
    </div>
  );
}
```

## Nuxt.js Integration

```vue
<!-- components/VantisPlayer.vue -->

<template>
  <ClientOnly>
    <div ref="container" class="vantis-player"></div>
  </ClientOnly>
</template>

<script>
export default {
  props: {
    source: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      default: () => ({})
    }
  },
  
  mounted() {
    if (process.client) {
      this.initPlayer();
    }
  },
  
  methods: {
    async initPlayer() {
      const VantisPlayer = (await import('@vantis/player')).default;
      
      this.player = new VantisPlayer(this.$refs.container, {
        controls: true,
        ...this.options
      });
      
      await this.player.load(this.source);
    }
  },
  
  beforeDestroy() {
    if (this.player) {
      this.player.destroy();
    }
  }
};
</script>
```

## TypeScript Integration

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
  onMarkerClick?: (marker: any) => void;
}

class TypedPlayer {
  private player: VantisPlayer | null = null;
  private config: MyPlayerConfig;
  private eventListeners: Map<keyof PlayerEvents, Function[]> = new Map();
  
  constructor(container: HTMLElement, config: MyPlayerConfig = {}) {
    this.config = config;
    this.player = new VantisPlayer(container, {
      controls: true,
      autoplay: false,
      ...config
    });
    
    this.setupEventHandlers();
  }
  
  private setupEventHandlers(): void {
    if (!this.player) return;
    
    const events: (keyof PlayerEvents)[] = [
      'player:ready', 'player:play', 'player:pause',
      'player:ended', 'player:error', 'player:timeupdate'
    ];
    
    events.forEach(event => {
      this.player!.on(event, (data: any) => {
        this.emit(event, data);
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
  
  setVolume(volume: number): void {
    this.player?.setVolume(volume);
  }
  
  getState(): PlayerState | undefined {
    return this.player?.getState();
  }
  
  on<K extends keyof PlayerEvents>(
    event: K,
    callback: PlayerEvents[K]
  ): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)?.push(callback);
  }
  
  off<K extends keyof PlayerEvents>(
    event: K,
    callback: PlayerEvents[K]
  ): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      const index = listeners.indexOf(callback);
      if (index !== -1) {
        listeners.splice(index, 1);
      }
    }
  }
  
  private emit<K extends keyof PlayerEvents>(
    event: K,
    data: Parameters<PlayerEvents[K]>[0]
  ): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach(callback => {
        try {
          (callback as Function)(data);
        } catch (error) {
          console.error(`Error in event handler for ${event}:`, error);
        }
      });
    }
  }
  
  destroy(): void {
    this.eventListeners.clear();
    if (this.player) {
      this.player.destroy();
      this.player = null;
    }
  }
}

export default TypedPlayer;
```

## Next Steps

- [API Reference](../api/overview) - Explore the full API
- [Plugin Development](./plugin-development) - Create custom plugins
- [Deployment Guide](../deployment/desktop) - Deploy your application