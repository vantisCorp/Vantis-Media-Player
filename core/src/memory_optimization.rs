//! Memory Optimization Module
//!
//! Comprehensive memory optimization to reduce memory usage by 20%.
//! Implements memory pooling, buffer optimization, and lazy loading.

use anyhow::{Result, anyhow};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};

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
    
    /// Enable memory monitoring
    pub enable_monitoring: bool,
    
    /// Memory warning threshold percentage
    pub warning_threshold: u8,
    
    /// Memory critical threshold percentage
    pub critical_threshold: u8,
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_pooling: true,
            enable_lazy_loading: true,
            enable_compression: false,
            target_reduction: 20,
            max_pool_size_mb: 512,
            enable_monitoring: true,
            warning_threshold: 80,
            critical_threshold: 90,
        }
    }
}

/// Memory pool for video frames
#[allow(dead_code)]
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
    
    /// Statistics
    stats: Arc<RwLock<PoolStats>>,
}

/// Pooled video frame
#[allow(dead_code)]
struct PooledFrame {
    id: u64,
    data: Vec<u8>,
    in_use: bool,
    last_used: Instant,
    use_count: u64,
}

/// Pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub total_allocations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub current_in_use: usize,
    pub peak_usage: usize,
    pub memory_saved_bytes: u64,
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
                id: i as u64,
                data: vec![0u8; frame_size],
                in_use: false,
                last_used: Instant::now(),
                use_count: 0,
            };
            pool.insert(i as u64, frame);
            available.push(i as u64);
        }
        
        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
            available: Arc::new(Mutex::new(available)),
            frame_size,
            max_frames,
            current_size: Arc::new(Mutex::new(0)),
            stats: Arc::new(RwLock::new(PoolStats::default())),
        })
    }
    
    /// Acquire a frame from the pool
    pub fn acquire(&self) -> Result<FrameHandle> {
        let mut available = self.available.lock();
        
        if let Some(id) = available.pop() {
            let mut pool = self.pool.lock();
            if let Some(frame) = pool.get_mut(&id) {
                frame.in_use = true;
                frame.last_used = Instant::now();
                frame.use_count += 1;
                
                // Update stats
                let mut stats = self.stats.write();
                stats.total_allocations += 1;
                stats.cache_hits += 1;
                stats.current_in_use += 1;
                stats.peak_usage = stats.peak_usage.max(stats.current_in_use);
                stats.memory_saved_bytes += self.frame_size as u64;
                
                return Ok(FrameHandle {
                    id,
                    pool: self.pool.clone(),
                    available: self.available.clone(),
                    stats: self.stats.clone(),
                });
            }
        }
        
        // No available frames
        let mut stats = self.stats.write();
        stats.cache_misses += 1;
        
        Err(anyhow!("No available frames in pool"))
    }
    
    /// Get pool statistics
    pub fn get_stats(&self) -> PoolStats {
        self.stats.read().clone()
    }
    
    /// Get current pool usage
    pub fn get_usage(&self) -> f64 {
        let stats = self.stats.read();
        stats.current_in_use as f64 / self.max_frames as f64 * 100.0
    }
    
    /// Shrink pool if possible
    pub fn shrink(&self, target_reduction: usize) -> Result<usize> {
        let mut pool = self.pool.lock();
        let mut available = self.available.lock();
        
        let mut removed = 0;
        let mut to_remove = Vec::new();
        
        // Find frames that can be removed (not in use, old)
        for (id, frame) in pool.iter() {
            if !frame.in_use && frame.last_used.elapsed() > Duration::from_secs(60) {
                to_remove.push(*id);
                if to_remove.len() >= target_reduction {
                    break;
                }
            }
        }
        
        // Remove frames
        for id in to_remove {
            pool.remove(&id);
            available.retain(|&x| x != id);
            removed += 1;
        }
        
        if removed > 0 {
            info!("🗑️ Shrunk frame pool by {} frames", removed);
        }
        
        Ok(removed)
    }
}

/// Frame handle
pub struct FrameHandle {
    id: u64,
    pool: Arc<Mutex<HashMap<u64, PooledFrame>>>,
    available: Arc<Mutex<Vec<u64>>>,
    stats: Arc<RwLock<PoolStats>>,
}

