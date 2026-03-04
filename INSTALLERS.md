# Installers Guide

This guide explains how to install Vantis Media Player on different platforms.

## Table of Contents

- [Windows](#windows)
- [macOS](#macos)
- [Linux](#linux)
- [Docker](#docker)
- [Building from Source](#building-from-source)

## Windows

### Download Installer

1. Go to the [Releases](https://github.com/vantisCorp/VantisMedia/releases) page
2. Download the latest `vantis-player-windows-x64.zip`
3. Extract the ZIP file to your desired location

### Installation

1. Extract the ZIP file to a folder (e.g., `C:\Program Files\Vantis Media Player`)
2. Run `vantis.exe` to launch the player
3. (Optional) Create a desktop shortcut:
   - Right-click on `vantis.exe`
   - Select "Send to" → "Desktop (create shortcut)"

### System Requirements

- Windows 10 or later (64-bit)
- 4GB RAM minimum (8GB recommended)
- GPU with DirectX 12 support
- 500MB free disk space

### FFmpeg Dependencies

The Windows installer includes FFmpeg libraries. No additional installation required.

### Uninstallation

1. Delete the Vantis Media Player folder
2. Remove the desktop shortcut (if created)
3. (Optional) Delete configuration files in `%APPDATA%\vantis\`

## macOS

### Download Installer

1. Go to the [Releases](https://github.com/vantisCorp/VantisMedia/releases) page
2. Download the latest `vantis-player-macos-universal.dmg`
3. Open the DMG file

### Installation

1. Double-click the DMG file to mount it
2. Drag "Vantis Media Player" to your Applications folder
3. Eject the DMG file
4. Launch Vantis Media Player from Applications

### System Requirements

- macOS 11 (Big Sur) or later
- Intel or Apple Silicon (M1/M2/M3)
- 4GB RAM minimum (8GB recommended)
- GPU with Metal support
- 500MB free disk space

### FFmpeg Dependencies

The macOS installer includes FFmpeg libraries. No additional installation required.

### Uninstallation

1. Move "Vantis Media Player" from Applications to Trash
2. Empty Trash
3. (Optional) Delete configuration files in `~/Library/Application Support/vantis/`

### Gatekeeper

If you see a warning about the app being from an unidentified developer:

1. Right-click (or Control-click) on the app
2. Select "Open"
3. Click "Open" in the dialog

## Linux

### Download Installer

1. Go to the [Releases](https://github.com/vantisCorp/VantisMedia/releases) page
2. Download the latest `vantis-player-linux-x64.tar.gz`
3. Extract the tarball

### Installation

```bash
# Extract the tarball
tar -xzf vantis-player-linux-x64.tar.gz

# Move to /opt (optional)
sudo mv vantis-player /opt/vantis-player

# Create symlink (optional)
sudo ln -s /opt/vantis-player/vantis /usr/local/bin/vantis

# Run the player
vantis
```

### System Requirements

- Linux (x86_64)
- 4GB RAM minimum (8GB recommended)
- GPU with Vulkan support
- 500MB free disk space

### FFmpeg Dependencies

The Linux installer includes FFmpeg libraries. No additional installation required.

### Package Manager Installation

#### Ubuntu/Debian

```bash
# Add repository (when available)
sudo add-apt-repository ppa:vantismedia/vantis-player
sudo apt-get update
sudo apt-get install vantis-player
```

#### Fedora

```bash
# Add repository (when available)
sudo dnf copr enable vantismedia/vantis-player
sudo dnf install vantis-player
```

#### Arch Linux

```bash
# Install from AUR (when available)
yay -S vantis-player
```

### Uninstallation

```bash
# Remove symlink
sudo rm /usr/local/bin/vantis

# Remove installation
sudo rm -rf /opt/vantis-player

# Delete configuration files
rm -rf ~/.config/vantis/
```

## Docker

Docker is the easiest way to run Vantis Media Player without installation.

### Quick Start

```bash
# Pull the image
docker pull vantismedia/vantis-player:latest

# Run the container
docker run -d \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  -v vantis-config:/config \
  -v vantis-cache:/cache \
  -v vantis-plugins:/plugins \
  -p 8080:8080 \
  --device /dev/dri:/dev/dri \
  vantismedia/vantis-player:latest \
  play /media
```

For detailed Docker instructions, see [DOCKER.md](DOCKER.md).

## Building from Source

### Prerequisites

- Rust 1.75 or later
- Cargo
- FFmpeg development libraries
- GPU drivers (Vulkan/DirectX 12/Metal)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisMedia.git
cd VantisMedia

# Build release
cargo build --release

# Run the player
./target/release/vantis
```

For detailed build instructions, see [README.md](README.md).

## Configuration

### Configuration File Location

- **Windows**: `%APPDATA%\vantis\config.toml`
- **macOS**: `~/Library/Application Support/vantis/config.toml`
- **Linux**: `~/.config/vantis/config.toml`

### Default Configuration

```toml
[audio]
device = "default"
exclusive_mode = true
loudness_normalization = true

[video]
renderer = "auto"
hardware_acceleration = true
upscaling = "ai"

[subtitles]
auto_download = true
preferred_language = "en"
encoding = "utf-8"

[ui]
theme = "dark"
omnibar_shortcut = "Ctrl+K"
```

## Troubleshooting

### Windows

#### Application won't start

1. Check if FFmpeg is installed (included in installer)
2. Run as Administrator
3. Check Windows Event Viewer for errors
4. Disable antivirus temporarily

#### No sound

1. Check system audio settings
2. Try different audio device in configuration
3. Restart the application

### macOS

#### Application won't open

1. Check System Preferences → Security & Privacy
2. Allow the app to run if blocked by Gatekeeper
3. Check Console.app for errors

#### No sound

1. Check system audio settings
2. Try different audio device in configuration
3. Restart the application

### Linux

#### Permission denied

```bash
# Make the binary executable
chmod +x vantis
```

#### Missing libraries

```bash
# Install FFmpeg
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev
```

#### No sound

1. Check PulseAudio/PipeWire is running
2. Try different audio device in configuration
3. Check ALSA settings

## Updates

### Automatic Updates

Vantis Media Player will check for updates on startup and notify you when a new version is available.

### Manual Updates

1. Download the latest installer from [Releases](https://github.com/vantisCorp/VantisMedia/releases)
2. Install over the existing version
3. Your configuration will be preserved

## Support

For issues or questions:

- Check the [Troubleshooting](docs/troubleshooting.html) page
- Search [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)
- Create a new issue with detailed information

## License

MIT License - See [LICENSE](LICENSE) for details.