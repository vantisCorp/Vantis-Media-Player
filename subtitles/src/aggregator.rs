//! Subtitle Aggregator
//! 
//! Aggregates subtitles from multiple sources:
/// - NapiProjekt
/// - Napisy24
/// - OpenSubtitles
/// - Subscene
/// - Addic7ed
/// - Podnapisi
/// - YIFY Subtitles
/// - Subtitulos

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tokio::task::JoinSet;

use crate::sources::{NapiProjekt, Napisy24, OpenSubtitles, Subscene, Addic7ed, Podnapisi, YifySubtitles, Subtitulos, SubtitleSource};
use crate::parser::SubtitleTrack;

/// Subtitle aggregator
pub struct SubtitleAggregator {
    /// Available subtitle sources
    sources: Vec<Box<dyn SubtitleSource>>,
}

impl SubtitleAggregator {
    /// Create a new subtitle aggregator
    pub fn new() -> Result<Self> {
        let sources: Vec<Box<dyn SubtitleSource>> = vec![
            Box::new(NapiProjekt::new()?),
            Box::new(Napisy24::new()?),
            Box::new(OpenSubtitles::new()?),
            Box::new(Subscene::new()?),
            Box::new(Addic7ed::new()?),
            Box::new(Podnapisi::new()?),
            Box::new(YifySubtitles::new()?),
            Box::new(Subtitulos::new()?),
        ];
        
        Ok(Self { sources })
    }
    
    /// Search subtitles from all sources
    pub async fn search(&self, video_path: &str, language: &str) -> Result<Vec<SubtitleTrack>> {
        let mut join_set = JoinSet::new();
        
        // Spawn search tasks for all sources
        for source in &self.sources {
            let source_clone = source.clone_box();
            let video_path = video_path.to_string();
            let language = language.to_string();
            
            join_set.spawn(async move {
                source_clone.search(&video_path, &language).await
            });
        }
        
        // Collect results
        let mut all_tracks = Vec::new();
        
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(tracks)) => {
                    all_tracks.extend(tracks);
                }
                Ok(Err(e)) => {
                    tracing::warn!("Source search error: {}", e);
                }
                Err(e) => {
                    tracing::error!("Search task failed: {}", e);
                }
            }
        }
        
        // Remove duplicates and sort
        all_tracks.dedup_by_key(|t| t.hash.clone());
        all_tracks.sort_by(|a, b| b.score.cmp(&a.score));
        
        Ok(all_tracks)
    }
}

impl Default for SubtitleAggregator {
    fn default() -> Self {
        Self::new().expect("Failed to create subtitle aggregator")
    }
}