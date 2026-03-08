//! User presence and online status service

use async_trait::async_trait;
use chrono::{Duration, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;
use crate::user_service::UserService;

/// Presence update interval in seconds
const PRESENCE_TIMEOUT_SECS: i64 = 300; // 5 minutes
const AWAY_TIMEOUT_SECS: i64 = 300; // 5 minutes of inactivity

/// Trait for user presence management
#[async_trait]
pub trait PresenceService: Send + Sync {
    /// Update user presence status
    async fn update_presence(&self, user_id: &UserId, status: PresenceStatus) -> SocialResult<UserPresence>;

    /// Get user presence
    async fn get_presence(&self, user_id: &UserId) -> SocialResult<UserPresence>;

    /// Get presence for multiple users
    async fn get_presences(&self, user_ids: &[UserId]) -> SocialResult<Vec<UserPresence>>;

    /// Set custom status message
    async fn set_custom_status(&self, user_id: &UserId, status: Option<String>) -> SocialResult<()>;

    /// Update current activity
    async fn set_current_activity(&self, user_id: &UserId, activity: Option<CurrentActivity>) -> SocialResult<()>;

    /// Subscribe to presence updates
    async fn subscribe(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<PresenceEvent>>;

    /// Subscribe to friend presence updates
    async fn subscribe_to_friends(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<PresenceEvent>>;

    /// Get online friends
    async fn get_online_friends(&self, user_id: &UserId) -> SocialResult<Vec<UserId>>;

    /// Get users by status
    async fn get_users_by_status(&self, status: PresenceStatus, limit: usize) -> SocialResult<Vec<UserId>>;

    /// Check if user is online
    async fn is_online(&self, user_id: &UserId) -> SocialResult<bool>;

    /// Heartbeat to maintain online status
    async fn heartbeat(&self, user_id: &UserId) -> SocialResult<()>;

    /// Set user as offline
    async fn set_offline(&self, user_id: &UserId) -> SocialResult<()>;

    /// Clean up stale presences (called periodically)
    async fn cleanup_stale(&self) -> SocialResult<u64>;
}

/// Presence event for real-time updates
#[derive(Debug, Clone)]
pub enum PresenceEvent {
    StatusChanged {
        user_id: UserId,
        old_status: PresenceStatus,
        new_status: PresenceStatus,
    },
    ActivityChanged {
        user_id: UserId,
        activity: Option<CurrentActivity>,
    },
    CustomStatusChanged {
        user_id: UserId,
        status: Option<String>,
    },
}

/// Default implementation of PresenceService
pub struct DefaultPresenceService {
    presences: DashMap<UserId, UserPresence>,
    event_sender: broadcast::Sender<PresenceEvent>,
    user_service: Arc<dyn UserService>,
    last_heartbeat: RwLock<HashMap<UserId, chrono::DateTime<Utc>>>,
}

impl DefaultPresenceService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            presences: DashMap::new(),
            event_sender: tx,
            user_service,
            last_heartbeat: RwLock::new(HashMap::new()),
        }
    }

    fn broadcast_event(&self, event: PresenceEvent) {
        let _ = self.event_sender.send(event);
    }

    fn get_or_create_presence(&self, user_id: &UserId) -> UserPresence {
        self.presences
            .entry(user_id.clone())
            .or_insert_with(|| UserPresence {
                user_id: user_id.clone(),
                status: PresenceStatus::Offline,
                last_seen: Utc::now(),
                current_activity: None,
                custom_status: None,
            })
            .clone()
    }

    fn determine_auto_status(&self, user_id: &UserId) -> PresenceStatus {
        let heartbeats = self.last_heartbeat.read();
        if let Some(last) = heartbeats.get(user_id) {
            let elapsed = (Utc::now() - *last).num_seconds();
            if elapsed > PRESENCE_TIMEOUT_SECS {
                return PresenceStatus::Offline;
            } else if elapsed > AWAY_TIMEOUT_SECS {
                return PresenceStatus::Away;
            }
        }
        PresenceStatus::Online
    }
}

