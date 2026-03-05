---
sidebar_position: 7
---

# Plugins API

The Plugins API allows you to extend Vantis Media Player with custom functionality.

## Plugin Overview

Plugins can:

- **Process audio/video data** in real-time
- **Add UI elements** and controls
- **Integrate external services**
- **Implement custom protocols**
- **Extend player functionality**

## Plugin Lifecycle

### Plugin Lifecycle Stages

1. **Load** - Plugin is loaded into the player
2. **Initialize** - Plugin initialization
3. **Activate** - Plugin is activated
4. **Process** - Plugin processes data/events
5. **Deactivate** - Plugin is deactivated
6. **Unload** - Plugin is unloaded

### Lifecycle Methods

```javascript
class MyPlugin {
    // Called when plugin is loaded
    onLoad(player, context) {
        this.player = player;
        this.context = context;
    }

    // Called when plugin is initialized
    async onInit() {
        // Setup plugin
    }

    // Called when plugin is activated
    onActivate() {
        // Activate functionality
    }

    // Called when plugin is deactivated
    onDeactivate() {
        // Deactivate functionality
    }

    // Called when plugin is unloaded
    onUnload() {
        // Cleanup resources
    }
}
```

## Installing Plugins

### JavaScript Plugin

```javascript
// Load plugin from file
await player.loadPlugin('my-plugin.js');

// Load plugin from URL
await player.loadPluginFromUrl('https://example.com/plugin.js');

// Load plugin with config
await player.loadPlugin('my-plugin.js', {
    enabled: true,
    config: { option1: 'value1' }
});
```

### WASM Plugin

```javascript
// Load WASM plugin
await player.loadWasmPlugin('plugin.wasm', {
    config: { }
});
```

### Native Plugin

```javascript
// Load native plugin
await player.loadNativePlugin('plugin.so', {
    config: { }
});
```

## Managing Plugins

### List Plugins

```javascript
// Get all plugins
const plugins = player.getPlugins();
// [
//   { id: 'visualizer', name: 'Visualizer', version: '1.0.0', enabled: true },
//   { id: 'audio-normalizer', name: 'Audio Normalizer', version: '2.1.0', enabled: true }
// ]

// Get plugin info
const info = player.getPluginInfo('visualizer');
```

### Enable/Disable Plugins

```javascript
// Enable plugin
await player.enablePlugin('visualizer');

// Disable plugin
await player.disablePlugin('visualizer');

// Check if enabled
const isEnabled = player.isPluginEnabled('visualizer');
```

### Configure Plugins

```javascript
// Set plugin configuration
player.setPluginConfig('visualizer', {
    type: 'bars',
    color: '#25c2a0',
    sensitivity: 1.5
});

// Get plugin configuration
const config = player.getPluginConfig('visualizer');
```

### Remove Plugins

```javascript
// Remove plugin
await player.removePlugin('visualizer');
```

## Plugin Context

The plugin context provides access to player functionality:

```javascript
class MyPlugin {
    onLoad(player, context) {
        this.player = player;
        this.context = context;

        // Logging
        this.context.log('Plugin loaded');
        this.context.debug('Debug message');
        this.context.warn('Warning message');
        this.context.error('Error message');

        // Player control
        this.context.play();
        this.context.pause();
        this.context.seek(60);
        this.context.setVolume(0.8);

        // Get player state
        const position = this.context.getPosition();
        const duration = this.context.getDuration();
        const volume = this.context.getVolume();
        const isPlaying = this.context.isPlaying();

        // Configuration
        const config = this.context.getConfig();
        this.context.setConfig({ key: 'value' });

        // Events
        this.context.emit('custom-event', { data });
        this.context.on('player-event', (data) => { });

        // UI integration
        this.context.addMenuItem({
            id: 'my-menu',
            label: 'My Menu',
            onClick: () => { }
        });

        this.context.addButton({
            id: 'my-button',
            icon: 'star',
            onClick: () => { }
        });

        this.context.showNotification('Message');
    }
}
```

## Plugin Hooks

### Audio Processing Hooks

```javascript
class AudioPlugin {
    onAudioData(audioData) {
        const { samples, sampleRate, channels } = audioData;
        // Process audio data
        return processedData;
    }

    onAudioFrame(frame) {
        // Process audio frame
    }
}
```

### Video Processing Hooks

```javascript
class VideoPlugin {
    onVideoFrame(videoFrame) {
        const { width, height, format, data } = videoFrame;
        // Process video frame
        return processedFrame;
    }
}
```

### Subtitle Hooks

