//! Configuration types for Voice Assistant integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration for voice assistant integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAssistantConfig {
    /// Enable/disable voice assistant features
    pub enabled: bool,
    
    /// Alexa configuration
    pub alexa: Option<AlexaConfig>,
    
    /// Google Assistant configuration
    pub google: Option<GoogleConfig>,
    
    /// Siri configuration
    pub siri: Option<SiriConfig>,
    
    /// Global settings
    pub global: GlobalVoiceConfig,
}

impl Default for VoiceAssistantConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alexa: None,
            google: None,
            siri: None,
            global: GlobalVoiceConfig::default(),
        }
    }
}

/// Global voice assistant settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVoiceConfig {
    /// Default language for responses
    pub default_language: String,
    
    /// Supported languages
    pub supported_languages: Vec<String>,
    
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    
    /// Maximum retries for failed requests
    pub max_retries: u32,
    
    /// Enable voice feedback
    pub voice_feedback_enabled: bool,
    
    /// Enable card responses
    pub card_responses_enabled: bool,
    
    /// Custom response templates
    pub response_templates: HashMap<String, String>,
}

impl Default for GlobalVoiceConfig {
    fn default() -> Self {
        Self {
            default_language: "en-US".to_string(),
            supported_languages: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "es-ES".to_string(),
                "it-IT".to_string(),
                "ja-JP".to_string(),
            ],
            request_timeout_seconds: 30,
            max_retries: 3,
            voice_feedback_enabled: true,
            card_responses_enabled: true,
            response_templates: HashMap::new(),
        }
    }
}

/// Alexa-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaConfig {
    /// Alexa skill ID
    pub skill_id: String,
    
    /// OAuth2 client ID
    pub client_id: String,
    
    /// OAuth2 client secret
    pub client_secret: String,
    
    /// OAuth2 redirect URI
    pub redirect_uri: String,
    
    /// API endpoint (production or sandbox)
    pub api_endpoint: String,
    
    /// Enable proactive events
    pub proactive_events_enabled: bool,
    
    /// Supported locales
    pub supported_locales: Vec<String>,
    
    /// Skill manifest
    pub skill_manifest: Option<SkillManifest>,
}

impl AlexaConfig {
    pub fn production(skill_id: String, client_id: String, client_secret: String) -> Self {
        Self {
            skill_id,
            client_id,
            client_secret,
            redirect_uri: "https://api.vantis.io/alexa/callback".to_string(),
            api_endpoint: "https://api.amazonalexa.com".to_string(),
            proactive_events_enabled: true,
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
            ],
            skill_manifest: None,
        }
    }
    
    pub fn sandbox(skill_id: String, client_id: String, client_secret: String) -> Self {
        let mut config = Self::production(skill_id, client_id, client_secret);
        config.api_endpoint = "https://api.sandbox.amazonalexa.com".to_string();
        config.proactive_events_enabled = false;
        config
    }
}

/// Alexa Skill Manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub manifest_version: String,
    pub publishing_information: PublishingInformation,
    pub apis: AlexaApis,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishingInformation {
    pub locales: HashMap<String, LocaleInfo>,
    pub distribution_countries: Vec<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub example_phrases: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaApis {
    pub custom: CustomApi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomApi {
    pub endpoint: Endpoint,
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    #[serde(rename = "type")]
    pub interface_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
}

/// Google Assistant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    /// Google Action project ID
    pub project_id: String,
    
    /// OAuth2 client ID
    pub client_id: String,
    
    /// OAuth2 client secret
    pub client_secret: String,
    
    /// OAuth2 redirect URI
    pub redirect_uri: String,
    
    /// API endpoint
    pub api_endpoint: String,
    
    /// Supported locales
    pub supported_locales: Vec<String>,
    
    /// Enable push notifications
    pub push_notifications_enabled: bool,
    
    /// Actions project configuration
    pub actions_project: Option<ActionsProject>,
}

impl GoogleConfig {
    pub fn production(project_id: String, client_id: String, client_secret: String) -> Self {
        Self {
            project_id,
            client_id,
            client_secret,
            redirect_uri: "https://api.vantis.io/google/callback".to_string(),
            api_endpoint: "https://actions.googleapis.com".to_string(),
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
            ],
            push_notifications_enabled: true,
            actions_project: None,
        }
    }
}

/// Google Actions Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsProject {
    pub action_package_name: String,
    pub default_locale: String,
    pub surface_requirements: SurfaceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceRequirements {
    pub minimum_capabilities: Vec<String>,
}

/// Siri configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriConfig {
    /// Bundle identifier for the app
    pub bundle_identifier: String,
    
    /// Team ID
    pub team_id: String,
    
    /// API endpoint
    pub api_endpoint: String,
    
    /// Supported intents
    pub supported_intents: Vec<String>,
    
    /// Enable Siri suggestions
    pub suggestions_enabled: bool,
    
    /// Donate intents automatically
    pub auto_donate_intents: bool,
    
    /// Intent definition configuration
    pub intent_definition: Option<IntentDefinition>,
}

impl SiriConfig {
    pub fn new(bundle_identifier: String, team_id: String) -> Self {
        Self {
            bundle_identifier,
            team_id,
            api_endpoint: "https://api.vantis.io/siri".to_string(),
            supported_intents: vec![
                "PlayMedia".to_string(),
                "PauseMedia".to_string(),
                "SearchMedia".to_string(),
                "SetVolume".to_string(),
            ],
            suggestions_enabled: true,
            auto_donate_intents: true,
            intent_definition: None,
        }
    }
}

/// Siri Intent Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentDefinition {
    pub intent_name: String,
    pub identifier: String,
    pub parameters: Vec<IntentParameter>,
    pub parameter_combinations: Vec<ParameterCombination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentParameter {
    pub name: String,
    pub r#type: String,
    pub prompts: Vec<String>,
    pub suggested_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterCombination {
    pub required_parameters: Vec<String>,
    pub optional_parameters: Vec<String>,
}