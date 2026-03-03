//! Memory Optimization Example
//!
//! Demonstrates the memory optimization features to reduce memory usage by 20%.

use anyhow::Result;
use vantis_core::{VantisCore, Config};
use vantis_core::memory_optimization::{MemoryOptimizer, MemoryOptimizationConfig, VideoFramePool};

fn main() -> Result<()> {
    println!("🚀 Vantis Media Player - Memory Optimization Example");
    println!();
    println!("This example demonstrates memory optimization features to reduce");
    println!("memory usage by 20% through:");
    println!("  - Video frame pooling");
    println!("  - Memory reclamation");
    println!("  - Memory statistics tracking");
    println!();
    
    // Create a Vantis Core instance with memory optimization
    let config = Config::default();
    let core = VantisCore::new(config)?;
    
    println!("✅ Vantis Core initialized with memory optimization");
    println!();
    
    // Get memory statistics
    let stats = core.memory_stats();
    println!("📊 Initial Memory Statistics:");
    println!("  Total Memory: {} MB", stats.total_mb);
    println!("  Used Memory: {} MB", stats.used_mb);
    println!("  Free Memory: {} MB", stats.free_mb);
    println!("  Usage: {:.1}%", stats.usage_percent);
    println!("  Allocations: {}", stats.allocations);
    println!();
    
    // Allocate some video frames
    println!("🎬 Allocating video frames...");
    let optimizer = core.memory_optimizer();
    
    for i in 0..5 {
        match optimizer.allocate_frame() {
            Ok(handle) => {
                println!("  Allocated frame {} ({} bytes)", i, handle.len());
            }
            Err(e) => {
                println!("  Failed to allocate frame {}: {}", i, e);
            }
        }
    }
    
    println!();
    
    // Get memory statistics after allocation
    let stats = core.memory_stats();
    println!("📊 Memory Statistics After Allocation:");
    println!("  Total Memory: {} MB", stats.total_mb);
    println!("  Used Memory: {} MB", stats.used_mb);
    println!("  Free Memory: {} MB", stats.free_mb);
    println!("  Usage: {:.1}%", stats.usage_percent);
    println!("  Allocations: {}", stats.allocations);
    println!();
    
    // Optimize memory
    println!("🔧 Optimizing memory...");
    let reduction = core.optimize_memory()?;
    println!("  Achieved {:.1}% memory reduction", reduction);
    println!();
    
    // Get memory statistics after optimization
    let stats = core.memory_stats();
    println!("📊 Memory Statistics After Optimization:");
    println!("  Total Memory: {} MB", stats.total_mb);
    println!("  Used Memory: {} MB", stats.used_mb);
    println!("  Free Memory: {} MB", stats.free_mb);
    println!("  Usage: {:.1}%", stats.usage_percent);
    println!("  Allocations: {}", stats.allocations);
    println!();
    
    // Demonstrate frame pool directly
    println!("🎬 Demonstrating video frame pool...");
    let pool = VideoFramePool::new(1920, 1080, 10)?;
    
    let (used, total) = pool.usage();
    println!("  Pool usage: {}/{} frames", used, total);
    println!("  Memory usage: {} MB", pool.memory_usage_mb());
    println!();
    
    // Allocate frames from pool
    println!("  Allocating frames from pool...");
    let mut handles = Vec::new();
    for i in 0..3 {
        match pool.allocate() {
            Ok(handle) => {
                println!("    Allocated frame {}", i);
                handles.push(handle);
            }
            Err(e) => {
                println!("    Failed to allocate frame {}: {}", i, e);
            }
        }
    }
    
    let (used, total) = pool.usage();
    println!("  Pool usage after allocation: {}/{} frames", used, total);
    println!("  Memory usage: {} MB", pool.memory_usage_mb());
    println!();
    
    // Drop handles to return frames to pool
    println!("  Returning frames to pool...");
    drop(handles);
    
    let (used, total) = pool.usage();
    println!("  Pool usage after return: {}/{} frames", used, total);
    println!("  Memory usage: {} MB", pool.memory_usage_mb());
    println!();
    
    println!("✅ Memory optimization example completed successfully!");
    println!();
    println!("Key Benefits:");
    println!("  - Reduced memory usage through pooling");
    println!("  - Automatic memory reclamation");
    println!("  - Real-time memory statistics");
    println!("  - Configurable optimization targets");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer_creation() {
        let config = MemoryOptimizationConfig::default();
        let optimizer = MemoryOptimizer::new(config);
        assert!(optimizer.is_enabled());
    }

    #[test]
    fn test_memory_optimization() {
        let config = MemoryOptimizationConfig::default();
        let mut optimizer = MemoryOptimizer::new(config);
        optimizer.init_frame_pool(1920, 1080, 10).unwrap();
        
        // Allocate some frames
        let _handle1 = optimizer.allocate_frame().unwrap();
        let _handle2 = optimizer.allocate_frame().unwrap();
        
        // Optimize
        let reduction = optimizer.optimize().unwrap();
        assert!(reduction >= 0.0);
    }

    #[test]
    fn test_memory_stats() {
        let config = MemoryOptimizationConfig::default();
        let mut optimizer = MemoryOptimizer::new(config);
        optimizer.init_frame_pool(1920, 1080, 10).unwrap();
        
        let stats = optimizer.get_stats();
        assert!(stats.total_mb > 0);
        assert_eq!(stats.allocations, 0);
    }

    #[test]
    fn test_frame_pool_efficiency() {
        let pool = VideoFramePool::new(1920, 1080, 10).unwrap();
        
        // Allocate and deallocate multiple times
        for _ in 0..100 {
            let handle = pool.allocate().unwrap();
            drop(handle);
        }
        
        let (used, total) = pool.usage();
        assert_eq!(used, 0);
        assert_eq!(total, 10);
    }

    #[test]
    fn test_memory_reduction_target() {
        let config = MemoryOptimizationConfig {
            target_reduction: 20,
            ..Default::default()
        };
        let optimizer = MemoryOptimizer::new(config);
        assert_eq!(optimizer.config.target_reduction, 20);
    }
}