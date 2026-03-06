# Example Plugins

This directory contains example plugins demonstrating the Vantis Media Player plugin API.

## Plugin Types

### 1. Example Audio Decoder (`decoder.json`)
Demonstrates an audio decoder plugin with:
- Support for MP3, WAV, FLAC, AAC formats
- Configurable sample rate (8kHz - 192kHz)
- Configurable channels (1-8)
- Bit depth options (8, 16, 24, 32 bits)

### 2. Example Video Filter (`filter.json`)
Demonstrates a video filter plugin with:
- Brightness adjustment (-100 to 100)
- Contrast adjustment (0.0 to 2.0)
- Saturation adjustment (0.0 to 2.0)
- Support for RGB, YUV, RGBA formats

### 3. Example Demuxer (`demuxer.json`)
Demonstrates a demuxer plugin with:
- Support for MP4, MKV, AVI, WebM containers
- Configurable buffer size (1KB - 64KB)
- Prefetch buffering option
- Maximum stream limit (1-32)

## Plugin Manifest Format

All plugins must include a manifest file (`plugin.json` or `plugin.toml`) with the following structure:

```json
{
  "id": "unique_plugin_id",
  "name": "Plugin Name",
  "version": "1.0.0",
  "api_version": 1,
  "description": "Plugin description",
  "author": "Author Name",
  "license": "MIT",
  "capabilities": ["capability_name"],
  "dependencies": [],
  "formats": ["format1", "format2"],
  "config_options": {}
}
```

## Plugin Capabilities

The following capabilities are supported:

### Decoders
- `audio_decoder` - Audio decoding
- `video_decoder` - Video decoding
- `subtitle_decoder` - Subtitle decoding

### Encoders
- `audio_encoder` - Audio encoding
- `video_encoder` - Video encoding
- `subtitle_encoder` - Subtitle encoding

### Filters
- `audio_filter` - Audio processing
- `video_filter` - Video processing
- `subtitle_filter` - Subtitle processing

### Input/Output
- `demuxer` - Container demuxing
- `muxer` - Container muxing

### Network
- `network_protocol` - Network protocol support

### UI
- `ui_component` - UI components
- `theme` - Theme support
- `visualizer` - Audio visualizers

### Extensions
- `extension` - General extensions
- `script` - Script support
- `ai_model` - AI models
- `hardware_acceleration` - Hardware acceleration

## Creating Your Own Plugin

1. Create a directory for your plugin
2. Create a `plugin.json` manifest file
3. Implement the `Plugin` trait in Rust (or use WASM/other languages)
4. Place the plugin binary/library in the directory
5. Add the plugin directory to the registry

## Example Usage

```rust
use vantis_player::plugins::{PluginRegistry, PluginCapability};

// Create registry
let mut registry = PluginRegistry::new();

// Add plugin directory
registry.add_plugin_dir("plugins");

// Enable capabilities
registry.enable_capability(PluginCapability::AudioDecoder);

// Discover plugins
let plugins = registry.discover_plugins()?;
println!("Found {} plugins", plugins.len());

// Load plugin
let plugin = registry.load_plugin("example_audio_decoder")?;
println!("Loaded: {}", plugin.metadata.name);
```

## Best Practices

1. **Unique IDs**: Use reverse domain notation (e.g., `com.example.plugin`)
2. **Versioning**: Follow semantic versioning (MAJOR.MINOR.PATCH)
3. **Dependencies**: Minimize dependencies to improve portability
4. **Error Handling**: Return meaningful error messages
5. **Performance**: Implement efficient processing for real-time use
6. **Configuration**: Provide sensible defaults for all options
7. **Documentation**: Document all configuration options
8. **Testing**: Test with various media formats

## Advanced Features

### Hot Reload
Enable hot-reload in the plugin configuration for development:

```rust
let config = PluginConfig {
    hot_reload: true,
    ..Default::default()
};
```

### Plugin Statistics
Get runtime statistics from plugins:

```rust
let stats = registry.get_plugin_stats("plugin_id")?;
println!("Statistics: {:?}", stats);
```

### Configuration Management
Set plugin configuration at runtime:

```rust
registry.set_plugin_config("plugin_id", "key", serde_json::json!(42))?;
```

## Support

For more information, see:
- Plugin API documentation: `src/plugins/api.rs`
- Plugin system guide: `docs/plugins.md`
- Examples: `examples/plugin_system/`