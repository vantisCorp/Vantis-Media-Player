# 🎬 Vantis Media Player - Usage Examples

## Basic Usage

### Playing a Video File

```bash
# Play a video file
vantis play movie.mp4

# Play starting at specific time (30 seconds)
vantis play movie.mp4 --start 30

# Play with verbose output
vantis play movie.mp4 --verbose
```

### Subtitle Operations

```bash
# Download subtitles for a video
vantis subtitles download movie.mp4

# Download subtitles in specific language
vantis subtitles download movie.mp4 --language pl

# Search for subtitles
vantis subtitles search "Inception"

# Sync subtitles with video
vantis subtitles sync subtitles.srt movie.mp4
```

### Plugin Management

```bash
# List installed plugins
vantis plugins list

# Load a plugin
vantis plugins load ./my_plugin.wasm

# Unload a plugin
vantis plugins unload my_plugin_name

# Build a plugin from source
vantis plugins build ./plugin_source/
```

### Media Library Scanning

```bash
# Scan a directory
vantis scan ~/Videos/

# Scan recursively
vantis scan ~/Movies/ --recursive

# Scan and generate configuration
vantis scan ~/Media/ --recursive > library.json
```

## Programming Examples

### Basic Player

```rust
use anyhow::Result;
use vanis_core::VantisCore;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();
    
    // Create core
    let mut core = VantisCore::new()?;
    
    // Initialize subsystems
    core.initialize_renderer().await?;
    core.initialize_audio().await?;
    core.initialize_subtitles().await?;
    
    // Run player
    core.run_event_loop().await;
    
    Ok(())
}
```

### Video Playback

```rust
use vanis_video::VideoEngine;

async fn play_video() -> Result<()> {
    // Create video engine
    let engine = VideoEngine::new(surface).await?;
    
    // Load video
    engine.load_video("movie.mp4").await?;
    
    // Playback loop
    loop {
        if let Some(frame) = engine.next_frame().await? {
            engine.render_frame(&frame)?;
        }
    }
}
```

### Audio Playback

```rust
use vanis_audio::AudioEngine;

async fn play_audio() -> Result<()> {
    // Create audio engine
    let mut engine = AudioEngine::new()?;
    
    // Initialize default device
    engine.initialize_device(None)?;
    
    // Load audio file
    engine.load_audio("music.mp3")?;
    
    // Start playback
    engine.play()?;
    
    // Set volume to 50%
    engine.set_volume(0.5);
    
    Ok(())
}
```

### Subtitle Download

```rust
use vanis_subtitles::VantisBabel;

async fn download_subtitles() -> Result<()> {
    let mut babel = VantisBabel::new()?;
    
    // Search for Polish subtitles
    let tracks = babel.search_subtitles("movie.mp4", Some("pl")).await?;
    
    // Download best match
    if let Some(track) = tracks.first() {
        println!("Found subtitle: {}", track.source);
        println!("Score: {:.2}", track.score);
    }
    
    Ok(())
}
```

---

**For more examples, check the `examples/` directory in the repository.**