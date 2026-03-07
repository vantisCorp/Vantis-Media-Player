//! Voice Configuration Module
//! 
//! Provides configuration options for voice recognition, synthesis, and providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main voice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Enable voice control
    pub enabled: bool,
    
    /// Recognition configuration
    pub recognition: RecognitionConfig,
    
    /// Synthesis configuration
    pub synthesis: SynthesisConfig,
    
    /// Provider configuration
    pub providers: ProvidersConfig,
    
    /// Wake word configuration
    pub wake_word: WakeWordConfig,
    
    /// Command processing configuration
    pub commands: CommandsConfig,
    
    /// Privacy settings
    pub privacy: PrivacyConfig,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            recognition: RecognitionConfig::default(),
            synthesis: SynthesisConfig::default(),
            providers: ProvidersConfig::default(),
            wake_word: WakeWordConfig::default(),
            commands: CommandsConfig::default(),
            privacy: PrivacyConfig::default(),
        }
    }
}

/// Speech recognition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionConfig {
    /// Recognition mode (local, cloud, hybrid)
    pub mode: RecognitionMode,
    
    /// Primary language for recognition
    pub language: String,
    
    /// Secondary languages for multi-language support
    pub secondary_languages: Vec<String>,
    
    /// Enable continuous listening
    pub continuous_listening: bool,
    
    /// Silence detection threshold (0.0 - 1.0)
    pub silence_threshold: f32,
    
    /// Silence duration to stop recording (ms)
    pub silence_duration_ms: u32,
    
    /// Maximum recording duration (ms)
    pub max_recording_duration_ms: u32,
    
    /// Enable automatic gain control
    pub auto_gain_control: bool,
    
    /// Noise suppression level (0.0 - 1.0)
    pub noise_suppression: f32,
    
    /// Echo cancellation enabled
    pub echo_cancellation: bool,
}

impl Default for RecognitionConfig {
    fn default() -> Self {
        Self {
            mode: RecognitionMode::Hybrid,
            language: "en-US".to_string(),
            secondary_languages: vec![],
            continuous_listening: false,
            silence_threshold: 0.15,
            silence_duration_ms: 1500,
            max_recording_duration_ms: 30000,
            auto_gain_control: true,
            noise_suppression: 0.7,
            echo_cancellation: true,
        }
    }
}

/// Recognition mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RecognitionMode {
    /// Local on-device recognition (privacy-focused, offline)
    Local,
    /// Cloud-based recognition (more accurate, requires internet)
    Cloud,
    /// Hybrid mode (local first, cloud fallback)
    Hybrid,
}

/// Speech synthesis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisConfig {
    /// Enable text-to-speech feedback
    pub enabled: bool,
    
    /// Default voice ID
    pub default_voice: Option<String>,
    
    /// Speech rate (0.5 - 2.0)
    pub rate: f32,
    
    /// Pitch adjustment (-50 to +50)
    pub pitch: i32,
    
    /// Volume (0.0 - 1.0)
    pub volume: f32,
    
    /// Enable audio caching
    pub cache_enabled: bool,
    
    /// Maximum cache size (entries)
    pub max_cache_size: usize,
    
    /// Synthesis provider preference
    pub provider: SynthesisProvider,
    
    /// Output audio format
    pub format: AudioOutputFormat,
    
    /// Enable SSML support
    pub ssml_enabled: bool,
    
    /// Feedback phrases customization
    pub feedback_phrases: HashMap<String, String>,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        let mut feedback_phrases = HashMap::new();
        feedback_phrases.insert("playing".to_string(), "Playing {title}".to_string());
        feedback_phrases.insert("paused".to_string(), "Paused".to_string());
        feedback_phrases.insert("stopped".to_string(), "Stopped".to_string());
        feedback_phrases.insert("volume_changed".to_string(), "Volume set to {level} percent".to_string());
        feedback_phrases.insert("next_track".to_string(), "Playing next track".to_string());
        feedback_phrases.insert("previous_track".to_string(), "Playing previous track".to_string());
        
        Self {
            enabled: true,
            default_voice: None,
            rate: 1.0,
            pitch: 0,
            volume: 0.8,
            cache_enabled: true,
            max_cache_size: 100,
            provider: SynthesisProvider::Auto,
            format: AudioOutputFormat::Wav,
            ssml_enabled: false,
            feedback_phrases,
        }
    }
}

