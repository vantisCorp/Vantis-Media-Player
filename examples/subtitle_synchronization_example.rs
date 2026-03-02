//! Subtitle Synchronization Example
//!
//! This example demonstrates the subtitle synchronization features of Vantis Media Player:
//! - Advanced sync algorithms
//! - Manual sync adjustment
//! - Subtitle delay support
//! - Sync presets
//! - Sync preview functionality

use std::time::Duration;
use vantissubs::synchronization::{
    SyncAlgorithm, SyncConfig, SyncPreset, SyncQuality, SubtitleSynchronizer,
};

fn main() -> anyhow::Result<()> {
    println!("=== Vantis Media Player - Subtitle Synchronization Example ===\n");

    // Example 1: Basic synchronizer setup
    println!("Example 1: Basic Synchronizer Setup");
    basic_synchronizer_setup()?;

    // Example 2: Manual sync adjustment
    println!("\nExample 2: Manual Sync Adjustment");
    manual_sync_adjustment()?;

    // Example 3: Subtitle delay
    println!("\nExample 3: Subtitle Delay");
    subtitle_delay()?;

    // Example 4: Sync presets
    println!("\nExample 4: Sync Presets");
    sync_presets()?;

    // Example 5: Advanced sync algorithms
    println!("\nExample 5: Advanced Sync Algorithms");
    advanced_sync_algorithms()?;

    // Example 6: Sync preview
    println!("\nExample 6: Sync Preview");
    sync_preview()?;

    // Example 7: Complete workflow
    println!("\nExample 7: Complete Workflow");
    complete_workflow()?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Basic synchronizer setup
fn basic_synchronizer_setup() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let synchronizer = SubtitleSynchronizer::new(config);

    println!("  Synchronizer created");
    println!("    Current delay: {}ms", synchronizer.current_delay());
    println!("    Auto-sync enabled: {}", synchronizer.config().auto_sync);
    println!("    Sync algorithm: {:?}", synchronizer.config().sync_algorithm);
    println!("    Presets: {}", synchronizer.presets().len());

    Ok(())
}

/// Example 2: Manual sync adjustment
fn manual_sync_adjustment() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let mut synchronizer = SubtitleSynchronizer::new(config);

    println!("  Initial delay: {}ms", synchronizer.current_delay());

    // Adjust delay
    synchronizer.adjust_delay(100)?;
    println!("  After +100ms: {}ms", synchronizer.current_delay());

    synchronizer.adjust_delay(-50)?;
    println!("  After -50ms: {}ms", synchronizer.current_delay());

    // Reset delay
    synchronizer.reset_delay();
    println!("  After reset: {}ms", synchronizer.current_delay());

    Ok(())
}

/// Example 3: Subtitle delay
fn subtitle_delay() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let mut synchronizer = SubtitleSynchronizer::new(config);

    println!("  Setting various delays:");

    synchronizer.set_delay(-500)?;
    println!("    Early 500ms: {}ms", synchronizer.current_delay());

    synchronizer.set_delay(-100)?;
    println!("    Early 100ms: {}ms", synchronizer.current_delay());

    synchronizer.set_delay(0)?;
    println!("    No delay: {}ms", synchronizer.current_delay());

    synchronizer.set_delay(100)?;
    println!("    Late 100ms: {}ms", synchronizer.current_delay());

    synchronizer.set_delay(500)?;
    println!("    Late 500ms: {}ms", synchronizer.current_delay());

    // Test out of range
    let result = synchronizer.set_delay(20000);
    println!("    Out of range (20000ms): {}", result.is_err());

    Ok(())
}

/// Example 4: Sync presets
fn sync_presets() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let mut synchronizer = SubtitleSynchronizer::new(config);

    println!("  Available sync presets:");
    for preset in synchronizer.presets() {
        println!("    - {}", preset.name);
        println!("      Delay: {}ms", preset.delay);
        println!("      Algorithm: {:?}", preset.algorithm);
        println!("      Description: {}", preset.description);
    }

    println!("\n  Applying presets:");
    synchronizer.apply_preset("Late 100ms")?;
    println!("    Applied 'Late 100ms': {}ms", synchronizer.current_delay());

    synchronizer.apply_preset("Early 250ms")?;
    println!("    Applied 'Early 250ms': {}ms", synchronizer.current_delay());

    Ok(())
}

