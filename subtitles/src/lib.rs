//! Vantis Babel - Advanced Subtitle System
//! 
//! Subtitle aggregation from multiple sources with:
/// - Automatic encoding detection and conversion
/// - AI-powered subtitle synchronization
/// - Hash-based matching for perfect alignment
/// 
//! Supported subtitle sources:
/// - NapiProjekt (Polish)
/// - Napisy24 (Polish)
/// - OpenSubtitles (International)
/// - Subscene (International)
/// - Addic7ed (TV shows)
/// - Podnapisi (European)
/// - YIFY Subtitles (Movies)
/// - Subtitulos (Spanish)

use anyhow::Result;
use std::collections::HashMap;
use tracing::{info, debug, warn};

pub mod aggregator;
pub mod sources;
pub mod parser;
pub mod sync;
pub mod synchronization;
pub mod encoding;
pub mod ttml;
pub mod webvtt;

use aggregator::SubtitleAggregator;
use parser::{SubtitleFormat, SubtitleTrack};

// Re-export new format parsers
pub use ttml::{TtmlParser, TtmlSubtitleEntry, TtmlStyle, TtmlRegion};
pub use webvtt::{WebVttParser, WebVttCue, WebVttSetting, WebVttRegion, WebVttStyle};

// Re-export subtitle sources for convenience
pub use sources::{
    NapiProjekt,
    Napisy24,
    OpenSubtitles,
    Subscene,
    Addic7ed,
    Podnapisi,
    YifySubtitles,
    Subtitulos,
    SubtitleSource,
};

/// Vantis Babel - Subtitle Engine
pub struct VantisBabel {
    /// Subtitle aggregator
    aggregator: SubtitleAggregator,
    
    /// Loaded subtitle tracks
    tracks: HashMap<String, SubtitleTrack>,
    
    /// Default language
    default_language: String,
}

impl VantisBabel {
    /// Create a new Vantis Babel instance
    pub fn new() -> Result<Self> {
        info!("📝 Initializing Vantis Babel (Subtitle Engine)");
        
        Ok(Self {
            aggregator: SubtitleAggregator::new()?,
            tracks: HashMap::new(),
            default_language: "pl".to_string(),
        })
    }
    
    /// Set default language
    pub fn set_default_language(&mut self, language: &str) {
        self.default_language = language.to_string();
        debug!("🌍 Default subtitle language: {}", language);
    }
    
    /// Search and download subtitles for a video file
    pub async fn search_subtitles(
        &mut self,
        video_path: &str,
        language: Option<&str>,
    ) -> Result<Vec<SubtitleTrack>> {
        let lang = language.unwrap_or(&self.default_language);
        
        info!("🔍 Searching subtitles for: {}", video_path);
        info!("   Language: {}", lang);
        
        // Use aggregator to search all sources
        let tracks = self.aggregator.search(video_path, lang).await?;
        
        info!("✅ Found {} subtitle track(s)", tracks.len());
        
        // Store tracks
        for track in &tracks {
            let key = format!("{}-{}", track.source, track.language);
            self.tracks.insert(key, track.clone());
        }
        
        Ok(tracks)
    }
    
    /// Load subtitle file from disk
    pub fn load_subtitle(&mut self, path: &str) -> Result<SubtitleTrack> {
        info!("📂 Loading subtitle: {}", path);
        
        let track = parser::SubtitleParser::parse_file(path)?;
        
        debug!("   Format: {:?}", track.format);
        debug!("   Entries: {}", track.entries.len());
        
        let key = format!("local-{}", track.language);
        self.tracks.insert(key, track.clone());
        
        Ok(track)
    }
    
    /// Get subtitle at specific time (in milliseconds)
    pub fn get_subtitle_at(&self, track_id: &str, time_ms: u64) -> Option<String> {
        if let Some(track) = self.tracks.get(track_id) {
            track.get_text_at(time_ms)
        } else {
            None
        }
    }
    
    /// Get all loaded tracks
    pub fn get_tracks(&self) -> Vec<SubtitleTrack> {
        self.tracks.values().cloned().collect()
    }
    
    /// Get tracks for specific language
    pub fn get_tracks_for_language(&self, language: &str) -> Vec<SubtitleTrack> {
        self.tracks
            .values()
            .filter(|t| t.language == language)
            .cloned()
            .collect()
    }
    
    /// Synchronize subtitle track using AI
    pub async fn sync_subtitle(&mut self, track_id: &str, audio_path: &str) -> Result<()> {
        info!("🎵 AI-syncing subtitle: {}", track_id);
        
        if let Some(track) = self.tracks.get_mut(track_id) {
            sync::ai_sync_subtitle(track, audio_path).await?;
            info!("✅ Subtitle synchronized");
        }
        
        Ok(())
    }
}

impl Default for VantisBabel {
    fn default() -> Self {
        Self::new().expect("Failed to create Vantis Babel")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_subtitle_engine_creation() {
        let engine = SubtitleEngine::new();
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_subtitle_engine_default() {
        let engine = SubtitleEngine::default();
        assert_eq!(engine.tracks.len(), 0);
    }
    
    #[test]
    fn test_subtitle_track_creation() {
        let track = SubtitleTrack::new("test.srt", "en");
        assert_eq!(track.language, "en");
        assert_eq!(track.format, SubtitleFormat::SRT);
    }
}