# Vantis Advanced Audio Features

## Overview

The Vantis Advanced Audio module provides cutting-edge audio processing capabilities for the Vantis Media Player. This module implements professional-grade audio features including room correction, headphone virtualization, audio fingerprinting, real-time visualization, and multi-channel processing.

## Features

### 1. Room Correction Engine

The room correction system compensates for acoustic imperfections in your listening environment using impulse response measurements and digital signal processing.

#### Capabilities

- **Automatic Calibration**: Measure room acoustics and generate correction filters
- **Impulse Response Analysis**: Analyze room reflections and reverberation
- **Acoustic Parameter Calculation**: RT60, EDT, bass ratio, brilliance ratio
- **Correction Filter Generation**: FIR filters for frequency response correction
- **Target Curve Customization**: Define desired frequency response
- **Calibration Import/Export**: Save and load calibration profiles

#### Usage Example

```rust
use vantis_advanced_audio::{AdvancedAudioEngine, AdvancedAudioConfig};

// Create engine with room correction enabled
let mut config = AdvancedAudioConfig::default();
config.room_correction.enabled = true;
config.room_correction.auto_calibration = true;

let engine = AdvancedAudioEngine::new(config)?;

// Start calibration
let measurement = engine.room_correction().start_calibration(10).await?;

// Get room parameters
let params = engine.room_correction().get_room_parameters().await;
println!("RT60: {:.2}s", params.rt60);

// Process audio with correction
let processed = engine.process_audio(&samples, 48000).await?;
```

#### Acoustic Parameters

- **RT60**: Reverberation time (time for sound to decay by 60dB)
- **EDT**: Early Decay Time (initial decay rate)
- **Bass Ratio**: Ratio of low-frequency to mid-frequency energy
- **Brilliance Ratio**: Ratio of high-frequency to low-frequency energy
- **C50**: Clarity for speech (early vs late energy)
- **C80**: Clarity for music
- **STI**: Speech Transmission Index

---

### 2. Headphone Virtualization

Transform stereo audio into immersive spatial audio for headphones using HRTF (Head-Related Transfer Functions) and advanced binaural rendering.

#### Virtualization Modes

- **Stereo**: Standard stereo playback with optional crossfeed
- **Surround 5.1**: Virtual 5.1 surround sound
- **Surround 7.1**: Virtual 7.1 surround sound
- **Atmos**: Dolby Atmos virtualization
- **Binaural**: True binaural rendering
- **Ambisonics**: Ambisonics format support

#### Features

- **HRTF-Based Rendering**: Accurate spatial audio simulation
- **Crossfeed**: Natural stereo imaging for headphones
- **Head Tracking**: Optional head orientation tracking
- **Custom HRTF Datasets**: Support for personalized HRTF profiles
- **Reverb Simulation**: Room acoustic simulation

#### Usage Example

```rust
// Set virtualization mode
engine.headphone_virtualizer().set_mode(VirtualizationMode::Surround51).await?;

// Enable crossfeed
engine.headphone_virtualizer().set_crossfeed(true, 0.5).await?;

// Process 5.1 audio for headphones
let processed = engine.headphone_virtualizer().process(&surround_51_samples, 48000).await?;

// Update head orientation (for head tracking)
engine.headphone_virtualizer().update_head_orientation(15.0, 5.0, 0.0).await?;
```

#### Crossfeed Settings

Crossfeed blends left and right channels to reduce listener fatigue and create a more natural soundstage.

- **Strength**: 0.0 (disabled) to 1.0 (maximum crossfeed)
- **Recommended**: 0.3-0.5 for most content

---

### 3. Audio Fingerprinting

Identify music and audio content using advanced fingerprinting algorithms.

#### Supported Algorithms

- **Chromaprint**: AcoustID-compatible fingerprinting
- **Custom FFT**: FFT-based fingerprinting with custom features
- **Deep Learning**: Neural network-based fingerprinting (experimental)

#### Features

- **Real-time Recognition**: Identify audio as it plays
- **Database Management**: Add, remove, and search fingerprints
- **Confidence Scoring**: Match quality assessment
- **Metadata Association**: Store track information with fingerprints
- **Import/Export**: Database backup and restore

