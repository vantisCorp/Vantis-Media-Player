---
sidebar_position: 3
---

# WebAssembly Plugins

WebAssembly (WASM) plugins allow you to write high-performance extensions in languages like Rust, C++, or Go while still running in the browser. This guide covers advanced WASM plugin development for Vantis Media Player.

## Why WASM Plugins?

WASM plugins offer several advantages:

- **Performance**: Near-native execution speed
- **Language Support**: Write in Rust, C++, Go, AssemblyScript
- **Safety**: Memory-safe sandboxed execution
- **Portability**: Run on any platform
- **Size**: Compact binary format

## Architecture

```
┌─────────────────────────────────────┐
│  Vantis Media Player               │
└──────────┬──────────────────────────┘
           │
┌──────────▼──────────────────────────┐
│  WASM Plugin Loader                 │
├─────────────────────────────────────┤
│  Plugin Interface (JavaScript)      │
└──────────┬──────────────────────────┘
           │
┌──────────▼──────────────────────────┐
│  WASM Runtime                       │
├─────────────────────────────────────┤
│  Plugin Logic (Rust/C++/Go)         │
└─────────────────────────────────────┘
```

## Rust WASM Plugins

### Project Setup

```bash
# Create new Rust project
cargo new --lib my-wasm-plugin
cd my-wasm-plugin

# Add WASM dependencies
cargo add wasm-bindgen
cargo add vantis-player-wasm

# Enable wasm32 target
rustup target add wasm32-unknown-unknown
```

### Cargo.toml

```toml
[package]
name = "my-wasm-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
vantis-player-wasm = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Window",
  "Performance",
]
```

### Basic Plugin Structure

```rust
use wasm_bindgen::prelude::*;
use vantis_player_wasm::{Plugin, Player, PlayerEvents};
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct MyWasmPlugin {
    player: Option<Player>,
    config: PluginConfig,
}

#[derive(Deserialize, Serialize)]
pub struct PluginConfig {
    threshold: f64,
    enabled: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            threshold: 0.5,
            enabled: true,
        }
    }
}

#[wasm_bindgen]
impl MyWasmPlugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log::init().ok();
        log::info!("MyWasmPlugin initialized");
        
        MyWasmPlugin {
            player: None,
            config: PluginConfig::default(),
        }
    }
    
    pub fn id(&self) -> String {
        "my-wasm-plugin".to_string()
    }
    
    pub fn name(&self) -> String {
        "My WASM Plugin".to_string()
    }
    
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

impl Plugin for MyWasmPlugin {
    fn on_load(&mut self, player: &mut Player) {
        log::info!("Plugin loaded!");
        self.player = Some(player.clone());
        
        // Register event handlers
        player.on(PlayerEvents::Play, Box::new(|_| {
            log::info!("Playback started");
        }));
        
        player.on(PlayerEvents::TimeUpdate, Box::new(|time| {
            if self.config.enabled && time > self.config.threshold {
                log::info!("Threshold reached at {}", time);
            }
        }));
    }
    
    fn on_unload(&mut self) {
        log::info!("Plugin unloaded!");
        self.player = None;
    }
    
    fn configure(&mut self, config: serde_json::Value) -> Result<(), String> {
        self.config = serde_json::from_value(config)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    
    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap()
    }
}
```

### Audio Processing Plugin

```rust
use vantis_player_wasm::{AudioBuffer, AudioProcessor};

#[wasm_bindgen]
pub struct AudioNormalizer {
    target_level: f64,
    smoothing: f64,
}

#[wasm_bindgen]
impl AudioNormalizer {
    #[wasm_bindgen(constructor)]
    pub fn new(target_level: f64, smoothing: f64) -> Self {
        AudioNormalizer {
            target_level,
            smoothing,
        }
    }
}

impl AudioProcessor for AudioNormalizer {
    fn process(&mut self, buffer: &mut AudioBuffer) {
        let channels = buffer.channels();
        let samples = buffer.length();
        
        for ch in 0..channels {
            let data = buffer.get_channel_data_mut(ch);
            let mut level = self.calculate_rms(data);
            
            for i in 0..samples {
                // Apply normalization with smoothing
                let factor = self.smooth_factor(level);
                data[i] *= factor;
            }
        }
    }
}

impl AudioNormalizer {
    fn calculate_rms(&self, data: &[f32]) -> f64 {
        let sum: f64 = data.iter().map(|&x| (x * x) as f64).sum();
        (sum / data.len() as f64).sqrt()
    }
    
    fn smooth_factor(&self, current_level: f64) -> f64 {
        if current_level == 0.0 {
            return 1.0;
        }
        let target = self.target_level / current_level;
        target.clamp(0.5, 2.0)
    }
}
```

