//! Advanced Vantis Media Player Example
//! 
//! Demonstrates:
//! - Custom event handling
//! - Playlist management
//! - Subtitle auto-download
//! - Custom UI integration

use anyhow::Result;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use vanis_core::{EventBus, Event, PlaybackState};

/// Advanced player with playlist support
pub struct AdvancedPlayer {
    /// Event bus
    event_bus: Arc<EventBus>,
    
    /// Playlist
    playlist: Arc<Mutex<VecDeque<PathBuf>>>,
    
    /// Current index
    current_index: Arc<Mutex<usize>>,
    
    /// Auto-play next track
    auto_play: Arc<Mutex<bool>>,
}

impl AdvancedPlayer {
    /// Create new advanced player
    pub fn new() -> Result<Self> {
        let event_bus = Arc::new(EventBus::new());
        let playlist = Arc::new(Mutex::new(VecDeque::new()));
        let current_index = Arc::new(Mutex::new(0));
        let auto_play = Arc::new(Mutex::new(true));
        
        info!("🎬 Advanced Player initialized");
        
        Ok(Self {
            event_bus,
            playlist,
            current_index,
            auto_play,
        })
    }
    
    /// Add files to playlist
    pub async fn add_to_playlist(&self, paths: Vec<PathBuf>) {
        let mut playlist = self.playlist.lock().await;
        for path in paths {
            playlist.push_back(path);
            info!("➕ Added to playlist: {:?}", path);
        }
        
        let count = playlist.len();
        info!("📋 Playlist now has {} item(s)", count);
    }
    
    /// Play next track in playlist
    pub async fn play_next(&self) -> Result<Option<PathBuf>> {
        let mut playlist = self.playlist.lock().await;
        let mut current_index = self.current_index.lock().await;
        
        if playlist.is_empty() {
            warn!("Playlist is empty");
            return Ok(None);
        }
        
        *current_index += 1;
        
        if *current_index >= playlist.len() {
            // Loop back to start
            *current_index = 0;
        }
        
        if let Some(next_track) = playlist.get(*current_index) {
            info!("▶ Playing next: {:?}", next_track);
            self.event_bus.publish(Event::PlaybackStateChanged {
                state: PlaybackState::Playing,
            });
            Ok(Some(next_track.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Play previous track
    pub async fn play_previous(&self) -> Result<Option<PathBuf>> {
        let mut playlist = self.playlist.lock().await;
        let mut current_index = self.current_index.lock().await;
        
        if playlist.is_empty() {
            warn!("Playlist is empty");
            return Ok(None);
        }
        
        if *current_index > 0 {
            *current_index -= 1;
        } else {
            // Go to last track
            *current_index = playlist.len() - 1;
        }
        
        if let Some(prev_track) = playlist.get(*current_index) {
            info!("⏮ Playing previous: {:?}", prev_track);
            self.event_bus.publish(Event::PlaybackStateChanged {
                state: PlaybackState::Playing,
            });
            Ok(Some(prev_track.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Shuffle playlist
    pub async fn shuffle_playlist(&self) {
        let mut playlist = self.playlist.lock().await;
        use std::collections::VecDeque;
        
        let mut items: Vec<PathBuf> = playlist.drain(..).collect();
        use rand::seq::SliceRandom;
        items.shuffle(&mut rand::thread_rng());
        
        *playlist = items.into_iter().collect();
        
        info!("🔀 Playlist shuffled");
    }
    
    /// Clear playlist
    pub async fn clear_playlist(&self) {
        let mut playlist = self.playlist.lock().await;
        playlist.clear();
        let mut current_index = self.current_index.lock().await;
        *current_index = 0;
        
        info!("🗑️ Playlist cleared");
    }
    
    /// Get playlist
    pub async fn get_playlist(&self) -> Vec<PathBuf> {
        let playlist = self.playlist.lock().await;
        playlist.iter().cloned().collect()
    }
    
    /// Get current track
    pub async fn get_current_track(&self) -> Option<PathBuf> {
        let playlist = self.playlist.lock().await;
        let current_index = self.current_index.lock().await;
        playlist.get(*current_index).cloned()
    }
    
    /// Set auto-play
    pub async fn set_auto_play(&self, enabled: bool) {
        let mut auto_play = self.auto_play.lock().await;
        *auto_play = enabled;
        info!("🔄 Auto-play: {}", enabled);
    }
    
    /// Start event loop
    pub async fn run_event_loop(&self) {
        info!("🎮 Starting advanced event loop");
        
        let subscription = self.event_bus.subscribe();
        let auto_play = self.auto_play.clone();
        let playlist = self.playlist.clone();
        let current_index = self.current_index.clone();
        
        tokio::spawn(async move {
            // Handle events
            while let Ok(event) = subscription.receiver.recv().await {
                match event {
                    Event::PlaybackStateChanged { state } => {
                        if state == PlaybackState::Stopped {
                            // Check if auto-play next track
                            if *auto_play.lock().await {
                                let playlist = playlist.lock().await;
                                let current_index = current_index.lock().await;
                                if *current_index < playlist.len() - 1 {
                                    info!("🔄 Auto-playing next track");
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("🚀 Vantis Advanced Player Example");
    
    // Create advanced player
    let player = AdvancedPlayer::new()?;
    
    // Add some files to playlist
    let files = vec![
        PathBuf::from("movie1.mp4"),
        PathBuf::from("movie2.mkv"),
        PathBuf::from("movie3.avi"),
    ];
    
    player.add_to_playlist(files).await;
    
    // Start event loop
    player.run_event_loop().await;
    
    // Play next track
    if let Some(track) = player.play_next().await? {
        info!("Now playing: {:?}", track);
    }
    
    // Shuffle playlist
    player.shuffle_playlist().await;
    
    // Display playlist
    let playlist = player.get_playlist().await;
    info!("Current playlist:");
    for (i, track) in playlist.iter().enumerate() {
        info!("  [{}] {:?}", i, track);
    }
    
    Ok(())
}