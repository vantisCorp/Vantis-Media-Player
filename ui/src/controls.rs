//! Playback Controls
//! 
/// Smart controls with keyboard shortcuts and mouse gestures.

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Pause,
    Stop,
    SeekForward,
    SeekBackward,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    NextTrack,
    PreviousTrack,
    ToggleFullscreen,
}

/// Controls state
pub struct ControlsState {
    /// Is playing
    pub playing: bool,
    
    /// Volume (0.0 - 1.0)
    pub volume: f32,
    
    /// Is muted
    pub muted: bool,
    
    /// Is fullscreen
    pub fullscreen: bool,
}

impl ControlsState {
    pub fn new() -> Self {
        Self {
            playing: false,
            volume: 1.0,
            muted: false,
            fullscreen: false,
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Play => {
                self.playing = true;
                tracing::info!("▶ Play");
            }
            Message::Pause => {
                self.playing = false;
                tracing::info!("⏸ Pause");
            }
            Message::Stop => {
                self.playing = false;
                tracing::info!("⏹ Stop");
            }
            Message::SeekForward => {
                tracing::info!("⏩ Seek forward");
            }
            Message::SeekBackward => {
                tracing::info!("⏪ Seek backward");
            }
            Message::VolumeUp => {
                self.volume = (self.volume + 0.1).min(1.0);
                tracing::info!("🔊 Volume: {:.0}%", self.volume * 100.0);
            }
            Message::VolumeDown => {
                self.volume = (self.volume - 0.1).max(0.0);
                tracing::info!("🔉 Volume: {:.0}%", self.volume * 100.0);
            }
            Message::ToggleMute => {
                self.muted = !self.muted;
                tracing::info!("🔇 Muted: {}", self.muted);
            }
            Message::NextTrack => {
                tracing::info!("⏭ Next track");
            }
            Message::PreviousTrack => {
                tracing::info!("⏮ Previous track");
            }
            Message::ToggleFullscreen => {
                self.fullscreen = !self.fullscreen;
                tracing::info!("📺 Fullscreen: {}", self.fullscreen);
            }
        }
    }
}

impl Default for ControlsState {
    fn default() -> Self {
        Self::new()
    }
}