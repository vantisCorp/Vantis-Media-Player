# 📚 Vantis API Reference

## Core API

### VantisCore

Main entry point for the Vantis Media Player.

```rust
use vantis_core::{VantisCore, Config};

let config = Config::default();
let mut core = VantisCore::new(config)?;

// Set engines
let video_engine = vantis_video::VideoEngine::new().await?;
core.set_video_engine(video_engine)?;

let audio_engine = vantis_audio::AudioEngine::new()?;
core.set_audio_engine(audio_engine)?;

// Playback control
core.play_media("movie.mp4").await?;
core.pause().await?;
core.resume().await?;
core.seek(30.0).await?;
core.set_volume(0.5).await?;
core.set_mute(true).await?;
core.set_speed(1.5).await?;

// Run event loop
core.run_event_loop().await;
```

### Event Types

```rust
use vantis_core::events::Event;

Event::Play { file: String }
Event::Pause
Event::Resume
Event::Stop
Event::Seek { position: f64 }
Event::SetVolume { volume: f32 }
Event::SetMute { muted: bool }
Event::SetSpeed { speed: f32 }
Event::Quit
```

## Video API

### VideoEngine

GPU-accelerated video processing.

```rust
use vantis_video::VideoEngine;

// Create without surface
let engine = VideoEngine::new().await?;

// Create with surface
let engine = VideoEngine::new_with_surface(surface).await?;

// Load and play
engine.load_video("movie.mp4").await?;
let frame = engine.next_frame().await?;
engine.render_frame(&frame)?;

// Resize
engine.resize(1920, 1080);
```

## Audio API

### AudioEngine

Bit-perfect audio playback.

```rust
use vantis_audio::AudioEngine;

let mut engine = AudioEngine::new()?;
engine.initialize_device(None)?;
engine.load_audio("audio.mp3")?;
engine.play()?;
engine.pause()?;
engine.stop()?;
engine.set_volume(0.5);
engine.set_muted(true);
```

## Subtitle API

### SubtitleEngine

```rust
use vantis_subtitles::SubtitleEngine;

let mut engine = SubtitleEngine::new()?;
engine.load_subtitle("movie.srt")?;
engine.download_subtitle("movie.mp4", "en").await?;
engine.sync_subtitle("track_id", "audio.mp3").await?;
```

## AI API

### AIEngine

```rust
use vantis_ai::AIEngine;

let engine = AIEngine::new()?;

// Video enhancement
engine.enhance_video("input.mp4", "output.mp4").await?;

// Scene detection
let scenes = engine.detect_scenes("movie.mp4").await?;

// Audio enhancement
engine.enhance_audio("input.mp3", "output.mp3").await?;

// Subtitle timing
engine.adjust_subtitle_timing("subs.srt", "audio.mp3").await?;

// Recommendations
let recommendations = engine.get_recommendations().await?;
```

## Streaming API

### StreamingEngine

```rust
use vantis_streaming::StreamingEngine;

let engine = StreamingEngine::new()?;

// Adaptive streaming
engine.play_stream("https://example.com/stream.m3u8").await?;

// P2P streaming
engine.start_p2p_stream("content_id").await?;

// Recording
engine.start_recording("output.mp4").await?;
engine.stop_recording().await?;
```

## Advanced Audio API

### AdvancedAudioEngine

```rust
use vantis_advanced_audio::AdvancedAudioEngine;

let engine = AdvancedAudioEngine::new()?;

// Room correction
engine.calibrate_room().await?;
engine.apply_room_correction().await?;

// Headphone virtualization
engine.enable_headphone_virtualization().await?;

// Audio fingerprinting
let fingerprint = engine.fingerprint_audio("song.mp3").await?;

// Visualization
let spectrum = engine.get_spectrum().await?;
```

## Advanced Video API

### AdvancedVideoEngine

```rust
use vantis_advanced_video::AdvancedVideoEngine;

let engine = AdvancedVideoEngine::new()?;

// Stabilization
engine.stabilize_video("input.mp4", "output.mp4").await?;

// Frame interpolation
engine.interpolate_frames("input.mp4", "output.mp4", 60).await?;

// Denoising
engine.denoise_video("input.mp4", "output.mp4").await?;

// Color grading
engine.apply_color_grade("input.mp4", "output.mp4", preset).await?;

// Comparison
let psnr = engine.compare_videos("video1.mp4", "video2.mp4").await?;
```

## Advanced UI API

### AdvancedUIEngine

```rust
use vantis_advanced_ui::AdvancedUIEngine;

let engine = AdvancedUIEngine::new()?;

// Picture-in-Picture
engine.enable_pip().await?;
engine.set_pip_position(PiPPosition::TopRight).await?;

// Mini-player
engine.enable_mini_player().await?;

// Theater mode
engine.enable_theater_mode().await?;

// Gestures
engine.enable_gestures().await?;

// Shortcuts
engine.register_shortcut("Ctrl+P", action).await?;
```

## Subtitle API

### VantisBabel

Advanced subtitle management.

```rust
use vanis_subtitles::VantisBabel;

let mut babel = VantisBabel::new()?;
let tracks = babel.search_subtitles("movie.mp4", Some("pl")).await?;
babel.sync_subtitle("track-id", "movie.mp4").await?;
let text = babel.get_subtitle_at("track-id", 5000);
```

---

**For more examples, see the `examples/` directory.**