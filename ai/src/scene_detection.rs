//! Scene detection and chapter generation module
//! 
//! Provides AI-powered scene detection including:
//! - Shot boundary detection
//! - Scene clustering
//! - Chapter generation
//! - Content analysis
//! - Keyframe extraction

use crate::{AIConfig, AIError, AIResult};
use crate::models::{AIModel, ModelType};
use crate::utils::{TensorOps, FeatureExtractor};
use image::{DynamicImage, GrayImage, Luma};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scene detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDetectionConfig {
    /// Minimum scene duration in seconds
    pub min_scene_duration_secs: f64,
    
    /// Maximum scene duration in seconds
    pub max_scene_duration_secs: f64,
    
    /// Threshold for shot boundary detection (0.0 - 1.0)
    pub shot_threshold: f32,
    
    /// Enable temporal smoothing
    pub enable_temporal_smoothing: bool,
    
    /// Temporal smoothing window size
    pub temporal_window_size: usize,
    
    /// Enable content analysis
    pub enable_content_analysis: bool,
    
    /// Number of keyframes per scene
    pub keyframes_per_scene: usize,
    
    /// Enable chapter generation
    pub enable_chapter_generation: bool,
    
    /// Chapter naming strategy
    pub chapter_naming: ChapterNamingStrategy,
}

impl Default for SceneDetectionConfig {
    fn default() -> Self {
        Self {
            min_scene_duration_secs: 5.0,
            max_scene_duration_secs: 300.0,
            shot_threshold: 0.3,
            enable_temporal_smoothing: true,
            temporal_window_size: 5,
            enable_content_analysis: true,
            keyframes_per_scene: 3,
            enable_chapter_generation: true,
            chapter_naming: ChapterNamingStrategy::Auto,
        }
    }
}

/// Chapter naming strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChapterNamingStrategy {
    /// Automatic naming based on content
    Auto,
    
    /// Numeric naming (Chapter 1, Chapter 2, etc.)
    Numeric,
    
    /// Time-based naming (00:00 - 05:30, etc.)
    TimeBased,
    
    /// Custom naming
    Custom,
}

/// Scene information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Scene start time in seconds
    pub start_time: f64,
    
    /// Scene end time in seconds
    pub end_time: f64,
    
    /// Scene duration in seconds
    pub duration: f64,
    
    /// Scene type
    pub scene_type: SceneType,
    
    /// Keyframes (timestamps in seconds)
    pub keyframes: Vec<f64>,
    
    /// Content features
    pub features: Option<SceneFeatures>,
    
    /// Scene description
    pub description: Option<String>,
}

/// Scene type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneType {
    /// Opening scene
    Opening,
    
    /// Dialogue scene
    Dialogue,
    
    /// Action scene
    Action,
    
    /// Montage
    Montage,
    
    /// Credits
    Credits,
    
    /// Unknown
    Unknown,
}

/// Scene features extracted by AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneFeatures {
    /// Dominant colors (RGB)
    pub dominant_colors: Vec<[u8; 3]>,
    
    /// Brightness level (0.0 - 1.0)
    pub brightness: f32,
    
    /// Motion level (0.0 - 1.0)
    pub motion: f32,
    
    /// Audio level (0.0 - 1.0)
    pub audio_level: f32,
    
    /// Face detection count
    pub face_count: usize,
    
    /// Text presence
    pub has_text: bool,
    
    /// Scene complexity (0.0 - 1.0)
    pub complexity: f32,
}

/// Chapter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Chapter index
    pub index: usize,
    
    /// Chapter start time in seconds
    pub start_time: f64,
    
    /// Chapter end time in seconds
    pub end_time: f64,
    
    /// Chapter title
    pub title: String,
    
    /// Chapter description
    pub description: Option<String>,
    
    /// Thumbnail timestamp
    pub thumbnail_time: f64,
}

/// Shot boundary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotBoundary {
    /// Frame index
    pub frame_index: usize,
    
    /// Timestamp in seconds
    pub timestamp: f64,
    
    /// Boundary type
    pub boundary_type: ShotBoundaryType,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// Shot boundary type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShotBoundaryType {
    /// Hard cut
    HardCut,
    
    /// Fade transition
    Fade,
    
    /// Dissolve transition
    Dissolve,
    
    /// Wipe transition
    Wipe,
    
    /// Unknown
    Unknown,
}

/// Scene detector
pub struct SceneDetector {
    config: SceneDetectionConfig,
    model: Option<AIModel>,
    feature_extractor: FeatureExtractor,
    frame_buffer: Vec<DynamicImage>,
    shot_boundaries: Vec<ShotBoundary>,
}

