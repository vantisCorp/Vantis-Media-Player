# Vantis Media Player - Mobile Apps

Mobile applications for iOS and Android platforms.

## Architecture

This directory contains the mobile application implementations:

```
mobile/
├── ios/           # iOS app (Swift/SwiftUI)
├── android/       # Android app (Kotlin/Jetpack Compose)
├── shared/        # Shared Rust core library (FFI)
└── README.md
```

## Shared Core

Both mobile apps share a common Rust core library via FFI (Foreign Function Interface):

- **Video decoding** - Hardware-accelerated via platform APIs
- **Audio processing** - Bit-perfect output with effects
- **Subtitle rendering** - Vantis Babel engine
- **Streaming** - Adaptive bitrate with caching
- **Cloud sync** - Settings and playlist synchronization

## iOS App

### Requirements
- Xcode 15+
- iOS 17.0+
- Swift 5.9+
- SwiftUI

### Building
```bash
cd ios
open VantisPlayer.xcodeproj
# Build and run in Xcode
```

### Features
- Native SwiftUI interface
- AVPlayer integration
- AirPlay support
- CarPlay integration
- Siri shortcuts
- Widgets for quick playback

## Android App

### Requirements
- Android Studio Hedgehog+
- Android SDK 34+
- Kotlin 1.9+
- Jetpack Compose

### Building
```bash
cd android
./gradlew assembleDebug
```

### Features
- Material 3 design
- ExoPlayer integration
- Android Auto support
- Wear OS companion
- Voice commands
- Quick Settings tiles

## Platform-Specific Features

| Feature | iOS | Android |
|---------|-----|---------|
| Hardware decoding | VideoToolbox | MediaCodec |
| Audio output | AVAudioEngine | Oboe |
| DRM | FairPlay | Widevine |
| Casting | AirPlay | Google Cast |
| Voice | Siri | Google Assistant |
| Auto | CarPlay | Android Auto |
| Watch | watchOS | Wear OS |

## Shared Core API

The shared Rust library exposes these functions via FFI:

```c
// Initialization
void vantis_init(const char* config_path);
void vantis_shutdown();

// Playback
void vantis_play(const char* url);
void vantis_pause();
void vantis_resume();
void vantis_stop();
void vantis_seek(double seconds);
double vantis_get_position();
double vantis_get_duration();

// Audio
void vantis_set_volume(float volume);
float vantis_get_volume();

// Subtitles
void vantis_load_subtitle(const char* path, const char* lang);
void vantis_set_subtitle_enabled(bool enabled);

// Cloud
void vantis_sync_begin();
int vantis_sync_get_status();

// Callbacks
typedef void (*event_callback_t)(const char* event_json);
void vantis_set_event_callback(event_callback_t callback);
```

## Development Setup

1. Install Rust targets:
```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

2. Build shared core:
```bash
# iOS
cargo build --target aarch64-apple-ios --release

# Android (requires NDK)
cargo build --target aarch64-linux-android --release
```

3. Open platform-specific project in IDE.

## License
MIT