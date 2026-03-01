//! Video Tutorials
//! 
//! Provides video tutorial management with metadata, transcripts,
//! and interactive features.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// Video tutorial manager
pub struct VideoTutorialManager {
    /// Video tutorials directory
    videos_dir: PathBuf,
    
    /// Loaded video tutorials
    videos: HashMap<String, VideoTutorial>,
}

/// Video tutorial
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoTutorial {
    /// Video ID
    pub id: String,
    
    /// Video title
    pub title: String,
    
    /// Video description
    pub description: String,
    
    /// Video URL
    pub video_url: String,
    
    /// Thumbnail URL
    pub thumbnail_url: String,
    
    /// Duration in seconds
    pub duration: u64,
    
    /// Video category
    pub category: VideoCategory,
    
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    
    /// Transcript
    pub transcript: Option<Transcript>,
    
    /// Chapters
    pub chapters: Vec<Chapter>,
    
    /// Related videos
    pub related_videos: Vec<String>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Author
    pub author: String,
    
    /// Published date
    pub published_date: String,
    
    /// View count
    pub view_count: u64,
}

/// Video category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoCategory {
    /// Getting started
    GettingStarted,
    
    /// Core concepts
    CoreConcepts,
    
    /// Video processing
    Video,
    
    /// Audio processing
    Audio,
    
    /// Subtitles
    Subtitles,
    
    /// Plugins
    Plugins,
    
    /// Advanced topics
    Advanced,
    
    /// Troubleshooting
    Troubleshooting,
}

/// Difficulty level
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    /// Beginner
    Beginner,
    
    /// Intermediate
    Intermediate,
    
    /// Advanced
    Advanced,
    
    /// Expert
    Expert,
}

/// Transcript
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transcript {
    /// Transcript language
    pub language: String,
    
    /// Transcript segments
    pub segments: Vec<TranscriptSegment>,
}

/// Transcript segment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptSegment {
    /// Start time in seconds
    pub start: f64,
    
    /// End time in seconds
    pub end: f64,
    
    /// Text
    pub text: String,
}

/// Chapter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    /// Chapter title
    pub title: String,
    
    /// Start time in seconds
    pub start: f64,
    
    /// End time in seconds
    pub end: f64,
}

impl VideoTutorialManager {
    /// Create a new video tutorial manager
    pub fn new(videos_dir: PathBuf) -> Result<Self> {
        info!("🎬 Initializing Video Tutorial Manager");
        
        // Create videos directory if it doesn't exist
        std::fs::create_dir_all(&amp;videos_dir)?;
        
        // Load video tutorials
        let videos = Self::load_videos(&amp;videos_dir)?;
        
        info!("✅ Video tutorial manager initialized");
        info!("   - Videos directory: {}", videos_dir.display());
        info!("   - Loaded {} video tutorials", videos.len());
        
        Ok(Self {
            videos_dir,
            videos,
        })
    }
    
    /// Load video tutorials from directory
    fn load_videos(dir: &amp;Path) -> Result<HashMap<String, VideoTutorial>> {
        let mut videos = HashMap::new();
        
        if !dir.exists() {
            return Ok(videos);
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = std::fs::read_to_string(&amp;path)?;
                let video: VideoTutorial = serde_json::from_str(&amp;content)?;
                videos.insert(video.id.clone(), video);
            }
        }
        
