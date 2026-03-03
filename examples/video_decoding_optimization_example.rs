//! Video Decoding Optimization Example
//!
//! This example demonstrates the video decoding optimization features of Vantis Media Player:
//! - Hardware-accelerated decoding
//! - Frame buffer management
//! - Frame skipping strategies
//! - GPU-CPU synchronization
//! - Adaptive quality adjustment

use std::time::Duration;
use vantisvideo::decoding_optimization::{
    AdaptiveQualityController, DecodingOptimizationConfig, DecodingStatistics,
    FrameBufferManager, FrameBufferEntry, FrameSkippingController, FrameSkippingStrategy,
    GpuCpuSyncManager, HardwareDecoder, VideoDecodingOptimizer,
};

fn main() -> anyhow::Result<()> {
    println!("=== Vantis Media Player - Video Decoding Optimization Example ===\n");

    // Example 1: Basic decoding optimization
    println!("Example 1: Basic Decoding Optimization");
    basic_decoding_optimization()?;

    // Example 2: Frame buffer management
    println!("\nExample 2: Frame Buffer Management");
    frame_buffer_management_example()?;

    // Example 3: Frame skipping strategies
    println!("\nExample 3: Frame Skipping Strategies");
    frame_skipping_strategies_example()?;

    // Example 4: GPU-CPU synchronization
    println!("\nExample 4: GPU-CPU Synchronization");
    gpu_cpu_sync_example()?;

    // Example 5: Adaptive quality control
    println!("\nExample 5: Adaptive Quality Control");
    adaptive_quality_control_example()?;

    // Example 6: Hardware decoder selection
    println!("\nExample 6: Hardware Decoder Selection");
    hardware_decoder_selection_example()?;

    // Example 7: Complete optimization workflow
    println!("\nExample 7: Complete Optimization Workflow");
    complete_optimization_workflow()?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Basic decoding optimization
fn basic_decoding_optimization() -> anyhow::Result<()> {
    let config = DecodingOptimizationConfig::default();
    let optimizer = VideoDecodingOptimizer::new(config);

    println!("  Supported codecs:");
    for codec in optimizer.supported_codecs() {
        println!("    - {}", codec.codec);
        println!("      Hardware decoders: {:?}", codec.hardware_decoders);
        println!("      Software decoder: {}", codec.software_decoder);
        println!("      Hardware recommended: {}", codec.hardware_recommended);
    }

    // Select decoder for H.264
    let decoder = optimizer.select_decoder("H.264");
    println!("\n  Selected decoder for H.264: {:?}", decoder);

    // Set decoder
    if let Some(decoder) = decoder {
        optimizer.set_decoder(decoder);
    }

    // Process some frames
    println!("\n  Processing frames...");
    for i in 0..10 {
        let processed = optimizer.process_frame(i, 1024 * 1024)?;
        if processed {
            println!("    Frame {} processed", i);
        } else {
            println!("    Frame {} skipped", i);
        }
    }

    // Get statistics
    let stats = optimizer.statistics();
    println!("\n  Statistics:");
    println!("    Frame buffer size: {}", stats.frame_buffer_size);
    println!("    Frame buffer usage: {:.1}%", stats.frame_buffer_usage);
    println!("    Current decoder: {:?}", stats.current_decoder);

    Ok(())
}

/// Example 2: Frame buffer management
fn frame_buffer_management_example() -> anyhow::Result<()> {
    let mut manager = FrameBufferManager::new(5);

    println!("  Adding frames to buffer...");
    for i in 0..7 {
        let entry = FrameBufferEntry {
            index: i,
            timestamp: Duration::from_secs(i as u64),
            size: 1024 * 1024,
            decode_time: Duration::from_millis(10),
            quality: 100,
        };
        manager.add_frame(entry)?;
        println!("    Frame {} added (buffer size: {})", i, manager.buffer_size());
    }

    println!("\n  Buffer statistics:");
    println!("    Buffer size: {}", manager.buffer_size());
    println!("    Total size: {} bytes", manager.total_size());
    println!("    Usage: {:.1}%", manager.usage_percentage());

    println!("\n  Retrieving frame 3...");
    if let Some(frame) = manager.get_frame(3) {
        println!("    Found frame: index={}, size={} bytes", frame.index, frame.size);
    }

    println!("\n  Removing frame 3...");
    manager.remove_frame(3);
    println!("    Buffer size after removal: {}", manager.buffer_size());

    println!("\n  Clearing buffer...");
    manager.clear();
    println!("    Buffer size after clear: {}", manager.buffer_size());

    Ok(())
}

/// Example 3: Frame skipping strategies
fn frame_skipping_strategies_example() -> anyhow::Result<()> {
    println!("  Strategy 1: Skip every 2nd frame");
    let controller1 = FrameSkippingController::new(FrameSkippingStrategy::SkipEveryNth(2), 60.0);
    for i in 0..10 {
        let skipped = controller1.should_skip_frame();
        println!("    Frame {}: {}", i, if skipped { "SKIPPED" } else "KEPT" });
    }

    println!("\n  Strategy 2: Adaptive skipping");
    let controller2 = FrameSkippingController::new(FrameSkippingStrategy::Adaptive, 60.0);
    for i in 0..10 {
        let skipped = controller2.should_skip_frame();
        println!("    Frame {}: {}", i, if skipped { "SKIPPED" } else "KEPT" });
    }

    println!("\n  Strategy 3: Target FPS (30 FPS)");
    let controller3 = FrameSkippingController::new(FrameSkippingStrategy::TargetFPS(30.0), 60.0);
    for i in 0..10 {
        let skipped = controller3.should_skip_frame();
        println!("    Frame {}: {}", i, if skipped { "SKIPPED" } else "KEPT" });
        std::thread::sleep(Duration::from_millis(10));
    }

    println!("\n  Disabling frame skipping");
    controller1.set_enabled(false);
    for i in 0..5 {
        let skipped = controller1.should_skip_frame();
        println!("    Frame {}: {}", i, if skipped { "SKIPPED" } else "KEPT" });
    }

    Ok(())
}