```javascript
class SubtitlePlugin {
    onSubtitleData(subtitleData) {
        const { text, start, end } = subtitleData;
        // Process subtitle data
    }
}
```

### Event Hooks

```javascript
class EventPlugin {
    onLoad() {
        console.log('Player loaded');
    }

    onPlay() {
        console.log('Playback started');
    }

    onPause() {
        console.log('Playback paused');
    }

    onStop() {
        console.log('Playback stopped');
    }

    onSeek(position) {
        console.log('Seeked to:', position);
    }

    onVolumeChange(volume) {
        console.log('Volume changed:', volume);
    }

    onQualityChange(quality) {
        console.log('Quality changed:', quality);
    }

    onSpeedChange(speed) {
        console.log('Speed changed:', speed);
    }

    onAudioTrackChange(track) {
        console.log('Audio track changed:', track);
    }

    onSubtitleTrackChange(track) {
        console.log('Subtitle track changed:', track);
    }
}
```

## Plugin Communication

### Plugin-to-Plugin Communication

```javascript
// Plugin A emits event
class PluginA {
    onLoad(player, context) {
        this.context.emit('custom-event', { data: 'Hello' });
    }
}

// Plugin B listens to event
class PluginB {
    onLoad(player, context) {
        context.on('custom-event', (data) => {
            console.log('Received:', data);
        });
    }
}
```

### Plugin-to-Player Communication

```javascript
class Plugin {
    onLoad(player, context) {
        // Call player methods
        player.play();
        player.pause();
        player.seek(60);
        player.setVolume(0.8);

        // Listen to player events
        player.on('playing', () => {
            console.log('Player is playing');
        });
    }
}
```

## Plugin UI Integration

### Adding Menu Items

```javascript
class UIPlugin {
    onLoad(player, context) {
        context.addMenuItem({
            id: 'my-menu-item',
            label: 'My Action',
            onClick: () => {
                console.log('Menu item clicked');
            }
        });
    }
}
```

### Adding Buttons

```javascript
class ButtonPlugin {
    onLoad(player, context) {
        context.addButton({
            id: 'my-button',
            icon: 'star',
            label: 'Star',
            position: 'right',
            onClick: () => {
                console.log('Button clicked');
            }
        });
    }
}
```

### Adding Custom UI

```javascript
class CustomUIPlugin {
    async onInit() {
        // Create custom UI element
        const element = document.createElement('div');
        element.id = 'my-plugin-ui';
        element.innerHTML = '<button>Click Me</button>';
        element.querySelector('button').addEventListener('click', () => {
            this.handleButtonClick();
        });

        // Add to player container
        this.player.getContainer().appendChild(element);
    }

    handleButtonClick() {
        console.log('Custom button clicked');
    }

    onUnload() {
        // Remove UI element
        const element = document.getElementById('my-plugin-ui');
        if (element) {
            element.remove();
        }
    }
}
```

## Plugin Configuration

### Configuration Schema

```javascript
class MyPlugin {
    // Define configuration schema
    getConfigSchema() {
        return {
            enabled: {
                type: 'boolean',
                default: true,
                description: 'Enable plugin'
            },
            option1: {
                type: 'string',
                default: 'default-value',
                description: 'Option 1 description'
            },
            option2: {
                type: 'number',
                default: 10,
                min: 0,
                max: 100,
                description: 'Option 2 description'
            }
        };
    }

    onConfigChange(config) {
        // Handle configuration changes
        console.log('Config changed:', config);
    }
}
```

### Validating Configuration

```javascript
class ConfiguredPlugin {
    onConfigChange(config) {
        const schema = this.getConfigSchema();
        
        // Validate configuration
        const errors = this.validateConfig(config, schema);
        if (errors.length > 0) {
            this.context.error('Invalid configuration:', errors);
            return;
        }

        // Apply configuration
        this.applyConfig(config);
    }

    validateConfig(config, schema) {
        const errors = [];
        
        for (const [key, field] of Object.entries(schema)) {
            const value = config[key];
            
            if (value === undefined) {
                if (field.default !== undefined) {
                    config[key] = field.default;
                } else {
                    errors.push(`Missing required field: ${key}`);
                }
            } else {
                // Type validation
                if (field.type &amp;&amp; typeof value !== field.type) {
                    errors.push(`Invalid type for ${key}: expected ${field.type}`);
                }
                
                // Range validation
                if (field.min !== undefined &amp;&amp; value < field.min) {
                    errors.push(`${key} must be >= ${field.min}`);
                }
                if (field.max !== undefined &amp;&amp; value > field.max) {
                    errors.push(`${key} must be <= ${field.max}`);
                }
            }
        }
        
        return errors;
    }
}
```

