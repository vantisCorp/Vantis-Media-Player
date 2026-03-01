//! Stream caching module
//! 
//! Provides intelligent stream caching for improved performance and
//! reduced bandwidth usage.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache directory path
    pub cache_dir: String,
    
    /// Maximum cache size in bytes
    pub max_cache_size: u64,
    
    /// Maximum cache duration in seconds
    pub max_cache_duration_secs: u64,
    
    /// Enable compression
    pub enable_compression: bool,
    
    /// Compression level (0-9)
    pub compression_level: u32,
    
    /// Enable encryption
    pub enable_encryption: bool,
    
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: ".cache/streaming".to_string(),
            max_cache_size: 2 * 1024 * 1024 * 1024, // 2 GB
            max_cache_duration_secs: 7 * 24 * 60 * 60, // 7 days
            enable_compression: false,
            compression_level: 6,
            enable_encryption: false,
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

/// Cache eviction policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    
    /// Least Frequently Used
    LFU,
    
    /// First In First Out
    FIFO,
    
    /// Size-based eviction
    Size,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Entry key
    pub key: String,
    
    /// Data
    pub data: Vec<u8>,
    
    /// Size in bytes
    pub size: u64,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last access timestamp
    pub last_accessed: u64,
    
    /// Access count
    pub access_count: usize,
    
    /// Expiration timestamp (0 = no expiration)
    pub expires_at: u64,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(key: String, data: Vec<u8>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            size: data.len() as u64,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            expires_at: 0,
            key,
            data,
            metadata: HashMap::new(),
        }
    }
    
    /// Check if entry is expired
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false;
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now > self.expires_at
    }
    
    /// Update access time
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.access_count += 1;
    }
}

/// Stream cache
pub struct StreamCache {
    config: CacheConfig,
    entries: HashMap<String, CacheEntry>,
    total_size: u64,
}

impl StreamCache {
    /// Create a new stream cache
    pub fn new(config: CacheConfig) -> StreamingResult<Self> {
        // Create cache directory
        if config.enabled {
            std::fs::create_dir_all(&config.cache_dir)?;
        }
        
        Ok(Self {
            config,
            entries: HashMap::new(),
            total_size: 0,
        })
    }
    
    /// Get a cache entry
    pub fn get(&mut self, key: &str) -> StreamingResult<Option<Vec<u8>>> {
        if !self.config.enabled {
            return Ok(None);
        }
        
        if let Some(entry) = self.entries.get_mut(key) {
            // Check if expired
            if entry.is_expired() {
                self.remove(key)?;
                return Ok(None);
            }
            
            // Update access time
            entry.touch();
            
            Ok(Some(entry.data.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Put a cache entry
    pub fn put(&mut self, key: String, data: Vec<u8>) -> StreamingResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Create entry
        let mut entry = CacheEntry::new(key.clone(), data);
        
        // Set expiration
        if self.config.max_cache_duration_secs > 0 {
            entry.expires_at = entry.created_at + self.config.max_cache_duration_secs;
        }
        
        // Check if we need to evict entries
        self.ensure_space(entry.size)?;
        
        // Add entry
        self.total_size += entry.size;
        self.entries.insert(key, entry);
        
        Ok(())
    }
    
    /// Remove a cache entry
    pub fn remove(&mut self, key: &str) -> StreamingResult<()> {
        if let Some(entry) = self.entries.remove(key) {
            self.total_size -= entry.size;
        }
        Ok(())
    }
    
    /// Clear all cache entries
    pub fn clear(&mut self) -> StreamingResult<()> {
        self.entries.clear();
        self.total_size = 0;
        Ok(())
    }
    
    /// Ensure there's enough space for a new entry
    fn ensure_space(&mut self, required_size: u64) -> StreamingResult<()> {
        while self.total_size + required_size > self.config.max_cache_size {
            if self.entries.is_empty() {
                return Err(StreamingError::CacheError(
                    "Cannot fit entry in cache".to_string()
                ));
            }
            
            self.evict_one()?;
        }
        
        Ok(())
    }
    
    /// Evict one entry based on policy
    fn evict_one(&mut self) -> StreamingResult<()> {
        if self.entries.is_empty() {
            return Ok(());
        }
        
        let key_to_remove = match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                self.entries
                    .iter()
                    .min_by_key(|(_, e)| e.last_accessed)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::LFU => {
                self.entries
                    .iter()
                    .min_by_key(|(_, e)| e.access_count)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::FIFO => {
                self.entries
                    .iter()
                    .min_by_key(|(_, e)| e.created_at)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::Size => {
                self.entries
                    .iter()
                    .max_by_key(|(_, e)| e.size)
                    .map(|(k, _)| k.clone())
            }
        };
        
        if let Some(key) = key_to_remove {
            self.remove(&key)?;
        }
        
        Ok(())
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let entry_count = self.entries.len();
        let hit_rate = if entry_count > 0 {
            let total_accesses: usize = self.entries.values().map(|e| e.access_count).sum();
            if total_accesses > 0 {
                let hits = self.entries.values().filter(|e| e.access_count > 1).count();
                hits as f32 / total_accesses as f32
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        CacheStats {
            entry_count,
            total_size: self.total_size,
            max_size: self.config.max_cache_size,
            hit_rate,
            utilization: self.total_size as f32 / self.config.max_cache_size as f32,
        }
    }
    
    /// Get cache directory path
    pub fn cache_dir(&self) -> &Path {
        Path::new(&self.config.cache_dir)
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of entries
    pub entry_count: usize,
    
    /// Total size in bytes
    pub total_size: u64,
    
    /// Maximum size in bytes
    pub max_size: u64,
    
    /// Hit rate (0.0 - 1.0)
    pub hit_rate: f32,
    
    /// Cache utilization (0.0 - 1.0)
    pub utilization: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_entry_creation() {
        let entry = CacheEntry::new("test_key".to_string(), vec![1, 2, 3]);
        assert_eq!(entry.key, "test_key");
        assert_eq!(entry.size, 3);
    }
    
    #[test]
    fn test_cache_entry_expiration() {
        let mut entry = CacheEntry::new("test_key".to_string(), vec![1, 2, 3]);
        entry.expires_at = 0;
        assert!(!entry.is_expired());
        
        entry.expires_at = 1;
        assert!(entry.is_expired());
    }
    
    #[test]
    fn test_stream_cache() {
        let config = CacheConfig {
            max_cache_size: 1024,
            ..Default::default()
        };
        let mut cache = StreamCache::new(config).unwrap();
        
        cache.put("key1".to_string(), vec![1, 2, 3]).unwrap();
        let result = cache.get("key1").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_cache_eviction() {
        let config = CacheConfig {
            max_cache_size: 10,
            eviction_policy: EvictionPolicy::LRU,
            ..Default::default()
        };
        let mut cache = StreamCache::new(config).unwrap();
        
        cache.put("key1".to_string(), vec![1, 2, 3]).unwrap();
        cache.put("key2".to_string(), vec![4, 5, 6]).unwrap();
        cache.put("key3".to_string(), vec![7, 8, 9]).unwrap();
        
        // Should have evicted one entry
        assert_eq!(cache.entries.len(), 2);
    }
}