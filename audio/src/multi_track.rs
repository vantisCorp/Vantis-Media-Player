//! Multi-track audio support
//! 
//! Handles media files with multiple audio tracks:
//! - Track enumeration and selection
//! - Language and codec metadata
//! - Seamless track switching
//! - Track synchronization

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};

/// Audio track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    /// Unique track identifier
    pub id: u32,
    
    /// Track index in the container
    pub index: usize,
    
    /// Track title/name
    pub title: Option<String>,
    
    /// Language code (ISO 639-2)
    pub language: Option<String>,
    
    /// Language name (human readable)
    pub language_name: Option<String>,
    
    /// Codec name
    pub codec: String,
    
    /// Number of channels
    pub channels: u32,
    
    /// Channel layout description
    pub channel_layout: Option<String>,
    
    /// Sample rate in Hz
    pub sample_rate: u32,
    
    /// Bit depth (if applicable)
    pub bit_depth: Option<u32>,
    
    /// Bitrate in bits per second
    pub bitrate: Option<u32>,
    
    /// Is this a default track
    pub is_default: bool,
    
    /// Is this track forced (e.g., for foreign language parts)
    pub is_forced: bool,
    
    /// Is this a commentary track
    pub is_commentary: bool,
    
    /// Is this a hearing impaired track
    pub is_hearing_impaired: bool,
    
    /// Is this track currently selected
    #[serde(skip)]
    pub is_selected: bool,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl AudioTrack {
    /// Create a new audio track with minimal info
    pub fn new(id: u32, index: usize, codec: String) -> Self {
        Self {
            id,
            index,
            title: None,
            language: None,
            language_name: None,
            codec,
            channels: 2,
            channel_layout: None,
            sample_rate: 48000,
            bit_depth: None,
            bitrate: None,
            is_default: false,
            is_forced: false,
            is_commentary: false,
            is_hearing_impaired: false,
            is_selected: false,
            metadata: HashMap::new(),
        }
    }
    
    /// Get a display name for the track
    pub fn display_name(&self) -> String {
        if let Some(ref title) = self.title {
            return title.clone();
        }
        
        let lang = self.language_name.as_ref()
            .or(self.language.as_ref())
            .map(|l| l.as_str())
            .unwrap_or("Unknown");
        
        let suffix = if self.is_commentary {
            " (Commentary)"
        } else if self.is_hearing_impaired {
            " (HI)"
        } else if self.is_forced {
            " (Forced)"
        } else {
            ""
        };
        
        format!("Track {} - {}{}", self.index + 1, lang, suffix)
    }
    
    /// Get short description
    pub fn short_description(&self) -> String {
        let channels = match self.channels {
            1 => "Mono",
            2 => "Stereo",
            6 => "5.1",
            8 => "7.1",
            n => return format!("{}ch", n),
        };
        
        let codec = &self.codec;
        
        format!("{} {}", channels, codec)
    }
    
    /// Get quality description
    pub fn quality_description(&self) -> String {
        let mut parts = Vec::new();
        
        // Sample rate
        if self.sample_rate >= 96000 {
            parts.push("96kHz".to_string());
        } else if self.sample_rate >= 48000 {
            parts.push("48kHz".to_string());
        } else if self.sample_rate >= 44100 {
            parts.push("44.1kHz".to_string());
        }
        
        // Bit depth
        if let Some(bd) = self.bit_depth {
            if bd >= 24 {
                parts.push(format!("{}-bit", bd));
            }
        }
        
        // Bitrate
        if let Some(br) = self.bitrate {
            if br >= 1_000_000 {
                parts.push(format!("{} Mbps", br / 1_000_000));
            } else {
                parts.push(format!("{} kbps", br / 1_000));
            }
        }
        
        parts.join(", ")
    }
}

/// Multi-track audio manager
pub struct MultiTrackAudio {
    /// Available audio tracks
    tracks: Vec<AudioTrack>,
    
    /// Currently selected track ID
    selected_track_id: Option<u32>,
    
    /// Default track ID
    default_track_id: Option<u32>,
    
    /// Preferred language
    preferred_language: Option<String>,
    
    /// Track switch callback
    on_track_change: Option<Arc<RwLock<Box<dyn Fn(u32) + Send + Sync>>>>,
}

