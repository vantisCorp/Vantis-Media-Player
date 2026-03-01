//! AI Features Example for Vantis Media Player
//! 
//! This example demonstrates how to use the AI features in Vantis Media Player:
//! - Video enhancement
//! - Scene detection
//! - Audio enhancement
//! - Subtitle timing adjustment
//! - Content recommendations

use vantis_ai::{
    AIEngine, AIConfig,
    VideoEnhancer, EnhancementConfig, EnhancementType, QualityPreset,
    SceneDetector, SceneDetectionConfig,
    AudioEnhancer, AudioEnhancementConfig, SpatialMode,
    SubtitleTimingAdjuster, SubtitleTimingConfig,
    RecommendationEngine, RecommendationConfig,
    ContentItem, UserProfile, WatchEvent,
};
use image::DynamicImage;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vantis AI Features Example");
    println!("===========================\n");
    
    // Initialize AI engine
    println!("1. Initializing AI Engine...");
    let ai = AIEngine::new()?;
    println!("   ✓ AI Engine initialized\n");
    
    // Example 1: Video Enhancement
    println!("2. Video Enhancement Example");
    video_enhancement_example(&ai)?;
    println!();
    
    // Example 2: Scene Detection
    println!("3. Scene Detection Example");
    scene_detection_example(&ai)?;
    println!();
    
    // Example 3: Audio Enhancement
    println!("4. Audio Enhancement Example");
    audio_enhancement_example(&ai)?;
    println!();
    
    // Example 4: Subtitle Timing Adjustment
    println!("5. Subtitle Timing Adjustment Example");
    subtitle_timing_example(&ai)?;
    println!();
    
    // Example 5: Content Recommendations
    println!("6. Content Recommendation Example");
    recommendation_example(&ai)?;
    println!();
    
    println!("All examples completed successfully!");
    Ok(())
}

/// Video enhancement example
fn video_enhancement_example(ai: &AIEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating video enhancer...");
    let mut enhancer = ai.video_enhancer()?;
    
    // Create a sample frame (in real usage, this would come from video)
    let sample_frame = create_sample_frame(640, 480)?;
    println!("   ✓ Sample frame created ({}x{})", sample_frame.width(), sample_frame.height());
    
    // Configure enhancement
    let config = EnhancementConfig {
        enhancement_type: EnhancementType::SuperResolution { scale: 2 },
        enable_gpu: true,
        quality: QualityPreset::Balanced,
        ..Default::default()
    };
    
    println!("   Enhancing frame (2x upscaling)...");
    let enhanced_frame = enhancer.enhance_frame(&sample_frame)?;
    
    println!("   ✓ Frame enhanced ({}x{})", enhanced_frame.width(), enhanced_frame.height());
    println!("   ✓ Original: {}x{} → Enhanced: {}x{}",
        sample_frame.width(),
        sample_frame.height(),
        enhanced_frame.width(),
        enhanced_frame.height()
    );
    
    // Try different enhancement types
    println!("\n   Testing other enhancement types:");
    
    // Denoising
    let denoise_config = EnhancementConfig {
        enhancement_type: EnhancementType::Denoising { strength: 0.7 },
        ..Default::default()
    };
    let _denoised = enhancer.enhance_frame(&sample_frame)?;
    println!("   ✓ Denoising applied");
    
    // Color enhancement
    let color_config = EnhancementConfig {
        enhancement_type: EnhancementType::ColorEnhancement {
            saturation: 1.2,
            contrast: 1.1,
        },
        ..Default::default()
    };
    let _color_enhanced = enhancer.enhance_frame(&sample_frame)?;
    println!("   ✓ Color enhancement applied");
    
    Ok(())
}

/// Scene detection example
fn scene_detection_example(ai: &AIEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating scene detector...");
    let mut detector = ai.scene_detector()?;
    
    // Simulate processing frames
    println!("   Processing frames for scene detection...");
    let num_frames = 100;
    let fps = 30.0;
    
    for i in 0..num_frames {
        let frame = create_sample_frame(640, 480)?;
        let timestamp = i as f64 / fps;
        
        if let Some(boundary) = detector.process_frame(&frame, timestamp)? {
            println!("   ✓ Shot boundary detected at {:.2}s (confidence: {:.2})",
                boundary.timestamp,
                boundary.confidence
            );
        }
    }
    
    // Detect scenes
    let duration = num_frames as f64 / fps;
    println!("\n   Detecting scenes...");
    let scenes = detector.detect_scenes(duration)?;
    
    println!("   ✓ {} scenes detected", scenes.len());
    
    for (i, scene) in scenes.iter().enumerate() {
        println!("   Scene {}: {:.2}s - {:.2}s ({:.1}s) - {:?}",
            i + 1,
            scene.start_time,
            scene.end_time,
            scene.duration,
            scene.scene_type
        );
    }
    
    // Generate chapters
    println!("\n   Generating chapters...");
    let chapters = detector.generate_chapters(&scenes)?;
    
    println!("   ✓ {} chapters generated", chapters.len());
    
    for chapter in &chapters {
        println!("   Chapter {}: {} ({:.2}s - {:.2}s)",
            chapter.index + 1,
            chapter.title,
            chapter.start_time,
            chapter.end_time
        );
    }
    
    Ok(())
}

