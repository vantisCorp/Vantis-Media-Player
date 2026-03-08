//! Broadcaster service for managing live streams

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{LiveStreamingError, LiveStreamingResult};
use crate::types::*;

/// Trait for broadcaster operations
#[async_trait]
pub trait BroadcasterService: Send + Sync {
    /// Create a new broadcaster profile
    async fn create_broadcaster(&self, username: String, display_name: String) -> LiveStreamingResult<BroadcasterProfile>;

    /// Get broadcaster by ID
    async fn get_broadcaster(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<BroadcasterProfile>;

    /// Get broadcaster by username
    async fn get_broadcaster_by_username(&self, username: &str) -> LiveStreamingResult<BroadcasterProfile>;

    /// Update broadcaster profile
    async fn update_profile(&self, broadcaster_id: &BroadcasterId, updates: ProfileUpdate) -> LiveStreamingResult<BroadcasterProfile>;

    /// Update broadcaster settings
    async fn update_settings(&self, broadcaster_id: &BroadcasterId, settings: BroadcasterSettings) -> LiveStreamingResult<()>;

    /// Regenerate stream key
    async fn regenerate_stream_key(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<String>;

    /// Get ingestion endpoint
    async fn get_ingestion_endpoint(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<IngestionEndpoint>;

    /// Create a new stream
    async fn create_stream(&self, broadcaster_id: &BroadcasterId, title: String, category: StreamCategory) -> LiveStreamingResult<LiveStream>;

    /// Start a stream
    async fn start_stream(&self, stream_id: &StreamId, stream_key: &str) -> LiveStreamingResult<LiveStream>;

    /// End a stream
    async fn end_stream(&self, stream_id: &StreamId) -> LiveStreamingResult<LiveStream>;

    /// Get stream by ID
    async fn get_stream(&self, stream_id: &StreamId) -> LiveStreamingResult<LiveStream>;

    /// Get active stream for broadcaster
    async fn get_active_stream(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<Option<LiveStream>>;

    /// Update stream info
    async fn update_stream_info(&self, stream_id: &StreamId, updates: StreamInfoUpdate) -> LiveStreamingResult<LiveStream>;

    /// Get broadcaster's past streams
    async fn get_past_streams(&self, broadcaster_id: &BroadcasterId, limit: usize, offset: usize) -> LiveStreamingResult<Vec<LiveStream>>;

    /// Get broadcaster statistics
    async fn get_broadcaster_stats(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<BroadcasterStats>;
}

/// Profile update request
#[derive(Debug, Clone, Default)]
pub struct ProfileUpdate {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}

/// Stream info update request
#[derive(Debug, Clone, Default)]
pub struct StreamInfoUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<StreamCategory>,
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
    pub is_mature: Option<bool>,
}

/// Broadcaster statistics
#[derive(Debug, Clone, Default)]
pub struct BroadcasterStats {
    pub total_streams: u64,
    pub total_stream_time_hours: f64,
    pub total_views: u64,
    pub average_viewers: f64,
    pub peak_viewers: u64,
    pub followers_gained: u64,
    pub subscribers_gained: u64,
    pub total_chat_messages: u64,
    pub total_donations: f64,
}

/// Default implementation of BroadcasterService
pub struct DefaultBroadcasterService {
    broadcasters: DashMap<BroadcasterId, BroadcasterProfile>,
    username_index: RwLock<HashMap<String, BroadcasterId>>,
    streams: DashMap<StreamId, LiveStream>,
    broadcaster_streams: DashMap<BroadcasterId, Vec<StreamId>>,
    ingestion_servers: Vec<IngestionServer>,
}

#[derive(Debug, Clone)]
struct IngestionServer {
    server: String,
    rtmp_port: u16,
    region: String,
    load: f32,
}

impl DefaultBroadcasterService {
    pub fn new() -> Self {
        Self {
            broadcasters: DashMap::new(),
            username_index: RwLock::new(HashMap::new()),
            streams: DashMap::new(),
            broadcaster_streams: DashMap::new(),
            ingestion_servers: vec![
                IngestionServer {
                    server: "ingest.vantis.media".to_string(),
                    rtmp_port: 1935,
                    region: "global".to_string(),
                    load: 0.0,
                },
                IngestionServer {
                    server: "ingest-eu.vantis.media".to_string(),
                    rtmp_port: 1935,
                    region: "europe".to_string(),
                    load: 0.0,
                },
                IngestionServer {
                    server: "ingest-as.vantis.media".to_string(),
                    rtmp_port: 1935,
                    region: "asia".to_string(),
                    load: 0.0,
                },
            ],
        }
    }

    fn generate_stream_key() -> String {
        // Generate a random stream key
        format!("live_{}", Uuid::new_v4().simple())
    }

    fn get_best_ingestion_server(&self) -> &IngestionServer {
        // Return the server with the lowest load
        self.ingestion_servers
            .iter()
            .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap())
            .unwrap_or(&self.ingestion_servers[0])
    }
}

impl Default for DefaultBroadcasterService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BroadcasterService for DefaultBroadcasterService {
    async fn create_broadcaster(&self, username: String, display_name: String) -> LiveStreamingResult<BroadcasterProfile> {
        // Check if username exists
        {
            let index = self.username_index.read();
            if index.contains_key(&username) {
                return Err(LiveStreamingError::InternalError("Username already taken".to_string()));
            }
        }

        let stream_key = Self::generate_stream_key();
        let server = self.get_best_ingestion_server();

        let profile = BroadcasterProfile {
            id: BroadcasterId::new(),
            username: username.clone(),
            display_name,
            avatar_url: None,
            bio: None,
            follower_count: 0,
            subscriber_count: 0,
            total_views: 0,
            is_partner: false,
            is_verified: false,
            created_at: Utc::now(),
            stream_key: Some(stream_key),
            ingestion_endpoint: Some(format!("rtmp://{}:{}", server.server, server.rtmp_port)),
            settings: BroadcasterSettings::default(),
        };

        let broadcaster_id = profile.id.clone();

        // Add to username index
        {
            let mut index = self.username_index.write();
            index.insert(username, broadcaster_id.clone());
        }

        self.broadcasters.insert(broadcaster_id, profile.clone());

        Ok(profile)
    }

    async fn get_broadcaster(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<BroadcasterProfile> {
        self.broadcasters
            .get(broadcaster_id)
            .map(|r| r.clone())
            .ok_or_else(|| LiveStreamingError::BroadcasterNotFound(broadcaster_id.0.to_string()))
    }

    async fn get_broadcaster_by_username(&self, username: &str) -> LiveStreamingResult<BroadcasterProfile> {
        let broadcaster_id = {
            let index = self.username_index.read();
            index.get(username).cloned()
        };

        match broadcaster_id {
            Some(id) => self.get_broadcaster(&id).await,
            None => Err(LiveStreamingError::BroadcasterNotFound(username.to_string())),
        }
    }

    async fn update_profile(&self, broadcaster_id: &BroadcasterId, updates: ProfileUpdate) -> LiveStreamingResult<BroadcasterProfile> {
        let mut profile = self.get_broadcaster(broadcaster_id).await?;

        if let Some(display_name) = updates.display_name {
            profile.display_name = display_name;
        }
        if let Some(avatar_url) = updates.avatar_url {
            profile.avatar_url = Some(avatar_url);
        }
        if let Some(bio) = updates.bio {
            profile.bio = Some(bio);
        }

        self.broadcasters.insert(broadcaster_id.clone(), profile.clone());
        Ok(profile)
    }

    async fn update_settings(&self, broadcaster_id: &BroadcasterId, settings: BroadcasterSettings) -> LiveStreamingResult<()> {
        let mut profile = self.get_broadcaster(broadcaster_id).await?;
        profile.settings = settings;
        self.broadcasters.insert(broadcaster_id.clone(), profile);
        Ok(())
    }

    async fn regenerate_stream_key(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<String> {
        let mut profile = self.get_broadcaster(broadcaster_id).await?;
        let new_key = Self::generate_stream_key();
        profile.stream_key = Some(new_key.clone());
        self.broadcasters.insert(broadcaster_id.clone(), profile);
        Ok(new_key)
    }

    async fn get_ingestion_endpoint(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<IngestionEndpoint> {
        let profile = self.get_broadcaster(broadcaster_id).await?;
        let stream_key = profile.stream_key.clone().ok_or(LiveStreamingError::InvalidStreamKey)?;
        let server = self.get_best_ingestion_server();

        Ok(IngestionEndpoint {
            url: format!("rtmp://{}:{}", server.server, server.rtmp_port),
            stream_key,
            server: server.server.clone(),
            rtmp_port: server.rtmp_port,
            protocol: IngestionProtocol::RTMPS,
            backup_servers: self.ingestion_servers.iter().skip(1).map(|s| s.server.clone()).collect(),
        })
    }

    async fn create_stream(&self, broadcaster_id: &BroadcasterId, title: String, category: StreamCategory) -> LiveStreamingResult<LiveStream> {
        let _profile = self.get_broadcaster(broadcaster_id).await?;

        let stream = LiveStream::new(broadcaster_id.clone(), title, category);

        // Add to streams
        self.streams.insert(stream.id.clone(), stream.clone());

        // Add to broadcaster streams index
        let mut streams = self.broadcaster_streams.entry(broadcaster_id.clone()).or_insert_with(Vec::new);
        streams.push(stream.id.clone());

        Ok(stream)
    }

    async fn start_stream(&self, stream_id: &StreamId, stream_key: &str) -> LiveStreamingResult<LiveStream> {
        let mut stream = self.get_stream(stream_id).await?;

        // Get broadcaster and verify stream key
        let profile = self.get_broadcaster(&stream.broadcaster_id).await?;
        let expected_key = profile.stream_key.as_ref().ok_or(LiveStreamingError::InvalidStreamKey)?;

        if stream_key != expected_key {
            return Err(LiveStreamingError::InvalidStreamKey);
        }

        // Check if already live
        if stream.status == StreamStatus::Live {
            return Err(LiveStreamingError::StreamAlreadyLive);
        }

        stream.status = StreamStatus::Live;
        stream.started_at = Some(Utc::now());

        self.streams.insert(stream_id.clone(), stream.clone());

        Ok(stream)
    }

    async fn end_stream(&self, stream_id: &StreamId) -> LiveStreamingResult<LiveStream> {
        let mut stream = self.get_stream(stream_id).await?;

        if stream.status != StreamStatus::Live {
            return Err(LiveStreamingError::StreamNotLive);
        }

        stream.status = StreamStatus::Ended;
        stream.ended_at = Some(Utc::now());

        // Calculate duration
        if let Some(started) = stream.started_at {
            stream.duration_seconds = (Utc::now() - started).num_seconds() as u64;
        }

        self.streams.insert(stream_id.clone(), stream.clone());

        Ok(stream)
    }

    async fn get_stream(&self, stream_id: &StreamId) -> LiveStreamingResult<LiveStream> {
        self.streams
            .get(stream_id)
            .map(|r| r.clone())
            .ok_or_else(|| LiveStreamingError::StreamNotFound(stream_id.0.to_string()))
    }

    async fn get_active_stream(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<Option<LiveStream>> {
        let stream_ids = self.broadcaster_streams
            .get(broadcaster_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        for id in stream_ids {
            if let Some(stream) = self.streams.get(&id) {
                if stream.status == StreamStatus::Live {
                    return Ok(Some(stream.clone()));
                }
            }
        }

        Ok(None)
    }

    async fn update_stream_info(&self, stream_id: &StreamId, updates: StreamInfoUpdate) -> LiveStreamingResult<LiveStream> {
        let mut stream = self.get_stream(stream_id).await?;

        if let Some(title) = updates.title {
            stream.title = title;
        }
        if let Some(description) = updates.description {
            stream.description = Some(description);
        }
        if let Some(category) = updates.category {
            stream.category = category;
        }
        if let Some(tags) = updates.tags {
            stream.tags = tags;
        }
        if let Some(language) = updates.language {
            stream.language = language;
        }
        if let Some(is_mature) = updates.is_mature {
            stream.is_mature = is_mature;
        }

        self.streams.insert(stream_id.clone(), stream.clone());
        Ok(stream)
    }

    async fn get_past_streams(&self, broadcaster_id: &BroadcasterId, limit: usize, offset: usize) -> LiveStreamingResult<Vec<LiveStream>> {
        let stream_ids = self.broadcaster_streams
            .get(broadcaster_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut past_streams: Vec<LiveStream> = stream_ids
            .into_iter()
            .filter_map(|id| self.streams.get(&id).map(|r| r.clone()))
            .filter(|s| s.status == StreamStatus::Ended || s.status == StreamStatus::Archived)
            .collect();

        // Sort by ended_at descending
        past_streams.sort_by(|a, b| {
            let a_time = a.ended_at.unwrap_or(a.created_at);
            let b_time = b.ended_at.unwrap_or(b.created_at);
            b_time.cmp(&a_time)
        });

        Ok(past_streams.into_iter().skip(offset).take(limit).collect())
    }

    async fn get_broadcaster_stats(&self, broadcaster_id: &BroadcasterId) -> LiveStreamingResult<BroadcasterStats> {
        let stream_ids = self.broadcaster_streams
            .get(broadcaster_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut stats = BroadcasterStats::default();

        for id in stream_ids {
            if let Some(stream) = self.streams.get(&id) {
                stats.total_streams += 1;
                stats.total_stream_time_hours += stream.duration_seconds as f64 / 3600.0;
                stats.total_views += stream.total_views;
                stats.peak_viewers = stats.peak_viewers.max(stream.peak_viewer_count);
            }
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_broadcaster() {
        let service = DefaultBroadcasterService::new();
        let profile = service.create_broadcaster("testuser".to_string(), "Test User".to_string()).await.unwrap();

        assert_eq!(profile.username, "testuser");
        assert!(profile.stream_key.is_some());
    }

    #[tokio::test]
    async fn test_create_and_start_stream() {
        let service = DefaultBroadcasterService::new();
        let profile = service.create_broadcaster("streamer".to_string(), "Streamer".to_string()).await.unwrap();

        let stream = service.create_stream(
            &profile.id,
            "My First Stream".to_string(),
            StreamCategory::new("gaming", "Gaming"),
        ).await.unwrap();

        assert_eq!(stream.status, StreamStatus::Created);

        let stream_key = profile.stream_key.unwrap();
        let started = service.start_stream(&stream.id, &stream_key).await.unwrap();

        assert_eq!(started.status, StreamStatus::Live);
    }

    #[tokio::test]
    async fn test_end_stream() {
        let service = DefaultBroadcasterService::new();
        let profile = service.create_broadcaster("streamer".to_string(), "Streamer".to_string()).await.unwrap();

        let stream = service.create_stream(
            &profile.id,
            "Test Stream".to_string(),
            StreamCategory::new("gaming", "Gaming"),
        ).await.unwrap();

        let stream_key = profile.stream_key.unwrap();
        service.start_stream(&stream.id, &stream_key).await.unwrap();

        let ended = service.end_stream(&stream.id).await.unwrap();

        assert_eq!(ended.status, StreamStatus::Ended);
        assert!(ended.ended_at.is_some());
    }
}