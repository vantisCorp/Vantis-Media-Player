# 🤝 Contributing to Vantis Media Player

Thank you for your interest in contributing to Vantis Media Player! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Documentation](#documentation)
7. [Pull Request Process](#pull-request-process)
8. [Reporting Issues](#reporting-issues)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

- **Respect**: Treat all contributors with respect
- **Inclusivity**: Welcome diverse perspectives
- **Collaboration**: Work together constructively
- **Focus**: Keep discussions on topic
- **Learning**: Help others learn and grow

### Unacceptable Behavior

- Harassment or discriminatory language
- Personal attacks
- Unwelcome sexual attention
- Trolling or inflammatory remarks
- Private information disclosure

### Reporting

Contact conduct@vantis-os.org to report violations.

## Getting Started

### Prerequisites

- **Rust**: 1.93.0 or later (Stable recommended)
- **Git**: Latest stable version
- **Clang**: For building FFmpeg dependencies
- **Make**: For build automation

### Setup Development Environment

```bash
# Fork the repository
# Click "Fork" on GitHub

# Clone your fork
git clone https://github.com/YOUR_USERNAME/vantis-player.git
cd vantis-player

# Add upstream remote
git remote add upstream https://github.com/vantis-os/vantis-player.git

# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Install development dependencies
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit
cargo install cargo-deny
```

### Building from Source

```bash
# Development build
cargo build

# Release build
cargo build --release

# Build with all features
cargo build --all-features

# Build specific component
cargo build -p vantin-core
cargo build -p vantin-video
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_buffer_pool

# Run ignored tests
cargo test -- --ignored
```

## Development Workflow

### Branch Strategy

```
main           # Stable releases
  ↑
develop        # Development branch
  ↑
feature/*      # New features
bugfix/*       # Bug fixes
hotfix/*       # Critical fixes
docs/*         # Documentation
```

### Creating a Feature Branch

```bash
# Sync with upstream
git fetch upstream
git checkout develop
git merge upstream/develop

# Create feature branch
git checkout -b feature/your-feature-name

# Make changes...
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Maintenance tasks

**Examples:**
```
feat(video): add AV1 codec support

Add hardware-accelerated AV1 decoding support for NVIDIA
and AMD GPUs. Falls back to software decoding on unsupported
hardware.

Closes #123

fix(audio): correct audio sync drift

Fix audio desynchronization issue when seeking in long
videos (> 2 hours). The drift was caused by accumulated
timestamp errors.

Fixes #456
```

### Pre-Commit Checks

```bash
# Run pre-commit manually
pre-commit run --all-files

# Automatic checks include:
# - Rust formatting (cargo fmt)
# - Clippy linting (cargo clippy)
# - Typos (typos)
# - Tests (cargo test)
```

## Coding Standards

### Rust Style Guide

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// ✅ GOOD: Clear naming, documentation, error handling
/// Creates a new video decoder with the specified configuration.
/// 
/// # Arguments
/// 
/// * `config` - Decoder configuration options
/// 
/// # Returns
/// 
/// Returns a `Result` containing the decoder or an error.
/// 
/// # Errors
/// 
/// Returns an error if:
/// - The codec is not supported
/// - Hardware initialization fails
pub fn create_decoder(config: DecoderConfig) -> Result<Decoder, DecoderError> {
    validate_config(&config)?;
    let decoder = Decoder::new(config)?;
    Ok(decoder)
}

// ❌ BAD: Unclear naming, no documentation
fn mk_dec(c: DecCfg) -> Res<Dec> {
    Dec::new(c)
}
```

### Naming Conventions

```rust
// Structs: PascalCase
struct VideoFrame { }

// Enums: PascalCase
enum CodecType { H264, HEVC, VP9 }

// Functions: snake_case
fn decode_frame() { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_BUFFER_SIZE: usize = 1024 * 1024;

// Type parameters: PascalCase, short and descriptive
struct FrameBuffer<T: Codec> { }
```

### Error Handling

```rust
// Define custom error types
#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    #[error("codec not supported: {0}")]
    UnsupportedCodec(String),
    
    #[error("hardware acceleration failed: {0}")]
    HardwareError(String),
    
    #[error("invalid frame format")]
    InvalidFrameFormat,
}

// Use Result for fallible operations
fn process_frame(frame: VideoFrame) -> Result<ProcessedFrame, VideoError> {
    if frame.is_valid() {
        Ok(ProcessedFrame::from(frame))
    } else {
        Err(VideoError::InvalidFrameFormat)
    }
}

// Use ? operator for propagation
fn decode_video(data: &[u8]) -> Result<Vec<VideoFrame>, VideoError> {
    let decoder = create_decoder(data)?;
    let frames = decoder.decode()?;
    Ok(frames)
}
```

### Documentation

```rust
/// Module-level documentation
//! 
//! This module provides video decoding capabilities using FFmpeg.
//! It supports hardware-accelerated decoding for various codecs.

/// Item-level documentation
/// 
/// The `VideoDecoder` struct handles video decoding operations.
/// It supports both hardware and software decoding methods.
pub struct VideoDecoder {
    /// The codec configuration
    codec: CodecConfig,
    
    /// Hardware decoder instance
    hardware_decoder: Option<HardwareDecoder>,
}

/// Public function documentation
/// 
/// Decodes a single video frame from the provided data.
/// 
/// # Arguments
/// 
/// * `data` - Raw video data to decode
/// 
/// # Returns
/// 
/// Returns `Some(VideoFrame)` if decoding succeeds, `None` otherwise.
/// 
/// # Examples
/// 
/// ```
/// use vantis_video::VideoDecoder;
/// 
/// let decoder = VideoDecoder::new();
/// let frame = decoder.decode_frame(&video_data);
/// ```
pub fn decode_frame(&mut self, data: &[u8]) -> Option<VideoFrame> {
    // Implementation
}
```

### Performance Guidelines

```rust
// ✅ GOOD: Use iterators, avoid allocations
fn process_frames(frames: &[VideoFrame]) -> Vec<ProcessedFrame> {
    frames
        .iter()
        .filter(|f| f.is_valid())
        .map(|f| f.process())
        .collect()
}

// ❌ BAD: Unnecessary allocations, manual loop
fn process_frames_bad(frames: &[VideoFrame]) -> Vec<ProcessedFrame> {
    let mut result = Vec::new();
    for i in 0..frames.len() {
        if frames[i].is_valid() {
            result.push(frames[i].process());
        }
    }
    result
}

// ✅ GOOD: Use Cow for lazy string operations
use std::borrow::Cow;

fn format_message(msg: &str) -> Cow<str> {
    if msg.contains(' ') {
        Cow::Owned(msg.replace(' ', "_"))
    } else {
        Cow::Borrowed(msg)
    }
}

// ✅ GOOD: Zero-copy where possible
fn process_slice(data: &[u8]) -> &[u8] {
    &data[1..data.len()-1]
}
```

### Thread Safety

```rust
use std::sync::{Arc, Mutex, RwLock};

// Use Arc for shared ownership
use std::sync::Arc;

struct SharedDecoder {
    inner: Arc<Mutex<VideoDecoder>>,
}

// Use RwLock for read-heavy workloads
struct SubtitleCache {
    cache: Arc<RwLock<HashMap<String, Subtitle>>>,
}

// Use channels for message passing
use tokio::sync::mpsc;

async fn process_messages(mut rx: mpsc::Receiver<Message>) {
    while let Some(msg) = rx.recv().await {
        handle_message(msg).await;
    }
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_allocation() {
        let buffer = BufferPool::new(1024);
        assert_eq!(buffer.capacity(), 1024);
    }

    #[test]
    fn test_video_frame_creation() {
        let frame = VideoFrame::new(1920, 1080);
        assert_eq!(frame.width(), 1920);
        assert_eq!(frame.height(), 1080);
    }

    #[test]
    fn test_error_handling() {
        let result = decode_frame(&[]);
        assert!(matches!(result, Err(VideoError::InvalidFrameFormat)));
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use vantis_core::*;
use vantis_video::*;
use vantis_audio::*;

#[test]
fn test_full_playback() {
    let player = Player::new(Config::default());
    
    // Load media
    player.load("test_data/sample.mp4").unwrap();
    
    // Play
    player.play().unwrap();
    
    // Verify playback
    assert!(player.is_playing());
    
    // Cleanup
    player.stop();
}
```

### Benchmark Tests

```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_core::buffer::BufferPool;

fn bench_buffer_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_allocation");
    
    for size in [1024, 4096, 16384].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                BufferPool::new(black_box(size))
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_buffer_allocation);
criterion_main!(benches);
```

### Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_buffer_roundtrip(data in prop::collection::vec(any::<u8>(), 0..1000)) {
            let mut buffer = BufferPool::new(data.len());
            buffer.write(&data);
            let result = buffer.read();
            assert_eq!(data, result);
        }

        #[test]
        fn test_frame_dimensions(width in 64u32..4096, height in 64u32..4096) {
            let frame = VideoFrame::new(width, height);
            assert_eq!(frame.width(), width);
            assert_eq!(frame.height(), height);
            assert_eq!(frame.size(), width * height * 4);
        }
    }
}
```

## Documentation

### Code Documentation

- All public items must have documentation
- Include examples for complex APIs
- Document error conditions
- Keep docs up to date

### User Documentation

- Update README for user-facing changes
- Add/update examples
- Update CHANGELOG.md
- Update relevant guides

### API Documentation

```rust
/// # Examples
/// 
/// Basic usage:
/// 
/// ```no_run
/// use vantis_core::Player;
/// 
/// let mut player = Player::new();
/// player.load("video.mp4").unwrap();
/// player.play().unwrap();
/// ```
/// 
/// Advanced usage with callbacks:
/// 
/// ```no_run
/// use vantis_core::Player;
/// 
/// let mut player = Player::new();
/// player.on_frame(|frame| {
///     println!("Frame: {}x{}", frame.width(), frame.height());
/// });
/// ```
```

## Pull Request Process

### Before Submitting

1. **Sync with upstream**
   ```bash
   git fetch upstream
   git rebase upstream/develop
   ```

2. **Run all checks**
   ```bash
   cargo fmt
   cargo clippy --all-targets
   cargo test
   pre-commit run --all-files
   ```

3. **Update documentation**
   - Update README if needed
   - Add CHANGELOG entry
   - Update examples

### Creating Pull Request

1. Push to your fork
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create PR on GitHub
   - Title: Follow conventional commits format
   - Description: Include:
     - What changes were made
     - Why the changes were made
     - Testing performed
     - Screenshots if UI changes
   - Link related issues

3. Fill PR template
   ```markdown
   ## Description
   [Description of changes]
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Testing
   [Describe testing performed]
   
   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] CHANGELOG.md updated
   ```

### PR Review Process

1. **Automated Checks**
   - CI/CD pipeline runs
   - Tests must pass
   - Code coverage must be maintained

2. **Manual Review**
   - At least one maintainer approval required
   - Address all review comments
   - Update PR as needed

3. **Merge**
   - Squash and merge
   - Delete branch after merge
   - Update CHANGELOG.md

## Reporting Issues

### Bug Reports

Use the issue template:

```markdown
## Bug Description
[Clear description of the bug]

## Steps to Reproduce
1. Go to...
2. Click on...
3. See error...

## Expected Behavior
[What you expected to happen]

## Actual Behavior
[What actually happened]

## Environment
- OS: [e.g., Ubuntu 22.04]
- Vantis Version: [e.g., 1.0.0]
- Rust Version: [e.g., 1.75.0]
- GPU: [e.g., RTX 3060]

## Additional Info
[Logs, screenshots, etc.]
```

### Feature Requests

```markdown
## Feature Description
[Description of the feature]

## Use Case
[Why do you need this feature?]

## Proposed Solution
[How should it work?]

## Alternatives
[What other approaches did you consider?]

## Additional Context
[Any other relevant information]
```

### Documentation Issues

```markdown
## Documentation Issue
[Description of the issue]

## Location
[File/Section where issue exists]

## Suggested Fix
[How should it be corrected?]
```

## Development Tools

### Recommended IDE Extensions

- **VS Code**:
  - rust-analyzer
  - CodeLLDB
  - Even Better TOML
  - crates

- **IntelliJ IDEA**:
  - Rust plugin
  - TOML plugin

### Useful Commands

```bash
# Watch for changes and rebuild
cargo watch -x check -x test

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench

# Fuzz testing
cargo fuzz run <target>

# Security audit
cargo audit
cargo deny check

# Measure test coverage
cargo tarpaulin --out Html
```

### Debugging

```bash
# Build with debug symbols
cargo build

# Run with gdb
gdb target/debug/vantis

# Run with lldb
lldb target/debug/vantis

# Enable debug logging
RUST_LOG=debug cargo run

# Backtrace on panic
RUST_BACKTRACE=1 cargo run
```

## Getting Help

- **GitHub Discussions**: Ask questions
- **Discord**: Join our community server
- **Documentation**: Read the guides
- **Issues**: Search existing issues

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md
- Release notes
- Project website

Thank you for contributing to Vantis Media Player! 🎉