/// Synthesis provider options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SynthesisProvider {
    /// Automatically select best available
    Auto,
    /// Local TTS only
    Local,
    /// AWS Polly
    AwsPolly,
    /// Google Cloud TTS
    GoogleCloudTTS,
    /// Azure Speech
    AzureSpeech,
    /// Custom provider
    Custom,
}

/// Audio output format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AudioOutputFormat {
    Wav,
    Mp3,
    Ogg,
    Flac,
    Pcm,
}

/// Voice providers configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersConfig {
    /// Alexa integration
    pub alexa: AlexaConfig,
    
    /// Google Assistant integration
    pub google_assistant: GoogleAssistantConfig,
    
    /// Siri integration (macOS/iOS)
    pub siri: SiriConfig,
    
    /// Local voice assistant
    pub local: LocalVoiceConfig,
    
    /// Preferred provider
    pub preferred_provider: PreferredProvider,
}

impl Default for ProvidersConfig {
    fn default() -> Self {
        Self {
            alexa: AlexaConfig::default(),
            google_assistant: GoogleAssistantConfig::default(),
            siri: SiriConfig::default(),
            local: LocalVoiceConfig::default(),
            preferred_provider: PreferredProvider::Auto,
        }
    }
}

/// Preferred voice provider
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PreferredProvider {
    Auto,
    Alexa,
    GoogleAssistant,
    Siri,
    Local,
}

/// Alexa integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaConfig {
    /// Enable Alexa integration
    pub enabled: bool,
    
    /// Alexa skill ID
    pub skill_id: Option<String>,
    
    /// Enable push notifications
    pub push_notifications: bool,
    
    /// Available skills
    pub skills: Vec<String>,
}

impl Default for AlexaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            skill_id: None,
            push_notifications: true,
            skills: vec!["VantisMediaPlayer".to_string()],
        }
    }
}

/// Google Assistant integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleAssistantConfig {
    /// Enable Google Assistant integration
    pub enabled: bool,
    
    /// Action package ID
    pub action_id: Option<String>,
    
    /// Project ID
    pub project_id: Option<String>,
    
    /// Enable push notifications
    pub push_notifications: bool,
}

impl Default for GoogleAssistantConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            action_id: None,
            project_id: None,
            push_notifications: true,
        }
    }
}

/// Siri integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriConfig {
    /// Enable Siri integration
    pub enabled: bool,
    
    /// Siri shortcut name
    pub shortcut_name: Option<String>,
    
    /// Enable Siri suggestions
    pub suggestions: bool,
    
    /// Custom intents bundle ID
    pub intents_bundle_id: Option<String>,
}

impl Default for SiriConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            shortcut_name: None,
            suggestions: true,
            intents_bundle_id: None,
        }
    }
}

/// Local voice assistant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalVoiceConfig {
    /// Enable local voice assistant
    pub enabled: bool,
    
    /// Wake word to activate
    pub wake_word: String,
    
    /// Wake word sensitivity (0.0 - 1.0)
    pub wake_word_sensitivity: f32,
    
    /// Enable always-on listening
    pub always_on: bool,
    
    /// Use GPU acceleration when available
    pub gpu_acceleration: bool,
    
    /// Model path for local recognition
    pub model_path: Option<String>,
}

impl Default for LocalVoiceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            wake_word: "hey vantis".to_string(),
            wake_word_sensitivity: 0.5,
            always_on: false,
            gpu_acceleration: true,
            model_path: None,
        }
    }
}

/// Wake word configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Enable wake word detection
    pub enabled: bool,
    
    /// Wake word phrase
    pub phrase: String,
    
    /// Sensitivity (0.0 - 1.0, higher = more sensitive)
    pub sensitivity: f32,
    
    /// Require confirmation before executing
    pub require_confirmation: bool,
    
    /// Timeout for command after wake word (ms)
    pub command_timeout_ms: u32,
    
    /// Play sound on wake word detected
    pub play_sound: bool,
    
    /// Sound file path
    pub sound_path: Option<String>,
}

impl Default for WakeWordConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            phrase: "hey vantis".to_string(),
            sensitivity: 0.5,
            require_confirmation: false,
            command_timeout_ms: 5000,
            play_sound: true,
            sound_path: None,
        }
    }
}

