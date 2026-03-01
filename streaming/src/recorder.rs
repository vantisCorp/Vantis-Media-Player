//! Stream recording module
//! 
//! Provides stream recording capabilities for saving live streams
//! and capturing content for offline viewing.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Recording format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingFormat {
    /// MP4 format
    MP4,
    
    /// MKV format
    MKV,
    
    /// WebM format
    WebM,
    
    /// TS format
    TS,
    
    /// Raw format
    Raw,
}

impl RecordingFormat {
    /// Get file extension
    pub fn extension(&self) -> &str {
        match self {
            RecordingFormat::MP4 => "mp4",
            RecordingFormat::MKV => "mkv",
            RecordingFormat::WebM => "webm",
            RecordingFormat::TS => "ts",
            RecordingFormat::Raw => "raw",
        }
    }
    
    /// Get MIME type
    pub fn mime_type(&self) -> &str {
        match self {
            RecordingFormat::MP4 => "video/mp4",
            RecordingFormat::MKV => "video/x-matroska",
            RecordingFormat::WebM => "video/webm",
            RecordingFormat::TS => "video/mp2t",
            RecordingFormat::Raw => "application/octet-stream",
        }
    }
}

/// Recorder configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecorderConfig {
    /// Output directory
    pub output_dir: String,
    
    /// Recording format
    pub format: RecordingFormat,
    
    /// Enable automatic splitting
    pub enable_auto_split: bool,
    
    /// Split interval in seconds (0 = no splitting)
    pub split_interval_secs: u64,
    
    /// Split by size (0 = no size limit)
    pub split_size_bytes: u64,
    
    /// Enable metadata embedding
    pub enable_metadata: bool,
    
    /// Enable chapter markers
    pub enable_chapters: bool,
    
    /// Buffer size for writing
    pub buffer_size: usize,
}

impl Default for RecorderConfig {
    fn default() -> Self {
        Self {
            output_dir: "recordings".to_string(),
            format: RecordingFormat::MP4,
            enable_auto_split: false,
            split_interval_secs: 0,
            split_size_bytes: 0,
            enable_metadata: true,
            enable_chapters: true,
            buffer_size: 8 * 1024 * 1024, // 8 MB
        }
    }
}

/// Recording metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingMetadata {
    /// Title
    pub title: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Stream URL
    pub stream_url: String,
    
    /// Start timestamp
    pub start_time: u64,
    
    /// End timestamp
    pub end_time: Option<u64>,
    
    /// Duration in seconds
    pub duration_secs: Option<u64>,
    
    /// Video codec
    pub video_codec: Option<String>,
    
    /// Audio codec
    pub audio_codec: Option<String>,
    
    /// Resolution
    pub resolution: Option<(u32, u32)>,
    
    /// Frame rate
    pub frame_rate: Option<f32>,
    
    /// Bitrate
    pub bitrate: Option<u64>,
    
    /// Custom metadata
    pub custom: std::collections::HashMap<String, String>,
}

impl Default for RecordingMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            stream_url: String::new(),
            start_time: 0,
            end_time: None,
            duration_secs: None,
            video_codec: None,
            audio_codec: None,
            resolution: None,
            frame_rate: None,
            bitrate: None,
            custom: std::collections::HashMap::new(),
        }
    }
}

/// Chapter marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMarker {
    /// Chapter title
    pub title: String,
    
    /// Start time in seconds
    pub start_time: f64,
    
    /// End time in seconds
    pub end_time: Option<f64>,
}

/// Stream recorder
pub struct StreamRecorder {
    config: RecorderConfig,
    metadata: RecordingMetadata,
    current_file: Option<File>,
    current_file_path: Option<PathBuf>,
    bytes_written: u64,
    recording_start_time: Option<u64>,
    chapter_markers: Vec<ChapterMarker>,
    is_recording: bool,
}

impl StreamRecorder {
    /// Create a new stream recorder
    pub fn new(config: RecorderConfig) -> StreamingResult<Self> {
        // Create output directory
        std::fs::create_dir_all(&config.output_dir)?;
        
        Ok(Self {
            config,
            metadata: RecordingMetadata::default(),
            current_file: None,
            current_file_path: None,
            bytes_written: 0,
            recording_start_time: None,
            chapter_markers: Vec::new(),
            is_recording: false,
        })
    }
    
    /// Start recording
    pub fn start(&mut self, metadata: RecordingMetadata) -> StreamingResult<PathBuf> {
        if self.is_recording {
            return Err(StreamingError::RecordingError(
                "Already recording".to_string()
            ));
        }
        
        // Set metadata
        self.metadata = metadata;
        self.recording_start_time = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        // Create output file
        let filename = self.generate_filename();
        let file_path = PathBuf::from(&self.config.output_dir).join(&filename);
        
        self.current_file = Some(File::create(&file_path)?);
        self.current_file_path = Some(file_path.clone());
        self.bytes_written = 0;
        self.is_recording = true;
        
        Ok(file_path)
    }
    
