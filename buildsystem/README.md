# Universal Build System

Inspired by VideoLAN's multi-platform build system, this provides unified building for all Vantis Media Player platforms.

## Quick Start

```bash
# Build for native platform
./buildsystem/build.sh native

# Build for all Linux architectures
./buildsystem/build.sh linux-all

# Build for all Android architectures
./buildsystem/build.sh android-all

# Build for WebAssembly
./buildsystem/build.sh wasm

# Clean build artifacts
./buildsystem/build.sh clean
```

## Supported Platforms

### Native Builds
- **Linux**: x86_64, aarch64, armv7
- **macOS**: x86_64, arm64 (Apple Silicon)
- **Windows**: x86_64

### Cross-Compilation
- **Android**: arm64-v8a, armeabi-v7a, x86_64, x86
- **iOS**: arm64 (device), arm64 (simulator)
- **WebAssembly**: wasm32-wasi

## Requirements

### Native Builds
- Rust 1.70+
- Cargo
- GCC/Clang

### Android Builds
- Android NDK (set ANDROID_NDK_HOME)
- Rust targets: aarch64-linux-android, armv7-linux-androideabi, x86_64-linux-android, i686-linux-android

### iOS Builds
- macOS
- Xcode Command Line Tools
- Rust targets: aarch64-apple-ios, aarch64-apple-ios-sim

### WebAssembly Builds
- Rust target: wasm32-wasi
- wasm-opt (optional, for optimization)

## Cross-Compilation Toolchain

The build system uses `cross` for Linux cross-compilation:

```bash
cargo install cross
```

## Build Output

Build artifacts are placed in:
- `target/build/` - Intermediate build files
- `target/dist/` - Final release binaries

### Output Format

- **Linux**: tar.gz archives
- **macOS**: DMG images (TODO)
- **Windows**: ZIP archives
- **Android**: Native libraries in architecture-specific directories
- **iOS**: Static libraries (.a) for Xcode integration
- **WebAssembly**: .wasm binaries

## Advanced Usage

### Build for Specific Target

```bash
./buildsystem/build.sh --target aarch64-linux-gnu
```

### Debug Build

```bash
./buildsystem/build.sh -d native
```

### Verbose Output

```bash
./buildsystem/build.sh -v native
```

## Continuous Integration

The build system is designed for CI/CD pipelines:

```yaml
- name: Build (Linux)
  run: ./buildsystem/build.sh linux-all

- name: Build (macOS)
  run: ./buildsystem/build.sh native

- name: Build (Windows)
  run: ./buildsystem/build.sh native

- name: Build (Android)
  run: ./buildsystem/build.sh android-all
  env:
    ANDROID_NDK_HOME: ${{ secrets.ANDROID_NDK }}

- name: Build (WebAssembly)
  run: ./buildsystem/build.sh wasm
```

## Troubleshooting

### Android NDK Not Found

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk
./buildsystem/build.sh android-all
```

### Cross-Compilation Fails

```bash
cargo install cross
rustup target add aarch64-unknown-linux-gnu
```

### WebAssembly Build Fails

```bash
rustup target add wasm32-wasi
```

## Comparison with VideoLAN

| Feature | VideoLAN (VLC) | Vantis |
|---------|---------------|--------|
| Autotools | ✅ | ❌ (Rust-based) |
| Meson | ✅ | ❌ (Cargo) |
| Custom Scripts | ✅ | ✅ |
| Cross-compilation | ✅ | ✅ |
| Multiple Platforms | ✅ | ✅ |
| 80% Assembly | ❌ | ❌ (Rust optimizations) |

## References

- VideoLAN Build System: https://code.videolan.org/videolan/vlc
- Cross: https://github.com/cross-rs/cross
- Rust Cross-Compilation: https://rust-lang.github.io/rustup/cross-compilation.html