### Video Filter Plugin

```rust
use vantis_player_wasm::{VideoFrame, VideoFilter};

#[wasm_bindgen]
pub struct EdgeDetectionFilter {
    threshold: u8,
}

#[wasm_bindgen]
impl EdgeDetectionFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(threshold: u8) -> Self {
        EdgeDetectionFilter { threshold }
    }
}

impl VideoFilter for EdgeDetectionFilter {
    fn process(&mut self, frame: &mut VideoFrame) {
        let width = frame.width();
        let height = frame.height();
        let data = frame.data_mut();
        
        // Apply Sobel operator
        for y in 1..height-1 {
            for x in 1..width-1 {
                let idx = (y * width + x) * 4;
                
                // Get pixel values
                let gx = self.calculate_gx(data, x, y, width);
                let gy = self.calculate_gy(data, x, y, width);
                
                let magnitude = ((gx * gx + gy * gy) as f32).sqrt();
                
                if magnitude > self.threshold as f32 {
                    data[idx] = 255;     // R
                    data[idx + 1] = 255; // G
                    data[idx + 2] = 255; // B
                } else {
                    data[idx] = 0;
                    data[idx + 1] = 0;
                    data[idx + 2] = 0;
                }
            }
        }
    }
}

impl EdgeDetectionFilter {
    fn calculate_gx(&self, data: &[u8], x: usize, y: usize, width: usize) -> i32 {
        let get_pixel = |dx: i32, dy: i32| -> u8 {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            let idx = (ny * width + nx) * 4;
            data[idx] // Using red channel
        };
        
        (-1 * get_pixel(-1, -1) as i32
         + 1 * get_pixel(1, -1) as i32
         - 2 * get_pixel(-1, 0) as i32
         + 2 * get_pixel(1, 0) as i32
         - 1 * get_pixel(-1, 1) as i32
         + 1 * get_pixel(1, 1) as i32).abs()
    }
    
    fn calculate_gy(&self, data: &[u8], x: usize, y: usize, width: usize) -> i32 {
        let get_pixel = |dx: i32, dy: i32| -> u8 {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            let idx = (ny * width + nx) * 4;
            data[idx]
        };
        
        (-1 * get_pixel(-1, -1) as i32
         - 2 * get_pixel(0, -1) as i32
         - 1 * get_pixel(1, -1) as i32
         + 1 * get_pixel(-1, 1) as i32
         + 2 * get_pixel(0, 1) as i32
         + 1 * get_pixel(1, 1) as i32).abs()
    }
}
```

### Building the Plugin

```bash
# Build WASM
wasm-pack build --target web --out-dir pkg

# Output structure:
# pkg/
#   ├── my_wasm_plugin_bg.wasm  # WASM binary
#   ├── my_wasm_plugin.js       # JavaScript bindings
#   ├── my_wasm_plugin.d.ts     # TypeScript definitions
#   └── package.json
```

### Loading WASM Plugin

```javascript
// Load WASM plugin in JavaScript
import init, { MyWasmPlugin } from './pkg/my_wasm_plugin.js';

async function loadWasmPlugin(player) {
  // Initialize WASM module
  await init();
  
  // Create plugin instance
  const plugin = new MyWasmPlugin();
  
  // Load into player
  await player.plugins.load(plugin);
}
```

## C++ WASM Plugins

### Project Setup

```bash
# Create Emscripten project
mkdir my-cpp-plugin
cd my-cpp-plugin

# Initialize with Emscripten
emcmake cmake .. -DCMAKE_BUILD_TYPE=Release
```

### CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.10)
project(MyCppPlugin)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Emscripten
find_package(EMSCRIPTEN)

# Add WASM support
set(CMAKE_EXECUTABLE_SUFFIX ".js")

# Source files
set(SOURCES
    src/plugin.cpp
    src/audio_processor.cpp
)

# Create library
add_library(${PROJECT_NAME} SHARED ${SOURCES})