        Ok(videos)
    }
    
    /// Generate video tutorials
    pub async fn generate(&amp;self) -> Result<()> {
        info!("🎬 Generating video tutorials");
        
        // Generate HTML for each video tutorial
        for (id, video) in &amp;self.videos {
            self.generate_video_html(video)?;
            debug!("✅ Generated video tutorial: {}", id);
        }
        
        // Generate index page
        self.generate_index()?;
        
        info!("✅ Video tutorials generated successfully");
        
        Ok(())
    }
    
    /// Generate video tutorial HTML
    fn generate_video_html(&amp;self, video: &amp;VideoTutorial) -> Result<()> {
        let output_path = self.videos_dir.join(format!("{}.html", video.id));
        
        // Generate HTML content
        let html = self.render_video(video)?;
        
        std::fs::write(&amp;output_path, html)?;
        
        Ok(())
    }
    
    /// Render video to HTML
    fn render_video(&amp;self, video: &amp;VideoTutorial) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&amp;video.title);
        html.push_str("</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/videos.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;video-container&quot;>\n");
        html.push_str("    <h1>");
        html.push_str(&amp;video.title);
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&amp;video.description);
        html.push_str("</p>\n");
        
        // Video player
        html.push_str("    <div class=&quot;video-player&quot;>\n");
        html.push_str("      <video controls>\n");
        html.push_str("        <source src=&quot;");
        html.push_str(&amp;video.video_url);
        html.push_str("&quot; type=&quot;video/mp4&quot;>\n");
        html.push_str("        Your browser does not support the video tag.\n");
        html.push_str("      </video>\n");
        html.push_str("    </div>\n");
        
        // Video metadata
        html.push_str("    <div class=&quot;video-meta&quot;>\n");
        html.push_str("      <span class=&quot;duration&quot;>");
        html.push_str(&amp;format!("{}:{:02}", video.duration / 60, video.duration % 60));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;category&quot;>");
        html.push_str(&amp;format!("{:?}", video.category));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;difficulty&quot;>");
        html.push_str(&amp;format!("{:?}", video.difficulty));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;views&quot;>");
        html.push_str(&amp;format!("{} views", video.view_count));
        html.push_str("</span>\n");
        html.push_str("    </div>\n");
        
        // Chapters
        if !video.chapters.is_empty() {
            html.push_str("    <div class=&quot;chapters&quot;>\n");
            html.push_str("      <h2>Chapters</h2>\n");
            html.push_str("      <ul>\n");
            for chapter in &amp;video.chapters {
                html.push_str("        <li><a href=&quot;#t=");
                html.push_str(&amp;chapter.start.to_string());
                html.push_str("&quot;>");
                html.push_str(&amp;chapter.title);
                html.push_str("</a></li>\n");
            }
            html.push_str("      </ul>\n");
            html.push_str("    </div>\n");
        }
        
        // Transcript
        if let Some(transcript) = &amp;video.transcript {
            html.push_str("    <div class=&quot;transcript&quot;>\n");
            html.push_str("      <h2>Transcript</h2>\n");
            for segment in &amp;transcript.segments {
                html.push_str("      <p class=&quot;segment&quot; data-start=&quot;");
                html.push_str(&amp;segment.start.to_string());
                html.push_str("&quot; data-end=&quot;");
                html.push_str(&amp;segment.end.to_string());
                html.push_str("&quot;>\n");
                html.push_str("        <span class=&quot;timestamp&quot;>");
                html.push_str(&amp;format!("{}:{:02}", segment.start as u64 / 60, segment.start as u64 % 60));
                html.push_str("</span>\n");
                html.push_str("        ");
                html.push_str(&amp;segment.text);
                html.push_str("\n");
                html.push_str("      </p>\n");
            }
            html.push_str("    </div>\n");
        }
        
        // Related videos
        if !video.related_videos.is_empty() {
            html.push_str("    <div class=&quot;related-videos&quot;>\n");
            html.push_str("      <h2>Related Videos</h2>\n");
            for related_id in &amp;video.related_videos {
                if let Some(related_video) = self.videos.get(related_id) {
                    html.push_str("      <div class=&quot;related-video&quot;>\n");
                    html.push_str("        <a href=&quot;");
                    html.push_str(related_id);
                    html.push_str(".html&quot;>\n");
                    html.push_str("          <img src=&quot;");
                    html.push_str(&amp;related_video.thumbnail_url);
                    html.push_str("&quot; alt=&quot;");
                    html.push_str(&amp;related_video.title);
                    html.push_str("&quot;>\n");
                    html.push_str("          <h3>");
                    html.push_str(&amp;related_video.title);
                    html.push_str("</h3>\n");
                    html.push_str("        </a>\n");
                    html.push_str("      </div>\n");
                }
            }
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/videos.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    /// Generate index page
    fn generate_index(&amp;self) -> Result<()> {
        let output_path = self.videos_dir.join("index.html");
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>Video Tutorials - Vantis Media Player</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/videos.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;videos-index&quot;>\n");
        html.push_str("    <h1>Video Tutorials</h1>\n");
        
        // Group by category
        let mut by_category: HashMap<VideoCategory, Vec<&amp;VideoTutorial>> = HashMap::new();
        for video in self.videos.values() {
            by_category.entry(video.category.clone()).or_default().push(video);
        }
        
        // Render categories
        for (category, videos) in by_category {
            html.push_str("    <div class=&quot;category&quot;>\n");
            html.push_str("      <h2>");
            html.push_str(&amp;format!("{:?}", category));
            html.push_str("</h2>\n");
            
            for video in videos {
                html.push_str("      <div class=&quot;video-card&quot;>\n");
                html.push_str("        <a href=&quot;");
                html.push_str(&amp;video.id);
                html.push_str(".html&quot;>\n");
                html.push_str("          <img src=&quot;");
                html.push_str(&amp;video.thumbnail_url);
                html.push_str("&quot; alt=&quot;");
                html.push_str(&amp;video.title);
                html.push_str("&quot;>\n");
                html.push_str("          <h3>");
                html.push_str(&amp;video.title);
                html.push_str("</h3>\n");
                html.push_str("          <p>");
                html.push_str(&amp;video.description);
                html.push_str("</p>\n");
                html.push_str("          <div class=&quot;meta&quot;>\n");
                html.push_str("            <span class=&quot;duration&quot;>");
                html.push_str(&amp;format!("{}:{:02}", video.duration / 60, video.duration % 60));
                html.push_str("</span>\n");
                html.push_str("            <span class=&quot;difficulty&quot;>");
                html.push_str(&amp;format!("{:?}", video.difficulty));
                html.push_str("</span>\n");
                html.push_str("          </div>\n");
                html.push_str("        </a>\n");
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&amp;output_path, html)?;
        
        Ok(())
    }
    
    /// Get video by ID
    pub fn get_video(&amp;self, id: &amp;str) -> Option<&amp;VideoTutorial> {
        self.videos.get(id)
    }
    
    /// Get all videos
    pub fn get_all_videos(&amp;self) -> Vec<&amp;VideoTutorial> {
        self.videos.values().collect()
    }
    
    /// Get videos by category
    pub fn get_videos_by_category(&amp;self, category: VideoCategory) -> Vec<&amp;VideoTutorial> {
        self.videos
            .values()
            .filter(|v| v.category == category)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_video_tutorial_manager_creation() {
        let dir = PathBuf::from("./video_tutorials");
        let manager = VideoTutorialManager::new(dir);
        assert!(manager.is_ok());
    }
}