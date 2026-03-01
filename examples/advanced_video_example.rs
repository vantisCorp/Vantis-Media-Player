//! Advanced Video Features Example
//! 
//! This example demonstrates all advanced video features including:
//! - Video stabilization
//! - Frame interpolation
//! - Video denoising
//! - Color grading
//! - Video comparison

use anyhow::Result;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use tracing_subscriber;
use image::{RgbImage, Rgb};

use vantis_advanced_video::{
    AdvancedVideoEngine, AdvancedVideoConfig,
    StabilizationConfig, FrameInterpolationConfig, DenoisingConfig,
    ColorGradingConfig, ComparisonConfig,
    MotionAnalysisMethod, InterpolationMethod, DenoisingMethod,
    ComparisonMethod, ColorAdjustments,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=== Vantis Advanced Video Features Example ===\n");

    // Create advanced video configuration
    let config = create_advanced_config();
    
    // Initialize the engine
    info!("Initializing Advanced Video Engine...");
    let engine = AdvancedVideoEngine::new(config)?;
    info!("✓ Engine initialized successfully\n");

    // Demonstrate video stabilization
    demo_video_stabilization(&engine).await?;

    // Demonstrate frame interpolation
    demo_frame_interpolation(&engine).await?;

    // Demonstrate video denoising
    demo_video_denoising(&engine).await?;

    // Demonstrate color grading
    demo_color_grading(&engine).await?;

    // Demonstrate video comparison
    demo_video_comparison(&engine).await?;

    // Demonstrate combined processing
    demo_combined_processing(&engine).await?;

    info!("\n=== All demonstrations completed successfully ===");
    Ok(())
}

/// Create advanced video configuration
fn create_advanced_config() -> AdvancedVideoConfig {
    AdvancedVideoConfig {
        stabilization: StabilizationConfig {
            enabled: true,
            strength: 0.7,
            motion_analysis: MotionAnalysisMethod::OpticalFlow,
            rolling_shutter_correction: true,
            crop_to_stabilize: true,
            max_crop: 0.1,
        },
        frame_interpolation: FrameInterpolationConfig {
            enabled: true,
            target_fps: 60,
            method: InterpolationMethod::AI,
            ai_model: "default".to_string(),
            quality: 0.8,
        },
        denoising: DenoisingConfig {
            enabled: true,
            method: DenoisingMethod::AI,
            strength: 0.6,
            spatial_denoising: true,
            temporal_denoising: true,
            detail_preservation: 0.8,
        },
        color_grading: ColorGradingConfig {
            enabled: true,
            preset: "cinematic".to_string(),
            custom_adjustments: ColorAdjustments::default(),
            lut_path: None,
        },
        comparison: ComparisonConfig {
            method: ComparisonMethod::SSIM,
            generate_difference_image: true,
        },
    }
}

/// Generate test video frame
fn generate_test_frame(width: u32, height: u32, pattern: u32) -> RgbImage {
    let mut frame = RgbImage::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = ((pattern as f32) % 255.0) as u8;
            frame.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    frame
}

/// Add noise to frame
fn add_noise(frame: &RgbImage, noise_level: f32) -> RgbImage {
    let mut noisy = frame.clone();
    
    for pixel in noisy.pixels_mut() {
        let noise = (rand::random::<f32>() - 0.5) * 2.0 * noise_level * 255.0;
        pixel[0] = (pixel[0] as f32 + noise).clamp(0.0, 255.0) as u8;
        pixel[1] = (pixel[1] as f32 + noise).clamp(0.0, 255.0) as u8;
        pixel[2] = (pixel[2] as f32 + noise).clamp(0.0, 255.0) as u8;
    }
    
    noisy
}

/// Apply shake to frame (simulate camera shake)
fn apply_shake(frame: &RgbImage, shake_amount: i32) -> RgbImage {
    let mut shaken = RgbImage::new(frame.width(), frame.height());
    
    let dx = (rand::random::<i32>() - 128) * shake_amount / 128;
    let dy = (rand::random::<i32>() - 128) * shake_amount / 128;
    
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let src_x = x as i32 + dx;
            let src_y = y as i32 + dy;
            
            if src_x >= 0 && src_x < frame.width() as i32 && src_y >= 0 && src_y < frame.height() as i32 {
                let pixel = frame.get_pixel(src_x as u32, src_y as u32);
                shaken.put_pixel(x, y, *pixel);
            }
        }
    }
    
    shaken
}