# Link with Emscripten
target_link_libraries(${PROJECT_NAME}
    ${EMSCRIPTEN_LIBRARIES}
    -s EXPORTED_FUNCTIONS=["_createPlugin","_destroyPlugin"]
    -s EXPORTED_RUNTIME_METHODS=["ccall","cwrap"]
    -s MODULARIZE=1
    -s EXPORT_NAME="createModule"
)

# Set output
set_target_properties(${PROJECT_NAME} PROPERTIES
    SUFFIX ".wasm"
)
```

### Plugin Implementation

```cpp
#include <emscripten.h>
#include <emscripten/bind.h>
#include <cmath>
#include <vector>

using namespace emscripten;
using namespace std;

class AudioCompressor {
private:
    double threshold;
    double ratio;
    double attack;
    double release;
    
public:
    AudioCompressor(double t, double r, double a, double rel)
        : threshold(t), ratio(r), attack(a), release(rel) {}
    
    val process(val audioData) {
        vector<float> audio = convertJSArrayToVector<float>(audioData);
        
        for (size_t i = 0; i < audio.size(); i++) {
            float sample = audio[i];
            float envelope = 0.0f;
            
            // Calculate envelope
            if (fabs(sample) > envelope) {
                envelope += (fabs(sample) - envelope) * (1.0 - exp(-1.0 / attack));
            } else {
                envelope += (fabs(sample) - envelope) * (1.0 - exp(-1.0 / release));
            }
            
            // Apply compression
            if (envelope > threshold) {
                double gain = threshold + (envelope - threshold) / ratio;
                sample *= static_cast<float>(gain / envelope);
            }
            
            audio[i] = sample;
        }
        
        return val::array(audio);
    }
    
    void setThreshold(double t) { threshold = t; }
    void setRatio(double r) { ratio = r; }
    void setAttack(double a) { attack = a; }
    void setRelease(double r) { release = r; }
};

EMSCRIPTEN_BINDINGS(audio_compressor) {
    class_<AudioCompressor>("AudioCompressor")
        .constructor<double, double, double, double>()
        .function("process", &AudioCompressor::process)
        .function("setThreshold", &AudioCompressor::setThreshold)
        .function("setRatio", &AudioCompressor::setRatio)
        .function("setAttack", &AudioCompressor::setAttack)
        .function("setRelease", &AudioCompressor::setRelease);
}
```

### Building C++ Plugin

```bash
# Configure
emcmake cmake . -DCMAKE_BUILD_TYPE=Release

# Build
emmake make

# Output: my_cpp_plugin.wasm, my_cpp_plugin.js
```

## Go WASM Plugins

### Project Setup

```bash
mkdir my-go-plugin
cd my-go-plugin
go mod init my-go-plugin
```

### Plugin Implementation

```go
package main

import (
    "encoding/json"
    "fmt"
    "syscall/js"
)

type Plugin struct {
    name     string
    version  string
    callback js.Value
}

type Config struct {
    Enabled  bool    `json:"enabled"`
    Interval float64 `json:"interval"`
}

func NewPlugin() *Plugin {
    return &Plugin{
        name:    "my-go-plugin",
        version: "1.0.0",
    }
}

func (p *Plugin) OnLoad(player js.Value) {
    fmt.Println("Go plugin loaded!")
    
    // Register callback
    p.callback = js.Global().Get("Function").New(
        "console.log('Callback from Go plugin')",
    )
}

func (p *Plugin) OnUnload() {
    fmt.Println("Go plugin unloaded!")
}

func (p *Plugin) ProcessFrame(frame js.Value) js.Value {
    // Get frame data
    data := frame.Get("data")
    width := frame.Get("width").Int()
    height := frame.Get("height").Int()
    
    // Process frame
    processed := p.applyEffect(data, width, height)
    
    return processed
}

func (p *Plugin) applyEffect(data js.Value, width, height int) js.Value {
    result := make([]byte, width*height*4)
    js.CopyBytesToGo(result, data)
    
    // Apply grayscale
    for i := 0; i < len(result); i += 4 {
        r := float64(result[i])
        g := float64(result[i+1])
        b := float64(result[i+2])
        
        gray := 0.299*r + 0.587*g + 0.114*b
        
        result[i] = byte(gray)
        result[i+1] = byte(gray)
        result[i+2] = byte(gray)
    }
    
    return js.Global().Get("Uint8Array").New(result)
}

