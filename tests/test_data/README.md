# Test Data Directory

This directory contains test media files and data used for integration and unit testing.

## Test Files

### Video Files
- **sample.mp4** - Sample video file (1080p, H.264, AAC)
- **sample_720p.mp4** - Sample video file (720p, H.264, AAC)
- **sample_4k.mp4** - Sample video file (4K, HEVC, AAC)

### Audio Files
- **sample_audio.mp3** - Sample MP3 audio file
- **sample_audio.flac** - Sample FLAC audio file
- **sample_audio.wav** - Sample WAV audio file

### Subtitle Files
- **subtitles.srt** - Sample SRT subtitle file (UTF-8)
- **subtitles_cp1250.srt** - Sample SRT subtitle file (CP1250 encoding)
- **subtitles_utf8.srt** - Sample SRT subtitle file (UTF-8 BOM)
- **subtitles_iso8859.srt** - Sample SRT subtitle file (ISO-8859-2 encoding)
- **subtitles.ass** - Sample ASS subtitle file
- **subtitles.vtt** - Sample WebVTT subtitle file
- **unsynced_subtitles.srt** - Poorly synced subtitle file for AI sync testing

### Plugin Files
- **test_plugin.wasm** - Test WASM plugin for integration testing

## File Specifications

### Video Files

#### sample.mp4
- **Resolution**: 1920x1080 (1080p)
- **Codec**: H.264
- **Bitrate**: 5 Mbps
- **Duration**: 2 minutes
- **Audio**: AAC, 48 kHz, stereo

#### sample_720p.mp4
- **Resolution**: 1280x720 (720p)
- **Codec**: H.264
- **Bitrate**: 2.5 Mbps
- **Duration**: 2 minutes
- **Audio**: AAC, 48 kHz, stereo

#### sample_4k.mp4
- **Resolution**: 3840x2160 (4K)
- **Codec**: HEVC/H.265
- **Bitrate**: 10 Mbps
- **Duration**: 2 minutes
- **Audio**: AAC, 48 kHz, stereo

### Audio Files

#### sample_audio.mp3
- **Codec**: MP3
- **Bitrate**: 320 kbps
- **Sample Rate**: 48 kHz
- **Channels**: Stereo
- **Duration**: 3 minutes

#### sample_audio.flac
- **Codec**: FLAC
- **Bit Depth**: 24-bit
- **Sample Rate**: 48 kHz
- **Channels**: Stereo
- **Duration**: 3 minutes

#### sample_audio.wav
- **Codec**: PCM
- **Bit Depth**: 16-bit
- **Sample Rate**: 48 kHz
- **Channels**: Stereo
- **Duration**: 3 minutes

### Subtitle Files

#### subtitles.srt
- **Format**: SubRip
- **Encoding**: UTF-8
- **Language**: Polish
- **Entries**: 200

#### subtitles_cp1250.srt
- **Format**: SubRip
- **Encoding**: CP1250 (Windows-1250)
- **Language**: Polish
- **Entries**: 200
- **Purpose**: Testing encoding detection

#### subtitles_utf8.srt
- **Format**: SubRip
- **Encoding**: UTF-8 with BOM
- **Language**: English
- **Entries**: 200

#### subtitles_iso8859.srt
- **Format**: SubRip
- **Encoding**: ISO-8859-2
- **Language**: Polish
- **Entries**: 200
- **Purpose**: Testing encoding detection

#### subtitles.ass
- **Format**: Advanced Substation Alpha
- **Encoding**: UTF-8
- **Language**: English
- **Entries**: 200

#### subtitles.vtt
- **Format**: WebVTT
- **Encoding**: UTF-8
- **Language**: English
- **Entries**: 200

#### unsynced_subtitles.srt
- **Format**: SubRip
- **Encoding**: UTF-8
- **Language**: English
- **Entries**: 200
- **Sync Offset**: +5 seconds
- **Purpose**: Testing AI subtitle synchronization

## Usage in Tests

### Integration Tests

Test files are used in `tests/integration_tests.rs`:

```rust
#[test]
fn test_full_playback_workflow() {
    let mut player = Player::new();
    player.load("tests/test_data/sample.mp4");
    // ...
}
```

### Unit Tests

Test files can be used in unit tests as well:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtitle_parsing() {
        let parser = SubtitleParser::new();
        let subtitles = parser.parse_file(
            "tests/test_data/subtitles.srt",
            SubtitleFormat::SRT
        );
        // ...
    }
}
```

## Generating Test Files

If you need to generate your own test files, you can use FFmpeg:

### Generate Sample Video

```bash
# Generate 1080p video
ffmpeg -f lavfi -i testsrc=duration=120:size=1920x1080:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=120 \
       -c:v libx264 -preset fast -crf 23 \
       -c:a aac -b:a 128k \
       sample.mp4

# Generate 720p video
ffmpeg -f lavfi -i testsrc=duration=120:size=1280x720:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=120 \
       -c:v libx264 -preset fast -crf 23 \
       -c:a aac -b:a 128k \
       sample_720p.mp4

# Generate 4K video
ffmpeg -f lavfi -i testsrc=duration=120:size=3840x2160:rate=30 \
       -f lavfi -i sine=frequency=1000:duration=120 \
       -c:v libx265 -preset fast -crf 28 \
       -c:a aac -b:a 128k \
       sample_4k.mp4
```

### Generate Sample Audio

```bash
# Generate MP3
ffmpeg -f lavfi -i sine=frequency=1000:duration=180 \
       -c:a libmp3lame -b:a 320k \
       sample_audio.mp3

# Generate FLAC
ffmpeg -f lavfi -i sine=frequency=1000:duration=180 \
       -c:a flac -sample_fmt s24 \
       sample_audio.flac

# Generate WAV
ffmpeg -f lavfi -i sine=frequency=1000:duration=180 \
       -c:a pcm_s16le \
       sample_audio.wav
```

### Generate Subtitles

Use subtitle editing tools or generate programmatically:

```python
# Example: Generate SRT subtitles
import time

def generate_srt(output_file, count=200):
    with open(output_file, 'w') as f:
        for i in range(1, count + 1):
            start = i * 3
            end = start + 2
            
            start_time = time.strftime('%H:%M:%S,000', time.gmtime(start))
            end_time = time.strftime('%H:%M:%S,000', time.gmtime(end))
            
            f.write(f"{i}\n")
            f.write(f"{start_time} --> {end_time}\n")
            f.write(f"This is subtitle number {i}.\n\n")

generate_srt('subtitles.srt')
```

## License

Test files are provided for testing purposes and may be subject to different licensing terms than the main project. Ensure you have the right to use and distribute any test files you add to this directory.

## Adding New Test Files

When adding new test files:

1. **Document the file** in this README with specifications
2. **Keep files small** for fast test execution
3. **Use standard formats** for compatibility
4. **Test encoding variety** for encoding detection tests
5. **Include corrupted files** for error handling tests

## Notes

- All test files should be small enough for quick CI/CD execution
- Avoid using copyrighted content
- Use synthetic content when possible
- Test files are tracked in Git repository (prefer small files)
- For large test files, consider using Git LFS or downloading during tests