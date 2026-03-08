//! Media sharing service

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;
use crate::user_service::UserService;
use crate::feed_service::FeedService;
use crate::messaging_service::MessagingService;

/// Trait for media sharing functionality
#[async_trait]
pub trait SharingService: Send + Sync {
    /// Share media to activity feed
    async fn share_to_feed(&self, request: ShareToFeedRequest) -> SocialResult<Post>;

    /// Share media via direct message
    async fn share_via_message(&self, request: ShareViaMessageRequest) -> SocialResult<Message>;

    /// Share to external platform
    async fn share_to_external(&self, request: ShareToExternalRequest) -> SocialResult<ExternalShareResult>;

    /// Share to multiple targets at once
    async fn share_multi(&self, request: MultiShareRequest) -> SocialResult<ShareResult>;

    /// Get sharing history for media
    async fn get_sharing_history(&self, media_id: &str, media_type: SharedMediaType) -> SocialResult<Vec<ShareRecord>>;

    /// Get shared media for a user
    async fn get_user_shared_media(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Post>>;

    /// Check if media is shared by user
    async fn is_shared_by_user(&self, user_id: &UserId, media_id: &str) -> SocialResult<bool>;

    /// Get share count for media
    async fn get_share_count(&self, media_id: &str) -> SocialResult<u64>;

    /// Create "Now Playing" post
    async fn share_now_playing(&self, request: NowPlayingRequest) -> SocialResult<Post>;

    /// Generate share link
    async fn generate_share_link(&self, media_id: &str, media_type: SharedMediaType) -> SocialResult<String>;

    /// Get embed code for sharing
    async fn get_embed_code(&self, media_id: &str, media_type: SharedMediaType) -> SocialResult<EmbedCode>;
}

/// Request to share to feed
#[derive(Debug, Clone)]
pub struct ShareToFeedRequest {
    pub user_id: UserId,
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub message: Option<String>,
    pub visibility: PostVisibility,
    pub thumbnail_url: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_seconds: Option<u64>,
}

/// Request to share via message
#[derive(Debug, Clone)]
pub struct ShareViaMessageRequest {
    pub sender_id: UserId,
    pub recipient_id: UserId,
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub message: Option<String>,
    pub thumbnail_url: Option<String>,
}

/// Request to share to external platform
#[derive(Debug, Clone)]
pub struct ShareToExternalRequest {
    pub user_id: UserId,
    pub platform: ConnectedPlatform,
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub message: Option<String>,
    pub thumbnail_url: Option<String>,
}

/// Request for multi-target share
#[derive(Debug, Clone)]
pub struct MultiShareRequest {
    pub user_id: UserId,
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub message: Option<String>,
    pub targets: Vec<ShareTarget>,
    pub thumbnail_url: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

/// Result of external share
#[derive(Debug, Clone)]
pub struct ExternalShareResult {
    pub platform: ConnectedPlatform,
    pub success: bool,
    pub post_url: Option<String>,
    pub error: Option<String>,
}

/// Record of a share
#[derive(Debug, Clone)]
pub struct ShareRecord {
    pub id: Uuid,
    pub user_id: UserId,
    pub media_id: String,
    pub media_type: SharedMediaType,
    pub media_title: String,
    pub target: ShareTarget,
    pub shared_at: chrono::DateTime<Utc>,
    pub message: Option<String>,
}

/// Request for now playing post
#[derive(Debug, Clone)]
pub struct NowPlayingRequest {
    pub user_id: UserId,
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub current_position_seconds: u64,
    pub total_duration_seconds: u64,
    pub thumbnail_url: Option<String>,
}

/// Embed code for sharing
#[derive(Debug, Clone)]
pub struct EmbedCode {
    pub html: String,
    pub width: u32,
    pub height: u32,
}

/// Default implementation of SharingService
pub struct DefaultSharingService {
    share_records: DashMap<Uuid, ShareRecord>,
    media_share_counts: DashMap<String, u64>,
    user_service: Arc<dyn UserService>,
    feed_service: Arc<dyn FeedService>,
    messaging_service: Arc<dyn MessagingService>,
}

impl DefaultSharingService {
    pub fn new(
        user_service: Arc<dyn UserService>,
        feed_service: Arc<dyn FeedService>,
        messaging_service: Arc<dyn MessagingService>,
    ) -> Self {
        Self {
            share_records: DashMap::new(),
            media_share_counts: DashMap::new(),
            user_service,
            feed_service,
            messaging_service,
        }
    }

