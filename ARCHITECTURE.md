# 🏛️ Vantis Media Player - Technical Architecture

## Executive Summary

Vantis Media Player is built on "The Omni-System Architecture" - a comprehensive, zero-cost approach to media playback that understands content, users, and surroundings. The entire system is implemented in 100% Rust (Nightly) following these core principles:

1. **Zero-Cost Abstractions**: No runtime overhead for safety guarantees
2. **Async-First Design**: Non-blocking I/O throughout the system
3. **GPU Acceleration**: Hardware-accelerated video/audio processing
4. **Modular Architecture**: ECS-based design for extensibility
5. **Plugin Safety**: WASM sandbox for all third-party code

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Vantis Player                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Liquid      │  │   Omnibar    │  │   Controls   │      │
│  │   Glass UI   │◄─┤  Command     │◄─┤   System     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│          │                 │                 │              │
│          └─────────────────┼─────────────────┘              │
│                            │                                │
│         ┌──────────────────┼──────────────────┐             │
│         │                  │                  │             │
│  ┌──────▼──────┐   ┌──────▼──────┐   ┌──────▼──────┐       │
│  │   Video     │   │   Audio     │   │  Subtitles  │       │
│  │   Engine    │   │   Engine    │   │   (Babel)   │       │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘       │
│         │                 │                 │              │
│         └─────────────────┼─────────────────┘              │
│                           │                                │
│                    ┌──────▼──────┐                         │
│                    │   Core      │                         │
│                    │  Systems    │                         │
│                    └──────┬──────┘                         │
│                           │                                │
│         ┌─────────────────┼─────────────────┐              │
│         │                 │                 │              │
│  ┌──────▼──────┐   ┌──────▼──────┐   ┌──────▼──────┐     │
│  │  Plugins    │   │Integrations │   │   IPC       │     │
│  │  (WASM)     │   │  (TMDB,     │   │   Guard     │     │
│  │             │   │   Filmweb)  │   │             │     │
│  └─────────────┘   └─────────────┘   └─────────────┘     │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Core Subsystems

### 1. Foundation Layer

**Memory Management (`core/memory.rs`)**
- Zero-copy buffer pool (512MB default)
- Direct DMA transfers NVMe → VRAM
- Memory blocks recycled without allocation
- Thread-safe with minimal locking

**Event System (`core/events.rs`)**
- Central event bus (pub/sub pattern)
- Decoupled component communication
- Async event propagation
- Event filtering and routing

**State Management (`core/state.rs`)**
- Thread-safe player state
- Playback position tracking
- Volume and control state
- Loop and shuffle modes

### 2. Media Engine

**Video Engine (`video/`)**
- Hardware-accelerated decoding (FFmpeg)
- GPU rendering (WGPU with Vulkan/DX12/Metal)
- AI upscaling (720p → 4K real-time)
- HDR tone mapping (Reinhard/ACES/Hable)
- Motion interpolation (Optical Flow)
- Frame-perfect synchronization

**Audio Engine (`audio/`)**
- Bit-perfect exclusive mode
- Symphonia multi-format decoder
- CPAL output with low latency
- Loudness normalization (EBU R128)
- Audio effects (EQ, spatial audio)
- Ray-tracing room acoustics

**Subtitle Engine (`subtitles/`)**
- Multi-source aggregation (NapiProjekt, Napisy24, OpenSubtitles)
- Hash-based matching (NapiProjekt algorithm)
- Automatic encoding detection (CP1250, ISO-8859-2 → UTF-8)
- AI-powered synchronization
- Format support (SRT, SSA, VTT, MicroDVD)

### 3. User Interface

**Liquid Glass UI (`ui/`)**
- Frameless, borderless design
- GPU-accelerated rendering
- Iced framework for modern widgets
- Smooth animations and transitions

**Omnibar System (`ui/omnibar.rs`)**
- Universal command system (Ctrl+K)
- Command history
- Fuzzy search
- Quick actions

**Media Library (`ui/library.rs`)**
- Grid/List/Timeline views
- Search and filtering
- Thumbnail generation
- Metadata display

### 4. Plugin System

**WASM Sandbox (`plugins/`)**
- All plugins run in WebAssembly
- Isolated memory and execution
- Safe host function API
- Hot-reload support

**Host Functions (`plugins/host.rs`)**
- Logging (debug, info, warn, error)
- Media control (play, pause, seek)
- Time utilities
- Memory access

### 5. Integrations

**TMDB (`integrations/tmdb.rs`)**
- Movie/TV metadata
- Poster and backdrop images
- Cast and crew information
- Search and details

**Filmweb (`integrations/filmweb.rs`)**
- Polish ratings and reviews
- Local content support
- HTML scraping with proper parsing

**Trakt.tv (`integrations/trakt.rs`)**
- Watch history synchronization
- Mark as watched
- Sync across devices

**IPC Guard (`integrations/ipc.rs`)**
- Antivirus integration
- File and buffer scanning
- Secure communication
- Threat detection

## Data Flow

### Media Playback Flow

```
1. File Load
   ↓
2. Hash Calculation (for subtitles)
   ↓
3. Metadata Scraping (TMDB/Filmweb)
   ↓
4. Subtitle Search & Download (Babel)
   ↓
5. Audio/Video Decoding
   ↓
6. Processing (Upscaling/Tone Mapping)
   ↓
7. GPU Rendering
   ↓
8. Bit-Perfect Output
   ↓
9. User Interaction
```

### Event Flow

```
User Input → Omnibar/Controls → Event Bus → Core → Media Engines → UI Update
                                                    ↓
                                              Integrations
                                                    ↓
                                              External APIs
```

## Performance Optimizations

### Memory
- Zero-copy buffers eliminate data copies
- Pre-allocated buffer pools prevent fragmentation
- Arena allocation for short-lived objects

### CPU
- Async I/O prevents blocking
- Lock-free data structures where possible
- SIMD instructions for media processing

### GPU
- Hardware-accelerated decoding
- GPU-based upscaling and effects
- Parallel rendering pipelines

## Security Architecture

### Memory Safety
- Rust ownership model prevents memory corruption
- No garbage collector eliminates pauses
- Compile-time safety guarantees

### Plugin Isolation
- WASM sandbox isolates third-party code
- Controlled host function API
- Memory bounds enforced

### External Communication
- IPC Guard for antivirus scanning
- HTTPS for all network calls
- Rate limiting and timeouts

## Configuration System

```rust
pub struct Config {
    pub audio: AudioConfig,      // Exclusive mode, sample rate
    pub video: VideoConfig,      // Hardware acceleration, upscaling
    pub subtitles: SubtitleConfig, // Language, sources, AI sync
    pub ui: UIConfig,            // Theme, animations
    pub advanced: AdvancedConfig, // WASM sandbox, IPC guard
}
```

## Error Handling

- Result<T> for recoverable errors
- anyhow for error context
- Graceful degradation
- Comprehensive logging

## Future Extensions

The architecture supports:
- Eye tracking integration
- IoT device sync (Hue, Smartwatches)
- VR/AR rendering
- P2P party mode
- Advanced AI features

## Design Patterns

1. **ECS (Entity-Component-System)**: Modular architecture
2. **Pub/Sub (Event Bus)**: Decoupled communication
3. **Builder Pattern**: Configuration
4. **Strategy Pattern**: Tone mapping algorithms
5. **Adapter Pattern**: External integrations

## Testing Strategy

- Unit tests for core logic
- Integration tests for components
- Property-based testing for critical paths
- Fuzzing for parsers and decoders

---

**Vantis: The Last Interface**