## Plugin Storage

### Persistent Storage

```javascript
class StoragePlugin {
    onLoad(player, context) {
        // Save data
        this.context.storage.set('key', 'value');
        this.context.storage.set('data', { complex: 'object' });

        // Load data
        const value = this.context.storage.get('key');
        const data = this.context.storage.get('data');

        // Remove data
        this.context.storage.remove('key');

        // Clear all data
        this.context.storage.clear();
    }
}
```

## Plugin Permissions

### Requesting Permissions

```javascript
class PermissionPlugin {
    getPermissions() {
        return [
            'network-access',      // Network access
            'filesystem-read',     // File system read
            'filesystem-write',    // File system write
            'audio-processing',    // Audio processing
            'video-processing',    // Video processing
            'ui-customization',    // UI customization
            'api-access',          // API access
            'storage'              // Storage access
        ];
    }
}
```

### Checking Permissions

```javascript
class PermissionPlugin {
    onLoad(player, context) {
        // Check if permission is granted
        const hasNetworkAccess = context.hasPermission('network-access');
        const hasFileSystemWrite = context.hasPermission('filesystem-write');

        if (hasNetworkAccess) {
            // Make network request
        }

        if (hasFileSystemWrite) {
            // Write to file system
        }
    }
}
```

## Plugin Debugging

### Debug Mode

```javascript
class DebugPlugin {
    onLoad(player, context) {
        // Enable debug logging
        context.setDebugMode(true);

        // Log debug information
        context.debug('Debug information');

        // Performance profiling
        const start = performance.now();
        // ... some operation
        const duration = performance.now() - start;
        context.debug(`Operation took ${duration}ms`);
    }
}
```

### Error Handling

```javascript
class ErrorHandlingPlugin {
    onAudioData(audioData) {
        try {
            // Process audio data
            return this.process(audioData);
        } catch (error) {
            this.context.error('Audio processing error:', error);
            // Return original data
            return audioData;
        }
    }
}
```

## Plugin Examples

### Simple Audio Visualizer

```javascript
class VisualizerPlugin {
    onLoad(player, context) {
        this.context = context;
        this.canvas = null;
        this.ctx = null;
    }

    async onInit() {
        // Create canvas
        this.canvas = document.createElement('canvas');
        this.canvas.style.position = 'absolute';
        this.canvas.style.bottom = '100px';
        this.canvas.style.left = '20px';
        this.canvas.style.width = '300px';
        this.canvas.style.height = '50px';
        this.player.getContainer().appendChild(this.canvas);
        this.ctx = this.canvas.getContext('2d');
    }

    onAudioData(audioData) {
        const { frequency } = audioData;
        
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw bars
        const barWidth = this.canvas.width / frequency.length;
        for (let i = 0; i < frequency.length; i++) {
            const height = frequency[i] / 255 * this.canvas.height;
            this.ctx.fillStyle = '#25c2a0';
            this.ctx.fillRect(i * barWidth, this.canvas.height - height, barWidth, height);
        }
    }

    onUnload() {
        if (this.canvas) {
            this.canvas.remove();
        }
    }
}

// Register plugin
vantismedia.registerPlugin('visualizer', VisualizerPlugin);
```

### Custom Subtitle Downloader

```javascript
class SubtitleDownloaderPlugin {
    onLoad(player, context) {
        this.context = context;
    }

    async onInit() {
        // Add menu item
        this.context.addMenuItem({
            id: 'download-subtitles',
            label: 'Download Subtitles',
            onClick: () => this.downloadSubtitles()
        });
    }

    async downloadSubtitles() {
        const mediaUrl = this.player.getMediaUrl();
        const response = await fetch(`https://subs.example.com/api?url=${encodeURIComponent(mediaUrl)}`);
        const subtitles = await response.json();

        if (subtitles.length > 0) {
            const sub = subtitles[0];
            await this.player.loadSubtitles(sub.url);
            this.context.showNotification('Subtitles downloaded');
        } else {
            this.context.showNotification('No subtitles found');
        }
    }
}

// Register plugin
vantismedia.registerPlugin('subtitle-downloader', SubtitleDownloaderPlugin);
```

## Next Steps

- **[Plugin Development Guide](../plugins/)** - Detailed plugin development
- **[Examples](../examples/)** - Plugin examples
- **[Advanced Features](../advanced-features/plugins)** - Plugin system overview

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)