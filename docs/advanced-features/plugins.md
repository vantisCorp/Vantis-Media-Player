---
sidebar_position: 3
---

# Plugins

Vantis Media Player features a powerful plugin system that allows you to extend functionality with custom features, visualizations, and integrations.

## Plugin Architecture

### Plugin Types

Vantis Media Player supports multiple plugin types:

| Type | Description | Use Case |
|------|-------------|----------|
| **WASM** | WebAssembly plugins | Cross-platform, sandboxed plugins |
| **Native** | System-level plugins | Hardware integration, native APIs |
| **JavaScript** | Web plugins | UI extensions, web integrations |

### Plugin Capabilities

Plugins can:

- **Process audio/video data** in real-time
- **Add custom UI elements** and controls
- **Implement visualizations** and effects
- **Integrate external services** and APIs
- **Handle custom protocols** and formats
- **Extend player functionality** with new features

## Installing Plugins

### From Plugin Registry

```bash
# List available plugins
vantismedia plugin list --available

# Install plugin
vantismedia plugin install visualizer
vantismedia plugin install audio-normalizer
vantismedia plugin install subtitle-downloader
```

### From File

```bash
# Install WASM plugin
vantismedia plugin install /path/to/plugin.wasm

# Install native plugin
vantismedia plugin install /path/to/plugin.so
```

### From URL

```bash
# Install from URL
vantismedia plugin install https://example.com/plugins/my-plugin.wasm
```

### Using API

```javascript
// Install plugin
await player.installPlugin('visualizer');

// Install from URL
await player.installPluginFromUrl('https://example.com/plugin.wasm');
```

## Managing Plugins

### List Installed Plugins

```bash
# List all installed plugins
vantismedia plugin list --installed

# Get plugin details
vantismedia plugin info visualizer
```

```javascript
// Get installed plugins
const plugins = player.getInstalledPlugins();
console.log(plugins);
// [
//   { name: 'visualizer', version: '1.0.0', enabled: true },
//   { name: 'audio-normalizer', version: '2.1.0', enabled: true }
// ]
```

### Enable/Disable Plugins

```bash
# Enable plugin
vantismedia plugin enable visualizer

# Disable plugin
vantismedia plugin disable visualizer
```

```javascript
// Enable plugin
player.enablePlugin('visualizer');

// Disable plugin
player.disablePlugin('visualizer');

// Check if plugin is enabled
const isEnabled = player.isPluginEnabled('visualizer');
```

### Update Plugins

```bash
# Update specific plugin
vantismedia plugin update visualizer

# Update all plugins
vantismedia plugin update --all
```

### Remove Plugins

```bash
# Remove plugin
vantismedia plugin remove visualizer
```

```javascript
// Remove plugin
await player.removePlugin('visualizer');
```

## Plugin Configuration

### Configuration File

Each plugin can have its own configuration:

```toml
[plugins.visualizer]
enabled = true
type = "bars"
color = "#25c2a0"
sensitivity = 1.5

[plugins.audio-normalizer]
enabled = true
target_level = -16  # dB LUFS
peak_limit = -1.0   # dB
```

### Runtime Configuration

```javascript
// Configure plugin
player.configurePlugin('visualizer', {
    type: 'bars',
    color: '#25c2a0',
    sensitivity: 1.5
});

// Get plugin configuration
const config = player.getPluginConfig('visualizer');
```

## Built-in Plugins

### Audio Visualizer

Visualizes audio frequencies in real-time:

```javascript
// Enable visualizer
player.enablePlugin('visualizer');
player.configurePlugin('visualizer', {
    type: 'bars',  // bars, wave, spectrum, circular
    color: '#25c2a0',
    backgroundColor: 'transparent',
    fps: 60
});
```

### Audio Normalizer

Normalizes audio volume across different sources:

```javascript
player.enablePlugin('audio-normalizer');
player.configurePlugin('audio-normalizer', {
    targetLevel: -16,  // dB LUFS
    peakLimit: -1.0    // dB
});
```

### Subtitle Downloader

Automatically downloads subtitles from online sources:

```javascript
player.enablePlugin('subtitle-downloader');
player.configurePlugin('subtitle-downloader', {
    sources: ['opensubtitles', 'subscene'],
    autoDownload: true,
    languages: ['en', 'pl', 'de']
});
```

### Equalizer Presets

Provides preset equalizer configurations:

```javascript
player.enablePlugin('equalizer-presets');
player.applyPreset('rock');  // flat, rock, pop, jazz, classical, etc.
```

### Media Keys

Enables system media key support:

```javascript
player.enablePlugin('media-keys');
```

### Last.fm Scrobbler

Scrobbles tracks to Last.fm:

```javascript
player.enablePlugin('lastfm-scrobbler');
player.configurePlugin('lastfm-scrobbler', {
    apiKey: 'your-api-key',
    apiSecret: 'your-api-secret',
    username: 'your-username',
    password: 'your-password'
});
```

## Creating Plugins

### WebAssembly Plugin

Create a new WASM plugin:

```rust
use vantis_plugin_sdk::{Plugin, PluginContext, AudioData, VideoFrame};

pub struct MyPlugin {
    name: String,
    enabled: bool,
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.log("MyPlugin loaded!");
    }

    fn on_unload(&mut self) {
        // Cleanup
    }

    fn on_audio_data(&mut self, data: &AudioData) {
        // Process audio data
    }

    fn on_video_frame(&mut self, frame: &VideoFrame) {
        // Process video frame
    }

    fn on_config_change(&mut self, config: &serde_json::Value) {
        // Handle configuration changes
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin {
        name: "My Plugin".to_string(),
        enabled: true,
    })
}
```

