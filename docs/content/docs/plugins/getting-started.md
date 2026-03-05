---
sidebar_position: 2
---

# Plugin Getting Started

Create your first plugin for Vantis Media Player with this step-by-step guide.

## Prerequisites

Before you begin, ensure you have:

- **For JavaScript plugins**: Node.js 18+ and npm
- **For WASM plugins**: Rust 1.75+ and cargo
- **Vantis Media Player** installed for testing

## Create a JavaScript Plugin

JavaScript plugins are the easiest way to get started with plugin development.

### Step 1: Create Project Directory

```bash
mkdir my-first-plugin
cd my-first-plugin
npm init -y
```

### Step 2: Create Plugin File

Create `index.js`:

```javascript
/**
 * My First Plugin
 * A simple plugin that logs playback events
 */
class MyFirstPlugin {
    constructor() {
        this.name = 'My First Plugin';
        this.version = '1.0.0';
        this.description = 'A simple logging plugin';
    }

    /**
     * Called when plugin is loaded
     */
    onLoad(player, context) {
        this.player = player;
        this.context = context;
        
        context.log('My First Plugin loaded!');
        
        // Register event listeners
        this.registerEventListeners();
        
        // Add a simple UI element
        this.addUI();
    }

    /**
     * Register player event listeners
     */
    registerEventListeners() {
        // Track playback events
        this.player.on('playing', () => {
            this.context.log('▶️ Playback started');
        });

        this.player.on('pause', () => {
            this.context.log('⏸️ Playback paused');
        });

        this.player.on('ended', () => {
            this.context.log('🏁 Playback ended');
        });

        this.player.on('timeupdate', (data) => {
            // Log every 10 seconds
            if (Math.floor(data.currentTime) % 10 === 0) {
                this.context.debug(`Time: ${data.currentTime}s`);
            }
        });
    }

    /**
     * Add custom UI elements
     */
    addUI() {
        // Add a button to the control bar
        this.context.addButton({
            id: 'my-plugin-button',
            icon: 'info',
            label: 'Plugin Info',
            position: 'right',
            onClick: () => {
                this.showInfo();
            }
        });
    }

    /**
     * Show plugin information
     */
    showInfo() {
        const mediaInfo = this.player.getMediaInfo();
        this.context.showNotification(`
            ${this.name} v${this.version}
            Duration: ${Math.round(mediaInfo.duration)}s
        `);
    }

    /**
     * Called when plugin is unloaded
     */
    onUnload() {
        this.context.log('My First Plugin unloaded');
    }
}

// Export for ES modules
export default MyFirstPlugin;

// Also support CommonJS
if (typeof module !== 'undefined') {
    module.exports = MyFirstPlugin;
}
```

### Step 3: Create Plugin Manifest

Create `plugin.json`:

```json
{
    "name": "my-first-plugin",
    "version": "1.0.0",
    "description": "A simple logging plugin for Vantis Media Player",
    "author": "Your Name",
    "license": "MIT",
    "type": "javascript",
    "main": "index.js",
    "permissions": [
        "player-events",
        "ui-customization"
    ],
    "minVersion": "1.0.0"
}
```

### Step 4: Test Your Plugin

```bash
# Install in Vantis Media Player
vantismedia plugin install ./index.js

# Test with a video
vantismedia --debug video.mp4
```

### Step 5: Debug Your Plugin

Enable debug logging:

```javascript
// In your plugin
this.context.setDebugMode(true);
```

Or via command line:

```bash
vantismedia --plugin-debug --verbose video.mp4
```

## Create a WASM Plugin

WebAssembly plugins offer better performance and cross-platform compatibility.

### Step 1: Create Rust Project

```bash
cargo new my-wasm-plugin --lib
cd my-wasm-plugin
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "my-wasm-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
vantis-plugin-sdk = "0.1"
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[profile.release]
opt-level = "s"
lto = true
```

### Step 3: Implement Plugin

Create `src/lib.rs`:

```rust
use vantis_plugin_sdk::{Plugin, PluginContext, AudioData, VideoFrame};
use wasm_bindgen::prelude::*;

/// My WASM Plugin
pub struct MyWasmPlugin {
    name: String,
    version: String,
    enabled: bool,
}

impl MyWasmPlugin {
    pub fn new() -> Self {
        Self {
            name: "My WASM Plugin".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
        }
    }
}

impl Plugin for MyWasmPlugin {
    /// Get plugin name
    fn name(&self) -> &str {
        &self.name
    }

    /// Get plugin version
    fn version(&self) -> &str {
        &self.version
    }

    /// Called when plugin is loaded
    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.log("My WASM Plugin loaded!");
    }

    /// Called when plugin is initialized
    fn on_init(&mut self, _ctx: &mut PluginContext) {
        // Initialize plugin resources
    }

    /// Called when plugin is activated
    fn on_activate(&mut self, _ctx: &mut PluginContext) {
        self.enabled = true;
    }

    /// Called when plugin is deactivated
    fn on_deactivate(&mut self, _ctx: &mut PluginContext) {
        self.enabled = false;
    }

    /// Process audio data
    fn on_audio_data(&mut self, data: &AudioData) {
        if !self.enabled {
            return;
        }

        // Access audio samples
        let samples = data.samples();
        
        // Calculate average amplitude
        let sum: f32 = samples.iter().sum();
        let avg = sum / samples.len() as f32;
        
        // Log average amplitude
        // Note: This is just an example - in real plugins,
        // you'd want to do something useful with this data
        let _ = avg; // Use the value to avoid warning
    }

    /// Process video frame
    fn on_video_frame(&mut self, _frame: &VideoFrame) {
        if !self.enabled {
            return;
        }

        // Process video frame
    }

    /// Handle configuration changes
    fn on_config_change(&mut self, config: &serde_json::Value) {
        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
            self.enabled = enabled;
        }
    }

    /// Called when plugin is unloaded
    fn on_unload(&mut self) {
        // Cleanup resources
    }
}

/// Create plugin instance
#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyWasmPlugin::new())
}
```

