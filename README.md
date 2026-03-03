# 🎬 Vantis Media Player

**The Omni-System Architecture for VantisOS**

An advanced media player built entirely in Rust with zero-cost abstractions, GPU acceleration, and AI-powered features. Vantis represents "The Last Interface" - one platform that understands content, users, and surroundings.

## Version

Current stable release: **v1.1.0** (March 3, 2026)

## 🌟 Features

### 🏛️ Foundation (Phase 1)
- **Zero-Cost Architecture**: Rust with guaranteed memory safety without garbage collection
- **Zero-Copy Memory**: Direct DMA transfers from NVMe to VRAM, bypassing CPU
- **Async Runtime**: Tokio for handling thousands of concurrent tasks
- **ECS Architecture**: Modular Entity-Component-System design

### 🎬 Hyper-Engine (Phase 2)
- **Cinema Grade Video**: WGPU renderer with Vulkan/DX12/Metal support
- **AI Upscaling**: Real-time 720p → 4K upscaling using Burn/Tensor cores
- **HDR Tone Mapping**: Advanced algorithms for HDR → SDR conversion
- **Motion Interpolation**: Optical Flow for perfect 60fps+ smoothness
- **Bit-Perfect Audio**: Exclusive mode bypassing system mixer
- **Ray-Tracing Audio**: Real-time room acoustics simulation
- **Loudness Normalization**: EBU R128 compliant audio level control

### 🧠 Vantis Cortex (Phase 3)
- **Multi-Source Scraping**: TMDB, Filmweb, IMDb, Trakt.tv integration
- **X-Ray Context**: AI face recognition for actor identification
- **Intelligent Matching**: Hash-based content identification
- **Vantis Babel**: Advanced subtitle system with:
  - Aggregation from NapiProjekt, Napisy24, OpenSubtitles
  - NapiProjekt hash algorithm for perfect matching
  - Automatic encoding detection (CP1250, ISO-8859-2 → UTF-8)
  - AI-powered subtitle synchronization

### 👁️ Liquid Glass Interface (Phase 4)
- **Frameless Design**: No system borders, fully GPU-accelerated
- **Omnibar**: Universal command system (Ctrl+K)
- **Smart Seek**: Timeline with thumbnails and scene detection
- **Eye Tracking**: Pause when looking away, interface highlight
- **Pie Menus**: Circular menus for instant mouse control

### 🌐 Connected (Phase 5)
- **P2P Party**: Watch together without servers (Libp2p)
- **Local Share**: Instant casting to devices on local network
- **4D Imersja**: IoT integration (Philips Hue, Smartwatch, Gamepads)

## 🏗️ Architecture

