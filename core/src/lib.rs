//! Vantis Core - The Foundation
//!
//! Zero-cost architecture with async/await and zero-copy memory management.
//! This is the heart of the Vantis Media Player.

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::{info, debug, error};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::memory::ZeroCopyBuffer;
use crate::events::EventBus;
use crate::state::PlayerState;
use crate::config::Config;

pub mod memory;
pub mod events;
pub mod state;
pub mod config;
pub mod memory_optimization;

// Re-export memory optimization
pub use memory_optimization::{
    MemoryOptimizer, MemoryOptimizationConfig,
    VideoFramePool, BufferPoolOptimization, MemoryMonitor,
    LazyLoader, FrameHandle, BufferHandle,
    PoolStats, BufferStats, MemoryStats,
};

/// The Core System - Heart of Vantis Player
///
/// Architecture Principles:
/// 1. Zero-Copy: Data flows directly from NVMe to VRAM
/// 2. Async-First: All I/O operations are non-blocking
/// 3. Thread-Safe: Shared state with minimal locking
/// 4. Plugin-Safe: WASM sandbox for all extensions
pub struct VantisCore {
    /// Unique instance identifier
    id: Uuid,
    
    /// Central event bus for all components
    event_bus: Arc<EventBus>,
    
    /// Player state (playback position, volume, etc.)
    state: Arc<RwLock<PlayerState>>,
    
    /// Zero-copy buffer pool for media data
    buffer_pool: Arc<ZeroCopyBuffer>,
    
    /// Configuration
    config: Config,
    
    /// Video engine
    video_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Audio engine
    audio_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// UI engine
    ui_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Subtitle engine
    subtitle_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// AI engine
    ai_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Plugin manager
    plugin_manager: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Integration manager
    integration_manager: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Streaming engine
    streaming_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Advanced audio engine
    advanced_audio_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Advanced video engine
    advanced_video_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
    
    /// Advanced UI engine
    advanced_ui_engine: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl VantisCore {
    /// Create a new Vantis Core instance
    pub fn new(config: Config) -> Result<Self> {
        let id = Uuid::new_v4();
        info!("Vantis Core initialized (ID: {})", id);
        
        Ok(Self {
            id,
            event_bus: Arc::new(EventBus::new()),
            state: Arc::new(RwLock::new(PlayerState::new())),
            buffer_pool: Arc::new(ZeroCopyBuffer::new(512 * 1024 * 1024)?), // 512MB pool
            config,
            video_engine: None,
            audio_engine: None,
            ui_engine: None,
            subtitle_engine: None,
            ai_engine: None,
            plugin_manager: None,
            integration_manager: None,
            streaming_engine: None,
            advanced_audio_engine: None,
            advanced_video_engine: None,
            advanced_ui_engine: None,
        })
    }
    
    /// Set the video engine
    pub fn set_video_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.video_engine = Some(Box::new(engine));
        info!("Video engine set");
        Ok(())
    }
    
