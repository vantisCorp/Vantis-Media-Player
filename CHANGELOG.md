# Changelog

All notable changes to Vantis Media Player will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and architecture
- Core systems (event bus, state management, configuration)
- Video engine with hardware acceleration
- Audio engine with bit-perfect output
- Vantis Babel subtitle system
- Liquid Glass UI framework
- WASM plugin system
- CLI interface
- Comprehensive documentation suite
- Example code and templates
- CI/CD pipeline
- Docker support

### Changed
- Updated Rust version requirement from 1.93.0 to 1.75.0 across all configuration files
- Simplified CI/CD workflows for improved reliability and maintainability
- Standardized all workflows to use stable Rust toolchain
- Added proper caching configuration for faster builds
- Fixed YAML linting issues in workflow files

### Fixed
- Updated Cargo.toml with correct Rust version (1.75.0)
- Fixed Dockerfile to use correct base image (rust:1.75-slim)
- Corrected Dockerfile binary name from "vantis" to "vantis-player"
- Removed invalid rust-version specifications from all workflows
- Fixed trailing whitespace and formatting issues in workflow files

### Documentation
- Added CI_FIXES_SUMMARY.md documenting all CI/CD improvements
- Added GITHUB_ACTIONS_DIAGNOSTICS.md with comprehensive diagnostic information
- Updated all documentation files with correct Rust version references (1.75+)
- Added critical_finding_summary.md documenting GitHub Actions investigation

## [1.0.0] - 2024-01-15

### Added
- Initial release of Vantis Media Player
- Complete modular architecture
- Video playback with FFmpeg and WGPU
- Audio playback with Symphonia and CPAL
- Subtitle system (Vantis Babel)
- WASM plugin system with Wasmtime
- Liquid Glass UI framework
- CLI interface with clap
- Configuration system
- Documentation:
  - README.md
  - QUICKSTART.md
  - ARCHITECTURE.md
  - API_REFERENCE.md
  - PLUGIN_DEVELOPMENT.md
  - TROUBLESHOOTING.md
  - USAGE_EXAMPLES.md
  - GETTING_STARTED.md
  - PERFORMANCE_GUIDE.md
  - SECURITY.md
  - CONTRIBUTING.md
  - FAQ.md
  - ROADMAP.md
- Examples:
  - simple_player.rs
  - advanced_player.rs
  - keyboard_shortcuts_example.rs
  - subtitle_management_example.rs
  - media_library_example.rs
  - wat_plugin_template.wat
  - rust_plugin_template/
- CI/CD with GitHub Actions
- Pre-commit hooks
- Docker support
- Integration with TMDB, Filmweb, Trakt.tv

## [0.9.0] - 2024-01-01

### Added
- Alpha release
- Basic video playback
- Basic audio playback
- Subtitle loading
- Simple UI

## [0.8.0] - 2023-12-15

### Added
- Beta release
- Plugin system prototype
- Basic CLI
- Configuration system

## [0.7.0] - 2023-12-01

### Added
- Development preview
- Core architecture
- Event system
- State management

---

## [Unreleased] - Future v1.1.0

### Planned
- Advanced subtitle editor (GUI)
- Real-time subtitle translation (AI)
- Subtitle style presets
- Enhanced Polish encoding detection
- Batch subtitle processing
- Subtitle sharing platform

### Performance
- Improved AI sync accuracy
- Faster subtitle parsing
- Memory optimization for large subtitle files

---

## [Unreleased] - Future v1.2.0

### Planned
- Scene detection and chaptering
- Content-aware thumbnails
- Intelligent video enhancement
- Audio normalization with machine learning
- Face detection for privacy mode
- Object detection in videos

### Technical
- ONNX runtime integration
- GPU-accelerated ML inference
- Model caching and updates
- Plugin ML API

---

## [Unreleased] - Future v1.3.0

### Planned
- DLNA/UPnP support
- Chromecast support
- AirPlay 2 support
- IPTV support (M3U playlists)
- Live streaming
- Network discovery

### Performance
- Adaptive bitrate streaming
- Low-latency streaming protocols
- Network caching
- Bandwidth optimization

---

## [Unreleased] - Future v2.0.0

### Planned
- Completely redesigned Liquid Glass UI
- 3D video support (VR/AR)
- 360° video playback
- Advanced audio visualization
- Media library management
- Watch history and resume
- Custom themes and skins

### Technical
- New rendering engine
- Improved GPU utilization
- Better memory management
- Plugin UI API
- Custom shader support

---

## Version History

### Version 1.0.0 (Current)
**Release Date**: January 15, 2024
**Status**: Stable
**Highlights**:
- Complete feature set
- Production-ready
- Comprehensive documentation
- Full plugin system

### Version 0.9.0 (Alpha)
**Release Date**: January 1, 2024
**Status**: Alpha
**Highlights**:
- Basic functionality
- Testing phase

### Version 0.8.0 (Beta)
**Release Date**: December 15, 2023
**Status**: Beta
**Highlights**:
- Plugin prototype
- CLI interface

### Version 0.7.0 (Development)
**Release Date**: December 1, 2023
**Status**: Development Preview
**Highlights**:
- Core architecture
- Initial systems

---

## Breaking Changes

### v1.0.0
- Changed plugin API to use WIT (Wasm Interface Types)
- Renamed `VantisPlayer` to `Player`
- Updated configuration file format (v2)

### v0.9.0
- Migrated from tokio to async-std (reverted in v1.0.0)
- Changed subtitle encoding detection algorithm

---

## Migration Guides

### From v0.9.x to v1.0.0

#### Configuration Migration
Old configuration (v1):
```toml
[video]
hardware = true
```

New configuration (v2):
```toml
[video]
hardware_acceleration = true
ai_upscaling = true
```

Run migration tool:
```bash
vantis config migrate
```

#### Plugin Migration
Old plugin API:
```rust
extern "C" {
    fn vantis_init();
}
```

New plugin API (WIT):
```wit
vantis-init: func()
```

See [PLUGIN_DEVELOPMENT.md](PLUGIN_DEVELOPMENT.md) for migration guide.

---

## Categories

### Added
New features, functionality, or capabilities

### Changed
Changes to existing functionality (not breaking)

### Deprecated
Features that will be removed in future releases

### Removed
Features removed from the project

### Fixed
Bug fixes and corrections

### Security
Security-related changes

### Performance
Performance improvements and optimizations

### Documentation
Documentation updates and additions

### Testing
Testing improvements and additions

### CI/CD
CI/CD pipeline changes and improvements

---

## Release Notes Format

### Header
```markdown
## [Version] - YYYY-MM-DD
```

### Sections
```markdown
### Added
- Description

### Changed
- Description

### Deprecated
- Description

### Removed
- Description

### Fixed
- Description

### Security
- Description

### Performance
- Description
```

---

## Contributors

Thank you to all contributors who have helped make Vantis Media Player possible!

- **Vantis Team** - Core development
- **Community Contributors** - Bug reports, features, documentation

---

## Links

- **Repository**: https://github.com/vantis-os/vantis-player
- **Issues**: https://github.com/vantis-os/vantis-player/issues
- **Discussions**: https://github.com/vantis-os/vantis-player/discussions
- **Documentation**: https://docs.vantis-os.org

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This changelog follows the [Keep a Changelog](https://keepachangelog.com/) format and versioning adheres to [Semantic Versioning](https://semver.org/).