# Build System Documentation

## Overview

The Vantis Media Player build system is inspired by VideoLAN's multi-platform approach, providing unified building for all supported platforms. It supports native builds, cross-compilation, and comprehensive CI/CD integration.

## Architecture

```
buildsystem/
├── build.sh              # Universal build script
├── README.md             # Build system documentation
└── crossfiles/           # Cross-compilation configurations
    ├── linux-aarch64.txt
    ├── linux-armv7.txt
    ├── android-arm64.txt
    ├── android-x86_64.txt
    ├── ios-arm64.txt
    └── wasm32-wasi.txt
```

## Features

### Multi-Platform Support

| Platform | Architectures | Status |
|----------|--------------|--------|
| Linux | x86_64, aarch64, armv7 | ✅ Production |
| macOS | x86_64, arm64 (Apple Silicon) | ✅ Production |
| Windows | x86_64 | ✅ Production |
| Android | arm64-v8a, armeabi-v7a, x86_64, x86 | ✅ Production |
| iOS | arm64 (device), arm64 (simulator) | ✅ Production |
| WebAssembly | wasm32-wasi | ✅ Production |

### Cross-Compilation

The build system uses multiple approaches for cross-compilation:

1. **cross**: For Linux cross-compilation (x86_64 → aarch64, armv7)
2. **Android NDK**: For Android builds (via ANDROID_NDK_HOME)
3. **Xcode**: For iOS builds (macOS only)
4. **wasm-pack**: For WebAssembly builds

### Build Targets

#### Native Builds

```bash
# Build for current platform
./buildsystem/build.sh native
```

#### Linux Cross-Compilation

```bash
# Build for all Linux architectures
./buildsystem/build.sh linux-all
```

#### Android Builds

```bash
# Build for all Android architectures
./buildsystem/build.sh android-all
```

#### iOS Builds

```bash
# Build for all iOS architectures (macOS only)
./buildsystem/build.sh ios-all
```

#### WebAssembly Builds

```bash
# Build for WebAssembly
./buildsystem/build.sh wasm
```

## CI/CD Integration

### GitHub Actions Workflows

#### Build Workflow (`.github/workflows/build.yml`)

- **Native Builds**: Linux, macOS, Windows
- **Cross-Compilation**: Linux ARM64, ARMv7
- **Android**: All supported architectures
- **iOS**: Device and simulator builds
- **WebAssembly**: Optimized WASM builds

#### Security Workflow (`.github/workflows/security.yml`)

- **Gitleaks**: Secret detection
- **Trivy**: Vulnerability scanning
- **CodeQL**: Semantic code analysis
- **Cargo Audit**: Rust security advisories
- **Cargo Deny**: License and dependency checks
- **SBOM**: Software Bill of Materials generation
- **Security Scorecard**: Security best practices

#### Testing Workflow (`.github/workflows/testing.yml`)

- **Unit Tests**: All platforms, multiple Rust versions
- **Integration Tests**: Full system integration
- **Fuzz Testing**: Automated fuzz testing
- **Benchmarks**: Performance regression detection
- **Code Coverage**: Tarpaulin-based coverage
- **Clippy**: Linting and warnings
- **Rustfmt**: Code formatting checks
- **Conformance Tests**: Media format compliance
- **Memory Safety**: Valgrind, ASan, UBSan
- **Thread Safety**: ThreadSanitizer

## Dependency Management

### Cargo Deny Configuration (`cargo-deny.toml`)

- **Advisories**: Security vulnerability checks
- **Licenses**: License compliance (MIT, Apache-2.0, BSD)
- **Bans**: Duplicate dependency detection
- **Sources**: Trusted crate registries

### Allowed Licenses

- MIT
- Apache-2.0
- Apache-2.0 WITH LLVM-exception
- BSD-2-Clause
- BSD-3-Clause
- BSL-1.0
- CC0-1.0
- ISC
- MPL-2.0
- Unicode-DFS-2016
- Unicode-3.0

### Disallowed Licenses

- GPL-2.0
- GPL-3.0
- AGPL-3.0

## Code Ownership

### CODEOWNERS File

The `.github/CODEOWNERS` file defines code review responsibility:

