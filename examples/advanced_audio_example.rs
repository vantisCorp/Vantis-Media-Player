//! Advanced Audio Features Example
//! 
//! This example demonstrates all advanced audio features including:
//! - Room correction and calibration
//! - Headphone virtualization
//! - Audio fingerprinting
//! - Audio visualization
//! - Multi-channel processing

use anyhow::Result;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use tracing_subscriber;

use vantis_advanced_audio::{
    AdvancedAudioEngine, AdvancedAudioConfig,
    RoomCorrectionConfig, HeadphoneVirtualizationConfig,
    FingerprintingConfig, VisualizationConfig, MultichannelConfig,
    VirtualizationMode, FingerprintAlgorithm, VisualizationType,
    ChannelConfiguration, ColorScheme,
    FingerprintEntry, RecognitionResult,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=== Vantis Advanced Audio Features Example ===\n");

    // Create advanced audio configuration
    let config = create_advanced_config();
    
    // Initialize the engine
    info!("Initializing Advanced Audio Engine...");
    let engine = AdvancedAudioEngine::new(config)?;
    info!("✓ Engine initialized successfully\n");

    // Demonstrate room correction
    demo_room_correction(&engine).await?;

    // Demonstrate headphone virtualization
    demo_headphone_virtualization(&engine).await?;

    // Demonstrate audio fingerprinting
    demo_audio_fingerprinting(&engine).await?;

    // Demonstrate audio visualization
    demo_audio_visualization(&engine).await?;

    // Demonstrate multi-channel processing
    demo_multichannel_processing(&engine).await?;

    // Demonstrate combined processing
    demo_combined_processing(&engine).await?;

    info!("\n=== All demonstrations completed successfully ===");
    Ok(())
}

/// Create advanced audio configuration
fn create_advanced_config() -> AdvancedAudioConfig {
    AdvancedAudioConfig {
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
    }
}

/// Generate test audio samples
fn generate_test_audio(duration_seconds: f32, sample_rate: u32, frequency: f32) -> Vec<f32> {
    let num_samples = (duration_seconds * sample_rate as f32) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.5
        })
        .collect()
}

/// Generate stereo test audio
fn generate_stereo_audio(duration_seconds: f32, sample_rate: u32) -> Vec<f32> {
    let num_samples = (duration_seconds * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 2);
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let left = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
        let right = (2.0 * std::f32::consts::PI * 445.0 * t).sin() * 0.5;
        samples.push(left);
        samples.push(right);
    }
    
    samples
}

/// Generate 5.1 surround test audio
fn generate_surround_51_audio(duration_seconds: f32, sample_rate: u32) -> Vec<f32> {
    let num_samples = (duration_seconds * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples * 6);
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let left = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
        let right = (2.0 * std::f32::consts::PI * 445.0 * t).sin() * 0.5;
        let center = (2.0 * std::f32::consts::PI * 442.5 * t).sin() * 0.5;
        let lfe = (2.0 * std::f32::consts::PI * 80.0 * t).sin() * 0.3;
        let ls = (2.0 * std::f32::consts::PI * 438.0 * t).sin() * 0.4;
        let rs = (2.0 * std::f32::consts::PI * 447.0 * t).sin() * 0.4;
        
        samples.extend_from_slice(&[left, right, center, lfe, ls, rs]);
    }
    
    samples
}

/// Demonstrate room correction features
async fn demo_room_correction(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Room Correction Demo ---");
    
    let room_correction = engine.room_correction();
    
    // Start calibration
    info!("Starting room calibration (10 seconds)...");
    let measurement = room_correction.start_calibration(10).await?;
    info!("✓ Calibration completed");
    
    // Display room parameters
    let params = room_correction.get_room_parameters().await;
    info!("Room Parameters:");
    info!("  RT60: {:.2}s", params.rt60);
    info!("  EDT: {:.2}s", params.edt);
    info!("  Bass Ratio: {:.2}", params.bass_ratio);
    info!("  Brilliance Ratio: {:.2}", params.brilliance_ratio);
    
    // Process audio with room correction
    info!("\nProcessing audio with room correction...");
    let samples = generate_test_audio(2.0, 48000, 440.0);
    let processed = room_correction.process(&samples, 48000).await?;
    info!("✓ Processed {} samples with room correction", processed.len());
    
    // Export calibration
    let calibration_data = room_correction.export_calibration().await?;
    info!("✓ Exported calibration data ({} bytes)", calibration_data.len());
    
    info!("--- Room Correction Demo Complete ---\n");
    Ok(())
}

