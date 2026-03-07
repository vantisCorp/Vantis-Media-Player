//! Event system for Vantis Media Player

use crate::{MediaId, PlaybackState, PlaybackPosition, Volume};
use serde::{Deserialize, Serialize};

/// Application events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    // Playback events
    PlaybackStarted(MediaId),
    PlaybackStopped(MediaId),
    PlaybackPaused(MediaId),
    PlaybackResumed(MediaId),
    PlaybackStateChanged(PlaybackState),
    PlaybackPositionChanged(PlaybackPosition),
    PlaybackEnded(MediaId),
    PlaybackError(MediaId, String),
    
    // Volume events
    VolumeChanged(Volume),
    MuteToggled(bool),
    
    // Media events
    MediaLoaded(MediaId),
    MediaUnloaded(MediaId),
    MediaError(MediaId, String),
    
    // Playlist events
    PlaylistChanged,
    PlaylistItemAdded(MediaId),
    PlaylistItemRemoved(MediaId),
    
    // Plugin events
    PluginLoaded(String),
    PluginUnloaded(String),
    PluginError(String, String),
    
    // Application events
    ApplicationReady,
    ApplicationShutdown,
    ConfigurationChanged,
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &Event);
}

/// Event dispatcher
pub struct EventDispatcher {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
    
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }
    
    pub fn dispatch(&self, event: Event) {
        for handler in &self.handlers {
            handler.handle(&event);
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}