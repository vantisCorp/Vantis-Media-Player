---
sidebar_position: 2
---

# Audio Playback

Vantis Media Player offers comprehensive audio playback capabilities with support for various formats, audio processing, and visualization.

## Supported Audio Formats

### Container Formats

- **MP3** (MPEG-1 Audio Layer III)
- **AAC** (Advanced Audio Coding)
- **FLAC** (Free Lossless Audio Codec)
- **WAV** (Waveform Audio File Format)
- **OGG** (Ogg Vorbis)
- **OPUS** (Opus Audio Codec)
- **M4A** (MPEG-4 Audio)
- **WMA** (Windows Media Audio)
- **ALAC** (Apple Lossless Audio Codec)
- **DSD** (Direct Stream Digital)

### Audio Codecs

- **MP3** - Up to 320 kbps
- **AAC** - Up to 256 kbps per channel
- **FLAC** - Lossless compression
- **Vorbis** - Ogg container
- **Opus** - Low latency, high quality
- **PCM** - Uncompressed
- **DSD** - High-resolution audio

## Basic Audio Playback

### Loading Audio

```javascript
const player = new Player('#container');
player.load('audio.mp3');
player.play();
```

```rust
let mut player = Player::new()?;
player.load("audio.mp3").await?;
player.play();
```

```bash
vantismedia audio.mp3
```

### Streaming Audio

```javascript
// HTTP stream
player.load('https://example.com/stream.mp3');

// HLS audio stream
player.load('https://example.com/audio.m3u8');

// Icecast/SHOUTcast stream
player.load('https://stream.example.com:8000/stream');
```

## Volume Control

### Setting Volume

```javascript
// Set volume (0.0 to 1.0)
player.setVolume(0.8);

// Get current volume
const volume = player.getVolume();

// Mute/unmute
player.setMuted(true);
const isMuted = player.isMuted();
```

```bash
# Set volume percentage
vantismedia --volume 80 audio.mp3

# Mute
vantismedia --mute audio.mp3
```

### Volume Normalization

```toml
[audio]
normalization = true
target_level = -16  # dB LUFS
```

```javascript
player.setNormalization(true);
```

## Audio Devices

### Listing Audio Devices

```bash
# List available audio devices
vantismedia --list-audio-devices

# Set default output device
vantismedia --audio-device "alsa_output.pci-0000_00_1f.3.analog-stereo" audio.mp3
```

```javascript
// Get available devices
const devices = player.getAudioDevices();
console.log(devices);

// Set output device
await player.setAudioDevice(deviceId);
```

### Device Priority

```toml
[audio]
output_device = "default"
fallback_device = "auto"
```

## Audio Processing

### Equalizer

Vantis Media Player includes a built-in 10-band equalizer.

```javascript
// Enable equalizer
player.setEqualizerEnabled(true);

// Set individual band (in dB)
// Bands: 32, 64, 125, 250, 500, 1k, 2k, 4k, 8k, 16k
player.setEqualizerBand(0, -3);   // 32 Hz
player.setEqualizerBand(1, -1);   // 64 Hz
player.setEqualizerBand(5, 2);    // 1 kHz
player.setEqualizerBand(9, 3);    // 16 kHz

// Get equalizer settings
const bands = player.getEqualizerBands();
```

### Equalizer Presets

```javascript
// Load preset
player.loadEqualizerPreset('rock');

// Available presets:
// - flat
// - classical
// - club
// - dance
// - folk
// - jazz
// - metal
// - pop
// - reggae
// - rock
// - techno
```

### Audio Effects

```javascript
// Bass boost
player.setBassBoost(true);
player.setBassBoostLevel(10);  // 0-20 dB

// Treble boost
player.setTrebleBoost(true);
player.setTrebleBoostLevel(5);  // 0-10 dB

// Reverb
player.setReverbEnabled(true);
player.setReverbRoomSize(0.5);
player.setReverbDamping(0.5);

// Echo
player.setEchoEnabled(true);
player.setEchoDelay(500);  // ms
player.setEchoDecay(0.5);
```

## Audio Channels

### Channel Configuration

```javascript
// Get channel count
const channels = player.getChannelCount();  // 1, 2, 4, 5, 6, 8

// Set channel layout
player.setChannelLayout('stereo');  // mono, stereo, 5.1, 7.1

// Downmix to stereo
player.setDownmix(true);
```

### Surround Sound

Vantis Media Player supports multi-channel audio:

- **Stereo** (2 channels)
- **5.1 Surround** (6 channels)
- **7.1 Surround** (8 channels)
- **Dolby Atmos** (object-based audio)

```toml
[audio]
surround = true
channel_layout = "5.1"
```

## Audio Visualization

### Built-in Visualizers