/// Commands configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsConfig {
    /// Enable all voice commands
    pub enabled: bool,
    
    /// Enable playback commands
    pub playback_commands: bool,
    
    /// Enable volume commands
    pub volume_commands: bool,
    
    /// Enable navigation commands
    pub navigation_commands: bool,
    
    /// Enable search commands
    pub search_commands: bool,
    
    /// Enable playlist commands
    pub playlist_commands: bool,
    
    /// Enable settings commands
    pub settings_commands: bool,
    
    /// Enable information queries
    pub info_commands: bool,
    
    /// Custom command aliases
    pub aliases: HashMap<String, String>,
    
    /// Enable fuzzy matching for commands
    pub fuzzy_matching: bool,
    
    /// Minimum confidence threshold for command matching (0.0 - 1.0)
    pub confidence_threshold: f32,
}

impl Default for CommandsConfig {
    fn default() -> Self {
        let mut aliases = HashMap::new();
        aliases.insert("play music".to_string(), "play".to_string());
        aliases.insert("turn it up".to_string(), "increase volume".to_string());
        aliases.insert("turn it down".to_string(), "decrease volume".to_string());
        aliases.insert("skip".to_string(), "next".to_string());
        aliases.insert("go back".to_string(), "previous".to_string());
        
        Self {
            enabled: true,
            playback_commands: true,
            volume_commands: true,
            navigation_commands: true,
            search_commands: true,
            playlist_commands: true,
            settings_commands: true,
            info_commands: true,
            aliases,
            fuzzy_matching: true,
            confidence_threshold: 0.7,
        }
    }
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Store voice recordings locally
    pub store_recordings: bool,
    
    /// Send voice data to cloud providers
    pub allow_cloud_processing: bool,
    
    /// Retention period for voice history (days)
    pub retention_days: u32,
    
    /// Anonymize voice data
    pub anonymize: bool,
    
    /// Require explicit consent for cloud features
    pub require_consent: bool,
    
    /// Delete voice data on exit
    pub delete_on_exit: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            store_recordings: false,
            allow_cloud_processing: false,
            retention_days: 30,
            anonymize: true,
            require_consent: true,
            delete_on_exit: true,
        }
    }
}

/// Voice feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceFeatures {
    /// Wake word detection available
    pub wake_word_available: bool,
    
    /// Local recognition available
    pub local_recognition_available: bool,
    
    /// Cloud recognition available
    pub cloud_recognition_available: bool,
    
    /// Local synthesis available
    pub local_synthesis_available: bool,
    
    /// Cloud synthesis available
    pub cloud_synthesis_available: bool,
    
    /// Alexa integration available
    pub alexa_available: bool,
    
    /// Google Assistant available
    pub google_assistant_available: bool,
    
    /// Siri available (macOS/iOS)
    pub siri_available: bool,
}

impl Default for VoiceFeatures {
    fn default() -> Self {
        Self {
            wake_word_available: true,
            local_recognition_available: true,
            cloud_recognition_available: false,
            local_synthesis_available: true,
            cloud_synthesis_available: false,
            alexa_available: false,
            google_assistant_available: false,
            siri_available: cfg!(target_os = "macos") || cfg!(target_os = "ios"),
        }
    }
}

