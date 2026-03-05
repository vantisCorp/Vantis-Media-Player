---
sidebar_position: 3
---

# Quick Start Guide

Get up and running with Vantis Media Player in just 5 minutes!

## Prerequisites

Before you begin, ensure you have:

- Vantis Media Player installed ([Installation Guide](./installation))
- A media file to test with (audio or video)
- Basic familiarity with command line (optional)

## Running Vantis Media Player

### Desktop Application

Simply launch Vantis Media Player from your applications menu. The player interface will open, allowing you to:

1. Click **Open File** or press `Ctrl+O` (Cmd+O on macOS)
2. Select your media file
3. Click **Play** to start playback

### Command Line

For quick playback from the terminal:

```bash
# Play a video file
vantismedia /path/to/video.mp4

# Play an audio file
vantismedia /path/to/audio.mp3

# Play from URL
vantismedia https://example.com/stream.m3u8

# Open with specific options
vantismedia --fullscreen /path/to/video.mp4
```

### Web Interface

If you're using the Docker container or web version:

1. Open your browser to `http://localhost:8080`
2. Drag and drop a media file onto the interface
3. Or click the folder icon to browse for files

## Basic Controls

### Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|--------------|-------|
| Play/Pause | `Space` | `Space` |
| Stop | `S` | `S` |
| Volume Up | `↑` | `↑` |
| Volume Down | `↓` | `↓` |
| Mute | `M` | `M` |
| Fullscreen | `F` | `F` |
| Seek Forward | `→` | `→` |
| Seek Backward | `←` | `←` |
| Next Track | `Ctrl+→` | `Cmd+→` |
| Previous Track | `Ctrl+←` | `Cmd+←` |
| Speed Up | `+` | `+` |
| Speed Down | `-` | `-` |
| Subtitles | `V` | `V` |

### Mouse Controls

- **Click**: Play/Pause
- **Double-click**: Toggle fullscreen
- **Right-click**: Context menu
- **Scroll**: Adjust volume
- **Drag progress bar**: Seek

## Loading Media

### From File

```bash
# Single file
vantismedia /path/to/media.mp4

# Multiple files
vantismedia file1.mp4 file2.mp3 file3.avi

# Directory
vantismedia /path/to/media/folder/
```

### From URL

Vantis Media Player supports various streaming protocols:

```bash
# HTTP/HTTPS
vantismedia https://example.com/video.mp4

# HLS
vantismedia https://example.com/stream.m3u8

# DASH
vantismedia https://example.com/stream.mpd

# RTMP
vantismedia rtmp://example.com/live/stream
```

### Using Playlists

Create a playlist file (M3U format):

```m3u
#EXTM3U
#EXTINF:180,Artist - Song Title
/path/to/song1.mp3
#EXTINF:240,Another Artist - Another Song
/path/to/song2.mp3
/path/to/video.mp4
```

Load the playlist:

```bash
vantismedia /path/to/playlist.m3u
```

## Basic Configuration

Create a configuration file at `~/.vantismedia/config.toml`:

```toml
[general]
theme = "dark"
language = "en"
volume = 80

[player]
auto_play = true
hardware_acceleration = true
remember_position = true

[subtitle]
font = "Roboto"
size = 24
language = "auto"
```

For more configuration options, see the [Configuration Guide](./configuration).

## Using Plugins

Vantis Media Player supports plugins to extend functionality:

### Installing a Plugin

```bash
# List available plugins
vantismedia plugin list

# Install a plugin from the registry
vantismedia plugin install visualizer

# Install from file
vantismedia plugin install /path/to/plugin.wasm
```

### Managing Plugins

```bash
# List installed plugins
vantismedia plugin list --installed

# Enable a plugin
vantismedia plugin enable visualizer

# Disable a plugin
vantismedia plugin disable visualizer

# Update a plugin
vantismedia plugin update visualizer

# Remove a plugin
vantismedia plugin remove visualizer
```

## Example: Playing a Video with Subtitles

```bash
# Play video with external subtitles
vantismedia video.mp4 --subtitles subtitles.srt

# Specify subtitle language
vantismedia video.mkv --subtitle-lang en

# Load subtitles from folder automatically
vantismedia video.mp4 --auto-subs
```

## Example: Streaming

```bash
# Stream from webcam (Linux)
vantismedia v4l2:///dev/video0

# Stream from network
vantismedia rtsp://camera.example.com/stream

# Play with low latency
vantismedia --low-latency stream.m3u8
```

## Quick API Example

For developers, here's how to use Vantis Media Player in your application:

```javascript
// JavaScript/TypeScript
import { Player } from '@vantismedia/web-sdk';

const player = new Player('#container');
await player.load('https://example.com/video.mp4');
player.play();

// Control playback
player.pause();
player.seek(60); // Seek to 60 seconds
player.setVolume(0.8); // 80% volume
```

```rust
// Rust
use vantismedia::Player;

let player = Player::new()?;
player.load("https://example.com/video.mp4")?;
player.play();

// Control playback
player.pause();
player.seek(Duration::from_secs(60));
player.set_volume(0.8);
```

## Next Steps

Now that you're up and running:

1. **[Configuration Guide](./configuration)** - Customize every aspect of Vantis
2. **[Core Features](../core-features/)** - Learn about all available features
3. **[API Reference](../api/)** - Dive into the programming interface
4. **[Plugin Development](../plugins/)** - Create your own extensions

## Common Issues

### Video doesn't play

Check that hardware acceleration is enabled:
```bash
vantismedia --check-hwaccel
```

### No audio

Verify audio output:
```bash
vantismedia --list-audio-devices
```

### Subtitles not showing

Make sure subtitles are correctly formatted (SRT, VTT, or ASS format).

## Getting Help

Need more help? Check out:

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)