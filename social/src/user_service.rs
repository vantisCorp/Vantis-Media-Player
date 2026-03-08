//! User profile service for managing user profiles and relationships

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;

/// Trait for user profile management
#[async_trait]
pub trait UserService: Send + Sync {
    /// Create a new user profile
    async fn create_profile(&self, username: String, display_name: String) -> SocialResult<UserProfile>;

    /// Get a user profile by ID
    async fn get_profile(&self, user_id: &UserId) -> SocialResult<UserProfile>;

    /// Get a user profile by username
    async fn get_profile_by_username(&self, username: &str) -> SocialResult<UserProfile>;

    /// Update a user profile
    async fn update_profile(&self, user_id: &UserId, updates: ProfileUpdate) -> SocialResult<UserProfile>;

    /// Update user settings
    async fn update_settings(&self, user_id: &UserId, settings: UserSettings) -> SocialResult<()>;

    /// Delete a user profile
    async fn delete_profile(&self, user_id: &UserId) -> SocialResult<()>;

    /// Search for users
    async fn search_users(&self, query: &str, limit: usize) -> SocialResult<Vec<UserProfile>>;

    /// Get user stats
    async fn get_user_stats(&self, user_id: &UserId) -> SocialResult<UserStats>;

    /// Update user stats
    async fn update_user_stats(&self, user_id: &UserId, stats_update: StatsUpdate) -> SocialResult<()>;
}

/// Updates for a user profile
#[derive(Debug, Clone, Default)]
pub struct ProfileUpdate {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub is_private: Option<bool>,
}

/// Updates for user stats
#[derive(Debug, Clone, Default)]
pub struct StatsUpdate {
    pub followers_delta: i64,
    pub following_delta: i64,
    pub posts_delta: i64,
    pub media_shared_delta: i64,
    pub playlists_delta: i64,
    pub hours_listened_delta: f64,
    pub hours_watched_delta: f64,
}

/// Default in-memory implementation of UserService
pub struct DefaultUserService {
    profiles: DashMap<UserId, UserProfile>,
    username_index: RwLock<std::collections::HashMap<String, UserId>>,
}