```
vantis-player/
├── Cargo.toml                 # Workspace configuration
├── cli/                       # Command-line interface
│   ├── src/
│   │   ├── lib.rs           # CLI library
│   │   └── main.rs          # CLI entry point
│   └── Cargo.toml
├── core/                      # Core systems
│   ├── src/
│   │   ├── lib.rs           # VantisCore
│   │   ├── memory.rs        # Zero-copy buffer management
│   │   ├── events.rs        # Event bus system
│   │   ├── state.rs         # Player state management
│   │   └── config.rs        # Configuration
│   └── Cargo.toml
├── video/                     # Video engine
│   ├── src/
│   │   ├── lib.rs           # VideoEngine
│   │   ├── decoder.rs       # Hardware-accelerated decoding
│   │   ├── renderer.rs      # GPU rendering
│   │   ├── upscaler.rs      # AI upscaling
│   │   ├── tonemap.rs       # HDR tone mapping
│   │   └── frame.rs         # Video frame structures
│   └── Cargo.toml
├── audio/                     # Audio engine
│   ├── src/
│   │   ├── lib.rs           # AudioEngine
│   │   ├── decoder.rs       # Audio decoding
│   │   ├── renderer.rs      # Bit-perfect output
│   │   └── effects.rs       # Audio processing
│   └── Cargo.toml
├── subtitles/                 # Vantis Babel
│   ├── src/
│   │   ├── lib.rs           # SubtitleEngine
│   │   ├── aggregator.rs    # Multi-source aggregation
│   │   ├── sources.rs       # Subtitle sources
│   │   ├── parser.rs        # Format parsing
│   │   ├── sync.rs          # AI synchronization
│   │   └── encoding.rs      # Encoding detection
│   └── Cargo.toml
├── ui/                        # Liquid Glass UI
│   ├── src/
│   │   ├── lib.rs           # UIEngine
│   │   ├── omnibar.rs       # Command system
│   │   ├── controls.rs      # Playback controls
│   │   ├── library.rs       # Media browser
│   │   └── theme.rs         # Theme system
│   └── Cargo.toml
├── plugins/                   # WASM Plugin System
│   ├── src/
│   │   ├── lib.rs           # PluginManager
│   │   └── host.rs          # Host functions
│   └── Cargo.toml
├── integrations/              # External Services
│   ├── src/
│   │   ├── lib.rs           # IntegrationManager
│   │   ├── tmdb.rs          # TMDB integration
│   │   ├── filmweb.rs       # Filmweb integration
│   │   ├── trakt.rs         # Trakt.tv integration
│   │   └── ipc.rs           # Antivirus guard
│   └── Cargo.toml
├── ai/                        # AI Features
│   ├── src/
│   │   ├── lib.rs           # AIEngine
│   │   ├── enhancement.rs   # Video enhancement
│   │   ├── scene_detection.rs # Scene detection
│   │   ├── audio_enhancement.rs # Audio enhancement
│   │   ├── subtitle_timing.rs # Subtitle timing
│   │   ├── recommendation.rs # Content recommendations
│   │   ├── models.rs        # Model management
│   │   └── utils.rs         # Utilities
│   └── Cargo.toml
├── streaming/                 # Network Streaming
│   ├── src/
│   │   ├── lib.rs           # StreamingEngine
│   │   ├── adaptive.rs      # Adaptive streaming
│   │   ├── quality.rs       # Quality selection
│   │   ├── bandwidth.rs     # Bandwidth monitoring
│   │   ├── cache.rs         # Stream caching
│   │   ├── recorder.rs      # Stream recording
│   │   ├── p2p.rs           # P2P streaming
│   │   ├── protocols.rs     # Protocol support
│   │   └── utils.rs         # Utilities
│   └── Cargo.toml
├── advanced_audio/            # Advanced Audio Features
│   ├── src/
│   │   ├── lib.rs           # AdvancedAudioEngine
│   │   ├── room_correction.rs # Room acoustic correction
│   │   ├── headphone_virtualization.rs # HRTF spatial audio
│   │   ├── fingerprinting.rs # Audio recognition
│   │   ├── visualization.rs # Audio visualization
│   │   ├── multichannel.rs  # Multi-channel processing
│   │   └── utils.rs         # Utilities
│   └── Cargo.toml
├── advanced_video/            # Advanced Video Features
│   ├── src/
│   │   ├── lib.rs           # AdvancedVideoEngine
│   │   ├── stabilization.rs # Video stabilization
│   │   ├── frame_interpolation.rs # Frame interpolation
│   │   ├── denoising.rs     # Video denoising
│   │   ├── color_grading.rs # Color grading
│   │   ├── comparison.rs    # Video comparison
│   │   └── utils.rs         # Utilities
│   └── Cargo.toml
├── advanced_ui/               # Advanced UI Features
│   ├── src/
│   │   ├── lib.rs           # AdvancedUIEngine
│   │   ├── pip.rs           # Picture-in-Picture
│   │   ├── mini_player.rs   # Mini-player mode
│   │   ├── theater_mode.rs  # Theater mode
│   │   ├── gestures.rs      # Gesture controls
│   │   └── shortcuts.rs     # Keyboard shortcuts
│   └── Cargo.toml
├── tests/                     # Integration tests
│   └── integration_tests.rs
├── benches/                   # Benchmarks
│   ├── benchmark.rs
│   └── comprehensive_benchmark.rs
└── examples/                  # Usage examples
    ├── simple_player.rs
    ├── advanced_player.rs
    ├── media_library_example.rs
    ├── video_processing_example.rs
    ├── audio_processing_example.rs
    ├── network_streaming_example.rs
    ├── custom_ui_example.rs
    ├── advanced_plugin_example.rs
    ├── ai_features_example.rs
    ├── streaming_example.rs
    ├── advanced_audio_example.rs
    ├── advanced_video_example.rs
    └── advanced_ui_example.rs
```

