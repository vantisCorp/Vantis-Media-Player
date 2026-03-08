//! Core types for the social module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a social post
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PostId(pub Uuid);

impl PostId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a comment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(pub Uuid);

impl CommentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a conversation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(pub Uuid);

impl ConversationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a message
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a notification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationId(pub Uuid);

impl NotificationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: UserId,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_verified: bool,
    pub is_private: bool,
    pub settings: UserSettings,
    pub stats: UserStats,
}

impl UserProfile {
    pub fn new(username: String, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            username,
            display_name,
            avatar_url: None,
            banner_url: None,
            bio: None,
            location: None,
            website: None,
            created_at: now,
            updated_at: now,
            is_verified: false,
            is_private: false,
            settings: UserSettings::default(),
            stats: UserStats::default(),
        }
    }
}

/// User privacy and notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub privacy: PrivacySettings,
    pub notifications: NotificationSettings,
    pub sharing: SharingSettings,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            privacy: PrivacySettings::default(),
            notifications: NotificationSettings::default(),
            sharing: SharingSettings::default(),
        }
    }
}

/// Privacy settings for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: ProfileVisibility,
    pub activity_visibility: ActivityVisibility,
    pub allow_friend_requests: bool,
    pub allow_messages: MessagePermission,
    pub show_listening_history: bool,
    pub show_watchlist: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: ProfileVisibility::Public,
            activity_visibility: ActivityVisibility::Friends,
            allow_friend_requests: true,
            allow_messages: MessagePermission::Everyone,
            show_listening_history: false,
            show_watchlist: false,
        }
    }
}

/// Profile visibility level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileVisibility {
    Public,
    FriendsOnly,
    Private,
}

/// Activity visibility level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityVisibility {
    Public,
    FriendsOnly,
    Private,
}

/// Message permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagePermission {
    Everyone,
    FriendsOnly,
    NoOne,
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub push_enabled: bool,
    pub email_enabled: bool,
    pub friend_request_notifications: bool,
    pub message_notifications: bool,
    pub mention_notifications: bool,
    pub activity_notifications: bool,
    pub marketing_emails: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            push_enabled: true,
            email_enabled: true,
            friend_request_notifications: true,
            message_notifications: true,
            mention_notifications: true,
            activity_notifications: true,
            marketing_emails: false,
        }
    }
}

/// Sharing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingSettings {
    pub auto_share_listening: bool,
    pub auto_share_watchlist: bool,
    pub share_to_connected_accounts: Vec<ConnectedPlatform>,
    pub default_share_message: Option<String>,
}

impl Default for SharingSettings {
    fn default() -> Self {
        Self {
            auto_share_listening: false,
            auto_share_watchlist: false,
            share_to_connected_accounts: Vec::new(),
            default_share_message: None,
        }
    }
}

/// Connected social platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectedPlatform {
    Twitter,
    Facebook,
    Instagram,
    Discord,
    Spotify,
    LastFm,
    YouTube,
}

/// User statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStats {
    pub followers_count: u64,
    pub following_count: u64,
    pub posts_count: u64,
    pub media_shared_count: u64,
    pub playlists_created: u64,
    pub hours_listened: f64,
    pub hours_watched: f64,
}

/// Friendship relationship between users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friendship {
    pub id: Uuid,
    pub requester_id: UserId,
    pub addressee_id: UserId,
    pub status: FriendshipStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Friendship {
    pub fn new(requester_id: UserId, addressee_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            requester_id,
            addressee_id,
            status: FriendshipStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Status of a friendship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Blocked,
    Declined,
}

/// User block relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBlock {
    pub blocker_id: UserId,
    pub blocked_id: UserId,
    pub created_at: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Activity feed post types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub author_id: UserId,
    pub content: PostContent,
    pub visibility: PostVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub likes_count: u64,
    pub comments_count: u64,
    pub shares_count: u64,
    pub is_edited: bool,
    pub tags: Vec<String>,
    pub mentions: Vec<UserId>,
}

impl Post {
    pub fn new(author_id: UserId, content: PostContent) -> Self {
        Self {
            id: PostId::new(),
            author_id,
            content,
            visibility: PostVisibility::Public,
            created_at: Utc::now(),
            updated_at: None,
            likes_count: 0,
            comments_count: 0,
            shares_count: 0,
            is_edited: false,
            tags: Vec::new(),
            mentions: Vec::new(),
        }
    }
}