func (p *Plugin) Configure(configJSON string) error {
    var config Config
    return json.Unmarshal([]byte(configJSON), &config)
}

func (p *Plugin) ID() string {
    return p.name
}

func (p *Plugin) Name() string {
    return p.name
}

func (p *Plugin) Version() string {
    return p.version
}

// Export functions to JavaScript
func main() {
    plugin := NewPlugin()
    
    c := make(chan struct{}, 0)
    
    js.Global().Set("createGoPlugin", js.FuncOf(func(this js.Value, args []js.Value) interface{} {
        return map[string]interface{}{
            "id":       plugin.ID(),
            "name":     plugin.Name(),
            "version":  plugin.Version(),
            "onLoad":   js.FuncOf(plugin.onLoad),
            "onUnload": js.FuncOf(plugin.onUnload),
        }
    }))
    
    <-c
}

func (p *Plugin) onLoad(this js.Value, args []js.Value) interface{} {
    p.OnLoad(args[0])
    return nil
}
```

### Building Go Plugin

```bash
# Set Go architecture
export GOOS=js
export GOARCH=wasm

# Build
go build -o my_go_plugin.wasm main.go

# Copy wasm_exec.js
cp $(go env GOROOT)/misc/wasm/wasm_exec.js .
```

### Loading Go Plugin

```html
<script src="wasm_exec.js"></script>
<script>
  const go = new Go();
  WebAssembly.instantiateStreaming(
    fetch('my_go_plugin.wasm'),
    go.importObject
  ).then(result => {
    go.run(result.instance);
    const plugin = createGoPlugin();
    // Use plugin...
  });
</script>
```

## Performance Optimization

### Memory Management

```rust
// Pre-allocate buffers
struct AudioProcessor {
    buffer: Vec<f32>,
    capacity: usize,
}

impl AudioProcessor {
    fn new(capacity: usize) -> Self {
        AudioProcessor {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    fn process(&mut self, input: &[f32]) -> &[f32] {
        self.buffer.clear();
        self.buffer.extend_from_slice(input);
        
        // Process in-place
        for sample in &mut self.buffer {
            *sample = *sample * 2.0;
        }
        
        &self.buffer
    }
}
```

### SIMD Instructions

```rust
#[cfg(target_arch = "wasm32")]
use std::arch::wasm32::*;

fn process_simd(input: &[f32]) -> Vec<f32> {
    let mut output = vec![0.0f32; input.len()];
    
    // Process 4 samples at a time
    for i in (0..input.len()).step_by(4) {
        unsafe {
            let v = v128_load(input.as_ptr().add(i) as *const v128);
            let result = f32x4_mul(v, f32x4_splat(2.0));
            v128_store(output.as_mut_ptr().add(i) as *mut v128, result);
        }
    }
    
    output
}
```

### Multi-threading

```rust
use rayon::prelude::*;

fn process_parallel(data: &mut [f32]) {
    data.par_chunks_mut(1024)
        .for_each(|chunk| {
            for sample in chunk {
                *sample = *sample * 2.0;
            }
        });
}
```

## Debugging WASM Plugins

### Console Logging

```rust
// In Rust
#[macro_use]
extern crate console_error_panic_hook;
use web_sys::console;

fn log(message: &str) {
    console::log_1(&message.into());
}
```

### Source Maps

```bash
# Build with source maps
wasm-pack build --dev --target web
```

### Browser DevTools

1. Open Chrome DevTools
2. Go to Sources panel
3. Find `.wasm` files
4. Set breakpoints
5. Inspect variables

## Best Practices

1. **Minimize WASM size**: Use `wasm-opt` to optimize
2. **Avoid frequent JS-Wasm calls**: Batch operations
3. **Use typed arrays**: For data transfer
4. **Pre-allocate memory**: Reduce allocations
5. **Profile performance**: Use browser tools
6. **Handle errors gracefully**: Provide fallbacks

## Deployment

```bash
# Optimize WASM size
wasm-opt -Oz my_plugin.wasm -o my_plugin_opt.wasm

# Create plugin bundle
tar czf my-plugin-v1.0.0.tar.gz \
  pkg/my_plugin_bg.wasm \
  pkg/my_plugin.js \
  manifest.json
```