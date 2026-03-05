---
sidebar_position: 3
---

# Audio API

The Audio API provides methods for managing audio playback, tracks, effects, and visualization.

## Audio Track Management

### getAudioTracks()

Get all available audio tracks.

```javascript
const tracks = player.getAudioTracks();
// [
//   { id: 1, language: 'en', label: 'English', enabled: true, codec: 'aac' },
//   { id: 2, language: 'es', label: 'Spanish', enabled: false, codec: 'aac' },
//   { id: 3, language: 'fr', label: 'French', enabled: false, codec: 'aac' }
// ]
```

### setAudioTrack()

Select an audio track.

```javascript
// Select by track ID
player.setAudioTrack(2);

// Select by language code
player.setAudioTrackByLanguage('es');

// Select by label
player.setAudioTrackByLabel('Spanish');
```

### getCurrentAudioTrack()

Get the currently selected audio track.

```javascript
const track = player.getCurrentAudioTrack();
// { id: 1, language: 'en', label: 'English', enabled: true }
```

### disableAudioTrack()

Disable audio track (mute audio).

```javascript
player.disableAudioTrack();
```

## Volume Control

### setVolume()

Set volume level.

```javascript
// Set volume (0.0 to 1.0)
player.setVolume(0.8);

// Animate volume change
player.setVolume(0.8, { duration: 500 });  // 500ms fade
```

### getVolume()

Get current volume.

```javascript
const volume = player.getVolume();  // 0.0 to 1.0
const volumePercent = Math.round(volume * 100);
```

### setMuted()

Set mute state.

```javascript
player.setMuted(true);
player.setMuted(false);
```

### isMuted()

Check mute state.

```javascript
const muted = player.isMuted();
```

### toggleMute()

Toggle mute.

```javascript
player.toggleMute();
```

## Audio Devices

### getAudioDevices()

Get available audio output devices.

```javascript
const devices = await player.getAudioDevices();
// [
//   { deviceId: 'default', label: 'Default', kind: 'audiooutput' },
//   { deviceId: 'speakers', label: 'Speakers', kind: 'audiooutput' },
//   { deviceId: 'headphones', label: 'Headphones', kind: 'audiooutput' }
// ]
```

### setAudioDevice()

Set audio output device.

```javascript
await player.setAudioDevice('speakers');
```

### getDefaultAudioDevice()

Get the default audio device.

```javascript
const defaultDevice = await player.getDefaultAudioDevice();
```

## Audio Effects

### Equalizer

#### getEqualizer()

Get equalizer settings.

```javascript
const eq = player.getEqualizer();
// {
//   enabled: true,
//   bands: [-3, -1, 0, 2, 3, 2, 0, -1, -2, -1]
// }
```

#### setEqualizer()

Set equalizer settings.

```javascript
// Enable/disable equalizer
player.setEqualizerEnabled(true);

// Set individual band (10 bands: 32, 64, 125, 250, 500, 1k, 2k, 4k, 8k, 16k Hz)
player.setEqualizerBand(0, -3);  // 32 Hz: -3 dB
player.setEqualizerBand(5, 2);   // 1 kHz: +2 dB

// Set all bands at once
player.setEqualizerBands([-3, -1, 0, 2, 3, 2, 0, -1, -2, -1]);
```

#### Equalizer Presets

```javascript
// Load preset
player.loadEqualizerPreset('rock');

// Available presets:
// - flat, classical, club, dance, electronic, folk
// - jazz, latin, metal, pop, r&b, reggae, rock, techno
```

### Bass Boost

```javascript
// Enable bass boost
player.setBassBoost(true);

// Set level (0-20 dB)
player.setBassBoostLevel(10);
```

### Treble Boost

```javascript
// Enable treble boost
player.setTrebleBoost(true);

// Set level (0-10 dB)
player.setTrebleBoostLevel(5);
```

### Surround Sound

```javascript
// Enable surround
player.setSurround(true);

// Set mode
player.setSurroundMode('5.1');  // 'stereo', '5.1', '7.1'
```

### Audio Normalization

```javascript
// Enable normalization
player.setAudioNormalization(true);

// Set target level (dB LUFS)
player.setNormalizationLevel(-16);
```

### Pitch Shift

```javascript
// Shift pitch (semitones)
player.setPitchShift(2);  // Up 2 semitones
player.setPitchShift(-1); // Down 1 semitone
player.setPitchShift(0);  // Reset
```

### Tempo Change

```javascript
// Change tempo without pitch shift
player.setTempo(1.1);  // 10% faster
player.setTempo(0.9);  // 10% slower
```

## Audio Visualization

### enableVisualizer()

Enable audio visualization.

```javascript
player.enableVisualizer('bars');
// Types: 'bars', 'wave', 'spectrum', 'circular', 'custom'
```

### setVisualizerConfig()

Configure visualizer.

```javascript
player.setVisualizerConfig({
    type: 'bars',
    sensitivity: 1.5,
    smoothing: 0.8,
    colors: {
        primary: '#25c2a0',
        secondary: '#00d4ff',
        background: 'transparent'
    },
    fftSize: 2048,
    fps: 60
});
```

### getAudioData()

Get raw audio data for custom visualization.

```javascript
// Frequency data
const frequencyData = player.getFrequencyData();  // Uint8Array

// Time domain data (waveform)
const waveformData = player.getWaveformData();  // Uint8Array

// Audio analysis
player.on('audioData', (data) => {
    const { frequency, waveform, peak, rms } = data;
    // Custom visualization logic
});
```