    fn create_media_attachment(&self, request: &ShareToFeedRequest) -> MediaAttachment {
        MediaAttachment {
            media_type: request.media_type,
            media_id: request.media_id.clone(),
            title: request.media_title.clone(),
            thumbnail_url: request.thumbnail_url.clone(),
            duration_seconds: request.duration_seconds,
            artist: request.artist.clone(),
            album: request.album.clone(),
            position: None,
        }
    }

    fn increment_share_count(&self, media_id: &str) {
        let mut count = self.media_share_counts.entry(media_id.to_string()).or_insert(0);
        *count += 1;
    }
}

#[async_trait]
impl SharingService for DefaultSharingService {
    async fn share_to_feed(&self, request: ShareToFeedRequest) -> SocialResult<Post> {
        // Verify user exists
        self.user_service.get_profile(&request.user_id).await?;

        let attachment = self.create_media_attachment(&request);

        let content = PostContent::with_media(request.message.clone(), attachment);

        let post_request = crate::feed_service::CreatePostRequest {
            author_id: request.user_id.clone(),
            content,
            visibility: request.visibility,
            tags: vec![],
            mentions: vec![],
        };

        let post = self.feed_service.create_post(post_request).await?;

        // Record the share
        let record = ShareRecord {
            id: Uuid::new_v4(),
            user_id: request.user_id.clone(),
            media_id: request.media_id.clone(),
            media_type: request.media_type,
            media_title: request.media_title.clone(),
            target: ShareTarget::ActivityFeed,
            shared_at: Utc::now(),
            message: request.message.clone(),
        };
        self.share_records.insert(record.id, record);

        // Increment share count
        self.increment_share_count(&request.media_id);

        // Update user stats
        self.user_service.update_user_stats(&request.user_id, crate::user_service::StatsUpdate {
            media_shared_delta: 1,
            ..Default::default()
        }).await.ok();

        Ok(post)
    }

    async fn share_via_message(&self, request: ShareViaMessageRequest) -> SocialResult<Message> {
        // Verify users exist
        self.user_service.get_profile(&request.sender_id).await?;
        self.user_service.get_profile(&request.recipient_id).await?;

        // Create or get conversation
        let conversations = self.messaging_service.get_user_conversations(&request.sender_id, 100, 0).await?;
        let existing_conv = conversations.iter().find(|c| {
            !c.is_group && c.participants.contains(&request.sender_id) && c.participants.contains(&request.recipient_id)
        });

        let conversation = if let Some(conv) = existing_conv {
            conv.clone()
        } else {
            self.messaging_service.create_conversation(vec![request.sender_id.clone(), request.recipient_id.clone()]).await?
        };

        // Create message content with shared media
        let attachment = MediaAttachment {
            media_type: request.media_type,
            media_id: request.media_id.clone(),
            title: request.media_title.clone(),
            thumbnail_url: request.thumbnail_url.clone(),
            duration_seconds: None,
            artist: None,
            album: None,
            position: None,
        };

        let content = MessageContent {
            text: request.message.clone(),
            media_attachment: None,
            shared_post: None,
            shared_media: Some(attachment),
        };

        let message_request = crate::messaging_service::SendMessageRequest {
            conversation_id: conversation.id.clone(),
            sender_id: request.sender_id.clone(),
            content,
            reply_to: None,
        };

        let message = self.messaging_service.send_message(message_request).await?;

        // Record the share
        let record = ShareRecord {
            id: Uuid::new_v4(),
            user_id: request.sender_id.clone(),
            media_id: request.media_id.clone(),
            media_type: request.media_type,
            media_title: request.media_title.clone(),
            target: ShareTarget::DirectMessage(request.recipient_id.clone()),
            shared_at: Utc::now(),
            message: request.message.clone(),
        };
        self.share_records.insert(record.id, record);

        // Increment share count
        self.increment_share_count(&request.media_id);

        Ok(message)
    }

