//! Live chat service for streams

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::{LiveStreamingError, LiveStreamingResult};
use crate::types::*;
use crate::broadcaster::BroadcasterService;
use crate::viewer::ViewerService;

/// Trait for chat operations
#[async_trait]
pub trait ChatService: Send + Sync {
    /// Send a chat message
    async fn send_message(&self, message: SendMessageRequest) -> LiveStreamingResult<ChatMessage>;

    /// Get chat messages for a stream
    async fn get_messages(&self, stream_id: &StreamId, limit: usize, before: Option<ChatMessageId>) -> LiveStreamingResult<Vec<ChatMessage>>;

    /// Delete a message
    async fn delete_message(&self, stream_id: &StreamId, message_id: &ChatMessageId, deleted_by: &ViewerId) -> LiveStreamingResult<()>;

    /// Clear chat for a stream
    async fn clear_chat(&self, stream_id: &StreamId, cleared_by: &ViewerId) -> LiveStreamingResult<()>;

    /// Ban a user from chat
    async fn ban_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId, reason: Option<String>) -> LiveStreamingResult<ModerationAction>;

    /// Unban a user
    async fn unban_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId) -> LiveStreamingResult<ModerationAction>;

    /// Timeout a user
    async fn timeout_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId, duration_seconds: u32, reason: Option<String>) -> LiveStreamingResult<ModerationAction>;

    /// Enable slow mode
    async fn enable_slow_mode(&self, stream_id: &StreamId, delay_seconds: u32) -> LiveStreamingResult<()>;

    /// Disable slow mode
    async fn disable_slow_mode(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Enable followers only mode
    async fn enable_followers_only(&self, stream_id: &StreamId, min_follow_time_minutes: u32) -> LiveStreamingResult<()>;

    /// Disable followers only mode
    async fn disable_followers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Enable subscribers only mode
    async fn enable_subscribers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Disable subscribers only mode
    async fn disable_subscribers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Enable emote only mode
    async fn enable_emote_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Disable emote only mode
    async fn disable_emote_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Subscribe to chat messages
    async fn subscribe(&self, stream_id: &StreamId) -> LiveStreamingResult<broadcast::Receiver<ChatEvent>>;

    /// Pin a message
    async fn pin_message(&self, stream_id: &StreamId, message_id: &ChatMessageId) -> LiveStreamingResult<()>;

    /// Unpin a message
    async fn unpin_message(&self, stream_id: &StreamId) -> LiveStreamingResult<()>;

    /// Get pinned message
    async fn get_pinned_message(&self, stream_id: &StreamId) -> LiveStreamingResult<Option<ChatMessage>>;
}

/// Request to send a chat message
#[derive(Debug, Clone)]
pub struct SendMessageRequest {
    pub stream_id: StreamId,
    pub sender_id: ViewerId,
    pub sender_name: String,
    pub content: String,
    pub color: Option<String>,
    pub badges: Vec<ChatBadge>,
}

/// Chat event for real-time updates
#[derive(Debug, Clone)]
pub enum ChatEvent {
    Message(ChatMessage),
    MessageDeleted { message_id: ChatMessageId, deleted_by: String },
    ChatCleared,
    UserBanned { user_id: ViewerId, username: String, reason: Option<String> },
    UserUnbanned { user_id: ViewerId, username: String },
    UserTimeout { user_id: ViewerId, username: String, duration: u32, reason: Option<String> },
    SlowModeChanged { enabled: bool, delay: u32 },
    FollowersOnlyChanged { enabled: bool, min_follow_time: u32 },
    SubscribersOnlyChanged { enabled: bool },
    EmoteOnlyChanged { enabled: bool },
    MessagePinned(ChatMessage),
    MessageUnpinned,
}

/// Chat mode state
#[derive(Debug, Clone, Default)]
struct ChatMode {
    slow_mode: Option<u32>,
    followers_only: Option<u32>,
    subscribers_only: bool,
    emote_only: bool,
}

