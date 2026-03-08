//! Notification service for social features

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;
use crate::user_service::UserService;

/// Trait for notification management
#[async_trait]
pub trait NotificationService: Send + Sync {
    /// Create a new notification
    async fn create_notification(&self, notification: CreateNotificationRequest) -> SocialResult<Notification>;

    /// Get notification by ID
    async fn get_notification(&self, notification_id: &NotificationId) -> SocialResult<Notification>;

    /// Get notifications for a user
    async fn get_user_notifications(&self, user_id: &UserId, limit: usize, offset: usize, unread_only: bool) -> SocialResult<Vec<Notification>>;

    /// Mark notification as read
    async fn mark_as_read(&self, notification_id: &NotificationId, user_id: &UserId) -> SocialResult<()>;

    /// Mark all notifications as read for a user
    async fn mark_all_as_read(&self, user_id: &UserId) -> SocialResult<u64>;

    /// Delete a notification
    async fn delete_notification(&self, notification_id: &NotificationId, user_id: &UserId) -> SocialResult<()>;

    /// Get unread count for a user
    async fn get_unread_count(&self, user_id: &UserId) -> SocialResult<u64>;

    /// Subscribe to real-time notifications
    async fn subscribe(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<NotificationEvent>>;

    /// Create friend request notification
    async fn notify_friend_request(&self, to_user: &UserId, from_user: &UserId) -> SocialResult<Notification>;

    /// Create friend accepted notification
    async fn notify_friend_accepted(&self, to_user: &UserId, from_user: &UserId) -> SocialResult<Notification>;

    /// Create mention notification
    async fn notify_mention(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification>;

    /// Create like notification
    async fn notify_like(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification>;

    /// Create comment notification
    async fn notify_comment(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId, comment_id: &CommentId) -> SocialResult<Notification>;

    /// Create message notification
    async fn notify_message(&self, to_user: &UserId, from_user: &UserId, conversation_id: &ConversationId) -> SocialResult<Notification>;

    /// Create share notification
    async fn notify_share(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification>;

    /// Create system notification
    async fn notify_system(&self, to_user: &UserId, title: String, body: String, action_url: Option<String>) -> SocialResult<Notification>;
}

/// Request to create a notification
#[derive(Debug, Clone)]
pub struct CreateNotificationRequest {
    pub user_id: UserId,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub action_url: Option<String>,
    pub actor_id: Option<UserId>,
    pub image_url: Option<String>,
}

/// Notification event for real-time updates
#[derive(Debug, Clone)]
pub enum NotificationEvent {
    NewNotification(Notification),
    NotificationRead(NotificationId),
    AllNotificationsRead(UserId),
    NotificationDeleted(NotificationId),
}

/// Default in-memory implementation of NotificationService
pub struct DefaultNotificationService {
    notifications: DashMap<NotificationId, Notification>,
    user_notifications: DashMap<UserId, Vec<NotificationId>>,
    event_senders: RwLock<HashMap<UserId, broadcast::Sender<NotificationEvent>>>,
    user_service: Arc<dyn UserService>,
}

impl DefaultNotificationService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self {
            notifications: DashMap::new(),
            user_notifications: DashMap::new(),
            event_senders: RwLock::new(HashMap::new()),
            user_service,
        }
    }

    fn broadcast_event(&self, user_id: &UserId, event: NotificationEvent) {
        let senders = self.event_senders.read();
        if let Some(sender) = senders.get(user_id) {
            let _ = sender.send(event);
        }
    }

    fn get_or_create_sender(&self, user_id: &UserId) -> broadcast::Sender<NotificationEvent> {
        let mut senders = self.event_senders.write();
        senders.entry(user_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone()
    }
}

#[async_trait]
impl NotificationService for DefaultNotificationService {
    async fn create_notification(&self, notification: CreateNotificationRequest) -> SocialResult<Notification> {
        // Verify user exists
        self.user_service.get_profile(&notification.user_id).await?;

        let mut new_notification = Notification::new(
            notification.user_id.clone(),
            notification.notification_type,
            notification.title,
            notification.body,
        );

        new_notification.action_url = notification.action_url;
        new_notification.actor_id = notification.actor_id;
        new_notification.image_url = notification.image_url;

        let notification_id = new_notification.id.clone();

        // Add to notifications
        self.notifications.insert(notification_id.clone(), new_notification.clone());

        // Add to user notifications index
        let mut user_notifs = self.user_notifications.entry(notification.user_id.clone()).or_insert_with(Vec::new);
        user_notifs.push(notification_id.clone());

        // Broadcast event
        self.broadcast_event(&notification.user_id, NotificationEvent::NewNotification(new_notification.clone()));

        Ok(new_notification)
    }

    async fn get_notification(&self, notification_id: &NotificationId) -> SocialResult<Notification> {
        self.notifications
            .get(notification_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::InternalError("Notification not found".to_string()))
    }

    async fn get_user_notifications(&self, user_id: &UserId, limit: usize, offset: usize, unread_only: bool) -> SocialResult<Vec<Notification>> {
        let notif_ids = self.user_notifications
            .get(user_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut notifications: Vec<Notification> = notif_ids
            .into_iter()
            .filter_map(|id| self.notifications.get(&id).map(|r| r.clone()))
            .filter(|n| !unread_only || n.read_at.is_none())
            .collect();

        // Sort by created_at descending
        notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        notifications = notifications.into_iter().skip(offset).take(limit).collect();

        Ok(notifications)
    }

    async fn mark_as_read(&self, notification_id: &NotificationId, user_id: &UserId) -> SocialResult<()> {
        let mut notification = self.get_notification(notification_id).await?;

        if &notification.user_id != user_id {
            return Err(SocialError::Unauthorized);
        }

        notification.read_at = Some(Utc::now());
        self.notifications.insert(notification_id.clone(), notification);

        // Broadcast event
        self.broadcast_event(user_id, NotificationEvent::NotificationRead(notification_id.clone()));

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: &UserId) -> SocialResult<u64> {
        let notif_ids = self.user_notifications
            .get(user_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut count = 0u64;

        for id in notif_ids {
            if let Some(mut notif) = self.notifications.get(&id).map(|r| r.clone()) {
                if notif.read_at.is_none() {
                    notif.read_at = Some(Utc::now());
                    self.notifications.insert(id, notif);
                    count += 1;
                }
            }
        }

        // Broadcast event
        self.broadcast_event(user_id, NotificationEvent::AllNotificationsRead(user_id.clone()));

        Ok(count)
    }

    async fn delete_notification(&self, notification_id: &NotificationId, user_id: &UserId) -> SocialResult<()> {
        let notification = self.get_notification(notification_id).await?;

        if &notification.user_id != user_id {
            return Err(SocialError::Unauthorized);
        }

        self.notifications.remove(notification_id);

        // Remove from user index
        if let Some(mut notif_ids) = self.user_notifications.get_mut(user_id) {
            notif_ids.retain(|id| id != notification_id);
        }

        // Broadcast event
        self.broadcast_event(user_id, NotificationEvent::NotificationDeleted(notification_id.clone()));

        Ok(())
    }

    async fn get_unread_count(&self, user_id: &UserId) -> SocialResult<u64> {
        let notif_ids = self.user_notifications
            .get(user_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let count = notif_ids
            .iter()
            .filter_map(|id| self.notifications.get(id))
            .filter(|n| n.read_at.is_none())
            .count() as u64;

        Ok(count)
    }

    async fn subscribe(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<NotificationEvent>> {
        let sender = self.get_or_create_sender(user_id);
        Ok(sender.subscribe())
    }

    async fn notify_friend_request(&self, to_user: &UserId, from_user: &UserId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::FriendRequest,
            title: "New Friend Request".to_string(),
            body: format!("{} wants to be your friend", from_profile.display_name),
            action_url: Some(format!("/friends/requests")),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_friend_accepted(&self, to_user: &UserId, from_user: &UserId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::FriendAccepted,
            title: "Friend Request Accepted".to_string(),
            body: format!("{} accepted your friend request", from_profile.display_name),
            action_url: Some(format!("/profile/{}", from_profile.username)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_mention(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::Mention,
            title: "You were mentioned".to_string(),
            body: format!("{} mentioned you in a post", from_profile.display_name),
            action_url: Some(format!("/post/{}", post_id.0)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_like(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::Like,
            title: "New like".to_string(),
            body: format!("{} liked your post", from_profile.display_name),
            action_url: Some(format!("/post/{}", post_id.0)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_comment(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId, _comment_id: &CommentId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::Comment,
            title: "New comment".to_string(),
            body: format!("{} commented on your post", from_profile.display_name),
            action_url: Some(format!("/post/{}", post_id.0)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_message(&self, to_user: &UserId, from_user: &UserId, conversation_id: &ConversationId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::Message,
            title: "New message".to_string(),
            body: format!("{} sent you a message", from_profile.display_name),
            action_url: Some(format!("/messages/{}", conversation_id.0)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_share(&self, to_user: &UserId, from_user: &UserId, post_id: &PostId) -> SocialResult<Notification> {
        let from_profile = self.user_service.get_profile(from_user).await?;

        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::Share,
            title: "Post shared".to_string(),
            body: format!("{} shared your post", from_profile.display_name),
            action_url: Some(format!("/post/{}", post_id.0)),
            actor_id: Some(from_user.clone()),
            image_url: from_profile.avatar_url,
        }).await
    }

    async fn notify_system(&self, to_user: &UserId, title: String, body: String, action_url: Option<String>) -> SocialResult<Notification> {
        self.create_notification(CreateNotificationRequest {
            user_id: to_user.clone(),
            notification_type: NotificationType::System,
            title,
            body,
            action_url,
            actor_id: None,
            image_url: None,
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::DefaultUserService;

    #[tokio::test]
    async fn test_create_notification() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let notif_service = DefaultNotificationService::new(user_service);

        let notif = notif_service.create_notification(CreateNotificationRequest {
            user_id: user.id.clone(),
            notification_type: NotificationType::System,
            title: "Test".to_string(),
            body: "Test notification".to_string(),
            action_url: None,
            actor_id: None,
            image_url: None,
        }).await.unwrap();

        assert_eq!(notif.title, "Test");
        assert!(notif.read_at.is_none());
    }

    #[tokio::test]
    async fn test_mark_as_read() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let notif_service = DefaultNotificationService::new(user_service);

        let notif = notif_service.create_notification(CreateNotificationRequest {
            user_id: user.id.clone(),
            notification_type: NotificationType::System,
            title: "Test".to_string(),
            body: "Test notification".to_string(),
            action_url: None,
            actor_id: None,
            image_url: None,
        }).await.unwrap();

        notif_service.mark_as_read(&notif.id, &user.id).await.unwrap();

        let updated = notif_service.get_notification(&notif.id).await.unwrap();
        assert!(updated.read_at.is_some());
    }

    #[tokio::test]
    async fn test_unread_count() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let notif_service = DefaultNotificationService::new(user_service);

        // Create 3 notifications
        for i in 0..3 {
            notif_service.create_notification(CreateNotificationRequest {
                user_id: user.id.clone(),
                notification_type: NotificationType::System,
                title: format!("Test {}", i),
                body: "Test notification".to_string(),
                action_url: None,
                actor_id: None,
                image_url: None,
            }).await.unwrap();
        }

        let count = notif_service.get_unread_count(&user.id).await.unwrap();
        assert_eq!(count, 3);

        // Mark one as read
        let notifs = notif_service.get_user_notifications(&user.id, 1, 0, true).await.unwrap();
        notif_service.mark_as_read(&notifs[0].id, &user.id).await.unwrap();

        let count = notif_service.get_unread_count(&user.id).await.unwrap();
        assert_eq!(count, 2);
    }
}