impl DefaultUserService {
    pub fn new() -> Self {
        Self {
            profiles: DashMap::new(),
            username_index: RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for DefaultUserService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserService for DefaultUserService {
    async fn create_profile(&self, username: String, display_name: String) -> SocialResult<UserProfile> {
        // Check if username already exists
        {
            let index = self.username_index.read();
            if index.contains_key(&username) {
                return Err(SocialError::ValidationError("Username already taken".to_string()));
            }
        }

        let profile = UserProfile::new(username.clone(), display_name);
        let user_id = profile.id.clone();

        // Add to username index
        {
            let mut index = self.username_index.write();
            index.insert(username, user_id.clone());
        }

        // Add to profiles
        self.profiles.insert(user_id, profile.clone());

        Ok(profile)
    }

    async fn get_profile(&self, user_id: &UserId) -> SocialResult<UserProfile> {
        self.profiles
            .get(user_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::UserNotFound(user_id.0.to_string()))
    }

    async fn get_profile_by_username(&self, username: &str) -> SocialResult<UserProfile> {
        let user_id = {
            let index = self.username_index.read();
            index.get(username).cloned()
        };

        match user_id {
            Some(id) => self.get_profile(&id).await,
            None => Err(SocialError::UserNotFound(username.to_string())),
        }
    }

    async fn update_profile(&self, user_id: &UserId, updates: ProfileUpdate) -> SocialResult<UserProfile> {
        let mut profile = self.get_profile(user_id).await?;

        if let Some(display_name) = updates.display_name {
            profile.display_name = display_name;
        }
        if let Some(avatar_url) = updates.avatar_url {
            profile.avatar_url = Some(avatar_url);
        }
        if let Some(banner_url) = updates.banner_url {
            profile.banner_url = Some(banner_url);
        }
        if let Some(bio) = updates.bio {
            profile.bio = Some(bio);
        }
        if let Some(location) = updates.location {
            profile.location = Some(location);
        }
        if let Some(website) = updates.website {
            profile.website = Some(website);
        }
        if let Some(is_private) = updates.is_private {
            profile.is_private = is_private;
        }

        profile.updated_at = Utc::now();
        self.profiles.insert(user_id.clone(), profile.clone());

        Ok(profile)
    }

    async fn update_settings(&self, user_id: &UserId, settings: UserSettings) -> SocialResult<()> {
        let mut profile = self.get_profile(user_id).await?;
        profile.settings = settings;
        profile.updated_at = Utc::now();
        self.profiles.insert(user_id.clone(), profile);
        Ok(())
    }

    async fn delete_profile(&self, user_id: &UserId) -> SocialResult<()> {
        // Get profile to find username
        if let Some((_, profile)) = self.profiles.remove(user_id) {
            // Remove from username index
            let mut index = self.username_index.write();
            index.remove(&profile.username);
        }
        Ok(())
    }

    async fn search_users(&self, query: &str, limit: usize) -> SocialResult<Vec<UserProfile>> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<UserProfile> = Vec::new();

        for entry in self.profiles.iter() {
            let profile = entry.value();
            if profile.username.to_lowercase().contains(&query_lower)
                || profile.display_name.to_lowercase().contains(&query_lower)
            {
                results.push(profile.clone());
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn get_user_stats(&self, user_id: &UserId) -> SocialResult<UserStats> {
        let profile = self.get_profile(user_id).await?;
        Ok(profile.stats)
    }

    async fn update_user_stats(&self, user_id: &UserId, stats_update: StatsUpdate) -> SocialResult<()> {
        let mut profile = self.get_profile(user_id).await?;

        // Apply deltas
        profile.stats.followers_count = (profile.stats.followers_count as i64 + stats_update.followers_delta).max(0) as u64;
        profile.stats.following_count = (profile.stats.following_count as i64 + stats_update.following_delta).max(0) as u64;
        profile.stats.posts_count = (profile.stats.posts_count as i64 + stats_update.posts_delta).max(0) as u64;
        profile.stats.media_shared_count = (profile.stats.media_shared_count as i64 + stats_update.media_shared_delta).max(0) as u64;
        profile.stats.playlists_created = (profile.stats.playlists_created as i64 + stats_update.playlists_delta).max(0) as u64;
        profile.stats.hours_listened = (profile.stats.hours_listened + stats_update.hours_listened_delta).max(0.0);
        profile.stats.hours_watched = (profile.stats.hours_watched + stats_update.hours_watched_delta).max(0.0);

        self.profiles.insert(user_id.clone(), profile);
        Ok(())
    }
}

/// Trait for friend relationship management
#[async_trait]
pub trait FriendService: Send + Sync {
    /// Send a friend request
    async fn send_friend_request(&self, requester_id: &UserId, addressee_id: &UserId) -> SocialResult<Friendship>;

    /// Accept a friend request
    async fn accept_friend_request(&self, friendship_id: &Uuid, acceptor_id: &UserId) -> SocialResult<Friendship>;

    /// Decline a friend request
    async fn decline_friend_request(&self, friendship_id: &Uuid) -> SocialResult<()>;

    /// Cancel a friend request
    async fn cancel_friend_request(&self, friendship_id: &Uuid, requester_id: &UserId) -> SocialResult<()>;

    /// Remove a friend
    async fn remove_friend(&self, user_id: &UserId, friend_id: &UserId) -> SocialResult<()>;

    /// Block a user
    async fn block_user(&self, blocker_id: &UserId, blocked_id: &UserId, reason: Option<String>) -> SocialResult<UserBlock>;

    /// Unblock a user
    async fn unblock_user(&self, blocker_id: &UserId, blocked_id: &UserId) -> SocialResult<()>;

    /// Get friends list
    async fn get_friends(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<UserProfile>>;

    /// Get pending friend requests
    async fn get_pending_requests(&self, user_id: &UserId) -> SocialResult<Vec<Friendship>>;

    /// Get sent friend requests
    async fn get_sent_requests(&self, user_id: &UserId) -> SocialResult<Vec<Friendship>>;

    /// Check friendship status
    async fn get_friendship_status(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<Option<FriendshipStatus>>;

    /// Check if user is blocked
    async fn is_user_blocked(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<bool>;

    /// Get blocked users
    async fn get_blocked_users(&self, user_id: &UserId) -> SocialResult<Vec<UserBlock>>;

    /// Get mutual friends
    async fn get_mutual_friends(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<Vec<UserProfile>>;
}

/// Default in-memory implementation of FriendService
pub struct DefaultFriendService {
    friendships: DashMap<Uuid, Friendship>,
    blocks: DashMap<(UserId, UserId), UserBlock>,
    user_service: Arc<dyn UserService>,
}

impl DefaultFriendService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self {
            friendships: DashMap::new(),
            blocks: DashMap::new(),
            user_service,
        }
    }

    fn make_key(user1: &UserId, user2: &UserId) -> (UserId, UserId) {
        // Use consistent ordering for the key
        if user1.0 < user2.0 {
            (user1.clone(), user2.clone())
        } else {
            (user2.clone(), user1.clone())
        }
    }

    fn find_friendship(&self, user1: &UserId, user2: &UserId) -> Option<(Uuid, Friendship)> {
        for entry in self.friendships.iter() {
            let friendship = entry.value();
            if (&friendship.requester_id == user1 && &friendship.addressee_id == user2)
                || (&friendship.requester_id == user2 && &friendship.addressee_id == user1)
            {
                return Some((entry.key().clone(), friendship.clone()));
            }
        }
        None
    }
}

#[async_trait]
impl FriendService for DefaultFriendService {
    async fn send_friend_request(&self, requester_id: &UserId, addressee_id: &UserId) -> SocialResult<Friendship> {
        // Check if users exist
        self.user_service.get_profile(requester_id).await?;
        self.user_service.get_profile(addressee_id).await?;

        // Cannot friend yourself
        if requester_id == addressee_id {
            return Err(SocialError::SelfFriendship);
        }

        // Check if blocked
        if self.is_user_blocked(addressee_id, requester_id).await? {
            return Err(SocialError::YouAreBlocked);
        }
        if self.is_user_blocked(requester_id, addressee_id).await? {
            return Err(SocialError::UserBlocked);
        }

        // Check if friendship already exists
        if self.find_friendship(requester_id, addressee_id).is_some() {
            return Err(SocialError::FriendshipAlreadyExists);
        }

        // Check privacy settings
        let addressee = self.user_service.get_profile(addressee_id).await?;
        if !addressee.settings.privacy.allow_friend_requests {
            return Err(SocialError::PrivacyViolation);
        }

        let friendship = Friendship::new(requester_id.clone(), addressee_id.clone());
        self.friendships.insert(friendship.id, friendship.clone());

        // Update stats
        self.user_service.update_user_stats(requester_id, StatsUpdate {
            following_delta: 1,
            ..Default::default()
        }).await.ok();

        Ok(friendship)
    }

    async fn accept_friend_request(&self, friendship_id: &Uuid, acceptor_id: &UserId) -> SocialResult<Friendship> {
        let mut friendship = self.friendships.get(friendship_id)
            .map(|r| r.clone())
            .ok_or(SocialError::FriendshipNotFound)?;

        // Verify the acceptor is the addressee
        if &friendship.addressee_id != acceptor_id {
            return Err(SocialError::Unauthorized);
        }

        // Check status
        if friendship.status != FriendshipStatus::Pending {
            return Err(SocialError::InvalidOperation("Friend request is not pending".to_string()));
        }

        friendship.status = FriendshipStatus::Accepted;
        friendship.updated_at = Utc::now();
        self.friendships.insert(*friendship_id, friendship.clone());

        // Update stats for both users
        self.user_service.update_user_stats(acceptor_id, StatsUpdate {
            following_delta: 1,
            ..Default::default()
        }).await.ok();

        self.user_service.update_user_stats(&friendship.requester_id, StatsUpdate {
            followers_delta: 1,
            ..Default::default()
        }).await.ok();

        self.user_service.update_user_stats(acceptor_id, StatsUpdate {
            followers_delta: 1,
            ..Default::default()
        }).await.ok();

        Ok(friendship)
    }

    async fn decline_friend_request(&self, friendship_id: &Uuid) -> SocialResult<()> {
        let friendship = self.friendships.get(friendship_id)
            .map(|r| r.clone())
            .ok_or(SocialError::FriendshipNotFound)?;

        if friendship.status != FriendshipStatus::Pending {
            return Err(SocialError::InvalidOperation("Friend request is not pending".to_string()));
        }

        self.friendships.remove(friendship_id);
        Ok(())
    }

    async fn cancel_friend_request(&self, friendship_id: &Uuid, requester_id: &UserId) -> SocialResult<()> {
        let friendship = self.friendships.get(friendship_id)
            .map(|r| r.clone())
            .ok_or(SocialError::FriendshipNotFound)?;

        if &friendship.requester_id != requester_id {
            return Err(SocialError::Unauthorized);
        }

        if friendship.status != FriendshipStatus::Pending {
            return Err(SocialError::InvalidOperation("Friend request is not pending".to_string()));
        }

        self.friendships.remove(friendship_id);

        // Update stats
        self.user_service.update_user_stats(requester_id, StatsUpdate {
            following_delta: -1,
            ..Default::default()
        }).await.ok();

        Ok(())
    }

    async fn remove_friend(&self, user_id: &UserId, friend_id: &UserId) -> SocialResult<()> {
        if let Some((id, _)) = self.find_friendship(user_id, friend_id) {
            self.friendships.remove(&id);

            // Update stats
            self.user_service.update_user_stats(user_id, StatsUpdate {
                following_delta: -1,
                followers_delta: -1,
                ..Default::default()
            }).await.ok();

            self.user_service.update_user_stats(friend_id, StatsUpdate {
                following_delta: -1,
                followers_delta: -1,
                ..Default::default()
            }).await.ok();

            Ok(())
        } else {
            Err(SocialError::FriendshipNotFound)
        }
    }

    async fn block_user(&self, blocker_id: &UserId, blocked_id: &UserId, reason: Option<String>) -> SocialResult<UserBlock> {
        // Check if users exist
        self.user_service.get_profile(blocker_id).await?;
        self.user_service.get_profile(blocked_id).await?;

        // Cannot block yourself
        if blocker_id == blocked_id {
            return Err(SocialError::InvalidOperation("Cannot block yourself".to_string()));
        }

        let block = UserBlock {
            blocker_id: blocker_id.clone(),
            blocked_id: blocked_id.clone(),
            created_at: Utc::now(),
            reason,
        };

        // Remove any existing friendship
        if let Some((id, _)) = self.find_friendship(blocker_id, blocked_id) {
            self.friendships.remove(&id);
        }

        self.blocks.insert((blocker_id.clone(), blocked_id.clone()), block.clone());
        Ok(block)
    }

    async fn unblock_user(&self, blocker_id: &UserId, blocked_id: &UserId) -> SocialResult<()> {
        self.blocks.remove(&(blocker_id.clone(), blocked_id.clone()));
        Ok(())
    }

    async fn get_friends(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<UserProfile>> {
        let mut friend_ids: Vec<UserId> = Vec::new();

        for entry in self.friendships.iter() {
            let friendship = entry.value();
            if friendship.status == FriendshipStatus::Accepted {
                if &friendship.requester_id == user_id {
                    friend_ids.push(friendship.addressee_id.clone());
                } else if &friendship.addressee_id == user_id {
                    friend_ids.push(friendship.requester_id.clone());
                }
            }
        }

        let result: Vec<UserProfile> = friend_ids.into_iter().skip(offset).take(limit).collect();

        let mut friends = Vec::new();
        for friend_id in result {
            if let Ok(profile) = self.user_service.get_profile(&friend_id).await {
                friends.push(profile);
            }
        }

        Ok(friends)
    }

    async fn get_pending_requests(&self, user_id: &UserId) -> SocialResult<Vec<Friendship>> {
        let mut requests = Vec::new();

        for entry in self.friendships.iter() {
            let friendship = entry.value();
            if &friendship.addressee_id == user_id && friendship.status == FriendshipStatus::Pending {
                requests.push(friendship.clone());
            }
        }

        Ok(requests)
    }

    async fn get_sent_requests(&self, user_id: &UserId) -> SocialResult<Vec<Friendship>> {
        let mut requests = Vec::new();

        for entry in self.friendships.iter() {
            let friendship = entry.value();
            if &friendship.requester_id == user_id && friendship.status == FriendshipStatus::Pending {
                requests.push(friendship.clone());
            }
        }

        Ok(requests)
    }

    async fn get_friendship_status(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<Option<FriendshipStatus>> {
        if let Some((_, friendship)) = self.find_friendship(user_id, other_user_id) {
            Ok(Some(friendship.status))
        } else {
            Ok(None)
        }
    }

    async fn is_user_blocked(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<bool> {
        Ok(self.blocks.contains_key(&(user_id.clone(), other_user_id.clone())))
    }

    async fn get_blocked_users(&self, user_id: &UserId) -> SocialResult<Vec<UserBlock>> {
        let mut blocks = Vec::new();

        for entry in self.blocks.iter() {
            let block = entry.value();
            if &block.blocker_id == user_id {
                blocks.push(block.clone());
            }
        }

        Ok(blocks)
    }

    async fn get_mutual_friends(&self, user_id: &UserId, other_user_id: &UserId) -> SocialResult<Vec<UserProfile>> {
        let mut user_friends: HashSet<UserId> = HashSet::new();
        let mut other_friends: HashSet<UserId> = HashSet::new();

        for entry in self.friendships.iter() {
            let friendship = entry.value();
            if friendship.status == FriendshipStatus::Accepted {
                if &friendship.requester_id == user_id {
                    user_friends.insert(friendship.addressee_id.clone());
                } else if &friendship.addressee_id == user_id {
                    user_friends.insert(friendship.requester_id.clone());
                }

                if &friendship.requester_id == other_user_id {
                    other_friends.insert(friendship.addressee_id.clone());
                } else if &friendship.addressee_id == other_user_id {
                    other_friends.insert(friendship.requester_id.clone());
                }
            }
        }

        let mutual: Vec<UserId> = user_friends.intersection(&other_friends).cloned().collect();

        let mut result = Vec::new();
        for friend_id in mutual {
            if let Ok(profile) = self.user_service.get_profile(&friend_id).await {
                result.push(profile);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_profile() {
        let service = DefaultUserService::new();
        let profile = service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();
        assert_eq!(profile.username, "testuser");
        assert_eq!(profile.display_name, "Test User");
    }

    #[tokio::test]
    async fn test_duplicate_username() {
        let service = DefaultUserService::new();
        service.create_profile("testuser".to_string(), "Test User".to_string()).await.unwrap();
        let result = service.create_profile("testuser".to_string(), "Another User".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_friend_request() {
        let user_service = Arc::new(DefaultUserService::new());
        let user1 = user_service.create_profile("user1".to_string(), "User One".to_string()).await.unwrap();
        let user2 = user_service.create_profile("user2".to_string(), "User Two".to_string()).await.unwrap();

        let friend_service = DefaultFriendService::new(user_service.clone());
        let friendship = friend_service.send_friend_request(&user1.id, &user2.id).await.unwrap();
        assert_eq!(friendship.status, FriendshipStatus::Pending);
    }
}