/// Content of a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostContent {
    pub text: Option<String>,
    pub media_attachment: Option<MediaAttachment>,
    pub link_preview: Option<LinkPreview>,
}

impl PostContent {
    pub fn text(text: String) -> Self {
        Self {
            text: Some(text),
            media_attachment: None,
            link_preview: None,
        }
    }

    pub fn with_media(text: Option<String>, attachment: MediaAttachment) -> Self {
        Self {
            text,
            media_attachment: Some(attachment),
            link_preview: None,
        }
    }
}

/// Media attachment for posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttachment {
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub title: String,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<u64>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub position: Option<MediaPosition>,
}

/// Type of shared media
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedMediaType {
    Song,
    Album,
    Playlist,
    Artist,
    Video,
    Movie,
    TVShow,
    Podcast,
    Audiobook,
}

/// Position in media (for "currently playing" posts)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MediaPosition {
    pub current_seconds: u64,
    pub total_seconds: u64,
}

/// Link preview for URLs in posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkPreview {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub site_name: String,
}

/// Post visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostVisibility {
    Public,
    FriendsOnly,
    Private,
    FollowersOnly,
}

/// Comment on a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: CommentId,
    pub post_id: PostId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub likes_count: u64,
    pub parent_comment_id: Option<CommentId>,
    pub is_edited: bool,
}

impl Comment {
    pub fn new(post_id: PostId, author_id: UserId, content: String) -> Self {
        Self {
            id: CommentId::new(),
            post_id,
            author_id,
            content,
            created_at: Utc::now(),
            updated_at: None,
            likes_count: 0,
            parent_comment_id: None,
            is_edited: false,
        }
    }
}

/// Like on a post or comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Like {
    pub user_id: UserId,
    pub target_type: LikeTarget,
    pub target_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Target type for likes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LikeTarget {
    Post,
    Comment,
}

/// User activity for feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: UserId,
    pub activity_type: ActivityType,
    pub created_at: DateTime<Utc>,
    pub visibility: ActivityVisibility,
    pub details: ActivityDetails,
}

/// Type of user activity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    NowPlaying,
    ListenedTo,
    Watched,
    AddedToPlaylist,
    CreatedPlaylist,
    FollowedUser,
    SharedMedia,
    LikedPost,
    CommentedPost,
    JoinedGroup,
}

/// Details of an activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDetails {
    pub media_id: Option<String>,
    pub media_title: Option<String>,
    pub media_type: Option<SharedMediaType>,
    pub target_user_id: Option<UserId>,
    pub target_post_id: Option<PostId>,
    pub playlist_name: Option<String>,
    pub additional_info: serde_json::Value,
}

/// Notification types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub action_url: Option<String>,
    pub actor_id: Option<UserId>,
    pub image_url: Option<String>,
}

impl Notification {
    pub fn new(user_id: UserId, notification_type: NotificationType, title: String, body: String) -> Self {
        Self {
            id: NotificationId::new(),
            user_id,
            notification_type,
            title,
            body,
            created_at: Utc::now(),
            read_at: None,
            action_url: None,
            actor_id: None,
            image_url: None,
        }
    }
}

/// Type of notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    FriendRequest,
    FriendAccepted,
    NewFollower,
    Mention,
    Like,
    Comment,
    Share,
    Message,
    GroupInvite,
    System,
    Recommendation,
}

/// Direct message conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: ConversationId,
    pub participants: Vec<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_message: Option<Message>,
    pub unread_count: HashMap<UserId, u64>,
    pub is_group: bool,
    pub group_name: Option<String>,
    pub group_avatar: Option<String>,
}

impl Conversation {
    pub fn direct(user1: UserId, user2: UserId) -> Self {
        let now = Utc::now();
        Self {
            id: ConversationId::new(),
            participants: vec![user1, user2],
            created_at: now,
            updated_at: now,
            last_message: None,
            unread_count: HashMap::new(),
            is_group: false,
            group_name: None,
            group_avatar: None,
        }
    }

