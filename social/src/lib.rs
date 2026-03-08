//! Social features module for Vantis Media Player
//!
//! This module provides comprehensive social networking capabilities including:
//! - User profiles and relationships (friends, followers, blocks)
//! - Activity feeds and posts
//! - Comments and likes
//! - Direct and group messaging
//! - Notifications
//! - Media sharing
//! - User presence and online status
//!
//! ## Architecture
//!
//! The module is organized into several services:
//!
//! - `UserService` - User profile management
//! - `FriendService` - Friend relationships and blocking
//! - `FeedService` - Posts, comments, likes, and activity feeds
//! - `MessagingService` - Direct and group messages
//! - `NotificationService` - Push and in-app notifications
//! - `SharingService` - Media sharing capabilities
//! - `PresenceService` - Online status and activity tracking

pub mod types;
pub mod error;
pub mod user_service;
pub mod feed_service;
pub mod messaging_service;
pub mod notification_service;
pub mod sharing_service;
pub mod presence_service;

// Re-export main types and traits
pub use types::*;
pub use error::{SocialError, SocialResult};

// User service exports
pub use user_service::{
    UserService, DefaultUserService,
    FriendService, DefaultFriendService,
    ProfileUpdate, StatsUpdate,
};

// Feed service exports
pub use feed_service::{
    FeedService, DefaultFeedService,
    CreatePostRequest, CreateCommentRequest,
};

// Messaging service exports
pub use messaging_service::{
    MessagingService, DefaultMessagingService,
    SendMessageRequest, GroupUpdate, MessageEvent,
};

// Notification service exports
pub use notification_service::{
    NotificationService, DefaultNotificationService,
    CreateNotificationRequest, NotificationEvent,
};

// Sharing service exports
pub use sharing_service::{
    SharingService, DefaultSharingService,
    ShareToFeedRequest, ShareViaMessageRequest, ShareToExternalRequest,
    MultiShareRequest, ExternalShareResult, ShareRecord, NowPlayingRequest, EmbedCode,
};

// Presence service exports
pub use presence_service::{
    PresenceService, DefaultPresenceService,
    PresenceEvent,
};

use std::sync::Arc;

/// Social module configuration
#[derive(Debug, Clone)]
pub struct SocialConfig {
    /// Maximum post length in characters
    pub max_post_length: usize,
    /// Maximum comment length in characters
    pub max_comment_length: usize,
    /// Maximum message length in characters
    pub max_message_length: usize,
    /// Maximum friends per user
    pub max_friends: usize,
    /// Maximum group members
    pub max_group_members: usize,
    /// Enable real-time features
    pub enable_realtime: bool,
    /// Enable external platform sharing
    pub enable_external_sharing: bool,
    /// Rate limit for posts per hour
    pub posts_rate_limit: u32,
    /// Rate limit for messages per hour
    pub messages_rate_limit: u32,
}

impl Default for SocialConfig {
    fn default() -> Self {
        Self {
            max_post_length: 2000,
            max_comment_length: 1000,
            max_message_length: 5000,
            max_friends: 5000,
            max_group_members: 100,
            enable_realtime: true,
            enable_external_sharing: true,
            posts_rate_limit: 50,
            messages_rate_limit: 100,
        }
    }
}

/// Container for all social services
pub struct SocialServices {
    pub users: Arc<dyn UserService>,
    pub friends: Arc<dyn FriendService>,
    pub feed: Arc<dyn FeedService>,
    pub messaging: Arc<dyn MessagingService>,
    pub notifications: Arc<dyn NotificationService>,
    pub sharing: Arc<dyn SharingService>,
    pub presence: Arc<dyn PresenceService>,
}

impl SocialServices {
    /// Create a new set of social services with default implementations
    pub fn new() -> Self {
        let users = Arc::new(DefaultUserService::new());
        let friends = Arc::new(DefaultFriendService::new(users.clone()));
        let feed = Arc::new(DefaultFeedService::new(users.clone()));
        let messaging = Arc::new(DefaultMessagingService::new(users.clone()));
        let notifications = Arc::new(DefaultNotificationService::new(users.clone()));
        let sharing = Arc::new(DefaultSharingService::new(
            users.clone(),
            feed.clone(),
            messaging.clone(),
        ));
        let presence = Arc::new(DefaultPresenceService::new(users.clone()));

        Self {
            users,
            friends,
            feed,
            messaging,
            notifications,
            sharing,
            presence,
        }
    }
}

impl Default for SocialServices {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating custom social service configurations
pub struct SocialServicesBuilder {
    config: SocialConfig,
}

impl SocialServicesBuilder {
    pub fn new() -> Self {
        Self {
            config: SocialConfig::default(),
        }
    }

    pub fn with_config(mut self, config: SocialConfig) -> Self {
        self.config = config;
        self
    }

    pub fn max_post_length(mut self, len: usize) -> Self {
        self.config.max_post_length = len;
        self
    }

    pub fn max_comment_length(mut self, len: usize) -> Self {
        self.config.max_comment_length = len;
        self
    }

    pub fn max_friends(mut self, count: usize) -> Self {
        self.config.max_friends = count;
        self
    }

