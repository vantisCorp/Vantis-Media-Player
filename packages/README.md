# Vantis Media Player - Packages

This directory contains shared packages/libraries used across all Vantis Media Player applications.

## Structure

```
packages/
├── core/       # Core functionality and primitives
├── media/      # Media processing (audio, video, subtitles)
├── ui/         # Shared UI components
├── plugins/    # Plugin system and SDK
├── streaming/  # Network streaming protocols
├── utils/      # Utility functions and helpers
└── types/      # TypeScript type definitions
```

## Packages

### `@vantis/core`
Core functionality shared across all applications:
- Configuration management
- Event system
- State management primitives
- Error handling
- Logging infrastructure

### `@vantis/media`
Media processing capabilities:
- Audio decoding/encoding (Symphonia, FFmpeg)
- Video decoding/encoding
- Subtitle parsing and rendering
- Hardware acceleration abstraction
- DSP operations (FFT, filters)

### `@vantis/ui`
Shared UI components:
- Design system tokens
- Component library
- Theme system (light/dark)
- Accessibility utilities

### `@vantis/plugins`
Plugin system:
- Plugin host implementation
- Plugin SDK for third-party developers
- Plugin marketplace client
- Sandbox security (WASM-based)

### `@vantis/streaming`
Network streaming protocols:
- HLS (HTTP Live Streaming)
- DASH (Dynamic Adaptive Streaming)
- RTSP (Real-Time Streaming Protocol)
- P2P streaming (libp2p)

### `@vantis/utils`
Utility functions:
- File format detection
- Path manipulation
- String utilities
- Async helpers

### `@vantis/types`
TypeScript type definitions:
- Media formats
- Plugin interfaces
- API contracts
- Configuration types

## Development

```bash
# Build all packages
npm run build

# Build specific package
npm run build --filter=@vantis/core

# Run tests
npm run test

# Publish (requires permissions)
npm run publish
```

## Rust Crates

Each package corresponds to a Rust crate in the workspace:

| Package | Rust Crate |
|---------|------------|
| `@vantis/core` | `vantis-core` |
| `@vantis/media` | `vantis-audio`, `vantis-video`, `vantis-subtitles` |
| `@vantis/ui` | `vantis-ui` |
| `@vantis/plugins` | `vantis-plugins` |
| `@vantis/streaming` | `vantis-streaming` |