    async fn share_to_external(&self, request: ShareToExternalRequest) -> SocialResult<ExternalShareResult> {
        // Verify user exists
        let profile = self.user_service.get_profile(&request.user_id).await?;

        // Check if platform is connected
        if !profile.settings.sharing.share_to_connected_accounts.contains(&request.platform) {
            return Err(SocialError::InvalidOperation(format!("Platform {:?} is not connected", request.platform)));
        }

        // In a real implementation, this would call the platform's API
        // For now, we simulate a successful share
        let post_url = match request.platform {
            ConnectedPlatform::Twitter => Some(format!("https://twitter.com/{}/status/{}", profile.username, Uuid::new_v4())),
            ConnectedPlatform::Facebook => Some(format!("https://facebook.com/posts/{}", Uuid::new_v4())),
            ConnectedPlatform::Instagram => Some(format!("https://instagram.com/p/{}", Uuid::new_v4())),
            ConnectedPlatform::Discord => None,
            ConnectedPlatform::Spotify => None,
            ConnectedPlatform::LastFm => None,
            ConnectedPlatform::YouTube => Some(format!("https://youtube.com/watch?v={}", request.media_id)),
        };

        // Record the share
        let record = ShareRecord {
            id: Uuid::new_v4(),
            user_id: request.user_id.clone(),
            media_id: request.media_id.clone(),
            media_type: request.media_type,
            media_title: request.media_title.clone(),
            target: ShareTarget::ExternalPlatform(request.platform),
            shared_at: Utc::now(),
            message: request.message.clone(),
        };
        self.share_records.insert(record.id, record);

        // Increment share count
        self.increment_share_count(&request.media_id);

        Ok(ExternalShareResult {
            platform: request.platform,
            success: true,
            post_url,
            error: None,
        })
    }

    async fn share_multi(&self, request: MultiShareRequest) -> SocialResult<ShareResult> {
        let mut post_id: Option<PostId> = None;
        let mut shared_to: Vec<ShareTarget> = Vec::new();
        let mut external_urls: std::collections::HashMap<ConnectedPlatform, String> = std::collections::HashMap::new();

        for target in request.targets.clone() {
            match target {
                ShareTarget::ActivityFeed => {
                    let share_request = ShareToFeedRequest {
                        user_id: request.user_id.clone(),
                        media_type: request.media_type,
                        media_id: request.media_id.clone(),
                        media_title: request.media_title.clone(),
                        message: request.message.clone(),
                        visibility: PostVisibility::Public,
                        thumbnail_url: request.thumbnail_url.clone(),
                        artist: request.artist.clone(),
                        album: request.album.clone(),
                        duration_seconds: None,
                    };

                    let post = self.share_to_feed(share_request).await?;
                    post_id = Some(post.id);
                    shared_to.push(ShareTarget::ActivityFeed);
                }
                ShareTarget::DirectMessage(recipient_id) => {
                    let share_request = ShareViaMessageRequest {
                        sender_id: request.user_id.clone(),
                        recipient_id,
                        media_type: request.media_type,
                        media_id: request.media_id.clone(),
                        media_title: request.media_title.clone(),
                        message: request.message.clone(),
                        thumbnail_url: request.thumbnail_url.clone(),
                    };

                    self.share_via_message(share_request).await?;
                    shared_to.push(ShareTarget::DirectMessage(recipient_id));
                }
                ShareTarget::Group(_group_id) => {
                    // Would share to group chat
                    shared_to.push(target);
                }
                ShareTarget::ExternalPlatform(platform) => {
                    let share_request = ShareToExternalRequest {
                        user_id: request.user_id.clone(),
                        platform,
                        media_type: request.media_type,
                        media_id: request.media_id.clone(),
                        media_title: request.media_title.clone(),
                        message: request.message.clone(),
                        thumbnail_url: request.thumbnail_url.clone(),
                    };

                    let result = self.share_to_external(share_request).await?;
                    if result.success {
                        if let Some(url) = result.post_url {
                            external_urls.insert(platform, url);
                        }
                        shared_to.push(ShareTarget::ExternalPlatform(platform));
                    }
                }
            }
        }

        Ok(ShareResult {
            success: !shared_to.is_empty(),
            post_id,
            shared_to,
            external_urls,
        })
    }

