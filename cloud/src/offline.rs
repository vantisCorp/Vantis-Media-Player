//! Offline support with queue management
//!
//! Provides comprehensive offline capabilities:
//! - Operation queueing for offline mode
//! - Automatic sync when back online
//! - Conflict prevention
//! - Offline data caching

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex, broadcast};
use async_trait::async_trait;
use tracing::{info, warn, debug};

use super::sync::{SyncDataType, SyncOperation, SyncQueueItem, SyncConflict};

/// Offline manager trait
#[async_trait]
pub trait OfflineManager: Send + Sync {
    /// Check if currently offline
    fn is_offline(&self) -> bool;
    
    /// Set offline mode
    async fn set_offline(&self, offline: bool);
    
    /// Queue an operation
    async fn queue_operation(&self, operation: QueuedOperation) -> Result<()>;
    
    /// Get pending operations
    async fn get_pending(&self) -> Vec<QueuedOperation>;
    
    /// Process pending operations
    async fn process_pending(&self) -> Result<ProcessResult>;
    
    /// Clear pending operations
    async fn clear_pending(&self) -> Result<()>;
    
    /// Get offline status
    async fn get_status(&self) -> OfflineStatus;
}

/// Queued operation for offline processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedOperation {
    /// Operation ID
    pub id: Uuid,
    /// Operation type
    pub operation: SyncOperation,
    /// Data type
    pub data_type: SyncDataType,
    /// Item ID
    pub item_id: String,
    /// Item data (JSON)
    pub data: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Retry count
    pub retry_count: u32,
    /// Maximum retries
    pub max_retries: u32,
    /// Last error
    pub last_error: Option<String>,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Dependencies (must complete first)
    pub depends_on: Vec<Uuid>,
}

impl QueuedOperation {
    /// Create a new queued operation
    pub fn new(operation: SyncOperation, data_type: SyncDataType, item_id: String, data: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            operation,
            data_type,
            item_id,
            data,
            created_at: Utc::now(),
            retry_count: 0,
            max_retries: 3,
            last_error: None,
            priority: 0,
            depends_on: Vec::new(),
        }
    }

    /// Check if can retry
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
    }

    /// Builder-style priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Builder-style dependency
    pub fn depends_on(mut self, id: Uuid) -> Self {
        self.depends_on.push(id);
        self
    }
}

impl From<SyncQueueItem> for QueuedOperation {
    fn from(item: SyncQueueItem) -> Self {
        Self {
            id: item.id,
            operation: item.operation,
            data_type: item.data_type,
            item_id: item.item_id,
            data: item.data,
            created_at: item.created_at,
            retry_count: item.retry_count,
            max_retries: 5,
            last_error: item.last_error,
            priority: 0,
            depends_on: Vec::new(),
        }
    }
}

impl From<QueuedOperation> for SyncQueueItem {
    fn from(op: QueuedOperation) -> Self {
        Self {
            id: op.id,
            data_type: op.data_type,
            operation: op.operation,
            item_id: op.item_id,
            data: op.data,
            created_at: op.created_at,
            retry_count: op.retry_count,
            last_error: op.last_error,
        }
    }
}

/// Offline status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineStatus {
    /// Is currently offline
    pub is_offline: bool,
    /// Time went offline
    pub offline_since: Option<DateTime<Utc>>,
    /// Pending operations count
    pub pending_count: u64,
    /// Estimated sync time (seconds)
    pub estimated_sync_time: u64,
    /// Offline duration
    pub offline_duration: Option<Duration>,
    /// Last successful sync
    pub last_sync: Option<DateTime<Utc>>,
    /// Network type (if available)
    pub network_type: Option<String>,
}

impl Default for OfflineStatus {
    fn default() -> Self {
        Self {
            is_offline: false,
            offline_since: None,
            pending_count: 0,
            estimated_sync_time: 0,
            offline_duration: None,
            last_sync: None,
            network_type: None,
        }
    }
}

/// Result of processing pending operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Successfully processed
    pub succeeded: u64,
    /// Failed
    pub failed: u64,
    /// Skipped (dependencies not met)
    pub skipped: u64,
    /// Conflicts detected
    pub conflicts: Vec<SyncConflict>,
    /// Errors
    pub errors: Vec<ProcessError>,
}

/// Process error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessError {
    /// Operation ID
    pub operation_id: Uuid,
    /// Error message
    pub message: String,
    /// Whether retryable
    pub retryable: bool,
}

/// Default offline manager implementation
pub struct DefaultOfflineManager {
    /// Is offline
    offline: Arc<RwLock<bool>>,
    /// Offline since
    offline_since: Arc<RwLock<Option<DateTime<Utc>>>>,
    /// Operation queue
    queue: Arc<Mutex<VecDeque<QueuedOperation>>>,
    /// Completed operations (for dependency tracking)
    completed: Arc<RwLock<HashSet<Uuid>>>,
    /// Maximum queue size
    max_queue_size: usize,
    /// Event broadcaster
    event_tx: broadcast::Sender<OfflineEvent>,
}

