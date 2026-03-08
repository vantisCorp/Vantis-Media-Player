//! Core types for live streaming

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a live stream
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StreamId(pub Uuid);

impl StreamId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for StreamId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a broadcaster
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BroadcasterId(pub Uuid);

impl BroadcasterId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a viewer
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViewerId(pub Uuid);

impl ViewerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a chat message
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChatMessageId(pub Uuid);

impl ChatMessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Live stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStream {
    pub id: StreamId,
    pub broadcaster_id: BroadcasterId,
    pub title: String,
    pub description: Option<String>,
    pub category: StreamCategory,
    pub tags: Vec<String>,
    pub thumbnail_url: Option<String>,
    pub status: StreamStatus,
    pub visibility: StreamVisibility,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub viewer_count: u64,
    pub peak_viewer_count: u64,
    pub total_views: u64,
    pub duration_seconds: u64,
    pub language: String,
    pub is_mature: bool,
    pub config: StreamConfig,
}

impl LiveStream {
    pub fn new(broadcaster_id: BroadcasterId, title: String, category: StreamCategory) -> Self {
        Self {
            id: StreamId::new(),
            broadcaster_id,
            title,
            description: None,
            category,
            tags: Vec::new(),
            thumbnail_url: None,
            status: StreamStatus::Created,
            visibility: StreamVisibility::Public,
            created_at: Utc::now(),
            started_at: None,
            ended_at: None,
            viewer_count: 0,
            peak_viewer_count: 0,
            total_views: 0,
            duration_seconds: 0,
            language: "en".to_string(),
            is_mature: false,
            config: StreamConfig::default(),
        }
    }
}

/// Stream status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamStatus {
    Created,
    Preparing,
    Live,
    Ending,
    Ended,
    Archived,
}

/// Stream visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamVisibility {
    Public,
    Unlisted,
    Private,
    FollowersOnly,
    SubscribersOnly,
}

/// Stream category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StreamCategory {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub tags: Vec<String>,
}

impl StreamCategory {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            parent_id: None,
            tags: Vec::new(),
        }
    }
}

/// Stream configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub video: VideoConfig,
    pub audio: AudioConfig,
    pub latency_mode: LatencyMode,
    pub dvr_enabled: bool,
    pub chat_enabled: bool,
    pub chat_delay_seconds: u32,
    pub max_duration_minutes: Option<u32>,
    pub allow_rewind: bool,
    pub record_vod: bool,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            video: VideoConfig::default(),
            audio: AudioConfig::default(),
            latency_mode: LatencyMode::LowLatency,
            dvr_enabled: true,
            chat_enabled: true,
            chat_delay_seconds: 0,
            max_duration_minutes: None,
            allow_rewind: true,
            record_vod: true,
        }
    }
}

/// Video configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub resolution: Resolution,
    pub frame_rate: u32,
    pub bitrate_kbps: u32,
    pub codec: VideoCodec,
    pub keyframe_interval_seconds: u32,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::FullHD,
            frame_rate: 60,
            bitrate_kbps: 6000,
            codec: VideoCodec::H264,
            keyframe_interval_seconds: 2,
        }
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resolution {
    SD,      // 480p
    HD,      // 720p
    FullHD,  // 1080p
    QHD,     // 1440p
    UHD,     // 4K
    Custom { width: u32, height: u32 },
}

impl Resolution {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Resolution::SD => (854, 480),
            Resolution::HD => (1280, 720),
            Resolution::FullHD => (1920, 1080),
            Resolution::QHD => (2560, 1440),
            Resolution::UHD => (3840, 2160),
            Resolution::Custom { width, height } => (*width, *height),
        }
    }
}

/// Video codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    VP8,
    VP9,
    AV1,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u32,
    pub bitrate_kbps: u32,
    pub codec: AudioCodec,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            channels: 2,
            bitrate_kbps: 192,
            codec: AudioCodec::AAC,
        }
    }
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    AAC,
    MP3,
    Opus,
    FLAC,
}

/// Latency mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatencyMode {
    RealTime,      // Ultra-low latency (~1s)
    LowLatency,    // Low latency (~5s)
    Normal,        // Normal latency (~10-15s)
    HighQuality,   // High quality, higher latency (~30s+)
}

/// Broadcaster profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterProfile {
    pub id: BroadcasterId,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub follower_count: u64,
    pub subscriber_count: u64,
    pub total_views: u64,
    pub is_partner: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub stream_key: Option<String>,
    pub ingestion_endpoint: Option<String>,
    pub settings: BroadcasterSettings,
}

/// Broadcaster settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcasterSettings {
    pub default_title: Option<String>,
    pub default_category: Option<StreamCategory>,
    pub default_tags: Vec<String>,
    pub chat_enabled: bool,
    pub chat_subscribers_only: bool,
    pub chat_followers_only: bool,
    pub chat_min_follow_time_minutes: u32,
    pub slow_mode_enabled: bool,
    pub slow_mode_delay_seconds: u32,
    pub emote_only_mode: bool,
    pub r9k_mode: bool,
}

