---
sidebar_position: 2
---

# Components

This document provides a detailed overview of the core components in Vantis Media Player, their responsibilities, and how they interact with each other.

## Component Hierarchy

```
Player
├── MediaLoader
├── PlaybackEngine
├── VideoEngine
│   ├── VideoDecoder
│   ├── VideoRenderer
│   └── VideoFilterSystem
├── AudioEngine
│   ├── AudioDecoder
│   ├── AudioProcessor
│   └── AudioDeviceManager
├── SubtitleEngine
│   ├── SubtitleLoader
│   └── SubtitleRenderer
├── UIManager
├── PluginManager
└── EventSystem
```

## Player Component

The `Player` is the main entry point and orchestrates all other components.

### Interface

```typescript
class Player {
  // Configuration
  private config: PlayerConfig;
  private state: PlayerState;
  
  // Components
  private mediaLoader: MediaLoader;
  private playbackEngine: PlaybackEngine;
  private videoEngine: VideoEngine;
  private audioEngine: AudioEngine;
  private subtitleEngine: SubtitleEngine;
  private uiManager: UIManager;
  private pluginManager: PluginManager;
  private eventSystem: EventSystem;
  
  // Public API
  constructor(container: string | HTMLElement, config?: PlayerConfig);
  async load(source: MediaSource): Promise<void>;
  play(): Promise<void>;
  pause(): Promise<void>;
  seek(time: number): Promise<void>;
  stop(): Promise<void>;
  
  // Getters
  getState(): PlayerState;
  getCurrentTime(): number;
  getDuration(): number;
  getVolume(): number;
  
  // Events
  on(event: string, callback: Function): void;
  off(event: string, callback: Function): void;
  emit(event: string, data?: any): void;
  
  // Lifecycle
  destroy(): void;
}
```

### Responsibilities

1. **Initialization**: Set up all sub-components
2. **API Surface**: Provide public API for application code
3. **Event Coordination**: Coordinate event emission and handling
4. **State Management**: Maintain overall player state
5. **Plugin Orchestration**: Load and manage plugins

## MediaLoader Component

The `MediaLoader` handles loading media from various sources.

### Interface

```typescript
class MediaLoader {
  private player: Player;
  private mediaSource: MediaSource | null;
  
  // Detection
  async detectFormat(source: MediaSource): Promise<MediaFormat>;
  async detectType(source: MediaSource): Promise<MediaType>;
  
  // Loading
  async load(source: MediaSource): Promise<void>;
  async unload(): Promise<void>;
  
  // Validation
  validate(source: MediaSource): boolean;
  canPlay(source: MediaSource): boolean;
  
  // Manifest parsing
  async parseHLSManifest(url: string): Promise<HLSManifest>;
  async parseDASHManifest(url: string): Promise<DASHManifest>;
}
```

### Supported Media Types

| Type | Formats | Description |
|------|---------|-------------|
| Video | MP4, WebM, OGG, AVI, MKV | Progressive download |
| Audio | MP3, AAC, OGG, FLAC, WAV | Progressive download |
| HLS | M3U8 | Adaptive streaming |
| DASH | MPD | Adaptive streaming |
| RTMP | RTMP | Live streaming |
| WebRTC | WebRTC | Real-time communication |

## PlaybackEngine Component

The `PlaybackEngine` manages the playback lifecycle and state transitions.

### Interface

```typescript
class PlaybackEngine {
  private player: Player;
  private playbackState: PlaybackState;
  
  // Control
  async play(): Promise<void>;
  async pause(): Promise<void>;
  async seek(time: number): Promise<void>;
  async stop(): Promise<void>;
  
  // State
  getState(): PlaybackState;
  isPlaying(): boolean;
  isPaused(): boolean;
  isEnded(): boolean;
  
  // Time management
  getCurrentTime(): number;
  getDuration(): number;
  
  // Rate
  setPlaybackRate(rate: number): void;
  getPlaybackRate(): number;
  
  // Loops
  setLoop(loop: boolean): void;
  getLoop(): boolean;
}
```

### State Machine

```
    ┌─────────┐
    │  IDLE   │
    └────┬────┘
         │ load()
         ▼
    ┌─────────┐
    │ LOADING │
    └────┬────┘
         │ ready
         ▼
    ┌─────────┐      play()      ┌──────────┐
    │  READY  │ ───────────────▶ │ PLAYING  │
    └─────────┘                  └────┬─────┘
         ▲                             │
         │                    pause()  │
         │                             ▼
         │                    ┌──────────┐
         └────────────────────│ PAUSED   │
                  seek()       └──────────┘
```

## VideoEngine Component

The `VideoEngine` handles video decoding and rendering.

### Interface

