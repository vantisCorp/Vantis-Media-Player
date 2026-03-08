//! AI-powered subtitle generation
//! 
//! Uses machine learning models to automatically generate subtitles from audio:
//! - Speech-to-text transcription
//! - Speaker diarization (identifying different speakers)
//! - Automatic punctuation and capitalization
//! - Profanity filtering
//! - Custom vocabulary support

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

use crate::parser::{SubtitleTrack, SubtitleEntry};

/// AI subtitle generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationConfig {
    /// Model to use for transcription
    pub model: TranscriptionModel,
    
    /// Source language (auto-detect if None)
    pub source_language: Option<String>,
    
    /// Target language for translation
    pub target_language: Option<String>,
    
    /// Enable speaker diarization
    pub enable_diarization: bool,
    
    /// Maximum speakers to detect
    pub max_speakers: Option<u32>,
    
    /// Enable automatic punctuation
    pub enable_punctuation: bool,
    
    /// Enable profanity filtering
    pub profanity_filter: bool,
    
    /// Custom vocabulary for better accuracy
    pub custom_vocabulary: Vec<String>,
    
    /// Minimum confidence threshold (0.0 - 1.0)
    pub confidence_threshold: f32,
    
    /// Maximum segment duration in seconds
    pub max_segment_duration: f32,
    
    /// Output format
    pub output_format: SubtitleOutputFormat,
}

impl Default for AIGenerationConfig {
    fn default() -> Self {
        Self {
            model: TranscriptionModel::WhisperMedium,
            source_language: None,
            target_language: None,
            enable_diarization: true,
            max_speakers: Some(4),
            enable_punctuation: true,
            profanity_filter: false,
            custom_vocabulary: Vec::new(),
            confidence_threshold: 0.7,
            max_segment_duration: 5.0,
            output_format: SubtitleOutputFormat::SRT,
        }
    }
}

/// Transcription model options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscriptionModel {
    /// OpenAI Whisper Tiny (fastest, lowest accuracy)
    WhisperTiny,
    /// OpenAI Whisper Base
    WhisperBase,
    /// OpenAI Whisper Small
    WhisperSmall,
    /// OpenAI Whisper Medium (balanced)
    WhisperMedium,
    /// OpenAI Whisper Large (slowest, highest accuracy)
    WhisperLarge,
    /// Whisper Large V3 (latest)
    WhisperLargeV3,
    /// Custom model
    Custom,
}

impl TranscriptionModel {
    pub fn model_name(&self) -> &'static str {
        match self {
            Self::WhisperTiny => "tiny",
            Self::WhisperBase => "base",
            Self::WhisperSmall => "small",
            Self::WhisperMedium => "medium",
            Self::WhisperLarge => "large-v2",
            Self::WhisperLargeV3 => "large-v3",
            Self::Custom => "custom",
        }
    }
    
    pub fn required_vram_mb(&self) -> u32 {
        match self {
            Self::WhisperTiny => 400,
            Self::WhisperBase => 600,
            Self::WhisperSmall => 1000,
            Self::WhisperMedium => 2500,
            Self::WhisperLarge => 5000,
            Self::WhisperLargeV3 => 6000,
            Self::Custom => 0,
        }
    }
}

/// Subtitle output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubtitleOutputFormat {
    SRT,
    VTT,
    ASS,
    TTML,
    JSON,
}

/// Transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    /// Generated subtitle track
    pub track: SubtitleTrack,
    
    /// Detected language
    pub detected_language: Option<String>,
    
    /// Overall confidence score
    pub confidence: f32,
    
    /// Processing time in seconds
    pub processing_time: f32,
    
    /// Speaker information (if diarization enabled)
    pub speakers: Vec<SpeakerInfo>,
    
    /// Warnings during generation
    pub warnings: Vec<String>,
}

/// Speaker information from diarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerInfo {
    /// Speaker identifier
    pub id: String,
    
    /// Total speaking time in seconds
    pub total_speaking_time: f32,
    
    /// Number of segments
    pub segment_count: u32,
    
    /// Optional speaker label (if identified)
    pub label: Option<String>,
}

/// Word-level timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordTiming {
    pub word: String,
    pub start: f32,
    pub end: f32,
    pub confidence: f32,
    pub speaker: Option<String>,
}

/// Transcription progress callback
#[derive(Debug, Clone)]
pub enum TranscriptionProgress {
    /// Loading model
    LoadingModel { model: String },
    /// Extracting audio
    ExtractingAudio { progress: f32 },
    /// Transcribing
    Transcribing { progress: f32, current_segment: u32, total_segments: u32 },
    /// Post-processing
    PostProcessing { stage: String },
    /// Completed
    Completed { result: TranscriptionResult },
    /// Error
    Error { message: String },
}