impl Default for BroadcasterSettings {
    fn default() -> Self {
        Self {
            default_title: None,
            default_category: None,
            default_tags: Vec::new(),
            chat_enabled: true,
            chat_subscribers_only: false,
            chat_followers_only: false,
            chat_min_follow_time_minutes: 0,
            slow_mode_enabled: false,
            slow_mode_delay_seconds: 30,
            emote_only_mode: false,
            r9k_mode: false,
        }
    }
}

/// Viewer profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerProfile {
    pub id: ViewerId,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_following: bool,
    pub is_subscribed: bool,
    pub is_moderator: bool,
    pub is_banned: bool,
    pub is_vip: bool,
    pub badges: Vec<ChatBadge>,
    pub watch_time_minutes: u64,
    pub first_seen: DateTime<Utc>,
}

/// Chat badge
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChatBadge {
    pub id: String,
    pub name: String,
    pub icon_url: String,
}

/// Live chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: ChatMessageId,
    pub stream_id: StreamId,
    pub sender_id: ViewerId,
    pub sender_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub message_type: ChatMessageType,
    pub badges: Vec<ChatBadge>,
    pub emotes: Vec<EmotePosition>,
    pub is_deleted: bool,
    pub deleted_by: Option<String>,
    pub reply_to: Option<ChatMessageId>,
    pub is_highlighted: bool,
    pub color: Option<String>,
}

impl ChatMessage {
    pub fn new(stream_id: StreamId, sender_id: ViewerId, sender_name: String, content: String) -> Self {
        Self {
            id: ChatMessageId::new(),
            stream_id,
            sender_id,
            sender_name,
            content,
            created_at: Utc::now(),
            message_type: ChatMessageType::Text,
            badges: Vec::new(),
            emotes: Vec::new(),
            is_deleted: false,
            deleted_by: None,
            reply_to: None,
            is_highlighted: false,
            color: None,
        }
    }
}

/// Chat message type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatMessageType {
    Text,
    System,
    Announcement,
    Subscription,
    Donation,
    Raid,
    Ban,
    Timeout,
    Clear,
}

/// Emote position in a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotePosition {
    pub emote_id: String,
    pub emote_name: String,
    pub start_index: usize,
    pub end_index: usize,
}

/// Stream viewer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamViewer {
    pub viewer_id: ViewerId,
    pub joined_at: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
    pub watch_duration_seconds: u64,
    pub quality: Resolution,
    pub is_paused: bool,
    pub playback_offset_seconds: i64,
    pub country: Option<String>,
    pub referrer: Option<String>,
}

/// Stream statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamStatistics {
    pub current_viewers: u64,
    pub peak_viewers: u64,
    pub total_unique_viewers: u64,
    pub average_watch_time_seconds: u64,
    pub total_chat_messages: u64,
    pub new_followers: u64,
    pub new_subscribers: u64,
    pub bits_used: u64,
    pub donations_total: f64,
    pub average_bitrate: f64,
    pub dropped_frames: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

/// Ingestion endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionEndpoint {
    pub url: String,
    pub stream_key: String,
    pub server: String,
    pub rtmp_port: u16,
    pub protocol: IngestionProtocol,
    pub backup_servers: Vec<String>,
}

/// Ingestion protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IngestionProtocol {
    RTMP,
    RTMPS,
    SRT,
    RIST,
    WebRTC,
}

/// Playback URL variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackUrls {
    pub hls: Option<String>,
    pub dash: Option<String>,
    pub webrtc: Option<String>,
    pub ll_hls: Option<String>,
    pub qualities: Vec<QualityPlaybackUrl>,
}

/// Quality-specific playback URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPlaybackUrl {
    pub quality: Resolution,
    pub url: String,
    pub bandwidth: u32,
    pub codecs: String,
}

/// Stream event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub event_type: StreamEventType,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

/// Stream event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamEventType {
    StreamStarted,
    StreamStopped,
    StreamCrashed,
    ViewerJoined,
    ViewerLeft,
    ChatMessage,
    ChatCleared,
    UserBanned,
    UserUnbanned,
    UserTimeout,
    NewFollower,
    NewSubscriber,
    Donation,
    RaidIncoming,
    RaidOutgoing,
    PollStarted,
    PollEnded,
    PredictionStarted,
    PredictionEnded,
    HypeTrainStarted,
    HypeTrainEnded,
    GoalProgress,
}

/// Chat moderation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationAction {
    pub action_type: ModerationActionType,
    pub target_user_id: ViewerId,
    pub target_username: String,
    pub moderator_id: ViewerId,
    pub moderator_name: String,
    pub reason: Option<String>,
    pub duration_seconds: Option<u32>,
    pub created_at: DateTime<Utc>,
}

/// Moderation action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModerationActionType {
    Ban,
    Unban,
    Timeout,
    Untimeout,
    Delete,
    SlowMode,
    SlowModeOff,
    FollowersOnly,
    FollowersOnlyOff,
    SubscribersOnly,
    SubscribersOnlyOff,
    EmoteOnly,
    EmoteOnlyOff,
    ClearChat,
    Vip,
    Unvip,
    Mod,
    Unmod,
}