/// Demonstrate video stabilization
async fn demo_video_stabilization(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Video Stabilization Demo ###\n");
    
    // Generate test frames with shake
    let original = generate_test_frame(640, 480, 0);
    let shaken = apply_shake(&original, 10);
    
    info!("Original frame: {}x{}", original.width(), original.height());
    info!("Shaken frame: {}x{}", shaken.width(), shaken.height());
    
    // Stabilize the frame
    let result = engine.stabilizer().stabilize(&shaken).await?;
    
    info!("Stabilized frame: {}x{}", result.frame.width(), result.frame.height());
    info!("Camera transform: tx={:.2}, ty={:.2}, rotation={:.3}°, scale={:.3}",
        result.transform.tx,
        result.transform.ty,
        result.transform.rotation.to_degrees(),
        result.transform.scale
    );
    info!("Crop rectangle: x={}, y={}, width={}, height={}",
        result.crop.x, result.crop.y, result.crop.width, result.crop.height
    );
    
    info!("✓ Video stabilization completed\n");
    Ok(())
}

/// Demonstrate frame interpolation
async fn demo_frame_interpolation(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Frame Interpolation Demo ###\n");
    
    // Generate two test frames
    let frame1 = generate_test_frame(640, 480, 0);
    let frame2 = generate_test_frame(640, 480, 50);
    
    info!("Frame 1: {}x{}", frame1.width(), frame1.height());
    info!("Frame 2: {}x{}", frame2.width(), frame2.height());
    
    // Interpolate 2 frames between them
    let result = engine.interpolator().interpolate(&frame1, &frame2, 2).await?;
    
    info!("Interpolated {} frames", result.frames.len());
    info!("Quality score: {:.2}", result.quality);
    
    for (i, frame) in result.frames.iter().enumerate() {
        info!("  Frame {}: {}x{}", i + 1, frame.width(), frame.height());
    }
    
    info!("✓ Frame interpolation completed\n");
    Ok(())
}

/// Demonstrate video denoising
async fn demo_video_denoising(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Video Denoising Demo ###\n");
    
    // Generate test frame with noise
    let original = generate_test_frame(640, 480, 0);
    let noisy = add_noise(&original, 0.1);
    
    info!("Original frame: {}x{}", original.width(), original.height());
    info!("Noisy frame: {}x{}", noisy.width(), noisy.height());
    
    // Denoise the frame
    let denoised = engine.denoiser().process(&noisy).await?;
    
    info!("Denoised frame: {}x{}", denoised.width(), denoised.height());
    
    // Calculate noise reduction
    let original_noise = calculate_noise_level(&original);
    let noisy_noise = calculate_noise_level(&noisy);
    let denoised_noise = calculate_noise_level(&denoised);
    
    info!("Noise levels:");
    info!("  Original: {:.4}", original_noise);
    info!("  Noisy: {:.4}", noisy_noise);
    info!("  Denoised: {:.4}", denoised_noise);
    info!("  Noise reduction: {:.1}%", 
        ((noisy_noise - denoised_noise) / noisy_noise * 100.0)
    );
    
    info!("✓ Video denoising completed\n");
    Ok(())
}

/// Demonstrate color grading
async fn demo_color_grading(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Color Grading Demo ###\n");
    
    // Generate test frame
    let original = generate_test_frame(640, 480, 0);
    
    info!("Original frame: {}x{}", original.width(), original.height());
    
    // Apply cinematic preset
    let cinematic = engine.color_grader().apply_preset(&original, "cinematic").await?;
    info!("✓ Applied 'cinematic' preset");
    
    // Apply vivid preset
    let vivid = engine.color_grader().apply_preset(&original, "vivid").await?;
    info!("✓ Applied 'vivid' preset");
    
    // Apply custom adjustments
    let adjustments = ColorAdjustments {
        brightness: -0.05,
        contrast: 0.15,
        saturation: 0.2,
        temperature: -10.0,
        vibrance: 0.1,
        ..Default::default()
    };
    
    let custom = engine.color_grader().apply_adjustments(&original, &adjustments).await?;
    info!("✓ Applied custom adjustments");
    
    // List available presets
    let presets = engine.color_grader().get_presets().await;
    info!("Available presets: {}", presets.join(", "));
    
    // Add custom preset
    engine.color_grader().add_preset("my_custom".to_string(), adjustments.clone()).await?;
    info!("✓ Added custom preset 'my_custom'");
    
    // Apply custom preset
    let my_preset = engine.color_grader().apply_preset(&original, "my_custom").await?;
    info!("✓ Applied custom preset 'my_custom'");
    
    info!("✓ Color grading completed\n");
    Ok(())
}

