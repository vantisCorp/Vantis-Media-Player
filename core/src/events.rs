//! Event Bus System
//! 
//! Central message passing system for all components.
//! Enables decoupled communication between subsystems.

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::debug;

/// Event types in the system
#[derive(Debug, Clone)]
pub enum Event {
    /// Play media file
    Play {
        file: String,
    },
    
    /// Pause playback
    Pause,
    
    /// Resume playback
    Resume,
    
    /// Stop playback
    Stop,
    
    /// Seek to position
    Seek {
        position: f64,
    },
    
    /// Set volume
    SetVolume {
        volume: f32,
    },
    
    /// Set mute
    SetMute {
        muted: bool,
    },
    
    /// Set playback speed
    SetSpeed {
        speed: f32,
    },
    
    /// Quit player
    Quit,
    
    /// Media loaded event
    MediaLoaded {
        path: String,
        duration: u64,
    },
    
    /// Playback started
    PlaybackStarted {
        file: String,
    },
    
    /// Playback paused
    PlaybackPaused,
    
    /// Playback resumed
    PlaybackResumed,
    
    /// Playback stopped
    PlaybackStopped,
    
    /// Playback state changed
    PlaybackStateChanged {
        state: PlaybackState,
    },
    
    /// Time position updated
    PositionUpdated {
        position_ms: u64,
    },
    
    /// Seeked to position
    Seeked {
        position: f64,
    },
    
    /// Volume changed
    VolumeChanged {
        volume: f32,
    },
    
    /// Mute changed
    MuteChanged {
        muted: bool,
    },
    
    /// Speed changed
    SpeedChanged {
        speed: f32,
    },
    
    /// Subtitle loaded
    SubtitleLoaded {
        language: String,
        source: String,
    },
    
    /// Error occurred
    Error {
        message: String,
    },
    
    /// Custom event
    Custom {
        event_type: String,
        data: String,
    },
}

/// Playback states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Buffering,
    Seeking,
}

impl std::fmt::Display for PlaybackState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaybackState::Playing => write!(f, "▶ Playing"),
            PlaybackState::Paused => write!(f, "⏸ Paused"),
            PlaybackState::Stopped => write!(f, "⏹ Stopped"),
            PlaybackState::Buffering => write!(f, "⏳ Buffering"),
            PlaybackState::Seeking => write!(f, "⏩ Seeking"),
        }
    }
}

/// Event subscription handle
pub struct Subscription {
    _id: uuid::Uuid,
    sender: mpsc::UnboundedSender<Event>,
}

/// Central event bus
pub struct EventBus {
    /// Subscribers to events
    subscribers: Arc<RwLock<Vec<mpsc::UnboundedSender<Event>>>>,
    
    /// Event channel
    sender: mpsc::UnboundedSender<Event>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Event>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        debug!("📡 Event bus initialized");
        
        Self {
            subscribers: Arc::new(RwLock::new(Vec::new())),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
    
    /// Subscribe to all events
    pub fn subscribe(&self) -> Subscription {
        let (tx, rx) = mpsc::unbounded_channel();
        
        let mut subscribers = self.subscribers.write();
        subscribers.push(tx.clone());
        
        // Start listening for events
        let receiver = self.receiver.clone();
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            while let Some(event) = receiver.lock().await.recv().await {
                let _ = tx_clone.send(event);
            }
        });
        
        Subscription {
            _id: uuid::Uuid::new_v4(),
            sender: tx,
        }
    }
    
    /// Publish an event
    pub fn publish(&self, event: Event) {
        let subscribers = self.subscribers.read();
        
        debug!("📤 Publishing event: {:?}", event);
        
        for sender in subscribers.iter() {
            let _ = sender.send(event.clone());
        }
    }
    
    /// Get event sender
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}