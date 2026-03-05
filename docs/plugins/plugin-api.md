---
sidebar_position: 2
---

# Plugin API Reference

The Vantis Media Player Plugin API provides a comprehensive set of interfaces, methods, and events for building powerful extensions. This section details the complete API reference for plugin development.

## Core Interfaces

### Plugin Interface

Every plugin must implement the base Plugin interface:

```typescript
interface Plugin {
  // Plugin metadata
  id: string;
  name: string;
  version: string;
  description?: string;
  author?: string;
  
  // Lifecycle hooks
  onLoad(player: Player): void | Promise<void>;
  onUnload(): void | Promise<void>;
  
  // Optional configuration
  config?: PluginConfig;
}
```

### PluginConfig Interface

```typescript
interface PluginConfig {
  // Configuration schema
  schema: Record<string, ConfigField>;
  
  // Default values
  defaults?: Record<string, any>;
  
  // Validation function
  validate?: (config: Record<string, any>) => boolean;
}

interface ConfigField {
  type: 'string' | 'number' | 'boolean' | 'object' | 'array';
  required?: boolean;
  default?: any;
  description?: string;
  enum?: any[];
  min?: number;
  max?: number;
}
```

## Player API Access

Plugins receive access to the Player instance through the `onLoad` hook. The Player API includes:

### Player Methods

```typescript
interface Player {
  // Core playback
  play(): Promise<void>;
  pause(): Promise<void>;
  stop(): Promise<void>;
  seek(time: number): Promise<void>;
  
  // Media loading
  load(source: MediaSource): Promise<void>;
  unload(): Promise<void>;
  
  // State access
  getState(): PlayerState;
  getDuration(): number;
  getCurrentTime(): number;
  
  // Volume and playback rate
  setVolume(volume: number): void;
  getVolume(): number;
  setPlaybackRate(rate: number): void;
  getPlaybackRate(): number;
  
  // Video controls
  setVideoTrack(trackId: string): void;
  setAudioTrack(trackId: string): void;
  setSubtitleTrack(trackId: string): void;
  
  // UI access
  getUI(): UI;
  registerComponent(component: UIComponent): void;
  unregisterComponent(componentId: string): void;
}
```

### Player Events

Plugins can subscribe to player events:

```typescript
interface PlayerEvents {
  'player:ready': () => void;
  'player:load': (source: MediaSource) => void;
  'player:play': () => void;
  'player:pause': () => void;
  'player:ended': () => void;
  'player:error': (error: PlayerError) => void;
  'player:seek': (time: number) => void;
  'player:timeupdate': (time: number) => void;
  'player:volumechange': (volume: number) => void;
  'player:ratechange': (rate: number) => void;
  'player:fullscreenchange': (isFullscreen: boolean) => void;
  
  // Media events
  'video:ready': () => void;
  'video:trackchange': (trackId: string) => void;
  'audio:trackchange': (trackId: string) => void;
  'subtitle:trackchange': (trackId: string) => void;
  
  // Buffer events
  'buffer:start': () => void;
  'buffer:end': () => void;
  'buffer:progress': (percent: number) => void;
}

interface Player {
  on<K extends keyof PlayerEvents>(
    event: K,
    callback: PlayerEvents[K]
  ): void;
  off<K extends keyof PlayerEvents>(
    event: K,
    callback: PlayerEvents[K]
  ): void;
  once<K extends keyof PlayerEvents>(
    event: K,
    callback: PlayerEvents[K]
  ): void;
}
```

## Hook System

Plugins can register hooks to modify or intercept player behavior:

### Available Hooks