/// Demonstrate headphone virtualization features
async fn demo_headphone_virtualization(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Headphone Virtualization Demo ---");
    
    let virtualizer = engine.headphone_virtualizer();
    
    // Test different virtualization modes
    let modes = [
        VirtualizationMode::Stereo,
        VirtualizationMode::Surround51,
        VirtualizationMode::Surround71,
        VirtualizationMode::Binaural,
    ];
    
    for mode in modes {
        info!("\nTesting mode: {:?}", mode);
        virtualizer.set_mode(mode).await?;
        
        let samples = match mode {
            VirtualizationMode::Stereo => generate_stereo_audio(1.0, 48000),
            VirtualizationMode::Surround51 => generate_surround_51_audio(1.0, 48000),
            VirtualizationMode::Surround71 => generate_surround_51_audio(1.0, 48000), // Use 5.1 for demo
            VirtualizationMode::Binaural => generate_stereo_audio(1.0, 48000),
            _ => generate_stereo_audio(1.0, 48000),
        };
        
        let processed = virtualizer.process(&samples, 48000).await?;
        info!("✓ Processed {} samples", processed.len());
    }
    
    // Test crossfeed
    info!("\nTesting crossfeed...");
    virtualizer.set_crossfeed(true, 0.5).await?;
    let samples = generate_stereo_audio(1.0, 48000);
    let processed = virtualizer.process(&samples, 48000).await?;
    info!("✓ Applied crossfeed to {} samples", processed.len());
    
    // Test head tracking
    info!("\nTesting head tracking...");
    virtualizer.update_head_orientation(15.0, 5.0, 0.0).await?;
    info!("✓ Updated head orientation: yaw=15°, pitch=5°, roll=0°");
    
    // Get HRTF info
    let (name, sample_rate, num_filters) = virtualizer.get_hrtf_info().await;
    info!("\nHRTF Dataset:");
    info!("  Name: {}", name);
    info!("  Sample Rate: {} Hz", sample_rate);
    info!("  Filters: {}", num_filters);
    
    info!("--- Headphone Virtualization Demo Complete ---\n");
    Ok(())
}

/// Demonstrate audio fingerprinting features
async fn demo_audio_fingerprinting(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Audio Fingerprinting Demo ---");
    
    let fingerprinter = engine.fingerprinter();
    
    // Test different algorithms
    let algorithms = [
        FingerprintAlgorithm::Chromaprint,
        FingerprintAlgorithm::CustomFFT,
    ];
    
    for algorithm in algorithms {
        info!("\nTesting algorithm: {:?}", algorithm);
        fingerprinter.set_algorithm(algorithm).await?;
        
        let samples = generate_test_audio(5.0, 48000, 440.0);
        let analysis = fingerprinter.generate_fingerprint(&samples, 48000).await?;
        info!("✓ Generated fingerprint ({} hashes)", analysis.fingerprint.len());
        info!("  Duration: {:.2}s", analysis.duration);
        info!("  Sample Rate: {} Hz", analysis.sample_rate);
    }
    
    // Add fingerprint to database
    info!("\nAdding fingerprint to database...");
    let samples = generate_test_audio(5.0, 48000, 440.0);
    let analysis = fingerprinter.generate_fingerprint(&samples, 48000).await?;
    
    let entry = FingerprintEntry {
        id: "test_track_001".to_string(),
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        album: "Test Album".to_string(),
        duration: 5,
        fingerprint: analysis.fingerprint.clone(),
        algorithm: analysis.algorithm,
    };
    
    fingerprinter.add_to_database(entry).await?;
    info!("✓ Added fingerprint to database");
    
    // Get database stats
    let stats = fingerprinter.get_database_stats().await;
    info!("\nDatabase Statistics:");
    info!("  Total Entries: {}", stats.total_entries);
    info!("  Total Fingerprints: {}", stats.total_fingerprints);
    info!("  Index Size: {}", stats.index_size);
    
    // Search database
    info!("\nSearching database...");
    let results = fingerprinter.search_database("Test").await?;
    info!("✓ Found {} results", results.len());
    for result in &results {
        info!("  - {} - {}", result.artist, result.title);
    }
    
    // Test recognition
    info!("\nTesting recognition...");
    let recognition = fingerprinter.recognize(&analysis).await?;
    if let Some(result) = recognition {
        info!("✓ Recognized: {} - {}", result.track.artist, result.track.title);
        info!("  Confidence: {:.2}%", result.confidence * 100.0);
    } else {
        info!("✓ No match found");
    }
    
    // Export database
    let db_data = fingerprinter.export_database().await?;
    info!("\n✓ Exported database ({} bytes)", db_data.len());
    
    info!("--- Audio Fingerprinting Demo Complete ---\n");
    Ok(())
}

