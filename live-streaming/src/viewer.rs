//! Viewer service for watching streams and chat

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::error::{LiveStreamingError, LiveStreamingResult};
use crate::types::*;
use crate::broadcaster::BroadcasterService;

/// Trait for viewer operations
#[async_trait]
pub trait ViewerService: Send + Sync {
    /// Create a viewer profile
    async fn create_viewer(&self, username: String, display_name: String) -> LiveStreamingResult<ViewerProfile>;

    /// Get viewer by ID
    async fn get_viewer(&self, viewer_id: &ViewerId) -> LiveStreamingResult<ViewerProfile>;

    /// Join a stream as a viewer
    async fn join_stream(&self, stream_id: &StreamId, viewer_id: &ViewerId) -> LiveStreamingResult<StreamViewer>;

    /// Leave a stream
    async fn leave_stream(&self, stream_id: &StreamId, viewer_id: &ViewerId) -> LiveStreamingResult<()>;

    /// Get viewers in a stream
    async fn get_stream_viewers(&self, stream_id: &StreamId, limit: usize, offset: usize) -> LiveStreamingResult<Vec<StreamViewer>>;

    /// Update viewer quality preference
    async fn set_quality(&self, stream_id: &StreamId, viewer_id: &ViewerId, quality: Resolution) -> LiveStreamingResult<()>;

    /// Follow a broadcaster
    async fn follow_broadcaster(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<()>;

    /// Unfollow a broadcaster
    async fn unfollow_broadcaster(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<()>;

    /// Check if viewer follows broadcaster
    async fn is_following(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<bool>;

    /// Get followed broadcasters
    async fn get_followed_broadcasters(&self, viewer_id: &ViewerId) -> LiveStreamingResult<Vec<BroadcasterId>>;

    /// Get live streams from followed broadcasters
    async fn get_followed_live_streams(&self, viewer_id: &ViewerId) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Subscribe to stream events
    async fn subscribe(&self, stream_id: &StreamId) -> LiveStreamingResult<broadcast::Receiver<StreamEvent>>;
}

/// Default implementation of ViewerService
pub struct DefaultViewerService {
    viewers: DashMap<ViewerId, ViewerProfile>,
    stream_viewers: DashMap<StreamId, Vec<StreamViewer>>,
    active_viewers: DashMap<(StreamId, ViewerId), StreamViewer>,
    follows: DashMap<ViewerId, Vec<BroadcasterId>>,
    event_senders: RwLock<HashMap<StreamId, broadcast::Sender<StreamEvent>>>,
    broadcaster_service: Arc<dyn BroadcasterService>,
}

impl DefaultViewerService {
    pub fn new(broadcaster_service: Arc<dyn BroadcasterService>) -> Self {
        Self {
            viewers: DashMap::new(),
            stream_viewers: DashMap::new(),
            active_viewers: DashMap::new(),
            follows: DashMap::new(),
            event_senders: RwLock::new(HashMap::new()),
            broadcaster_service,
        }
    }

    fn get_or_create_event_sender(&self, stream_id: &StreamId) -> broadcast::Sender<StreamEvent> {
        let mut senders = self.event_senders.write();
        senders.entry(stream_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1000);
            tx
        }).clone()
    }

    fn broadcast_event(&self, stream_id: &StreamId, event: StreamEvent) {
        let senders = self.event_senders.read();
        if let Some(sender) = senders.get(stream_id) {
            let _ = sender.send(event);
        }
    }
}

#[async_trait]
impl ViewerService for DefaultViewerService {
    async fn create_viewer(&self, username: String, display_name: String) -> LiveStreamingResult<ViewerProfile> {
        let viewer = ViewerProfile {
            id: ViewerId::new(),
            username,
            display_name,
            avatar_url: None,
            is_following: false,
            is_subscribed: false,
            is_moderator: false,
            is_banned: false,
            is_vip: false,
            badges: Vec::new(),
            watch_time_minutes: 0,
            first_seen: Utc::now(),
        };

        self.viewers.insert(viewer.id.clone(), viewer.clone());
        Ok(viewer)
    }

    async fn get_viewer(&self, viewer_id: &ViewerId) -> LiveStreamingResult<ViewerProfile> {
        self.viewers
            .get(viewer_id)
            .map(|r| r.clone())
            .ok_or_else(|| LiveStreamingError::ViewerNotFound(viewer_id.0.to_string()))
    }