impl FrameHandle {
    /// Get frame data
    pub fn data(&self) -> Vec<u8> {
        let pool = self.pool.lock();
        pool.get(&self.id).map(|f| f.data.clone()).unwrap_or_default()
    }
    
    /// Write data into the frame
    pub fn write_data(&mut self, data: &[u8]) {
        let mut pool = self.pool.lock();
        if let Some(frame) = pool.get_mut(&self.id) {
            let len = data.len().min(frame.data.len());
            frame.data[..len].copy_from_slice(&data[..len]);
        }
    }
}

impl Drop for FrameHandle {
    fn drop(&mut self) {
        let mut pool = self.pool.lock();
        if let Some(frame) = pool.get_mut(&self.id) {
            frame.in_use = false;
            frame.last_used = Instant::now();
        }
        
        self.available.lock().push(self.id);
        
        let mut stats = self.stats.write();
        stats.current_in_use = stats.current_in_use.saturating_sub(1);
    }
}

/// Buffer pool optimization
pub struct BufferPoolOptimization {
    /// Small buffer pool (< 4KB)
    small_pool: Arc<Mutex<Vec<Vec<u8>>>>,
    
    /// Medium buffer pool (4KB - 64KB)
    medium_pool: Arc<Mutex<Vec<Vec<u8>>>>,
    
    /// Large buffer pool (64KB - 1MB)
    large_pool: Arc<Mutex<Vec<Vec<u8>>>>,
    
    /// Statistics
    stats: Arc<RwLock<BufferStats>>,
}

/// Buffer statistics
#[derive(Debug, Clone, Default)]
pub struct BufferStats {
    pub small_allocations: u64,
    pub medium_allocations: u64,
    pub large_allocations: u64,
    pub memory_fragmentation: f64,
    pub total_memory_bytes: usize,
}

impl BufferPoolOptimization {
    /// Create new buffer pool optimization
    pub fn new() -> Self {
        Self {
            small_pool: Arc::new(Mutex::new(Vec::new())),
            medium_pool: Arc::new(Mutex::new(Vec::new())),
            large_pool: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(RwLock::new(BufferStats::default())),
        }
    }
    
    /// Allocate a buffer with optimal size class
    pub fn allocate(&self, size: usize) -> Result<BufferHandle> {
        let (pool, size_class) = if size < 4096 {
            (&self.small_pool, SizeClass::Small)
        } else if size < 65536 {
            (&self.medium_pool, SizeClass::Medium)
        } else {
            (&self.large_pool, SizeClass::Large)
        };
        
        let mut pool_lock = pool.lock();
        
        // Try to reuse existing buffer
        if let Some(mut buffer) = pool_lock.pop() {
            if buffer.capacity() >= size {
                buffer.resize(size, 0);
                return Ok(BufferHandle {
                    data: buffer,
                    size_class,
                });
            }
        }
        
        // Allocate new buffer
        let buffer = vec![0u8; size];
        
        let mut stats = self.stats.write();
        match size_class {
            SizeClass::Small => stats.small_allocations += 1,
            SizeClass::Medium => stats.medium_allocations += 1,
            SizeClass::Large => stats.large_allocations += 1,
        }
        stats.total_memory_bytes += size;
        
        Ok(BufferHandle {
            data: buffer,
            size_class,
        })
    }
    
    /// Get buffer statistics
    pub fn get_stats(&self) -> BufferStats {
        self.stats.read().clone()
    }
}

/// Size class for buffers
#[derive(Debug, Clone, Copy, PartialEq)]
enum SizeClass {
    Small,
    Medium,
    Large,
}

/// Buffer handle
#[allow(dead_code)]
pub struct BufferHandle {
    data: Vec<u8>,
    size_class: SizeClass,
}

impl BufferHandle {
    /// Get buffer data
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    /// Get mutable buffer data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
    
    /// Get buffer size
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Memory monitor
pub struct MemoryMonitor {
    /// Current memory usage
    current_usage: Arc<Mutex<usize>>,
    
    /// Peak memory usage
    peak_usage: Arc<Mutex<usize>>,
    
    /// Warning threshold in bytes
    warning_threshold: usize,
    
