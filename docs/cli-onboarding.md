# Vantis CLI - Quick Start Guide

> **Terminal-style onboarding for power users**

---

```ascii
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     ██╗   ██╗███████╗████████╗██╗   ██╗███╗   ██╗            ║
║     ██║   ██║██╔════╝╚══██╔══╝██║   ██║████╗  ██║            ║
║     ██║   ██║█████╗     ██║   ██║   ██║██╔██╗ ██║            ║
║     ╚██╗ ██╔╝██╔══╝     ██║   ██║   ██║██║╚██╗██║            ║
║      ╚████╔╝ ███████╗   ██║   ╚██████╔╝██║ ╚████║            ║
║       ╚═══╝  ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝            ║
║                                                              ║
║              M E D I A   P L A Y E R                        ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

---

## ⚡ Quick Start (3 Commands)

```bash
# 1. Install
curl -fsSL https://get.vantis.dev | sh

# 2. Play
vantis play /path/to/video.mp4

# 3. Enjoy! 🎬
```

---

## 📦 Installation

### Linux/macOS
```bash
# Using curl
curl -fsSL https://get.vantis.dev | sh

# Using Homebrew (macOS)
brew install vantis

# Using Snap (Linux)
snap install vantis
```

### Windows
```powershell
# Using winget
winget install vantis

# Using Chocolatey
choco install vantis

# Using PowerShell
irm https://get.vantis.dev/windows | iex
```

### From Source
```bash
git clone https://github.com/vantisCorp/Vantis-Media-Player.git
cd Vantis-Media-Player
cargo build --release
./target/release/vantis
```

---

## 🎮 Basic Commands

| Command | Description | Example |
|---------|-------------|---------|
| `play` | Play media file | `vantis play movie.mp4` |
| `pause` | Pause playback | `vantis pause` |
| `stop` | Stop playback | `vantis stop` |
| `next` | Next track | `vantis next` |
| `prev` | Previous track | `vantis prev` |
| `vol` | Set volume (0-100) | `vantis vol 75` |
| `seek` | Seek to position | `vantis seek 1:30` |
| `info` | Show media info | `vantis info video.mp4` |

---

## ⌨️ Keyboard Shortcuts

```
┌─────────────────────────────────────────────────────┐
│  SPACE    Play/Pause                                │
│  ← →      Seek -/+ 5 seconds                       │
│  ↑ ↓      Volume -/+ 10%                            │
│  F        Toggle fullscreen                         │
│  M        Toggle mute                               │
│  S        Toggle subtitles                          │
│  N        Next track                                │
│  P        Previous track                            │
│  Q        Quit                                      │
│  ?        Show help                                 │
│  /        Search                                    │
│  Cmd+K    Command palette                           │
└─────────────────────────────────────────────────────┘
```

---

## 🔧 Configuration

Config file location: `~/.config/vantis/config.toml`

```toml
# ~/.config/vantis/config.toml

[general]
language = "en"
theme = "netflix-dark"
check_updates = true

[playback]
volume = 80
remember_position = true
hw_acceleration = "auto"

[library]
paths = [
    "~/Videos",
    "~/Music"
]
auto_scan = true
scan_interval = "1h"

[subtitles]
font = "Inter"
size = 24
color = "#FFFFFF"
background = "#000000CC"

[keybindings]
play_pause = "Space"
fullscreen = "F"
# Custom bindings...
```

---

## 🎯 CLI Examples

```bash
# Play with specific subtitle
vantis play movie.mp4 --subtitles movie.srt

# Play from URL
vantis play https://example.com/video.mp4

# Play with specific audio track
vantis play movie.mkv --audio 2

# Stream from DLNA server
vantis stream dlna://192.168.1.100:8200

# Set custom keybinding
vantis config set keybindings.play_pause "K"

# List all plugins
vantis plugin list

# Install plugin from marketplace
vantis plugin install visualizer

# Export library to JSON
vantis library export --format json > library.json
```

---

## 🔍 Debug Mode

```bash
# Enable verbose logging
vantis --verbose play video.mp4

# Debug mode
vantis --debug play video.mp4

# Show system info
vantis doctor

# Benchmark playback
vantis benchmark video.mp4
```

---

## 📊 Library Commands

```bash
# Scan library
vantis library scan

# Search library
vantis library search "movie name"

# List by type
vantis library list --type video

# Get media info
vantis library info <media-id>

# Add folder to library
vantis library add ~/Videos
```

---

## 🔌 Plugin Commands

```bash
# List installed plugins
vantis plugin list

# Search marketplace
vantis plugin search "subtitle"

# Install plugin
vantis plugin install plugin-name

# Update all plugins
vantis plugin update --all

# Remove plugin
vantis plugin remove plugin-name
```

---

## 🆘 Getting Help

```bash
# General help
vantis --help

# Command help
vantis play --help

# Interactive tutorial
vantis tutorial

# Version info
vantis --version
```

---

## 🌐 Resources

| Resource | URL |
|----------|-----|
| Documentation | https://docs.vantis.dev |
| API Reference | https://api.vantis.dev |
| GitHub | https://github.com/vantisCorp/Vantis-Media-Player |
| Discord | https://discord.gg/vantis |
| Twitter | https://twitter.com/vantisplayer |

---

```ascii
            ╭───────────────────────────────────────╮
            │  Ready to rock? Start with:            │
            │                                       │
            │    $ vantis play your-video.mp4        │
            │                                       │
            │  Need help? Type:                      │
            │    $ vantis --help                     │
            ╰───────────────────────────────────────╯
```

---

*Generated with ❤️ by Vantis Team*