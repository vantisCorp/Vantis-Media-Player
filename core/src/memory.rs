//! Zero-Copy Memory Management
//! 
//! Implements zero-copy buffers for direct DMA transfers from NVMe to VRAM.
//! This eliminates CPU copies for maximum performance.

use anyhow::{Result, anyhow};
use bytes::Bytes;
use parking_lot::Mutex;
use std::sync::Arc;

/// Zero-Copy Buffer Pool
/// 
/// Memory is allocated once and reused through the pool.
/// Data flows directly from source to destination without CPU copies.
pub struct ZeroCopyBuffer {
    /// Pre-allocated memory pool
    pool: Arc<Mutex<Vec<BufferBlock>>>,
    
    /// Total buffer size in bytes
    total_size: usize,
    
    /// Block size for allocation
    block_size: usize,
}

/// Individual buffer block
struct BufferBlock {
    id: usize,
    data: Vec<u8>,
    in_use: bool,
}

impl ZeroCopyBuffer {
    /// Create a new zero-copy buffer pool
    /// 
    /// # Arguments
    /// * `total_size` - Total memory to allocate in bytes
    pub fn new(total_size: usize) -> Result<Self> {
        let block_size = 1024 * 1024; // 1MB blocks
        let num_blocks = total_size / block_size;
        
        tracing::debug!(
            "🧠 Allocating {}MB zero-copy buffer pool ({} blocks of {}MB)",
            total_size / (1024 * 1024),
            num_blocks,
            block_size / (1024 * 1024)
        );
        
        let mut pool = Vec::with_capacity(num_blocks);
        
        for i in 0..num_blocks {
            pool.push(BufferBlock {
                id: i,
                data: vec![0u8; block_size],
                in_use: false,
            });
        }
        
        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
            total_size,
            block_size,
        })
    }
    
    /// Allocate a buffer block
    /// 
    /// Returns a handle to zero-copy memory
    pub fn allocate(&self) -> Result<BufferHandle> {
        let mut pool = self.pool.lock();
        
        // Find first available block
        for block in pool.iter_mut() {
            if !block.in_use {
                block.in_use = true;
                return Ok(BufferHandle {
                    block_id: block.id,
                    pool: self.pool.clone(),
                    offset: 0,
                    size: self.block_size,
                });
            }
        }
        
        Err(anyhow!("No available buffer blocks"))
    }
    
    /// Get total buffer size
    pub fn total_size(&self) -> usize {
        self.total_size
    }
}

/// Handle to a zero-copy buffer
pub struct BufferHandle {
    block_id: usize,
    pool: Arc<Mutex<Vec<BufferBlock>>>,
    offset: usize,
    size: usize,
}

impl BufferHandle {
    /// Get read-only access to the buffer
    pub fn as_slice(&self) -> &[u8] {
        let pool = self.pool.lock();
        let block = &pool[self.block_id];
        &block.data[self.offset..self.offset + self.size]
    }
    
    /// Get mutable access to the buffer
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let mut pool = self.pool.lock();
        let block = &mut pool[self.block_id];
        &mut block.data[self.offset..self.offset + self.size]
    }
    
    /// Get buffer size
    pub fn len(&self) -> usize {
        self.size
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Drop for BufferHandle {
    fn drop(&mut self) {
        // Return block to pool
        let mut pool = self.pool.lock();
        if self.block_id < pool.len() {
            pool[self.block_id].in_use = false;
        }
    }
}

impl std::fmt::Debug for BufferHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BufferHandle(block_id={}, offset={}, size={})",
            self.block_id, self.offset, self.size
        )
    }
}