---
sidebar_position: 1
---

# Plugin Development Introduction

Vantis Media Player's plugin system allows you to extend and customize the player with new features, integrations, and functionality.

## Why Create Plugins?

Plugins enable you to:

- **Add new features** without modifying core code
- **Integrate external services** (APIs, databases, services)
- **Create custom visualizations** and effects
- **Support new formats** and protocols
- **Build domain-specific solutions** for specific use cases
- **Share functionality** with the community

## Plugin Types

Vantis Media Player supports three types of plugins:

### 1. WebAssembly (WASM) Plugins

**Best for:** Cross-platform plugins, audio/video processing, performance-critical operations

```rust
// WASM Plugin Example
use vantis_plugin_sdk::{Plugin, PluginContext, AudioData};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "My Plugin"
    }

    fn on_audio_data(&mut self, data: &AudioData) {
        // Process audio data
    }
}
```

**Advantages:**
- Cross-platform compatibility
- Sandboxed execution
- Near-native performance
- Safe memory management

### 2. Native Plugins

**Best for:** System-level integration, hardware access, native APIs

```rust
// Native Plugin Example
#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyNativePlugin::new())
}

pub struct MyNativePlugin {
    // Native state
}

impl Plugin for MyNativePlugin {
    fn on_load(&mut self, ctx: &mut PluginContext) {
        // Access native system APIs
    }
}
```

**Advantages:**
- Direct system access
- Maximum performance
- Hardware integration
- Native library integration

### 3. JavaScript Plugins

**Best for:** Web-specific features, UI extensions, quick prototyping

```javascript
// JavaScript Plugin Example
class MyPlugin {
    onLoad(player, context) {
        this.player = player;
        this.context = context;
    }

    onPlay() {
        console.log('Playback started');
    }
}

vantismedia.registerPlugin('my-plugin', MyPlugin);
```

**Advantages:**
- Easy to develop and debug
- No compilation required
- Perfect for web integration
- Quick prototyping

## Plugin Capabilities

Plugins can interact with the player in various ways:

### Audio Processing

Process audio data in real-time:

```javascript
class AudioProcessorPlugin {
    onAudioData(audioData) {
        const { samples, sampleRate, channels } = audioData;
        // Apply effects, filters, analysis
        return processedData;
    }
}
```

### Video Processing

Process video frames:

```javascript
class VideoProcessorPlugin {
    onVideoFrame(videoFrame) {
        const { width, height, data } = videoFrame;
        // Apply filters, effects, overlays
        return processedFrame;
    }
}
```

### UI Extensions

Add custom UI elements:

```javascript
class UIPlugin {
    onLoad(player, context) {
        context.addButton({
            id: 'my-button',
            icon: 'star',
            onClick: () => this.doSomething()
        });

        context.addMenuItem({
            id: 'my-menu',
            label: 'My Feature',
            onClick: () => this.openDialog()
        });
    }
}
```

### Event Handling

Respond to player events:

```javascript
class EventPlugin {
    onLoad(player, context) {
        player.on('playing', () => {
            this.onPlay();
        });

        player.on('timeupdate', (data) => {
            this.onTimeUpdate(data);
        });
    }
}
```

### Subtitle Processing

Process and modify subtitles:

```javascript
class SubtitlePlugin {
    onSubtitleData(subtitleData) {
        const { text, start, end } = subtitleData;
        // Translate, correct, enhance subtitles
        return modifiedSubtitle;
    }
}
```

## Plugin Architecture

### Lifecycle

```
1. Load     → Plugin is loaded into memory
2. Init     → Plugin is initialized
3. Activate → Plugin becomes active
4. Process  → Plugin handles events/data
5. Deactivate → Plugin becomes inactive
6. Unload   → Plugin is removed from memory
```

### Communication

Plugins communicate with the player through:

- **Context API** - Access player functionality
- **Event System** - Send and receive events
- **Hooks** - Intercept and modify data
- **Configuration** - Read/write settings

### Security Model

Plugins run with specific permissions:

```javascript
class SecurePlugin {
    getPermissions() {
        return [
            'audio-processing',
            'video-processing',
            'network-access',
            'filesystem-read',
            'ui-customization'
        ];
    }
}
```

## Getting Started

### Prerequisites

- Basic knowledge of JavaScript, Rust, or both
- Understanding of media playback concepts
- Familiarity with event-driven programming

### Development Environment

For WASM/Native plugins:
- Rust 1.75 or later
- Cargo package manager
- wasm32 target (for WASM)

For JavaScript plugins:
- Node.js 18 or later
- npm or yarn
- Modern browser for testing

### Quick Start

1. **Create a plugin project**
2. **Implement the Plugin trait/interface**
3. **Build the plugin**
4. **Install in Vantis Media Player**
5. **Test and debug**

## Plugin Distribution

### Plugin Registry

Publish your plugin to the Vantis Plugin Registry:

```bash
vantismedia plugin publish my-plugin.json
```

### Plugin Manifest

```json
{
    "name": "my-plugin",
    "version": "1.0.0",
    "description": "My awesome plugin",
    "author": "Your Name",
    "license": "MIT",
    "type": "wasm",
    "permissions": ["audio-processing"],
    "minVersion": "1.0.0",
    "downloadUrl": "https://example.com/plugins/my-plugin.wasm"
}
```

## Best Practices

### 1. Keep Plugins Focused

Each plugin should do one thing well:

```javascript
// Good: Focused purpose
class AudioNormalizerPlugin {
    // Only handles audio normalization
}

// Avoid: Multiple unrelated features
class KitchenSinkPlugin {
    // Audio processing + UI + network + ...
}
```

### 2. Handle Errors Gracefully

```javascript
class SafePlugin {
    onAudioData(data) {
        try {
            return this.process(data);
        } catch (error) {
            this.context.error('Processing failed:', error);
            return data; // Return original data
        }
    }
}
```

### 3. Clean Up Resources

```javascript
class CleanPlugin {
    onLoad(player, context) {
        this.interval = setInterval(() => {}, 1000);
        this.eventHandler = (e) => this.handle(e);
        player.on('event', this.eventHandler);
    }

    onUnload() {
        clearInterval(this.interval);
        this.player.off('event', this.eventHandler);
    }
}
```

### 4. Use Configuration

```javascript
class ConfigurablePlugin {
    getConfigSchema() {
        return {
            threshold: {
                type: 'number',
                default: 0.5,
                min: 0,
                max: 1
            }
        };
    }

    onConfigChange(config) {
        this.threshold = config.threshold;
    }
}
```

### 5. Document Your Plugin

```javascript
/**
 * Audio Normalizer Plugin
 * 
 * Normalizes audio levels for consistent playback.
 * 
 * Configuration:
 * - targetLevel: Target loudness in dB LUFS (default: -16)
 * - peakLimit: Maximum peak level in dB (default: -1)
 * 
 * @example
 * player.setPluginConfig('audio-normalizer', {
 *     targetLevel: -16,
 *     peakLimit: -1
 * });
 */
class AudioNormalizerPlugin {
    // Implementation
}
```

## Community Resources

- **Plugin Examples**: GitHub repository with sample plugins
- **Plugin Registry**: Browse and discover plugins
- **Developer Forum**: Ask questions and share ideas
- **Discord Channel**: Real-time community support

## Next Steps

- **[Getting Started](./getting-started)** - Create your first plugin
- **[Plugin API](./plugin-api)** - Complete API reference
- **[WASM Plugins](./wasm-plugins)** - WebAssembly plugin development
- **[Examples](./examples)** - Real-world plugin examples

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)