impl SceneDetector {
    /// Create a new scene detector
    pub fn new(ai_config: AIConfig) -> AIResult<Self> {
        let config = SceneDetectionConfig::default();
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            frame_buffer: Vec::new(),
            shot_boundaries: Vec::new(),
        })
    }
    
    /// Create a new scene detector with custom configuration
    pub fn with_config(config: SceneDetectionConfig, ai_config: AIConfig) -> AIResult<Self> {
        let feature_extractor = FeatureExtractor::new(ai_config)?;
        
        Ok(Self {
            config,
            model: None,
            feature_extractor,
            frame_buffer: Vec::new(),
            shot_boundaries: Vec::new(),
        })
    }
    
    /// Load the scene detection model
    pub fn load_model(&mut self) -> AIResult<()> {
        self.model = Some(AIModel::load(ModelType::SceneDetection)?);
        Ok(())
    }
    
    /// Process a frame for scene detection
    pub fn process_frame(&mut self, frame: &DynamicImage, timestamp: f64) -> AIResult<Option<ShotBoundary>> {
        // Add frame to buffer
        self.frame_buffer.push(frame.clone());
        
        // Maintain buffer size
        if self.frame_buffer.len() > self.config.temporal_window_size {
            self.frame_buffer.remove(0);
        }
        
        // Need at least 2 frames for comparison
        if self.frame_buffer.len() < 2 {
            return Ok(None);
        }
        
        // Calculate frame difference
        let diff = self.calculate_frame_difference(
            &self.frame_buffer[self.frame_buffer.len() - 2],
            &self.frame_buffer[self.frame_buffer.len() - 1],
        );
        
        // Check if this is a shot boundary
        if diff > self.config.shot_threshold {
            let boundary = ShotBoundary {
                frame_index: self.frame_buffer.len() - 1,
                timestamp,
                boundary_type: self.classify_boundary_type(diff),
                confidence: diff,
            };
            
            self.shot_boundaries.push(boundary.clone());
            Ok(Some(boundary))
        } else {
            Ok(None)
        }
    }
    
    /// Calculate frame difference
    fn calculate_frame_difference(&self, frame1: &DynamicImage, frame2: &DynamicImage) -> f32 {
        let gray1 = frame1.to_luma8();
        let gray2 = frame2.to_luma8();
        
        let mut total_diff = 0.0;
        let mut count = 0;
        
        for (p1, p2) in gray1.pixels().zip(gray2.pixels()) {
            let diff = (p1[0] as f32 - p2[0] as f32).abs();
            total_diff += diff;
            count += 1;
        }
        
        if count > 0 {
            total_diff / (count as f32 * 255.0)
        } else {
            0.0
        }
    }
    
    /// Classify boundary type
    fn classify_boundary_type(&self, diff: f32) -> ShotBoundaryType {
        if diff > 0.7 {
            ShotBoundaryType::HardCut
        } else if diff > 0.5 {
            ShotBoundaryType::Fade
        } else if diff > 0.4 {
            ShotBoundaryType::Dissolve
        } else {
            ShotBoundaryType::Unknown
        }
    }
    
    /// Detect scenes from shot boundaries
    pub fn detect_scenes(&self, duration: f64) -> AIResult<Vec<Scene>> {
        let mut scenes = Vec::new();
        
        if self.shot_boundaries.is_empty() {
            // Single scene
            scenes.push(Scene {
                start_time: 0.0,
                end_time: duration,
                duration,
                scene_type: SceneType::Unknown,
                keyframes: vec![duration / 2.0],
                features: None,
                description: None,
            });
            return Ok(scenes);
        }
        
        // Group shots into scenes
        let mut scene_start = 0.0;
        let mut shots_in_scene = 0;
        
        for boundary in &self.shot_boundaries {
            shots_in_scene += 1;
            
            // Check if we should end the scene
            let scene_duration = boundary.timestamp - scene_start;
            
            if scene_duration >= self.config.min_scene_duration_secs
                || shots_in_scene >= 5
                || boundary.timestamp - scene_start >= self.config.max_scene_duration_secs
            {
                // Create scene
                let scene = self.create_scene(
                    scene_start,
                    boundary.timestamp,
                    shots_in_scene,
                )?;
                scenes.push(scene);
                
                scene_start = boundary.timestamp;
                shots_in_scene = 0;
            }
        }
        
        // Add final scene
        if scene_start < duration {
            let scene = self.create_scene(
                scene_start,
                duration,
                shots_in_scene + 1,
            )?;
            scenes.push(scene);
        }
        
        Ok(scenes)
    }
    
    /// Create a scene from shots
    fn create_scene(&self, start: f64, end: f64, shot_count: usize) -> AIResult<Scene> {
        let duration = end - start;
        
        // Extract keyframes
        let keyframes = self.extract_keyframes(start, end, shot_count)?;
        
        // Classify scene type
        let scene_type = self.classify_scene_type(shot_count, duration);
        
        // Extract features if enabled
        let features = if self.config.enable_content_analysis {
            Some(self.extract_scene_features(start, end)?)
        } else {
            None
        };
        
        Ok(Scene {
            start_time: start,
            end_time: end,
            duration,
            scene_type,
            keyframes,
            features,
            description: None,
        })
    }
    
    /// Extract keyframes for a scene
    fn extract_keyframes(&self, start: f64, end: f64, shot_count: usize) -> AIResult<Vec<f64>> {
        let mut keyframes = Vec::new();
        let duration = end - start;
        
        if shot_count <= self.config.keyframes_per_scene {
            // Use shot boundaries as keyframes
            for boundary in &self.shot_boundaries {
                if boundary.timestamp >= start && boundary.timestamp <= end {
                    keyframes.push(boundary.timestamp);
                }
            }
        } else {
            // Distribute keyframes evenly
            let step = duration / (self.config.keyframes_per_scene as f64 + 1.0);
            for i in 1..=self.config.keyframes_per_scene {
                keyframes.push(start + step * i as f64);
            }
        }
        
        Ok(keyframes)
    }
    
    /// Classify scene type
    fn classify_scene_type(&self, shot_count: usize, duration: f64) -> SceneType {
        let shots_per_second = shot_count as f64 / duration;
        
        if shots_per_second > 0.5 {
            SceneType::Action
        } else if shots_per_second > 0.2 {
            SceneType::Montage
        } else if duration > 60.0 {
            SceneType::Dialogue
        } else {
            SceneType::Unknown
        }
    }
    
    /// Extract scene features
    fn extract_scene_features(&self, start: f64, end: f64) -> AIResult<SceneFeatures> {
        // Find frames in this scene
        let scene_frames: Vec<_> = self
            .frame_buffer
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                // Estimate timestamp from frame index
                let timestamp = *i as f64 / 30.0; // Assume 30 FPS
                timestamp >= start && timestamp <= end
            })
            .map(|(_, frame)| frame.clone())
            .collect();
        
        if scene_frames.is_empty() {
            return Ok(SceneFeatures {
                dominant_colors: vec![[128, 128, 128]],
                brightness: 0.5,
                motion: 0.0,
                audio_level: 0.0,
                face_count: 0,
                has_text: false,
                complexity: 0.0,
            });
        }
        
        // Calculate features
        let dominant_colors = self.extract_dominant_colors(&scene_frames)?;
        let brightness = self.calculate_brightness(&scene_frames);
        let motion = self.calculate_motion(&scene_frames);
        let complexity = self.calculate_complexity(&scene_frames);
        
        Ok(SceneFeatures {
            dominant_colors,
            brightness,
            motion,
            audio_level: 0.0, // Would need audio data
            face_count: 0,    // Would need face detection model
            has_text: false,  // Would need OCR
            complexity,
        })
    }
    
    /// Extract dominant colors
    fn extract_dominant_colors(&self, frames: &[DynamicImage]) -> AIResult<Vec<[u8; 3]>> {
        use std::collections::HashMap;
        
        let mut color_counts: HashMap<[u8; 3], usize> = HashMap::new();
        
        for frame in frames {
            let rgb = frame.to_rgb8();
            for pixel in rgb.pixels() {
                // Quantize colors
                let quantized = [
                    (pixel[0] / 32) * 32,
                    (pixel[1] / 32) * 32,
                    (pixel[2] / 32) * 32,
                ];
                *color_counts.entry(quantized).or_insert(0) += 1;
            }
        }
        
        // Get top 5 colors
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));
        
        Ok(colors
            .into_iter()
            .take(5)
            .map(|(color, _)| color)
            .collect())
    }
    
    /// Calculate average brightness
    fn calculate_brightness(&self, frames: &[DynamicImage]) -> f32 {
        let mut total_brightness = 0.0;
        let mut count = 0;
        
        for frame in frames {
            let gray = frame.to_luma8();
            for pixel in gray.pixels() {
                total_brightness += pixel[0] as f32;
                count += 1;
            }
        }
        
        if count > 0 {
            total_brightness / (count as f32 * 255.0)
        } else {
            0.5
        }
    }
    
    /// Calculate motion level
    fn calculate_motion(&self, frames: &[DynamicImage]) -> f32 {
        if frames.len() < 2 {
            return 0.0;
        }
        
        let mut total_motion = 0.0;
        let mut count = 0;
        
        for window in frames.windows(2) {
            let diff = self.calculate_frame_difference(&window[0], &window[1]);
            total_motion += diff;
            count += 1;
        }
        
        if count > 0 {
            total_motion / count as f32
        } else {
            0.0
        }
    }
    
    /// Calculate scene complexity
    fn calculate_complexity(&self, frames: &[DynamicImage]) -> f32 {
        if frames.is_empty() {
            return 0.0;
        }
        
        let mut total_edges = 0.0;
        let mut count = 0;
        
        for frame in frames {
            let gray = frame.to_luma8();
            let edges = self.detect_edges(&gray);
            total_edges += edges;
            count += 1;
        }
        
        if count > 0 {
            (total_edges / count as f32).min(1.0)
        } else {
            0.0
        }
    }
    
    /// Detect edges in grayscale image
    fn detect_edges(&self, image: &GrayImage) -> f32 {
        let mut edge_count = 0;
        let mut total = 0;
        
        let width = image.width() as usize;
        let height = image.height() as usize;
        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = image.get_pixel(x as u32, y as u32)[0] as f32;
                let left = image.get_pixel((x - 1) as u32, y as u32)[0] as f32;
                let right = image.get_pixel((x + 1) as u32, y as u32)[0] as f32;
                let top = image.get_pixel(x as u32, (y - 1) as u32)[0] as f32;
                let bottom = image.get_pixel(x as u32, (y + 1) as u32)[0] as f32;
                
                let gx = (right - left).abs();
                let gy = (bottom - top).abs();
                let magnitude = (gx * gx + gy * gy).sqrt();
                
                if magnitude > 30.0 {
                    edge_count += 1;
                }
                total += 1;
            }
        }
        
        if total > 0 {
            edge_count as f32 / total as f32
        } else {
            0.0
        }
    }
    
    /// Generate chapters from scenes
    pub fn generate_chapters(&self, scenes: &[Scene]) -> AIResult<Vec<Chapter>> {
        if !self.config.enable_chapter_generation {
            return Ok(Vec::new());
        }
        
        let mut chapters = Vec::new();
        
        for (index, scene) in scenes.iter().enumerate() {
            let title = match self.config.chapter_naming {
                ChapterNamingStrategy::Auto => {
                    self.generate_auto_title(scene, index)
                }
                ChapterNamingStrategy::Numeric => {
                    format!("Chapter {}", index + 1)
                }
                ChapterNamingStrategy::TimeBased => {
                    self.format_time(scene.start_time)
                }
                ChapterNamingStrategy::Custom => {
                    format!("Scene {}", index + 1)
                }
            };
            
            let chapter = Chapter {
                index,
                start_time: scene.start_time,
                end_time: scene.end_time,
                title,
                description: scene.description.clone(),
                thumbnail_time: scene.keyframes.first().copied().unwrap_or(scene.start_time),
            };
            
            chapters.push(chapter);
        }
        
        Ok(chapters)
    }
    
    /// Generate automatic chapter title
    fn generate_auto_title(&self, scene: &Scene, index: usize) -> String {
        match scene.scene_type {
            SceneType::Opening => "Opening".to_string(),
            SceneType::Dialogue => format!("Dialogue {}", index + 1),
            SceneType::Action => format!("Action Sequence {}", index + 1),
            SceneType::Montage => format!("Montage {}", index + 1),
            SceneType::Credits => "Credits".to_string(),
            SceneType::Unknown => format!("Scene {}", index + 1),
        }
    }
    
    /// Format time as MM:SS
    fn format_time(&self, seconds: f64) -> String {
        let mins = (seconds / 60.0) as u32;
        let secs = (seconds % 60.0) as u32;
        format!("{:02}:{:02}", mins, secs)
    }
    
    /// Reset the detector state
    pub fn reset(&mut self) {
        self.frame_buffer.clear();
        self.shot_boundaries.clear();
    }
    
    /// Get shot boundaries
    pub fn shot_boundaries(&self) -> &[ShotBoundary] {
        &self.shot_boundaries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scene_detection_config_default() {
        let config = SceneDetectionConfig::default();
        assert_eq!(config.min_scene_duration_secs, 5.0);
        assert_eq!(config.shot_threshold, 0.3);
    }
}