```javascript
// Enable visualizer
player.setVisualizer('bars');

// Available visualizers:
// - none
// - bars
// - waveform
// - spectrum
// - circular

// Set visualizer colors
player.setVisualizerColors({
    primary: '#25c2a0',
    secondary: '#00d4ff'
});

// Set sensitivity
player.setVisualizerSensitivity(1.5);
```

### Custom Visualizer

```javascript
// Get audio data for custom visualization
player.on('audioData', (data) => {
    const { samples, frequencyData } = data;
    // Use data to render custom visualization
});
```

## Audio Metadata

### Reading Audio Information

```javascript
// Get audio metadata
const metadata = player.getAudioMetadata();
console.log(metadata);

// Example output:
{
  codec: 'mp3',
  bitrate: 320000,
  sampleRate: 44100,
  channels: 2,
  duration: 240,
  title: 'Song Title',
  artist: 'Artist Name',
  album: 'Album Name',
  year: 2024
}
```

### ID3 Tags

Vantis Media Player reads and displays ID3 tags:

```javascript
// Get track info
const trackInfo = player.getTrackInfo();
console.log(trackInfo.title);
console.log(trackInfo.artist);
console.log(trackInfo.album);
console.log(trackInfo.trackNumber);
console.log(trackInfo.genre);
```

### Album Art

```javascript
// Get album artwork
const artwork = player.getAlbumArt();
if (artwork) {
    document.getElementById('cover').src = artwork;
}
```

## Playback Speed

### Changing Speed

```javascript
// Set playback speed (0.5x to 2.0x)
player.setSpeed(1.5);  // 1.5x speed

// Get current speed
const speed = player.getSpeed();

// Speed presets
player.setSpeed(0.5);  // Half speed
player.setSpeed(0.75); // 75% speed
player.setSpeed(1.0);  // Normal
player.setSpeed(1.5);  // 1.5x speed
player.setSpeed(2.0);  // Double speed
```

### Pitch Preservation

```javascript
// Preserve pitch when changing speed
player.setPitchPreservation(true);
```

## Audio Latency

### Latency Control

```toml
[audio]
latency = 50  # milliseconds
```

```javascript
// Set audio latency in ms
player.setAudioLatency(50);

// Get current latency
const latency = player.getAudioLatency();
```

### Low Latency Mode

```bash
vantismedia --low-latency stream.m3u8
```

## Gapless Playback

### Enabling Gapless Playback

```toml
[player]
gapless = true
```

```javascript
// Enable gapless playback for playlists
player.setGapless(true);
```

### Crossfading

```javascript
// Enable crossfade
player.setCrossfade(true);
player.setCrossfadeDuration(2000);  // 2 seconds
```

## Audio Streams

### Multiple Audio Tracks

```javascript
// Get available audio tracks
const tracks = player.getAudioTracks();
console.log(tracks);

// Select audio track
player.setAudioTrack(1);

// Get current track
const currentTrack = player.getCurrentAudioTrack();
```

### Language Selection

```javascript
// Select track by language
player.setAudioTrackByLanguage('en');
```

## Recording Audio

### Recording Playback

```javascript
// Start recording
player.startRecording({
    format: 'mp3',
    quality: 'high'
});

// Stop recording
const recording = await player.stopRecording();

// Save recording
recording.save('recording.mp3');
```

### Recording Configuration

```javascript
player.startRecording({
    format: 'wav',  // wav, mp3, flac
    bitrate: 320,   // kbps
    sampleRate: 44100
});
```

## Audio Filters

### Advanced Audio Filters

```javascript
// High-pass filter
player.setHighPassFilter(80);  // Hz

// Low-pass filter
player.setLowPassFilter(16000);  // Hz

// Band-pass filter
player.setBandPassFilter(1000, 4000);  // Hz

// Notch filter
player.setNotchFilter(1000);  // Hz

// Compressor
player.setCompressor({
    threshold: -20,  // dB
    ratio: 4,
    attack: 10,      // ms
    release: 100     // ms
});
```

## Audio Sync

### Syncing Audio with Video

```javascript
// Set audio delay (in milliseconds)
player.setAudioDelay(100);

// Get current delay
const delay = player.getAudioDelay();
```

## Troubleshooting

### No Audio

```bash
# Check audio devices
vantismedia --list-audio-devices

# Try different device
vantismedia --audio-device "default" audio.mp3
```

### Distorted Audio

```toml
[audio]
normalization = false
```

### Audio Out of Sync

```javascript
// Reset audio sync
player.setAudioDelay(0);
```

## Next Steps

- **[Video Playback](./video-playback)** - Learn about video features
- **[Subtitles](./subtitles)** - Subtitle management
- **[API Reference](../api/)** - Complete API documentation

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)