    pub fn enable_realtime(mut self, enabled: bool) -> Self {
        self.config.enable_realtime = enabled;
        self
    }

    pub fn build(self) -> SocialServices {
        // In a more complex implementation, we'd use the config
        // For now, just create default services
        SocialServices::new()
    }
}

impl Default for SocialServicesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_social_services_integration() {
        let services = SocialServices::new();

        // Create a user
        let user = services.users.create_profile("testuser".to_string(), "Test User".to_string())
            .await
            .unwrap();

        // Update presence
        services.presence.update_presence(&user.id, PresenceStatus::Online)
            .await
            .unwrap();

        // Create a post
        let post = services.feed.create_post(CreatePostRequest {
            author_id: user.id.clone(),
            content: PostContent::text("Hello, world!".to_string()),
            visibility: PostVisibility::Public,
            tags: vec![],
            mentions: vec![],
        }).await.unwrap();

        // Get feed
        let feed = services.feed.get_feed(&user.id, 10, None).await.unwrap();
        assert_eq!(feed.items.len(), 1);
    }

    #[tokio::test]
    async fn test_friend_workflow() {
        let services = SocialServices::new();

        // Create two users
        let user1 = services.users.create_profile("user1".to_string(), "User One".to_string())
            .await
            .unwrap();
        let user2 = services.users.create_profile("user2".to_string(), "User Two".to_string())
            .await
            .unwrap();

        // Send friend request
        let friendship = services.friends.send_friend_request(&user1.id, &user2.id)
            .await
            .unwrap();
        assert_eq!(friendship.status, FriendshipStatus::Pending);

        // Get pending requests
        let pending = services.friends.get_pending_requests(&user2.id).await.unwrap();
        assert_eq!(pending.len(), 1);

        // Accept friend request
        let accepted = services.friends.accept_friend_request(&friendship.id, &user2.id)
            .await
            .unwrap();
        assert_eq!(accepted.status, FriendshipStatus::Accepted);

        // Get friends
        let friends = services.friends.get_friends(&user1.id, 10, 0).await.unwrap();
        assert_eq!(friends.len(), 1);
    }

    #[tokio::test]
    async fn test_messaging() {
        let services = SocialServices::new();

        // Create users
        let user1 = services.users.create_profile("user1".to_string(), "User One".to_string())
            .await
            .unwrap();
        let user2 = services.users.create_profile("user2".to_string(), "User Two".to_string())
            .await
            .unwrap();

        // Create conversation
        let conv = services.messaging.create_conversation(vec![user1.id.clone(), user2.id.clone()])
            .await
            .unwrap();

        // Send message
        let msg = services.messaging.send_message(SendMessageRequest {
            conversation_id: conv.id.clone(),
            sender_id: user1.id.clone(),
            content: MessageContent::text("Hello!".to_string()),
            reply_to: None,
        }).await.unwrap();

        assert_eq!(msg.content.text, Some("Hello!".to_string()));

        // Get messages
        let messages = services.messaging.get_messages(&conv.id, 10, None).await.unwrap();
        assert_eq!(messages.len(), 1);
    }

    #[tokio::test]
    async fn test_sharing() {
        let services = SocialServices::new();

        let user = services.users.create_profile("testuser".to_string(), "Test User".to_string())
            .await
            .unwrap();

        // Share to feed
        let post = services.sharing.share_to_feed(ShareToFeedRequest {
            user_id: user.id.clone(),
            media_type: SharedMediaType::Song,
            media_id: "song123".to_string(),
            media_title: "Test Song".to_string(),
            message: Some("Great song!".to_string()),
            visibility: PostVisibility::Public,
            thumbnail_url: None,
            artist: Some("Artist".to_string()),
            album: None,
            duration_seconds: Some(180),
        }).await.unwrap();

        assert!(post.content.media_attachment.is_some());

        // Now playing
        let now_playing = services.sharing.share_now_playing(NowPlayingRequest {
            user_id: user.id.clone(),
            media_type: SharedMediaType::Song,
            media_id: "song456".to_string(),
            media_title: "Another Song".to_string(),
            artist: Some("Artist".to_string()),
            album: None,
            current_position_seconds: 60,
            total_duration_seconds: 180,
            thumbnail_url: None,
        }).await.unwrap();

        assert!(now_playing.tags.contains(&"nowplaying".to_string()));
    }

    #[tokio::test]
    async fn test_notifications() {
        let services = SocialServices::new();

        let user = services.users.create_profile("testuser".to_string(), "Test User".to_string())
            .await
            .unwrap();

        // Create notification
        let notif = services.notifications.create_notification(CreateNotificationRequest {
            user_id: user.id.clone(),
            notification_type: NotificationType::System,
            title: "Test".to_string(),
            body: "Test notification".to_string(),
            action_url: None,
            actor_id: None,
            image_url: None,
        }).await.unwrap();

        // Get unread count
        let count = services.notifications.get_unread_count(&user.id).await.unwrap();
        assert_eq!(count, 1);

        // Mark as read
        services.notifications.mark_as_read(&notif.id, &user.id).await.unwrap();

        let count = services.notifications.get_unread_count(&user.id).await.unwrap();
        assert_eq!(count, 0);
    }
}