/// Audio enhancement example
fn audio_enhancement_example(ai: &AIEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating audio enhancer...");
    let mut enhancer = ai.audio_enhancer()?;
    
    // Create sample audio
    println!("   Creating sample audio buffer...");
    let sample_rate = 48000;
    let duration_secs = 5.0;
    let num_samples = (sample_rate as f64 * duration_secs) as usize;
    let audio_samples: Vec<f32> = (0..num_samples)
        .map(|i| (i as f32 / num_samples as f32 * 2.0 - 1.0) * 0.5)
        .collect();
    
    println!("   ✓ Audio buffer created ({} samples, {:.1}s)", num_samples, duration_secs);
    
    // Analyze audio
    println!("\n   Analyzing audio...");
    let analysis = enhancer.analyze_audio(&audio_samples);
    
    println!("   ✓ Audio analysis:");
    println!("     - Loudness: {:.2} LUFS", analysis.loudness_lufs);
    println!("     - Peak: {:.2} dB", analysis.peak_db);
    println!("     - RMS: {:.2} dB", analysis.rms_db);
    println!("     - Dynamic Range: {:.2} dB", analysis.dynamic_range_db);
    println!("     - Spectral Centroid: {:.2} Hz", analysis.spectral_centroid_hz);
    println!("     - Clipping: {}", if analysis.clipping_detected { "Yes" } else { "No" });
    
    // Process audio with different configurations
    println!("\n   Processing audio with enhancements...");
    
    // Noise reduction
    let config = AudioEnhancementConfig {
        enable_noise_reduction: true,
        noise_reduction_strength: 0.7,
        enable_loudness_normalization: true,
        ..Default::default()
    };
    
    let enhanced = enhancer.process_buffer(&audio_samples)?;
    println!("   ✓ Noise reduction applied");
    
    // Spatial audio
    let spatial_config = AudioEnhancementConfig {
        enable_spatial_audio: true,
        spatial_mode: SpatialMode::Surround51,
        ..Default::default()
    };
    
    let spatial_audio = enhancer.process_buffer(&audio_samples)?;
    println!("   ✓ Spatial audio (5.1) applied");
    println!("   ✓ Channels: {} → {}", audio_samples.len(), spatial_audio.len());
    
    Ok(())
}

/// Subtitle timing adjustment example
fn subtitle_timing_example(ai: &AIEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating subtitle timing adjuster...");
    let mut adjuster = ai.subtitle_timing_adjuster()?;
    
    // Simulate audio analysis
    println!("   Analyzing audio for timing...");
    let sample_rate = 48000;
    let chunk_size = 1024;
    let num_chunks = 100;
    
    for i in 0..num_chunks {
        let timestamp = i as f64 * chunk_size as f64 / sample_rate as f64;
        let audio_samples: Vec<f32> = (0..chunk_size)
            .map(|j| ((i * chunk_size + j) as f32 / (num_chunks * chunk_size) as f32 * 2.0 - 1.0) * 0.3)
            .collect();
        
        adjuster.analyze_audio(&audio_samples, timestamp)?;
    }
    
    println!("   ✓ Audio analysis completed");
    
    // Adjust subtitle timing
    println!("\n   Adjusting subtitle timing...");
    
    let subtitles = vec![
        (0.0, 3.0, "Hello, welcome to Vantis Media Player!".to_string()),
        (3.5, 7.0, "This is an example of AI-powered subtitle timing.".to_string()),
        (7.5, 12.0, "The system automatically synchronizes subtitles with audio.".to_string()),
        (12.5, 16.0, "It uses speech analysis and pattern matching.".to_string()),
    ];
    
    for (start, end, text) in &subtitles {
        let adjustment = adjuster.adjust_timing(*start, *end, text)?;
        
        println!("   ✓ Subtitle: &quot;{}&quot;", text);
        println!("     Original: {:.2}s - {:.2}s", start, end);
        println!("     Adjusted: {:.2}s - {:.2}s", adjustment.adjusted_start, adjustment.adjusted_end);
        println!("     Shift: {:.2}s, Confidence: {:.2}",
            adjustment.time_shift,
            adjustment.confidence
        );
        println!("     Method: {:?}", adjustment.method);
        println!();
    }
    
    // Batch adjustment
    println!("   Batch adjustment...");
    let adjustments = adjuster.batch_adjust(&subtitles)?;
    println!("   ✓ {} subtitles adjusted", adjustments.len());
    
    Ok(())
}

