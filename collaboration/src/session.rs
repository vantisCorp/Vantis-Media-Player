//! Viewing Session Management
//! 
//! Manages collaborative viewing sessions for synchronized playback.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::participant::{Participant, ParticipantRole, ParticipantStatus};
use crate::sync::SyncState;
use crate::{CollaborationError, CollaborationResult, SessionId, ParticipantId};

/// Viewing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewingSession {
    /// Unique session identifier
    pub id: SessionId,
    
    /// Session name
    pub name: String,
    
    /// Session description
    pub description: Option<String>,
    
    /// Content being watched
    pub content: ContentInfo,
    
    /// Session creator
    pub host_id: ParticipantId,
    
    /// All participants
    pub participants: HashMap<ParticipantId, Participant>,
    
    /// Current session state
    pub state: SessionState,
    
    /// Sync state for playback
    pub sync_state: SyncState,
    
    /// Session settings
    pub settings: SessionSettings,
    
    /// Session creation time
    pub created_at: DateTime<Utc>,
    
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    
    /// Maximum participants allowed
    pub max_participants: u32,
    
    /// Is the session public (discoverable)
    pub is_public: bool,
    
    /// Session password (if private)
    pub password_hash: Option<String>,
    
    /// Session tags
    pub tags: Vec<String>,
}

/// Content information being watched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentInfo {
    /// Content identifier
    pub id: String,
    
    /// Content title
    pub title: String,
    
    /// Content type (movie, series, etc.)
    pub content_type: String,
    
    /// Content URL or streaming source
    pub url: Option<String>,
    
    /// Total duration in seconds
    pub duration_seconds: u64,
    
    /// Poster image URL
    pub poster_url: Option<String>,
    
    /// Content source service
    pub source: Option<String>,
}

/// Session state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    /// Session created, waiting for participants
    Waiting,
    /// Session is active and playing
    Playing,
    /// Session is paused
    Paused,
    /// Session is buffering
    Buffering,
    /// Session ended
    Ended,
}

/// Session settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Auto-sync playback for all participants
    pub auto_sync: bool,
    
    /// Allow anyone to control playback
    pub anyone_can_control: bool,
    
    /// Show participant cursors/indicators
    pub show_participants: bool,
    
    /// Enable chat
    pub chat_enabled: bool,
    
    /// Enable reactions
    pub reactions_enabled: bool,
    
    /// Sync threshold in milliseconds (how far out of sync before auto-correct)
    pub sync_threshold_ms: u32,
    
    /// Buffer time before starting for all participants
    pub buffer_time_seconds: u32,
    
    /// Maximum chat message length
    pub max_chat_length: u32,
    
    /// Rate limit reactions (per minute)
    pub reaction_rate_limit: u32,
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            auto_sync: true,
            anyone_can_control: false,
            show_participants: true,
            chat_enabled: true,
            reactions_enabled: true,
            sync_threshold_ms: 500,
            buffer_time_seconds: 3,
            max_chat_length: 500,
            reaction_rate_limit: 30,
        }
    }
}