#### Usage Example

```rust
// Generate fingerprint
let analysis = engine.fingerprinter().generate_fingerprint(&samples, 48000).await?;

// Recognize audio
if let Some(result) = engine.fingerprinter().recognize(&analysis).await? {
    println!("Found: {} - {}", result.track.artist, result.track.title);
    println!("Confidence: {:.2}%", result.confidence * 100.0);
}

// Add to database
let entry = FingerprintEntry {
    id: "track_001".to_string(),
    title: "Song Title".to_string(),
    artist: "Artist Name".to_string(),
    album: "Album Name".to_string(),
    duration: 180,
    fingerprint: analysis.fingerprint.clone(),
    algorithm: FingerprintAlgorithm::Chromaprint,
};
engine.fingerprinter().add_to_database(entry).await?;

// Search database
let results = engine.fingerprinter().search_database("Artist").await?;
```

#### Recognition Parameters

- **Confidence Threshold**: Minimum confidence for matches (default: 0.8)
- **Algorithm**: Fingerprinting method to use
- **Database Path**: Location of fingerprint database

---

### 4. Audio Visualization

Generate real-time visualizations of audio content for display and analysis.

#### Visualization Types

- **Spectrum**: Frequency spectrum analyzer
- **Waveform**: Time-domain waveform display
- **Spectrogram**: Time-frequency spectrogram
- **Frequency Bands**: Bass, mid, treble level meters
- **Circular Spectrum**: Radial spectrum display
- **3D Visualization**: Three-dimensional audio visualization

#### Features

- **Real-time Processing**: Low-latency visualization
- **Customizable Colors**: Full color scheme control
- **Multiple Resolutions**: Support for various display sizes
- **Frequency Band Analysis**: Extract bass, mid, treble levels
- **Frame Export**: Save visualization frames as images

#### Usage Example

```rust
// Set visualization type
engine.visualizer().set_visualization_type(VisualizationType::Spectrum).await?;

// Set color scheme
let scheme = ColorScheme {
    primary: (0, 255, 255),
    secondary: (255, 0, 255),
    background: (0, 0, 0),
};
engine.visualizer().set_color_scheme(scheme).await?;

// Update with audio
engine.visualizer().update(&samples, 48000).await?;

// Generate frame
let frame = engine.visualizer().generate_frame(800, 600).await?;

// Get frequency bands
let bands = engine.visualizer().get_frequency_bands().await?;
println!("Bass: {:.2}, Mid: {:.2}, Treble: {:.2}", 
    bands.bass, bands.mid, bands.treble);
```

#### Visualization Parameters

- **FFT Size**: 512, 1024, 2048, 4096 (default: 2048)
- **Update Rate**: Frames per second (default: 60)
- **Color Scheme**: Primary, secondary, background colors

---

### 5. Multi-Channel Processing

Handle various audio channel configurations with intelligent upmixing and downmixing.

#### Supported Configurations

- **Mono**: Single channel
- **Stereo**: 2 channels
- **Stereo 2.1**: 2 channels + LFE
- **Surround 5.1**: 6 channels (L, R, C, LFE, LS, RS)
- **Surround 7.1**: 8 channels (L, R, C, LFE, LS, RS, LSR, RSR)
- **Atmos**: Object-based audio

#### Features

- **Automatic Layout Detection**: Identify input channel configuration
- **Intelligent Upmixing**: Expand stereo to surround
- **Intelligent Downmixing**: Convert surround to stereo
- **Bass Management**: Redirect low frequencies to LFE
- **Channel Mapping**: Custom channel routing

#### Usage Example

```rust
// Set output configuration
engine.multichannel_processor().set_output_config(ChannelConfiguration::Surround51).await?;

// Enable upmixing
engine.multichannel_processor().set_upmixing(true).await?;

// Enable bass management
engine.multichannel_processor().set_bass_management(true, 80.0).await?;

// Process audio (auto-detects input layout)
let processed = engine.multichannel_processor().process(&stereo_samples, 48000).await?;
```