/// Stream schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSchedule {
    pub id: Uuid,
    pub broadcaster_id: BroadcasterId,
    pub title: String,
    pub category: StreamCategory,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_duration_minutes: u32,
    pub is_recurring: bool,
    pub recurrence: Option<RecurrenceRule>,
    pub created_at: DateTime<Utc>,
}

/// Recurrence rule for scheduled streams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    pub frequency: RecurrenceFrequency,
    pub interval: u32,
    pub by_days: Vec<WeekDay>,
    pub until: Option<DateTime<Utc>>,
    pub count: Option<u32>,
}

/// Recurrence frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
}

/// Day of the week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Stream alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAlert {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub alert_type: AlertType,
    pub title: String,
    pub message: String,
    pub image_url: Option<String>,
    pub sound_url: Option<String>,
    pub duration_seconds: u32,
    pub created_at: DateTime<Utc>,
}

/// Alert type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    Follow,
    Subscription,
    Donation,
    Raid,
    Host,
    Bits,
    PointsRedemption,
    GoalReached,
    Custom,
}

/// Poll for viewers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub title: String,
    pub choices: Vec<PollChoice>,
    pub total_votes: u32,
    pub status: PollStatus,
    pub duration_seconds: u32,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Poll choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollChoice {
    pub id: Uuid,
    pub title: String,
    pub votes: u32,
    pub percentage: f32,
}

/// Poll status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollStatus {
    Active,
    Completed,
    Cancelled,
    Archived,
}

/// Prediction (betting on outcomes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub title: String,
    pub outcomes: Vec<PredictionOutcome>,
    pub prediction_window_seconds: u32,
    pub status: PredictionStatus,
    pub total_points: u64,
    pub created_at: DateTime<Utc>,
    pub locked_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Prediction outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutcome {
    pub id: Uuid,
    pub title: String,
    pub users: u32,
    pub total_points: u64,
    pub percentage: f32,
    pub is_winner: bool,
}

/// Prediction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionStatus {
    Active,
    Locked,
    Resolved,
    Cancelled,
}

/// Hype train (progressive celebration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrain {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub level: u32,
    pub max_level: u32,
    pub progress: f32,
    pub goal: u32,
    pub total: u32,
    pub top_contributors: Vec<Contributor>,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}

/// Contributor to hype train
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    pub user_id: ViewerId,
    pub username: String,
    pub total: u32,
    pub contribution_type: ContributionType,
}

/// Type of contribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionType {
    Bits,
    Subscription,
    Donation,
    GiftSub,
}

/// Stream goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGoal {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub goal_type: GoalType,
    pub current_amount: u64,
    pub target_amount: u64,
    pub title: String,
    pub description: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub is_reached: bool,
}

/// Type of stream goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType {
    Followers,
    Subscribers,
    Bits,
    Donations,
    ViewerTime,
    NewViewers,
    Custom,
}

/// Clip from a live stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamClip {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub creator_id: ViewerId,
    pub title: String,
    pub duration_seconds: u32,
    pub view_count: u64,
    pub created_at: DateTime<Utc>,
    pub thumbnail_url: String,
    pub video_url: String,
    pub start_offset: u64,
    pub end_offset: u64,
}

/// Video on Demand (VOD) from past stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamVod {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub broadcaster_id: BroadcasterId,
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: u64,
    pub view_count: u64,
    pub created_at: DateTime<Utc>,
    pub thumbnail_url: String,
    pub playback_urls: PlaybackUrls,
    pub muted_segments: Vec<MutedSegment>,
    pub is_processing: bool,
}

/// Muted segment in VOD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutedSegment {
    pub start_offset: u64,
    pub duration_seconds: u64,
    pub reason: String,
}

/// Raid information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raid {
    pub id: Uuid,
    pub from_broadcaster_id: BroadcasterId,
    pub to_broadcaster_id: BroadcasterId,
    pub viewer_count: u32,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

/// Chat emote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmote {
    pub id: String,
    pub name: String,
    pub images: EmoteImages,
    pub tier: Option<String>,
    pub emote_type: EmoteType,
    pub is_animated: bool,
}

/// Emote image variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmoteImages {
    pub url_1x: String,
    pub url_2x: String,
    pub url_3x: String,
}

/// Emote type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmoteType {
    Global,
    Subscriber,
    Custom,
    Follower,
    BitsTier,
    Smilies,
    ChannelPoints,
}

/// Viewer watch party
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchParty {
    pub id: Uuid,
    pub stream_id: StreamId,
    pub host_id: ViewerId,
    pub participants: Vec<WatchPartyParticipant>,
    pub started_at: DateTime<Utc>,
    pub status: WatchPartyStatus,
}

/// Watch party participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPartyParticipant {
    pub viewer_id: ViewerId,
    pub username: String,
    pub joined_at: DateTime<Utc>,
    pub is_muted: bool,
    pub has_video: bool,
}

/// Watch party status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WatchPartyStatus {
    Waiting,
    Active,
    Paused,
    Ended,
}