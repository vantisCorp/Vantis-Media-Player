// Example: Video processing and effects in Vantis Media Player
//
// This example demonstrates advanced video processing capabilities,
// including filters, effects, and post-processing.

use vantis_core::{Player, VideoProcessor, FilterType, EffectType};
use vantis_video::{VideoFrame, VideoFormat};

fn main() {
    println!("Vantis Media Player - Video Processing Example");
    println!("================================================\n");
    
    // Create player and video processor
    let mut player = Player::new();
    let mut processor = VideoProcessor::new();
    
    // Load a video
    player.load("examples/sample_video.mp4")
        .expect("Failed to load video");
    
    // Example 1: Apply brightness filter
    println!("Example 1: Brightness adjustment");
    println!("--------------------------------");
    
    processor.add_filter(FilterType::Brightness(0.2));
    println!("✓ Brightness increased by 20%");
    
    player.play();
    
    // Wait for a few frames
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    player.pause();
    
    println!();
    
    // Example 2: Apply contrast filter
    println!("Example 2: Contrast adjustment");
    println!("-------------------------------");
    
    processor.add_filter(FilterType::Contrast(1.5));
    println!("✓ Contrast increased by 50%");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 3: Apply saturation filter
    println!("Example 3: Saturation adjustment");
    println!("---------------------------------");
    
    processor.add_filter(FilterType::Saturation(1.3));
    println!("✓ Saturation increased by 30%");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 4: Apply blur effect
    println!("Example 4: Blur effect");
    println!("---------------------");
    
    processor.add_effect(EffectType::Blur(3.0));
    println!("✓ Blur effect applied (radius: 3.0)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    processor.remove_effect(EffectType::Blur(3.0));
    println!("✓ Blur effect removed");
    
    println!();
    
    // Example 5: Apply sharpen effect
    println!("Example 5: Sharpen effect");
    println!("------------------------");
    
    processor.add_effect(EffectType::Sharpen(0.5));
    println!("✓ Sharpen effect applied (intensity: 0.5)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 6: Apply vignette effect
    println!("Example 6: Vignette effect");
    println!("-------------------------");
    
    processor.add_effect(EffectType::Vignette {
        intensity: 0.5,
        radius: 0.7,
    });
    println!("✓ Vignette effect applied");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 7: Apply color correction
    println!("Example 7: Color correction");
    println!("---------------------------");
    
    processor.add_filter(FilterType::ColorCorrection {
        temperature: 0.1,    // Warm
        tint: 0.0,
        vibrance: 0.2,
    });
    println!("✓ Color correction applied (warm temperature, increased vibrance)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 8: Apply film grain
    println!("Example 8: Film grain effect");
    println!("----------------------------");
    
    processor.add_effect(EffectType::FilmGrain {
        intensity: 0.15,
        size: 1.0,
    });
    println!("✓ Film grain effect applied");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 9: Custom filter chain
    println!("Example 9: Custom filter chain");
    println!("------------------------------");
    
    // Clear all filters
    processor.clear_filters();
    
    // Create a cinematic look
    processor.add_filter(FilterType::Brightness(-0.05));
    processor.add_filter(FilterType::Contrast(1.1));
    processor.add_filter(FilterType::Saturation(0.9));
    processor.add_effect(EffectType::Vignette {
        intensity: 0.4,
        radius: 0.75,
    });
    
    println!("✓ Cinematic filter chain applied:");
    println!("  - Brightness: -5%");
    println!("  - Contrast: +10%");
    println!("  - Saturation: -10%");
    println!("  - Vignette: 40% intensity");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(3));
    player.pause();
    
    println!();
    
    // Example 10: Frame-by-frame processing
    println!("Example 10: Frame-by-frame processing");
    println!("-------------------------------------");
    
    processor.clear_filters();
    processor.clear_effects();
    
    let mut frame_count = 0;
    let start_time = std::time::Instant::now();
    
    // Process 100 frames
    while frame_count < 100 {
        // Get next frame
        if let Some(frame) = processor.next_frame() {
            // Apply custom processing
            let processed = process_custom_frame(&frame);
            
            // Display frame
            frame_count += 1;
            
            if frame_count % 10 == 0 {
                println!("  Processed {} frames", frame_count);
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();
    
    println!("✓ Processed {} frames in {:.2}s ({:.1} FPS)", 
        frame_count, elapsed.as_secs_f64(), fps);
    
    println!();
    
    // Example 11: Export processed video
    println!("Example 11: Export processed video");
    println!("----------------------------------");
    
    // Apply processing chain
    processor.add_filter(FilterType::Brightness(0.1));
    processor.add_filter(FilterType::Contrast(1.2));
    
    // Export to file
    match processor.export("examples/processed_output.mp4", VideoFormat::H264) {
        Ok(_) => {
            println!("✓ Video exported to: examples/processed_output.mp4");
        }
        Err(e) => {
            println!("✗ Export failed: {}", e);
        }
    }
    
    println!();
    
    // Example 12: Save and load filter presets
    println!("Example 12: Save and load filter presets");
    println!("----------------------------------------");
    
    // Create a preset
    let preset = vec![
        FilterType::Brightness(0.1),
        FilterType::Contrast(1.2),
        FilterType::Saturation(0.95),
    ];
    
    // Save preset
    processor.save_preset("cinematic", &preset)
        .expect("Failed to save preset");
    println!("✓ Preset saved: cinematic");
    
    // Load preset
    match processor.load_preset("cinematic") {
        Ok(loaded_preset) => {
            println!("✓ Preset loaded: {} filters", loaded_preset.len());
        }
        Err(e) => {
            println!("✗ Failed to load preset: {}", e);
        }
    }
    
    // List all presets
    let presets = processor.list_presets();
    println!("Available presets: {:?}", presets);
    
    println!();
    
    // Example 13: Real-time filter adjustment
    println!("Example 13: Real-time filter adjustment");
    println!("---------------------------------------");
    
    player.play();
    
    // Animate brightness
    for i in 0..20 {
        let brightness = (i as f64 / 20.0) * 0.3 - 0.15; // -0.15 to +0.15
        processor.update_filter(FilterType::Brightness(brightness));
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        println!("  Brightness: {:.2}", brightness);
    }
    
    player.pause();
    
    println!();
    
    // Example 14: Apply color overlay
    println!("Example 14: Color overlay");
    println!("------------------------");
    
    processor.add_effect(EffectType::ColorOverlay {
        color: (255, 0, 0),  // Red
        opacity: 0.1,
    });
    println!("✓ Red color overlay applied (10% opacity)");
    
    player.play();
    std::thread::sleep(std::time::Duration::from_secs(2));
    player.pause();
    
    println!();
    
    // Example 15: Performance monitoring
    println!("Example 15: Performance monitoring");
    println!("----------------------------------");
    
    processor.clear_filters();
    processor.clear_effects();
    
    let stats = processor.get_statistics();
    println!("Video Processing Statistics:");
    println!("  Frames processed: {}", stats.frames_processed);
    println!("  Average processing time: {:.2}ms", stats.avg_processing_time_ms);
    println!("  Peak processing time: {:.2}ms", stats.peak_processing_time_ms);
    println!("  Filters active: {}", stats.active_filters);
    println!("  Effects active: {}", stats.active_effects);
    println!("  GPU memory usage: {:.2}MB", stats.gpu_memory_mb);
    
    println!();
    println!("Video processing examples completed!");
    
    player.stop();
}

/// Custom frame processing function
fn process_custom_frame(frame: &VideoFrame) -> VideoFrame {
    // Example: Apply simple threshold effect
    let threshold = 128u8;
    
    // This is a simplified example - in reality, you'd process
    // the pixel data directly
    let mut processed = frame.clone();
    
    // Process would happen here
    // for pixel in processed.pixels_mut() {
    //     let gray = (pixel.r + pixel.g + pixel.b) / 3;
    //     let value = if gray > threshold { 255 } else { 0 };
    //     pixel.r = value;
    //     pixel.g = value;
    //     pixel.b = value;
    // }
    
    processed
}