/// Example 5: Advanced sync algorithms
fn advanced_sync_algorithms() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let mut synchronizer = SubtitleSynchronizer::new(config);

    println!("  Testing sync algorithms:");

    // Add sync points
    synchronizer.add_sync_point(Duration::from_secs(10), Duration::from_secs(10), 1.0);
    synchronizer.add_sync_point(Duration::from_secs(20), Duration::from_secs(20), 0.9);
    synchronizer.add_sync_point(Duration::from_secs(30), Duration::from_secs(30), 0.8);

    println!("    Added 3 sync points");

    // Test Linear algorithm
    let mut config = SyncConfig::default();
    config.sync_algorithm = SyncAlgorithm::Linear;
    synchronizer.set_config(config);
    let result = synchronizer.auto_sync()?;
    println!("    Linear sync: delay={}ms, confidence={:.2}, quality={}",
             result.applied_delay, result.confidence, result.quality.as_str());

    // Test Adaptive algorithm
    let mut config = SyncConfig::default();
    config.sync_algorithm = SyncAlgorithm::Adaptive;
    synchronizer.set_config(config);
    let result = synchronizer.auto_sync()?;
    println!("    Adaptive sync: delay={}ms, confidence={:.2}, quality={}",
             result.applied_delay, result.confidence, result.quality.as_str());

    // Test Waveform algorithm
    let mut config = SyncConfig::default();
    config.sync_algorithm = SyncAlgorithm::Waveform;
    synchronizer.set_config(config);
    let result = synchronizer.auto_sync()?;
    println!("    Waveform sync: delay={}ms, confidence={:.2}, quality={}",
             result.applied_delay, result.confidence, result.quality.as_str());

    // Test Speech Recognition algorithm
    let mut config = SyncConfig::default();
    config.sync_algorithm = SyncAlgorithm::SpeechRecognition;
    synchronizer.set_config(config);
    let result = synchronizer.auto_sync()?;
    println!("    Speech Recognition sync: delay={}ms, confidence={:.2}, quality={}",
             result.applied_delay, result.confidence, result.quality.as_str());

    Ok(())
}

/// Example 6: Sync preview
fn sync_preview() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let synchronizer = SubtitleSynchronizer::new(config);

    println!("  Previewing sync with different delays:");

    let delays = vec![-500, -250, -100, 0, 100, 250, 500];
    for delay in delays {
        let result = synchronizer.preview_sync(delay);
        println!("    Delay {}ms: quality={}", delay, result.quality.as_str());
    }

    Ok(())
}

/// Example 7: Complete workflow
fn complete_workflow() -> anyhow::Result<()> {
    let config = SyncConfig::default();
    let mut synchronizer = SubtitleSynchronizer::new(config);

    println!("  Step 1: Add sync points");
    synchronizer.add_sync_point(Duration::from_secs(5), Duration::from_secs(5), 1.0);
    synchronizer.add_sync_point(Duration::from_secs(15), Duration::from_secs(15), 0.95);
    synchronizer.add_sync_point(Duration::from_secs(25), Duration::from_secs(25), 0.9);
    synchronizer.add_sync_point(Duration::from_secs(35), Duration::from_secs(35), 0.85);
    synchronizer.add_sync_point(Duration::from_secs(45), Duration::from_secs(45), 0.8);
    println!("    Added 5 sync points");

    println!("  Step 2: Perform auto-sync");
    let result = synchronizer.auto_sync()?;
    println!("    Auto-sync completed");
    println!("    Applied delay: {}ms", result.applied_delay);
    println!("    Algorithm: {:?}", result.algorithm);
    println!("    Confidence: {:.2}", result.confidence);
    println!("    Quality: {}", result.quality.as_str());

    println!("  Step 3: Fine-tune manually");
    synchronizer.adjust_delay(50)?;
    println!("    Adjusted by +50ms");
    println!("    New delay: {}ms", synchronizer.current_delay());

    println!("  Step 4: Preview sync");
    let preview_result = synchronizer.preview_sync(synchronizer.current_delay());
    println!("    Preview quality: {}", preview_result.quality.as_str());

    println!("  Step 5: Save as preset");
    println!("    (In a real implementation, this would save the current settings as a custom preset)");

    println!("  Step 6: Clear sync points");
    synchronizer.clear_sync_points();
    println!("    Sync points cleared");

    println!("  Complete workflow finished successfully!");

    Ok(())
}