    async fn join_stream(&self, stream_id: &StreamId, viewer_id: &ViewerId) -> LiveStreamingResult<StreamViewer> {
        // Verify viewer exists
        let _viewer = self.get_viewer(viewer_id).await?;

        // Get stream and verify it's live
        let stream = self.broadcaster_service.get_stream(stream_id).await?;
        if stream.status != StreamStatus::Live {
            return Err(LiveStreamingError::StreamNotLive);
        }

        // Check if already joined
        if self.active_viewers.contains_key(&(stream_id.clone(), viewer_id.clone())) {
            return Err(LiveStreamingError::InternalError("Already joined stream".to_string()));
        }

        let stream_viewer = StreamViewer {
            viewer_id: viewer_id.clone(),
            joined_at: Utc::now(),
            left_at: None,
            watch_duration_seconds: 0,
            quality: Resolution::Auto { width: 0, height: 0 },
            is_paused: false,
            playback_offset_seconds: 0,
            country: None,
            referrer: None,
        };

        // Add to active viewers
        self.active_viewers.insert((stream_id.clone(), viewer_id.clone()), stream_viewer.clone());

        // Add to stream viewers
        let mut viewers = self.stream_viewers.entry(stream_id.clone()).or_insert_with(Vec::new);
        viewers.push(stream_viewer.clone());

        // Broadcast viewer joined event
        self.broadcast_event(stream_id, StreamEvent {
            id: uuid::Uuid::new_v4(),
            stream_id: stream_id.clone(),
            event_type: StreamEventType::ViewerJoined,
            timestamp: Utc::now(),
            data: serde_json::json!({ "viewer_id": viewer_id.0.to_string() }),
        });

        Ok(stream_viewer)
    }

    async fn leave_stream(&self, stream_id: &StreamId, viewer_id: &ViewerId) -> LiveStreamingResult<()> {
        // Remove from active viewers
        if let Some((_, mut stream_viewer)) = self.active_viewers.remove(&(stream_id.clone(), viewer_id.clone())) {
            stream_viewer.left_at = Some(Utc::now());

            // Calculate watch duration
            if let Some(joined) = stream_viewer.joined_at {
                let duration = (Utc::now() - joined).num_seconds() as u64;
                stream_viewer.watch_duration_seconds = duration;

                // Update viewer watch time
                if let Some(mut viewer) = self.viewers.get_mut(viewer_id) {
                    viewer.watch_time_minutes += duration / 60;
                }
            }
        }

        // Remove from stream viewers
        if let Some(mut viewers) = self.stream_viewers.get_mut(stream_id) {
            viewers.retain(|v| &v.viewer_id != viewer_id);
        }

        // Broadcast viewer left event
        self.broadcast_event(stream_id, StreamEvent {
            id: uuid::Uuid::new_v4(),
            stream_id: stream_id.clone(),
            event_type: StreamEventType::ViewerLeft,
            timestamp: Utc::now(),
            data: serde_json::json!({ "viewer_id": viewer_id.0.to_string() }),
        });

        Ok(())
    }

