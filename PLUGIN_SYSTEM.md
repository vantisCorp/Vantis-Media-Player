# Plugin System Documentation

## Overview

The Vantis Media Player plugin system is inspired by VideoLAN's modular architecture (400+ plugins in VLC). It provides a flexible, extensible framework for adding functionality through plugins supporting multiple capabilities, formats, and languages.

## Architecture

```
Plugin System
├── API (api.rs)              - Core plugin interface and types
├── Discovery (discovery.rs)  - Plugin discovery and validation
├── Loader (loader.rs)        - Plugin loading and lifecycle management
├── Registry (registry.rs)    - Central plugin management
└── Examples (plugins/example/)- Example plugins
```

## Core Components

### 1. Plugin API (`api.rs`)

The plugin API defines the interface that all plugins must implement:

```rust
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_config(&self, key: &str) -> Option<serde_json::Value>;
    fn set_config(&mut self, key: &str, value: serde_json::Value) -> Result<(), String>;
    fn process(&mut self, input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn get_stats(&self) -> HashMap<String, serde_json::Value>;
}
```

### Plugin Capabilities

The system supports 18 different plugin capabilities:

| Category | Capabilities |
|----------|-------------|
| **Decoders** | audio_decoder, video_decoder, subtitle_decoder |
| **Encoders** | audio_encoder, video_encoder, subtitle_encoder |
| **Filters** | audio_filter, video_filter, subtitle_filter |
| **I/O** | demuxer, muxer, network_protocol |
| **UI** | ui_component, theme, visualizer |
| **Extensions** | extension, script, ai_model, hardware_acceleration |

### Plugin Metadata

Every plugin must provide metadata:

```rust
pub struct PluginMetadata {
    pub id: String,           // Unique identifier
    pub name: String,         // Display name
    pub version: String,      // Semantic version
    pub api_version: u32,     // Plugin API version
    pub description: String,  // Description
    pub author: String,       // Author name
    pub license: String,      // License type
    pub homepage: Option<String>,
    pub capabilities: Vec<PluginCapability>,
    pub dependencies: Vec<String>,
    pub formats: Vec<String>,
    pub config_options: HashMap<String, ConfigOption>,
}
```

### 2. Plugin Discovery (`discovery.rs`)

The discovery engine finds and validates plugins:

```rust
pub struct PluginDiscovery {
    plugin_dirs: Vec<PathBuf>,
    enabled_capabilities: HashSet<PluginCapability>,
}
```

**Supported Plugin Formats**:
- Native shared libraries (.so, .dylib, .dll)
- WebAssembly modules (.wasm)
- Manifest files (.json, .toml)
- Directory plugins with manifest

**Discovery Methods**:
- `discover_all()` - Discover all plugins in registered directories
- `discover_in_directory()` - Discover plugins in specific directory
- `discover_plugin()` - Discover a single plugin at given path
- `validate_metadata()` - Validate plugin metadata

### 3. Plugin Loader (`loader.rs`)

The loader manages plugin lifecycle:

```rust
pub struct PluginLoader {
    loaded_plugins: HashMap<String, Arc<LoadedPlugin>>,
    config: PluginConfig,
}
```

**Loader Operations**:
- `load_plugin()` - Load a plugin by ID
- `unload_plugin()` - Unload a plugin by ID
- `reload_plugin()` - Reload a plugin (hot-reload)
- `get_plugin()` - Get a loaded plugin
- `get_plugins_by_capability()` - Get plugins with specific capability

**Load Statistics**:
```rust
pub struct LoadStats {
    pub total_plugins: usize,
    pub total_load_time: u64,
    pub average_load_time: u64,
}
```

### 4. Plugin Registry (`registry.rs`)

The registry provides central plugin management:

```rust
pub struct PluginRegistry {
    loader: PluginLoader,
    discovery: PluginDiscovery,
    plugins: HashMap<String, Arc<LoadedPlugin>>,
    capability_index: HashMap<PluginCapability, Vec<String>>,
}
```

**Registry Operations**:
- `discover_plugins()` - Discover all available plugins
- `load_plugin()` - Load and register a plugin
- `unload_plugin()` - Unload and remove a plugin
- `get_plugin()` - Get a registered plugin
- `get_plugins_by_capability()` - Get plugins by capability
- `get_registry_stats()` - Get registry statistics
- `shutdown()` - Shutdown all plugins

## Usage Example

### Basic Usage

