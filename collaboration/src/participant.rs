//! Participant Management
//! 
//! Manages participants in collaborative viewing sessions.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::ParticipantId;

/// Participant in a viewing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Unique participant identifier
    pub id: ParticipantId,
    
    /// Display name
    pub name: String,
    
    /// Role in the session
    pub role: ParticipantRole,
    
    /// Current status
    pub status: ParticipantStatus,
    
    /// Avatar URL
    pub avatar_url: Option<String>,
    
    /// When the participant joined
    pub joined_at: DateTime<Utc>,
    
    /// Last activity timestamp
    pub last_seen: DateTime<Utc>,
    
    /// Participant color for UI indicators
    pub color: Option<String>,
    
    /// Custom emote/emoji
    pub emoji: Option<String>,
    
    /// Is muted (cannot chat)
    pub is_muted: bool,
    
    /// Is hidden from others
    pub is_hidden: bool,
}

/// Participant role
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipantRole {
    /// Session host - full control
    Host,
    /// Co-host - can manage participants
    CoHost,
    /// Moderator - can manage chat
    Moderator,
    /// Regular viewer
    Viewer,
    /// Guest with limited permissions
    Guest,
}

impl ParticipantRole {
    /// Get the permission level for this role
    pub fn permission_level(&self) -> u8 {
        match self {
            ParticipantRole::Host => 5,
            ParticipantRole::CoHost => 4,
            ParticipantRole::Moderator => 3,
            ParticipantRole::Viewer => 2,
            ParticipantRole::Guest => 1,
        }
    }
    
    /// Check if this role can manage participants
    pub fn can_manage_participants(&self) -> bool {
        matches!(self, ParticipantRole::Host | ParticipantRole::CoHost)
    }
    
    /// Check if this role can moderate chat
    pub fn can_moderate_chat(&self) -> bool {
        matches!(self, ParticipantRole::Host | ParticipantRole::CoHost | ParticipantRole::Moderator)
    }
    
    /// Check if this role can control playback
    pub fn can_control_playback(&self) -> bool {
        matches!(self, ParticipantRole::Host | ParticipantRole::CoHost)
    }
}

/// Participant status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ParticipantStatus {
    /// Actively participating
    Active,
    /// Temporarily away
    Away,
    /// Connection issues
    Buffering,
    /// Disconnected
    Disconnected,
    /// Kicked from session
    Kicked,
}

impl Participant {
    /// Create a new participant
    pub fn new(name: String, role: ParticipantRole) -> Self {
        Self {
            id: format!("part_{}", uuid::Uuid::new_v4()),
            name,
            role,
            status: ParticipantStatus::Active,
            avatar_url: None,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
            color: None,
            emoji: None,
            is_muted: false,
            is_hidden: false,
        }
    }
    
    /// Create a host participant
    pub fn host(name: String) -> Self {
        Self::new(name, ParticipantRole::Host)
    }
    
    /// Create a viewer participant
    pub fn viewer(name: String) -> Self {
        Self::new(name, ParticipantRole::Viewer)
    }
    
    /// Create a guest participant
    pub fn guest(name: String) -> Self {
        Self::new(name, ParticipantRole::Guest)
    }
    
    /// Update last seen timestamp
    pub fn update_seen(&mut self) {
        self.last_seen = Utc::now();
    }
    
    /// Set participant status
    pub fn set_status(&mut self, status: ParticipantStatus) {
        self.status = status;
        self.update_seen();
    }
    
    /// Check if participant is active
    pub fn is_active(&self) -> bool {
        self.status == ParticipantStatus::Active
    }
    
    /// Check if participant is connected
    pub fn is_connected(&self) -> bool {
        matches!(self.status, ParticipantStatus::Active | ParticipantStatus::Away | ParticipantStatus::Buffering)
    }
    
    /// Set role
    pub fn set_role(&mut self, role: ParticipantRole) {
        self.role = role;
    }
    
    /// Set avatar
    pub fn set_avatar(&mut self, url: String) {
        self.avatar_url = Some(url);
    }
    
    /// Set color
    pub fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }
    
    /// Mute participant
    pub fn mute(&mut self) {
        self.is_muted = true;
    }
    
    /// Unmute participant
    pub fn unmute(&mut self) {
        self.is_muted = false;
    }
    
    /// Hide participant
    pub fn hide(&mut self) {
        self.is_hidden = true;
    }
    
    /// Show participant
    pub fn show(&mut self) {
        self.is_hidden = false;
    }
    
    /// Get display name with emoji if available
    pub fn display_name(&self) -> String {
        if let Some(ref emoji) = self.emoji {
            format!("{} {}", emoji, self.name)
        } else {
            self.name.clone()
        }
    }
    
    /// Get time since joined
    pub fn time_since_joined(&self) -> chrono::Duration {
        Utc::now() - self.joined_at
    }
    
    /// Get time since last seen
    pub fn time_since_seen(&self) -> chrono::Duration {
        Utc::now() - self.last_seen
    }
    
    /// Check if participant is idle (inactive for too long)
    pub fn is_idle(&self, idle_threshold_minutes: i64) -> bool {
        self.time_since_seen().num_minutes() >= idle_threshold_minutes
    }
}