    /// Stop recording
    pub fn stop(&mut self) -> StreamingResult<()> {
        if !self.is_recording {
            return Ok(());
        }
        
        // Update metadata
        if let Some(start_time) = self.recording_start_time {
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            self.metadata.end_time = Some(end_time);
            self.metadata.duration_secs = Some(end_time - start_time);
        }
        
        // Close file
        self.current_file = None;
        self.is_recording = false;
        
        Ok(())
    }
    
    /// Write data to recording
    pub fn write(&mut self, data: &[u8]) -> StreamingResult<()> {
        if !self.is_recording {
            return Err(StreamingError::RecordingError(
                "Not recording".to_string()
            ));
        }
        
        // Check if we need to split
        if self.should_split() {
            self.split()?;
        }
        
        // Write data
        if let Some(ref mut file) = self.current_file {
            file.write_all(data)?;
            self.bytes_written += data.len() as u64;
        }
        
        Ok(())
    }
    
    /// Add chapter marker
    pub fn add_chapter(&mut self, title: String, start_time: f64) -> StreamingResult<()> {
        if !self.config.enable_chapters {
            return Ok(());
        }
        
        let chapter = ChapterMarker {
            title,
            start_time,
            end_time: None,
        };
        
        self.chapter_markers.push(chapter);
        Ok(())
    }
    
    /// Check if recording should be split
    fn should_split(&self) -> bool {
        if !self.config.enable_auto_split {
            return false;
        }
        
        // Check size limit
        if self.config.split_size_bytes > 0 && self.bytes_written >= self.config.split_size_bytes {
            return true;
        }
        
        // Check time limit
        if self.config.split_interval_secs > 0 {
            if let Some(start_time) = self.recording_start_time {
                let elapsed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() - start_time;
                
                if elapsed >= self.config.split_interval_secs {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Split recording into new file
    fn split(&mut self) -> StreamingResult<()> {
        // Stop current recording
        self.stop()?;
        
        // Update chapter end time
        if let Some(last_chapter) = self.chapter_markers.last_mut() {
            if let Some(duration) = self.metadata.duration_secs {
                last_chapter.end_time = Some(duration as f64);
            }
        }
        
        // Start new recording with incremented filename
        let mut new_metadata = self.metadata.clone();
        new_metadata.start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.start(new_metadata)?;
        
        Ok(())
    }
    
    /// Generate filename for recording
    fn generate_filename(&self) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let title = if self.metadata.title.is_empty() {
            "recording".to_string()
        } else {
            self.metadata.title
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
                .collect()
        };
        
        format!("{}_{}.{}", title, timestamp, self.config.format.extension())
    }
    
    /// Get current file path
    pub fn current_file_path(&self) -> Option<&Path> {
        self.current_file_path.as_deref()
    }
    
    /// Get recording statistics
    pub fn stats(&self) -> RecordingStats {
        RecordingStats {
            is_recording: self.is_recording,
            bytes_written: self.bytes_written,
            duration_secs: self.metadata.duration_secs,
            chapter_count: self.chapter_markers.len(),
            file_count: if self.is_recording { 1 } else { 0 },
        }
    }
    
    /// Get metadata
    pub fn metadata(&self) -> &RecordingMetadata {
        &self.metadata
    }
    
    /// Get chapter markers
    pub fn chapters(&self) -> &[ChapterMarker] {
        &self.chapter_markers
    }
}

/// Recording statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStats {
    /// Is currently recording
    pub is_recording: bool,
    
    /// Bytes written
    pub bytes_written: u64,
    
    /// Duration in seconds
    pub duration_secs: Option<u64>,
    
    /// Number of chapters
    pub chapter_count: usize,
    
    /// Number of files (for split recordings)
    pub file_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recording_format_extension() {
        assert_eq!(RecordingFormat::MP4.extension(), "mp4");
        assert_eq!(RecordingFormat::MKV.extension(), "mkv");
    }
    
    #[test]
    fn test_recorder_creation() {
        let config = RecorderConfig::default();
        let recorder = StreamRecorder::new(config);
        assert!(recorder.is_ok());
    }
    
    #[test]
    fn test_start_stop_recording() {
        let mut recorder = StreamRecorder::new(RecorderConfig::default()).unwrap();
        let metadata = RecordingMetadata {
            title: "Test Recording".to_string(),
            ..Default::default()
        };
        
        let path = recorder.start(metadata).unwrap();
        assert!(path.exists());
        
        recorder.stop().unwrap();
        assert!(!recorder.is_recording);
    }
    
    #[test]
    fn test_add_chapter() {
        let mut recorder = StreamRecorder::new(RecorderConfig::default()).unwrap();
        recorder.add_chapter("Chapter 1".to_string(), 0.0).unwrap();
        assert_eq!(recorder.chapters().len(), 1);
    }
}