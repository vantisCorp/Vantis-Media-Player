//! Activity feed and posts service

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;
use crate::user_service::UserService;

/// Trait for managing posts and activity feed
#[async_trait]
pub trait FeedService: Send + Sync {
    /// Create a new post
    async fn create_post(&self, post: CreatePostRequest) -> SocialResult<Post>;

    /// Get a post by ID
    async fn get_post(&self, post_id: &PostId) -> SocialResult<Post>;

    /// Update a post
    async fn update_post(&self, post_id: &PostId, author_id: &UserId, content: PostContent) -> SocialResult<Post>;

    /// Delete a post
    async fn delete_post(&self, post_id: &PostId, author_id: &UserId) -> SocialResult<()>;

    /// Get posts by user
    async fn get_user_posts(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Post>>;

    /// Get activity feed for a user
    async fn get_feed(&self, user_id: &UserId, limit: usize, cursor: Option<String>) -> SocialResult<ActivityFeed>;

    /// Like a post
    async fn like_post(&self, user_id: &UserId, post_id: &PostId) -> SocialResult<()>;

    /// Unlike a post
    async fn unlike_post(&self, user_id: &UserId, post_id: &PostId) -> SocialResult<()>;

    /// Get post likes
    async fn get_post_likes(&self, post_id: &PostId, limit: usize, offset: usize) -> SocialResult<Vec<UserId>>;

    /// Share a post
    async fn share_post(&self, user_id: &UserId, post_id: &PostId, message: Option<String>) -> SocialResult<Post>;

    /// Add comment to a post
    async fn add_comment(&self, comment: CreateCommentRequest) -> SocialResult<Comment>;

    /// Get comments for a post
    async fn get_comments(&self, post_id: &PostId, limit: usize, offset: usize) -> SocialResult<Vec<Comment>>;

    /// Delete a comment
    async fn delete_comment(&self, comment_id: &CommentId, author_id: &UserId) -> SocialResult<()>;

    /// Like a comment
    async fn like_comment(&self, user_id: &UserId, comment_id: &CommentId) -> SocialResult<()>;

    /// Unlike a comment
    async fn unlike_comment(&self, user_id: &UserId, comment_id: &CommentId) -> SocialResult<()>;

    /// Record user activity
    async fn record_activity(&self, activity: UserActivity) -> SocialResult<()>;

    /// Get user activities
    async fn get_user_activities(&self, user_id: &UserId, limit: usize) -> SocialResult<Vec<UserActivity>>;
}

/// Request to create a new post
#[derive(Debug, Clone)]
pub struct CreatePostRequest {
    pub author_id: UserId,
    pub content: PostContent,
    pub visibility: PostVisibility,
    pub tags: Vec<String>,
    pub mentions: Vec<UserId>,
}

/// Request to create a comment
#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    pub post_id: PostId,
    pub author_id: UserId,
    pub content: String,
    pub parent_comment_id: Option<CommentId>,
}

/// Default in-memory implementation of FeedService
pub struct DefaultFeedService {
    posts: DashMap<PostId, Post>,
    comments: DashMap<CommentId, Comment>,
    likes: DashMap<(Uuid, UserId), Like>,
    activities: DashMap<Uuid, UserActivity>,
    user_service: Arc<dyn UserService>,
    // Feed cache per user (simplified)
    feed_cache: RwLock<HashMap<UserId, Vec<PostId>>>,
}

impl DefaultFeedService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self {
            posts: DashMap::new(),
            comments: DashMap::new(),
            likes: DashMap::new(),
            activities: DashMap::new(),
            user_service,
            feed_cache: RwLock::new(HashMap::new()),
        }
    }

    fn can_view_post(&self, post: &Post, viewer_id: &UserId) -> bool {
        match post.visibility {
            PostVisibility::Public => true,
            PostVisibility::Private => &post.author_id == viewer_id,
            PostVisibility::FriendsOnly | PostVisibility::FollowersOnly => {
                // Simplified - in real implementation would check friendship
                &post.author_id == viewer_id
            }
        }
    }
}