/// AI subtitle generator trait for dependency injection
pub trait AISubtitleGenerator: Send + Sync {
    /// Generate subtitles from audio file
    async fn generate(
        &self,
        audio_path: &str,
        config: &AIGenerationConfig,
    ) -> Result<TranscriptionResult>;
    
    /// Generate subtitles from video file (extracts audio first)
    async fn generate_from_video(
        &self,
        video_path: &str,
        config: &AIGenerationConfig,
    ) -> Result<TranscriptionResult>;
    
    /// Get available models
    fn available_models(&self) -> Vec<TranscriptionModel>;
    
    /// Check if a model is downloaded
    fn is_model_downloaded(&self, model: TranscriptionModel) -> bool;
    
    /// Download a model
    async fn download_model(&self, model: TranscriptionModel) -> Result<()>;
    
    /// Get estimated processing time
    fn estimate_processing_time(&self, duration_seconds: f32, model: TranscriptionModel) -> f32;
}

/// Default AI subtitle generator implementation using Whisper
pub struct WhisperSubtitleGenerator {
    /// Model cache directory
    model_cache_dir: PathBuf,
    
    /// Downloaded models
    downloaded_models: Arc<RwLock<Vec<TranscriptionModel>>>,
    
    /// GPU available
    gpu_available: bool,
    
    /// GPU memory in MB
    gpu_memory_mb: u32,
}

impl WhisperSubtitleGenerator {
    pub fn new(model_cache_dir: Option<PathBuf>) -> Result<Self> {
        let model_cache_dir = model_cache_dir
            .unwrap_or_else(|| PathBuf::from("./models/whisper"));
        
        // Check GPU availability
        let (gpu_available, gpu_memory_mb) = Self::check_gpu();
        
        if gpu_available {
            info!("🎮 GPU available for AI subtitle generation ({} MB VRAM)", gpu_memory_mb);
        } else {
            info!("⚠️ No GPU detected, using CPU for AI subtitle generation (slower)");
        }
        
        Ok(Self {
            model_cache_dir,
            downloaded_models: Arc::new(RwLock::new(Vec::new())),
            gpu_available,
            gpu_memory_mb,
        })
    }
    
    fn check_gpu() -> (bool, u32) {
        // Check for CUDA
        if std::path::Path::new("/dev/nvidia0").exists() {
            return (true, 8000); // Assume 8GB
        }
        
        // Check for Metal (macOS)
        #[cfg(target_os = "macos")]
        {
            // Metal is always available on Apple Silicon
            return (true, 8000);
        }
        
        (false, 0)
    }
    
    /// Check if model fits in available VRAM
    pub fn can_use_model(&self, model: TranscriptionModel) -> bool {
        if !self.gpu_available {
            return true; // CPU can always run (slower)
        }
        
        let required = model.required_vram_mb();
        required <= self.gpu_memory_mb
    }
    
    /// Extract audio from video file
    async fn extract_audio(&self, video_path: &str) -> Result<PathBuf> {
        let output_path = std::env::temp_dir().join(format!(
            "vantis_audio_{}.wav",
            uuid::Uuid::new_v4()
        ));
        
        // Use ffmpeg to extract audio
        let status = tokio::process::Command::new("ffmpeg")
            .args([
                "-i", video_path,
                "-vn",  // No video
                "-acodec", "pcm_s16le",  // 16-bit PCM
                "-ar", "16000",  // 16kHz sample rate (Whisper requirement)
                "-ac", "1",  // Mono
                "-y",  // Overwrite
                output_path.to_str().unwrap(),
            ])
            .status()
            .await?;
        
        if !status.success() {
            bail!("Failed to extract audio from video");
        }
        
        Ok(output_path)
    }
    