/// Example 4: GPU-CPU synchronization
fn gpu_cpu_sync_example() -> anyhow::Result<()> {
    println!("  Without zero-copy:");
    let manager1 = GpuCpuSyncManager::new(Duration::from_millis(16), false);
    println!("    Zero-copy enabled: {}", manager1.is_zero_copy_enabled());

    println!("\n  Submitting frames to GPU...");
    for i in 0..5 {
        manager1.submit_to_gpu()?;
        println!("    Frame {} submitted (pending: {})", i, manager1.pending_frames());
    }

    println!("\n  Waiting for GPU...");
    for i in 0..5 {
        manager1.wait_for_gpu()?;
        println!("    Frame {} completed (pending: {})", i, manager1.pending_frames());
    }

    println!("\n  With zero-copy:");
    let manager2 = GpuCpuSyncManager::new(Duration::from_millis(16), true);
    println!("    Zero-copy enabled: {}", manager2.is_zero_copy_enabled());

    println!("\n  Submitting frames to GPU...");
    for i in 0..5 {
        manager2.submit_to_gpu()?;
        println!("    Frame {} submitted (pending: {})", i, manager2.pending_frames());
    }

    Ok(())
}

/// Example 5: Adaptive quality control
fn adaptive_quality_control_example() -> anyhow::Result<()> {
    let controller = AdaptiveQualityController::new(50, 100, 60.0);

    println!("  Initial quality: {}", controller.current_quality());

    println!("\n  Simulating poor performance (30 FPS)...");
    controller.adjust_quality(30.0);
    println!("    Quality adjusted to: {}", controller.current_quality());

    println!("\n  Simulating good performance (80 FPS)...");
    controller.adjust_quality(80.0);
    println!("    Quality adjusted to: {}", controller.current_quality());

    println!("\n  Manual quality setting:");
    controller.set_quality(75);
    println!("    Quality set to: {}", controller.current_quality());

    println!("\n  Testing quality clamping:");
    controller.set_quality(150);
    println!("    Set to 150, clamped to: {}", controller.current_quality());

    controller.set_quality(25);
    println!("    Set to 25, clamped to: {}", controller.current_quality());

    Ok(())
}