```rust
use vantis_player::plugins::{PluginRegistry, PluginCapability};

// Create registry
let mut registry = PluginRegistry::new();

// Add plugin directories
registry.add_plugin_dir("plugins");
registry.add_plugin_dir("/usr/local/lib/vantis/plugins");

// Enable capabilities
registry.enable_capability(PluginCapability::AudioDecoder);
registry.enable_capability(PluginCapability::VideoDecoder);

// Discover plugins
let plugins = registry.discover_plugins()?;
println!("Found {} plugins", plugins.len());

// List plugins
for metadata in &plugins {
    println!("  - {} v{} ({})", 
             metadata.name, 
             metadata.version, 
             metadata.id);
}

// Load a plugin
let plugin = registry.load_plugin("example_audio_decoder")?;
println!("Loaded: {}", plugin.metadata.name);

// Get plugins by capability
let audio_decoders = registry.get_plugins_by_capability(PluginCapability::AudioDecoder);
println!("Audio decoders: {}", audio_decoders.len());

// Get plugin statistics
let stats = registry.get_plugin_stats("example_audio_decoder")?;
println!("Stats: {:?}", stats);

// Unload plugin
registry.unload_plugin("example_audio_decoder")?;

// Shutdown all plugins
registry.shutdown()?;
```

### Advanced Usage

```rust
use vantis_player::plugins::{PluginConfig, PluginRegistry};

// Custom configuration
let config = PluginConfig {
    plugin_dir: PathBuf::from("plugins"),
    hot_reload: true,        // Enable hot-reload for development
    validate: true,          // Enable plugin validation
    max_load_time: 30,       // Maximum load time in seconds
};

let mut registry = PluginRegistry::with_config(config);

// Enable all capabilities
registry.enable_all_capabilities();

// Discover and load all plugins
let plugins = registry.discover_plugins()?;
for metadata in &plugins {
    if let Ok(plugin) = registry.load_plugin(&metadata.id) {
        println!("Loaded: {} v{}", plugin.metadata.name, plugin.metadata.version);
    }
}

// Get registry statistics
let stats = registry.get_registry_stats();
println!("Registry stats:");
println!("  Total plugins: {}", stats.total_plugins);
println!("  Total capabilities: {}", stats.total_capabilities);
println!("  Total load time: {}ms", stats.total_load_time);
println!("  Average load time: {}ms", stats.average_load_time);

// Configure plugin
registry.set_plugin_config("example_audio_decoder", "sample_rate", serde_json::json!(44100))?;

// Get configuration
let config = registry.get_plugin_config("example_audio_decoder", "sample_rate")?;
println!("Sample rate: {:?}", config);

// Hot-reload plugin (if hot-reload is enabled)
registry.reload_plugin("example_audio_decoder")?;
```

## Plugin Development

### Creating a Native Plugin

1. **Create Plugin Directory**:
```bash
mkdir -p plugins/my_plugin
cd plugins/my_plugin
```

2. **Create Manifest** (`plugin.json`):
```json
{
  "id": "com.example.my_plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "api_version": 1,
  "description": "My awesome plugin",
  "author": "Your Name",
  "license": "MIT",
  "capabilities": ["audio_decoder"],
  "dependencies": [],
  "formats": ["mp3"],
  "config_options": {}
}
```

3. **Implement Plugin Trait**:
```rust
use vantis_player::plugins::{Plugin, PluginMetadata, PluginCapability};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata {
        static METADATA: std::sync::OnceLock<PluginMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| {
            PluginMetadata::new("com.example.my_plugin", "My Plugin", "1.0.0")
                .with_capability(PluginCapability::AudioDecoder)
                .with_format("mp3")
        })
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("MyPlugin initialized");
        Ok(())
    }

    fn process(&mut self, input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Process audio data
        Ok(input.to_vec())
    }
}
```

4. **Build as Shared Library**:
```bash
cargo build --release --crate-type cdylib
```

### Creating a WASM Plugin

1. **Create Plugin Directory**:
```bash
mkdir -p plugins/my_wasm_plugin
cd plugins/my_wasm_plugin
```

2. **Create Manifest** (`plugin.json`):
```json
{
  "id": "com.example.my_wasm_plugin",
  "name": "My WASM Plugin",
  "version": "1.0.0",
  "api_version": 1,
  "description": "My awesome WASM plugin",
  "author": "Your Name",
  "license": "MIT",
  "capabilities": ["video_filter"],
  "dependencies": [],
  "formats": ["rgb"],
  "config_options": {}
}
```

