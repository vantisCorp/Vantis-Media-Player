# Build Scripts

This directory contains scripts for building Vantis Media Player installers.

## Scripts

### build-installers.sh

Builds installers for Linux, macOS, and Windows.

**Usage:**

```bash
# Build for current platform
./build-installers.sh

# Build for specific platform
./build-installers.sh linux
./build-installers.sh macos
./build-installers.sh windows

# Build for all platforms
./build-installers.sh all
```

**Requirements:**

- Linux/macOS: Bash, standard build tools
- Windows cross-compilation: mingw-w64
- macOS: Xcode Command Line Tools

### build-installers.ps1

Builds installers for Windows.

**Usage:**

```powershell
# Build with default version
.\build-installers.ps1

# Build with custom version
.\build-installers.ps1 -Version "1.0.0"

# Build with custom output directory
.\build-installers.ps1 -OutputDir "dist"
```

**Requirements:**

- PowerShell 5.1 or later
- Rust and Cargo
- Git

## Output

Installers are created in the `installers/` directory:

- `vantis-player-linux-x64-VERSION.tar.gz` - Linux tarball
- `vantis-player-macos-universal-VERSION.dmg` - macOS DMG
- `vantis-player-windows-x64-VERSION.zip` - Windows ZIP

## Automated Builds

Installers are also built automatically by GitHub Actions on:

- Push to main branch
- Creating a new tag (e.g., `v1.0.0`)
- Manual workflow dispatch

See [`.github/workflows/build-installers.yml`](../.github/workflows/build-installers.yml) for details.

## Troubleshooting

### Linux

**Missing cross-compilation tools:**

```bash
sudo apt-get install mingw-w64
```

**Missing FFmpeg:**

```bash
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev
```

### macOS

**Missing Xcode Command Line Tools:**

```bash
xcode-select --install
```

**Missing FFmpeg:**

```bash
brew install ffmpeg
```

### Windows

**Missing Rust:**

Download and install from [rustup.rs](https://rustup.rs/)

**Missing FFmpeg:**

Download and install from [ffmpeg.org](https://ffmpeg.org/download.html)

## Support

For issues or questions:

- Check the main [README.md](../README.md)
- See [INSTALLERS.md](../INSTALLERS.md)
- Search [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)