/// Example 6: Hardware decoder selection
fn hardware_decoder_selection_example() -> anyhow::Result<()> {
    let config = DecodingOptimizationConfig::default();
    let optimizer = VideoDecodingOptimizer::new(config);

    println!("  Hardware decoders:");
    for decoder in [
        HardwareDecoder::NVDEC,
        HardwareDecoder::QuickSync,
        HardwareDecoder::VCE,
        HardwareDecoder::VideoToolbox,
        HardwareDecoder::VAAPI,
        HardwareDecoder::VDPAU,
        HardwareDecoder::DXVA2,
        HardwareDecoder::D3D11VA,
        HardwareDecoder::Software,
    ] {
        println!("    - {}: {}", decoder.as_str(), if decoder.is_hardware() { "Hardware" } else { "Software" });
    }

    println!("\n  Decoder selection for different codecs:");
    for codec in ["H.264", "H.265/HEVC", "VP9", "AV1"] {
        let decoder = optimizer.select_decoder(codec);
        println!("    {}: {:?}", codec, decoder);
    }

    Ok(())
}

/// Example 7: Complete optimization workflow
fn complete_optimization_workflow() -> anyhow::Result<()> {
    let config = DecodingOptimizationConfig {
        hardware_acceleration: true,
        enable_frame_skipping: true,
        max_frame_buffer_size: 20,
        frame_skip_threshold: 0.8,
        enable_adaptive_quality: true,
        min_quality: 50,
        max_quality: 100,
        gpu_cpu_sync_timeout: 16,
        enable_zero_copy: true,
        prefetch_frame_count: 5,
    };

    let optimizer = VideoDecodingOptimizer::new(config);

    println!("  Step 1: Select hardware decoder");
    let decoder = optimizer.select_decoder("H.264");
    if let Some(decoder) = decoder {
        optimizer.set_decoder(decoder);
        println!("    Decoder set to: {}", decoder.as_str());
    }

    println!("\n  Step 2: Process frames with optimization");
    for i in 0..30 {
        let processed = optimizer.process_frame(i, 1024 * 1024)?;
        if processed {
            // Simulate CPU usage
            let cpu_usage = 0.3 + (i as f32 % 10) / 20.0;
            optimizer.update_cpu_usage(cpu_usage);
        }
    }

    println!("\n  Step 3: Get final statistics");
    let stats = optimizer.statistics();
    println!("    Frame buffer size: {}", stats.frame_buffer_size);
    println!("    Frame buffer usage: {:.1}%", stats.frame_buffer_usage);
    println!("    Frame buffer total size: {} bytes", stats.frame_buffer_total_size);
    println!("    Average CPU usage: {:.1}%", stats.average_cpu_usage * 100.0);
    println!("    Current quality: {}", stats.current_quality);
    println!("    Current decoder: {:?}", stats.current_decoder);
    println!("    Pending frames: {}", stats.pending_frames);
    println!("    Sync errors: {}", stats.sync_errors);

    println!("\n  Step 4: Access individual components");
    let frame_buffer = optimizer.frame_buffer();
    let buffer = frame_buffer.lock().unwrap();
    println!("    Frame buffer manager: {} frames", buffer.buffer_size());

    let frame_skipping = optimizer.frame_skipping();
    println!("    Frame skipping controller: enabled");

    let gpu_cpu_sync = optimizer.gpu_cpu_sync();
    println!("    GPU-CPU sync manager: zero-copy={}", gpu_cpu_sync.is_zero_copy_enabled());

    let adaptive_quality = optimizer.adaptive_quality();
    println!("    Adaptive quality controller: quality={}", adaptive_quality.current_quality());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_decoding_optimization() {
        let result = basic_decoding_optimization();
        assert!(result.is_ok());
    }

    #[test]
    fn test_frame_buffer_management() {
        let result = frame_buffer_management_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_frame_skipping_strategies() {
        let result = frame_skipping_strategies_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_gpu_cpu_sync() {
        let result = gpu_cpu_sync_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_adaptive_quality_control() {
        let result = adaptive_quality_control_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_hardware_decoder_selection() {
        let result = hardware_decoder_selection_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_complete_optimization_workflow() {
        let result = complete_optimization_workflow();
        assert!(result.is_ok());
    }
}