### Step 4: Build for WASM

```bash
# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

### Step 5: Install Plugin

```bash
vantismedia plugin install target/wasm32-unknown-unknown/release/my_wasm_plugin.wasm
```

## Create a Native Plugin

Native plugins provide direct system access and maximum performance.

### Step 1: Create Rust Project

```bash
cargo new my-native-plugin --lib
cd my-native-plugin
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "my-native-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
vantis-native-plugin = "0.1"
```

### Step 3: Implement Plugin

Create `src/lib.rs`:

```rust
use vantis_native_plugin::{Plugin, PluginContext};

pub struct MyNativePlugin {
    // Plugin state
}

impl MyNativePlugin {
    pub fn new() -> Self {
        Self {
            // Initialize state
        }
    }
}

impl Plugin for MyNativePlugin {
    fn name(&self) -> &str {
        "My Native Plugin"
    }

    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.log("My Native Plugin loaded!");
        
        // Access native system features
        // This is where you'd integrate with system APIs
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyNativePlugin::new())
}
```

### Step 4: Build Native Library

```bash
# Build for current platform
cargo build --release

# Output:
# Linux: libmy_native_plugin.so
# macOS: libmy_native_plugin.dylib
# Windows: my_native_plugin.dll
```

### Step 5: Install Plugin

```bash
# Linux
vantismedia plugin install target/release/libmy_native_plugin.so

# macOS
vantismedia plugin install target/release/libmy_native_plugin.dylib

# Windows
vantismedia plugin install target/release/my_native_plugin.dll
```

## Testing Your Plugin

### Manual Testing

```bash
# Run with plugin enabled
vantismedia --plugin-debug video.mp4

# Check plugin output
vantismedia --verbose video.mp4 2>&1 | grep -i plugin
```

### Automated Testing

Create `test.js`:

```javascript
const assert = require('assert');
const { Player } = require('@vantismedia/web-sdk');
const MyPlugin = require('./index');

describe('MyPlugin', () => {
    let player;
    let plugin;

    beforeEach(() => {
        player = new Player('#test-container');
        plugin = new MyPlugin();
        plugin.onLoad(player, {
            log: console.log,
            addButton: () => {}
        });
    });

    afterEach(() => {
        player.destroy();
    });

    it('should have correct name', () => {
        assert.strictEqual(plugin.name, 'My First Plugin');
    });

    it('should respond to play event', () => {
        let logged = false;
        plugin.context = {
            log: (msg) => {
                if (msg.includes('started')) logged = true;
            }
        };
        player.emit('playing');
        // Check if logged
    });
});
```

Run tests:

```bash
npm test
```

## Debugging Tips

### JavaScript Plugin Debugging

```javascript
// Use console.log for debugging
console.log('Debug info:', someValue);

// Use debugger statement
debugger;

// Use context logging
this.context.debug('Debug message');
```

### WASM Plugin Debugging

```rust
// Use context logging
ctx.log("Debug message");
ctx.debug("Debug value: {}", value);

// Use console_log crate for browser console
use web_sys::console;
console::log_1(&"Debug message".into());
```

### Native Plugin Debugging

```rust
// Use eprintln for debugging
eprintln!("Debug: {:?}", value);

// Use logging crate
use log::{info, debug, error};
debug!("Debug message");
```

## Common Issues

### Plugin Not Loading

```bash
# Check plugin file exists
ls -la my-plugin.js

# Check plugin syntax
node --check my-plugin.js

# Check permissions
vantismedia plugin check my-plugin.js
```

### Plugin Not Working

```bash
# Enable verbose logging
vantismedia --verbose --plugin-debug video.mp4

# Check plugin is enabled
vantismedia plugin list --installed
```

### Build Errors (WASM)

```bash
# Ensure wasm target is installed
rustup target add wasm32-unknown-unknown

# Update dependencies
cargo update

# Clean and rebuild
cargo clean && cargo build --target wasm32-unknown-unknown --release
```

## Next Steps

- **[Plugin API](./plugin-api)** - Complete API reference
- **[WASM Plugins](./wasm-plugins)** - Advanced WASM development
- **[Examples](./examples)** - Real-world plugin examples

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)