//! Playback Synchronization
//! 
//! Manages synchronized playback across all participants in a session.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::{ParticipantId, CollaborationError, CollaborationResult};

/// Sync state for playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Current playback position in seconds
    pub position_seconds: f64,
    
    /// Playback rate (1.0 = normal)
    pub playback_rate: f64,
    
    /// Is playing
    pub is_playing: bool,
    
    /// Last sync timestamp
    pub last_sync_time: DateTime<Utc>,
    
    /// Server timestamp for latency calculation
    pub server_time: DateTime<Utc>,
    
    /// Participant sync statuses
    pub participant_states: HashMap<ParticipantId, ParticipantSyncState>,
    
    /// Pending sync action
    pub pending_action: Option<SyncAction>,
    
    /// Sync leader (who initiated current sync)
    pub sync_leader: Option<ParticipantId>,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            position_seconds: 0.0,
            playback_rate: 1.0,
            is_playing: false,
            last_sync_time: Utc::now(),
            server_time: Utc::now(),
            participant_states: HashMap::new(),
            pending_action: None,
            sync_leader: None,
        }
    }
}

/// Per-participant sync state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantSyncState {
    /// Participant ID
    pub participant_id: ParticipantId,
    
    /// Reported position
    pub position_seconds: f64,
    
    /// Reported playback rate
    pub playback_rate: f64,
    
    /// Is playing
    pub is_playing: bool,
    
    /// Buffer state
    pub buffer_state: BufferState,
    
    /// Last update time
    pub last_update: DateTime<Utc>,
    
    /// Estimated latency in milliseconds
    pub estimated_latency_ms: u32,
    
    /// Sync offset from leader
    pub sync_offset_ms: i32,
}

impl ParticipantSyncState {
    /// Create a new participant sync state
    pub fn new(participant_id: ParticipantId) -> Self {
        Self {
            participant_id,
            position_seconds: 0.0,
            playback_rate: 1.0,
            is_playing: false,
            buffer_state: BufferState::Ready,
            last_update: Utc::now(),
            estimated_latency_ms: 0,
            sync_offset_ms: 0,
        }
    }
}

/// Buffer state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BufferState {
    /// Buffering in progress
    Buffering { progress: f32 },
    /// Ready to play
    Ready,
    /// Error occurred
    Error { code: String },
}

/// Sync action to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncAction {
    /// Play action
    Play { 
        position: f64,
        target_time: DateTime<Utc>,
    },
    /// Pause action
    Pause { 
        position: f64,
    },
    /// Seek action
    Seek { 
        position: f64,
        target_time: DateTime<Utc>,
    },
    /// Rate change action
    RateChange { 
        rate: f64,
    },
}

/// Sync event for broadcasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    /// Event ID
    pub id: String,
    
    /// Source participant
    pub source: ParticipantId,
    
    /// Action
    pub action: SyncAction,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Execute at server time
    pub execute_at: DateTime<Utc>,
}

impl SyncEvent {
    /// Create a new sync event
    pub fn new(source: ParticipantId, action: SyncAction) -> Self {
        Self {
            id: format!("sync_{}", uuid::Uuid::new_v4()),
            source,
            action,
            timestamp: Utc::now(),
            execute_at: Utc::now(),
        }
    }
}

/// Sync manager
pub struct SyncManager {
    /// Sync threshold in milliseconds
    sync_threshold_ms: u32,
    
    /// Buffer time before play
    buffer_time_seconds: u32,
    
    /// Maximum latency allowed
    max_latency_ms: u32,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(sync_threshold_ms: u32, buffer_time_seconds: u32, max_latency_ms: u32) -> Self {
        Self {
            sync_threshold_ms,
            buffer_time_seconds,
            max_latency_ms,
        }
    }
    
    /// Calculate the target position for all participants
    pub fn calculate_target_position(&self, state: &SyncState) -> f64 {
        state.position_seconds
    }
    
    /// Check if participants are in sync
    pub fn check_sync(&self, state: &SyncState) -> SyncCheckResult {
        let mut out_of_sync = Vec::new();
        let threshold = self.sync_threshold_ms as f64 / 1000.0;
        
        for (id, ps) in &state.participant_states {
            let position_diff = (ps.position_seconds - state.position_seconds).abs();
            
            if position_diff > threshold {
                out_of_sync.push((*id).clone());
            }
        }
        
        SyncCheckResult {
            is_synced: out_of_sync.is_empty(),
            out_of_sync,
        }
    }
    
