//! Voice assistant providers

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::commands::VoiceCommand;

/// Supported voice assistants
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoiceAssistant {
    Alexa,
    GoogleAssistant,
    Siri,
    Local,
}

/// Voice provider trait
#[async_trait]
pub trait VoiceProvider: Send + Sync {
    /// Get the assistant type
    fn assistant(&self) -> VoiceAssistant;
    
    /// Process incoming voice request
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse>;
    
    /// Check if the provider is available
    fn is_available(&self) -> bool;
    
    /// Get provider name
    fn name(&self) -> &str;
}

/// Voice request from a voice assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceRequest {
    /// Request ID
    pub id: String,
    
    /// Source assistant
    pub assistant: VoiceAssistant,
    
    /// User ID
    pub user_id: Option<String>,
    
    /// Device ID
    pub device_id: Option<String>,
    
    /// Raw transcript
    pub transcript: Option<String>,
    
    /// Parsed intent (platform-specific)
    pub intent: Option<serde_json::Value>,
    
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Locale/language
    pub locale: String,
    
    /// New session flag
    pub new_session: bool,
}

/// Voice response to a voice assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    /// Response version
    pub version: String,
    
    /// Response body
    pub response: ResponseBody,
    
    /// Session attributes
    pub session_attributes: serde_json::Value,
}

/// Response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody {
    /// Output speech
    pub output_speech: Option<OutputSpeech>,
    
    /// Card (visual display)
    pub card: Option<Card>,
    
    /// Reprompt speech
    pub reprompt: Option<Reprompt>,
    
    /// Should end session
    pub should_end_session: bool,
    
    /// Directives (for Alexa)
    pub directives: Vec<Directive>,
}

/// Output speech types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSpeech {
    #[serde(rename = "type")]
    pub speech_type: SpeechType,
    pub text: Option<String>,
    pub ssml: Option<String>,
}

/// Speech types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeechType {
    #[serde(rename = "PlainText")]
    PlainText,
    #[serde(rename = "SSML")]
    SSML,
}

/// Card types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "type")]
    pub card_type: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub text: Option<String>,
    pub image: Option<CardImage>,
}

/// Card image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImage {
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

/// Reprompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reprompt {
    pub output_speech: OutputSpeech,
}

/// Directive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directive {
    #[serde(rename = "type")]
    pub directive_type: String,
    pub name: Option<String>,
    pub payload: Option<serde_json::Value>,
}

// ============================================================================
// Alexa Provider
// ============================================================================

/// Alexa voice provider
pub struct AlexaProvider {
    skill_id: String,
}

impl AlexaProvider {
    pub fn new(skill_id: String) -> Self {
        Self { skill_id }
    }
}

#[async_trait]
impl VoiceProvider for AlexaProvider {
    fn assistant(&self) -> VoiceAssistant {
        VoiceAssistant::Alexa
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse> {
        // Process Alexa-specific request format
        Ok(VoiceResponse {
            version: "1.0".to_string(),
            response: ResponseBody {
                output_speech: Some(OutputSpeech {
                    speech_type: SpeechType::PlainText,
                    text: Some("Processing your request".to_string()),
                    ssml: None,
                }),
                card: None,
                reprompt: None,
                should_end_session: true,
                directives: vec![],
            },
            session_attributes: serde_json::Value::Null,
        })
    }
    
    fn is_available(&self) -> bool {
        true
    }
    
    fn name(&self) -> &str {
        "alexa"
    }
}

// ============================================================================
// Google Assistant Provider
// ============================================================================

/// Google Assistant provider
pub struct GoogleAssistantProvider {
    project_id: String,
}

impl GoogleAssistantProvider {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
}

#[async_trait]
impl VoiceProvider for GoogleAssistantProvider {
    fn assistant(&self) -> VoiceAssistant {
        VoiceAssistant::GoogleAssistant
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse> {
        Ok(VoiceResponse {
            version: "2.0".to_string(),
            response: ResponseBody {
                output_speech: Some(OutputSpeech {
                    speech_type: SpeechType::SSML,
                    text: None,
                    ssml: Some("<speak>Processing your request</speak>".to_string()),
                }),
                card: None,
                reprompt: None,
                should_end_session: true,
                directives: vec![],
            },
            session_attributes: serde_json::Value::Null,
        })
    }
    
    fn is_available(&self) -> bool {
        true
    }
    
    fn name(&self) -> &str {
        "google"
    }
}

// ============================================================================
// Siri Provider
// ============================================================================

/// Siri provider
pub struct SiriProvider {
    bundle_id: String,
}

impl SiriProvider {
    pub fn new(bundle_id: String) -> Self {
        Self { bundle_id }
    }
}

#[async_trait]
impl VoiceProvider for SiriProvider {
    fn assistant(&self) -> VoiceAssistant {
        VoiceAssistant::Siri
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse> {
        // Siri uses Intents framework
        Ok(VoiceResponse {
            version: "1.0".to_string(),
            response: ResponseBody {
                output_speech: Some(OutputSpeech {
                    speech_type: SpeechType::PlainText,
                    text: Some("Processing your request".to_string()),
                    ssml: None,
                }),
                card: None,
                reprompt: None,
                should_end_session: true,
                directives: vec![],
            },
            session_attributes: serde_json::Value::Null,
        })
    }
    
    fn is_available(&self) -> bool {
        true
    }
    
    fn name(&self) -> &str {
        "siri"
    }
}

// ============================================================================
// Local Provider
// ============================================================================

/// Local voice provider (offline)
pub struct LocalProvider {
    enabled: bool,
}

impl LocalProvider {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for LocalProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VoiceProvider for LocalProvider {
    fn assistant(&self) -> VoiceAssistant {
        VoiceAssistant::Local
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse> {
        Ok(VoiceResponse {
            version: "1.0".to_string(),
            response: ResponseBody {
                output_speech: Some(OutputSpeech {
                    speech_type: SpeechType::PlainText,
                    text: Some("Local processing".to_string()),
                    ssml: None,
                }),
                card: None,
                reprompt: None,
                should_end_session: true,
                directives: vec![],
            },
            session_attributes: serde_json::Value::Null,
        })
    }
    
    fn is_available(&self) -> bool {
        self.enabled
    }
    
    fn name(&self) -> &str {
        "local"
    }
}