    async fn get_stream_viewers(&self, stream_id: &StreamId, limit: usize, offset: usize) -> LiveStreamingResult<Vec<StreamViewer>> {
        let viewers = self.stream_viewers
            .get(stream_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        Ok(viewers.into_iter().skip(offset).take(limit).collect())
    }

    async fn set_quality(&self, stream_id: &StreamId, viewer_id: &ViewerId, quality: Resolution) -> LiveStreamingResult<()> {
        if let Some(mut stream_viewer) = self.active_viewers.get_mut(&(stream_id.clone(), viewer_id.clone())) {
            stream_viewer.quality = quality;
            Ok(())
        } else {
            Err(LiveStreamingError::InternalError("Not viewing stream".to_string()))
        }
    }

    async fn follow_broadcaster(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<()> {
        // Verify viewer and broadcaster exist
        self.get_viewer(viewer_id).await?;
        self.broadcaster_service.get_broadcaster(broadcaster_id).await?;

        // Add to follows
        let mut follows = self.follows.entry(viewer_id.clone()).or_insert_with(Vec::new);
        if !follows.contains(broadcaster_id) {
            follows.push(broadcaster_id.clone());
        }

        Ok(())
    }

    async fn unfollow_broadcaster(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<()> {
        if let Some(mut follows) = self.follows.get_mut(viewer_id) {
            follows.retain(|id| id != broadcaster_id);
        }
        Ok(())
    }

    async fn is_following(&self, viewer_id: &ViewerId, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<bool> {
        let follows = self.follows
            .get(viewer_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        Ok(follows.contains(broadcaster_id))
    }

    async fn get_followed_broadcasters(&self, viewer_id: &ViewerId) -> LiveStreamingResult<Vec<BroadcasterId>> {
        let follows = self.follows
            .get(viewer_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        Ok(follows)
    }

    async fn get_followed_live_streams(&self, viewer_id: &ViewerId) -> LiveStreamingResult<Vec<LiveStream>> {
        let follows = self.get_followed_broadcasters(viewer_id).await?;
        let mut live_streams = Vec::new();

        for broadcaster_id in follows {
            if let Ok(Some(stream)) = self.broadcaster_service.get_active_stream(&broadcaster_id).await {
                live_streams.push(stream);
            }
        }

        Ok(live_streams)
    }

    async fn subscribe(&self, stream_id: &StreamId) -> LiveStreamingResult<broadcast::Receiver<StreamEvent>> {
        let sender = self.get_or_create_event_sender(stream_id);
        Ok(sender.subscribe())
    }
}

/// Discovery service for finding streams
#[async_trait]
pub trait DiscoveryService: Send + Sync {
    /// Get live streams by category
    async fn get_streams_by_category(&self, category_id: &str, limit: usize, offset: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get streams by tag
    async fn get_streams_by_tag(&self, tag: &str, limit: usize, offset: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Search streams
    async fn search_streams(&self, query: &str, limit: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get recommended streams for viewer
    async fn get_recommended_streams(&self, viewer_id: &ViewerId, limit: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get top streams by viewer count
    async fn get_top_streams(&self, limit: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get featured streams
    async fn get_featured_streams(&self, limit: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get all categories
    async fn get_categories(&self) -> LiveStreamingResult<Vec<StreamCategory>>;
}

/// Default implementation of DiscoveryService
pub struct DefaultDiscoveryService {
    broadcaster_service: Arc<dyn BroadcasterService>,
    featured_streams: RwLock<Vec<StreamId>>,
    categories: Vec<StreamCategory>,
}

impl DefaultDiscoveryService {
    pub fn new(broadcaster_service: Arc<dyn BroadcasterService>) -> Self {
        Self {
            broadcaster_service,
            featured_streams: RwLock::new(Vec::new()),
            categories: vec![
                StreamCategory::new("gaming", "Gaming"),
                StreamCategory::new("music", "Music"),
                StreamCategory::new("creative", "Creative"),
                StreamCategory::new("irl", "Just Chatting"),
                StreamCategory::new("sports", "Sports"),
                StreamCategory::new("education", "Education"),
                StreamCategory::new("technology", "Technology"),
                StreamCategory::new("entertainment", "Entertainment"),
            ],
        }
    }
}

#[async_trait]
impl DiscoveryService for DefaultDiscoveryService {
    async fn get_streams_by_category(&self, _category_id: &str, _limit: usize, _offset: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        // In a real implementation, this would query a database
        Ok(Vec::new())
    }

    async fn get_streams_by_tag(&self, _tag: &str, _limit: usize, _offset: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        Ok(Vec::new())
    }

    async fn search_streams(&self, _query: &str, _limit: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        Ok(Vec::new())
    }

    async fn get_recommended_streams(&self, _viewer_id: &ViewerId, _limit: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        Ok(Vec::new())
    }

    async fn get_top_streams(&self, _limit: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        Ok(Vec::new())
    }

    async fn get_featured_streams(&self, limit: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        let featured = self.featured_streams.read();
        let mut streams = Vec::new();

        for stream_id in featured.iter().take(limit) {
            if let Ok(stream) = self.broadcaster_service.get_stream(stream_id).await {
                if stream.status == StreamStatus::Live {
                    streams.push(stream);
                }
            }
        }

        Ok(streams)
    }

    async fn get_categories(&self) -> LiveStreamingResult<Vec<StreamCategory>> {
        Ok(self.categories.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broadcaster::DefaultBroadcasterService;

    #[tokio::test]
    async fn test_create_viewer() {
        let broadcaster_service = Arc::new(DefaultBroadcasterService::new());
        let service = DefaultViewerService::new(broadcaster_service);

        let viewer = service.create_viewer("testviewer".to_string(), "Test Viewer".to_string()).await.unwrap();
        assert_eq!(viewer.username, "testviewer");
    }

    #[tokio::test]
    async fn test_follow_broadcaster() {
        let broadcaster_service = Arc::new(DefaultBroadcasterService::new());
        let service = DefaultViewerService::new(broadcaster_service.clone());

        let viewer = service.create_viewer("viewer".to_string(), "Viewer".to_string()).await.unwrap();
        let broadcaster = broadcaster_service.create_broadcaster("streamer".to_string(), "Streamer".to_string()).await.unwrap();

        service.follow_broadcaster(&viewer.id, &broadcaster.id).await.unwrap();

        let is_following = service.is_following(&viewer.id, &broadcaster.id).await.unwrap();
        assert!(is_following);
    }
}