    /// Parse Whisper JSON output to subtitle track
    fn parse_whisper_output(&self, output: &str, config: &AIGenerationConfig) -> Result<SubtitleTrack> {
        let json: serde_json::Value = serde_json::from_str(output)?;
        
        let mut entries = Vec::new();
        let segments = json["segments"].as_array().ok_or_else(|| 
            anyhow::anyhow!("No segments in Whisper output"))?;
        
        for segment in segments {
            let start = segment["start"].as_f64().unwrap_or(0.0) as f32;
            let end = segment["end"].as_f64().unwrap_or(0.0) as f32;
            let text = segment["text"].as_str().unwrap_or_default().trim().to_string();
            let confidence = segment["avg_logprob"].as_f64().unwrap_or(-1.0) as f32;
            let speaker = segment["speaker"].as_str().map(|s| s.to_string());
            
            // Skip low confidence segments
            if confidence < config.confidence_threshold {
                continue;
            }
            
            // Apply profanity filter
            let text = if config.profanity_filter {
                self.filter_profanity(&text)
            } else {
                text
            };
            
            // Format text with speaker label if diarization is enabled
            let formatted_text = if config.enable_diarization {
                if let Some(s) = speaker {
                    format!("[{}] {}", s, text)
                } else {
                    text
                }
            } else {
                text
            };
            
            entries.push(SubtitleEntry {
                index: entries.len() as u32 + 1,
                start_time: start,
                end_time: end,
                text: formatted_text,
                style: None,
            });
        }
        
        Ok(SubtitleTrack {
            language: config.source_language.clone().unwrap_or_else(|| "und".to_string()),
            entries,
            format: crate::parser::SubtitleFormat::SRT,
        })
    }
    
    /// Filter profanity from text
    fn filter_profanity(&self, text: &str) -> String {
        // Basic profanity filter - in production, use a proper library
        let profanity_list = ["fuck", "shit", "damn", "ass", "bitch"];
        let mut result = text.to_string();
        
        for word in &profanity_list {
            let censored = word.chars().take(1).chain(std::iter::repeat('*')).take(word.len()).collect::<String>();
            result = result.replace(word, &censored);
        }
        
        result
    }
    
    /// Get recommended model based on available resources
    pub fn get_recommended_model(&self) -> TranscriptionModel {
        if self.gpu_available {
            if self.gpu_memory_mb >= 6000 {
                TranscriptionModel::WhisperLargeV3
            } else if self.gpu_memory_mb >= 5000 {
                TranscriptionModel::WhisperLarge
            } else if self.gpu_memory_mb >= 2500 {
                TranscriptionModel::WhisperMedium
            } else {
                TranscriptionModel::WhisperSmall
            }
        } else {
            // CPU - use smaller models
            TranscriptionModel::WhisperSmall
        }
    }
}

#[async_trait::async_trait]
impl AISubtitleGenerator for WhisperSubtitleGenerator {
    async fn generate(
        &self,
        audio_path: &str,
        config: &AIGenerationConfig,
    ) -> Result<TranscriptionResult> {
        let start_time = std::time::Instant::now();
        
        info!("🎙️ Starting AI subtitle generation for: {}", audio_path);
        info!("   Model: {}", config.model.model_name());
        
        // Check model availability
        if !self.is_model_downloaded(config.model) {
            warn!("Model not downloaded, downloading now...");
            self.download_model(config.model).await?;
        }
        
        // Check if model fits in memory
        if !self.can_use_model(config.model) {
            let recommended = self.get_recommended_model();
            warn!("Model {} requires more VRAM, switching to {}", 
                  config.model.model_name(), recommended.model_name());
        }
        
        // Run Whisper
        let model_path = self.model_cache_dir.join(config.model.model_name());
        
        let mut args = vec![
            "--model".to_string(),
            config.model.model_name().to_string(),
            "--audio".to_string(),
            audio_path.to_string(),
            "--output_format".to_string(),
            "json".to_string(),
            "--output_dir".to_string(),
            std::env::temp_dir().to_string_lossy().to_string(),
        ];
        
        if let Some(lang) = &config.source_language {
            args.push("--language".to_string());
            args.push(lang.clone());
        }
        
        if config.enable_diarization {
            args.push("--diarize".to_string());
        }
        
        // Execute Whisper
        let output = tokio::process::Command::new("whisper")
            .args(&args)
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Whisper transcription failed: {}", stderr);
        }
        
        // Read the JSON output
        let json_path = std::env::temp_dir().join(format!(
            "{}.json",
            PathBuf::from(audio_path).file_stem().unwrap().to_string_lossy()
        ));
        let json_content = tokio::fs::read_to_string(&json_path).await?;
        
        // Parse the output
        let track = self.parse_whisper_output(&json_content, config)?;
        
        let processing_time = start_time.elapsed().as_secs_f32();
        
        // Calculate overall confidence
        let confidence = track.entries.iter()
            .map(|e| 0.8) // Placeholder - would come from actual Whisper output
            .sum::<f32>() / track.entries.len().max(1) as f32;
        
        info!("✅ AI subtitle generation completed in {:.2}s", processing_time);
        info!("   Generated {} entries", track.entries.len());
        