    /// Set the audio engine
    pub fn set_audio_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.audio_engine = Some(Box::new(engine));
        info!("Audio engine set");
        Ok(())
    }
    
    /// Set the UI engine
    pub fn set_ui_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.ui_engine = Some(Box::new(engine));
        info!("UI engine set");
        Ok(())
    }
    
    /// Set the subtitle engine
    pub fn set_subtitle_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.subtitle_engine = Some(Box::new(engine));
        info!("Subtitle engine set");
        Ok(())
    }
    
    /// Set the AI engine
    pub fn set_ai_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.ai_engine = Some(Box::new(engine));
        info!("AI engine set");
        Ok(())
    }
    
    /// Set the plugin manager
    pub fn set_plugin_manager<T: std::any::Any + Send + Sync>(&mut self, manager: T) -> Result<()> {
        self.plugin_manager = Some(Box::new(manager));
        info!("Plugin manager set");
        Ok(())
    }
    
    /// Set the integration manager
    pub fn set_integration_manager<T: std::any::Any + Send + Sync>(&mut self, manager: T) -> Result<()> {
        self.integration_manager = Some(Box::new(manager));
        info!("Integration manager set");
        Ok(())
    }
    
    /// Set the streaming engine
    pub fn set_streaming_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.streaming_engine = Some(Box::new(engine));
        info!("Streaming engine set");
        Ok(())
    }
    
    /// Set the advanced audio engine
    pub fn set_advanced_audio_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.advanced_audio_engine = Some(Box::new(engine));
        info!("Advanced audio engine set");
        Ok(())
    }
    
    /// Set the advanced video engine
    pub fn set_advanced_video_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.advanced_video_engine = Some(Box::new(engine));
        info!("Advanced video engine set");
        Ok(())
    }
    
    /// Set the advanced UI engine
    pub fn set_advanced_ui_engine<T: std::any::Any + Send + Sync>(&mut self, engine: T) -> Result<()> {
        self.advanced_ui_engine = Some(Box::new(engine));
        info!("Advanced UI engine set");
        Ok(())
    }
    
    /// Main event loop
    pub async fn run_event_loop(&self) {
        info!("Starting event loop...");
        
        // Create event channels
        let (event_tx, mut event_rx) = mpsc::channel(1000);
        
        // Subscribe to event bus
        let _subscription = self.event_bus.subscribe();
        
        // Main loop
        loop {
            tokio::select! {
                // Handle events from channel
                Some(event) = event_rx.recv() => {
                    if let Err(e) = self.handle_event(event).await {
                        error!("Error handling event: {}", e);
                    }
                }
                
                // Handle events from event bus
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    // Periodic tasks
                    self.update_state().await;
                }
            }
        }
    }
    
    /// Handle an event
    async fn handle_event(&self, event: crate::events::Event) -> Result<()> {
        match event {
            crate::events::Event::Play { file } => {
                info!("Playing file: {:?}", file);
                self.play_media(&file).await?;
            }
            crate::events::Event::Pause => {
                info!("Pausing playback");
                self.pause().await?;
            }
            crate::events::Event::Resume => {
                info!("Resuming playback");
                self.resume().await?;
            }
            crate::events::Event::Stop => {
                info!("Stopping playback");
                self.stop().await?;
            }
            crate::events::Event::Seek { position } => {
                info!("Seeking to: {} seconds", position);
                self.seek(position).await?;
            }
            crate::events::Event::SetVolume { volume } => {
                info!("Setting volume to: {}", volume);
                self.set_volume(volume).await?;
            }
            crate::events::Event::SetMute { muted } => {
                info!("Setting mute to: {}", muted);
                self.set_mute(muted).await?;
            }
            crate::events::Event::SetSpeed { speed } => {
                info!("Setting playback speed to: {}", speed);
                self.set_speed(speed).await?;
            }
            crate::events::Event::Quit => {
                info!("Quitting");
                self.quit().await?;
            }
            _ => {
                debug!("Unhandled event: {:?}", event);
            }
        }
        Ok(())
    }
    
    /// Update player state
    async fn update_state(&self) {
        let mut state = self.state.write();
        state.update();
    }
    
    /// Play media file
    pub async fn play_media(&self, file: &str) -> Result<()> {
        info!("Playing media: {}", file);
        
        // Update state
        {
            let mut state = self.state.write();
            state.set_current_file(file.to_string());
            state.set_playing(true);
        }
        
        // Publish event
        self.event_bus.publish(crate::events::Event::PlaybackStarted {
            file: file.to_string(),
        });
        
        Ok(())
    }
    
    /// Pause playback
    pub async fn pause(&self) -> Result<()> {
        info!("Pausing playback");
        
        {
            let mut state = self.state.write();
            state.set_playing(false);
        }
        
        self.event_bus.publish(crate::events::Event::PlaybackPaused);
        
        Ok(())
    }
    
    /// Resume playback
    pub async fn resume(&self) -> Result<()> {
        info!("Resuming playback");
        
        {
            let mut state = self.state.write();
            state.set_playing(true);
        }
        
        self.event_bus.publish(crate::events::Event::PlaybackResumed);
        
        Ok(())
    }
    
    /// Stop playback
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping playback");
        
        {
            let mut state = self.state.write();
            state.set_playing(false);
            state.set_position(0.0);
        }
        
        self.event_bus.publish(crate::events::Event::PlaybackStopped);
        
        Ok(())
    }
    
    /// Seek to position
    pub async fn seek(&self, position: f64) -> Result<()> {
        info!("Seeking to: {} seconds", position);
        
        {
            let mut state = self.state.write();
            state.set_position(position);
        }
        
        self.event_bus.publish(crate::events::Event::Seeked { position });
        
        Ok(())
    }
    
    /// Set volume
    pub async fn set_volume(&self, volume: f32) -> Result<()> {
        info!("Setting volume to: {}", volume);
        
        {
            let mut state = self.state.write();
            state.set_volume(volume.clamp(0.0, 1.0));
        }
        
        self.event_bus.publish(crate::events::Event::VolumeChanged { volume });
        
        Ok(())
    }
    
    /// Set mute
    pub async fn set_mute(&self, muted: bool) -> Result<()> {
        info!("Setting mute to: {}", muted);
        
        {
            let mut state = self.state.write();
            state.set_muted(muted);
        }
        
        self.event_bus.publish(crate::events::Event::MuteChanged { muted });
        
        Ok(())
    }
    
    /// Set playback speed
    pub async fn set_speed(&self, speed: f32) -> Result<()> {
        info!("Setting playback speed to: {}", speed);
        
        {
            let mut state = self.state.write();
            state.set_speed(speed.max(0.25).min(4.0));
        }
        
        self.event_bus.publish(crate::events::Event::SpeedChanged { speed });
        
        Ok(())
    }
    
    /// Quit the player
    pub async fn quit(&self) -> Result<()> {
        info!("Quitting player");
        
        // Stop playback
        self.stop().await?;
        
        // Publish quit event
        self.event_bus.publish(crate::events::Event::Quit);
        
        Ok(())
    }
    
    /// Get event bus
    pub fn event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }
    
    /// Get player state
    pub fn state(&self) -> Arc<RwLock<PlayerState>> {
        self.state.clone()
    }
    
    /// Get configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get buffer pool
    pub fn buffer_pool(&self) -> Arc<ZeroCopyBuffer> {
        self.buffer_pool.clone()
    }
    
    /// Get core ID
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_vantis_core_creation() {
        let config = Config::default();
        let core = VantisCore::new(config);
        assert!(core.is_ok());
        
        let core = core.unwrap();
        assert_ne!(core.id(), Uuid::nil());
    }
    
    #[test]
    fn test_event_bus() {
        let event_bus = EventBus::new();
        let subscription = event_bus.subscribe();
        
        event_bus.publish(Event::Play {
            file: "test.mp4".to_string(),
        });
    }
    
    #[test]
    fn test_player_state_creation() {
        let state = PlayerState::new();
        assert_eq!(state.playback_state, PlaybackState::Stopped);
        assert_eq!(state.volume, 1.0);
        assert_eq!(state.muted, false);
        assert_eq!(state.playback_speed, 1.0);
    }
    
    #[test]
    fn test_player_state_set_volume() {
        let mut state = PlayerState::new();
        state.set_volume(0.5);
        assert_eq!(state.volume, 0.5);
        
        // Test clamping
        state.set_volume(1.5);
        assert_eq!(state.volume, 1.0);
        
        state.set_volume(-0.5);
        assert_eq!(state.volume, 0.0);
    }
    
    #[test]
    fn test_player_state_set_position() {
        let mut state = PlayerState::new();
        state.set_position(10.5);
        assert_eq!(state.position_seconds(), 10.5);
    }
    
    #[test]
    fn test_player_state_set_playing() {
        let mut state = PlayerState::new();
        assert_eq!(state.playback_state, PlaybackState::Stopped);
        
        state.set_playing(true);
        assert_eq!(state.playback_state, PlaybackState::Playing);
        
        state.set_playing(false);
        assert_eq!(state.playback_state, PlaybackState::Paused);
    }
    
    #[test]
    fn test_player_state_progress() {
        let mut state = PlayerState::new();
        state.duration_ms = 10000; // 10 seconds
        state.position_ms = 5000; // 5 seconds
        
        assert_eq!(state.progress(), 0.5);
    }
    
    #[test]
    fn test_player_state_update() {
        let mut state = PlayerState::new();
        state.duration_ms = 10000;
        state.playback_state = PlaybackState::Playing;
        
        state.update();
        
        // Position should have increased
        assert!(state.position_ms > 0);
    }
    
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.buffer_size, 512 * 1024 * 1024);
        assert!(config.hardware_acceleration);
        assert!(config.hdr_support);
    }
}