#[async_trait]
impl PresenceService for DefaultPresenceService {
    async fn update_presence(&self, user_id: &UserId, status: PresenceStatus) -> SocialResult<UserPresence> {
        // Verify user exists
        self.user_service.get_profile(user_id).await?;

        let old_presence = self.get_or_create_presence(user_id);
        let old_status = old_presence.status;

        let new_presence = UserPresence {
            user_id: user_id.clone(),
            status,
            last_seen: Utc::now(),
            current_activity: old_presence.current_activity,
            custom_status: old_presence.custom_status,
        };

        self.presences.insert(user_id.clone(), new_presence.clone());

        // Update heartbeat
        {
            let mut heartbeats = self.last_heartbeat.write();
            heartbeats.insert(user_id.clone(), Utc::now());
        }

        // Broadcast event if status changed
        if old_status != status {
            self.broadcast_event(PresenceEvent::StatusChanged {
                user_id: user_id.clone(),
                old_status,
                new_status: status,
            });
        }

        Ok(new_presence)
    }

    async fn get_presence(&self, user_id: &UserId) -> SocialResult<UserPresence> {
        let mut presence = self.get_or_create_presence(user_id);

        // Auto-update status based on last heartbeat
        if presence.status == PresenceStatus::Online || presence.status == PresenceStatus::Away {
            let auto_status = self.determine_auto_status(user_id);
            if auto_status != presence.status {
                presence.status = auto_status;
                self.presences.insert(user_id.clone(), presence.clone());
            }
        }

        Ok(presence)
    }

    async fn get_presences(&self, user_ids: &[UserId]) -> SocialResult<Vec<UserPresence>> {
        let mut presences = Vec::new();
        for user_id in user_ids {
            presences.push(self.get_presence(user_id).await?);
        }
        Ok(presences)
    }

    async fn set_custom_status(&self, user_id: &UserId, status: Option<String>) -> SocialResult<()> {
        let mut presence = self.get_or_create_presence(user_id);
        presence.custom_status = status.clone();
        self.presences.insert(user_id.clone(), presence);

        self.broadcast_event(PresenceEvent::CustomStatusChanged {
            user_id: user_id.clone(),
            status,
        });

        Ok(())
    }

    async fn set_current_activity(&self, user_id: &UserId, activity: Option<CurrentActivity>) -> SocialResult<()> {
        let mut presence = self.get_or_create_presence(user_id);
        presence.current_activity = activity.clone();
        presence.last_seen = Utc::now();
        self.presences.insert(user_id.clone(), presence);

        // Update heartbeat
        {
            let mut heartbeats = self.last_heartbeat.write();
            heartbeats.insert(user_id.clone(), Utc::now());
        }

        self.broadcast_event(PresenceEvent::ActivityChanged {
            user_id: user_id.clone(),
            activity,
        });

        Ok(())
    }

    async fn subscribe(&self, _user_id: &UserId) -> SocialResult<broadcast::Receiver<PresenceEvent>> {
        Ok(self.event_sender.subscribe())
    }

