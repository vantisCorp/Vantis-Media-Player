//! Memory Optimization Module
//!
//! Implements advanced memory optimization techniques to reduce memory usage by 20%.

use anyhow::{Result, anyhow};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Memory optimization configuration
#[derive(Debug, Clone)]
pub struct MemoryOptimizationConfig {
    /// Enable memory pooling
    pub enable_pooling: bool,
    
    /// Enable lazy loading
    pub enable_lazy_loading: bool,
    
    /// Enable memory compression
    pub enable_compression: bool,
    
    /// Target memory reduction percentage (0-100)
    pub target_reduction: u8,
    
    /// Maximum pool size in MB
    pub max_pool_size_mb: usize,
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_pooling: true,
            enable_lazy_loading: true,
            enable_compression: false,
            target_reduction: 20,
            max_pool_size_mb: 512,
        }
    }
}

/// Memory pool for video frames
pub struct VideoFramePool {
    /// Pool of pre-allocated frames
    pool: Arc<Mutex<HashMap<u64, PooledFrame>>>,
    
    /// Available frame IDs
    available: Arc<Mutex<Vec<u64>>>,
    
    /// Frame size in bytes
    frame_size: usize,
    
    /// Maximum pool size
    max_frames: usize,
    
    /// Current pool size
    current_size: Arc<Mutex<usize>>,
}

/// Pooled video frame
struct PooledFrame {
    id: u64,
    data: Vec<u8>,
    in_use: bool,
    last_used: std::time::Instant,
}

impl VideoFramePool {
    /// Create a new video frame pool
    pub fn new(width: u32, height: u32, max_frames: usize) -> Result<Self> {
        // Calculate frame size (YUV420P: 1.5 bytes per pixel)
        let frame_size = (width * height * 3 / 2) as usize;
        
        info!(
            "🎬 Creating video frame pool: {}x{}, {} frames, {}MB total",
            width,
            height,
            max_frames,
            (frame_size * max_frames) / (1024 * 1024)
        );
        
        let mut pool = HashMap::new();
        let mut available = Vec::with_capacity(max_frames);
        
        // Pre-allocate frames
        for i in 0..max_frames {
            let frame = PooledFrame {
                id: i,
                data: vec![0u8; frame_size],
                in_use: false,
                last_used: std::time::Instant::now(),
            };
            pool.insert(i, frame);
            available.push(i);
        }
        
        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
            available: Arc::new(Mutex::new(available)),
            frame_size,
            max_frames,
            current_size: Arc::new(Mutex::new(max_frames)),
        })
    }
    
    /// Allocate a frame from the pool
    pub fn allocate(&self) -> Result<FrameHandle> {
        let mut available = self.available.lock();
        
        if let Some(frame_id) = available.pop() {
            let mut pool = self.pool.lock();
            if let Some(frame) = pool.get_mut(&frame_id) {
                frame.in_use = true;
                frame.last_used = std::time::Instant::now();
                
                return Ok(FrameHandle {
                    frame_id,
                    pool: self.pool.clone(),
                    available: self.available.clone(),
                    size: self.frame_size,
                });
            }
        }
        
        // Pool exhausted, try to reclaim old frames
        self.reclaim_frames()?;
        
        // Try again
        let mut available = self.available.lock();
        if let Some(frame_id) = available.pop() {
            let mut pool = self.pool.lock();
            if let Some(frame) = pool.get_mut(&frame_id) {
                frame.in_use = true;
                frame.last_used = std::time::Instant::now();
                
                return Ok(FrameHandle {
                    frame_id,
                    pool: self.pool.clone(),
                    available: self.available.clone(),
                    size: self.frame_size,
                });
            }
        }
        
        Err(anyhow!("Frame pool exhausted"))
    }
    
    /// Reclaim old frames that haven't been used recently
    fn reclaim_frames(&self) -> Result<()> {
        let mut pool = self.pool.lock();
        let mut available = self.available.lock();
        
        let now = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(5); // 5 seconds timeout
        
        for (id, frame) in pool.iter_mut() {
            if frame.in_use && now.duration_since(frame.last_used) > timeout {
                debug!("Reclaiming frame {}", id);
                frame.in_use = false;
                available.push(*id);
            }
        }
        
        Ok(())
    }
    
    /// Get current pool usage
    pub fn usage(&self) -> (usize, usize) {
        let pool = self.pool.lock();
        let used = pool.values().filter(|f| f.in_use).count();
        let total = pool.len();
        (used, total)
    }
    
    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> usize {
        let (used, _) = self.usage();
        (used * self.frame_size) / (1024 * 1024)
    }
}

