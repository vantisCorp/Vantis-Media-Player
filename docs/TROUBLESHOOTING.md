# 🔧 Troubleshooting Guide

## Common Issues and Solutions

### Build Issues

#### "Failed to find suitable GPU adapter"

**Problem**: Vantis cannot find a compatible GPU for rendering.

**Solutions**:
1. Update your graphics drivers
2. Ensure you have a GPU that supports Vulkan/DX12/Metal
3. Try running with software rendering:
   ```bash
   RUSTFLAGS="--cfg gl" cargo run
   ```

#### "FFmpeg not found"

**Problem**: Video decoding libraries are missing.

**Solutions**:
- **Linux**: `sudo apt install ffmpeg libavcodec-dev`
- **macOS**: `brew install ffmpeg`
- **Windows**: Download from [ffmpeg.org](https://ffmpeg.org/download.html)

### Runtime Issues

#### No audio output

**Problem**: Video plays but no sound.

**Solutions**:
1. Check audio device permissions
2. Verify exclusive mode is supported:
   ```bash
   vantis --verbose
   ```
3. Try non-exclusive mode:
   ```toml
   [audio]
   exclusive_mode = false
   ```

#### Subtitles not displaying

**Problem**: Subtitles downloaded but not showing.

**Solutions**:
1. Check subtitle encoding (should be UTF-8)
2. Verify subtitle format is supported
3. Manually sync subtitles:
   ```bash
   vantis subtitles sync subtitles.srt video.mp4
   ```

#### Crashes on startup

**Problem**: Player crashes immediately.

**Solutions**:
1. Enable verbose logging:
   ```bash
   RUST_LOG=debug vantis
   ```
2. Check logs for specific errors
3. Try resetting configuration:
   ```bash
   rm ~/.vantis/config.toml
   ```

### Performance Issues

#### Choppy video playback

**Problem**: Video stutters or drops frames.

**Solutions**:
1. Disable hardware acceleration if using weak GPU:
   ```toml
   [video]
   hardware_acceleration = false
   ```
2. Reduce upscaling quality:
   ```toml
   [video]
   ai_upscaling = false
   ```
3. Increase buffer size:
   ```toml
   [advanced]
   buffer_size_mb = 1024
   ```

#### High CPU usage

**Problem**: CPU usage is too high.

**Solutions**:
1. Enable hardware acceleration
2. Disable motion interpolation:
   ```toml
   [video]
   motion_interpolation = false
   ```
3. Reduce thread count:
   ```toml
   [advanced]
   max_threads = 4
   ```

### Subtitle Issues

#### "Krzaczy" (garbled characters) in Polish subtitles

**Problem**: Polish characters display incorrectly.

**Solutions**:
1. Enable automatic encoding detection:
   ```toml
   [subtitles]
   auto_encoding_detection = true
   ```
2. Manually convert subtitle file:
   ```bash
   iconv -f CP1250 -t UTF-8 subtitles.srt > subtitles_utf8.srt
   ```

#### Subtitles out of sync

**Problem**: Subtitles appear too early or late.

**Solutions**:
1. Use AI sync:
   ```bash
   vantis subtitles sync --ai subtitles.srt video.mp4
   ```
2. Manual adjustment:
   ```bash
   vantis subtitles sync --offset 500 subtitles.srt video.mp4
   ```

### Plugin Issues

#### Plugin won't load

**Problem**: WASM plugin fails to load.

**Solutions**:
1. Verify WASM file format:
   ```bash
   wasm-validate plugin.wasm
   ```
2. Check for required exports:
   - `init()`
   - `shutdown()`
   - `tick()`

#### Plugin crashes

**Problem**: Plugin causes player to crash.

**Solutions**:
1. Enable WASM debugging:
   ```toml
   [advanced]
   wasm_debug = true
   ```
2. Check plugin logs
3. Test plugin in isolation

### Network Issues

#### Cannot connect to TMDB

**Problem**: Metadata not downloading.

**Solutions**:
1. Check internet connection
2. Verify API key in configuration
3. Check rate limiting
4. Use proxy if needed

#### Subtitle download fails

**Problem**: Cannot download subtitles.

**Solutions**:
1. Check subtitle source availability
2. Verify video file hash
3. Try different sources:
   ```bash
   vantis subtitles download --source opensubtitles video.mp4
   ```

### Platform-Specific Issues

#### Linux

**No sound on PipeWire/PulseAudio**

```bash
# Install PipeWire support
sudo apt install pipewire pipewire-audio-client-libraries

# Restart audio
systemctl --user restart pipewire pipewire-pulse
```

**GPU rendering fails on NVIDIA**

```bash
# Install NVIDIA Vulkan support
sudo apt install vulkan-tools nvidia-vulkan-icd

# Verify Vulkan
vulkaninfo
```

#### Windows

**WSL2 GPU support**

```powershell
# Update WSL2
wsl --update

# Install WSL2 GPU drivers
# Download from: https://developer.nvidia.com/cuda/wsl
```

**DirectX12 not available**

- Update Windows 10/11
- Install latest GPU drivers
- Check Windows update for DirectX12

#### macOS

**Metal not supported**

- Ensure macOS 10.15+ (Catalina)
- Update to latest macOS version
- Check GPU compatibility

**File permissions**

```bash
# Grant file access permissions
xattr -cr /Applications/Vantis.app
```

### Debug Mode

Enable comprehensive debugging:

```bash
# Full debug output
RUST_LOG=debug vantis --verbose

# Trace specific component
RUST_LOG=vantis_core=trace,vantis_video=debug vantis

# Save logs to file
RUST_LOG=debug vantis 2>&1 | tee vantis_debug.log
```

### Performance Profiling

Profile the player:

```bash
# Build with profiling
cargo build --release

# Run with perf (Linux)
perf record --call-graph dwarf vantis
perf report

# Or use flamegraph
cargo flamegraph --bin vantis
```

### Memory Issues

#### Out of memory

**Problem**: Player crashes with OOM error.

**Solutions**:
1. Reduce buffer size:
   ```toml
   [advanced]
   buffer_size_mb = 256
   ```
2. Disable upscaling
3. Close other applications
4. Check for memory leaks:
   ```bash
   valgrind --leak-check=full vantis
   ```

### Getting Help

If you can't resolve your issue:

1. **Check logs**: Enable verbose mode and review output
2. **Search issues**: Check existing GitHub issues
3. **Provide information**:
   - Vantis version
   - Operating system
   - GPU/CPU information
   - Error messages
   - Steps to reproduce

4. **Create bug report**:
   ```bash
   vantis --version > system_info.txt
   RUST_LOG=debug vantis > debug_log.txt 2>&1
   ```

### Recovery

#### Reset all settings

```bash
# Backup current config
cp ~/.vantis/config.toml ~/.vantis/config.toml.backup

# Reset to defaults
rm ~/.vantis/config.toml
vantis  # Will create new config
```

#### Clear cache

```bash
# Clear thumbnail cache
rm -rf ~/.vantis/cache/thumbnails/*

# Clear subtitle cache
rm -rf ~/.vantis/cache/subtitles/*

# Clear all cache
rm -rf ~/.vantis/cache/*
```

#### Reinstall plugins

```bash
# Remove all plugins
rm -rf ~/.vantis/plugins/*

# Reinstall from source
vantis plugins install --all
```

---

**Still having issues?** Check the documentation or open an issue on GitHub.