impl VoiceConfig {
    /// Create a new configuration with privacy-focused defaults
    pub fn privacy_focused() -> Self {
        Self {
            enabled: true,
            recognition: RecognitionConfig {
                mode: RecognitionMode::Local,
                ..Default::default()
            },
            synthesis: SynthesisConfig {
                provider: SynthesisProvider::Local,
                ..Default::default()
            },
            providers: ProvidersConfig {
                preferred_provider: PreferredProvider::Local,
                local: LocalVoiceConfig {
                    enabled: true,
                    always_on: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            privacy: PrivacyConfig {
                store_recordings: false,
                allow_cloud_processing: false,
                delete_on_exit: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Create configuration optimized for accuracy
    pub fn accuracy_optimized() -> Self {
        Self {
            enabled: true,
            recognition: RecognitionConfig {
                mode: RecognitionMode::Cloud,
                ..Default::default()
            },
            synthesis: SynthesisConfig {
                provider: SynthesisProvider::Auto,
                ..Default::default()
            },
            providers: ProvidersConfig {
                preferred_provider: PreferredProvider::Auto,
                ..Default::default()
            },
            privacy: PrivacyConfig {
                allow_cloud_processing: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Load configuration from file
    pub fn load(path: &str) -> Result<Self, VoiceConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| VoiceConfigError::IoError(e.to_string()))?;
        
        let config: VoiceConfig = if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| VoiceConfigError::ParseError(e.to_string()))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| VoiceConfigError::ParseError(e.to_string()))?
        };
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<(), VoiceConfigError> {
        let content = if path.ends_with(".toml") {
            toml::to_string_pretty(self)
                .map_err(|e| VoiceConfigError::SerializeError(e.to_string()))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| VoiceConfigError::SerializeError(e.to_string()))?
        };
        
        std::fs::write(path, content)
            .map_err(|e| VoiceConfigError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), VoiceConfigError> {
        // Validate recognition settings
        if self.recognition.silence_threshold < 0.0 || self.recognition.silence_threshold > 1.0 {
            return Err(VoiceConfigError::ValidationError(
                "silence_threshold must be between 0.0 and 1.0".to_string()
            ));
        }
        
        if self.recognition.noise_suppression < 0.0 || self.recognition.noise_suppression > 1.0 {
            return Err(VoiceConfigError::ValidationError(
                "noise_suppression must be between 0.0 and 1.0".to_string()
            ));
        }
        
        // Validate synthesis settings
        if self.synthesis.rate < 0.5 || self.synthesis.rate > 2.0 {
            return Err(VoiceConfigError::ValidationError(
                "synthesis rate must be between 0.5 and 2.0".to_string()
            ));
        }
        
        if self.synthesis.pitch < -50 || self.synthesis.pitch > 50 {
            return Err(VoiceConfigError::ValidationError(
                "synthesis pitch must be between -50 and 50".to_string()
            ));
        }
        
        if self.synthesis.volume < 0.0 || self.synthesis.volume > 1.0 {
            return Err(VoiceConfigError::ValidationError(
                "synthesis volume must be between 0.0 and 1.0".to_string()
            ));
        }
        
        // Validate wake word settings
        if self.wake_word.sensitivity < 0.0 || self.wake_word.sensitivity > 1.0 {
            return Err(VoiceConfigError::ValidationError(
                "wake_word sensitivity must be between 0.0 and 1.0".to_string()
            ));
        }
        
        // Validate command settings
        if self.commands.confidence_threshold < 0.0 || self.commands.confidence_threshold > 1.0 {
            return Err(VoiceConfigError::ValidationError(
                "confidence_threshold must be between 0.0 and 1.0".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Voice configuration error types
#[derive(Debug, thiserror::Error)]
pub enum VoiceConfigError {
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Serialize error: {0}")]
    SerializeError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = VoiceConfig::default();
        assert!(config.enabled);
        assert!(config.recognition.auto_gain_control);
        assert!(config.synthesis.enabled);
    }
    
    #[test]
    fn test_privacy_focused_config() {
        let config = VoiceConfig::privacy_focused();
        assert_eq!(config.recognition.mode, RecognitionMode::Local);
        assert!(!config.privacy.allow_cloud_processing);
    }
    
    #[test]
    fn test_accuracy_optimized_config() {
        let config = VoiceConfig::accuracy_optimized();
        assert_eq!(config.recognition.mode, RecognitionMode::Cloud);
        assert!(config.privacy.allow_cloud_processing);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = VoiceConfig::default();
        assert!(config.validate().is_ok());
        
        config.synthesis.volume = 1.5;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_recognition_config_defaults() {
        let config = RecognitionConfig::default();
        assert_eq!(config.mode, RecognitionMode::Hybrid);
        assert_eq!(config.language, "en-US");
        assert!(config.echo_cancellation);
    }
    
    #[test]
    fn test_synthesis_config_defaults() {
        let config = SynthesisConfig::default();
        assert!(config.enabled);
        assert_eq!(config.rate, 1.0);
        assert!(config.cache_enabled);
    }
    
    #[test]
    fn test_wake_word_config() {
        let config = WakeWordConfig::default();
        assert!(config.enabled);
        assert_eq!(config.phrase, "hey vantis");
    }
    
    #[test]
    fn test_commands_config() {
        let config = CommandsConfig::default();
        assert!(config.fuzzy_matching);
        assert!(config.aliases.contains_key("play music"));
    }
}