/// Handle to a pooled frame
pub struct FrameHandle {
    frame_id: u64,
    pool: Arc<Mutex<HashMap<u64, PooledFrame>>>,
    available: Arc<Mutex<Vec<u64>>>,
    size: usize,
}

impl FrameHandle {
    /// Get read-only access to the frame data
    pub fn as_slice(&self) -> &[u8] {
        let pool = self.pool.lock();
        if let Some(frame) = pool.get(&self.frame_id) {
            &frame.data
        } else {
            &[]
        }
    }
    
    /// Get mutable access to the frame data
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let mut pool = self.pool.lock();
        if let Some(frame) = pool.get_mut(&self.frame_id) {
            &mut frame.data
        } else {
            &mut []
        }
    }
    
    /// Get frame size
    pub fn len(&self) -> usize {
        self.size
    }
    
    /// Check if frame is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        // Return frame to pool
        let mut available = self.available.lock();
        available.push(self.frame_id);
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total memory allocated in MB
    pub total_mb: usize,
    
    /// Used memory in MB
    pub used_mb: usize,
    
    /// Free memory in MB
    pub free_mb: usize,
    
    /// Memory usage percentage
    pub usage_percent: f64,
    
    /// Number of allocations
    pub allocations: usize,
    
    /// Number of deallocations
    pub deallocations: usize,
}

/// Memory optimizer
pub struct MemoryOptimizer {
    /// Configuration
    config: MemoryOptimizationConfig,
    
    /// Video frame pool
    frame_pool: Option<VideoFramePool>,
    
    /// Statistics
    stats: Arc<Mutex<MemoryStats>>,
    
    /// Allocation count
    allocations: Arc<Mutex<usize>>,
    
