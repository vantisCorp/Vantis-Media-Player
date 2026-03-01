# 🔌 Plugin Development Guide

## Introduction

Vantis Media Player supports plugins written in WebAssembly (WASM). All plugins run in an isolated sandbox for safety, ensuring that crashes or malicious code cannot affect the main application.

## Plugin Architecture

### Why WASM?

- **Security**: Plugins are isolated from the host system
- **Performance**: Near-native execution speed
- **Portability**: Plugins work across all platforms
- **Safety**: Memory bounds enforced by WASM runtime

### Host Functions

The host exposes these functions to plugins:

| Function | Description |
|----------|-------------|
| `vantis.log(ptr, len)` | Log message at default level |
| `vantis.log_info(ptr, len)` | Log info message |
| `vantis.log_warn(ptr, len)` | Log warning message |
| `vantis.log_error(ptr, len)` | Log error message |
| `vantis.play()` | Start playback |
| `vantis.pause()` | Pause playback |
| `vantis.stop()` | Stop playback |
| `vantis.seek(ms)` | Seek to position |
| `vantis.get_time()` | Get current timestamp |
| `vantis.sleep(ms)` | Sleep for milliseconds |

## Plugin Lifecycle

### Required Exports

Every plugin must export these functions:

```wat
;; Initialize plugin
(func (export "init") (result i32)
  ;; Return 0 on success, non-zero on error
)

;; Shutdown plugin
(func (export "shutdown") (result i32)
  ;; Cleanup resources
  ;; Return 0 on success
)

;; Called periodically (optional)
(func (export "tick") (result i32)
  ;; Do periodic work
  ;; Return 0 on success
)
```

### Optional Exports

```wat
;; Plugin metadata
(func (export "get_name") (result i32 i32))
(func (export "get_version") (result i32 i32))
(func (export "get_description") (result i32 i32))
```

## Creating a Plugin

### Using WAT (WebAssembly Text)

Create a file named `my_plugin.wat`:

```wat
(module
  ;; Import host functions
  (import "vantis" "log_info" (func $log_info (param i32 i32)))
  
  ;; Export memory
  (memory (export "memory") 1)
  
  ;; Data
  (data (i32.const 0) "My Plugin Initialized!")
  
  ;; Export required functions
  (func (export "init") (result i32)
    (call $log_info
      (i32.const 0)   ;; Pointer to message
      (i32.const 22)   ;; Length
    )
    (i32.const 0)     ;; Success
  )
  
  (func (export "shutdown") (result i32)
    (i32.const 0)     ;; Success
  )
  
  (func (export "tick") (result i32)
    (i32.const 0)     ;; Success
  )
)
```

### Using Rust

Create a new Rust project:

```bash
cargo new --lib my_plugin
cd my_plugin
```

Add to `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

Create `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() -> i32 {
    log_info("My Plugin Initialized!");
    0 // Success
}

#[wasm_bindgen]
pub fn shutdown() -> i32 {
    0 // Success
}

#[wasm_bindgen]
pub fn tick() -> i32 {
    0 // Success
}

fn log_info(message: &str) {
    // Call host function via FFI
    // Implementation depends on your WASM setup
}
```

## Building Plugins

### WAT Plugin

```bash
# Install wat2wasm
# From: https://github.com/WebAssembly/wabt

wat2wasm my_plugin.wat -o my_plugin.wasm
```

### Rust Plugin

```bash
# Build WASM
cargo build --release --target wasm32-unknown-unknown

# Optimize
wasm-opt -O3 target/wasm32-unknown-unknown/release/my_plugin.wasm -o my_plugin.wasm
```

## Loading Plugins

### Using CLI

```bash
vantis plugins load ./my_plugin.wasm
```

### Programmatically

```rust
use vanis_plugins::PluginManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut plugin_manager = PluginManager::new()?;
    
    plugin_manager.load_plugin("./my_plugin.wasm").await?;
    
    Ok(())
}
```

## Plugin Examples

### Subtitle Downloader Plugin

```wat
(module
  (import "vantis" "log_info" (func $log_info (param i32 i32)))
  (import "vantis" "log_error" (func $log_error (param i32 i32)))
  
  (memory (export "memory") 1)
  (data (i32.const 0) "Downloading subtitles...")
  
  (func (export "init") (result i32)
    (call $log_info (i32.const 0) (i32.const 23))
    (i32.const 0)
  )
  
  (func (export "shutdown") (result i32)
    (i32.const 0)
  )
  
  (func (export "tick") (result i32)
    (i32.const 0)
  )
)
```

### Audio Visualizer Plugin

```wat
(module
  (import "vantis" "log_info" (func $log_info (param i32 i32)))
  
  (memory (export "memory") 2)
  
  (func (export "init") (result i32)
    (i32.const 0)
  )
  
  (func (export "shutdown") (result i32)
    (i32.const 0)
  )
  
  (func (export "tick") (result i32)
    ;; Process audio data and generate visualizations
    (i32.const 0)
  )
)
```

## Best Practices

### 1. Memory Management

- Allocate memory once at initialization
- Reuse buffers when possible
- Free resources in `shutdown()`

### 2. Error Handling

- Return non-zero values on errors
- Log errors using `vantis.log_error()`
- Handle failures gracefully

### 3. Performance

- Minimize host function calls
- Batch operations when possible
- Use efficient algorithms

### 4. Security

- Don't try to access host memory
- Don't assume external resources
- Validate all inputs

## Debugging

### Enable WASM Debugging

```rust
let mut config = Config::new();
config.debug_info(true);
config.wasm_simd(true);

let engine = Engine::new(&config)?;
```

### Logging

Use host logging functions extensively:

```wat
(call $log_info (i32.const 0) (i32.const 10))  ;; "Initialized"
(call $log_warn (i32.const 10) (i32.const 12)) ;; "Warning..."
(call $log_error (i32.const 22) (i32.const 14)) ;; "Error!!!"
```

## Advanced Topics

### Shared Memory

Plugins can share memory with the host:

```rust
// Allocate shared memory
let shared_memory = Memory::new(&mut store, MemoryType::new(1, Some(1), false))?;
```

### Streaming Data

For streaming large amounts of data:

1. Use shared memory
2. Implement chunking
3. Use callbacks for progress

### Plugin Communication

Plugins can communicate with each other through the host:

```rust
// Plugin A sends message
host.send_message("plugin_b", "hello");

// Plugin B receives
let msg = host.receive_message("plugin_a");
```

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init() {
        assert_eq!(init(), 0);
    }
    
    #[test]
    fn test_shutdown() {
        assert_eq!(shutdown(), 0);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_plugin_loading() -> Result<()> {
    let mut manager = PluginManager::new()?;
    manager.load_plugin("./test_plugin.wasm").await?;
    Ok(())
}
```

## Troubleshooting

### Plugin Won't Load

- Check WASM format is valid
- Verify required exports exist
- Check memory allocation

### Crashes on Init

- Review logs for errors
- Check host function calls
- Verify memory access

### Poor Performance

- Reduce host function calls
- Optimize algorithms
- Use WASM-SIMD if available

## Resources

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [WABT Tools](https://github.com/WebAssembly/wabt)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [Wasm-Bindgen](https://rustwasm.github.io/wasm-bindgen/)

---

**Happy Plugin Development!** 🚀