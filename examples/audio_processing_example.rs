// Example: Audio processing and effects in Vantis Media Player
//
// This example demonstrates advanced audio processing capabilities,
// including equalization, effects, and audio analysis.

use vantis_core::{Player, AudioProcessor, EffectType};
use vantis_audio::{AudioFormat, AudioSample};

fn main() {
    println!("Vantis Media Player - Audio Processing Example");
    println!("==============================================\n");
    
    // Create player and audio processor
    let mut player = Player::new();
    let mut processor = AudioProcessor::new();
    
    // Load a video
    player.load("examples/sample_video.mp4")
        .expect("Failed to load video");
    
    // Example 1: Adjust volume
    println!("Example 1: Volume control");
    println!("-------------------------");
    
    player.set_volume(0.5);
    println!("✓ Volume set to 50%");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    player.set_volume(0.8);
    println!("✓ Volume increased to 80%");
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 2: Mute/Unmute
    println!("Example 2: Mute control");
    println!("-----------------------");
    
    player.set_mute(true);
    println!("✓ Audio muted");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    player.set_mute(false);
    println!("✓ Audio unmuted");
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 3: Apply equalizer
    println!("Example 3: Equalizer bands");
    println!("-------------------------");
    
    // Configure equalizer with 10 bands
    let bands = vec![
        (32.0, 0.0),    // 32 Hz
        (64.0, 0.0),    // 64 Hz
        (125.0, 2.0),   // 125 Hz
        (250.0, 3.0),   // 250 Hz
        (500.0, 0.0),   // 500 Hz
        (1000.0, 0.0),  // 1 kHz
        (2000.0, 2.0),  // 2 kHz
        (4000.0, 4.0),  // 4 kHz
        (8000.0, 5.0),  // 8 kHz
        (16000.0, 3.0), // 16 kHz
    ];
    
    processor.set_equalizer(&bands);
    println!("✓ Equalizer configured:");
    for (freq, gain) in &bands {
        println!("    {:.0} Hz: {:+.1} dB", freq, gain);
    }
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 4: Apply bass boost
    println!("Example 4: Bass boost");
    println!("---------------------");
    
    processor.add_effect(EffectType::BassBoost(6.0));
    println!("✓ Bass boost applied (+6 dB)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 5: Apply treble boost
    println!("Example 5: Treble boost");
    println!("-----------------------");
    
    processor.add_effect(EffectType::TrebleBoost(4.0));
    println!("✓ Treble boost applied (+4 dB)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 6: Apply reverb effect
    println!("Example 6: Reverb effect");
    println!("-----------------------");
    
    processor.add_effect(EffectType::Reverb {
        room_size: 0.5,
        damping: 0.5,
        wet_level: 0.3,
        dry_level: 0.7,
    });
    println!("✓ Reverb effect applied");
    println!("  Room size: 0.5");
    println!("  Damping: 0.5");
    println!("  Wet level: 30%");
    println!("  Dry level: 70%");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(4));
    player.pause();
    
    processor.remove_effect(EffectType::Reverb {
        room_size: 0.5,
        damping: 0.5,
        wet_level: 0.3,
        dry_level: 0.7,
    });
    println!("✓ Reverb effect removed");
    
    println!();
    
    // Example 7: Apply chorus effect
    println!("Example 7: Chorus effect");
    println!("------------------------");
    
    processor.add_effect(EffectType::Chorus {
        rate: 1.5,
        depth: 0.5,
        feedback: 0.3,
        delay: 0.02,
    });
    println!("✓ Chorus effect applied");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 8: Apply delay effect
    println!("Example 8: Delay effect");
    println!("----------------------");
    
    processor.add_effect(EffectType::Delay {
        time: 0.3,
        feedback: 0.4,
        mix: 0.3,
    });
    println!("✓ Delay effect applied (300ms, 40% feedback, 30% mix)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(4));
    player.pause();
    
    processor.clear_effects();
    
    println!();
    
    // Example 9: Audio normalization (EBU R128)
    println!("Example 9: Loudness normalization");
    println!("----------------------------------");
    
    processor.enable_loudness_normalization(-16.0);
    println!("✓ Loudness normalization enabled");
    println!("  Target loudness: -16 LUFS (EBU R128)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 10: Dynamic range compression
    println!("Example 10: Compression");
    println!("-----------------------");
    
    processor.add_effect(EffectType::Compressor {
        threshold: -20.0,
        ratio: 4.0,
        attack: 0.005,
        release: 0.1,
    });
    println!("✓ Compressor applied");
    println!("  Threshold: -20 dB");
    println!("  Ratio: 4:1");
    println!("  Attack: 5ms");
    println!("  Release: 100ms");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 11: Spatial audio
    println!("Example 11: Spatial audio");
    println!("-------------------------");
    
    processor.enable_spatial_audio(true);
    println!("✓ Spatial audio enabled");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 12: Audio analysis
    println!("Example 12: Audio analysis");
    println!("----------------------------");
    
    match processor.analyze_audio() {
        Ok(analysis) => {
            println!("Audio Analysis Results:");
            println!("  Peak level: {:.1} dB", analysis.peak_level);
            println!("  RMS level: {:.1} dB", analysis.rms_level);
            println!("  Loudness: {:.1} LUFS", analysis.loudness);
            println!("  Dynamic range: {:.1} dB", analysis.dynamic_range);
            println!("  Frequency spectrum:");
            for (freq, magnitude) in &analysis.frequency_spectrum {
                println!("    {:.0} Hz: {:.1} dB", freq, magnitude);
            }
        }
        Err(e) => {
            println!("✗ Analysis failed: {}", e);
        }
    }
    
    println!();
    
    // Example 13: Export processed audio
    println!("Example 13: Export processed audio");
    println!("------------------------------------");
    
    // Apply effects
    processor.add_effect(EffectType::Compressor {
        threshold: -20.0,
        ratio: 4.0,
        attack: 0.005,
        release: 0.1,
    });
    
    // Export to file
    match processor.export("examples/processed_audio.wav", AudioFormat::WAV) {
        Ok(_) => {
            println!("✓ Audio exported to: examples/processed_audio.wav");
        }
        Err(e) => {
            println!("✗ Export failed: {}", e);
        }
    }
    
    println!();
    
    // Example 14: Save and load audio presets
    println!("Example 14: Save and load audio presets");
    println!("---------------------------------------");
    
    // Create a preset
    let preset = vec![
        EffectType::BassBoost(6.0),
        EffectType::Compressor {
            threshold: -20.0,
            ratio: 4.0,
            attack: 0.005,
            release: 0.1,
        },
    ];
    
    // Save preset
    processor.save_preset("bass_boost", &preset)
        .expect("Failed to save preset");
    println!("✓ Preset saved: bass_boost");
    
    // Load preset
    match processor.load_preset("bass_boost") {
        Ok(loaded_preset) => {
            println!("✓ Preset loaded: {} effects", loaded_preset.len());
        }
        Err(e) => {
            println!("✗ Failed to load preset: {}", e);
        }
    }
    
    // List all presets
    let presets = processor.list_presets();
    println!("Available presets: {:?}", presets);
    
    println!();
    
    // Example 15: Performance monitoring
    println!("Example 15: Performance monitoring");
    println!("-----------------------------------");
    
    let stats = processor.get_statistics();
    println!("Audio Processing Statistics:");
    println!("  Samples processed: {}", stats.samples_processed);
    println!("  Average processing time: {:.2}ms", stats.avg_processing_time_ms);
    println!("  Peak processing time: {:.2}ms", stats.peak_processing_time_ms);
    println!("  Effects active: {}", stats.active_effects);
    println!("  Latency: {:.2}ms", stats.latency_ms);
    println!("  CPU usage: {:.1}%", stats.cpu_usage);
    
    println!();
    println!("Audio processing examples completed!");
    
    player.stop();
}