/// Default implementation of ChatService
pub struct DefaultChatService {
    messages: DashMap<ChatMessageId, ChatMessage>,
    stream_messages: DashMap<StreamId, Vec<ChatMessageId>>,
    chat_modes: DashMap<StreamId, ChatMode>,
    banned_users: DashMap<(StreamId, ViewerId), ModerationAction>,
    pinned_messages: DashMap<StreamId, ChatMessageId>,
    event_senders: RwLock<HashMap<StreamId, broadcast::Sender<ChatEvent>>>,
    last_message_time: DashMap<ViewerId, chrono::DateTime<Utc>>,
    broadcaster_service: Arc<dyn BroadcasterService>,
    viewer_service: Arc<dyn ViewerService>,
}

impl DefaultChatService {
    pub fn new(broadcaster_service: Arc<dyn BroadcasterService>, viewer_service: Arc<dyn ViewerService>) -> Self {
        Self {
            messages: DashMap::new(),
            stream_messages: DashMap::new(),
            chat_modes: DashMap::new(),
            banned_users: DashMap::new(),
            pinned_messages: DashMap::new(),
            event_senders: RwLock::new(HashMap::new()),
            last_message_time: DashMap::new(),
            broadcaster_service,
            viewer_service,
        }
    }

    fn get_or_create_sender(&self, stream_id: &StreamId) -> broadcast::Sender<ChatEvent> {
        let mut senders = self.event_senders.write();
        senders.entry(stream_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1000);
            tx
        }).clone()
    }

    fn broadcast(&self, stream_id: &StreamId, event: ChatEvent) {
        let senders = self.event_senders.read();
        if let Some(sender) = senders.get(stream_id) {
            let _ = sender.send(event);
        }
    }

    fn check_slow_mode(&self, stream_id: &StreamId, sender_id: &ViewerId) -> LiveStreamingResult<()> {
        if let Some(mode) = self.chat_modes.get(stream_id) {
            if let Some(delay) = mode.slow_mode {
                if let Some(last_time) = self.last_message_time.get(sender_id) {
                    let elapsed = (Utc::now() - *last_time).num_seconds() as u32;
                    if elapsed < delay {
                        return Err(LiveStreamingError::SlowModeActive(delay - elapsed));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_banned(&self, stream_id: &StreamId, viewer_id: &ViewerId) -> LiveStreamingResult<()> {
        if let Some(ban) = self.banned_users.get(&(stream_id.clone(), viewer_id.clone())) {
            if ban.action_type == ModerationActionType::Ban {
                return Err(LiveStreamingError::UserBanned);
            }
            if let Some(duration) = ban.duration_seconds {
                let elapsed = (Utc::now() - ban.created_at).num_seconds() as u32;
                if elapsed < duration {
                    return Err(LiveStreamingError::UserTimeout(duration - elapsed));
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ChatService for DefaultChatService {
    async fn send_message(&self, message: SendMessageRequest) -> LiveStreamingResult<ChatMessage> {
        // Get stream and verify it's live
        let stream = self.broadcaster_service.get_stream(&message.stream_id).await?;
        if stream.status != StreamStatus::Live {
            return Err(LiveStreamingError::StreamNotLive);
        }

        // Check if chat is enabled
        if !stream.config.chat_enabled {
            return Err(LiveStreamingError::ChatDisabled);
        }

        // Check if user is banned
        self.check_banned(&message.stream_id, &message.sender_id)?;

        // Check slow mode
        self.check_slow_mode(&message.stream_id, &message.sender_id)?;

        // Validate message length
        if message.content.len() > 500 {
            return Err(LiveStreamingError::MessageTooLong { max: 500 });
        }

        // Check followers only mode
        if let Some(mode) = self.chat_modes.get(&message.stream_id) {
            if mode.followers_only.is_some() {
                let is_follower = self.viewer_service.is_following(&message.sender_id, &stream.broadcaster_id).await?;
                if !is_follower {
                    return Err(LiveStreamingError::FollowersOnlyMode);
                }
            }
            if mode.subscribers_only {
                // Would check subscription status
                return Err(LiveStreamingError::SubscribersOnlyMode);
            }
            if mode.emote_only {
                // Would check if message is only emotes
                return Err(LiveStreamingError::EmoteOnlyMode);
            }
        }

        let mut chat_message = ChatMessage::new(
            message.stream_id.clone(),
            message.sender_id.clone(),
            message.sender_name,
            message.content,
        );
        chat_message.color = message.color;
        chat_message.badges = message.badges;

        let message_id = chat_message.id.clone();

        // Store message
        self.messages.insert(message_id.clone(), chat_message.clone());

        // Add to stream messages
        let mut stream_msgs = self.stream_messages.entry(message.stream_id.clone()).or_insert_with(Vec::new);
        stream_msgs.push(message_id);

        // Update last message time
        self.last_message_time.insert(message.sender_id, Utc::now());

        // Broadcast message
        self.broadcast(&message.stream_id, ChatEvent::Message(chat_message.clone()));

        Ok(chat_message)
    }

    async fn get_messages(&self, stream_id: &StreamId, limit: usize, before: Option<ChatMessageId>) -> LiveStreamingResult<Vec<ChatMessage>> {
        let message_ids = self.stream_messages
            .get(stream_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut messages: Vec<ChatMessage> = message_ids
            .into_iter()
            .filter_map(|id| self.messages.get(&id).map(|r| r.clone()))
            .filter(|m| !m.is_deleted)
            .collect();

        // Sort by created_at descending
        messages.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Filter by before if provided
        if let Some(before_id) = before {
            if let Some(pos) = messages.iter().position(|m| m.id == before_id) {
                messages = messages.into_iter().skip(pos + 1).collect();
            }
        }

        Ok(messages.into_iter().take(limit).collect())
    }

    async fn delete_message(&self, stream_id: &StreamId, message_id: &ChatMessageId, deleted_by: &ViewerId) -> LiveStreamingResult<()> {
        let mut message = self.messages
            .get(message_id)
            .map(|r| r.clone())
            .ok_or_else(|| LiveStreamingError::InternalError("Message not found".to_string()))?;

        // Get viewer for deleted_by name
        let deleter = self.viewer_service.get_viewer(deleted_by).await?;

        message.is_deleted = true;
        message.deleted_by = Some(deleter.display_name.clone());

        self.messages.insert(message_id.clone(), message);

        // Broadcast deletion
        self.broadcast(stream_id, ChatEvent::MessageDeleted {
            message_id: message_id.clone(),
            deleted_by: deleter.display_name,
        });

        Ok(())
    }

    async fn clear_chat(&self, stream_id: &StreamId, _cleared_by: &ViewerId) -> LiveStreamingResult<()> {
        // Mark all messages as deleted
        if let Some(message_ids) = self.stream_messages.get(stream_id) {
            for id in message_ids.iter() {
                if let Some(mut msg) = self.messages.get_mut(id) {
                    msg.is_deleted = true;
                }
            }
        }

        self.broadcast(stream_id, ChatEvent::ChatCleared);
        Ok(())
    }

    async fn ban_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId, reason: Option<String>) -> LiveStreamingResult<ModerationAction> {
        let target_viewer = self.viewer_service.get_viewer(target).await?;
        let moderator_viewer = self.viewer_service.get_viewer(moderator).await?;

        let action = ModerationAction {
            action_type: ModerationActionType::Ban,
            target_user_id: target.clone(),
            target_username: target_viewer.display_name.clone(),
            moderator_id: moderator.clone(),
            moderator_name: moderator_viewer.display_name.clone(),
            reason,
            duration_seconds: None,
            created_at: Utc::now(),
        };

        self.banned_users.insert((stream_id.clone(), target.clone()), action.clone());

        self.broadcast(stream_id, ChatEvent::UserBanned {
            user_id: target.clone(),
            username: target_viewer.display_name,
            reason: action.reason.clone(),
        });

        Ok(action)
    }

    async fn unban_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId) -> LiveStreamingResult<ModerationAction> {
        let target_viewer = self.viewer_service.get_viewer(target).await?;
        let moderator_viewer = self.viewer_service.get_viewer(moderator).await?;

        self.banned_users.remove(&(stream_id.clone(), target.clone()));

        let action = ModerationAction {
            action_type: ModerationActionType::Unban,
            target_user_id: target.clone(),
            target_username: target_viewer.display_name.clone(),
            moderator_id: moderator.clone(),
            moderator_name: moderator_viewer.display_name.clone(),
            reason: None,
            duration_seconds: None,
            created_at: Utc::now(),
        };

        self.broadcast(stream_id, ChatEvent::UserUnbanned {
            user_id: target.clone(),
            username: target_viewer.display_name,
        });

        Ok(action)
    }

    async fn timeout_user(&self, stream_id: &StreamId, target: &ViewerId, moderator: &ViewerId, duration_seconds: u32, reason: Option<String>) -> LiveStreamingResult<ModerationAction> {
        let target_viewer = self.viewer_service.get_viewer(target).await?;
        let moderator_viewer = self.viewer_service.get_viewer(moderator).await?;

        let action = ModerationAction {
            action_type: ModerationActionType::Timeout,
            target_user_id: target.clone(),
            target_username: target_viewer.display_name.clone(),
            moderator_id: moderator.clone(),
            moderator_name: moderator_viewer.display_name.clone(),
            reason,
            duration_seconds: Some(duration_seconds),
            created_at: Utc::now(),
        };

        self.banned_users.insert((stream_id.clone(), target.clone()), action.clone());

        self.broadcast(stream_id, ChatEvent::UserTimeout {
            user_id: target.clone(),
            username: target_viewer.display_name,
            duration: duration_seconds,
            reason: action.reason.clone(),
        });

        Ok(action)
    }

    async fn enable_slow_mode(&self, stream_id: &StreamId, delay_seconds: u32) -> LiveStreamingResult<()> {
        let mut mode = self.chat_modes.entry(stream_id.clone()).or_insert_with(ChatMode::default);
        mode.slow_mode = Some(delay_seconds);
        self.broadcast(stream_id, ChatEvent::SlowModeChanged { enabled: true, delay: delay_seconds });
        Ok(())
    }

    async fn disable_slow_mode(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        if let Some(mut mode) = self.chat_modes.get_mut(stream_id) {
            mode.slow_mode = None;
        }
        self.broadcast(stream_id, ChatEvent::SlowModeChanged { enabled: false, delay: 0 });
        Ok(())
    }

    async fn enable_followers_only(&self, stream_id: &StreamId, min_follow_time_minutes: u32) -> LiveStreamingResult<()> {
        let mut mode = self.chat_modes.entry(stream_id.clone()).or_insert_with(ChatMode::default);
        mode.followers_only = Some(min_follow_time_minutes);
        self.broadcast(stream_id, ChatEvent::FollowersOnlyChanged { enabled: true, min_follow_time: min_follow_time_minutes });
        Ok(())
    }

    async fn disable_followers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        if let Some(mut mode) = self.chat_modes.get_mut(stream_id) {
            mode.followers_only = None;
        }
        self.broadcast(stream_id, ChatEvent::FollowersOnlyChanged { enabled: false, min_follow_time: 0 });
        Ok(())
    }

    async fn enable_subscribers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        let mut mode = self.chat_modes.entry(stream_id.clone()).or_insert_with(ChatMode::default);
        mode.subscribers_only = true;
        self.broadcast(stream_id, ChatEvent::SubscribersOnlyChanged { enabled: true });
        Ok(())
    }

    async fn disable_subscribers_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        if let Some(mut mode) = self.chat_modes.get_mut(stream_id) {
            mode.subscribers_only = false;
        }
        self.broadcast(stream_id, ChatEvent::SubscribersOnlyChanged { enabled: false });
        Ok(())
    }

    async fn enable_emote_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        let mut mode = self.chat_modes.entry(stream_id.clone()).or_insert_with(ChatMode::default);
        mode.emote_only = true;
        self.broadcast(stream_id, ChatEvent::EmoteOnlyChanged { enabled: true });
        Ok(())
    }

    async fn disable_emote_only(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        if let Some(mut mode) = self.chat_modes.get_mut(stream_id) {
            mode.emote_only = false;
        }
        self.broadcast(stream_id, ChatEvent::EmoteOnlyChanged { enabled: false });
        Ok(())
    }

    async fn subscribe(&self, stream_id: &StreamId) -> LiveStreamingResult<broadcast::Receiver<ChatEvent>> {
        let sender = self.get_or_create_sender(stream_id);
        Ok(sender.subscribe())
    }

    async fn pin_message(&self, stream_id: &StreamId, message_id: &ChatMessageId) -> LiveStreamingResult<()> {
        let message = self.messages
            .get(message_id)
            .map(|r| r.clone())
            .ok_or_else(|| LiveStreamingError::InternalError("Message not found".to_string()))?;

        self.pinned_messages.insert(stream_id.clone(), message_id.clone());

        self.broadcast(stream_id, ChatEvent::MessagePinned(message));
        Ok(())
    }

    async fn unpin_message(&self, stream_id: &StreamId) -> LiveStreamingResult<()> {
        self.pinned_messages.remove(stream_id);
        self.broadcast(stream_id, ChatEvent::MessageUnpinned);
        Ok(())
    }

    async fn get_pinned_message(&self, stream_id: &StreamId) -> LiveStreamingResult<Option<ChatMessage>> {
        if let Some(message_id) = self.pinned_messages.get(stream_id) {
            Ok(self.messages.get(&message_id).map(|r| r.clone()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broadcaster::DefaultBroadcasterService;
    use crate::viewer::DefaultViewerService;

    async fn setup_services() -> (Arc<dyn BroadcasterService>, Arc<dyn ViewerService>, Arc<dyn ChatService>) {
        let broadcaster_service = Arc::new(DefaultBroadcasterService::new());
        let viewer_service = Arc::new(DefaultViewerService::new(broadcaster_service.clone()));
        let chat_service = Arc::new(DefaultChatService::new(broadcaster_service.clone(), viewer_service.clone()));
        (broadcaster_service, viewer_service, chat_service)
    }

    #[tokio::test]
    async fn test_send_message() {
        let (broadcaster, viewer, chat) = setup_services().await;

        let b = broadcaster.create_broadcaster("streamer".to_string(), "Streamer".to_string()).await.unwrap();
        let stream = broadcaster.create_stream(&b.id, "Test Stream".to_string(), StreamCategory::new("gaming", "Gaming")).await.unwrap();
        broadcaster.start_stream(&stream.id, b.stream_key.as_ref().unwrap()).await.unwrap();

        let v = viewer.create_viewer("viewer".to_string(), "Viewer".to_string()).await.unwrap();

        let msg = chat.send_message(SendMessageRequest {
            stream_id: stream.id.clone(),
            sender_id: v.id.clone(),
            sender_name: v.display_name.clone(),
            content: "Hello chat!".to_string(),
            color: None,
            badges: vec![],
        }).await.unwrap();

        assert_eq!(msg.content, "Hello chat!");
    }

    #[tokio::test]
    async fn test_slow_mode() {
        let (broadcaster, viewer, chat) = setup_services().await;

        let b = broadcaster.create_broadcaster("streamer".to_string(), "Streamer".to_string()).await.unwrap();
        let stream = broadcaster.create_stream(&b.id, "Test Stream".to_string(), StreamCategory::new("gaming", "Gaming")).await.unwrap();
        broadcaster.start_stream(&stream.id, b.stream_key.as_ref().unwrap()).await.unwrap();

        let v = viewer.create_viewer("viewer".to_string(), "Viewer".to_string()).await.unwrap();

        // Enable slow mode
        chat.enable_slow_mode(&stream.id, 10).await.unwrap();

        // First message should succeed
        chat.send_message(SendMessageRequest {
            stream_id: stream.id.clone(),
            sender_id: v.id.clone(),
            sender_name: v.display_name.clone(),
            content: "Message 1".to_string(),
            color: None,
            badges: vec![],
        }).await.unwrap();

        // Immediate second message should fail
        let result = chat.send_message(SendMessageRequest {
            stream_id: stream.id.clone(),
            sender_id: v.id.clone(),
            sender_name: v.display_name.clone(),
            content: "Message 2".to_string(),
            color: None,
            badges: vec![],
        }).await;

        assert!(result.is_err());
    }
}