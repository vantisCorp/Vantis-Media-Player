//! Amazon Alexa Integration
//!
//! Handles Alexa skills for Vantis Media Player including:
//! - Custom skill for full media control
//! - Smart Home skill for basic playback
//! - Video skill for content discovery

use crate::{
    VoiceAssistant, VoiceAssistantError, VoiceRequest, VoiceResponse,
    LinkedAccount, IntentData, commands::{VoiceCommand, MediaCommand},
    response::{Card, Directive, OutputSpeech},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Alexa-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaConfig {
    /// Skill ID from Alexa Developer Console
    pub skill_id: String,
    /// Application ID
    pub application_id: String,
    /// Default locale
    pub default_locale: String,
    /// Supported locales
    pub supported_locales: Vec<String>,
    /// Enable personalization
    pub enable_personalization: bool,
    /// Enable geo-location
    pub enable_geolocation: bool,
    /// Enable reminders
    pub enable_reminders: bool,
}

impl Default for AlexaConfig {
    fn default() -> Self {
        Self {
            skill_id: String::new(),
            application_id: String::new(),
            default_locale: "en-US".to_string(),
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "ja-JP".to_string(),
            ],
            enable_personalization: true,
            enable_geolocation: false,
            enable_reminders: false,
        }
    }
}

/// Alexa request wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaRequest {
    pub version: String,
    pub session: AlexaSession,
    pub context: AlexaContext,
    pub request: AlexaRequestBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaSession {
    pub new: bool,
    pub session_id: String,
    pub application: AlexaApplication,
    pub attributes: HashMap<String, serde_json::Value>,
    pub user: AlexaUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaApplication {
    pub application_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaUser {
    pub user_id: String,
    pub access_token: Option<String>,
    pub permissions: Option<AlexaPermissions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaPermissions {
    pub consent_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaContext {
    pub system: AlexaSystem,
    pub device: Option<AlexaDevice>,
    pub viewport: Option<AlexaViewport>,
    pub geolocation: Option<AlexaGeolocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaSystem {
    pub application: AlexaApplication,
    pub user: AlexaUser,
    pub device: AlexaDevice,
    pub api_endpoint: String,
    pub api_access_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaDevice {
    pub device_id: String,
    pub supported_interfaces: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaViewport {
    pub width: u32,
    pub height: u32,
    pub shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaGeolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AlexaRequestBody {
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
    #[serde(flatten)]
    pub body: RequestBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestBody {
    Launch(LaunchRequest),
    Intent(IntentRequest),
    SessionEnded(SessionEndedRequest),
    AccountLink(AccountLinkRequest),
    CanFulfill(CanFulfillIntentRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchRequest {
    #[serde(rename = "type")]
    pub request_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub intent: AlexaIntent,
    pub dialog_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaIntent {
    pub name: String,
    pub confirmation_status: Option<String>,
    pub slots: HashMap<String, AlexaSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaSlot {
    pub name: String,
    pub value: Option<String>,
    pub resolutions: Option<AlexaResolutions>,
    pub confirmation_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResolutions {
    pub resolutions_per_authority: Vec<AlexaResolutionAuthority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResolutionAuthority {
    pub authority: String,
    pub status: AlexaResolutionStatus,
    pub values: Vec<AlexaResolutionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResolutionStatus {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResolutionValue {
    pub value: AlexaResolutionValueInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResolutionValueInner {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEndedRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLinkRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanFulfillIntentRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub intent: AlexaIntent,
}

/// Alexa response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResponse {
    pub version: String,
    pub response: AlexaResponseBody,
    pub session_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexaResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_speech: Option<OutputSpeech>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<Reprompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<Directive>>,
    pub should_end_session: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reprompt {
    pub output_speech: OutputSpeech,
}

/// Alexa handler implementation
pub struct AlexaHandler {
    config: AlexaConfig,
    intent_handlers: HashMap<String, Box<dyn IntentProcessor + Send + Sync>>,
}

#[async_trait]
trait IntentProcessor {
    async fn process(&self, intent: &AlexaIntent, user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError>;
}

impl AlexaHandler {
    pub fn new(config: AlexaConfig) -> Self {
        let mut handlers: HashMap<String, Box<dyn IntentProcessor + Send + Sync>> = HashMap::new();
        
        // Register built-in intent handlers
        handlers.insert("PlayMedia".to_string(), Box::new(PlayMediaHandler));
        handlers.insert("PauseMedia".to_string(), Box::new(PauseMediaHandler));
        handlers.insert("ResumeMedia".to_string(), Box::new(ResumeMediaHandler));
        handlers.insert("StopMedia".to_string(), Box::new(StopMediaHandler));
        handlers.insert("SeekMedia".to_string(), Box::new(SeekMediaHandler));
        handlers.insert("SearchMedia".to_string(), Box::new(SearchMediaHandler));
        handlers.insert("SetVolume".to_string(), Box::new(SetVolumeHandler));
        handlers.insert("ListPlaylists".to_string(), Box::new(ListPlaylistsHandler));
        
        Self {
            config,
            intent_handlers: handlers,
        }
    }
    
    fn convert_request(&self, alexa: AlexaRequest) -> VoiceRequest {
        VoiceRequest {
            id: alexa.request.request_id.clone(),
            timestamp: chrono::Utc::now(),
            locale: alexa.request.locale.clone(),
            request_type: match alexa.request.body {
                RequestBody::Launch(_) => crate::RequestType::Launch,
                RequestBody::Intent(_) => crate::RequestType::Intent,
                RequestBody::SessionEnded(_) => crate::RequestType::SessionEnded,
                RequestBody::AccountLink(_) => crate::RequestType::AccountLink,
                RequestBody::CanFulfill(_) => crate::RequestType::System,
            },
            user: crate::UserInfo {
                id: alexa.session.user.user_id.clone(),
                access_token: alexa.session.user.access_token.clone(),
                permissions: Vec::new(),
            },
            session: Some(crate::SessionInfo {
                id: alexa.session.session_id.clone(),
                new: alexa.session.new,
                attributes: serde_json::Value::Object(
                    alexa.session.attributes.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                ),
            }),
            device: Some(crate::DeviceInfo {
                device_id: alexa.context.system.device.device_id.clone(),
                device_type: crate::DeviceType::SmartSpeaker,
                capabilities: vec![crate::DeviceCapability::AudioPlayer],
            }),
            intent: match alexa.request.body {
                RequestBody::Intent(ref intent_req) => Some(crate::IntentData {
                    name: intent_req.intent.name.clone(),
                    slots: intent_req.intent.slots.iter()
                        .map(|(k, v)| (k.clone(), crate::SlotValue {
                            value: v.value.clone().unwrap_or_default(),
                            resolved: v.resolutions.as_ref().and_then(|r| 
                                r.resolutions_per_authority.first()
                                    .and_then(|a| a.values.first())
                                    .map(|v| v.value.name.clone())
                            ),
                            status: crate::SlotStatus::Ok,
                        }))
                        .collect(),
                    confidence: None,
                }),
                _ => None,
            },
        }
    }
}

#[async_trait]
impl VoiceAssistant for AlexaHandler {
    fn name(&self) -> &str {
        "Alexa"
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse, VoiceAssistantError> {
        match request.request_type {
            crate::RequestType::Launch => {
                Ok(VoiceResponse::simple(
                    "Welcome to Vantis Media Player. What would you like to watch?",
                    false,
                ))
            }
            crate::RequestType::Intent => {
                if let Some(intent) = &request.intent {
                    if let Some(handler) = self.intent_handlers.get(&intent.name) {
                        // Create a temporary AlexaUser from request
                        let alexa_user = AlexaUser {
                            user_id: request.user.id.clone(),
                            access_token: request.user.access_token.clone(),
                            permissions: None,
                        };
                        
                        // Create AlexaIntent from IntentData
                        let alexa_intent = AlexaIntent {
                            name: intent.name.clone(),
                            confirmation_status: None,
                            slots: intent.slots.iter()
                                .map(|(k, v)| (k.clone(), AlexaSlot {
                                    name: k.clone(),
                                    value: Some(v.value.clone()),
                                    resolutions: None,
                                    confirmation_status: None,
                                }))
                                .collect(),
                        };
                        
                        handler.process(&alexa_intent, &alexa_user).await
                    } else {
                        Ok(VoiceResponse::simple(
                            "I'm not sure how to help with that. Try saying play, pause, or search.",
                            false,
                        ))
                    }
                } else {
                    Err(VoiceAssistantError::InvalidRequest("No intent in request".into()))
                }
            }
            crate::RequestType::SessionEnded => {
                Ok(VoiceResponse::empty())
            }
            crate::RequestType::AccountLink => {
                Ok(VoiceResponse::simple("Your account has been linked successfully.", true))
            }
            _ => Err(VoiceAssistantError::InvalidRequest("Unsupported request type".into())),
        }
    }
    
    async fn link_account(&self, auth_code: &str) -> Result<LinkedAccount, VoiceAssistantError> {
        // In a real implementation, this would exchange the auth code for tokens
        Ok(LinkedAccount {
            user_id: uuid::Uuid::new_v4().to_string(),
            assistant_user_id: auth_code.to_string(),
            access_token: "access_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        })
    }
    
    async fn unlink_account(&self, _user_id: &str) -> Result<(), VoiceAssistantError> {
        Ok(())
    }
    
    async fn health_check(&self) -> Result<bool, VoiceAssistantError> {
        Ok(true)
    }
}

// Intent Handlers

struct PlayMediaHandler;

#[async_trait]
impl IntentProcessor for PlayMediaHandler {
    async fn process(&self, intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let media_name = intent.slots.get("MediaName")
            .and_then(|s| s.value.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("that");
        
        Ok(VoiceResponse::with_directives(
            &format!("Playing {} on Vantis Player", media_name),
            vec![Directive::AudioPlayer {
                url: "https://example.com/stream.m3u8".to_string(),
                token: uuid::Uuid::new_v4().to_string(),
                offset_ms: 0,
            }],
        ))
    }
}

struct PauseMediaHandler;

#[async_trait]
impl IntentProcessor for PauseMediaHandler {
    async fn process(&self, _intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::with_directives(
            "Paused",
            vec![Directive::PlaybackController { action: "Pause".to_string() }],
        ))
    }
}

struct ResumeMediaHandler;

#[async_trait]
impl IntentProcessor for ResumeMediaHandler {
    async fn process(&self, _intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::with_directives(
            "Resuming",
            vec![Directive::PlaybackController { action: "Play".to_string() }],
        ))
    }
}

struct StopMediaHandler;

#[async_trait]
impl IntentProcessor for StopMediaHandler {
    async fn process(&self, _intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::with_directives(
            "Stopped",
            vec![Directive::PlaybackController { action: "Stop".to_string() }],
        ))
    }
}

struct SeekMediaHandler;

#[async_trait]
impl IntentProcessor for SeekMediaHandler {
    async fn process(&self, intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let position = intent.slots.get("Position")
            .and_then(|s| s.value.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("beginning");
        
        Ok(VoiceResponse::simple(&format!("Seeking to {}", position), true))
    }
}

struct SearchMediaHandler;

#[async_trait]
impl IntentProcessor for SearchMediaHandler {
    async fn process(&self, intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let query = intent.slots.get("Query")
            .and_then(|s| s.value.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("content");
        
        Ok(VoiceResponse::with_card(
            &format!("I found several results for {}", query),
            Card::List {
                title: format!("Search Results for {}", query),
                items: vec![
                    "The Last Kingdom".to_string(),
                    "Stranger Things".to_string(),
                    "Oppenheimer".to_string(),
                ],
            },
            false,
        ))
    }
}

struct SetVolumeHandler;

#[async_trait]
impl IntentProcessor for SetVolumeHandler {
    async fn process(&self, intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let level = intent.slots.get("Level")
            .and_then(|s| s.value.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("50");
        
        Ok(VoiceResponse::simple(&format!("Volume set to {} percent", level), true))
    }
}

struct ListPlaylistsHandler;

#[async_trait]
impl IntentProcessor for ListPlaylistsHandler {
    async fn process(&self, _intent: &AlexaIntent, _user: &AlexaUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::with_card(
            "You have 3 playlists: Watch Later, Favorites, and Action Movies",
            Card::List {
                title: "Your Playlists".to_string(),
                items: vec![
                    "Watch Later".to_string(),
                    "Favorites".to_string(),
                    "Action Movies".to_string(),
                ],
            },
            false,
        ))
    }
}