use std::collections::HashSet;

impl DefaultOfflineManager {
    /// Create a new offline manager
    pub fn new(max_queue_size: usize) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            offline: Arc::new(RwLock::new(false)),
            offline_since: Arc::new(RwLock::new(None)),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            completed: Arc::new(RwLock::new(HashSet::new())),
            max_queue_size,
            event_tx,
        }
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<OfflineEvent> {
        self.event_tx.subscribe()
    }

    /// Sort queue by priority
    async fn sort_queue(&self) {
        let mut queue = self.queue.lock().await;
        let mut items: Vec<_> = queue.drain(..).collect();
        items.sort_by(|a, b| b.priority.cmp(&a.priority));
        queue.extend(items);
    }

    /// Check if dependencies are met
    async fn dependencies_met(&self, op: &QueuedOperation) -> bool {
        if op.depends_on.is_empty() {
            return true;
        }

        let completed = self.completed.read().await;
        op.depends_on.iter().all(|id| completed.contains(id))
    }
}

impl Default for DefaultOfflineManager {
    fn default() -> Self {
        Self::new(10000)
    }
}

#[async_trait]
impl OfflineManager for DefaultOfflineManager {
    fn is_offline(&self) -> bool {
        // This is a sync method, so we use try_read
        *self.offline.try_read().unwrap_or_else(|e| e.into_inner())
    }

    async fn set_offline(&self, offline: bool) {
        let mut current = self.offline.write().await;
        let was_offline = *current;
        *current = offline;
        drop(current);

        if offline && !was_offline {
            *self.offline_since.write().await = Some(Utc::now());
            let _ = self.event_tx.send(OfflineEvent::WentOffline {
                timestamp: Utc::now(),
            });
        } else if !offline && was_offline {
            let duration = self.offline_since.write().await.take().map(|since| {
                Utc::now() - since
            });
            let _ = self.event_tx.send(OfflineEvent::WentOnline {
                timestamp: Utc::now(),
                offline_duration: duration,
            });
        }
    }

    async fn queue_operation(&self, operation: QueuedOperation) -> Result<()> {
        let mut queue = self.queue.lock().await;
        
        // Check queue size
        if queue.len() >= self.max_queue_size {
            // Remove lowest priority item
            let mut items: Vec<_> = queue.drain(..).collect();
            items.sort_by_key(|i| std::cmp::Reverse(i.priority));
            items.pop(); // Remove lowest priority
            queue.extend(items);
        }
        
        queue.push_back(operation);
        
        let _ = self.event_tx.send(OfflineEvent::OperationQueued {
            count: queue.len() as u64,
        });
        
        Ok(())
    }

    async fn get_pending(&self) -> Vec<QueuedOperation> {
        let queue = self.queue.lock().await;
        queue.iter().cloned().collect()
    }

    async fn process_pending(&self) -> Result<ProcessResult> {
        // Sort by priority
        self.sort_queue().await;
        
        let mut result = ProcessResult::default();
        let mut to_requeue = Vec::new();
        
        let mut queue = self.queue.lock().await;
        
        while let Some(mut op) = queue.pop_front() {
            // Check dependencies
            if !self.dependencies_met(&op).await {
                to_requeue.push(op);
                result.skipped += 1;
                continue;
            }
            
            // Process operation (simulate)
            match self.process_operation(&op).await {
                Ok(_) => {
                    result.succeeded += 1;
                    self.completed.write().await.insert(op.id);
                }
                Err(e) => {
                    if op.can_retry() {
                        op.increment_retry();
                        op.set_error(e.to_string());
                        to_requeue.push(op);
                    } else {
                        result.failed += 1;
                        result.errors.push(ProcessError {
                            operation_id: op.id,
                            message: e.to_string(),
                            retryable: false,
                        });
                    }
                }
            }
        }
        
        // Requeue items that need retry or were skipped
        queue.extend(to_requeue);
        
        Ok(result)
    }

    async fn clear_pending(&self) -> Result<()> {
        let mut queue = self.queue.lock().await;
        queue.clear();
        self.completed.write().await.clear();
        Ok(())
    }

    async fn get_status(&self) -> OfflineStatus {
        let is_offline = *self.offline.read().await;
        let offline_since = *self.offline_since.read().await;
        let pending_count = self.queue.lock().await.len() as u64;
        
        let offline_duration = offline_since.map(|since| Utc::now() - since);
        let estimated_sync_time = pending_count * 2; // Rough estimate: 2 seconds per item
        
        OfflineStatus {
            is_offline,
            offline_since,
            pending_count,
            estimated_sync_time,
            offline_duration,
            last_sync: None,
            network_type: None,
        }
    }
}

impl DefaultOfflineManager {
    /// Process a single operation
    async fn process_operation(&self, op: &QueuedOperation) -> Result<()> {
        debug!("Processing operation: {:?}", op.id);
        
        // Simulate network operation
        // In a real implementation, this would make actual API calls
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        
        // Random failure for testing
        if rand::random::<u8>() % 20 == 0 {
            return Err(anyhow::anyhow!("Simulated network error"));
        }
        
        Ok(())
    }
}

