//! Memory Optimization Example
//!
//! Demonstrates comprehensive memory optimization features including
//! video frame pooling, buffer optimization, and lazy loading.

use anyhow::Result;
use vanis_core::{MemoryOptimizer, MemoryOptimizationConfig};

fn main() -> Result<()> {
    println!("🔧 Memory Optimization Example\n");
    
    // Create memory optimizer with configuration
    let config = MemoryOptimizationConfig {
        enable_pooling: true,
        enable_lazy_loading: true,
        enable_compression: false,
        target_reduction: 20,
        max_pool_size_mb: 512,
        enable_monitoring: true,
        warning_threshold: 80,
        critical_threshold: 90,
    };
    
    let mut optimizer = MemoryOptimizer::new(config)?;
    
    println!("✅ Memory optimizer initialized");
    println!("   Target reduction: {}%", optimizer.config.target_reduction);
    println!("   Pooling enabled: {}", optimizer.config.enable_pooling);
    println!("   Lazy loading enabled: {}", optimizer.config.enable_lazy_loading);
    
    // Example 1: Initialize video frame pool
    println!("\n🎬 Example 1: Video Frame Pool");
    println!("   Initializing frame pool for 1080p video...");
    optimizer.init_frame_pool(1920, 1080, 10)?;
    
    // Example 2: Acquire and release frames
    println!("\n📦 Example 2: Acquiring Frames");
    println!("   Acquiring frame from pool...");
    let frame = optimizer.acquire_frame()?;
    println!("   Frame acquired successfully");
    println!("   Frame size: {} bytes", frame.len());
    
    // Frame is automatically released when dropped
    drop(frame);
    println!("   Frame released back to pool");
    
    // Example 3: Allocate buffers
    println!("\n💾 Example 3: Buffer Allocation");
    println!("   Allocating small buffer (1KB)...");
    let buffer1 = optimizer.allocate_buffer(1024)?;
    println!("   Buffer allocated: {} bytes", buffer1.len());
    
    println!("   Allocating medium buffer (32KB)...");
    let buffer2 = optimizer.allocate_buffer(32 * 1024)?;
    println!("   Buffer allocated: {} bytes", buffer2.len());
    
    println!("   Allocating large buffer (256KB)...");
    let buffer3 = optimizer.allocate_buffer(256 * 1024)?;
    println!("   Buffer allocated: {} bytes", buffer3.len());
    
    // Example 4: Get memory statistics
    println!("\n📊 Example 4: Memory Statistics");
    let stats = optimizer.get_stats();
    println!("   Current usage: {} MB", stats.current_usage / (1024 * 1024));
    println!("   Peak usage: {} MB", stats.peak_usage / (1024 * 1024));
    println!("   Usage percentage: {:.1}%", stats.usage_percentage);
    
    if let Some(frame_stats) = stats.frame_pool {
        println!("   Frame pool statistics:");
        println!("     Total allocations: {}", frame_stats.total_allocations);
        println!("     Cache hits: {}", frame_stats.cache_hits);
        println!("     Cache misses: {}", frame_stats.cache_misses);
        println!("     Memory saved: {} MB", frame_stats.memory_saved_bytes / (1024 * 1024));
    }
    
    // Example 5: Run optimization
    println!("\n⚡ Example 5: Running Optimization");
    let result = optimizer.optimize()?;
    println!("   Frames removed: {}", result.frames_removed);
    println!("   Buffers freed: {}", result.buffers_freed);
    println!("   Memory freed: {} MB", result.memory_freed / (1024 * 1024));
    
    // Example 6: Lazy loading demonstration
    println!("\n🐌 Example 6: Lazy Loading");
    println!("   Creating lazy loader for heavy resource...");
    use vanis_core::LazyLoader;
    
    let lazy_loader = LazyLoader::new(|| {
        println!("   Loading resource...");
        Ok(vec![1, 2, 3, 4, 5])
    });
    
    println!("   Lazy loader created (not loaded yet)");
    println!("   Is loaded: {}", lazy_loader.is_loaded());
    
    let value = lazy_loader.get()?;
    println!("   Value retrieved: {:?}", value);
    println!("   Is loaded: {}", lazy_loader.is_loaded());
    
    // Unload
    lazy_loader.unload();
    println!("   Resource unloaded");
    println!("   Is loaded: {}", lazy_loader.is_loaded());
    
    println!("\n✅ All memory optimization features demonstrated!");
    
    Ok(())
}
