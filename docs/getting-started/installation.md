---
sidebar_position: 2
---

# Installation Guide

This guide will help you install Vantis Media Player on your preferred platform.

## System Requirements

### Minimum Requirements

- **Operating System**: 
  - Windows 10 or later
  - macOS 10.15 (Catalina) or later
  - Linux (Ubuntu 20.04+, Debian 11+, or equivalent)
- **RAM**: 2GB minimum, 4GB recommended
- **Storage**: 500MB for installation
- **Network**: Internet connection for streaming

### Recommended Requirements

- **RAM**: 8GB or more
- **Storage**: SSD with at least 2GB free space
- **GPU**: Hardware acceleration support (OpenGL 3.0+ or DirectX 11+)
- **Network**: Broadband connection (10+ Mbps for HD streaming)

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

#### Windows

1. Download the latest installer from the [GitHub Releases](https://github.com/vantisCorp/VantisMedia/releases) page
2. Run the `.exe` installer
3. Follow the installation wizard
4. Launch Vantis Media Player from the Start Menu

```powershell
# Using winget
winget install VantisMedia

# Using Chocolatey
choco install vantismedia
```

#### macOS

1. Download the latest `.dmg` from the [GitHub Releases](https://github.com/vantisCorp/VantisMedia/releases) page
2. Open the downloaded file
3. Drag Vantis Media Player to your Applications folder
4. Launch from Applications or Launchpad

```bash
# Using Homebrew
brew install --cask vantismedia
```

#### Linux

**Ubuntu/Debian:**

```bash
# Download and install .deb package
wget https://github.com/vantisCorp/VantisMedia/releases/latest/download/vantismedia_amd64.deb
sudo dpkg -i vantismedia_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

**Fedora/RHEL:**

```bash
# Download and install .rpm package
wget https://github.com/vantisCorp/VantisMedia/releases/latest/download/vantismedia_x86_64.rpm
sudo dnf install vantismedia_x86_64.rpm
```

**Arch Linux:**

```bash
# Install from AUR
yay -S vantismedia
```

### Method 2: Docker

Vantis Media Player is available as a Docker container for headless or cloud deployments.

```bash
# Pull the latest image
docker pull vantismedia/player:latest

# Run the container
docker run -d \
  --name vantismedia \
  -p 8080:8080 \
  -v /path/to/media:/media \
  vantismedia/player:latest
```

For more Docker deployment options, see the [Deployment Guide](../deployment/docker).

### Method 3: Build from Source

For developers and custom builds, you can compile Vantis Media Player from source.

#### Prerequisites

- Rust 1.75 or later
- Node.js 18 or later
- System build tools (gcc/clang, make, etc.)

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisMedia.git
cd VantisMedia

# Install dependencies
npm install

# Build the project
npm run build

# Run the application
npm run start
```

For detailed build instructions, see the [Development Guide](../development/building).

### Method 4: Web Version

Vantis Media Player is also available as a web application:

1. Visit [https://vantismedia.app](https://vantismedia.app)
2. No installation required
3. Works in all modern browsers

## Verification

After installation, verify that Vantis Media Player is working correctly:

```bash
# Check version
vantismedia --version

# Run diagnostic tests
vantismedia --diagnostic
```

## Upgrading

### Desktop Applications

Update notifications will appear automatically when new versions are available. You can also:

- **Windows**: Use `winget upgrade VantisMedia`
- **macOS**: Use `brew upgrade --cask vantismedia`
- **Linux**: Reinstall using the package manager

### Docker

```bash
# Pull the latest image
docker pull vantismedia/player:latest

# Stop and remove old container
docker stop vantismedia
docker rm vantismedia

# Start new container with latest image
docker run -d \
  --name vantismedia \
  -p 8080:8080 \
  -v /path/to/media:/media \
  vantismedia/player:latest
```

## Troubleshooting

### Windows

**Issue**: "The application cannot be opened"  
**Solution**: Right-click the installer → Properties → Unblock → OK

**Issue**: Missing DLL files  
**Solution**: Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

### macOS

**Issue**: "App is damaged"  
**Solution**: 
```bash
xattr -cr /Applications/VantisMedia.app
```

**Issue**: Cannot open due to security settings  
**Solution**: System Preferences → Security & Privacy → Allow apps from "Vantis"

### Linux

**Issue**: Missing dependencies  
**Solution**: 
```bash
sudo apt-get install -f  # Ubuntu/Debian
sudo dnf install --allowerasing vantismedia  # Fedora
```

**Issue**: Permission denied  
**Solution**: Ensure the executable has proper permissions:
```bash
chmod +x /usr/bin/vantismedia
```

## Next Steps

Now that Vantis Media Player is installed:

1. [Quick Start Guide](./quickstart) - Get started in 5 minutes
2. [Configuration Guide](./configuration) - Customize your experience
3. [First Project](./first-project) - Build your first media player integration

## Need Help?

If you encounter any issues:

- Check the [Troubleshooting Guide](../reference/troubleshooting)
- Search [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)
- Ask a question in [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)