- **Core Team**: Review required for all changes
- **Architects**: Architecture and design documents
- **DevOps**: Build system, CI/CD, Docker
- **Security Team**: Security-related code
- **Core Engine**: Core media engine
- **Video Team**: Video processing, codecs
- **Audio Team**: Audio processing, spatial audio
- **Subtitles Team**: Subtitles and captions
- **UI Team**: User interface components
- **Streaming Team**: Network and streaming
- **Plugin Team**: Plugin system
- **AI Team**: AI and ML features
- **QA Team**: Testing and benchmarks
- **Docs Team**: Documentation
- **CLI Team**: Command-line interface
- **Integrations Team**: Third-party integrations
- **DevTools Team**: Development tools

## Best Practices

### Build System Usage

1. **Always use the build script**: Don't call `cargo build` directly for production builds
2. **Test on all platforms**: Use CI/CD to test on all supported platforms
3. **Keep dependencies updated**: Regularly run `cargo update` and security scans
4. **Monitor build times**: Use caching to speed up builds
5. **Check license compliance**: Use `cargo deny check` before releasing

### Cross-Compilation

1. **Use appropriate toolchains**: Install required Rust targets
2. **Set environment variables**: Configure ANDROID_NDK_HOME for Android builds
3. **Test on actual devices**: Emulators may not catch all issues
4. **Optimize for target platform**: Use platform-specific optimizations

### Security

1. **Run security scans**: All security checks must pass before merging
2. **Report vulnerabilities**: Use GitHub's private vulnerability reporting
3. **Keep dependencies updated**: Regularly run `cargo audit`
4. **Review new dependencies**: Check licenses and security advisories

### Testing

1. **Maintain high coverage**: Aim for >80% code coverage
2. **Run all test types**: Unit, integration, fuzz, conformance tests
3. **Monitor benchmarks**: Catch performance regressions early
4. **Test on all platforms**: Don't assume portability

## Comparison with VideoLAN

| Feature | VideoLAN (VLC) | Vantis |
|---------|---------------|--------|
| Build Systems | Autotools, Meson, Cargo | Cargo + Custom Scripts |
| Cross-Compilation | ✅ Advanced | ✅ Simplified |
| Platform Support | 20+ platforms | 6 platforms |
| CI/CD | GitLab CI | GitHub Actions |
| Testing | Comprehensive | Comprehensive |
| Security | Gitleaks, SBOM | Gitleaks, Cargo Audit, SBOM |
| Code Ownership | CODEOWNERS | CODEOWNERS |
| License Compliance | Custom | Cargo Deny |

## Troubleshooting

### Build Failures

**Problem**: Build fails with linker errors

**Solution**:
```bash
# Install required system dependencies
sudo apt-get install -y build-essential libssl-dev pkg-config

# For Android
export ANDROID_NDK_HOME=/path/to/android-ndk
```

**Problem**: Cross-compilation fails

**Solution**:
```bash
# Install cross
cargo install cross

# Add target
rustup target add aarch64-unknown-linux-gnu
```

### Security Scan Failures

**Problem**: Gitleaks detects secrets

**Solution**:
1. Remove secrets from code
2. Use environment variables
3. Add to `.gitleaks.toml` if false positive

**Problem**: Cargo audit finds vulnerabilities

**Solution**:
```bash
# Update dependencies
cargo update

# Check for patches
cargo audit
```

### Test Failures

**Problem**: Tests fail on specific platform

**Solution**:
1. Check platform-specific code
2. Add platform-specific tests
3. Use conditional compilation

## References

- [VideoLAN Build System](https://code.videolan.org/videolan/vlc)
- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Cross](https://github.com/cross-rs/cross)
- [Cargo Deny](https://embarkstudios.github.io/cargo-deny/)
- [GitHub Actions](https://docs.github.com/en/actions)

## Future Improvements

- [ ] Add Bazel build system support
- [ ] Implement reproducible builds
- [ ] Add build cache sharing across jobs
- [ ] Implement build matrix optimization
- [ ] Add more platforms (FreeBSD, OpenBSD, etc.)
- [ ] Implement distributed builds
- [ ] Add build performance monitoring