3. **Build WASM Module**:
```bash
wasm-pack build --target web
```

## Plugin Configuration

### Configuration Options

Plugins can expose configuration options:

```json
{
  "config_options": {
    "option_name": {
      "description": "Option description",
      "value_type": "Integer|Float|Boolean|String|Enum|List",
      "default_value": 42,
      "required": true,
      "min_value": 0,
      "max_value": 100,
      "allowed_values": [1, 2, 3]
    }
  }
}
```

### Value Types

| Type | Description | Example |
|------|-------------|---------|
| `Integer` | Integer values | `42` |
| `Float` | Floating-point values | `3.14` |
| `Boolean` | Boolean values | `true` |
| `String` | String values | `"hello"` |
| `Enum` | Enumerated values | `["option1", "option2"]` |
| `List` | List of values | `[1, 2, 3]` |

## Best Practices

### Plugin Development

1. **Unique IDs**: Use reverse domain notation (e.g., `com.example.plugin`)
2. **Versioning**: Follow semantic versioning (MAJOR.MINOR.PATCH)
3. **Error Handling**: Return meaningful error messages
4. **Performance**: Implement efficient processing for real-time use
5. **Thread Safety**: Ensure `Send + Sync` trait implementation
6. **Configuration**: Provide sensible defaults for all options
7. **Documentation**: Document all capabilities and options

### Plugin Discovery

1. **Directory Structure**: Organize plugins by type
2. **Manifest Files**: Always include a manifest file
3. **Validation**: Validate metadata before loading
4. **Dependencies**: Check dependencies before loading
5. **Capabilities**: Filter by capabilities when needed

### Plugin Loading

1. **Load Order**: Load dependencies first
2. **Error Handling**: Handle load failures gracefully
3. **Hot Reload**: Use hot-reload only in development
4. **Validation**: Validate plugin API version
5. **Performance**: Monitor load times

## Comparison with VideoLAN

| Feature | VideoLAN (VLC) | Vantis |
|---------|---------------|--------|
| Plugin Count | 400+ | Extensible |
| Plugin API | C-based | Rust-based |
| Hot Reload | ❌ | ✅ |
| WASM Support | ❌ | ✅ |
| Capability System | ✅ | ✅ |
| Metadata | ✅ | ✅ |
| Configuration | ✅ | ✅ |
| Statistics | ✅ | ✅ |

## Advanced Features

### Hot Reload

Enable hot-reload for development:

```rust
let config = PluginConfig {
    hot_reload: true,
    ..Default::default()
};
```

### Capability Indexing

Automatic indexing by capabilities for fast lookup:

```rust
let video_filters = registry.get_plugins_by_capability(PluginCapability::VideoFilter);
```

### Plugin Statistics

Get runtime statistics:

```rust
let stats = registry.get_registry_stats();
```

### Configuration Management

Set plugin configuration at runtime:

```rust
registry.set_plugin_config("plugin_id", "key", serde_json::json!(42))?;
```

## Troubleshooting

### Plugin Not Found

**Problem**: Plugin not found when loading

**Solution**:
1. Check plugin directory path
2. Verify manifest file exists
3. Check plugin ID matches manifest

### Load Failed

**Problem**: Plugin load failed

**Solution**:
1. Check plugin API version
2. Verify dependencies are loaded
3. Check plugin logs
4. Validate manifest syntax

### Performance Issues

**Problem**: Plugin slow to load or process

**Solution**:
1. Check load time statistics
2. Optimize initialization
3. Use caching where appropriate
4. Profile plugin code

## Future Improvements

- [ ] Plugin marketplace
- [ ] Plugin sandboxing
- [ ] Plugin version compatibility
- [ ] Plugin dependency resolution
- [ ] Plugin hot-patching
- [ ] Plugin telemetry
- [ ] Plugin auto-update
- [ ] Plugin signing and verification

## References

- Plugin API: `src/plugins/api.rs`
- Plugin Discovery: `src/plugins/discovery.rs`
- Plugin Loader: `src/plugins/loader.rs`
- Plugin Registry: `src/plugins/registry.rs`
- Example Plugins: `plugins/example/`
- VideoLAN Plugin System: https://code.videolan.org/videolan/vlc/tree/master/modules

---

*Last updated: 2026-03-06*
*Version: 1.0.0*
*Plugin API Version: 1*