    /// Calculate required corrections for out-of-sync participants
    pub fn calculate_corrections(&self, state: &SyncState) -> HashMap<ParticipantId, SyncCorrection> {
        let mut corrections = HashMap::new();
        
        for (id, ps) in &state.participant_states {
            let position_diff = ps.position_seconds - state.position_seconds;
            let threshold = self.sync_threshold_ms as f64 / 1000.0;
            
            if position_diff.abs() > threshold {
                corrections.insert((*id).clone(), SyncCorrection {
                    target_position: state.position_seconds,
                    current_position: ps.position_seconds,
                    correction_ms: (position_diff * 1000.0) as i32,
                });
            }
        }
        
        corrections
    }
    
    /// Calculate target time for synchronized play
    pub fn calculate_play_target_time(&self) -> DateTime<Utc> {
        // Give participants time to buffer
        Utc::now() + chrono::Duration::seconds(self.buffer_time_seconds as i64)
    }
    
    /// Update participant sync state
    pub fn update_participant_state(
        &self,
        state: &mut SyncState,
        participant_id: ParticipantId,
        position: f64,
        playback_rate: f64,
        is_playing: bool,
        buffer_state: BufferState,
        latency_ms: u32,
    ) {
        let sync_offset = if let Some(leader_id) = &state.sync_leader {
            if let Some(leader_state) = state.participant_states.get(leader_id) {
                ((position - leader_state.position_seconds) * 1000.0) as i32
            } else {
                0
            }
        } else {
            0
        };
        
        state.participant_states.insert(participant_id.clone(), ParticipantSyncState {
            participant_id,
            position_seconds: position,
            playback_rate,
            is_playing,
            buffer_state,
            last_update: Utc::now(),
            estimated_latency_ms: latency_ms,
            sync_offset_ms: sync_offset,
        });
    }
    
    /// Apply a sync action
    pub fn apply_action(&self, state: &mut SyncState, action: &SyncAction) {
        match action {
            SyncAction::Play { position, .. } => {
                state.position_seconds = *position;
                state.is_playing = true;
            }
            SyncAction::Pause { position } => {
                state.position_seconds = *position;
                state.is_playing = false;
            }
            SyncAction::Seek { position, .. } => {
                state.position_seconds = *position;
            }
            SyncAction::RateChange { rate } => {
                state.playback_rate = *rate;
            }
        }
        
        state.last_sync_time = Utc::now();
        state.server_time = Utc::now();
    }
    
    /// Get participants that are buffering
    pub fn get_buffering_participants(&self, state: &SyncState) -> Vec<&ParticipantId> {
        state.participant_states.iter()
            .filter(|(_, ps)| matches!(ps.buffer_state, BufferState::Buffering { .. }))
            .map(|(id, _)| id)
            .collect()
    }
    
    /// Check if all participants are ready
    pub fn all_ready(&self, state: &SyncState) -> bool {
        state.participant_states.values()
            .all(|ps| ps.buffer_state == BufferState::Ready)
    }
    
    /// Calculate average latency
    pub fn average_latency(&self, state: &SyncState) -> u32 {
        if state.participant_states.is_empty() {
            return 0;
        }
        
        let total: u32 = state.participant_states.values()
            .map(|ps| ps.estimated_latency_ms)
            .sum();
        
        total / state.participant_states.len() as u32
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new(500, 3, 2000)
    }
}

/// Result of sync check
#[derive(Debug, Clone)]
pub struct SyncCheckResult {
    /// Whether all participants are in sync
    pub is_synced: bool,
    
    /// Participants that are out of sync
    pub out_of_sync: Vec<ParticipantId>,
}

/// Sync correction for a participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCorrection {
    /// Target position
    pub target_position: f64,
    
    /// Current position
    pub current_position: f64,
    
    /// Correction in milliseconds
    pub correction_ms: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sync_state_default() {
        let state = SyncState::default();
        assert_eq!(state.position_seconds, 0.0);
        assert!(!state.is_playing);
    }
    
    #[test]
    fn test_sync_manager_check() {
        let manager = SyncManager::default();
        let state = SyncState::default();
        let result = manager.check_sync(&state);
        
        assert!(result.is_synced);
    }
    
    #[test]
    fn test_participant_sync_state() {
        let ps = ParticipantSyncState::new("part_1".to_string());
        assert_eq!(ps.playback_rate, 1.0);
    }
    
    #[test]
    fn test_apply_play_action() {
        let manager = SyncManager::default();
        let mut state = SyncState::default();
        
        let action = SyncAction::Play { 
            position: 10.0, 
            target_time: Utc::now(),
        };
        
        manager.apply_action(&mut state, &action);
        assert!(state.is_playing);
        assert_eq!(state.position_seconds, 10.0);
    }
}