```typescript
class VideoEngine {
  private player: Player;
  private decoder: VideoDecoder;
  private renderer: VideoRenderer;
  private filterSystem: VideoFilterSystem;
  
  // Initialization
  async initialize(source: MediaSource): Promise<void>;
  async destroy(): Promise<void>;
  
  // Decoding
  async decodeFrame(): Promise<VideoFrame | null>;
  
  // Rendering
  renderFrame(frame: VideoFrame): void;
  
  // Quality
  getQualities(): VideoQuality[];
  setQuality(qualityId: string): void;
  getQuality(): VideoQuality;
  
  // Filters
  addFilter(filter: VideoFilter): void;
  removeFilter(filterId: string): void;
  
  // Screenshot
  takeScreenshot(): Promise<Blob>;
  
  // PiP
  togglePictureInPicture(): Promise<void>;
}
```

### VideoDecoder

```typescript
class VideoDecoder {
  private codec: VideoCodec;
  private hardwareAcceleration: boolean;
  
  async initialize(config: DecoderConfig): Promise<void>;
  async decode(data: Uint8Array): Promise<VideoFrame>;
  async flush(): Promise<void>;
  reset(): void;
  
  // Codec support
  static isCodecSupported(codec: VideoCodec): boolean;
  static getHardwareDecoderType(): HardwareDecoderType;
}
```

### VideoRenderer

```typescript
class VideoRenderer {
  private canvas: HTMLCanvasElement;
  private context: CanvasRenderingContext2D | WebGLRenderingContext;
  
  render(frame: VideoFrame): void;
  resize(width: number, height: number): void;
  
  // Display modes
  setDisplayMode(mode: DisplayMode): void;
  setFillMode(mode: FillMode): void; // contain, cover, fill
}
```

## AudioEngine Component

The `AudioEngine` handles audio processing and output.

### Interface

```typescript
class AudioEngine {
  private player: Player;
  private decoder: AudioDecoder;
  private processor: AudioProcessor;
  private deviceManager: AudioDeviceManager;
  
  // Initialization
  async initialize(source: MediaSource): Promise<void>;
  async destroy(): Promise<void>;
  
  // Volume
  setVolume(volume: number): void;
  getVolume(): number;
  setMuted(muted: boolean): void;
  getMuted(): boolean;
  
  // Tracks
  getAudioTracks(): AudioTrack[];
  setAudioTrack(trackId: string): void;
  
  // Effects
  addEffect(effect: AudioEffect): void;
  removeEffect(effectId: string): void;
  
  // Equalizer
  getEqualizer(): Equalizer;
}
```

### AudioDecoder

```typescript
class AudioDecoder {
  private codec: AudioCodec;
  private sampleRate: number;
  private channels: number;
  
  async initialize(config: DecoderConfig): Promise<void>;
  async decode(data: Uint8Array): Promise<AudioBuffer>;
  
  // Codec support
  static isCodecSupported(codec: AudioCodec): boolean;
}
```

### AudioProcessor

```typescript
class AudioProcessor {
  private audioContext: AudioContext;
  private effects: Map<string, AudioNode>;
  
  process(buffer: AudioBuffer): AudioBuffer;
  
  // Built-in effects
  addEqualizer(bands: EqualizerBand[]): void;
  addCompressor(config: CompressorConfig): void;
  addReverb(config: ReverbConfig): void;
  
  // Custom effects
  addCustomEffect(processor: AudioWorkletProcessor): void;
}
```

### AudioDeviceManager

```typescript
class AudioDeviceManager {
  async enumerateDevices(): Promise<MediaDeviceInfo[]>;
  async setDevice(deviceId: string): Promise<void>;
  getDevice(): MediaDeviceInfo;
  
  // Permissions
  async requestPermission(): Promise<boolean>;
  
  // Events
  onDeviceChange(callback: (devices: MediaDeviceInfo[]) => void): void;
}
```

## SubtitleEngine Component

The `SubtitleEngine` manages subtitle loading and rendering.

### Interface

```typescript
class SubtitleEngine {
  private player: Player;
  private loader: SubtitleLoader;
  private renderer: SubtitleRenderer;
  
  // Loading
  async load(source: SubtitleSource): Promise<void>;
  async unload(): Promise<void>;
  
  // Tracks
  getSubtitleTracks(): SubtitleTrack[];
  setSubtitleTrack(trackId: string): void;
  disableSubtitles(): void;
  
  // Styling
  setStyle(style: SubtitleStyle): void;
  getStyle(): SubtitleStyle;
  
  // Sync
  setOffset(offset: number): void;
  getOffset(): number;
}
```

### SubtitleLoader

```typescript
class SubtitleLoader {
  async load(url: string): Promise<SubtitleData>;
  async parse(content: string, format: SubtitleFormat): Promise<SubtitleData>;
  
  // Format detection
  detectFormat(content: string): SubtitleFormat;
  
  // Supported formats
  static supportedFormats: SubtitleFormat[] = [
    'srt', 'vtt', 'ass', 'ssa', 'sub'
  ];
}
```

