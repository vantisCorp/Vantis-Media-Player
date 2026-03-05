---
sidebar_position: 5
---

# Your First Project

Learn how to integrate Vantis Media Player into your own application.

## Overview

This tutorial will guide you through creating your first application using Vantis Media Player. We'll cover several common scenarios:

1. Web application with the JavaScript SDK
2. Desktop application with the Rust SDK
3. Custom plugin development

## Prerequisites

Before starting, ensure you have:

- Vantis Media Player installed ([Installation Guide](./installation))
- Node.js 18+ or Rust 1.75+ installed
- A code editor (VS Code recommended)
- Basic programming knowledge

## Project 1: Web Player Application

Let's build a web-based media player using the Vantis Media Player Web SDK.

### Step 1: Create Project Structure

```bash
mkdir my-vantis-webapp
cd my-vantis-webapp
npm init -y
```

### Step 2: Install Dependencies

```bash
npm install @vantismedia/web-sdk
```

### Step 3: Create HTML File

Create `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Vantis Player</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="container">
        <h1>My Vantis Player</h1>
        
        <div id="player-container"></div>
        
        <div class="controls">
            <button id="play-btn">Play</button>
            <button id="pause-btn">Pause</button>
            <button id="stop-btn">Stop</button>
            <input type="range" id="volume-slider" min="0" max="100" value="80">
            <span id="volume-label">80%</span>
        </div>
        
        <div class="input-section">
            <input type="text" id="media-url" placeholder="Enter media URL">
            <button id="load-btn">Load</button>
        </div>
        
        <div class="status">
            <span id="status-text">Ready</span>
            <span id="time-display">00:00 / 00:00</span>
        </div>
    </div>
    
    <script type="module" src="main.js"></script>
</body>
</html>
```

### Step 4: Create Styles

Create `styles.css`:

```css
.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    font-family: system-ui, sans-serif;
}

#player-container {
    width: 100%;
    aspect-ratio: 16 / 9;
    background: #000;
    border-radius: 8px;
    margin-bottom: 20px;
}

.controls {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-bottom: 20px;
}

button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    background: #25c2a0;
    color: white;
    cursor: pointer;
    transition: background 0.2s;
}

button:hover {
    background: #1ea384;
}

#volume-slider {
    width: 100px;
}

.input-section {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}

#media-url {
    flex: 1;
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
}

.status {
    display: flex;
    justify-content: space-between;
    color: #666;
}
```

### Step 5: Create Application Logic

Create `main.js`:

```javascript
import { Player, Events } from '@vantismedia/web-sdk';

// Initialize player
const container = document.getElementById('player-container');
const player = new Player(container, {
    theme: 'dark',
    controls: true,
    autoplay: false,
});

// DOM elements
const playBtn = document.getElementById('play-btn');
const pauseBtn = document.getElementById('pause-btn');
const stopBtn = document.getElementById('stop-btn');
const volumeSlider = document.getElementById('volume-slider');
const volumeLabel = document.getElementById('volume-label');
const mediaUrl = document.getElementById('media-url');
const loadBtn = document.getElementById('load-btn');
const statusText = document.getElementById('status-text');
const timeDisplay = document.getElementById('time-display');

// Event handlers
playBtn.addEventListener('click', () => player.play());
pauseBtn.addEventListener('click', () => player.pause());
stopBtn.addEventListener('click', () => player.stop());

volumeSlider.addEventListener('input', (e) => {
    const volume = e.target.value / 100;
    player.setVolume(volume);
    volumeLabel.textContent = `${e.target.value}%`;
});

loadBtn.addEventListener('click', async () => {
    const url = mediaUrl.value.trim();
    if (url) {
        statusText.textContent = 'Loading...';
        try {
            await player.load(url);
            statusText.textContent = 'Ready';
        } catch (error) {
            statusText.textContent = `Error: ${error.message}`;
        }
    }
});

// Player events
player.on(Events.LOADED, () => {
    statusText.textContent = 'Loaded';
});

player.on(Events.PLAYING, () => {
    statusText.textContent = 'Playing';
});

player.on(Events.PAUSED, () => {
    statusText.textContent = 'Paused';
});

player.on(Events.ENDED, () => {
    statusText.textContent = 'Ended';
});

player.on(Events.TIME_UPDATE, (time) => {
    const current = formatTime(time.current);
    const total = formatTime(time.total);
    timeDisplay.textContent = `${current} / ${total}`;
});

player.on(Events.ERROR, (error) => {
    statusText.textContent = `Error: ${error.message}`;
});

// Utility functions
function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    player.destroy();
});
```

### Step 6: Run Your Application

```bash
# Start a local server
npx serve .

# Or use the Vite dev server
npx vite
```

Open your browser to `http://localhost:3000` to see your player in action!

## Project 2: Rust Desktop Application

Now let's build a desktop application using the Rust SDK.

### Step 1: Create Project

```bash
cargo new vantis-desktop-player
cd vantis-desktop-player
```

### Step 2: Add Dependencies

