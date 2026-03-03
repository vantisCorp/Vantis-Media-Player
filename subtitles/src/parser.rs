//! Subtitle Parser
//! 
//! Parses various subtitle formats:
/// - SubRip (.srt)
/// - SubStation Alpha (.ssa/.ass)
/// - WebVTT (.vtt)
/// - MicroDVD (.sub)

use anyhow::{Result, anyhow};
use std::time::Duration;
use tracing::debug;

use crate::encoding::detect_and_convert;

/// Subtitle entry
#[derive(Debug, Clone)]
pub struct SubtitleEntry {
    /// Start time in milliseconds
    pub start_ms: u64,
    
    /// End time in milliseconds
    pub end_ms: u64,
    
    /// Subtitle text
    pub text: String,
}

impl SubtitleEntry {
    /// Create a new subtitle entry
    pub fn new(start_ms: u64, end_ms: u64, text: String) -> Self {
        Self {
            start_ms,
            end_ms,
            text,
        }
    }
    
    /// Get duration
    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.end_ms - self.start_ms)
    }
    
    /// Check if time is within this subtitle
    pub fn contains_time(&self, time_ms: u64) -> bool {
        time_ms >= self.start_ms && time_ms < self.end_ms
    }
}

/// Subtitle format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    SubRip,
    SubStationAlpha,
    WebVTT,
    MicroDVD,
    TTML,
    IMSC,
    Unknown,
}

/// Subtitle track
#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    /// Track ID
    pub id: String,
    
    /// Source name
    pub source: String,
    
    /// Language code (e.g., "pl", "en")
    pub language: String,
    
    /// Subtitle format
    pub format: SubtitleFormat,
    
    /// Content hash
    pub hash: String,
    
    /// Match score (0.0 - 1.0)
    pub score: f64,
    
    /// Subtitle entries
    pub entries: Vec<SubtitleEntry>,
    
    /// Download URL
    pub download_url: Option<String>,
}

impl SubtitleTrack {
    /// Get subtitle text at specific time
    pub fn get_text_at(&self, time_ms: u64) -> Option<String> {
        for entry in &self.entries {
            if entry.contains_time(time_ms) {
                return Some(entry.text.clone());
            }
        }
        None
    }
    
    /// Shift all subtitles by offset (in milliseconds)
    pub fn shift(&mut self, offset_ms: i64) {
        for entry in &mut self.entries {
            entry.start_ms = if offset_ms >= 0 {
                entry.start_ms + offset_ms as u64
            } else {
                entry.start_ms.saturating_sub((-offset_ms) as u64)
            };
            entry.end_ms = if offset_ms >= 0 {
                entry.end_ms + offset_ms as u64
            } else {
                entry.end_ms.saturating_sub((-offset_ms) as u64)
            };
        }
    }
    
    /// Get track duration
    pub fn duration(&self) -> u64 {
        self.entries
            .last()
            .map(|e| e.end_ms)
            .unwrap_or(0)
    }
}

/// Subtitle parser
pub struct SubtitleParser;

