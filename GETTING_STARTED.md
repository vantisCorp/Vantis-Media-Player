# 🚀 Getting Started with Vantis Media Player

Welcome to Vantis Media Player! This guide will help you get up and running quickly.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Basic Usage](#basic-usage)
5. [Configuration](#configuration)
6. [Next Steps](#next-steps)

## Prerequisites

### System Requirements

- **OS**: Linux, Windows, or macOS
- **RAM**: 4GB minimum, 8GB recommended
- **GPU**: Any modern GPU with Vulkan/DX12/Metal support
- **Storage**: 500MB for installation
- **Audio**: PulseAudio, PipeWire, or WASAPI

### Required Dependencies

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
    ffmpeg \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libssl-dev \
    pkg-config

# Fedora
sudo dnf install ffmpeg-devel openssl-devel pkg-config

# Arch Linux
sudo pacman -S ffmpeg openssl pkgconf
```

#### macOS
```bash
brew install ffmpeg openssl pkg-config
```

#### Windows
- Download FFmpeg from [ffmpeg.org](https://ffmpeg.org/download.html)
- Extract to a location in your PATH
- Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

### Rust Installation

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/vantis-os/vantis-player.git
cd vantis-player

# Build release version
cargo build --release

# The binary will be at: target/release/vantis
```

### Option 2: Use Docker

```bash
# Pull the image
docker pull vantis-player:latest

# Run the player
docker run -it --rm \
    -v /path/to/videos:/media \
    --device /dev/snd \
    vantis-player:latest
```

### Option 3: Install from Release

```bash
# Download the latest release
wget https://github.com/vantis-os/vantis-player/releases/latest/download/vantis-linux-x86_64.tar.gz

# Extract
tar -xzf vantis-linux-x86_64.tar.gz

# Run
./vantis/vantis
```

## Quick Start

### Basic Playback

```bash
# Play a video file
vantis play /path/to/video.mp4

# Play starting at 30 seconds
vantis play /path/to/video.mp4 --start 30

# Play with verbose output
vantis play /path/to/video.mp4 --verbose
```

### Subtitle Management

```bash
# Download subtitles
vantis subtitles download /path/to/video.mp4

# Download in specific language
vantis subtitles download /path/to/video.mp4 --language pl

# Search for subtitles
vantis subtitles search "Inception"

# Sync subtitles
vantis subtitles sync subtitles.srt /path/to/video.mp4
```

### Plugin Management

```bash
# List installed plugins
vantis plugins list

# Load a plugin
vantis plugins load /path/to/plugin.wasm

# Unload a plugin
vantis plugins unload plugin-name
```

### Media Library

```bash
# Scan directory
vantis scan /path/to/Movies

# Scan recursively
vantis scan /path/to/Media --recursive
```

## Basic Usage

### Playing Media

#### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| → | Seek forward 5s |
| ← | Seek backward 5s |
| ↑ | Volume up |
| ↓ | Volume down |
| M | Mute/Unmute |
| F | Toggle fullscreen |
| Ctrl+K | Open Omnibar |
| Q | Quit |
| S | Toggle subtitles |

#### Command Line

```bash
# Basic playback
vantis play movie.mp4

# With options
vantis play movie.mp4 \
    --start 60 \
    --volume 0.8 \
    --fullscreen \
    --subtitles pl
```

### Managing Subtitles

#### Automatic Download

Vantis automatically downloads subtitles when playing a video:

```bash
vantis play movie.mp4
# Subtitles will be downloaded automatically if available
```

#### Manual Control

```bash
# Download specific subtitles
vantis subtitles download movie.mp4 --source napprojekt

# List available subtitle tracks
vantis play movie.mp4 --list-subtitles

# Select subtitle track
vantis play movie.mp4 --subtitle-track 2
```

#### Encoding Issues

If you see "krzacz" (garbled characters):

```bash
# Auto-fix encoding
vantis subtitles fix-encoding subtitles.srt

# Manual conversion
iconv -f CP1250 -t UTF-8 subtitles.srt > subtitles_utf8.srt
```

### Using Plugins

#### Installing Plugins

```bash
# From WASM file
vantis plugins load /path/to/plugin.wasm

# Build from source
vantis plugins build /path/to/plugin-source/
```

#### Developing Plugins

See [PLUGIN_DEVELOPMENT.md](PLUGIN_DEVELOPMENT.md) for detailed guide.

#### Plugin Examples

The project includes plugin templates:
- `examples/example_plugin.wat` - WAT template
- `examples/rust_plugin_template/` - Rust template

## Configuration

### Configuration File

Default location: `~/.vantis/config.toml`

```toml
[audio]
exclusive_mode = true
sample_rate = 48000
channels = 2
loudness_normalization = true
target_loudness = -16.0

[video]
hardware_acceleration = true
ai_upscaling = true
target_resolution = "UHD"
hdr_tone_mapping = true
motion_interpolation = true

[subtitles]
default_language = "pl"
enable_aggregation = true
auto_download = true
sources = ["napprojekt", "napisy24", "opensubtitles"]
ai_sync = true
font_size = 28

[ui]
theme = "Dark"
show_borders = false
animations = true
eye_tracking = true
pie_menus = true

[advanced]
wasm_sandbox = true
ipc_guard = true
buffer_size_mb = 512
max_threads = 8
log_level = "INFO"
```

### Generating Configuration

```bash
# Generate default configuration
vantis config --output config.toml

# Edit configuration
nano ~/.vantis/config.toml
```

### Environment Variables

```bash
# Set log level
export RUST_LOG=debug

# Set config directory
export VANTIS_CONFIG_DIR=/path/to/config

# Set cache directory
export VANTIS_CACHE_DIR=/path/to/cache
```

## Next Steps

### Learn More

- **Documentation**: Read [README.md](README.md) for full features
- **Architecture**: See [ARCHITECTURE.md](ARCHITECTURE.md) for technical details
- **API Reference**: Check [API_REFERENCE.md](API_REFERENCE.md) for API docs
- **Examples**: Look in `examples/` directory for code examples

### Contribute

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

### Get Help

- **Documentation**: Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **Issues**: Open an issue on GitHub
- **Discussions**: Join our community discussions

### Explore Features

- **Vantis Babel**: Advanced subtitle system
- **AI Features**: Upscaling and subtitle sync
- **Plugins**: Extend functionality with WASM
- **Integrations**: TMDB, Filmweb, Trakt.tv

## Tips & Tricks

### Performance Optimization

1. **Enable Hardware Acceleration**
   ```toml
   [video]
   hardware_acceleration = true
   ```

2. **Use Exclusive Audio Mode**
   ```toml
   [audio]
   exclusive_mode = true
   ```

3. **Adjust Buffer Size**
   ```toml
   [advanced]
   buffer_size_mb = 1024
   ```

### Troubleshooting

#### No Audio
```bash
# Check audio devices
vantis --list-audio-devices

# Disable exclusive mode
vantis play video.mp4 --no-exclusive-audio
```

#### No Video
```bash
# Check GPU support
vantis --check-gpu

# Try software rendering
vantis play video.mp4 --software-rendering
```

#### Subtitle Issues
```bash
# Force subtitle reload
vantis play video.mp4 --reload-subtitles

# Manual subtitle file
vantis play video.mp4 --subtitles-file subs.srt
```

### Advanced Usage

#### Playlist
```bash
# Create playlist
echo -e "movie1.mp4\nmovie2.mp4\nmovie3.mp4" > playlist.txt

# Play playlist
vantis play --playlist playlist.txt
```

#### Streaming
```bash
# Stream from URL
vantis play https://example.com/video.mp4

# Stream with custom headers
vantis play https://example.com/video.mp4 --headers "Authorization: Bearer token"
```

#### Screencast
```bash
# Capture screen
vantis play screen://0

# Capture window
vantis play window://12345
```

---

**Ready to go!** Start using Vantis Media Player and enjoy your media! 🎬

For more information, check out the [full documentation](README.md).