Update `Cargo.toml`:

```toml
[dependencies]
vantismedia = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Step 3: Create Application

Update `src/main.rs`:

```rust
use std::env;
use std::time::Duration;

use vantismedia::{Player, PlayerConfig, Events};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get media file from command line
    let args: Vec<String> = env::args().collect();
    let media_path = args.get(1).expect("Usage: vantis-desktop-player <media-file>");

    // Configure player
    let config = PlayerConfig {
        hardware_acceleration: true,
        volume: 0.8,
        ..Default::default()
    };

    // Create player
    let mut player = Player::new(config)?;

    // Register event handlers
    player.on(Events::Loaded, || {
        println!("Media loaded successfully");
    });

    player.on(Events::Playing, || {
        println!("Now playing...");
    });

    player.on(Events::Ended, || {
        println!("Playback finished");
    });

    // Load and play media
    player.load(media_path).await?;
    player.play();

    // Keep the application running
    println!("Press Ctrl+C to exit");
    tokio::signal::ctrl_c().await?;

    // Cleanup
    player.destroy();

    Ok(())
}
```

### Step 4: Build and Run

```bash
cargo build --release
cargo run --release -- /path/to/video.mp4
```

## Project 3: Custom Plugin

Create a simple plugin that adds audio visualization.

### Step 1: Create Plugin Project

```bash
mkdir vantis-plugin-visualizer
cd vantis-plugin-visualizer
cargo init --lib
```

### Step 2: Configure for WASM

Update `Cargo.toml`:

```toml
[package]
name = "vantis-plugin-visualizer"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
vantis-plugin-sdk = "0.1"
wasm-bindgen = "0.2"

[profile.release]
opt-level = "s"
lto = true
```

### Step 3: Implement Plugin

Update `src/lib.rs`:

```rust
use vantis_plugin_sdk::{Plugin, PluginContext, AudioData};

#[derive(Debug)]
pub struct VisualizerPlugin {
    name: String,
    enabled: bool,
}

impl Plugin for VisualizerPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.log("Visualizer plugin loaded!");
    }

    fn on_unload(&mut self) {
        // Cleanup
    }

    fn on_audio_data(&mut self, data: &AudioData) {
        // Process audio data for visualization
        self.render_visualization(data);
    }
}

impl VisualizerPlugin {
    pub fn new() -> Self {
        Self {
            name: "Audio Visualizer".to_string(),
            enabled: true,
        }
    }

    fn render_visualization(&self, data: &AudioData) {
        // Your visualization logic here
        let samples = data.samples();
        // Render bars, waveform, spectrum, etc.
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(VisualizerPlugin::new())
}
```

### Step 4: Build for WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Step 5: Install Plugin

```bash
vantismedia plugin install target/wasm32-unknown-unknown/release/vantis_plugin_visualizer.wasm
```

## Testing Your Integration

### Unit Tests

Create `src/player.test.js`:

```javascript
import { Player } from '@vantismedia/web-sdk';

describe('Player', () => {
    let player;

    beforeEach(() => {
        const container = document.createElement('div');
        player = new Player(container);
    });

    afterEach(() => {
        player.destroy();
    });

    test('should initialize with default config', () => {
        expect(player).toBeDefined();
    });

    test('should load media successfully', async () => {
        await expect(player.load('test.mp4')).resolves.not.toThrow();
    });

    test('should emit playing event', async () => {
        const handler = jest.fn();
        player.on('playing', handler);
        await player.load('test.mp4');
        player.play();
        expect(handler).toHaveBeenCalled();
    });
});
```

### Integration Tests

```rust
// tests/integration_test.rs
use vantismedia::Player;

#[tokio::test]
async fn test_player_loads_media() {
    let player = Player::new(Default::default()).unwrap();
    let result = player.load("test.mp4").await;
    assert!(result.is_ok());
}
```

## Best Practices

### Memory Management

Always clean up resources when the player is no longer needed:

```javascript
// JavaScript
window.addEventListener('beforeunload', () => {
    player.destroy();
});
```

```rust
// Rust
// RAII handles cleanup automatically
drop(player);
```

### Error Handling

Handle errors gracefully:

```javascript
try {
    await player.load(url);
} catch (error) {
    if (error.code === 'UNSUPPORTED_FORMAT') {
        showUserMessage('This format is not supported');
    } else if (error.code === 'NETWORK_ERROR') {
        showUserMessage('Could not load the media. Check your connection.');
    } else {
        showUserMessage('An error occurred: ' + error.message);
    }
}
```

### Performance

Optimize for performance:

```javascript
// Use hardware acceleration
const player = new Player(container, {
    hardwareAcceleration: true,
});

// Preload media for smoother playback
player.preload(url);
```

## Next Steps

Now that you've built your first project:

- **[API Reference](../api/)** - Dive deeper into the API
- **[Plugin Development](../plugins/)** - Learn more about plugins
- **[Examples](../examples/)** - See more code examples
- **[Architecture](../architecture/)** - Understand the internals

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)
- [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)