    /// Critical threshold in bytes
    critical_threshold: usize,
    
    /// Total system memory
    total_memory: usize,
}

impl MemoryMonitor {
    /// Create new memory monitor
    pub fn new(total_memory: usize, warning_threshold: u8, critical_threshold: u8) -> Self {
        Self {
            current_usage: Arc::new(Mutex::new(0)),
            peak_usage: Arc::new(Mutex::new(0)),
            warning_threshold: (total_memory as f64 * warning_threshold as f64 / 100.0) as usize,
            critical_threshold: (total_memory as f64 * critical_threshold as f64 / 100.0) as usize,
            total_memory,
        }
    }
    
    /// Record memory allocation
    pub fn record_allocation(&self, size: usize) {
        let mut current = self.current_usage.lock();
        *current += size;
        
        let mut peak = self.peak_usage.lock();
        *peak = (*peak).max(*current);
        
        // Check thresholds
        if *current >= self.critical_threshold {
            warn!("🔴 Critical memory usage: {}MB / {}MB", 
                  *current / (1024 * 1024), 
                  self.total_memory / (1024 * 1024));
        } else if *current >= self.warning_threshold {
            warn!("🟡 High memory usage: {}MB / {}MB",
                  *current / (1024 * 1024),
                  self.total_memory / (1024 * 1024));
        }
    }
    
    /// Record memory deallocation
    pub fn record_deallocation(&self, size: usize) {
        let mut current = self.current_usage.lock();
        *current = current.saturating_sub(size);
    }
    
    /// Get current memory usage
    pub fn get_current_usage(&self) -> usize {
        *self.current_usage.lock()
    }
    
    /// Get peak memory usage
    pub fn get_peak_usage(&self) -> usize {
        *self.peak_usage.lock()
    }
    
    /// Get memory usage percentage
    pub fn get_usage_percentage(&self) -> f64 {
        *self.current_usage.lock() as f64 / self.total_memory as f64 * 100.0
    }
}

/// Lazy loading manager for plugins
pub struct LazyLoader<T> {
    /// Loader function
    loader: Box<dyn Fn() -> Result<T> + Send + Sync>,
    
    /// Loaded value
    value: Arc<Mutex<Option<T>>>,
    