    pub fn group(creator: UserId, name: String, initial_members: Vec<UserId>) -> Self {
        let now = Utc::now();
        let mut participants = vec![creator];
        participants.extend(initial_members);
        Self {
            id: ConversationId::new(),
            participants,
            created_at: now,
            updated_at: now,
            last_message: None,
            unread_count: HashMap::new(),
            is_group: true,
            group_name: Some(name),
            group_avatar: None,
        }
    }
}

/// Direct message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    pub content: MessageContent,
    pub created_at: DateTime<Utc>,
    pub read_by: Vec<MessageReadReceipt>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Message {
    pub fn new(conversation_id: ConversationId, sender_id: UserId, content: MessageContent) -> Self {
        Self {
            id: MessageId::new(),
            conversation_id,
            sender_id,
            content,
            created_at: Utc::now(),
            read_by: Vec::new(),
            edited_at: None,
            deleted_at: None,
        }
    }
}

/// Content of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    pub text: Option<String>,
    pub media_attachment: Option<MediaAttachment>,
    pub shared_post: Option<PostId>,
    pub shared_media: Option<MediaAttachment>,
}

impl MessageContent {
    pub fn text(text: String) -> Self {
        Self {
            text: Some(text),
            media_attachment: None,
            shared_post: None,
            shared_media: None,
        }
    }
}

/// Read receipt for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReadReceipt {
    pub user_id: UserId,
    pub read_at: DateTime<Utc>,
}

/// User presence/online status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPresence {
    pub user_id: UserId,
    pub status: PresenceStatus,
    pub last_seen: DateTime<Utc>,
    pub current_activity: Option<CurrentActivity>,
    pub custom_status: Option<String>,
}

/// User presence status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
    Invisible,
}

/// Current activity for presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentActivity {
    pub activity_type: ActivityType,
    pub media_title: Option<String>,
    pub media_id: Option<String>,
    pub started_at: DateTime<Utc>,
}

use std::collections::HashMap;

/// Social group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialGroup {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner_id: UserId,
    pub admins: Vec<UserId>,
    pub members: Vec<UserId>,
    pub created_at: DateTime<Utc>,
    pub settings: GroupSettings,
    pub stats: GroupStats,
}

/// Group settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSettings {
    pub is_public: bool,
    pub join_permission: GroupJoinPermission,
    pub post_permission: GroupPostPermission,
    pub allow_media_sharing: bool,
    pub allow_events: bool,
}

/// Group join permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupJoinPermission {
    Open,
    InviteOnly,
    RequestToJoin,
}

/// Group post permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupPostPermission {
    Anyone,
    AdminsOnly,
    MembersOnly,
}

/// Group statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupStats {
    pub members_count: u64,
    pub posts_count: u64,
    pub media_shared_count: u64,
}

/// Activity feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityFeed {
    pub items: Vec<FeedItem>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

/// Item in activity feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: Uuid,
    pub item_type: FeedItemType,
    pub created_at: DateTime<Utc>,
    pub actor: UserProfile,
    pub content: FeedItemContent,
}

/// Type of feed item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedItemType {
    Post,
    Activity,
    Recommendation,
    GroupPost,
}

/// Content of feed item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItemContent {
    pub post: Option<Post>,
    pub activity: Option<UserActivity>,
    pub recommendation: Option<MediaRecommendation>,
}

/// Media recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaRecommendation {
    pub media_id: String,
    pub media_type: SharedMediaType,
    pub title: String,
    pub reason: RecommendationReason,
    pub recommended_by: Option<UserId>,
    pub score: f64,
}

/// Reason for recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationReason {
    BecauseYouListened,
    BecauseYouWatched,
    PopularInNetwork,
    FriendShared,
    Trending,
    NewRelease,
    SimilarToFavorite,
}

/// Share request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRequest {
    pub media_type: SharedMediaType,
    pub media_id: String,
    pub media_title: String,
    pub message: Option<String>,
    pub share_to: ShareTarget,
    pub visibility: PostVisibility,
}

/// Target for sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShareTarget {
    ActivityFeed,
    DirectMessage(UserId),
    Group(Uuid),
    ExternalPlatform(ConnectedPlatform),
}

/// Result of a share operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareResult {
    pub success: bool,
    pub post_id: Option<PostId>,
    pub shared_to: Vec<ShareTarget>,
    pub external_urls: HashMap<ConnectedPlatform, String>,
}