### SubtitleRenderer

```typescript
class SubtitleRenderer {
  private container: HTMLElement;
  private currentStyle: SubtitleStyle;
  
  render(cue: SubtitleCue): void;
  clear(): void;
  
  setStyle(style: SubtitleStyle): void;
  show(): void;
  hide(): void;
}
```

## UIManager Component

The `UIManager` handles all user interface elements.

### Interface

```typescript
class UIManager {
  private player: Player;
  private container: HTMLElement;
  private components: Map<string, UIComponent>;
  
  // Controls
  createControls(): ControlBar;
  createProgressBar(): ProgressBar;
  createVolumeControl(): VolumeControl;
  
  // Overlays
  showOverlay(content: string | HTMLElement): void;
  hideOverlay(): void;
  
  // Custom components
  registerComponent(component: UIComponent): void;
  unregisterComponent(componentId: string): void;
  
  // Theming
  setTheme(theme: Theme): void;
  getTheme(): Theme;
}
```

## PluginManager Component

The `PluginManager` manages plugin lifecycle and communication.

### Interface

```typescript
class PluginManager {
  private player: Player;
  private plugins: Map<string, Plugin>;
  private pluginAPI: PluginAPI;
  
  // Registration
  async register(plugin: Plugin): Promise<void>;
  async unregister(pluginId: string): Promise<void>;
  
  // Plugin access
  getPlugin(pluginId: string): Plugin | null;
  getPlugins(): Plugin[];
  
  // Communication
  sendTo(pluginId: string, message: any): void;
  broadcast(message: any): void;
  
  // Lifecycle
  async loadPlugins(config: PluginConfig[]): Promise<void>;
  async unloadAll(): Promise<void>;
}
```

## EventSystem Component

The `EventSystem` manages event emission and handling.

### Interface

```typescript
class EventSystem {
  private listeners: Map<string, Set<Function>>;
  
  // Subscription
  on(event: string, callback: Function): void;
  once(event: string, callback: Function): void;
  off(event: string, callback: Function): void;
  
  // Emission
  emit(event: string, data?: any): void;
  
  // Cleanup
  clear(): void;
  clearEvent(event: string): void;
}
```

### Event Types

```typescript
interface PlayerEvents {
  // Lifecycle
  'player:ready': () => void;
  'player:load': (source: MediaSource) => void;
  'player:play': () => void;
  'player:pause': () => void;
  'player:ended': () => void;
  'player:error': (error: PlayerError) => void;
  'player:destroy': () => void;
  
  // Playback
  'player:seek': (time: number) => void;
  'player:timeupdate': (time: number) => void;
  'player:ratechange': (rate: number) => void;
  
  // Volume
  'player:volumechange': (volume: number) => void;
  'player:mutechange': (muted: boolean) => void;
  
  // Video
  'video:ready': () => void;
  'video:qualitychange': (quality: VideoQuality) => void;
  
  // Audio
  'audio:ready': () => void;
  'audio:trackchange': (track: AudioTrack) => void;
  
  // Subtitles
  'subtitle:ready': () => void;
  'subtitle:trackchange': (track: SubtitleTrack) => void;
}
```

## Component Interaction

### Initialization Sequence

```
1. Player.constructor()
   ↓
2. Player.initializeComponents()
   ├── MediaLoader.initialize()
   ├── PlaybackEngine.initialize()
   ├── VideoEngine.initialize()
   ├── AudioEngine.initialize()
   ├── SubtitleEngine.initialize()
   ├── UIManager.initialize()
   ├── PluginManager.initialize()
   └── EventSystem.initialize()
   ↓
3. Player.emit('player:ready')
```

### Load Sequence

```
Player.load(source)
   ↓
MediaLoader.validate(source)
   ↓
MediaLoader.detectFormat(source)
   ↓
[Format-specific loading]
   ↓
VideoEngine.initialize(source)
   ↓
AudioEngine.initialize(source)
   ↓
SubtitleEngine.load(source)
   ↓
Player.emit('player:load')
```

### Play Sequence

```
Player.play()
   ↓
PlaybackEngine.play()
   ↓
VideoEngine.startDecoding()
   ↓
AudioEngine.startDecoding()
   ↓
requestAnimationFrame(renderLoop)
   ↓
Player.emit('player:play')
```

## Best Practices

1. **Component Isolation**: Each component should be self-contained
2. **Event-Driven**: Use events for cross-component communication
3. **Async/Await**: Use async/await for async operations
4. **Error Handling**: Proper error handling in all components
5. **Resource Cleanup**: Clean up resources in destroy methods
6. **Type Safety**: Use TypeScript for type safety

## Related Documentation

- [Overview](./overview) - High-level architecture
- [Event System](./event-system) - Event system details
- [State Management](./state-management) - State management
- [Plugin System](./plugin-system) - Plugin architecture