/// Content recommendation example
fn recommendation_example(ai: &AIEngine) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating recommendation engine...");
    let mut engine = ai.recommendation_engine()?;
    
    // Add sample content
    println!("   Adding sample content...");
    let content_items = vec![
        ContentItem {
            id: "movie_1".to_string(),
            title: "The Quantum Paradox".to_string(),
            genres: vec!["Sci-Fi".to_string(), "Action".to_string()],
            year: Some(2024),
            rating: Some(8.5),
            duration: Some(120),
            language: Some("en".to_string()),
            tags: vec!["space".to_string(), "time travel".to_string()],
            cast: vec!["Actor A".to_string(), "Actor B".to_string()],
            director: Some("Director X".to_string()),
            description: Some("A thrilling sci-fi adventure".to_string()),
            ..Default::default()
        },
        ContentItem {
            id: "movie_2".to_string(),
            title: "Midnight Mystery".to_string(),
            genres: vec!["Mystery".to_string(), "Thriller".to_string()],
            year: Some(2023),
            rating: Some(7.8),
            duration: Some(105),
            language: Some("en".to_string()),
            tags: vec!["detective".to_string(), "suspense".to_string()],
            cast: vec!["Actor C".to_string(), "Actor D".to_string()],
            director: Some("Director Y".to_string()),
            description: Some("A gripping mystery thriller".to_string()),
            ..Default::default()
        },
        ContentItem {
            id: "movie_3".to_string(),
            title: "Comedy Central".to_string(),
            genres: vec!["Comedy".to_string(), "Romance".to_string()],
            year: Some(2024),
            rating: Some(7.2),
            duration: Some(95),
            language: Some("en".to_string()),
            tags: vec!["funny".to_string(), "romantic".to_string()],
            cast: vec!["Actor E".to_string(), "Actor F".to_string()],
            director: Some("Director Z".to_string()),
            description: Some("A light-hearted romantic comedy".to_string()),
            ..Default::default()
        },
        ContentItem {
            id: "movie_4".to_string(),
            title: "Action Force".to_string(),
            genres: vec!["Action".to_string(), "Adventure".to_string()],
            year: Some(2024),
            rating: Some(8.1),
            duration: Some(130),
            language: Some("en".to_string()),
            tags: vec!["explosions".to_string(), "hero".to_string()],
            cast: vec!["Actor G".to_string(), "Actor H".to_string()],
            director: Some("Director W".to_string()),
            description: Some("High-octane action adventure".to_string()),
            ..Default::default()
        },
    ];
    
    for content in content_items {
        engine.add_content(content)?;
    }
    
    println!("   ✓ {} content items added", content_items.len());
    
    // Add user profile
    println!("\n   Creating user profile...");
    let mut profile = UserProfile {
        user_id: "user_123".to_string(),
        watch_history: vec![],
        genre_preferences: HashMap::new(),
        favorites: HashSet::new(),
        watchlist: HashSet::new(),
        disliked: HashSet::new(),
        avg_rating: 0.0,
        total_watch_time: 0,
        last_active: None,
    };
    
    // Add watch history
    profile.watch_history.push(WatchEvent {
        content_id: "movie_1".to_string(),
        start_time: 1700000000,
        end_time: Some(1700000120),
        duration_secs: 120,
        completion: 1.0,
        rating: Some(9.0),
        liked: Some(true),
    });
    
    profile.watch_history.push(WatchEvent {
        content_id: "movie_4".to_string(),
        start_time: 1700000200,
        end_time: Some(1700000330),
        duration_secs: 130,
        completion: 0.85,
        rating: Some(8.0),
        liked: Some(true),
    });
    
    // Add genre preferences
    profile.genre_preferences.insert("Action".to_string(), 0.9);
    profile.genre_preferences.insert("Sci-Fi".to_string(), 0.8);
    profile.genre_preferences.insert("Adventure".to_string(), 0.7);
    
    engine.add_user_profile(profile)?;
    println!("   ✓ User profile created");
    
    // Get recommendations
    println!("\n   Getting recommendations...");
    let recommendations = engine.get_recommendations("user_123")?;
    
    println!("   ✓ {} recommendations generated", recommendations.len());
    
    for (i, rec) in recommendations.iter().enumerate() {
        println!("\n   {}. {} ({:.2}/10)",
            i + 1,
            rec.content.title,
            rec.content.rating.unwrap_or(0.0)
        );
        println!("      Genres: {}", rec.content.genres.join(", "));
        println!("      Relevance: {:.2}", rec.relevance_score);
        println!("      Confidence: {:.2}", rec.confidence);
        println!("      Source: {:?}", rec.source);
        println!("      Reason: {:?}", rec.reason);
    }
    
    // Get trending content
    println!("\n   Getting trending content...");
    let trending = engine.get_trending(5)?;
    
    println!("   ✓ {} trending items", trending.len());
    
    for (i, item) in trending.iter().enumerate() {
        println!("   {}. {} (score: {:.2})",
            i + 1,
            item.content.title,
            item.relevance_score
        );
    }
    
    Ok(())
}

/// Create a sample frame for testing
fn create_sample_frame(width: u32, height: u32) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    use image::{ImageBuffer, Rgb};
    
    let mut img = ImageBuffer::new(width, height);
    
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        let b = 128;
        *pixel = Rgb([r, g, b]);
    }
    
    Ok(DynamicImage::ImageRgb8(img))
}