#[async_trait]
impl FeedService for DefaultFeedService {
    async fn create_post(&self, post: CreatePostRequest) -> SocialResult<Post> {
        // Verify user exists
        self.user_service.get_profile(&post.author_id).await?;

        let mut new_post = Post::new(post.author_id.clone(), post.content);
        new_post.visibility = post.visibility;
        new_post.tags = post.tags;
        new_post.mentions = post.mentions;
        new_post.created_at = Utc::now();

        let post_id = new_post.id.clone();
        self.posts.insert(post_id.clone(), new_post.clone());

        // Update user stats
        self.user_service.update_user_stats(&post.author_id, crate::user_service::StatsUpdate {
            posts_delta: 1,
            ..Default::default()
        }).await.ok();

        // Record activity
        let activity = UserActivity {
            id: Uuid::new_v4(),
            user_id: post.author_id,
            activity_type: ActivityType::SharedMedia,
            created_at: Utc::now(),
            visibility: ActivityVisibility::Public,
            details: ActivityDetails {
                media_id: None,
                media_title: None,
                media_type: None,
                target_user_id: None,
                target_post_id: Some(post_id),
                playlist_name: None,
                additional_info: serde_json::json!({}),
            },
        };
        self.activities.insert(activity.id, activity);

        Ok(new_post)
    }

    async fn get_post(&self, post_id: &PostId) -> SocialResult<Post> {
        self.posts
            .get(post_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::PostNotFound(post_id.0.to_string()))
    }

    async fn update_post(&self, post_id: &PostId, author_id: &UserId, content: PostContent) -> SocialResult<Post> {
        let mut post = self.get_post(post_id).await?;

        if &post.author_id != author_id {
            return Err(SocialError::Unauthorized);
        }

        post.content = content;
        post.is_edited = true;
        post.updated_at = Some(Utc::now());

        self.posts.insert(post_id.clone(), post.clone());
        Ok(post)
    }

    async fn delete_post(&self, post_id: &PostId, author_id: &UserId) -> SocialResult<()> {
        let post = self.get_post(post_id).await?;

        if &post.author_id != author_id {
            return Err(SocialError::Unauthorized);
        }

        self.posts.remove(post_id);

        // Remove associated comments
        let comment_ids: Vec<CommentId> = self.comments
            .iter()
            .filter(|e| e.value().post_id == *post_id)
            .map(|e| e.key().clone())
            .collect();

        for id in comment_ids {
            self.comments.remove(&id);
        }

        // Remove associated likes
        let like_keys: Vec<(Uuid, UserId)> = self.likes
            .iter()
            .filter(|e| e.key().0 == post_id.0)
            .map(|e| e.key().clone())
            .collect();

        for key in like_keys {
            self.likes.remove(&key);
        }

        // Update user stats
        self.user_service.update_user_stats(author_id, crate::user_service::StatsUpdate {
            posts_delta: -1,
            ..Default::default()
        }).await.ok();

        Ok(())
    }