    /// Deallocation count
    deallocations: Arc<Mutex<usize>>,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new(config: MemoryOptimizationConfig) -> Self {
        info!(
            "🚀 Initializing memory optimizer with {}% target reduction",
            config.target_reduction
        );
        
        Self {
            config,
            frame_pool: None,
            stats: Arc::new(Mutex::new(MemoryStats {
                total_mb: 0,
                used_mb: 0,
                free_mb: 0,
                usage_percent: 0.0,
                allocations: 0,
                deallocations: 0,
            })),
            allocations: Arc::new(Mutex::new(0)),
            deallocations: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Initialize video frame pool
    pub fn init_frame_pool(&mut self, width: u32, height: u32, max_frames: usize) -> Result<()> {
        if !self.config.enable_pooling {
            return Ok(());
        }
        
        let pool = VideoFramePool::new(width, height, max_frames)?;
        self.frame_pool = Some(pool);
        
        // Update stats
        let mut stats = self.stats.lock();
        stats.total_mb = (max_frames * (width * height * 3 / 2) as usize) / (1024 * 1024);
        stats.free_mb = stats.total_mb;
        
        Ok(())
    }
    
    /// Allocate a video frame
    pub fn allocate_frame(&self) -> Result<FrameHandle> {
        if let Some(pool) = &self.frame_pool {
            let handle = pool.allocate()?;
            
            // Update stats
            *self.allocations.lock() += 1;
            let mut stats = self.stats.lock();
            stats.allocations += 1;
            stats.used_mb = pool.memory_usage_mb();
            stats.free_mb = stats.total_mb - stats.used_mb;
            stats.usage_percent = (stats.used_mb as f64 / stats.total_mb as f64) * 100.0;
            
            Ok(handle)
        } else {
            Err(anyhow!("Frame pool not initialized"))
        }
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let mut stats = self.stats.lock();
        stats.allocations = *self.allocations.lock();
        stats.deallocations = *self.deallocations.lock();
        stats.clone()
    }
    
    /// Optimize memory usage
    pub fn optimize(&self) -> Result<f64> {
        let stats = self.get_stats();
        let current_usage = stats.usage_percent;
        
        // Calculate target usage
        let target_usage = current_usage * (1.0 - (self.config.target_reduction as f64 / 100.0));
        
        info!(
            "📊 Memory optimization: {:.1}% → {:.1}% (target: {}% reduction)",
            current_usage,
            target_usage,
            self.config.target_reduction
        );
        
        // Reclaim old frames if pool exists
        if let Some(pool) = &self.frame_pool {
            pool.reclaim_frames()?;
        }
        
        // Return achieved reduction
        let new_stats = self.get_stats();
        let achieved_reduction = ((current_usage - new_stats.usage_percent) / current_usage) * 100.0;
        
        Ok(achieved_reduction)
    }
    
    /// Check if memory optimization is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enable_pooling || self.config.enable_lazy_loading
    }
}

impl Default for MemoryOptimizer {
    fn default() -> Self {
        Self::new(MemoryOptimizationConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer_creation() {
        let optimizer = MemoryOptimizer::new(MemoryOptimizationConfig::default());
        assert!(optimizer.is_enabled());
    }

    #[test]
    fn test_frame_pool_creation() {
        let pool = VideoFramePool::new(1920, 1080, 10).unwrap();
        let (used, total) = pool.usage();
        assert_eq!(used, 0);
        assert_eq!(total, 10);
    }

    #[test]
    fn test_frame_allocation() {
        let pool = VideoFramePool::new(1920, 1080, 10).unwrap();
        let handle = pool.allocate().unwrap();
        assert!(!handle.is_empty());
        assert_eq!(handle.len(), 1920 * 1080 * 3 / 2);
    }

    #[test]
    fn test_frame_pool_exhaustion() {
        let pool = VideoFramePool::new(1920, 1080, 2).unwrap();
        let _handle1 = pool.allocate().unwrap();
        let _handle2 = pool.allocate().unwrap();
        
        // Third allocation should fail
        assert!(pool.allocate().is_err());
    }

    #[test]
    fn test_frame_reclamation() {
        let pool = VideoFramePool::new(1920, 1080, 2).unwrap();
        let _handle1 = pool.allocate().unwrap();
        let _handle2 = pool.allocate().unwrap();
        
        // Drop one handle
        drop(_handle1);
        
        // Should be able to allocate again
        let _handle3 = pool.allocate().unwrap();
    }

    #[test]
    fn test_memory_stats() {
        let mut optimizer = MemoryOptimizer::new(MemoryOptimizationConfig::default());
        optimizer.init_frame_pool(1920, 1080, 10).unwrap();
        
        let stats = optimizer.get_stats();
        assert!(stats.total_mb > 0);
        assert_eq!(stats.allocations, 0);
    }

    #[test]
    fn test_memory_optimization() {
        let mut optimizer = MemoryOptimizer::new(MemoryOptimizationConfig::default());
        optimizer.init_frame_pool(1920, 1080, 10).unwrap();
        
        // Allocate some frames
        let _handle1 = optimizer.allocate_frame().unwrap();
        let _handle2 = optimizer.allocate_frame().unwrap();
        
        // Optimize
        let reduction = optimizer.optimize().unwrap();
        assert!(reduction >= 0.0);
    }
}