    /// Is loaded
    is_loaded: Arc<Mutex<bool>>,
}

impl<T: Clone + Send + Sync + 'static> LazyLoader<T> {
    /// Create new lazy loader
    pub fn new<F>(loader: F) -> Self
    where
        F: Fn() -> Result<T> + Send + Sync + 'static,
    {
        Self {
            loader: Box::new(loader),
            value: Arc::new(Mutex::new(None)),
            is_loaded: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Get the value, loading if necessary
    pub fn get(&self) -> Result<T> {
        let mut is_loaded = self.is_loaded.lock();
        
        if *is_loaded {
            let value = self.value.lock();
            value.clone().ok_or_else(|| anyhow!("Value not loaded"))
        } else {
            let value = (self.loader)()?;
            *self.value.lock() = Some(value.clone());
            *is_loaded = true;
            Ok(value)
        }
    }
    
    /// Check if loaded
    pub fn is_loaded(&self) -> bool {
        *self.is_loaded.lock()
    }
    
    /// Unload the value
    pub fn unload(&self) {
        let mut is_loaded = self.is_loaded.lock();
        *is_loaded = false;
        *self.value.lock() = None;
    }
}

/// Memory optimizer
#[allow(dead_code)]
pub struct MemoryOptimizer {
    /// Configuration
    config: MemoryOptimizationConfig,
    
    /// Video frame pool
    frame_pool: Option<VideoFramePool>,
    
    /// Buffer pool
    buffer_pool: BufferPoolOptimization,
    
    /// Memory monitor
    monitor: MemoryMonitor,
    
    /// Memory saved
    memory_saved: Arc<Mutex<u64>>,
}

impl MemoryOptimizer {
    /// Create new memory optimizer
    pub fn new(config: MemoryOptimizationConfig) -> Result<Self> {
        info!("🔧 Initializing memory optimizer");
        info!("   Target reduction: {}%", config.target_reduction);
        info!("   Pooling enabled: {}", config.enable_pooling);
        info!("   Lazy loading enabled: {}", config.enable_lazy_loading);
        
        // Estimate total memory (default to 8GB)
        let total_memory = 8 * 1024 * 1024 * 1024;
        
        let monitor = MemoryMonitor::new(
            total_memory,
            config.warning_threshold,
            config.critical_threshold,
        );
        
        Ok(Self {
            config,
            frame_pool: None,
            buffer_pool: BufferPoolOptimization::new(),
            monitor,
            memory_saved: Arc::new(Mutex::new(0)),
        })
    }
    
    /// Initialize frame pool
    pub fn init_frame_pool(&mut self, width: u32, height: u32, max_frames: usize) -> Result<()> {
        self.frame_pool = Some(VideoFramePool::new(width, height, max_frames)?);
        info!("✅ Frame pool initialized");
        Ok(())
    }
    
    /// Acquire a frame
    pub fn acquire_frame(&self) -> Result<FrameHandle> {
        if let Some(ref pool) = self.frame_pool {
            pool.acquire()
        } else {
            Err(anyhow!("Frame pool not initialized"))
        }
    }
    
    /// Allocate a buffer
    pub fn allocate_buffer(&self, size: usize) -> Result<BufferHandle> {
        self.buffer_pool.allocate(size)
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let frame_stats = self.frame_pool.as_ref().map(|p| p.get_stats());
        let buffer_stats = self.buffer_pool.get_stats();
        
        MemoryStats {
            frame_pool: frame_stats,
            buffer_pool: buffer_stats,
            current_usage: self.monitor.get_current_usage(),
            peak_usage: self.monitor.get_peak_usage(),
            usage_percentage: self.monitor.get_usage_percentage(),
            memory_saved: *self.memory_saved.lock(),
        }
    }
    
    /// Run memory optimization
    pub fn optimize(&self) -> Result<OptimizationResult> {
        let mut result = OptimizationResult::default();
        
        // Shrink frame pool if possible
        if let Some(ref pool) = self.frame_pool {
            let frames_removed = pool.shrink(10)?;
            result.frames_removed = frames_removed;
            result.memory_freed += frames_removed as u64 * pool.frame_size as u64;
        }
        
        // Update memory saved
        let mut saved = self.memory_saved.lock();
        *saved += result.memory_freed;
        
        Ok(result)
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub frame_pool: Option<PoolStats>,
    pub buffer_pool: BufferStats,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub usage_percentage: f64,
    pub memory_saved: u64,
}

/// Optimization result
#[derive(Debug, Clone, Default)]
pub struct OptimizationResult {
    pub frames_removed: usize,
    pub buffers_freed: usize,
    pub memory_freed: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frame_pool_creation() {
        let pool = VideoFramePool::new(1920, 1080, 10);
        assert!(pool.is_ok());
    }
    
    #[test]
    fn test_frame_acquire_release() {
        let pool = VideoFramePool::new(1920, 1080, 10).unwrap();
        let frame = pool.acquire();
        assert!(frame.is_ok());
        
        // Release frame
        drop(frame);
        
        // Should be able to acquire again
        let frame2 = pool.acquire();
        assert!(frame2.is_ok());
    }
    
    #[test]
    fn test_buffer_pool() {
        let pool = BufferPoolOptimization::new();
        let buffer = pool.allocate(1024);
        assert!(buffer.is_ok());
        assert_eq!(buffer.unwrap().len(), 1024);
    }
    
    #[test]
    fn test_lazy_loader() {
        let loader = LazyLoader::new(|| Ok(42));
        assert!(!loader.is_loaded());
        
        let value = loader.get();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 42);
        assert!(loader.is_loaded());
        
        loader.unload();
        assert!(!loader.is_loaded());
    }
    
    #[test]
    fn test_memory_monitor() {
        let monitor = MemoryMonitor::new(1024 * 1024 * 1024, 80, 90);
        monitor.record_allocation(1024 * 1024 * 100);
        assert_eq!(monitor.get_current_usage(), 1024 * 1024 * 100);
        
        monitor.record_deallocation(1024 * 1024 * 50);
        assert_eq!(monitor.get_current_usage(), 1024 * 1024 * 50);
    }
}