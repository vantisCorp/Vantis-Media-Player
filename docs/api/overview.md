---
sidebar_position: 1
---

# API Overview

Vantis Media Player provides comprehensive APIs for JavaScript/TypeScript, Rust, and command-line interfaces.

## APIs Available

### JavaScript/TypeScript API

For web and Node.js applications:

```javascript
import { Player } from '@vantismedia/web-sdk';

const player = new Player('#container');
await player.load('video.mp4');
player.play();
```

**Features:**
- Full browser support
- React/Vue/Angular integration
- TypeScript definitions
- Promise-based API
- Event-driven architecture

### Rust API

For desktop and system applications:

```rust
use vantismedia::Player;

let mut player = Player::new()?;
player.load("video.mp4").await?;
player.play();
```

**Features:**
- High performance
- Memory safe
- Async/await support
- Cross-platform
- Native plugins support

### CLI API

For command-line usage:

```bash
vantismedia video.mp4
vantismedia --volume 80 --fullscreen video.mp4
```

**Features:**
- Simple interface
- Batch processing
- Scripting friendly
- Configuration files
- Extensive options

## API Comparison

| Feature | JavaScript | Rust | CLI |
|---------|-----------|------|-----|
| Web Support | ✓ | ✗ | ✗ |
| Desktop Support | ✓ | ✓ | ✓ |
| Mobile Support | ✓ | ✓ | ✗ |
| Full API Access | ✓ | ✓ | Limited |
| UI Customization | ✓ | Limited | ✗ |
| Performance | Good | Excellent | N/A |
| Ease of Use | High | Medium | High |
| TypeScript | ✓ | ✗ | ✗ |

## Choosing the Right API

### Use JavaScript API When:
- Building web applications
- Using frontend frameworks (React, Vue, Angular)
- Need extensive UI customization
- Developing browser extensions
- Working with web technologies

### Use Rust API When:
- Building desktop applications
- Need maximum performance
- Require native system integration
- Developing native plugins
- Working with system-level features

### Use CLI When:
- Quick media playback
- Batch processing
- Shell scripting
- Automation
- Server environments

## Getting Started

### JavaScript API Installation

```bash
# npm
npm install @vantismedia/web-sdk

# yarn
yarn add @vantismedia/web-sdk

# pnpm
pnpm add @vantismedia/web-sdk
```

### Rust API Installation

Add to `Cargo.toml`:

```toml
[dependencies]
vantismedia = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

### CLI Installation

See [Installation Guide](../getting-started/installation).

## API Structure

### Core Components

All APIs share common concepts:

- **Player** - Main media playback engine
- **Events** - Event system for notifications
- **Plugins** - Plugin system for extensions
- **Configuration** - Settings and preferences
- **Controls** - Playback controls and UI

### Common Patterns

#### Loading Media

```javascript
// JavaScript
await player.load('video.mp4');
```

```rust
// Rust
player.load("video.mp4").await?;
```

```bash
# CLI
vantismedia video.mp4
```

#### Playing/Pausing

```javascript
// JavaScript
player.play();
player.pause();
player.stop();
```

```rust
// Rust
player.play();
player.pause();
player.stop();
```

```bash
# CLI
vantismedia --play video.mp4
vantismedia --pause
```

#### Event Handling

```javascript
// JavaScript
player.on('playing', () => {
    console.log('Now playing');
});
```

```rust
// Rust
player.on(Events::Playing, || {
    println!("Now playing");
});
```

## API Documentation Structure

This API reference is organized into sections:

- **[Player API](./player)** - Core player functionality
- **[Audio API](./audio)** - Audio-specific features
- **[Video API](./video)** - Video-specific features
- **[Subtitles API](./subtitles)** - Subtitle management
- **[Events API](./events)** - Event system
- **[Plugins API](./plugins)** - Plugin system
- **[Configuration API](./config)** - Configuration options
- **[Controls API](./controls)** - Playback controls

## Type Definitions

### JavaScript/TypeScript

Full TypeScript definitions are included:

```typescript
interface PlayerOptions {
    container: string | HTMLElement;
    theme?: 'light' | 'dark' | 'auto';
    hardwareAcceleration?: boolean;
    volume?: number;
    autoplay?: boolean;
    muted?: boolean;
}

interface MediaInfo {
    duration: number;
    width: number;
    height: number;
    codec: string;
    bitrate: number;
}

class Player {
    constructor(options: PlayerOptions);
    load(url: string, options?: LoadOptions): Promise<void>;
    play(): void;
    pause(): void;
    stop(): void;
    // ... more methods
}
```

### Rust

```rust
pub struct PlayerOptions {
    pub hardware_acceleration: bool,
    pub volume: f32,
    pub autoplay: bool,
}

pub struct MediaInfo {
    pub duration: Duration,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub bitrate: u64,
}

impl Player {
    pub fn new(options: PlayerOptions) -> Result<Self, Error>;
    pub async fn load(&mut self, url: &str) -> Result<(), Error>;
    pub fn play(&mut self);
    pub fn pause(&mut self);
    pub fn stop(&mut self);
    // ... more methods
}
```

## API Versioning

Vantis Media Player follows Semantic Versioning:

- **Major version**: Breaking changes
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

### Example: 1.2.3

- **1**: Major version
- **2**: Minor version
- **3**: Patch version

### Version Policies

- **Stable APIs**: No breaking changes without major version bump
- **Deprecated APIs**: Marked as deprecated for at least one minor version
- **Experimental APIs**: May change without notice

## API Changelog

See [Changelog](../reference/changelog) for detailed version history.

## Support

### Documentation

- [Getting Started](../getting-started/)
- [Examples](../examples/)
- [API Reference](./)

### Community

- [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/vantismedia)

### Professional Support

For enterprise support, contact us at support@vantismedia.app

## Next Steps

- **[Player API](./player)** - Core player functionality
- **[Events API](./events)** - Event system
- **[Examples](../examples/)** - Code examples

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)