/// Participant connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantConnection {
    /// Participant ID
    pub participant_id: ParticipantId,
    
    /// Connection ID
    pub connection_id: String,
    
    /// IP address (if available)
    pub ip_address: Option<String>,
    
    /// User agent
    pub user_agent: Option<String>,
    
    /// Connected at
    pub connected_at: DateTime<Utc>,
    
    /// Reconnection count
    pub reconnect_count: u32,
}

impl ParticipantConnection {
    /// Create a new connection
    pub fn new(participant_id: ParticipantId) -> Self {
        Self {
            participant_id,
            connection_id: format!("conn_{}", uuid::Uuid::new_v4()),
            ip_address: None,
            user_agent: None,
            connected_at: Utc::now(),
            reconnect_count: 0,
        }
    }
    
    /// Record a reconnection
    pub fn record_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.connected_at = Utc::now();
    }
}

/// Participant permission set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPermissions {
    /// Can play/pause
    pub can_play_pause: bool,
    
    /// Can seek
    pub can_seek: bool,
    
    /// Can change volume
    pub can_change_volume: bool,
    
    /// Can change quality
    pub can_change_quality: bool,
    
    /// Can change subtitles
    pub can_change_subtitles: bool,
    
    /// Can send chat messages
    pub can_chat: bool,
    
    /// Can send reactions
    pub can_react: bool,
    
    /// Can invite others
    pub can_invite: bool,
    
    /// Can kick others
    pub can_kick: bool,
    
    /// Can change settings
    pub can_change_settings: bool,
}

impl From<ParticipantRole> for ParticipantPermissions {
    fn from(role: ParticipantRole) -> Self {
        match role {
            ParticipantRole::Host => Self {
                can_play_pause: true,
                can_seek: true,
                can_change_volume: true,
                can_change_quality: true,
                can_change_subtitles: true,
                can_chat: true,
                can_react: true,
                can_invite: true,
                can_kick: true,
                can_change_settings: true,
            },
            ParticipantRole::CoHost => Self {
                can_play_pause: true,
                can_seek: true,
                can_change_volume: true,
                can_change_quality: true,
                can_change_subtitles: true,
                can_chat: true,
                can_react: true,
                can_invite: true,
                can_kick: true,
                can_change_settings: true,
            },
            ParticipantRole::Moderator => Self {
                can_play_pause: false,
                can_seek: false,
                can_change_volume: false,
                can_change_quality: false,
                can_change_subtitles: false,
                can_chat: true,
                can_react: true,
                can_invite: true,
                can_kick: false,
                can_change_settings: false,
            },
            ParticipantRole::Viewer => Self {
                can_play_pause: false,
                can_seek: false,
                can_change_volume: false,
                can_change_quality: false,
                can_change_subtitles: false,
                can_chat: true,
                can_react: true,
                can_invite: true,
                can_kick: false,
                can_change_settings: false,
            },
            ParticipantRole::Guest => Self {
                can_play_pause: false,
                can_seek: false,
                can_change_volume: false,
                can_change_quality: false,
                can_change_subtitles: false,
                can_chat: true,
                can_react: true,
                can_invite: false,
                can_kick: false,
                can_change_settings: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_participant_creation() {
        let participant = Participant::host("Host User".to_string());
        assert_eq!(participant.role, ParticipantRole::Host);
        assert!(participant.is_active());
    }
    
    #[test]
    fn test_role_permissions() {
        assert!(ParticipantRole::Host.can_manage_participants());
        assert!(ParticipantRole::CoHost.can_manage_participants());
        assert!(!ParticipantRole::Viewer.can_manage_participants());
    }
    
    #[test]
    fn test_participant_status() {
        let mut participant = Participant::viewer("Test".to_string());
        assert!(participant.is_active());
        assert!(participant.is_connected());
        
        participant.set_status(ParticipantStatus::Away);
        assert!(!participant.is_active());
        assert!(participant.is_connected());
        
        participant.set_status(ParticipantStatus::Disconnected);
        assert!(!participant.is_connected());
    }
    
    #[test]
    fn test_permissions_from_role() {
        let host_perms: ParticipantPermissions = ParticipantRole::Host.into();
        assert!(host_perms.can_play_pause);
        assert!(host_perms.can_kick);
        
        let guest_perms: ParticipantPermissions = ParticipantRole::Guest.into();
        assert!(!guest_perms.can_play_pause);
        assert!(!guest_perms.can_kick);
    }
}