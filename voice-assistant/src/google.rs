//! Google Assistant Integration
//!
//! Handles Google Assistant Actions for Vantis Media Player including:
//! - Conversational Action for media control
//! - Smart Home integration for basic playback
//! - App Actions for deep linking

use crate::{
    VoiceAssistant, VoiceAssistantError, VoiceRequest, VoiceResponse,
    LinkedAccount, commands::{VoiceCommand, MediaCommand},
    response::{Card, Directive},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Google Assistant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    /// Project ID from Google Cloud Console
    pub project_id: String,
    /// Action ID
    pub action_id: String,
    /// Default locale
    pub default_locale: String,
    /// Supported locales
    pub supported_locales: Vec<String>,
    /// OAuth2 client ID
    pub oauth_client_id: String,
    /// OAuth2 client secret
    pub oauth_client_secret: String,
    /// Enable surface detection
    pub enable_surface_detection: bool,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            project_id: String::new(),
            action_id: String::new(),
            default_locale: "en-US".to_string(),
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "ja-JP".to_string(),
            ],
            oauth_client_id: String::new(),
            oauth_client_secret: String::new(),
            enable_surface_detection: true,
        }
    }
}

/// Google Assistant request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleRequest {
    pub handler: Handler,
    pub intent: GoogleIntent,
    pub scene: Option<Scene>,
    pub session: GoogleSession,
    pub user: GoogleUser,
    pub home: Option<GoogleHome>,
    pub device: GoogleDevice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handler {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleIntent {
    pub name: String,
    pub params: HashMap<String, ParamValue>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamValue {
    pub original: String,
    pub resolved: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub slot_filling: Option<SlotFillingStatus>,
    pub next: Option<NextScene>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotFillingStatus {
    pub slots: HashMap<String, SlotStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotStatus {
    pub mode: String,
    pub status: String,
    pub value: Option<serde_json::Value>,
    pub updated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextScene {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleSession {
    pub id: String,
    pub params: HashMap<String, serde_json::Value>,
    #[serde(rename = "typeOverrides")]
    pub type_overrides: Option<Vec<TypeOverride>>,
    pub storage: Option<SessionStorage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeOverride {
    pub name: String,
    pub mode: String,
    pub synonym: Option<SynonymType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymType {
    pub entries: Vec<SynonymEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymEntry {
    pub name: String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStorage {
    #[serde(rename = "lastPromptSize")]
    pub last_prompt_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUser {
    pub locale: String,
    pub params: HashMap<String, serde_json::Value>,
    pub account_linking: Option<AccountLinkingStatus>,
    pub verification: Option<VerificationStatus>,
    #[serde(rename = "lastSeenTime")]
    pub last_seen_time: Option<String>,
    pub engagement: Option<EngagementData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLinkingStatus {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStatus {
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementData {
    pub push_notification: Option<PushNotificationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotificationState {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleHome {
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDevice {
    pub capabilities: Vec<String>,
    pub time_zone: Option<TimeZoneInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeZoneInfo {
    pub id: String,
    pub version: String,
}

/// Google Assistant response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleResponse {
    pub session: GoogleSessionResponse,
    pub prompt: PromptResponse,
    pub scene: Option<SceneResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleSessionResponse {
    pub id: String,
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_: Option<bool>,
    pub first_simple: Option<SimpleResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_simple: Option<SimpleResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<Suggestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<GoogleCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas: Option<CanvasResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub speech: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCard {
    pub title: String,
    pub subtitle: Option<String>,
    pub text: String,
    pub image: Option<CardImage>,
    pub button: Option<CardButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImage {
    pub url: String,
    pub alt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardButton {
    pub name: String,
    pub open: OpenUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenUrl {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasResponse {
    pub url: String,
    pub data: Option<Vec<serde_json::Value>>,
    pub suppress_microphone: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneResponse {
    pub name: String,
    pub slots: Option<HashMap<String, SlotStatus>>,
    pub next: Option<NextScene>,
}

/// Google Assistant handler
pub struct GoogleHandler {
    config: GoogleConfig,
    intent_handlers: HashMap<String, Box<dyn GoogleIntentProcessor + Send + Sync>>,
}

#[async_trait]
trait GoogleIntentProcessor {
    async fn process(&self, intent: &GoogleIntent, user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError>;
}

impl GoogleHandler {
    pub fn new(config: GoogleConfig) -> Self {
        let mut handlers: HashMap<String, Box<dyn GoogleIntentProcessor + Send + Sync>> = HashMap::new();
        
        // Register intent handlers
        handlers.insert("PlayMedia".to_string(), Box::new(GooglePlayMediaHandler));
        handlers.insert("PauseMedia".to_string(), Box::new(GooglePauseMediaHandler));
        handlers.insert("ResumeMedia".to_string(), Box::new(GoogleResumeMediaHandler));
        handlers.insert("StopMedia".to_string(), Box::new(GoogleStopMediaHandler));
        handlers.insert("SearchMedia".to_string(), Box::new(GoogleSearchMediaHandler));
        handlers.insert("SetVolume".to_string(), Box::new(GoogleSetVolumeHandler));
        handlers.insert("ListPlaylists".to_string(), Box::new(GoogleListPlaylistsHandler));
        
        Self {
            config,
            intent_handlers: handlers,
        }
    }
    
    fn convert_request(&self, google: GoogleRequest) -> VoiceRequest {
        VoiceRequest {
            id: google.session.id.clone(),
            timestamp: chrono::Utc::now(),
            locale: google.user.locale.clone(),
            request_type: crate::RequestType::Intent,
            user: crate::UserInfo {
                id: google.user.account_linking
                    .as_ref()
                    .map(|_| "linked_user".to_string())
                    .unwrap_or_else(|| "anonymous".to_string()),
                access_token: None,
                permissions: Vec::new(),
            },
            session: Some(crate::SessionInfo {
                id: google.session.id.clone(),
                new: false,
                attributes: serde_json::Value::Object(
                    google.session.params.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                ),
            }),
            device: Some(crate::DeviceInfo {
                device_id: "google_device".to_string(),
                device_type: crate::DeviceType::SmartSpeaker,
                capabilities: google.device.capabilities.iter()
                    .map(|c| match c.as_str() {
                        "SPEECH" => crate::DeviceCapability::AudioPlayer,
                        "SCREEN_OUTPUT" => crate::DeviceCapability::Screen,
                        "WEB_LINK" => crate::DeviceCapability::VideoPlayer,
                        _ => crate::DeviceCapability::AudioPlayer,
                    })
                    .collect(),
            }),
            intent: Some(crate::IntentData {
                name: google.intent.name.clone(),
                slots: google.intent.params.iter()
                    .map(|(k, v)| (k.clone(), crate::SlotValue {
                        value: v.original.clone(),
                        resolved: v.resolved.clone(),
                        status: crate::SlotStatus::Ok,
                    }))
                    .collect(),
                confidence: None,
            }),
        }
    }
}

#[async_trait]
impl VoiceAssistant for GoogleHandler {
    fn name(&self) -> &str {
        "Google Assistant"
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse, VoiceAssistantError> {
        match request.request_type {
            crate::RequestType::Launch => {
                Ok(VoiceResponse::simple(
                    "Welcome to Vantis Media Player! What would you like to watch?",
                    false,
                ))
            }
            crate::RequestType::Intent => {
                if let Some(intent) = &request.intent {
                    // Create Google user from request
                    let google_user = GoogleUser {
                        locale: request.locale.clone(),
                        params: HashMap::new(),
                        account_linking: None,
                        verification: None,
                        last_seen_time: None,
                        engagement: None,
                    };
                    
                    // Create Google intent from IntentData
                    let google_intent = GoogleIntent {
                        name: intent.name.clone(),
                        params: intent.slots.iter()
                            .map(|(k, v)| (k.clone(), ParamValue {
                                original: v.value.clone(),
                                resolved: None,
                            }))
                            .collect(),
                        query: None,
                    };
                    
                    if let Some(handler) = self.intent_handlers.get(&intent.name) {
                        handler.process(&google_intent, &google_user).await
                    } else {
                        Ok(VoiceResponse::simple(
                            "I'm not sure how to help with that. You can ask me to play, pause, or search for content.",
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
        Ok(LinkedAccount {
            user_id: uuid::Uuid::new_v4().to_string(),
            assistant_user_id: auth_code.to_string(),
            access_token: "google_access_token".to_string(),
            refresh_token: Some("google_refresh_token".to_string()),
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

// Google Intent Handlers

struct GooglePlayMediaHandler;

#[async_trait]
impl GoogleIntentProcessor for GooglePlayMediaHandler {
    async fn process(&self, intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let media_name = intent.params.get("MediaName")
            .map(|p| p.original.as_str())
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

struct GooglePauseMediaHandler;

#[async_trait]
impl GoogleIntentProcessor for GooglePauseMediaHandler {
    async fn process(&self, _intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::simple("Paused playback", true))
    }
}

struct GoogleResumeMediaHandler;

#[async_trait]
impl GoogleIntentProcessor for GoogleResumeMediaHandler {
    async fn process(&self, _intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::simple("Resuming playback", true))
    }
}

struct GoogleStopMediaHandler;

#[async_trait]
impl GoogleIntentProcessor for GoogleStopMediaHandler {
    async fn process(&self, _intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::simple("Stopped playback", true))
    }
}

struct GoogleSearchMediaHandler;

#[async_trait]
impl GoogleIntentProcessor for GoogleSearchMediaHandler {
    async fn process(&self, intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let query = intent.params.get("Query")
            .map(|p| p.original.as_str())
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

struct GoogleSetVolumeHandler;

#[async_trait]
impl GoogleIntentProcessor for GoogleSetVolumeHandler {
    async fn process(&self, intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let level = intent.params.get("Level")
            .map(|p| p.original.as_str())
            .unwrap_or("50");
        
        Ok(VoiceResponse::simple(&format!("Volume set to {} percent", level), true))
    }
}

struct GoogleListPlaylistsHandler;

#[async_trait]
impl GoogleIntentProcessor for GoogleListPlaylistsHandler {
    async fn process(&self, _intent: &GoogleIntent, _user: &GoogleUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::with_card(
            "You have 3 playlists",
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