#### Upmixing/Downmixing Strategies

**Stereo to 5.1**:
- Center channel: (L + R) × 0.707
- LFE: (L + R) × 0.5
- Surrounds: L/R × 0.707

**5.1 to Stereo**:
- Lt = L + C×0.707 + LS×0.707 + LFE×0.5
- Rt = R + C×0.707 + RS×0.707 + LFE×0.5

---

## Configuration

### Complete Configuration Example

```rust
use vantis_advanced_audio::{AdvancedAudioConfig, RoomCorrectionConfig, 
    HeadphoneVirtualizationConfig, FingerprintingConfig, 
    VisualizationConfig, MultichannelConfig};

let config = AdvancedAudioConfig {
    room_correction: RoomCorrectionConfig {
        enabled: true,
        frequency_bands: 64,
        target_curve: vec![0.0; 64],
        measurement_duration: 10,
        auto_calibration: true,
    },
    headphone_virtualization: HeadphoneVirtualizationConfig {
        enabled: true,
        mode: VirtualizationMode::Surround51,
        hrtf_dataset: "default".to_string(),
        crossfeed: true,
        crossfeed_strength: 0.5,
    },
    fingerprinting: FingerprintingConfig {
        enabled: true,
        algorithm: FingerprintAlgorithm::Chromaprint,
        database_path: "fingerprints.db".to_string(),
        online_recognition: false,
        confidence_threshold: 0.8,
    },
    visualization: VisualizationConfig {
        enabled: true,
        visualization_type: VisualizationType::Spectrum,
        fft_size: 2048,
        update_rate: 60,
        color_scheme: ColorScheme::default(),
    },
    multichannel: MultichannelConfig {
        enabled: true,
        output_channels: ChannelConfiguration::Surround51,
        upmixing: true,
        downmixing: true,
        bass_management: true,
        crossover_frequency: 80.0,
    },
};

let engine = AdvancedAudioEngine::new(config)?;
```

---

## Performance Considerations

### CPU Usage

- **Room Correction**: Moderate (FIR filtering)
- **Headphone Virtualization**: Low to Moderate (HRTF convolution)
- **Audio Fingerprinting**: Low (Chromaprint), Moderate (FFT), High (Deep Learning)
- **Visualization**: Low to Moderate (FFT-based)
- **Multi-Channel Processing**: Very Low (matrix operations)

### Memory Usage

- **Room Correction**: ~10-50 MB (depends on filter length)
- **Headphone Virtualization**: ~5-20 MB (HRTF dataset)
- **Audio Fingerprinting**: ~10-100 MB (database size)
- **Visualization**: ~5-10 MB (frame buffers)
- **Multi-Channel Processing**: ~1-5 MB (temporary buffers)

### Latency

- **Room Correction**: 10-50 ms (filter length dependent)
- **Headphone Virtualization**: 5-20 ms
- **Audio Fingerprinting**: 100-500 ms (recognition)
- **Visualization**: 5-15 ms (frame generation)
- **Multi-Channel Processing**: <1 ms

---

## Best Practices

### Room Correction

1. **Calibration Environment**: Perform calibration in quiet conditions
2. **Microphone Placement**: Use measurement microphone at listening position
3. **Multiple Measurements**: Take several measurements and average
4. **Target Curve**: Use flat or slight house curve for natural sound
5. **Regular Recalibration**: Recalibrate when room changes

### Headphone Virtualization

1. **Mode Selection**: Use appropriate mode for content type
2. **Crossfeed**: Enable for long listening sessions
3. **HRTF Selection**: Try different HRTF datasets for best fit
4. **Volume**: Keep at moderate levels for best spatial effect
5. **Content**: Works best with properly mixed surround content

### Audio Fingerprinting

1. **Database Size**: Keep database manageable for fast recognition
2. **Confidence Threshold**: Adjust based on use case
3. **Algorithm Selection**: Chromaprint for music, Custom FFT for general audio
4. **Quality**: Use high-quality audio for best recognition
5. **Duration**: Minimum 10-30 seconds for reliable recognition

### Visualization

