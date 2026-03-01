# 🚀 Vantis Media Player - Quick Start Guide

## System Overview

Vantis Media Player is a comprehensive media player built entirely in Rust for the VantisOS ecosystem. It features zero-cost architecture, GPU acceleration, AI-powered features, and advanced subtitle support.

## Architecture Highlights

### 🏛️ Core Foundation
- **Zero-Copy Memory**: Direct DMA transfers from NVMe to VRAM
- **Async Runtime**: Tokio for non-blocking operations
- **Event Bus**: Decoupled component communication
- **State Management**: Thread-safe player state

### 🎬 Video Engine
- **Hardware Acceleration**: FFmpeg-based decoding
- **GPU Rendering**: WGPU with Vulkan/DX12/Metal support
- **AI Upscaling**: Real-time upscaling using Burn framework
- **HDR Tone Mapping**: Advanced HDR → SDR conversion

### 🔊 Audio Engine
- **Bit-Perfect Output**: Exclusive mode bypassing system mixer
- **Symphonia Decoder**: Multi-format audio support
- **Audio Effects**: Loudness normalization, EQ, spatial audio

### 📝 Vantis Babel (Subtitles)
- **Multi-Source Aggregation**: NapiProjekt, Napisy24, OpenSubtitles
- **Hash-Based Matching**: Perfect subtitle alignment
- **Encoding Detection**: Auto-convert CP1250/ISO-8859-2 to UTF-8
- **AI Synchronization**: Audio analysis for perfect timing

### 🎨 Liquid Glass UI
- **Frameless Design**: No system borders, GPU-accelerated
- **Omnibar**: Universal command system (Ctrl+K)
- **Smart Controls**: Keyboard shortcuts and mouse gestures
- **Media Library**: Browse and organize your collection

### 🔌 Plugin System
- **WASM Sandbox**: All plugins run in isolated environment
- **Host Functions**: Safe API for plugin communication
- **Hot Reload**: Load/unload plugins without restart

### 🌐 Integrations
- **TMDB**: Movie metadata, posters, cast
- **Filmweb**: Polish ratings and reviews
- **Trakt.tv**: Watch history synchronization
- **IPC Guard**: Antivirus integration for safe streaming

## Project Structure

```
vantis-player/
├── Cargo.toml              # Main workspace configuration
├── src/main.rs            # Application entry point
├── core/                  # Core systems (memory, events, state)
├── video/                 # Video processing and rendering
├── audio/                 # Audio decoding and output
├── subtitles/             # Vantis Babel subtitle system
├── ui/                    # Liquid Glass interface
├── plugins/               # WASM plugin sandbox
└── integrations/          # External service integrations
```

## Building the Project

```bash
# Build debug version
cargo build

# Build optimized release
cargo build --release

# Run the player
cargo run --bin vantis

# Run tests
cargo test
```

## Key Features by Component

### Core
- Zero-copy buffer pool for memory efficiency
- Event-driven architecture with pub/sub
- Thread-safe state management
- Comprehensive configuration system

### Video
- Hardware-accelerated decoding
- GPU-based rendering pipeline
- AI-powered upscaling (720p → 4K)
- HDR to SDR tone mapping
- Motion interpolation for smooth playback

### Audio
- Bit-perfect exclusive mode output
- Loudness normalization (EBU R128)
- Audio effects (EQ, bass/treble, spatial)
- Multi-format decoding support

### Subtitles
- Aggregation from 3+ sources
- Automatic encoding detection
- Hash-based matching algorithms
- AI-powered synchronization
- Multiple format support (SRT, SSA, VTT, SUB)

### UI
- Frameless, borderless design
- Omnibar command system
- Smart playback controls
- Media library browser
- Theme system (Dark/Light)

### Plugins
- WASM isolation for safety
- Host function API
- Plugin lifecycle management
- Hot-reload support

### Integrations
- TMDB metadata fetching
- Filmweb ratings
- Trakt.tv sync
- Antivirus scanning

## Configuration

The player uses a comprehensive configuration system with sections for:
- Audio (exclusive mode, sample rate, normalization)
- Video (hardware acceleration, upscaling, HDR)
- Subtitles (default language, sources, AI sync)
- UI (theme, animations, controls)
- Advanced (WASM sandbox, IPC guard, buffers)

## Security Features

- WASM sandbox isolates all plugins
- IPC Guard for antivirus integration
- Rust memory safety guarantees
- Zero-copy reduces attack surface

## Performance Optimizations

- Zero-copy memory transfers
- GPU-accelerated rendering
- Async I/O operations
- Hardware acceleration
- Efficient buffer management

## Future Roadmap

The architecture supports future enhancements:
- Eye tracking integration
- IoT device sync (Philips Hue, Smartwatches)
- P2P party mode
- VR/AR support
- Advanced AI features

## Development Philosophy

1. **Zero-Cost**: No unnecessary abstractions or runtime overhead
2. **Async-First**: All I/O operations are non-blocking
3. **Thread-Safe**: Proper synchronization with minimal locking
4. **Plugin-Safe**: Extensible architecture through WASM
5. **Performance First**: Optimized for modern hardware

---

**Vantis: The Last Interface** 🎬