/// Demonstrate audio visualization features
async fn demo_audio_visualization(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Audio Visualization Demo ---");
    
    let visualizer = engine.visualizer();
    
    // Test different visualization types
    let viz_types = [
        VisualizationType::Spectrum,
        VisualizationType::Waveform,
        VisualizationType::Spectrogram,
        VisualizationType::FrequencyBands,
        VisualizationType::CircularSpectrum,
    ];
    
    for viz_type in viz_types {
        info!("\nTesting visualization: {:?}", viz_type);
        visualizer.set_visualization_type(viz_type).await?;
        
        let samples = generate_test_audio(2.0, 48000, 440.0);
        visualizer.update(&samples, 48000).await?;
        
        let frame = visualizer.generate_frame(800, 600).await?;
        info!("✓ Generated frame: {}x{} ({} bytes)", 
            frame.width, frame.height, frame.data.len());
    }
    
    // Test frequency bands
    info!("\nTesting frequency bands...");
    let samples = generate_test_audio(2.0, 48000, 440.0);
    visualizer.update(&samples, 48000).await?;
    
    let bands = visualizer.get_frequency_bands().await?;
    info!("Frequency Bands:");
    info!("  Bass: {:.2}", bands.bass);
    info!("  Low Mid: {:.2}", bands.low_mid);
    info!("  Mid: {:.2}", bands.mid);
    info!("  High Mid: {:.2}", bands.high_mid);
    info!("  Treble: {:.2}", bands.treble);
    
    // Test custom color scheme
    info!("\nTesting custom color scheme...");
    let scheme = ColorScheme {
        primary: (255, 100, 100),
        secondary: (100, 255, 100),
        background: (20, 20, 20),
    };
    visualizer.set_color_scheme(scheme).await?;
    info!("✓ Applied custom color scheme");
    
    // Test different FFT sizes
    info!("\nTesting FFT sizes...");
    for fft_size in [512, 1024, 2048, 4096] {
        visualizer.set_fft_size(fft_size).await?;
        let samples = generate_test_audio(1.0, 48000, 440.0);
        visualizer.update(&samples, 48000).await?;
        let spectrum = visualizer.get_spectrum().await?;
        info!("✓ FFT size {}: {} frequency bins", fft_size, spectrum.len());
    }
    
    info!("--- Audio Visualization Demo Complete ---\n");
    Ok(())
}