## 🚀 Building

### Prerequisites
- Rust 1.75+ with Nightly features
- Cargo
- System dependencies for video/audio processing

### Build Commands

```bash
# Build debug version
cargo build

# Build optimized release
cargo build --release

# Run the player
cargo run --bin vantis

# Run tests
cargo test

# Check code
cargo check
```

## 🎯 Roadmap

### ✅ Phase 1: Foundation (Complete)
- [x] Rust workspace structure
- [x] Core memory management
- [x] Event system
- [x] State management

### ✅ Phase 2: Media Engine (Complete)
- [x] Video decoder backend
- [x] Audio subsystem
- [x] GPU renderer setup

### ✅ Phase 3: Cortex & Babel (Complete)
- [x] Subtitle aggregation
- [x] Multi-source support
- [x] Encoding detection
- [x] Hash-based matching

### 🚧 Phase 4: UI & AI (In Progress)
- [ ] Liquid Glass UI framework
- [ ] Omnibar implementation
- [ ] AI upscaling integration
- [ ] X-Ray feature

### 🔮 Phase 5: Imersja & IoT (Planned)
- [ ] Bluetooth integration
- [ ] Philips Hue sync
- [ ] P2P Party mode
- [ ] Eye tracking

### 📦 Phase 6: Release (Planned)
- [ ] Cross-platform testing
- [ ] Performance optimization
- [ ] Beta release

## 🧪 Testing

The project includes comprehensive test coverage:

### Unit Tests (26 tests)
- Core: VantisCore, EventBus, PlayerState, Config
- Video: VideoEngine, VideoFrame
- Audio: AudioEngine, volume, mute
- Subtitles: SubtitleEngine, SubtitleTrack
- Plugins: PluginManager, PluginInfo
- Integrations: IntegrationManager
- Advanced UI: AdvancedUIEngine

### Integration Tests (15 tests)
- Core initialization
- All engine initializations
- Full system initialization
- Event bus integration
- Playback control integration
- Volume control integration
- Seek integration
- Speed control integration

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Run tests with output
cargo test --workspace --verbose

# Run tests for specific module
cargo test -p vantis-core
```

### CI/CD

The project uses GitHub Actions for continuous integration:
- Multi-platform builds (Linux, Windows, macOS)
- Automated testing with coverage reporting
- Performance benchmarks
- Security audits (cargo-audit, cargo-deny)
- Automatic releases

## 🔧 Configuration

Configuration is managed through `Config` struct with these sections:

- **Audio**: Exclusive mode, sample rate, loudness normalization
- **Video**: Hardware acceleration, upscaling, HDR support
- **Subtitles**: Default language, aggregation sources, AI sync
- **UI**: Theme, animations, eye tracking
- **Advanced**: WASM sandbox, IPC guard, buffer sizes

## 🔒 Security

- **WASM Sandbox**: All plugins run in isolated WebAssembly environment
- **IPC Guard**: Secure communication with antivirus system
- **Memory Safety**: Rust's ownership model prevents memory corruption
- **Zero-Copy**: Reduced attack surface for buffer overflows

## 🤝 Contributing

Contributions are welcome! The project follows these principles:

1. **Zero-Cost**: No unnecessary abstractions
2. **Async-First**: All I/O must be non-blocking
3. **Thread-Safe**: Proper synchronization
4. **Plugin-Safe**: Extensible through WASM

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

Built with love for VantisOS - The Omni-System Architecture.

Special thanks to:
- Rust community for amazing tooling
- WGPU team for modern graphics
- Symphonia for audio decoding
- Burn for ML capabilities

---

**Vantis: The Last Interface**