# ❓ Frequently Asked Questions

This FAQ addresses common questions about Vantis Media Player.

## Table of Contents

1. [General Questions](#general-questions)
2. [Installation & Setup](#installation--setup)
3. [Usage](#usage)
4. [Performance](#performance)
5. [Subtitles](#subtitle-questions)
6. [Plugins](#plugins)
7. [Troubleshooting](#troubleshooting)
8. [Development](#development)

## General Questions

### What is Vantis Media Player?

Vantis Media Player is an advanced, open-source media player built with Rust, featuring GPU acceleration, AI-powered features, and a WASM sandbox plugin system. It's designed for VantisOS but works on Linux, Windows, and macOS.

### Is Vantis Media Player free?

Yes! Vantis Media Player is completely free and open-source, licensed under the MIT License.

### What formats are supported?

**Video**: MP4, MKV, AVI, WebM, MOV, FLV, and more
**Audio**: MP3, FLAC, OGG, AAC, WAV, and more
**Subtitles**: SRT, SSA/ASS, VTT, MicroDVD

### What are the system requirements?

**Minimum**:
- 2-core CPU, 2.0GHz
- 4GB RAM
- Intel HD 4000+ GPU
- 500MB storage

**Recommended**:
- 4+ core CPU, 3.0GHz+
- 8GB RAM
- Dedicated GPU
- SSD storage

See [PERFORMANCE_GUIDE.md](PERFORMANCE_GUIDE.md) for detailed requirements.

### Does Vantis Media Player collect data?

No. Vantis Media Player does not collect any user data. Optional crash reports and performance metrics can be enabled in settings. See [SECURITY.md](SECURITY.md) for details.

## Installation & Setup

### How do I install Vantis Media Player?

**From Source**:
```bash
git clone https://github.com/vantis-os/vantis-player.git
cd vantis-player
cargo build --release
```

**From Release**:
Download the latest release from GitHub and extract.

**With Docker**:
```bash
docker pull vantis-player:latest
```

See [GETTING_STARTED.md](GETTING_STARTED.md) for detailed installation instructions.

### I'm getting build errors. What should I do?

1. **Check Rust version**:
   ```bash
   rustc --version
   ```
   You need Rust 1.93.0 or later (Stable recommended).

2. **Install dependencies**:
   ```bash
   # Linux
   sudo apt-get install ffmpeg libssl-dev pkg-config
   
   # macOS
   brew install ffmpeg openssl pkg-config
   ```

3. **Clear cache**:
   ```bash
   cargo clean
   cargo build
   ```

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more solutions.

### Can I run Vantis Media Player without a GPU?

Yes, software rendering is available, but performance will be lower. Hardware acceleration is highly recommended.

### Does Vantis Media Player work on ARM processors?

Yes! Vantis Media Player supports ARM processors, including:
- Apple Silicon (M1/M2)
- Raspberry Pi (4+)
- ARM64 Linux systems

## Usage

### How do I play a video?

```bash
# Basic playback
vantis play video.mp4

# With options
vantis play video.mp4 --fullscreen --volume 0.8
```

### How do I use keyboard shortcuts?

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| → | Seek forward 5s |
| ← | Seek backward 5s |
| ↑ | Volume up |
| ↓ | Volume down |
| M | Mute/Unmute |
| F | Toggle fullscreen |
| S | Toggle subtitles |
| Q | Quit |

### How do I use the Omnibar?

Press `Ctrl+K` to open the Omnibar. You can:
- Search for files
- Execute commands
- Open settings
- View keyboard shortcuts

### Can I create playlists?

Yes! Create a text file with video paths (one per line):

```bash
# Create playlist
echo -e "movie1.mp4\nmovie2.mp4\nmovie3.mp4" > playlist.txt

# Play playlist
vantis play --playlist playlist.txt
```

### Does Vantis Media Player support streaming?

Yes! You can stream from URLs:

```bash
vantis play https://example.com/video.mp4
```

### Can I use Vantis Media Player from another application?

Yes! Vantis Media Player provides both CLI and library APIs. See [API_REFERENCE.md](API_REFERENCE.md) for documentation.

## Performance

### Why is my video lagging?

Try these solutions:

1. **Enable hardware acceleration**:
   ```toml
   [video]
   hardware_acceleration = true
   ```

2. **Lower resolution**:
   ```bash
   vantis play video.mp4 --resolution 720
   ```

3. **Disable AI features**:
   ```toml
   [video]
   ai_upscaling = false
   motion_interpolation = false
   ```

4. **Check GPU drivers**: Update to the latest version

See [PERFORMANCE_GUIDE.md](PERFORMANCE_GUIDE.md) for more tips.

### What's the recommended configuration for 4K?

```toml
[video]
hardware_acceleration = true
ai_upscaling = true
target_resolution = "UHD"

[advanced]
buffer_size_mb = 1024
max_threads = 8
```

You'll need a GPU with at least 4GB VRAM and an SSD.

### How do I reduce CPU usage?

1. Enable hardware acceleration
2. Use appropriate resolution
3. Disable AI features
4. Increase buffer size
5. Close background applications

### How do I check performance metrics?

```bash
# Enable performance logging
vantis play video.mp4 --verbose

# Check system resources
top
htop

# Monitor GPU
nvidia-smi  # NVIDIA
radeontop   # AMD
intel_gpu_top  # Intel
```

## Subtitle Questions

### How do I download subtitles?

**Automatic**:
```bash
vantis play video.mp4
# Subtitles download automatically
```

**Manual**:
```bash
vantis subtitles download video.mp4 --language pl
```

### Why are my subtitles showing "krzacz" (garbled characters)?

This is an encoding issue. Vantis auto-detects Polish encodings (CP1250, ISO-8859-2).

**Fix manually**:
```bash
vantis subtitles fix-encoding subtitles.srt

# Or with iconv
iconv -f CP1250 -t UTF-8 subtitles.srt > subtitles_utf8.srt
```

### Which subtitle services are supported?

- **NapiProjekt** (Polish, hash-based perfect matching)
- **Napisy24** (Polish)
- **OpenSubtitles** (International)
- **Manual files**: SRT, SSA/ASS, VTT, MicroDVD

### How does AI subtitle sync work?

Vantis analyzes the audio track and adjusts subtitle timing automatically:

```toml
[subtitles]
ai_sync = true
```

This is useful for:
- Poorly synced subtitles
- Different video cuts
- PAL/NTSC speed differences

### Can I use custom subtitle fonts?

Yes! Configure in your config file:

```toml
[subtitles]
font_family = "Arial"
font_size = 28
font_color = "#FFFFFF"
background_color = "#000000"
border_width = 2
```

## Plugins

### What are plugins?

Plugins are WASM modules that extend Vantis Media Player functionality. They run in a sandboxed environment for security.

### Where can I find plugins?

The official plugin repository is coming soon. For now, you can create your own plugins using the templates in the `examples/` directory.

See [PLUGIN_DEVELOPMENT.md](PLUGIN_DEVELOPMENT.md) for development guides.

### How do I install a plugin?

```bash
# Load a plugin
vantis plugins load /path/to/plugin.wasm

# List installed plugins
vantis plugins list

# Unload a plugin
vantis plugins unload plugin-name
```

### Are plugins safe?

Yes! All plugins run in a WASM sandbox with:
- Memory isolation
- API restrictions
- Resource limits
- No direct system access

See [SECURITY.md](SECURITY.md) for security details.

### Can I write plugins in languages other than Rust?

Yes! Any language that compiles to WASM is supported:
- Rust
- C/C++
- AssemblyScript
- Go
- Python (via PyScript)

## Troubleshooting

### No audio output

**Check audio device**:
```bash
vantis --list-audio-devices
```

**Disable exclusive mode**:
```bash
vantis play video.mp4 --no-exclusive-audio
```

**Check system audio**:
- Verify system audio works
- Check volume levels
- Try different output device

### No video output

**Check GPU support**:
```bash
vantis --check-gpu
```

**Try software rendering**:
```bash
vantis play video.mp4 --software-rendering
```

**Update GPU drivers**:
- NVIDIA: Download from nvidia.com
- AMD: Download from amd.com
- Intel: Use OS update

### Application crashes

**Enable debug logging**:
```bash
RUST_LOG=debug vantis play video.mp4 > debug.log 2>&1
```

**Report the issue**:
Include:
- Debug log file
- System information
- Steps to reproduce

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more solutions.

### Subtitles don't download

**Check internet connection**:
```bash
ping nappiprojekt.pl
```

**Try different source**:
```bash
vantis subtitles download video.mp4 --source opensubtitles
```

**Manual download**:
1. Download subtitle file manually
2. Load it:
   ```bash
   vantis play video.mp4 --subtitles-file subs.srt
   ```

### Plugin doesn't load

**Check WASM support**:
```bash
vantis --check-wasm
```

**Verify plugin signature**:
```bash
vantis plugins verify /path/to/plugin.wasm
```

**Check plugin manifest**:
Ensure `plugin.toml` is valid.

## Development

### How do I contribute?

1. Fork the repository
2. Create a feature branch
3. Make changes
4. Run tests
5. Submit a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### What's the architecture?

Vantis Media Player uses a modular architecture:
- **vantis-core**: Core systems and utilities
- **vantis-video**: Video decoding and rendering
- **vantis-audio**: Audio processing and output
- **vantis-subtitles**: Subtitle system (Vantis Babel)
- **vantis-ui**: User interface (Liquid Glass)
- **vantis-plugins**: Plugin system

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

### Can I use Vantis Media Player as a library?

Yes! Add to your `Cargo.toml`:

```toml
[dependencies]
vantis-core = "0.1.0"
vantis-video = "0.1.0"
vantis-audio = "0.1.0"
```

See [API_REFERENCE.md](API_REFERENCE.md] for API documentation.

### Is WASM output supported?

Yes! You can build Vantis Media Player for WASM:

```bash
cargo build --target wasm32-unknown-unknown
```

## Additional Resources

### Documentation

- [README.md](README.md) - Project overview
- [GETTING_STARTED.md](GETTING_STARTED.md) - Quick start guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture
- [API_REFERENCE.md](API_REFERENCE.md) - API documentation
- [PLUGIN_DEVELOPMENT.md](PLUGIN_DEVELOPMENT.md) - Plugin development
- [PERFORMANCE_GUIDE.md](PERFORMANCE_GUIDE.md) - Performance optimization
- [SECURITY.md](SECURITY.md) - Security information
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - Troubleshooting guide

### Community

- **GitHub**: https://github.com/vantis-os/vantis-player
- **Discussions**: https://github.com/vantis-os/vantis-player/discussions
- **Issues**: https://github.com/vantis-os/vantis-player/issues

### Support

- **Email**: support@vantis-os.org
- **Security**: security@vantis-os.org
- **Contributions**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

Still have questions? Open an issue on GitHub or join our community discussions!