    async fn get_user_posts(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Post>> {
        let mut posts: Vec<Post> = self.posts
            .iter()
            .filter(|e| &e.value().author_id == user_id)
            .map(|e| e.value().clone())
            .collect();

        // Sort by created_at descending
        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        posts = posts.into_iter().skip(offset).take(limit).collect();

        Ok(posts)
    }

    async fn get_feed(&self, user_id: &UserId, limit: usize, cursor: Option<String>) -> SocialResult<ActivityFeed> {
        // Verify user exists
        self.user_service.get_profile(user_id).await?;

        // Get all visible posts (simplified - should include friends' posts)
        let mut feed_items: Vec<FeedItem> = Vec::new();

        for entry in self.posts.iter() {
            let post = entry.value();
            if self.can_view_post(post, user_id) {
                if let Ok(author) = self.user_service.get_profile(&post.author_id).await {
                    feed_items.push(FeedItem {
                        id: Uuid::new_v4(),
                        item_type: FeedItemType::Post,
                        created_at: post.created_at,
                        actor: author,
                        content: FeedItemContent {
                            post: Some(post.clone()),
                            activity: None,
                            recommendation: None,
                        },
                    });
                }
            }
        }

        // Sort by created_at descending
        feed_items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Handle cursor pagination
        if let Some(cursor_str) = cursor {
            if let Ok(cursor_time) = cursor_str.parse::<i64>() {
                feed_items = feed_items
                    .into_iter()
                    .filter(|item| item.created_at.timestamp() < cursor_time)
                    .collect();
            }
        }

        let has_more = feed_items.len() > limit;
        let items: Vec<FeedItem> = feed_items.into_iter().take(limit).collect();

        let next_cursor = if has_more && !items.is_empty() {
            Some(items.last().unwrap().created_at.timestamp().to_string())
        } else {
            None
        };

        Ok(ActivityFeed {
            items,
            next_cursor,
            has_more,
        })
    }

    async fn like_post(&self, user_id: &UserId, post_id: &PostId) -> SocialResult<()> {
        // Verify user and post exist
        self.user_service.get_profile(user_id).await?;
        let post = self.get_post(post_id).await?;

        let key = (post_id.0, user_id.clone());
        if self.likes.contains_key(&key) {
            return Err(SocialError::InvalidOperation("Already liked".to_string()));
        }

        let like = Like {
            user_id: user_id.clone(),
            target_type: LikeTarget::Post,
            target_id: post_id.0,
            created_at: Utc::now(),
        };

        self.likes.insert(key, like);

        // Update post likes count
        let mut updated_post = post;
        updated_post.likes_count += 1;
        self.posts.insert(post_id.clone(), updated_post);

        Ok(())
    }

    async fn unlike_post(&self, user_id: &UserId, post_id: &PostId) -> SocialResult<()> {
        let key = (post_id.0, user_id.clone());
        if self.likes.remove(&key).is_none() {
            return Err(SocialError::InvalidOperation("Not liked".to_string()));
        }

        // Update post likes count
        if let Some(mut post) = self.posts.get(post_id).map(|r| r.clone()) {
            post.likes_count = post.likes_count.saturating_sub(1);
            self.posts.insert(post_id.clone(), post);
        }

        Ok(())
    }

    async fn get_post_likes(&self, post_id: &PostId, limit: usize, offset: usize) -> SocialResult<Vec<UserId>> {
        let likes: Vec<UserId> = self.likes
            .iter()
            .filter(|e| e.key().0 == post_id.0 && e.value().target_type == LikeTarget::Post)
            .map(|e| e.value().user_id.clone())
            .skip(offset)
            .take(limit)
            .collect();

        Ok(likes)
    }

    async fn share_post(&self, user_id: &UserId, post_id: &PostId, message: Option<String>) -> SocialResult<Post> {
        // Verify user and post exist
        self.user_service.get_profile(user_id).await?;
        let original_post = self.get_post(post_id).await?;

        // Create new post with shared content
        let content = PostContent {
            text: message,
            media_attachment: original_post.content.media_attachment.clone(),
            link_preview: original_post.content.link_preview.clone(),
        };

        let shared_post = CreatePostRequest {
            author_id: user_id.clone(),
            content,
            visibility: PostVisibility::Public,
            tags: original_post.tags.clone(),
            mentions: vec![],
        };

        let new_post = self.create_post(shared_post).await?;

        // Update original post shares count
        let mut updated_original = original_post;
        updated_original.shares_count += 1;
        self.posts.insert(post_id.clone(), updated_original);

        Ok(new_post)
    }

    async fn add_comment(&self, comment: CreateCommentRequest) -> SocialResult<Comment> {
        // Verify user and post exist
        self.user_service.get_profile(&comment.author_id).await?;
        self.get_post(&comment.post_id).await?;

        // Validate content length
        if comment.content.len() > 2000 {
            return Err(SocialError::ContentTooLong { max: 2000 });
        }

        let mut new_comment = Comment::new(
            comment.post_id.clone(),
            comment.author_id.clone(),
            comment.content,
        );

        if let Some(parent_id) = comment.parent_comment_id {
            // Verify parent comment exists
            if self.comments.get(&parent_id).is_none() {
                return Err(SocialError::CommentNotFound(parent_id.0.to_string()));
            }
            new_comment.parent_comment_id = Some(parent_id);
        }

        let comment_id = new_comment.id.clone();
        self.comments.insert(comment_id, new_comment.clone());

        // Update post comments count
        if let Some(mut post) = self.posts.get(&comment.post_id).map(|r| r.clone()) {
            post.comments_count += 1;
            self.posts.insert(comment.post_id.clone(), post);
        }

        Ok(new_comment)
    }

    async fn get_comments(&self, post_id: &PostId, limit: usize, offset: usize) -> SocialResult<Vec<Comment>> {
        let mut comments: Vec<Comment> = self.comments
            .iter()
            .filter(|e| &e.value().post_id == post_id)
            .map(|e| e.value().clone())
            .collect();

        // Sort by created_at ascending (oldest first for comments)
        comments.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        comments = comments.into_iter().skip(offset).take(limit).collect();

        Ok(comments)
    }

    async fn delete_comment(&self, comment_id: &CommentId, author_id: &UserId) -> SocialResult<()> {
        let comment = self.comments
            .get(comment_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::CommentNotFound(comment_id.0.to_string()))?;

        if &comment.author_id != author_id {
            return Err(SocialError::Unauthorized);
        }

        self.comments.remove(comment_id);

        // Update post comments count
        if let Some(mut post) = self.posts.get(&comment.post_id).map(|r| r.clone()) {
            post.comments_count = post.comments_count.saturating_sub(1);
            self.posts.insert(comment.post_id.clone(), post);
        }

        Ok(())
    }

    async fn like_comment(&self, user_id: &UserId, comment_id: &CommentId) -> SocialResult<()> {
        // Verify user and comment exist
        self.user_service.get_profile(user_id).await?;
        let comment = self.comments
            .get(comment_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::CommentNotFound(comment_id.0.to_string()))?;

        let key = (comment_id.0, user_id.clone());
        if self.likes.contains_key(&key) {
            return Err(SocialError::InvalidOperation("Already liked".to_string()));
        }

        let like = Like {
            user_id: user_id.clone(),
            target_type: LikeTarget::Comment,
            target_id: comment_id.0,
            created_at: Utc::now(),
        };

        self.likes.insert(key, like);

        // Update comment likes count
        let mut updated_comment = comment;
        updated_comment.likes_count += 1;
        self.comments.insert(comment_id.clone(), updated_comment);

        Ok(())
    }

    async fn unlike_comment(&self, user_id: &UserId, comment_id: &CommentId) -> SocialResult<()> {
        let key = (comment_id.0, user_id.clone());
        if self.likes.remove(&key).is_none() {
            return Err(SocialError::InvalidOperation("Not liked".to_string()));
        }

        // Update comment likes count
        if let Some(mut comment) = self.comments.get(comment_id).map(|r| r.clone()) {
            comment.likes_count = comment.likes_count.saturating_sub(1);
            self.comments.insert(comment_id.clone(), comment);
        }

        Ok(())
    }

    async fn record_activity(&self, activity: UserActivity) -> SocialResult<()> {
        // Verify user exists
        self.user_service.get_profile(&activity.user_id).await?;

        let activity_id = activity.id;
        self.activities.insert(activity_id, activity);
        Ok(())
    }

    async fn get_user_activities(&self, user_id: &UserId, limit: usize) -> SocialResult<Vec<UserActivity>> {
        let mut activities: Vec<UserActivity> = self.activities
            .iter()
            .filter(|e| &e.value().user_id == user_id)
            .map(|e| e.value().clone())
            .collect();

        // Sort by created_at descending
        activities.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        activities = activities.into_iter().take(limit).collect();

        Ok(activities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::DefaultUserService;

    #[tokio::test]
    async fn test_create_post() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let feed_service = DefaultFeedService::new(user_service.clone());

        let post = feed_service.create_post(CreatePostRequest {
            author_id: user.id.clone(),
            content: PostContent::text("Hello, world!".to_string()),
            visibility: PostVisibility::Public,
            tags: vec![],
            mentions: vec![],
        }).await.unwrap();

        assert_eq!(post.content.text, Some("Hello, world!".to_string()));
    }

    #[tokio::test]
    async fn test_like_post() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let feed_service = DefaultFeedService::new(user_service.clone());

        let post = feed_service.create_post(CreatePostRequest {
            author_id: user.id.clone(),
            content: PostContent::text("Test post".to_string()),
            visibility: PostVisibility::Public,
            tags: vec![],
            mentions: vec![],
        }).await.unwrap();

        // Create another user to like
        let liker = user_service.create_profile("liker".to_string(), "Liker".to_string()).await.unwrap();

        feed_service.like_post(&liker.id, &post.id).await.unwrap();

        let updated_post = feed_service.get_post(&post.id).await.unwrap();
        assert_eq!(updated_post.likes_count, 1);
    }

    #[tokio::test]
    async fn test_comments() {
        let user_service = Arc::new(DefaultUserService::new());
        let user = user_service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();

        let feed_service = DefaultFeedService::new(user_service.clone());

        let post = feed_service.create_post(CreatePostRequest {
            author_id: user.id.clone(),
            content: PostContent::text("Test post".to_string()),
            visibility: PostVisibility::Public,
            tags: vec![],
            mentions: vec![],
        }).await.unwrap();

        let comment = feed_service.add_comment(CreateCommentRequest {
            post_id: post.id.clone(),
            author_id: user.id.clone(),
            content: "Great post!".to_string(),
            parent_comment_id: None,
        }).await.unwrap();

        assert_eq!(comment.content, "Great post!");

        let comments = feed_service.get_comments(&post.id, 10, 0).await.unwrap();
        assert_eq!(comments.len(), 1);
    }
}