    async fn get_sharing_history(&self, media_id: &str, _media_type: SharedMediaType) -> SocialResult<Vec<ShareRecord>> {
        let records: Vec<ShareRecord> = self.share_records
            .iter()
            .filter(|e| e.value().media_id == media_id)
            .map(|e| e.value().clone())
            .collect();

        Ok(records)
    }

    async fn get_user_shared_media(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Post>> {
        let posts = self.feed_service.get_user_posts(user_id, limit, offset).await?;

        // Filter to only include posts with media attachments
        let media_posts: Vec<Post> = posts
            .into_iter()
            .filter(|p| p.content.media_attachment.is_some())
            .collect();

        Ok(media_posts)
    }

    async fn is_shared_by_user(&self, user_id: &UserId, media_id: &str) -> SocialResult<bool> {
        let exists = self.share_records
            .iter()
            .any(|e| &e.value().user_id == user_id && e.value().media_id == media_id);

        Ok(exists)
    }

    async fn get_share_count(&self, media_id: &str) -> SocialResult<u64> {
        let count = self.media_share_counts
            .get(media_id)
            .map(|r| *r)
            .unwrap_or(0);

        Ok(count)
    }

    async fn share_now_playing(&self, request: NowPlayingRequest) -> SocialResult<Post> {
        // Verify user exists
        self.user_service.get_profile(&request.user_id).await?;

        let attachment = MediaAttachment {
            media_type: request.media_type,
            media_id: request.media_id.clone(),
            title: request.media_title.clone(),
            thumbnail_url: request.thumbnail_url.clone(),
            duration_seconds: Some(request.total_duration_seconds),
            artist: request.artist.clone(),
            album: request.album.clone(),
            position: Some(MediaPosition {
                current_seconds: request.current_position_seconds,
                total_seconds: request.total_duration_seconds,
            }),
        };

        let content = PostContent::with_media(Some("🎵 Now Playing".to_string()), attachment);

        let post_request = crate::feed_service::CreatePostRequest {
            author_id: request.user_id.clone(),
            content,
            visibility: PostVisibility::Public,
            tags: vec!["nowplaying".to_string()],
            mentions: vec![],
        };

        let post = self.feed_service.create_post(post_request).await?;

        // Record activity
        let activity = UserActivity {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            activity_type: ActivityType::NowPlaying,
            created_at: Utc::now(),
            visibility: ActivityVisibility::Public,
            details: ActivityDetails {
                media_id: Some(request.media_id),
                media_title: Some(request.media_title),
                media_type: Some(request.media_type),
                target_user_id: None,
                target_post_id: None,
                playlist_name: None,
                additional_info: serde_json::json!({
                    "artist": request.artist,
                    "album": request.album,
                }),
            },
        };

        self.feed_service.record_activity(activity).await?;

        Ok(post)
    }

    async fn generate_share_link(&self, media_id: &str, media_type: SharedMediaType) -> SocialResult<String> {
        // Generate a shareable link for the media
        let type_path = match media_type {
            SharedMediaType::Song => "song",
            SharedMediaType::Album => "album",
            SharedMediaType::Playlist => "playlist",
            SharedMediaType::Artist => "artist",
            SharedMediaType::Video => "video",
            SharedMediaType::Movie => "movie",
            SharedMediaType::TVShow => "show",
            SharedMediaType::Podcast => "podcast",
            SharedMediaType::Audiobook => "audiobook",
        };

        Ok(format!("https://vantis.media/{}/{}", type_path, media_id))
    }

    async fn get_embed_code(&self, media_id: &str, media_type: SharedMediaType) -> SocialResult<EmbedCode> {
        let type_str = match media_type {
            SharedMediaType::Song => "player",
            SharedMediaType::Album => "album",
            SharedMediaType::Playlist => "playlist",
            SharedMediaType::Video => "video",
            _ => "player",
        };

        let html = format!(
            r#"<iframe src="https://embed.vantis.media/{}/{}/{}" width="400" height="300" frameborder="0" allowfullscreen></iframe>"#,
            type_str, media_id, "?theme=dark"
        );

        Ok(EmbedCode {
            html,
            width: 400,
            height: 300,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::DefaultUserService;
    use crate::feed_service::DefaultFeedService;
    use crate::messaging_service::DefaultMessagingService;

    async fn setup_services() -> (Arc<dyn UserService>, Arc<dyn FeedService>, Arc<dyn MessagingService>) {
        let user_service = Arc::new(DefaultUserService::new());
        let feed_service = Arc::new(DefaultFeedService::new(user_service.clone()));
        let messaging_service = Arc::new(DefaultMessagingService::new(user_service.clone()));
        (user_service, feed_service, messaging_service)
    }

    #[tokio::test]
    async fn test_share_to_feed() {
        let (user_service, feed_service, messaging_service) = setup_services().await;
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let sharing_service = DefaultSharingService::new(user_service, feed_service, messaging_service);

        let post = sharing_service.share_to_feed(ShareToFeedRequest {
            user_id: user.id.clone(),
            media_type: SharedMediaType::Song,
            media_id: "song123".to_string(),
            media_title: "Test Song".to_string(),
            message: Some("Check this out!".to_string()),
            visibility: PostVisibility::Public,
            thumbnail_url: None,
            artist: Some("Artist".to_string()),
            album: None,
            duration_seconds: Some(180),
        }).await.unwrap();

        assert!(post.content.media_attachment.is_some());
    }

    #[tokio::test]
    async fn test_share_count() {
        let (user_service, feed_service, messaging_service) = setup_services().await;
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let sharing_service = DefaultSharingService::new(user_service, feed_service, messaging_service);

        let count_before = sharing_service.get_share_count("song123").await.unwrap();
        assert_eq!(count_before, 0);

        sharing_service.share_to_feed(ShareToFeedRequest {
            user_id: user.id.clone(),
            media_type: SharedMediaType::Song,
            media_id: "song123".to_string(),
            media_title: "Test Song".to_string(),
            message: None,
            visibility: PostVisibility::Public,
            thumbnail_url: None,
            artist: None,
            album: None,
            duration_seconds: None,
        }).await.unwrap();

        let count_after = sharing_service.get_share_count("song123").await.unwrap();
        assert_eq!(count_after, 1);
    }

    #[tokio::test]
    async fn test_now_playing() {
        let (user_service, feed_service, messaging_service) = setup_services().await;
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let sharing_service = DefaultSharingService::new(user_service, feed_service, messaging_service);

        let post = sharing_service.share_now_playing(NowPlayingRequest {
            user_id: user.id.clone(),
            media_type: SharedMediaType::Song,
            media_id: "song456".to_string(),
            media_title: "Currently Playing".to_string(),
            artist: Some("Artist".to_string()),
            album: Some("Album".to_string()),
            current_position_seconds: 60,
            total_duration_seconds: 180,
            thumbnail_url: None,
        }).await.unwrap();

        assert!(post.tags.contains(&"nowplaying".to_string()));
        let attachment = post.content.media_attachment.unwrap();
        assert!(attachment.position.is_some());
    }
}