impl ViewingSession {
    /// Create a new viewing session
    pub fn new(
        name: String,
        content: ContentInfo,
        host: Participant,
        max_participants: u32,
    ) -> Self {
        let id = generate_session_id();
        let host_id = host.id.clone();
        let mut participants = HashMap::new();
        participants.insert(host.id.clone(), host);
        
        Self {
            id,
            name,
            description: None,
            content,
            host_id,
            participants,
            state: SessionState::Waiting,
            sync_state: SyncState::default(),
            settings: SessionSettings::default(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            max_participants,
            is_public: false,
            password_hash: None,
            tags: Vec::new(),
        }
    }
    
    /// Add a participant to the session
    pub fn add_participant(&mut self, participant: Participant) -> CollaborationResult<()> {
        if self.participants.len() >= self.max_participants as usize {
            return Err(CollaborationError::SessionFull);
        }
        
        if self.participants.contains_key(&participant.id) {
            return Err(CollaborationError::InvalidState("Participant already in session".to_string()));
        }
        
        self.participants.insert(participant.id.clone(), participant);
        self.last_activity = Utc::now();
        
        Ok(())
    }
    
    /// Remove a participant from the session
    pub fn remove_participant(&mut self, participant_id: &ParticipantId) -> CollaborationResult<Participant> {
        let participant = self.participants.remove(participant_id)
            .ok_or_else(|| CollaborationError::ParticipantNotFound(participant_id.clone()))?;
        
        // If host leaves, transfer to another participant
        if participant_id == &self.host_id {
            if let Some(new_host) = self.participants.values().next() {
                self.host_id = new_host.id.clone();
            }
        }
        
        self.last_activity = Utc::now();
        Ok(participant)
    }
    
    /// Get participant by ID
    pub fn get_participant(&self, participant_id: &ParticipantId) -> Option<&Participant> {
        self.participants.get(participant_id)
    }
    
    /// Get mutable participant by ID
    pub fn get_participant_mut(&mut self, participant_id: &ParticipantId) -> Option<&mut Participant> {
        self.participants.get_mut(participant_id)
    }
    
    /// Check if session is empty
    pub fn is_empty(&self) -> bool {
        self.participants.is_empty()
    }
    
    /// Get participant count
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
    
    /// Check if a participant is the host
    pub fn is_host(&self, participant_id: &ParticipantId) -> bool {
        &self.host_id == participant_id
    }
    
    /// Check if participant can control playback
    pub fn can_control(&self, participant_id: &ParticipantId) -> bool {
        if self.settings.anyone_can_control {
            return true;
        }
        
        self.is_host(participant_id)
    }
    
    /// Update session state
    pub fn update_state(&mut self, new_state: SessionState) {
        self.state = new_state;
        self.last_activity = Utc::now();
    }
    
    /// Get all participant IDs
    pub fn participant_ids(&self) -> Vec<ParticipantId> {
        self.participants.keys().cloned().collect()
    }
    
    /// Get active participants
    pub fn active_participants(&self) -> Vec<&Participant> {
        self.participants.values()
            .filter(|p| p.status == ParticipantStatus::Active)
            .collect()
    }
}

/// Generate a unique session ID
fn generate_session_id() -> SessionId {
    format!("sess_{}", uuid::Uuid::new_v4())
}

/// Session manager for managing multiple sessions
pub struct SessionManager {
    sessions: RwLock<HashMap<SessionId, ViewingSession>>,
    max_sessions: usize,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(max_sessions: usize) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            max_sessions,
        }
    }
    
    /// Create a new session
    pub async fn create_session(
        &self,
        name: String,
        content: ContentInfo,
        host: Participant,
        max_participants: u32,
    ) -> CollaborationResult<ViewingSession> {
        let mut sessions = self.sessions.write().await;
        
        if sessions.len() >= self.max_sessions {
            return Err(CollaborationError::InvalidState("Maximum sessions reached".to_string()));
        }
        
        let session = ViewingSession::new(name, content, host, max_participants);
        sessions.insert(session.id.clone(), session.clone());
        
        Ok(session)
    }
    
    /// Get a session by ID
    pub async fn get_session(&self, session_id: &SessionId) -> Option<ViewingSession> {
        self.sessions.read().await.get(session_id).cloned()
    }
    
    /// Update a session
    pub async fn update_session(&self, session: ViewingSession) -> CollaborationResult<()> {
        let mut sessions = self.sessions.write().await;
        
        if !sessions.contains_key(&session.id) {
            return Err(CollaborationError::SessionNotFound(session.id));
        }
        
        sessions.insert(session.id.clone(), session);
        Ok(())
    }
    
    /// Delete a session
    pub async fn delete_session(&self, session_id: &SessionId) -> CollaborationResult<ViewingSession> {
        let mut sessions = self.sessions.write().await;
        
        sessions.remove(session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))
    }
    
    /// Get all public sessions
    pub async fn get_public_sessions(&self) -> Vec<ViewingSession> {
        self.sessions.read().await.values()
            .filter(|s| s.is_public)
            .cloned()
            .collect()
    }
    
    /// Get sessions by host
    pub async fn get_sessions_by_host(&self, host_id: &ParticipantId) -> Vec<ViewingSession> {
        self.sessions.read().await.values()
            .filter(|s| s.host_id == *host_id)
            .cloned()
            .collect()
    }
    
    /// Get active session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }
    
    /// Clean up expired/inactive sessions
    pub async fn cleanup_inactive(&self, max_inactive_minutes: i64) -> usize {
        let mut sessions = self.sessions.write().await;
        let threshold = Utc::now() - chrono::Duration::minutes(max_inactive_minutes);
        
        let initial_count = sessions.len();
        sessions.retain(|_, s| s.last_activity > threshold || !s.is_empty());
        
        initial_count - sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_participant() -> Participant {
        Participant {
            id: format!("part_{}", uuid::Uuid::new_v4()),
            name: "Test User".to_string(),
            role: ParticipantRole::Host,
            status: ParticipantStatus::Active,
            avatar_url: None,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
        }
    }
    
    fn create_test_content() -> ContentInfo {
        ContentInfo {
            id: "movie123".to_string(),
            title: "Test Movie".to_string(),
            content_type: "movie".to_string(),
            url: Some("https://example.com/movie.mp4".to_string()),
            duration_seconds: 7200,
            poster_url: None,
            source: None,
        }
    }
    
    #[test]
    fn test_session_creation() {
        let host = create_test_participant();
        let content = create_test_content();
        let session = ViewingSession::new("Test Session".to_string(), content, host, 10);
        
        assert_eq!(session.state, SessionState::Waiting);
        assert_eq!(session.participant_count(), 1);
    }
    
    #[test]
    fn test_add_participant() {
        let host = create_test_participant();
        let content = create_test_content();
        let mut session = ViewingSession::new("Test Session".to_string(), content, host, 10);
        
        let participant = Participant {
            id: "part_2".to_string(),
            name: "Guest".to_string(),
            role: ParticipantRole::Viewer,
            status: ParticipantStatus::Active,
            avatar_url: None,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
        };
        
        assert!(session.add_participant(participant).is_ok());
        assert_eq!(session.participant_count(), 2);
    }
    
    #[test]
    fn test_session_full() {
        let host = create_test_participant();
        let content = create_test_content();
        let mut session = ViewingSession::new("Test Session".to_string(), content, host, 1);
        
        let participant = Participant {
            id: "part_2".to_string(),
            name: "Guest".to_string(),
            role: ParticipantRole::Viewer,
            status: ParticipantStatus::Active,
            avatar_url: None,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
        };
        
        assert!(matches!(session.add_participant(participant), Err(CollaborationError::SessionFull)));
    }
}