impl MultiTrackAudio {
    /// Create a new multi-track audio manager
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            selected_track_id: None,
            default_track_id: None,
            preferred_language: None,
            on_track_change: None,
        }
    }
    
    /// Add a track to the manager
    pub fn add_track(&mut self, track: AudioTrack) {
        if track.is_default && self.default_track_id.is_none() {
            self.default_track_id = Some(track.id);
        }
        
        self.tracks.push(track);
    }
    
    /// Set tracks from a vector
    pub fn set_tracks(&mut self, tracks: Vec<AudioTrack>) {
        // Clear existing tracks
        self.tracks.clear();
        self.selected_track_id = None;
        self.default_track_id = None;
        
        // Add all tracks
        for track in tracks {
            self.add_track(track);
        }
        
        // Auto-select track
        self.auto_select_track();
        
        info!("🎵 Loaded {} audio track(s)", self.tracks.len());
    }
    
    /// Get all available tracks
    pub fn tracks(&self) -> &[AudioTrack] {
        &self.tracks
    }
    
    /// Get track by ID
    pub fn get_track(&self, id: u32) -> Option<&AudioTrack> {
        self.tracks.iter().find(|t| t.id == id)
    }
    
    /// Get mutable track by ID
    pub fn get_track_mut(&mut self, id: u32) -> Option<&mut AudioTrack> {
        self.tracks.iter_mut().find(|t| t.id == id)
    }
    
    /// Get currently selected track
    pub fn selected_track(&self) -> Option<&AudioTrack> {
        self.selected_track_id
            .and_then(|id| self.get_track(id))
    }
    
    /// Get selected track ID
    pub fn selected_track_id(&self) -> Option<u32> {
        self.selected_track_id
    }
    
    /// Set preferred language for track selection
    pub fn set_preferred_language(&mut self, language: &str) {
        self.preferred_language = Some(language.to_string());
        debug!("🌐 Preferred audio language: {}", language);
    }
    
    /// Select a track by ID
    pub fn select_track(&mut self, id: u32) -> Result<()> {
        // Validate track exists and get display info before mutating
        let (display_name, codec, channels, sample_rate) = {
            let track = self.get_track(id)
                .ok_or_else(|| anyhow::anyhow!("Audio track {} not found", id))?;
            (track.display_name().to_string(), track.codec.clone(), track.channels, track.sample_rate)
        };
        
        // Update selection state
        for t in &mut self.tracks {
            t.is_selected = t.id == id;
        }
        
        let prev_id = self.selected_track_id;
        self.selected_track_id = Some(id);
        
        info!("🔊 Selected audio track: {}", display_name);
        debug!("   Codec: {}, Channels: {}, Rate: {} Hz", 
               codec, channels, sample_rate);
        
        // Call callback if set
        if let Some(callback) = &self.on_track_change {
            if prev_id != Some(id) {
                let callback = callback.read();
                callback(id);
            }
        }
        
        Ok(())
    }
    
    /// Auto-select the best track based on preferences
    pub fn auto_select_track(&mut self) {
        if self.tracks.is_empty() {
            return;
        }
        
        // Priority: preferred language > default track > first track
        let selected = if let Some(ref lang) = self.preferred_language {
            // Try to find track with preferred language
            self.tracks.iter()
                .find(|t| t.language.as_ref().map(|l| l.starts_with(lang)).unwrap_or(false))
                .map(|t| t.id)
        } else {
            None
        };
        
        let selected = selected.or_else(|| {
            // Try default track
            self.default_track_id
        });
        
        let selected = selected.or_else(|| {
            // First track
            self.tracks.first().map(|t| t.id)
        });
        
        if let Some(id) = selected {
            let _ = self.select_track(id);
        }
    }
    
    /// Set track change callback
    pub fn set_on_track_change<F>(&mut self, callback: F) 
    where 
        F: Fn(u32) + Send + Sync + 'static 
    {
        self.on_track_change = Some(Arc::new(RwLock::new(Box::new(callback))));
    }
    
    /// Get tracks by language
    pub fn tracks_by_language(&self, language: &str) -> Vec<&AudioTrack> {
        self.tracks.iter()
            .filter(|t| t.language.as_ref().map(|l| l.starts_with(language)).unwrap_or(false))
            .collect()
    }
    
    /// Get tracks by codec
    pub fn tracks_by_codec(&self, codec: &str) -> Vec<&AudioTrack> {
        self.tracks.iter()
            .filter(|t| t.codec.to_lowercase().contains(&codec.to_lowercase()))
            .collect()
    }
    
    /// Check if multiple tracks are available
    pub fn has_multiple_tracks(&self) -> bool {
        self.tracks.len() > 1
    }
    
    /// Get number of tracks
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
    
    /// Clear all tracks
    pub fn clear(&mut self) {
        self.tracks.clear();
        self.selected_track_id = None;
        self.default_track_id = None;
    }
    
    /// Get next track (cyclic)
    pub fn next_track(&mut self) -> Option<&AudioTrack> {
        if self.tracks.len() <= 1 {
            return self.selected_track();
        }
        
        let current_index = self.selected_track_id
            .and_then(|id| self.tracks.iter().position(|t| t.id == id))
            .unwrap_or(0);
        
        let next_index = (current_index + 1) % self.tracks.len();
        let next_id = self.tracks[next_index].id;
        
        let _ = self.select_track(next_id);
        self.selected_track()
    }
    
    /// Get previous track (cyclic)
    pub fn previous_track(&mut self) -> Option<&AudioTrack> {
        if self.tracks.len() <= 1 {
            return self.selected_track();
        }
        
        let current_index = self.selected_track_id
            .and_then(|id| self.tracks.iter().position(|t| t.id == id))
            .unwrap_or(0);
        
        let prev_index = if current_index == 0 {
            self.tracks.len() - 1
        } else {
            current_index - 1
        };
        
        let prev_id = self.tracks[prev_index].id;
        
        let _ = self.select_track(prev_id);
        self.selected_track()
    }
    
    /// Find best quality track
    pub fn find_best_quality_track(&self) -> Option<&AudioTrack> {
        self.tracks.iter()
            .max_by(|a, b| {
                let score_a = self.quality_score(a);
                let score_b = self.quality_score(b);
                score_a.cmp(&score_b)
            })
    }
    
    /// Calculate quality score for a track
    fn quality_score(&self, track: &AudioTrack) -> u32 {
        let mut score = 0u32;
        
        // Channels (more = better for most content)
        score += track.channels * 10;
        
        // Sample rate
        score += track.sample_rate / 1000;
        
        // Bit depth
        if let Some(bd) = track.bit_depth {
            score += bd;
        }
        
        // Bitrate
        if let Some(br) = track.bitrate {
            score += br / 10000;
        }
        
        // Prefer lossless codecs
        if track.codec.to_lowercase().contains("flac") ||
           track.codec.to_lowercase().contains("truehd") ||
           track.codec.to_lowercase().contains("dtshd") ||
           track.codec.to_lowercase().contains("lpcm") {
            score += 100;
        }
        
        score
    }
    
    /// Export track list as JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.tracks)
            .map_err(|e| anyhow::anyhow!("Failed to serialize tracks: {}", e))
    }
}

