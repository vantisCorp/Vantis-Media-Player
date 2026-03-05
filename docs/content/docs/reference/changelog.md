---
sidebar_position: 4
title: Changelog
sidebar_label: Changelog
---

# Changelog

All notable changes to Vantis Media Player are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- AV1 codec support
- WebRTC streaming capability
- Advanced analytics dashboard
- Custom theme system
- Mobile gesture controls

### Changed
- Improved HLS performance
- Enhanced DASH parser
- Better memory management
- Updated dependencies

### Fixed
- Audio sync issues on Safari
- Buffer overflow on large videos
- Memory leak in plugin system
- CORS configuration problems

### Removed
- Legacy Flash support
- IE11 compatibility code
- Deprecated APIs

## [2.0.0] - 2024-01-15

### Added
- **BREAKING**: Complete rewrite with React 18
- Plugin architecture system
- WASM plugin support
- TypeScript migration (full type safety)
- Flux-based state management
- Component library with Tailwind CSS
- PWA support
- Service worker integration
- Offline playback capability
- Advanced debugging tools
- Performance profiler
- Memory leak detection
- Network monitoring dashboard

### Changed
- **BREAKING**: New API surface
- **BREAKING**: Event system overhaul
- Improved rendering performance
- Better mobile support
- Enhanced accessibility (WCAG 2.1 AA)
- Optimized bundle size (40% reduction)

### Fixed
- Video playback issues on low-end devices
- Audio synchronization problems
- Subtitle rendering bugs
- Fullscreen behavior on mobile
- Memory leaks in long-running sessions

### Removed
- **BREAKING**: Deprecated `play()` method (use `startPlay()`)
- **BREAKING**: Old event system
- jQuery dependency
- Legacy browser support (< Chrome 90)

## [1.5.0] - 2023-08-10

### Added
- Multi-audio track support
- Subtitle track switching
- Picture-in-Picture mode
- Keyboard shortcuts
- Touch gesture controls
- Custom control skins
- Video quality selector
- Playback speed control
- Frame-by-frame navigation

### Changed
- Improved HLS buffering algorithm
- Better DASH manifest parsing
- Enhanced error handling
- Smoother seeking performance

### Fixed
- Subtitle timing issues
- Audio muting bugs
- Volume slider problems
- Control bar visibility issues

## [1.4.0] - 2023-05-20

### Added
- DASH streaming support
- Adaptive bitrate streaming (HLS & DASH)
- Bandwidth adaptation
- Quality switching
- Network error recovery
- Automatic retry mechanism
- Buffer health monitoring

### Changed
- Improved streaming performance
- Better network condition detection
- Enhanced quality selection algorithm

### Fixed
- DASH manifest parsing errors
- Quality switching glitches
- Buffer underflow issues
- Network timeout handling

## [1.3.0] - 2023-02-15

### Added
- HLS streaming support
- m3u8 manifest parsing
- Adaptive quality levels
- Live streaming support
- DRM support (Widevine, PlayReady)
- Encrypted media playback
- AES-128 encryption

### Changed
- Enhanced video loading performance
- Better memory management for streams
- Improved error recovery

### Fixed
- HLS parsing errors
- Live stream latency issues
- DRM license acquisition problems
- Encrypted content playback failures

## [1.2.0] - 2022-11-30

### Added
- Subtitle support (SRT, VTT, ASS)
- Multiple subtitle tracks
- Subtitle styling options
- Font customization
- Text positioning controls
- Background color options

### Changed
- Improved subtitle rendering
- Better timing synchronization
- Enhanced subtitle parsing

### Fixed
- Subtitle display bugs
- Timing offset issues
- Character encoding problems
- Multiple subtitle track switching

## [1.1.0] - 2022-09-15

### Added
- Audio track switching
- Multiple audio language support
- Audio visualization
- Volume normalization
- Audio effects (equalizer)
- Volume boost option

### Changed
- Improved audio decoding performance
- Better audio synchronization
- Enhanced audio quality

### Fixed
- Audio playback issues
- Volume control problems
- Audio track switching bugs
- Equalizer performance issues

## [1.0.0] - 2022-06-01

### Added
- Initial release
- Basic video playback
- HTML5 video element support
- Custom controls
- Play/pause functionality
- Volume control
- Seek functionality
- Fullscreen mode
- Progress bar
- Time display
- Keyboard controls
- Mobile responsive design

### Supported Formats
- MP4 (H.264)
- WebM (VP8, VP9)
- Ogg (Theora)
- Audio: MP3, AAC, Ogg

### Browser Support
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## [0.9.0] - 2022-05-15

### Added
- Beta release
- Core player functionality
- Basic controls
- Event system
- API documentation

### Known Issues
- Limited browser support
- No streaming support
- Basic error handling

## [0.1.0] - 2022-03-01

### Added
- Initial development release
- Proof of concept
- Basic video playback

---

## Version Summary

| Version | Date | Major Features |
|---------|------|----------------|
| 2.0.0 | 2024-01-15 | Complete rewrite, React 18, Plugin system, TypeScript |
| 1.5.0 | 2023-08-10 | Multi-audio, Subtitles, PiP, Custom controls |
| 1.4.0 | 2023-05-20 | DASH streaming, ABR |
| 1.3.0 | 2023-02-15 | HLS streaming, DRM |
| 1.2.0 | 2022-11-30 | Subtitle support |
| 1.1.0 | 2022-09-15 | Audio tracks, Visualization |
| 1.0.0 | 2022-06-01 | Initial stable release |

## Migration Guides

### Upgrading from 1.x to 2.0

The 2.0 release includes breaking changes. See the [Getting Started Guide](../getting-started/introduction) for detailed instructions.

### Key Changes

1. **API Surface**: Complete API redesign
2. **Event System**: New event-based architecture
3. **State Management**: Flux pattern implementation
4. **TypeScript**: Full TypeScript support
5. **Plugins**: New plugin architecture

### Quick Migration

```typescript
// Old API (1.x)
const player = new VantisPlayer('#player')
player.play()

// New API (2.0)
import { createPlayer } from '@vantis/player'
const player = createPlayer({
  container: '#player',
  src: 'video.mp4'
})
player.startPlay()
```

## Future Releases

### Planned for 2.1.0
- Enhanced analytics
- Advanced advertising integration
- Improved mobile performance
- Additional codec support

### Planned for 2.2.0
- VR/360 video support
- Spatial audio
- Advanced accessibility features
- Multi-view streaming

### Planned for 3.0.0
- AI-powered features
- Advanced recommendation engine
- Social sharing integration
- Live interaction features

## Support Policy

### Maintenance Schedule

- **Current Version (2.x)**: Active development and support
- **Previous Version (1.x)**: Security fixes only
- **Older Versions**: No support

### Long-Term Support

Critical security fixes will be provided for the latest major version for at least 12 months after the next major release.

## Contributing to Changelog

When contributing to the project, please include appropriate entries in the "Unreleased" section of this file following the format:

```
### Added
- New feature description

### Changed
- Description of change

### Fixed
- Bug fix description

### Removed
- Description of removed feature
```

## References

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Releases](https://github.com/vantisCorp/Vantis-Media-Player/releases)