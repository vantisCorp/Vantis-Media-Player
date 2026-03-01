# Example Rust Plugin for Vantis Media Player

This is an example plugin demonstrating how to create plugins for Vantis Media Player using Rust and WebAssembly.

## Features

This plugin provides:

- **Audio Visualization**: Tracks playback statistics
- **Playback Monitoring**: Logs FPS, frame counts, and playback state
- **Custom Commands**: Responds to custom commands:
  - `ping` - Responds with "Pong!"
  - `stats` - Displays current statistics
  - `echo <message>` - Echoes back the message
  - `reset` - Resets the statistics
- **Event Handling**: Responds to media and playback events

## Building

### Prerequisites

- Rust 1.75.0 or later
- wasm32-unknown-unknown target

### Install WASM Target

```bash
rustup target add wasm32-unknown-unknown
```

### Build the Plugin

```bash
cargo build --release --target wasm32-unknown-unknown
```

The compiled plugin will be at:
```
target/wasm32-unknown-unknown/release/rust_example_plugin.wasm
```

## Using the Plugin

### Load the Plugin

```bash
vantis plugins load target/wasm32-unknown-unknown/release/rust_example_plugin.wasm
```

### Send Commands

```bash
# Ping the plugin
vantis plugin send-command rust_example_plugin ping

# Get statistics
vantis plugin send-command rust_example_plugin stats

# Echo a message
vantis plugin send-command rust_example_plugin echo "Hello, World!"

# Reset statistics
vantis plugin send-command rust_example_plugin reset
```

## Plugin Structure

### Main Components

1. **Plugin State**: Tracks frame counts, start time, and log intervals
2. **Plugin Implementation**: Implements the WIT plugin interface
3. **Factory Functions**: Create and destroy plugin instances

### WIT Interface

The plugin implements the `vantis:plugin/plugin` WIT interface, which includes:

- `init()` - Initialize the plugin
- `tick(delta_time)` - Called periodically (60 Hz)
- `handle_command(command, args)` - Handle custom commands
- `on_media_loaded(info)` - Called when media is loaded
- `on_playback_state_changed(state)` - Called on playback state change
- `on_position_changed(position)` - Called on position change
- `on_volume_changed(volume)` - Called on volume change
- `shutdown()` - Cleanup before unloading

## Creating Your Own Plugin

### Step 1: Create a New Project

```bash
cargo new --lib my_vantis_plugin
cd my_vantis_plugin
```

### Step 2: Update Cargo.toml

```toml
[package]
name = "my_vantis_plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wit-bindgen = { version = "0.16.0", features = ["realloc"] }
```

### Step 3: Implement the Plugin

Copy this example's structure and modify the `Plugin` implementation to suit your needs.

### Step 4: Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Step 5: Test

```bash
vantis plugins load target/wasm32-unknown-unknown/release/my_vantis_plugin.wasm
```

## Plugin API

### Host Functions

The plugin can call the following host functions:

```rust
extern "C" {
    // Logging (0: Debug, 1: Info, 2: Warning, 3: Error)
    fn vantis_log(level: u32, message: *const u8, len: u32);
    
    // Media control
    fn vantis_play();
    fn vantis_pause();
    fn vantis_stop();
    fn vantis_seek(time_ms: u32);
    
    // Get media info
    fn vantis_get_media_info() -> MediaInfo;
    
    // Get playback state
    fn vantis_get_playback_state() -> PlaybackState;
    
    // Get/set volume
    fn vantis_get_volume() -> f64;
    fn vantis_set_volume(volume: f64);
}
```

### Types

```rust
struct MediaInfo {
    title: String,
    duration: f64,
    width: u32,
    height: u32,
    audio_channels: u32,
    audio_sample_rate: u32,
}

enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Seeking,
}
```

## Example Output

When loaded, the plugin will log:

```
[INFO] Example Rust Plugin initialized!
[INFO] Version: 0.1.0
[INFO] Author: Vantis Team
[INFO] Media loaded: My Movie
[INFO]   Duration: 7200.5s
[INFO]   Video: 1920x1080
[INFO]   Audio: 2 channels, 48000Hz
[INFO] Playback started
[INFO] Position: 0.0s
[INFO] Current FPS: 60.0
[INFO] Total frames: 300
[INFO] Volume: 80%
```

## Troubleshooting

### Build Errors

**Problem**: `error: linker 'wasm-ld' not found`
- **Solution**: Install lld: `sudo apt-get install lld` (Linux) or use the correct Rust target

**Problem**: `error: unknown crate type 'cdylib'`
- **Solution**: Ensure you're using Rust 1.75.0 or later

### Runtime Errors

**Problem**: `Failed to load plugin`
- **Solution**: Verify the WASM file was compiled correctly and matches the WIT interface

**Problem**: `Plugin crashes on init`
- **Solution**: Check your `init()` implementation for errors. Use `Result<(), String>` to report errors.

## Resources

- [Plugin Development Guide](../../../PLUGIN_DEVELOPMENT.md)
- [API Reference](../../../API_REFERENCE.md)
- [WIT Specification](https://github.com/WebAssembly/component-model)

## License

This example is part of Vantis Media Player and is licensed under the MIT License.