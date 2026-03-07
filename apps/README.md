# Vantis Media Player - Applications

This directory contains all user-facing applications that compose the Vantis Media Player ecosystem.

## Structure

```
apps/
├── cli/         # Command-line interface application
├── desktop/     # Native desktop application (Windows, macOS, Linux, FreeBSD)
├── web/         # Web-based player (WASM)
└── mobile/      # Mobile companion app (iOS, Android)
```

## Applications

### CLI (`apps/cli`)
Command-line interface for Vantis Media Player. Provides:
- Media playback control
- Playlist management
- Format conversion
- Batch processing

### Desktop (`apps/desktop`)
Native desktop application built with Rust and Iced UI framework.
- Hardware-accelerated rendering
- Native OS integration
- System tray support
- Keyboard shortcuts

### Web (`apps/web`)
WebAssembly-based player for browser environments.
- No installation required
- Progressive Web App (PWA) support
- Streaming from cloud sources

### Mobile (`apps/mobile`)
Mobile companion application for iOS and Android.
- Remote control functionality
- Media syncing
- Push notifications

## Development

```bash
# Build all apps
npm run build

# Run specific app
npm run dev --filter=cli
npm run dev --filter=desktop

# Run tests
npm run test
```

## Architecture

All apps share common packages from `../../packages/`:
- `@vantis/core` - Core functionality
- `@vantis/media` - Media processing
- `@vantis/ui` - Shared UI components
- `@vantis/plugins` - Plugin system
- `@vantis/streaming` - Network streaming
- `@vantis/utils` - Utility functions
- `@vantis/types` - TypeScript type definitions