1. **FFT Size**: Larger = better resolution, higher CPU
2. **Update Rate**: Match display refresh rate
3. **Color Scheme**: Use high-contrast colors for visibility
4. **Resolution**: Match output display resolution
5. **Performance**: Monitor CPU usage with high update rates

### Multi-Channel Processing

1. **Output Configuration**: Match your speaker setup
2. **Upmixing/Downmixing**: Enable based on content
3. **Bass Management**: Enable if you have a subwoofer
4. **Crossover Frequency**: 80 Hz is standard, adjust based on speakers
5. **Testing**: Test with various content types

---

## Troubleshooting

### Room Correction Issues

**Problem**: Calibration fails or produces poor results
- **Solution**: Ensure quiet environment, check microphone placement, increase measurement duration

**Problem**: Audio sounds unnatural after correction
- **Solution**: Adjust target curve, reduce correction strength, recalibrate

### Headphone Virtualization Issues

**Problem**: No spatial effect
- **Solution**: Check virtualization mode, ensure content is multi-channel, try different HRTF

**Problem**: Audio sounds phasey or unnatural
- **Solution**: Reduce crossfeed strength, try different HRTF dataset, disable virtualization

### Audio Fingerprinting Issues

**Problem**: No matches found
- **Solution**: Lower confidence threshold, check database, ensure audio quality

**Problem**: False positives
- **Solution**: Increase confidence threshold, improve database quality

### Visualization Issues

**Problem**: Low frame rate
- **Solution**: Reduce FFT size, lower update rate, check CPU usage

**Problem**: Visualization looks wrong
- **Solution**: Check audio format, verify sample rate, adjust color scheme

### Multi-Channel Issues

**Problem**: Wrong channel mapping
- **Solution**: Verify input/output configuration, check channel layout detection

**Problem**: Audio clipping after processing
- **Solution**: Check gain levels, enable normalization, verify upmixing/downmixing

---

## API Reference

### AdvancedAudioEngine

Main engine coordinating all advanced audio features.

```rust
pub struct AdvancedAudioEngine {
    // ...
}

impl AdvancedAudioEngine {
    pub fn new(config: AdvancedAudioConfig) -> Result<Self>;
    pub fn room_correction(&self) -> &RoomCorrectionEngine;
    pub fn headphone_virtualizer(&self) -> &HeadphoneVirtualizer;
    pub fn fingerprinter(&self) -> &AudioFingerprinter;
    pub fn visualizer(&self) -> &AudioVisualizer;
    pub fn multichannel_processor(&self) -> &MultichannelProcessor;
    pub async fn update_config(&self, config: AdvancedAudioConfig) -> Result<()>;
    pub async fn get_config(&self) -> AdvancedAudioConfig;
    pub async fn process_audio(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
}
```

### RoomCorrectionEngine

Room acoustic correction and calibration.

```rust
pub struct RoomCorrectionEngine {
    // ...
}

impl RoomCorrectionEngine {
    pub fn new() -> Result<Self>;
    pub fn is_initialized(&self) -> bool;
    pub async fn start_calibration(&self, duration: u32) -> Result<CalibrationMeasurement>;
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
    pub async fn set_target_curve(&self, curve: Vec<f32>) -> Result<()>;
    pub async fn get_room_parameters(&self) -> RoomParameters;
    pub async fn export_calibration(&self) -> Result<String>;
    pub async fn import_calibration(&self, data: &str) -> Result<()>;
}
```

### HeadphoneVirtualizer

Binaural rendering and spatial audio for headphones.

```rust
pub struct HeadphoneVirtualizer {
    // ...
}

impl HeadphoneVirtualizer {
    pub fn new() -> Result<Self>;
    pub fn is_initialized(&self) -> bool;
    pub async fn set_mode(&self, mode: VirtualizationMode) -> Result<()>;
    pub async fn get_mode(&self) -> VirtualizationMode;
    pub async fn set_crossfeed(&self, enabled: bool, strength: f32) -> Result<()>;
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
    pub async fn update_head_orientation(&self, yaw: f32, pitch: f32, roll: f32) -> Result<()>;
}
```

### AudioFingerprinter