Build for WASM:

```bash
cargo build --target wasm32-unknown-unknown --release
```

### JavaScript Plugin

Create a JavaScript plugin for web:

```javascript
class MyPlugin {
    constructor() {
        this.name = 'My Plugin';
        this.version = '1.0.0';
    }

    onLoad(player, context) {
        this.player = player;
        this.context = context;
        console.log('My Plugin loaded!');
    }

    onUnload() {
        // Cleanup
    }

    onAudioData(data) {
        // Process audio data
    }

    onVideoFrame(frame) {
        // Process video frame
    }

    onConfigChange(config) {
        // Handle configuration changes
    }
}

// Register plugin
vantismedia.registerPlugin('my-plugin', MyPlugin);
```

### Native Plugin

Create a native plugin for system-level integration:

```rust
use vantis_native_plugin::{Plugin, PluginContext};

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyNativePlugin::new())
}

pub struct MyNativePlugin {
    // Plugin state
}

impl Plugin for MyNativePlugin {
    fn name(&self) -> &str {
        "My Native Plugin"
    }

    fn on_load(&mut self, ctx: &mut PluginContext) {
        // Access system APIs
    }
}
```

Build as shared library:

```bash
cargo build --release
# Output: libmy_plugin.so (Linux), my_plugin.dll (Windows), libmy_plugin.dylib (macOS)
```

## Plugin API

### Lifecycle Events

```rust
trait Plugin {
    // Called when plugin is loaded
    fn on_load(&mut self, ctx: &mut PluginContext);

    // Called when plugin is unloaded
    fn on_unload(&mut self);

    // Called when configuration changes
    fn on_config_change(&mut self, config: &Value);

    // Called on each audio frame
    fn on_audio_data(&mut self, data: &AudioData);

    // Called on each video frame
    fn on_video_frame(&mut self, frame: &VideoFrame);

    // Called when playback starts
    fn on_play(&mut self);

    // Called when playback pauses
    fn on_pause(&mut self);

    // Called when playback stops
    fn on_stop(&mut self);

    // Called when volume changes
    fn on_volume_change(&mut self, volume: f32);

    // Called when seeking
    fn on_seek(&mut self, position: Duration);

    // Called when a new file is loaded
    fn on_load_media(&mut self, url: &str);
}
```

### Plugin Context

The plugin context provides access to player functionality:

```rust
impl PluginContext {
    // Logging
    fn log(&self, message: &str);
    fn debug(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);

    // Player control
    fn play(&self);
    fn pause(&self);
    fn stop(&self);
    fn seek(&self, position: Duration);
    fn set_volume(&self, volume: f32);

    // Get player state
    fn get_position(&self) -> Duration;
    fn get_duration(&self) -> Duration;
    fn get_volume(&self) -> f32;
    fn is_playing(&self) -> bool;

    // Configuration
    fn get_config(&self) -> Value;
    fn set_config(&self, config: Value);

    // UI integration
    fn add_menu_item(&self, item: MenuItem);
    fn add_button(&self, button: Button);
    fn show_notification(&self, message: &str);

    // Events
    fn emit(&self, event: &str, data: Value);
    fn on(&self, event: &str, callback: Callback);
}
```

## Plugin Security

### Sandboxing

WASM plugins run in a sandboxed environment:

- **Memory isolation** - Cannot access player memory directly
- **API restrictions** - Can only use exposed APIs
- **Resource limits** - CPU and memory limits enforced
- **Network restrictions** - Can only access allowed hosts

### Permissions

Plugins can request permissions:

```rust
fn permissions(&self) -> Vec<Permission> {
    vec![
        Permission::NetworkAccess,
        Permission::FileSystemRead,
        Permission::FileSystemWrite,
        Permission::AudioProcessing,
        Permission::VideoProcessing,
    ]
}
```

### Configuration

```toml
[plugins.security]
allow_network = true
allowed_hosts = ["api.example.com"]
allow_file_system = false
max_memory_mb = 128
max_cpu_percent = 50
```

## Plugin Marketplace

### Publishing Plugins

1. Create a plugin manifest:

```json
{
    "name": "my-plugin",
    "version": "1.0.0",
    "description": "My awesome plugin",
    "author": "Your Name",
    "license": "MIT",
    "minVersion": "1.0.0",
    "maxVersion": "2.0.0",
    "permissions": ["audio-processing"],
    "downloadUrl": "https://example.com/plugins/my-plugin.wasm",
    "checksum": "sha256:..."
}
```

2. Submit to the plugin registry:

```bash
vantismedia plugin publish my-plugin.json
```

### Discovering Plugins

```bash
# Search plugins
vantismedia plugin search visualizer

# Get plugin info
vantismedia plugin info my-plugin

# View plugin ratings
vantismedia plugin ratings my-plugin
```

## Troubleshooting

### Plugin Won't Load

```bash
# Check plugin compatibility
vantismedia plugin check my-plugin.wasm

# Verbose plugin loading
vantismedia --verbose --plugin-debug
```

### Plugin Crashes

```bash
# Check plugin logs
vantismedia plugin logs my-plugin

# Disable and test
vantismedia plugin disable my-plugin
```

### Performance Issues

```bash
# Check plugin resource usage
vantismedia plugin stats my-plugin
```

## Next Steps

- **[Plugin Development Guide](../plugins/)** - Detailed plugin development
- **[API Reference](../api/plugins)** - Plugin API documentation
- **[Examples](../examples/)** - Plugin examples

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)