```typescript
interface HookSystem {
  // Lifecycle hooks
  beforeLoad(source: MediaSource): Promise<MediaSource>;
  afterLoad(source: MediaSource): void;
  
  beforePlay(): Promise<void>;
  afterPlay(): void;
  
  beforePause(): Promise<void>;
  afterPause(): void;
  
  beforeSeek(time: number): Promise<number>;
  afterSeek(time: number): void;
  
  // Media hooks
  beforeVideoTrackChange(trackId: string): Promise<string>;
  afterVideoTrackChange(trackId: string): void;
  
  beforeSubtitleLoad(subtitle: SubtitleSource): Promise<SubtitleSource>;
  afterSubtitleLoad(subtitle: SubtitleSource): void;
  
  // Rendering hooks
  beforeRender(frame: VideoFrame): Promise<VideoFrame | null>;
  afterRender(frame: VideoFrame): void;
  
  // Audio hooks
  beforeAudioProcess(audio: AudioBuffer): Promise<AudioBuffer>;
  afterAudioProcess(audio: AudioBuffer): void;
}

interface Player {
  hooks: HookSystem;
}
```

### Hook Registration

```typescript
// TypeScript/JavaScript
class MyPlugin implements Plugin {
  onLoad(player: Player) {
    player.hooks.beforePlay = async () => {
      console.log('About to play!');
      await this.preparePlayback();
    };
    
    player.hooks.beforeRender = async (frame) => {
      // Modify frame before rendering
      return this.processFrame(frame);
    };
  }
}
```

```rust
// Rust
impl Plugin for MyPlugin {
    fn on_load(&mut self, player: &mut Player) {
        player.hooks().before_play(Box::new(|_| {
            println!("About to play!");
            Box::pin(async { Ok(()) })
        }));
        
        player.hooks().before_render(Box::new(|frame| {
            Box::pin(async {
                Ok(Some(process_frame(frame)))
            })
        }));
    }
}
```

## Custom Events

Plugins can emit and listen to custom events:

```typescript
interface Player {
  emit(event: string, data?: any): void;
  on(event: string, callback: (data?: any) => void): void;
  off(event: string, callback: (data?: any) => void): void;
}

// Emitting custom events
class AnalyticsPlugin implements Plugin {
  onLoad(player: Player) {
    player.emit('analytics:start', {
      plugin: 'analytics',
      timestamp: Date.now()
    });
  }
}

// Listening to custom events
class LoggerPlugin implements Plugin {
  onLoad(player: Player) {
    player.on('analytics:start', (data) => {
      console.log('Analytics started:', data);
    });
  }
}
```

## Storage API

Plugins can access persistent storage:

```typescript
interface StorageAPI {
  // Local storage (per domain)
  get(key: string): Promise<any>;
  set(key: string, value: any): Promise<void>;
  remove(key: string): Promise<void>;
  clear(): Promise<void>;
  
  // Session storage (cleared on session end)
  getSession(key: string): Promise<any>;
  setSession(key: string, value: any): Promise<void>;
  removeSession(key: string): Promise<void>;
  
  // IndexedDB for larger data
  getDB(): Promise<IDBDatabase>;
}

interface Player {
  storage: StorageAPI;
}

// Usage
class SettingsPlugin implements Plugin {
  async onLoad(player: Player) {
    const settings = await player.storage.get('settings') || {};
    this.applySettings(settings);
  }
  
  async saveSettings(settings: any) {
    await this.player.storage.set('settings', settings);
  }
}
```

## Network API

Plugins can make network requests:

```typescript
interface NetworkAPI {
  fetch(url: string, options?: RequestInit): Promise<Response>;
  download(url: string, options?: DownloadOptions): Promise<Blob>;
  upload(url: string, data: FormData): Promise<UploadResponse>;
}

interface Player {
  network: NetworkAPI;
}

// Usage
class SubtitleDownloader implements Plugin {
  async onLoad(player: Player) {
    player.on('player:load', async (source) => {
      const subtitles = await player.network.download(
        source.subtitleUrl
      );
      player.loadSubtitles(subtitles);
    });
  }
}
```

## UI API

Plugins can extend the player UI:

```typescript
interface UI {
  // Component registration
  registerComponent(component: UIComponent): string;
  unregisterComponent(componentId: string): void;
  
  // Control bar access
  addControlBarItem(item: ControlBarItem): void;
  removeControlBarItem(itemId: string): void;
  
  // Overlay access
  showOverlay(content: string | HTMLElement): void;
  hideOverlay(): void;
  
  // Theme access
  getTheme(): Theme;
  setTheme(theme: Theme): void;
}

interface UIComponent {
  id: string;
  type: 'button' | 'panel' | 'overlay' | 'menu';
  render(): HTMLElement | string;
  position?: 'left' | 'right' | 'top' | 'bottom';
  container?: string;
}

interface ControlBarItem {
  id: string;
  icon: string;
  label: string;
  onClick: () => void;
  order?: number;
}

// Usage
class VolumeBoostPlugin implements Plugin {
  onLoad(player: Player) {
    player.getUI().addControlBarItem({
      id: 'volume-boost',
      icon: 'volume-up',
      label: 'Boost Volume',
      onClick: () => this.boostVolume(),
      order: 5
    });
  }
}
```

## Audio/Video Processing API

Plugins can process audio and video data:

```typescript
interface ProcessingAPI {
  // Audio processing
  createAudioWorklet(processor: AudioWorkletProcessor): void;
  addAudioEffect(effect: AudioEffect): void;
  removeAudioEffect(effectId: string): void;
  
  // Video processing
  createVideoFilter(filter: VideoFilter): void;
  addVideoEffect(effect: VideoEffect): void;
  removeVideoEffect(effectId: string): void;
}

interface AudioEffect {
  id: string;
  type: 'equalizer' | 'compressor' | 'reverb' | 'custom';
  process(audioBuffer: AudioBuffer): AudioBuffer;
}

interface VideoFilter {
  id: string;
  type: 'brightness' | 'contrast' | 'saturation' | 'custom';
  process(videoFrame: VideoFrame): VideoFrame;
}

// Usage
class AudioEnhancerPlugin implements Plugin {
  onLoad(player: Player) {
    player.processing.addAudioEffect({
      id: 'bass-boost',
      type: 'equalizer',
      process: (buffer) => this.boostBass(buffer)
    });
  }
}
```

## Plugin Communication

Plugins can communicate with each other:

```typescript
interface PluginSystem {
  // Get plugin instance
  getPlugin(pluginId: string): Plugin | null;
  
  // Check if plugin is loaded
  hasPlugin(pluginId: string): boolean;
  
  // Send message to plugin
  sendTo(pluginId: string, message: any): void;
  
  // Broadcast message to all plugins
  broadcast(message: any): void;
}

interface Player {
  plugins: PluginSystem;
}

// Usage
class PluginA implements Plugin {
  onLoad(player: Player) {
    // Send message to PluginB
    player.plugins.sendTo('plugin-b', {
      type: 'greeting',
      message: 'Hello from PluginA!'
    });
  }
}

class PluginB implements Plugin {
  onLoad(player: Player) {
    // Listen for messages from other plugins
    player.plugins.on('message', (sender, message) => {
      console.log(`Received from ${sender}:`, message);
    });
  }
}
```

## Error Handling

Plugins should properly handle errors:

```typescript
class RobustPlugin implements Plugin {
  onLoad(player: Player) {
    try {
      this.setupFeature(player);
    } catch (error) {
      console.error('Plugin load error:', error);
      // Optionally report to player
      player.emit('plugin:error', {
        plugin: this.id,
        error: error.message
      });
    }
  }
  
  onUnload() {
    this.cleanup();
  }
}
```

## Performance Considerations

- Use async/await for heavy operations
- Avoid blocking the main thread
- Cache frequently accessed data
- Clean up resources in `onUnload`
- Use Web Workers for CPU-intensive tasks

## Best Practices

1. **Always implement both `onLoad` and `onUnload`**
2. **Clean up event listeners in `onUnload`**
3. **Use unique IDs for components and effects**
4. **Provide meaningful error messages**
5. **Document your plugin's API**
6. **Handle configuration validation**
7. **Test with different media formats**
8. **Consider mobile performance**

## API Versioning

The Plugin API uses semantic versioning. Check compatibility:

```typescript
interface Plugin {
  apiVersion: string;
  minPlayerVersion: string;
}

const plugin = {
  id: 'my-plugin',
  apiVersion: '2.0.0',
  minPlayerVersion: '3.0.0',
  // ...
};
```