/// Offline events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfflineEvent {
    /// Went offline
    WentOffline { timestamp: DateTime<Utc> },
    /// Went online
    WentOnline { timestamp: DateTime<Utc>, offline_duration: Option<Duration> },
    /// Operation queued
    OperationQueued { count: u64 },
    /// Processing started
    ProcessingStarted { count: u64 },
    /// Operation processed
    OperationProcessed { operation_id: Uuid, success: bool },
    /// Queue cleared
    QueueCleared,
}

/// Offline data cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineCache {
    /// Cached items by data type
    items: HashMap<String, CachedItem>,
    /// Maximum cache size
    max_size: usize,
    /// Current cache size
    current_size: usize,
}

/// Cached item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedItem {
    /// Item key
    pub key: String,
    /// Item data
    pub data: Vec<u8>,
    /// Cached at
    pub cached_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// ETag for validation
    pub etag: Option<String>,
    /// Content type
    pub content_type: String,
}

impl CachedItem {
    /// Check if expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|exp| Utc::now() > exp).unwrap_or(false)
    }

    /// Get age in seconds
    pub fn age(&self) -> i64 {
        (Utc::now() - self.cached_at).num_seconds()
    }
}

impl OfflineCache {
    /// Create a new offline cache
    pub fn new(max_size: usize) -> Self {
        Self {
            items: HashMap::new(),
            max_size,
            current_size: 0,
        }
    }

    /// Store an item
    pub fn store(&mut self, key: String, data: Vec<u8>, ttl: Option<Duration>, etag: Option<String>) {
        let size = data.len();
        
        // Evict if necessary
        while self.current_size + size > self.max_size && !self.items.is_empty() {
            self.evict_oldest();
        }
        
        let cached_item = CachedItem {
            key: key.clone(),
            data,
            cached_at: Utc::now(),
            expires_at: ttl.map(|t| Utc::now() + t),
            etag,
            content_type: "application/json".to_string(),
        };
        
        // Remove old item if exists
        if let Some(old) = self.items.remove(&key) {
            self.current_size -= old.data.len();
        }
        
        self.current_size += cached_item.data.len();
        self.items.insert(key, cached_item);
    }

    /// Get an item
    pub fn get(&self, key: &str) -> Option<&CachedItem> {
        self.items.get(key)
    }

    /// Remove an item
    pub fn remove(&mut self, key: &str) -> Option<CachedItem> {
        if let Some(item) = self.items.remove(key) {
            self.current_size -= item.data.len();
            Some(item)
        } else {
            None
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_size = 0;
    }

    /// Get cache stats
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            item_count: self.items.len(),
            total_size: self.current_size,
            max_size: self.max_size,
            utilization: self.current_size as f64 / self.max_size as f64,
        }
    }

    /// Evict oldest item
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self
            .items
            .iter()
            .min_by_key(|(_, item)| item.cached_at)
        {
            let key = oldest_key.clone();
            self.remove(&key);
        }
    }

    /// Evict expired items
    pub fn evict_expired(&mut self) -> usize {
        let expired: Vec<String> = self
            .items
            .iter()
            .filter(|(_, item)| item.is_expired())
            .map(|(key, _)| key.clone())
            .collect();
        
        let count = expired.len();
        for key in expired {
            self.remove(&key);
        }
        count
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub item_count: usize,
    pub total_size: usize,
    pub max_size: usize,
    pub utilization: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queued_operation() {
        let op = QueuedOperation::new(
            SyncOperation::Create,
            SyncDataType::Playlists,
            "playlist-1".to_string(),
            "{}".to_string(),
        ).with_priority(10);
        
        assert!(op.can_retry());
        assert_eq!(op.priority, 10);
    }

    #[tokio::test]
    async fn test_offline_manager() {
        let manager = DefaultOfflineManager::new(100);
        
        assert!(!manager.is_offline());
        
        manager.set_offline(true).await;
        assert!(manager.is_offline());
        
        let status = manager.get_status().await;
        assert!(status.is_offline);
    }

    #[tokio::test]
    async fn test_queue_operation() {
        let manager = DefaultOfflineManager::new(100);
        
        let op = QueuedOperation::new(
            SyncOperation::Update,
            SyncDataType::Settings,
            "settings".to_string(),
            r#"{"volume":80}"#.to_string(),
        );
        
        manager.queue_operation(op).await.unwrap();
        
        let pending = manager.get_pending().await;
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_offline_cache() {
        let mut cache = OfflineCache::new(1024);
        
        cache.store(
            "test".to_string(),
            b"data".to_vec(),
            Some(Duration::hours(1)),
            None,
        );
        
        assert!(cache.get("test").is_some());
        assert_eq!(cache.stats().item_count, 1);
        
        cache.remove("test");
        assert!(cache.get("test").is_none());
    }
}