/// Demonstrate video comparison
async fn demo_video_comparison(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Video Comparison Demo ###\n");
    
    // Generate two test frames
    let frame1 = generate_test_frame(640, 480, 0);
    let frame2 = generate_test_frame(640, 480, 10);
    
    info!("Frame 1: {}x{}", frame1.width(), frame1.height());
    info!("Frame 2: {}x{}", frame2.width(), frame2.height());
    
    // Compare frames
    let result = engine.comparator().compare(&frame1, &frame2).await?;
    
    info!("Comparison results:");
    info!("  Similarity: {:.4}", result.similarity);
    info!("  Difference: {:.4}", result.difference);
    info!("  PSNR: {:.2} dB", result.psnr);
    info!("  SSIM: {:.4}", result.ssim);
    info!("  MSE: {:.2}", result.mse);
    
    if result.difference_image.is_some() {
        info!("  Difference image: Generated ({}x{})",
            result.difference_image.as_ref().unwrap().width(),
            result.difference_image.as_ref().unwrap().height()
        );
    }
    
    // Compare identical frames
    let identical_result = engine.comparator().compare(&frame1, &frame1).await?;
    info!("\nIdentical frames comparison:");
    info!("  Similarity: {:.4}", identical_result.similarity);
    info!("  PSNR: {:.2} dB", identical_result.psnr);
    info!("  SSIM: {:.4}", identical_result.ssim);
    
    info!("✓ Video comparison completed\n");
    Ok(())
}

/// Demonstrate combined processing
async fn demo_combined_processing(engine: &AdvancedVideoEngine) -> Result<()> {
    info!("### Combined Processing Demo ###\n");
    
    // Generate test frame with noise and shake
    let original = generate_test_frame(640, 480, 0);
    let noisy = add_noise(&original, 0.1);
    let shaken = apply_shake(&noisy, 5);
    
    info!("Original frame: {}x{}", original.width(), original.height());
    info!("Noisy and shaken frame: {}x{}", shaken.width(), shaken.height());
    
    // Apply combined processing
    let stabilized = engine.stabilizer().stabilize(&shaken).await?;
    info!("✓ Stabilized");
    
    let denoised = engine.denoiser().process(&stabilized.frame).await?;
    info!("✓ Denoised");
    
    let graded = engine.color_grader().apply_preset(&denoised, "cinematic").await?;
    info!("✓ Color graded");
    
    info!("Final frame: {}x{}", graded.width(), graded.height());
    
    // Compare with original
    let comparison = engine.comparator().compare(&original, &graded).await?;
    info!("Comparison with original:");
    info!("  Similarity: {:.4}", comparison.similarity);
    info!("  PSNR: {:.2} dB", comparison.psnr);
    info!("  SSIM: {:.4}", comparison.ssim);
    
    info!("✓ Combined processing completed\n");
    Ok(())
}

/// Calculate noise level of frame
fn calculate_noise_level(frame: &RgbImage) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;
    
    for y in 1..frame.height() - 1 {
        for x in 1..frame.width() - 1 {
            let center = frame.get_pixel(x, y);
            let left = frame.get_pixel(x - 1, y);
            let right = frame.get_pixel(x + 1, y);
            let top = frame.get_pixel(x, y - 1);
            let bottom = frame.get_pixel(x, y + 1);
            
            let diff = (
                (center[0] as f32 - left[0] as f32).abs() +
                (center[0] as f32 - right[0] as f32).abs() +
                (center[1] as f32 - top[1] as f32).abs() +
                (center[1] as f32 - bottom[1] as f32).abs()
            ) / 4.0;
            
            sum += diff;
            count += 1;
        }
    }
    
    if count > 0 {
        sum / count as f32 / 255.0
    } else {
        0.0
    }
}