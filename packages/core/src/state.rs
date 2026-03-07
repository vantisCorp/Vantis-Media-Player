//! State management for Vantis Media Player

use crate::{MediaId, MediaInfo, PlaybackState, PlaybackPosition, Volume};
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;

/// Maximum playlist history
const MAX_HISTORY: usize = 100;

/// Application state
#[derive(Debug, Default)]
pub struct AppState {
    pub playback: PlaybackStateInner,
    pub playlist: PlaylistState,
    pub ui: UiState,
}

/// Playback state
#[derive(Debug)]
pub struct PlaybackStateInner {
    pub current_media: Option<MediaInfo>,
    pub state: PlaybackState,
    pub position: PlaybackPosition,
    pub volume: Volume,
    pub muted: bool,
    pub speed: f32,
}

impl Default for PlaybackStateInner {
    fn default() -> Self {
        Self {
            current_media: None,
            state: PlaybackState::default(),
            position: PlaybackPosition::default(),
            volume: Volume::default(),
            muted: false,
            speed: 1.0,
        }
    }
}

/// Playlist state
#[derive(Debug, Default)]
pub struct PlaylistState {
    pub items: Vec<MediaInfo>,
    pub current_index: Option<usize>,
    pub history: VecDeque<MediaId>,
}

impl PlaylistState {
    pub fn current(&self) -> Option<&MediaInfo> {
        self.current_index.and_then(|i| self.items.get(i))
    }
    
    pub fn next(&mut self) -> Option<&MediaInfo> {
        if let Some(current) = self.current_index {
            if current + 1 < self.items.len() {
                self.current_index = Some(current + 1);
                return self.items.get(current + 1);
            }
        }
        None
    }
    
    pub fn previous(&mut self) -> Option<&MediaInfo> {
        if let Some(current) = self.current_index {
            if current > 0 {
                self.current_index = Some(current - 1);
                return self.items.get(current - 1);
            }
        }
        None
    }
    
    pub fn add(&mut self, media: MediaInfo) {
        let id = media.id.clone();
        self.items.push(media);
        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
        self.history.push_front(id);
        if self.history.len() > MAX_HISTORY {
            self.history.pop_back();
        }
    }
    
    pub fn remove(&mut self, id: &MediaId) {
        self.items.retain(|m| &m.id != id);
        self.history.retain(|h| h != id);
        if let Some(current) = self.current_index {
            if current >= self.items.len() {
                self.current_index = if self.items.is_empty() {
                    None
                } else {
                    Some(self.items.len() - 1)
                }
            }
        }
    }
    
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
    }
}

/// UI state
#[derive(Debug, Default)]
pub struct UiState {
    pub fullscreen: bool,
    pub sidebar_visible: bool,
    pub settings_open: bool,
    pub about_open: bool,
}

/// Thread-safe state wrapper
pub struct SharedState {
    inner: Arc<RwLock<AppState>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppState::default())),
        }
    }
    
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, AppState> {
        self.inner.read()
    }
    
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, AppState> {
        self.inner.write()
    }
    
    pub fn clone_inner(&self) -> Arc<RwLock<AppState>> {
        Arc::clone(&self.inner)
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}