Audio content identification and recognition.

```rust
pub struct AudioFingerprinter {
    // ...
}

impl AudioFingerprinter {
    pub fn new() -> Result<Self>;
    pub fn is_initialized(&self) -> bool;
    pub async fn set_algorithm(&self, algorithm: FingerprintAlgorithm) -> Result<()>;
    pub async fn set_confidence_threshold(&self, threshold: f32) -> Result<()>;
    pub async fn generate_fingerprint(&self, samples: &[f32], sample_rate: u32) -> Result<FingerprintAnalysis>;
    pub async fn recognize(&self, analysis: &FingerprintAnalysis) -> Result<Option<RecognitionResult>>;
    pub async fn add_to_database(&self, entry: FingerprintEntry) -> Result<()>;
    pub async fn remove_from_database(&self, id: &str) -> Result<bool>;
    pub async fn search_database(&self, query: &str) -> Result<Vec<FingerprintEntry>>;
    pub async fn get_database_stats(&self) -> DatabaseStats;
    pub async fn export_database(&self) -> Result<String>;
    pub async fn import_database(&self, data: &str) -> Result<()>;
}
```

### AudioVisualizer

Real-time audio visualization.

```rust
pub struct AudioVisualizer {
    // ...
}

impl AudioVisualizer {
    pub fn new() -> Result<Self>;
    pub fn is_initialized(&self) -> bool;
    pub async fn set_visualization_type(&self, viz_type: VisualizationType) -> Result<()>;
    pub async fn set_color_scheme(&self, scheme: ColorScheme) -> Result<()>;
    pub async fn set_fft_size(&self, size: usize) -> Result<()>;
    pub async fn update(&self, samples: &[f32], sample_rate: u32) -> Result<()>;
    pub async fn generate_frame(&self, width: usize, height: usize) -> Result<VisualizationFrame>;
    pub async fn get_frequency_bands(&self) -> Result<FrequencyBands>;
    pub async fn get_spectrum(&self) -> Vec<f32>;
    pub async fn get_waveform(&self) -> Vec<f32>;
}
```

### MultichannelProcessor

Multi-channel audio processing and conversion.

```rust
pub struct MultichannelProcessor {
    // ...
}

impl MultichannelProcessor {
    pub fn new() -> Result<Self>;
    pub fn is_initialized(&self) -> bool;
    pub fn detect_layout(&self, samples: &[f32]) -> ChannelLayout;
    pub async fn set_output_config(&self, config: ChannelConfiguration) -> Result<()>;
    pub async fn set_upmixing(&self, enabled: bool) -> Result<()>;
    pub async fn set_downmixing(&self, enabled: bool) -> Result<()>;
    pub async fn set_bass_management(&self, enabled: bool, crossover_freq: f32) -> Result<()>;
    pub async fn process(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
}
```

---

## Examples

See the `examples/` directory for complete working examples:

- `advanced_audio_example.rs`: Comprehensive example demonstrating all features
- `room_correction_example.rs`: Room calibration and correction
- `headphone_virtualization_example.rs`: Spatial audio for headphones
- `audio_fingerprinting_example.rs`: Music recognition
- `audio_visualization_example.rs`: Real-time visualization
- `multichannel_processing_example.rs`: Channel conversion

---

## Dependencies

### Core Dependencies

- `tokio`: Async runtime
- `anyhow`: Error handling
- `thiserror`: Error types
- `tracing`: Logging
- `serde`: Serialization

### Audio Processing

- `rubato`: Audio resampling
- `symphonia`: Audio decoding
- `cpal`: Audio output
- `rustfft`: FFT computation
- `realfft`: Real FFT

### Machine Learning

- `burn`: ML framework for AI features
- `nalgebra`: Linear algebra
- `num-complex`: Complex numbers

### Visualization

- `image`: Image processing

### Fingerprinting

- `chromaprint`: Audio fingerprinting

---

## License

MIT License - See LICENSE file for details

## Contributing

See CONTRIBUTING.md for guidelines on contributing to the Vantis Media Player project.

## Support

For issues, questions, or contributions, please visit the project repository.