impl Default for MultiTrackAudio {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio track builder for convenient track creation
pub struct AudioTrackBuilder {
    track: AudioTrack,
}

impl AudioTrackBuilder {
    pub fn new(id: u32, index: usize, codec: &str) -> Self {
        Self {
            track: AudioTrack::new(id, index, codec.to_string()),
        }
    }
    
    pub fn title(mut self, title: &str) -> Self {
        self.track.title = Some(title.to_string());
        self
    }
    
    pub fn language(mut self, code: &str, name: &str) -> Self {
        self.track.language = Some(code.to_string());
        self.track.language_name = Some(name.to_string());
        self
    }
    
    pub fn channels(mut self, channels: u32) -> Self {
        self.track.channels = channels;
        self
    }
    
    pub fn channel_layout(mut self, layout: &str) -> Self {
        self.track.channel_layout = Some(layout.to_string());
        self
    }
    
    pub fn sample_rate(mut self, rate: u32) -> Self {
        self.track.sample_rate = rate;
        self
    }
    
    pub fn bit_depth(mut self, depth: u32) -> Self {
        self.track.bit_depth = Some(depth);
        self
    }
    
    pub fn bitrate(mut self, bitrate: u32) -> Self {
        self.track.bitrate = Some(bitrate);
        self
    }
    
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.track.is_default = is_default;
        self
    }
    
    pub fn is_forced(mut self, is_forced: bool) -> Self {
        self.track.is_forced = is_forced;
        self
    }
    
    pub fn is_commentary(mut self, is_commentary: bool) -> Self {
        self.track.is_commentary = is_commentary;
        self
    }
    
    pub fn is_hearing_impaired(mut self, is_hi: bool) -> Self {
        self.track.is_hearing_impaired = is_hi;
        self
    }
    
    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        self.track.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn build(self) -> AudioTrack {
        self.track
    }
}