### disableVisualizer()

Disable visualization.

```javascript
player.disableVisualizer();
```

## Audio Delay

### setAudioDelay()

Set audio delay for sync.

```javascript
// Positive delay (audio plays later)
player.setAudioDelay(100);  // 100ms delay

// Negative delay (audio plays earlier)
player.setAudioDelay(-50);  // 50ms advance
```

### getAudioDelay()

Get current audio delay.

```javascript
const delay = player.getAudioDelay();  // milliseconds
```

## Audio Statistics

### getAudioStats()

Get audio playback statistics.

```javascript
const stats = player.getAudioStats();
// {
//   codec: 'aac',
//   sampleRate: 44100,
//   channels: 2,
//   bitrate: 128000,
//   latency: 50,
//   underruns: 0,
//   peakLevel: -3.2,
//   rmsLevel: -18.5
// }
```

## Channel Control

### getChannelCount()

Get number of audio channels.

```javascript
const channels = player.getChannelCount();  // 1, 2, 6, 8
```

### setChannelLayout()

Set channel layout.

```javascript
player.setChannelLayout('stereo');  // 'mono', 'stereo', '5.1', '7.1'
```

### setDownmix()

Enable downmixing to stereo.

```javascript
player.setDownmix(true);  // Downmix surround to stereo
```

### setChannelGain()

Set gain for specific channel.

```javascript
player.setChannelGain(0, 1.2);  // 20% boost for left channel
player.setChannelGain(1, 0.8);  // 20% reduction for right channel
```

## Audio Filters

### High-Pass Filter

```javascript
player.setHighPassFilter({
    frequency: 80,  // Hz
    resonance: 0.7
});
```

### Low-Pass Filter

```javascript
player.setLowPassFilter({
    frequency: 16000,  // Hz
    resonance: 0.7
});
```

### Notch Filter

```javascript
// Remove specific frequency (e.g., hum at 60 Hz)
player.setNotchFilter({
    frequency: 60,  // Hz
    bandwidth: 10
});
```

### Compressor

```javascript
player.setCompressor({
    threshold: -20,  // dB
    ratio: 4,
    attack: 10,      // ms
    release: 100,    // ms
    knee: 10         // dB
});
```

### Limiter

```javascript
player.setLimiter({
    threshold: -1,  // dB
    release: 50     // ms
});
```

## Audio Recording

### startAudioRecording()

Start recording audio output.

```javascript
const recording = player.startAudioRecording({
    format: 'wav',      // 'wav', 'mp3', 'flac'
    sampleRate: 44100,
    channels: 2,
    bitrate: 320        // kbps for mp3
});
```

### stopAudioRecording()

Stop recording and save.

```javascript
const result = await player.stopAudioRecording();
await result.save('recording.wav');

// Or get as blob
const blob = result.getBlob();
```

## Audio Format Support

### Supported Formats

| Format | Extension | Codec | Max Bitrate |
|--------|-----------|-------|-------------|
| MP3 | .mp3 | MPEG Audio | 320 kbps |
| AAC | .m4a, .aac | AAC | 256 kbps |
| FLAC | .flac | FLAC | Lossless |
| Opus | .opus | Opus | 510 kbps |
| Vorbis | .ogg | Vorbis | 500 kbps |
| WAV | .wav | PCM | N/A |
| ALAC | .m4a | ALAC | Lossless |
| DSD | .dsf, .dff | DSD | N/A |

### Check Format Support

```javascript
const isSupported = player.isAudioFormatSupported('flac');
const codecSupport = player.getSupportedAudioCodecs();
```

## Events

### Audio Events

```javascript
// Audio track changed
player.on('audiotrackchange', (track) => {
    console.log('Audio track changed to:', track.label);
});

// Volume changed
player.on('volumechange', (volume) => {
    console.log('Volume:', Math.round(volume * 100) + '%');
});

// Mute changed
player.on('mutechange', (muted) => {
    console.log('Muted:', muted);
});

// Audio device changed
player.on('audiodevicechange', (device) => {
    console.log('Audio device:', device.label);
});

// Equalizer changed
player.on('equalizerchange', (bands) => {
    console.log('EQ bands:', bands);
});
```

## Configuration

### Audio Configuration Object

```javascript
const audioConfig = {
    outputDevice: 'default',
    latency: 50,           // ms
    normalization: false,
    targetLevel: -16,      // dB LUFS
    equalizer: {
        enabled: false,
        preset: 'flat',
        bands: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    },
    visualizer: {
        enabled: false,
        type: 'bars'
    },
    sampleRate: 44100,
    channels: 2
};

player.setAudioConfig(audioConfig);
```

## Rust API

```rust
use vantismedia::audio::{AudioConfig, Equalizer};

// Get audio tracks
let tracks = player.audio_tracks();

// Set audio track
player.set_audio_track(2)?;

// Volume control
player.set_volume(0.8);
let volume = player.volume();

// Mute
player.set_muted(true);
let is_muted = player.is_muted();

// Equalizer
let eq = Equalizer::new()
    .with_band(0, -3.0)
    .with_band(5, 2.0)
    .with_preset("rock");
player.set_equalizer(eq)?;

// Audio delay
player.set_audio_delay(Duration::from_millis(100));
```

## Next Steps

- **[Video API](./video)** - Video-specific features
- **[Subtitles API](./subtitles)** - Subtitle management
- **[Events API](./events)** - Event system

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)