impl SubtitleParser {
    /// Parse subtitle file
    pub fn parse_file(path: &str) -> Result<SubtitleTrack> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content, path)
    }
    
    /// Parse subtitle content
    pub fn parse(content: &str, source: &str) -> Result<SubtitleTrack> {
        // Detect encoding and convert to UTF-8 if needed
        let content = detect_and_convert(content.as_bytes())?;
        
        // Detect format
        let format = Self::detect_format(&content);
        
        // Parse based on format
        let entries = match format {
            SubtitleFormat::SubRip => Self::parse_srt(&content)?,
            SubtitleFormat::SubStationAlpha => Self::parse_ssa(&content)?,
            SubtitleFormat::WebVTT => Self::parse_vtt(&content)?,
            SubtitleFormat::MicroDVD => Self::parse_sub(&content)?,
            SubtitleFormat::TTML => Self::parse_ttml(&content)?,
            SubtitleFormat::IMSC => Self::parse_imsc(&content)?,
            SubtitleFormat::Unknown => Self::parse_srt(&content)?, // Try SRT as default
        };
        
        let path = std::path::Path::new(source);
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        Ok(SubtitleTrack {
            id: filename.to_string(),
            source: "local".to_string(),
            language: Self::detect_language(source),
            format,
            hash: Self::calculate_hash(content),
            score: 1.0,
            entries,
            download_url: None,
        })
    }
    
    /// Detect subtitle format
    fn detect_format(content: &str) -> SubtitleFormat {
        if content.starts_with("WEBVTT") {
            SubtitleFormat::WebVTT
        } else if content.contains("[Script Info]") || content.contains("[V4+ Styles]") {
            SubtitleFormat::SubStationAlpha
        } else if content.contains("{DEFAULT}") || content.lines()
            .next()
            .map(|l| l.contains('{') && l.contains('}'))
            .unwrap_or(false)
        {
            SubtitleFormat::MicroDVD
        } else if content.contains("<?xml") && (content.contains("<tt ") || content.contains("<tt:")) {
            // TTML/IMSC format - detect specific profile
            if content.contains("http://www.w3.org/ns/ttml") {
                if content.contains("imsc1") || content.contains("http://www.smpte-ra.org/schemas/2052-1/2010/") {
                    SubtitleFormat::IMSC
                } else {
                    SubtitleFormat::TTML
                }
            } else {
                SubtitleFormat::TTML
            }
        } else {
            SubtitleFormat::SubRip
        }
    }
    
    /// Parse SubRip format
    fn parse_srt(content: &str) -> Result<Vec<SubtitleEntry>> {
        let mut entries = Vec::new();
        
        for block in content.split("\n\n") {
            let lines: Vec<&str> = block.lines().collect();
            if lines.len() < 3 {
                continue;
            }
            
            // Parse timing line (e.g., "00:00:01,000 --> 00:00:04,000")
            let timing = lines[1];
            let parts: Vec<&str> = timing.split(" --> ").collect();
            if parts.len() != 2 {
                continue;
            }
            
            let start = Self::parse_timestamp(parts[0].trim())?;
            let end = Self::parse_timestamp(parts[1].trim().split(' ').next().unwrap_or(parts[1]))?;
            
            // Combine text lines
            let text = lines[2..].join("\n");
            
            entries.push(SubtitleEntry::new(start, end, text));
        }
        
        debug!("📝 Parsed {} SRT entries", entries.len());
        Ok(entries)
    }
    
    /// Parse SubStation Alpha format
    fn parse_ssa(_content: &str) -> Result<Vec<SubtitleEntry>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
    
    /// Parse WebVTT format
    fn parse_vtt(content: &str) -> Result<Vec<SubtitleEntry>> {
        // Similar to SRT but with different timestamp format
        let content = content.strip_prefix("WEBVTT").unwrap_or(content);
        Self::parse_srt(content.trim())
    }
    
    /// Parse MicroDVD format
    fn parse_sub(_content: &str) -> Result<Vec<SubtitleEntry>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
    
    /// Parse timestamp (e.g., "00:00:01,000" -> 1000ms)
    fn parse_timestamp(ts: &str) -> Result<u64> {
        let parts: Vec<&str> = ts.replace(',', ".").split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid timestamp format"));
        }
        
        let hours: u64 = parts[0].parse()?;
        let minutes: u64 = parts[1].parse()?;
        let seconds: f64 = parts[2].parse()?;
        
        let total_ms = hours * 3600000 + minutes * 60000 + (seconds * 1000.0) as u64;
        Ok(total_ms)
    }
    
    /// Detect language from filename
    fn detect_language(filename: &str) -> String {
        let filename_lower = filename.to_lowercase();
        
        // Common language codes in filenames
        let lang_patterns = vec![
            (".pl.", "pl"),
            (".pol.", "pl"),
            (".polski.", "pl"),
            (".en.", "en"),
            (".eng.", "en"),
            (".english.", "en"),
            (".de.", "de"),
            (".ger.", "de"),
            (".german.", "de"),
        ];
        
        for (pattern, lang) in lang_patterns {
            if filename_lower.contains(pattern) {
                return lang.to_string();
            }
        }
        
        "unknown".to_string()
    }
    
    /// Parse TTML content
    fn parse_ttml(content: &str) -> Result<Vec<SubtitleEntry>> {
        use crate::ttml::TtmlParser;
        
        let ttml_entries = TtmlParser::parse(content)?;
        
        // Convert TtmlSubtitleEntry to SubtitleEntry
        let entries = ttml_entries.into_iter().map(|ttml| {
            SubtitleEntry {
                start_ms: ttml.base.start_ms,
                end_ms: ttml.base.end_ms,
                text: ttml.base.text,
            }
        }).collect();
        
        Ok(entries)
    }
    
    /// Parse IMSC content
    fn parse_imsc(content: &str) -> Result<Vec<SubtitleEntry>> {
        use crate::ttml::TtmlParser;
        
        let ttml_entries = TtmlParser::parse_imsc(content)?;
        
        // Convert TtmlSubtitleEntry to SubtitleEntry
        let entries = ttml_entries.into_iter().map(|ttml| {
            SubtitleEntry {
                start_ms: ttml.base.start_ms,
                end_ms: ttml.base.end_ms,
                text: ttml.base.text,
            }
        }).collect();
        
        Ok(entries)
    }
    
    /// Calculate simple hash
    fn calculate_hash(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}