/// Language code to name mapping
pub fn language_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "eng" | "en" => "English",
        "pol" | "pl" => "Polish",
        "ger" | "de" | "deu" => "German",
        "fre" | "fr" | "fra" => "French",
        "spa" | "es" => "Spanish",
        "ita" | "it" => "Italian",
        "jpn" | "ja" => "Japanese",
        "kor" | "ko" => "Korean",
        "chi" | "zh" | "zho" => "Chinese",
        "rus" | "ru" => "Russian",
        "por" | "pt" => "Portuguese",
        "dut" | "nl" | "nld" => "Dutch",
        "ara" | "ar" => "Arabic",
        "hin" | "hi" => "Hindi",
        "tur" | "tr" => "Turkish",
        "ukr" | "uk" => "Ukrainian",
        "cze" | "cs" => "Czech",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_track_creation() {
        let track = AudioTrack::new(1, 0, "aac".to_string());
        assert_eq!(track.id, 1);
        assert_eq!(track.codec, "aac");
        assert_eq!(track.channels, 2);
    }
    
    #[test]
    fn test_track_builder() {
        let track = AudioTrackBuilder::new(1, 0, "aac")
            .title("English 5.1")
            .language("eng", "English")
            .channels(6)
            .sample_rate(48000)
            .is_default(true)
            .build();
        
        assert_eq!(track.title, Some("English 5.1".to_string()));
        assert_eq!(track.language, Some("eng".to_string()));
        assert_eq!(track.channels, 6);
        assert!(track.is_default);
    }
    
    #[test]
    fn test_multi_track_manager() {
        let mut manager = MultiTrackAudio::new();
        
        let track1 = AudioTrackBuilder::new(1, 0, "aac")
            .language("eng", "English")
            .is_default(true)
            .build();
        
        let track2 = AudioTrackBuilder::new(2, 1, "ac3")
            .language("pol", "Polish")
            .build();
        
        manager.add_track(track1);
        manager.add_track(track2);
        
        assert_eq!(manager.track_count(), 2);
        assert!(manager.has_multiple_tracks());
    }
    
    #[test]
    fn test_track_selection() {
        let mut manager = MultiTrackAudio::new();
        
        let track1 = AudioTrackBuilder::new(1, 0, "aac")
            .language("eng", "English")
            .is_default(true)
            .build();
        
        let track2 = AudioTrackBuilder::new(2, 1, "ac3")
            .language("pol", "Polish")
            .build();
        
        manager.set_tracks(vec![track1, track2]);
        
        // Should auto-select default track
        assert_eq!(manager.selected_track_id(), Some(1));
        
        // Switch to track 2
        manager.select_track(2).unwrap();
        assert_eq!(manager.selected_track_id(), Some(2));
    }
    
    #[test]
    fn test_preferred_language() {
        let mut manager = MultiTrackAudio::new();
        manager.set_preferred_language("pol");
        
        let track1 = AudioTrackBuilder::new(1, 0, "aac")
            .language("eng", "English")
            .is_default(true)
            .build();
        
        let track2 = AudioTrackBuilder::new(2, 1, "ac3")
            .language("pol", "Polish")
            .build();
        
        manager.set_tracks(vec![track1, track2]);
        
        // Should select Polish track due to preference
        assert_eq!(manager.selected_track_id(), Some(2));
    }
    
    #[test]
    fn test_track_navigation() {
        let mut manager = MultiTrackAudio::new();
        
        manager.set_tracks(vec![
            AudioTrackBuilder::new(1, 0, "aac").build(),
            AudioTrackBuilder::new(2, 1, "ac3").build(),
            AudioTrackBuilder::new(3, 2, "dts").build(),
        ]);
        
        assert_eq!(manager.selected_track_id(), Some(1));
        
        manager.next_track();
        assert_eq!(manager.selected_track_id(), Some(2));
        
        manager.next_track();
        assert_eq!(manager.selected_track_id(), Some(3));
        
        manager.next_track(); // Wrap around
        assert_eq!(manager.selected_track_id(), Some(1));
        
        manager.previous_track();
        assert_eq!(manager.selected_track_id(), Some(3));
    }
    
    #[test]
    fn test_quality_score() {
        let manager = MultiTrackAudio::new();
        
        let low = AudioTrackBuilder::new(1, 0, "aac")
            .channels(2)
            .sample_rate(44100)
            .build();
        
        let high = AudioTrackBuilder::new(2, 1, "truehd")
            .channels(8)
            .sample_rate(96000)
            .bit_depth(24)
            .build();
        
        let low_score = manager.quality_score(&low);
        let high_score = manager.quality_score(&high);
        
        assert!(high_score > low_score);
    }
    
    #[test]
    fn test_display_name() {
        let track = AudioTrackBuilder::new(1, 0, "aac")
            .title("Director's Commentary")
            .is_commentary(true)
            .build();
        
        assert_eq!(track.display_name(), "Director's Commentary");
        
        let track2 = AudioTrackBuilder::new(2, 1, "ac3")
            .language("eng", "English")
            .build();
        
        assert_eq!(track2.display_name(), "Track 2 - English");
    }
    
    #[test]
    fn test_language_name() {
        assert_eq!(language_name("eng"), "English");
        assert_eq!(language_name("pol"), "Polish");
        assert_eq!(language_name("jpn"), "Japanese");
    }
}