    async fn subscribe_to_friends(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<PresenceEvent>> {
        // In a real implementation, we'd filter events to only those from friends
        // For now, subscribe to all events
        self.subscribe(user_id).await
    }

    async fn get_online_friends(&self, _user_id: &UserId) -> SocialResult<Vec<UserId>> {
        // This would need FriendService to get friends list
        // Return all online users for now
        let online_users: Vec<UserId> = self.presences
            .iter()
            .filter(|e| e.value().status == PresenceStatus::Online)
            .map(|e| e.key().clone())
            .collect();

        Ok(online_users)
    }

    async fn get_users_by_status(&self, status: PresenceStatus, limit: usize) -> SocialResult<Vec<UserId>> {
        let users: Vec<UserId> = self.presences
            .iter()
            .filter(|e| e.value().status == status)
            .take(limit)
            .map(|e| e.key().clone())
            .collect();

        Ok(users)
    }

    async fn is_online(&self, user_id: &UserId) -> SocialResult<bool> {
        let presence = self.get_presence(user_id).await?;
        Ok(presence.status == PresenceStatus::Online)
    }

    async fn heartbeat(&self, user_id: &UserId) -> SocialResult<()> {
        // Verify user exists
        self.user_service.get_profile(user_id).await?;

        // Update heartbeat time
        {
            let mut heartbeats = self.last_heartbeat.write();
            heartbeats.insert(user_id.clone(), Utc::now());
        }

        // Update presence if was offline
        let presence = self.get_or_create_presence(user_id);
        if presence.status == PresenceStatus::Offline {
            self.update_presence(user_id, PresenceStatus::Online).await?;
        }

        Ok(())
    }

    async fn set_offline(&self, user_id: &UserId) -> SocialResult<()> {
        let old_presence = self.get_or_create_presence(user_id);
        let old_status = old_presence.status;

        let new_presence = UserPresence {
            user_id: user_id.clone(),
            status: PresenceStatus::Offline,
            last_seen: Utc::now(),
            current_activity: None,
            custom_status: old_presence.custom_status,
        };

        self.presences.insert(user_id.clone(), new_presence);

        if old_status != PresenceStatus::Offline {
            self.broadcast_event(PresenceEvent::StatusChanged {
                user_id: user_id.clone(),
                old_status,
                new_status: PresenceStatus::Offline,
            });
        }

        Ok(())
    }

    async fn cleanup_stale(&self) -> SocialResult<u64> {
        let now = Utc::now();
        let mut cleaned = 0u64;

        // Find stale presences
        let stale_users: Vec<UserId> = {
            let heartbeats = self.last_heartbeat.read();
            heartbeats
                .iter()
                .filter(|(_, last)| (now - **last).num_seconds() > PRESENCE_TIMEOUT_SECS)
                .map(|(user_id, _)| user_id.clone())
                .collect()
        };

        // Set them offline
        for user_id in stale_users {
            let mut presence = self.get_or_create_presence(&user_id);
            if presence.status != PresenceStatus::Offline {
                presence.status = PresenceStatus::Offline;
                presence.current_activity = None;
                self.presences.insert(user_id.clone(), presence);
                cleaned += 1;
            }
        }

        Ok(cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::DefaultUserService;

    #[tokio::test]
    async fn test_update_presence() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let presence_service = DefaultPresenceService::new(user_service);

        let presence = presence_service.update_presence(&user.id, PresenceStatus::Online).await.unwrap();
        assert_eq!(presence.status, PresenceStatus::Online);
    }

    #[tokio::test]
    async fn test_set_activity() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let presence_service = DefaultPresenceService::new(user_service);

        presence_service.update_presence(&user.id, PresenceStatus::Online).await.unwrap();

        let activity = CurrentActivity {
            activity_type: ActivityType::NowPlaying,
            media_title: Some("Test Song".to_string()),
            media_id: Some("song123".to_string()),
            started_at: Utc::now(),
        };

        presence_service.set_current_activity(&user.id, Some(activity)).await.unwrap();

        let presence = presence_service.get_presence(&user.id).await.unwrap();
        assert!(presence.current_activity.is_some());
    }

    #[tokio::test]
    async fn test_custom_status() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let presence_service = DefaultPresenceService::new(user_service);

        presence_service.set_custom_status(&user.id, Some("Busy coding".to_string())).await.unwrap();

        let presence = presence_service.get_presence(&user.id).await.unwrap();
        assert_eq!(presence.custom_status, Some("Busy coding".to_string()));
    }

    #[tokio::test]
    async fn test_is_online() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let presence_service = DefaultPresenceService::new(user_service);

        let online = presence_service.is_online(&user.id).await.unwrap();
        assert!(!online);

        presence_service.update_presence(&user.id, PresenceStatus::Online).await.unwrap();

        let online = presence_service.is_online(&user.id).await.unwrap();
        assert!(online);
    }
}