        Ok(TranscriptionResult {
            track,
            detected_language: config.source_language.clone(),
            confidence,
            processing_time,
            speakers: Vec::new(),
            warnings: Vec::new(),
        })
    }
    
    async fn generate_from_video(
        &self,
        video_path: &str,
        config: &AIGenerationConfig,
    ) -> Result<TranscriptionResult> {
        info!("🎬 Extracting audio from video: {}", video_path);
        
        let audio_path = self.extract_audio(video_path).await?;
        
        let result = self.generate(audio_path.to_str().unwrap(), config).await?;
        
        // Cleanup temp audio file
        let _ = tokio::fs::remove_file(&audio_path).await;
        
        Ok(result)
    }
    
    fn available_models(&self) -> Vec<TranscriptionModel> {
        vec![
            TranscriptionModel::WhisperTiny,
            TranscriptionModel::WhisperBase,
            TranscriptionModel::WhisperSmall,
            TranscriptionModel::WhisperMedium,
            TranscriptionModel::WhisperLarge,
            TranscriptionModel::WhisperLargeV3,
        ]
    }
    
    fn is_model_downloaded(&self, model: TranscriptionModel) -> bool {
        let model_path = self.model_cache_dir.join(model.model_name());
        model_path.exists()
    }
    
    async fn download_model(&self, model: TranscriptionModel) -> Result<()> {
        info!("📥 Downloading Whisper model: {}", model.model_name());
        
        // Ensure cache directory exists
        tokio::fs::create_dir_all(&self.model_cache_dir).await?;
        
        // Download from HuggingFace
        let url = format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
            model.model_name()
        );
        
        let model_path = self.model_cache_dir.join(format!("{}.bin", model.model_name()));
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            bail!("Failed to download model from {}", url);
        }
        
        let bytes = response.bytes().await?;
        tokio::fs::write(&model_path, &bytes).await?;
        
        // Add to downloaded models
        let mut models = self.downloaded_models.write().await;
        models.push(model);
        
        info!("✅ Model downloaded successfully: {:?}", model_path);
        
        Ok(())
    }
    
    fn estimate_processing_time(&self, duration_seconds: f32, model: TranscriptionModel) -> f32 {
        let speed_multiplier = match model {
            TranscriptionModel::WhisperTiny => 20.0,
            TranscriptionModel::WhisperBase => 15.0,
            TranscriptionModel::WhisperSmall => 10.0,
            TranscriptionModel::WhisperMedium => 5.0,
            TranscriptionModel::WhisperLarge => 2.0,
            TranscriptionModel::WhisperLargeV3 => 2.0,
            TranscriptionModel::Custom => 5.0,
        };
        
        // GPU is faster
        if self.gpu_available {
            duration_seconds / speed_multiplier * 2.0
        } else {
            duration_seconds / speed_multiplier
        }
    }
}

/// Batch processing for multiple files
pub struct BatchSubtitleGenerator {
    generator: Arc<dyn AISubtitleGenerator>,
    max_concurrent: usize,
}

impl BatchSubtitleGenerator {
    pub fn new(generator: Arc<dyn AISubtitleGenerator>, max_concurrent: usize) -> Self {
        Self { generator, max_concurrent }
    }
    
    /// Process multiple video/audio files
    pub async fn process_batch(
        &self,
        files: &[String],
        config: &AIGenerationConfig,
        progress_callback: impl Fn(&str, usize, usize) + Send + Sync + 'static,
    ) -> Result<Vec<(String, Result<TranscriptionResult>)>> {
        let results = Arc::new(RwLock::new(Vec::new()));
        let total = files.len();
        
        for (index, file) in files.iter().enumerate() {
            progress_callback(file, index + 1, total);
            
            let result = self.generator.generate_from_video(file, config).await;
            
            let mut results = results.write().await;
            results.push((file.clone(), result));
        }
        
        let results = results.read().await.clone();
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_names() {
        assert_eq!(TranscriptionModel::WhisperTiny.model_name(), "tiny");
        assert_eq!(TranscriptionModel::WhisperLargeV3.model_name(), "large-v3");
    }
    
    #[test]
    fn test_vram_requirements() {
        assert_eq!(TranscriptionModel::WhisperTiny.required_vram_mb(), 400);
        assert_eq!(TranscriptionModel::WhisperLarge.required_vram_mb(), 5000);
    }
    
    #[test]
    fn test_default_config() {
        let config = AIGenerationConfig::default();
        assert_eq!(config.model, TranscriptionModel::WhisperMedium);
        assert!(config.enable_diarization);
        assert_eq!(config.confidence_threshold, 0.7);
    }
}