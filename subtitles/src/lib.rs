//! Vantis Babel - Advanced Subtitle System
//! 
//! Subtitle aggregation from multiple sources with:
/// - Automatic encoding detection and conversion
/// - AI-powered subtitle synchronization
/// - Hash-based matching for perfect alignment
/// - Machine translation support for subtitles
/// - Subtitle style customization
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
pub mod encoding;
pub mod translation;
pub mod styles;
pub mod ai_generation;

use aggregator::SubtitleAggregator;
use parser::{SubtitleFormat, SubtitleTrack};
use translation::{MachineTranslator, TranslationConfig};
use styles::{StyleManager, SubtitleStyle};

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

// Re-export AI generation types
pub use ai_generation::{
    AIGenerationConfig,
    TranscriptionModel,
    TranscriptionResult,
    SpeakerInfo,
    WordTiming,
    SubtitleSegment,
    SubtitleOutputFormat,
    AISubtitleGenerator,
    WhisperSubtitleGenerator,
    BatchSubtitleGenerator,
    AISubtitleError,
};

/// Vantis Babel - Subtitle Engine
pub struct VantisBabel {
    /// Subtitle aggregator
    aggregator: SubtitleAggregator,
    
    /// Loaded subtitle tracks
    tracks: HashMap<String, SubtitleTrack>,
    
    /// Default language
    default_language: String,
    
    /// Machine translator
    translator: MachineTranslator,
    
    /// Style manager
    style_manager: StyleManager,
}

impl VantisBabel {
    /// Create a new Vantis Babel instance
    pub fn new() -> Result<Self> {
        info!("📝 Initializing Vantis Babel (Subtitle Engine)");
        
        let translator = MachineTranslator::new(TranslationConfig::default());
        let style_manager = StyleManager::new();
        
        Ok(Self {
            aggregator: SubtitleAggregator::new()?,
            tracks: HashMap::new(),
            default_language: "pl".to_string(),
            translator,
            style_manager,
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
    
    /// Translate subtitle track to target language
    pub async fn translate_subtitle(
        &mut self,
        track_id: &str,
        target_language: &str,
    ) -> Result<()> {
        info!("🌍 Translating subtitle: {} to {}", track_id, target_language);
        
        if let Some(track) = self.tracks.get_mut(track_id) {
            let source_language = &track.language;
            
            // Translate all subtitle entries
            for entry in &mut track.entries {
                if let Some(result) = self.translator
                    .translate(&entry.text, source_language, target_language)
                    .await
                    .context("Failed to translate subtitle text")?
                {
                    entry.text = result.translated_text;
                }
            }
            
            // Update track language
            track.language = target_language.to_string();
            
            info!("✅ Subtitle translated to {}", target_language);
        }
        
        Ok(())
    }
    
    /// Get machine translator
    pub fn translator(&self) -> &MachineTranslator {
        &self.translator
    }
    
    /// Get machine translator mutable reference
    pub fn translator_mut(&mut self) -> &mut MachineTranslator {
        &mut self.translator
    }
    
    /// Get style manager
    pub fn style_manager(&self) -> &StyleManager {
        &self.style_manager
    }
    
    /// Get style manager mutable reference
    pub fn style_manager_mut(&mut self) -> &mut StyleManager {
        &mut self.style_manager
    }
    
    /// Apply subtitle style
    pub fn apply_subtitle_style(&mut self, style: SubtitleStyle) -> Result<()> {
        info!("🎨 Applying subtitle style");
        
        // Validate and set the style
        style.validate()?;
        self.style_manager.set_current_style(style).await?;
        
        info!("✅ Subtitle style applied");
        Ok(())
    }
    
    /// Get current subtitle style
    pub fn get_subtitle_style(&self) -> SubtitleStyle {
        // Return current style from style manager
        // Note: This is synchronous but the style manager is async
        // In a real implementation, you'd handle this differently
        SubtitleStyle::default()
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