/// Demonstrate multi-channel processing features
async fn demo_multichannel_processing(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Multi-Channel Processing Demo ---");
    
    let processor = engine.multichannel_processor();
    
    // Test different output configurations
    let configs = [
        ChannelConfiguration::Mono,
        ChannelConfiguration::Stereo,
        ChannelConfiguration::Surround51,
        ChannelConfiguration::Surround71,
    ];
    
    for config in configs {
        info!("\nTesting output configuration: {:?}", config);
        processor.set_output_config(config).await?;
        
        let samples = generate_stereo_audio(1.0, 48000);
        let processed = processor.process(&samples, 48000).await?;
        info!("✓ Processed {} samples", processed.len());
    }
    
    // Test upmixing
    info!("\nTesting upmixing (Stereo → 5.1)...");
    processor.set_output_config(ChannelConfiguration::Surround51).await?;
    processor.set_upmixing(true).await?;
    
    let stereo_samples = generate_stereo_audio(1.0, 48000);
    let upmixed = processor.process(&stereo_samples, 48000).await?;
    info!("✓ Upmixed {} stereo samples to {} 5.1 samples", 
        stereo_samples.len(), upmixed.len());
    
    // Test downmixing
    info!("\nTesting downmixing (5.1 → Stereo)...");
    processor.set_output_config(ChannelConfiguration::Stereo).await?;
    processor.set_downmixing(true).await?;
    
    let surround_samples = generate_surround_51_audio(1.0, 48000);
    let downmixed = processor.process(&surround_samples, 48000).await?;
    info!("✓ Downmixed {} 5.1 samples to {} stereo samples", 
        surround_samples.len(), downmixed.len());
    
    // Test bass management
    info!("\nTesting bass management...");
    processor.set_output_config(ChannelConfiguration::Surround51).await?;
    processor.set_bass_management(true, 80.0).await?;
    
    let samples = generate_stereo_audio(1.0, 48000);
    let processed = processor.process(&samples, 48000).await?;
    info!("✓ Applied bass management to {} samples", processed.len());
    
    // Test layout detection
    info!("\nTesting layout detection...");
    let mono = vec![0.5; 1000];
    let stereo = vec![0.5; 2000];
    let surround_51 = vec![0.5; 6000];
    
    info!("  Mono layout: {:?}", processor.detect_layout(&mono));
    info!("  Stereo layout: {:?}", processor.detect_layout(&stereo));
    info!("  5.1 layout: {:?}", processor.detect_layout(&surround_51));
    
    info!("--- Multi-Channel Processing Demo Complete ---\n");
    Ok(())
}

/// Demonstrate combined processing with all features
async fn demo_combined_processing(engine: &AdvancedAudioEngine) -> Result<()> {
    info!("--- Combined Processing Demo ---");
    
    // Generate test audio
    let samples = generate_stereo_audio(5.0, 48000);
    info!("Generated {} stereo samples", samples.len());
    
    // Process with all enabled features
    info!("\nProcessing with all advanced features enabled...");
    let processed = engine.process_audio(&samples, 48000).await?;
    info!("✓ Processed {} samples with all features", processed.len());
    
    // Update visualization
    info!("\nUpdating visualization...");
    engine.visualizer().update(&processed, 48000).await?;
    
    let bands = engine.visualizer().get_frequency_bands().await?;
    info!("Frequency Bands after processing:");
    info!("  Bass: {:.2}", bands.bass);
    info!("  Mid: {:.2}", bands.mid);
    info!("  Treble: {:.2}", bands.treble);
    
    // Generate fingerprint
    info!("\nGenerating fingerprint...");
    let analysis = engine.fingerprinter().generate_fingerprint(&processed, 48000).await?;
    info!("✓ Generated fingerprint ({} hashes)", analysis.fingerprint.len());
    
    // Get current configuration
    info!("\nCurrent Configuration:");
    let config = engine.get_config().await;
    info!("  Room Correction: {}", config.room_correction.enabled);
    info!("  Headphone Virtualization: {}", config.headphone_virtualization.enabled);
    info!("  Fingerprinting: {}", config.fingerprinting.enabled);
    info!("  Visualization: {}", config.visualization.enabled);
    info!("  Multi-Channel: {}", config.multichannel.enabled);
    
    info!("--- Combined Processing Demo Complete ---\n");
    Ok(())
}