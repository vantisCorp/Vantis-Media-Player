//! Apple Siri Integration
//!
//! Handles Siri shortcuts and intents for Vantis Media Player including:
//! - Media playback shortcuts
//! - Content search shortcuts
//! - Custom intents for advanced control

use crate::{
    VoiceAssistant, VoiceAssistantError, VoiceRequest, VoiceResponse,
    LinkedAccount, commands::{VoiceCommand, MediaCommand},
    response::{Card, Directive},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Siri configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriConfig {
    /// App identifier
    pub app_identifier: String,
    /// Team identifier
    pub team_identifier: String,
    /// Default locale
    pub default_locale: String,
    /// Supported locales
    pub supported_locales: Vec<String>,
    /// Enable custom responses
    pub enable_custom_responses: bool,
    /// Enable parameter combinations
    pub enable_parameter_combinations: bool,
}

impl Default for SiriConfig {
    fn default() -> Self {
        Self {
            app_identifier: String::new(),
            team_identifier: String::new(),
            default_locale: "en-US".to_string(),
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "ja-JP".to_string(),
                "zh-CN".to_string(),
            ],
            enable_custom_responses: true,
            enable_parameter_combinations: true,
        }
    }
}

/// Siri intent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriRequest {
    pub intent: SiriIntent,
    pub scene: Option<SiriScene>,
    pub user: SiriUser,
    pub device: SiriDevice,
    pub app: SiriApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "identifier")]
pub enum SiriIntent {
    #[serde(rename = "INPlayMediaIntent")]
    PlayMedia(PlayMediaIntent),
    #[serde(rename = "INPauseMediaIntent")]
    PauseMedia(PauseMediaIntent),
    #[serde(rename = "INSearchForMediaIntent")]
    SearchMedia(SearchMediaIntent),
    #[serde(rename = "INSetVolumeIntent")]
    SetVolume(SetVolumeIntent),
    #[serde(rename = "VantisCustomIntent")]
    Custom(CustomIntent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayMediaIntent {
    pub media_items: Option<Vec<SiriMediaItem>>,
    pub media_container: Option<SiriMediaContainer>,
    pub playback_speed: Option<f64>,
    pub playback_queue_location: Option<String>,
    pub media_search: Option<SiriMediaSearch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriMediaItem {
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub media_type: Option<String>,
    pub artwork: Option<SiriArtwork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriMediaContainer {
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub container_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriMediaSearch {
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub genre_names: Option<Vec<String>>,
    pub mood_names: Option<Vec<String>>,
    pub release_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriArtwork {
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseMediaIntent {
    pub paused: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMediaIntent {
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub genre_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVolumeIntent {
    pub volume_level: Option<f64>,
    pub volume_relative_level: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomIntent {
    pub intent_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriScene {
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriUser {
    pub identifier: Option<String>,
    pub localized_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriDevice {
    pub identifier: String,
    pub display_name: Option<String>,
    pub device_type: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriApp {
    pub identifier: String,
    pub version: String,
}

/// Siri response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriResponse {
    pub result: SiriResult,
    pub error: Option<SiriError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriResult {
    pub status: String,
    pub output: SiriOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriOutput {
    pub speech: Option<String>,
    pub card: Option<SiriCard>,
    pub directives: Option<Vec<SiriDirective>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriCard {
    pub title: String,
    pub subtitle: Option<String>,
    pub text: String,
    pub image_url: Option<String>,
    pub button: Option<SiriButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriButton {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriDirective {
    #[serde(rename = "type")]
    pub directive_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriError {
    pub code: String,
    pub message: String,
}

/// Siri handler
pub struct SiriHandler {
    config: SiriConfig,
    intent_handlers: HashMap<String, Box<dyn SiriIntentProcessor + Send + Sync>>,
}

#[async_trait]
trait SiriIntentProcessor {
    async fn process(&self, intent: &SiriIntent, user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError>;
}

impl SiriHandler {
    pub fn new(config: SiriConfig) -> Self {
        let mut handlers: HashMap<String, Box<dyn SiriIntentProcessor + Send + Sync>> = HashMap::new();
        
        // Register intent handlers
        handlers.insert("INPlayMediaIntent".to_string(), Box::new(SiriPlayMediaHandler));
        handlers.insert("INPauseMediaIntent".to_string(), Box::new(SiriPauseMediaHandler));
        handlers.insert("INSearchForMediaIntent".to_string(), Box::new(SiriSearchMediaHandler));
        handlers.insert("INSetVolumeIntent".to_string(), Box::new(SiriSetVolumeHandler));
        handlers.insert("VantisCustomIntent".to_string(), Box::new(SiriCustomIntentHandler));
        
        Self {
            config,
            intent_handlers: handlers,
        }
    }
    
    fn convert_request(&self, siri: SiriRequest) -> VoiceRequest {
        let intent_name = match &siri.intent {
            SiriIntent::PlayMedia(_) => "PlayMedia",
            SiriIntent::PauseMedia(_) => "PauseMedia",
            SiriIntent::SearchMedia(_) => "SearchMedia",
            SiriIntent::SetVolume(_) => "SetVolume",
            SiriIntent::Custom(c) => &c.intent_name,
        };
        
        let slots = match &siri.intent {
            SiriIntent::PlayMedia(play) => {
                let mut map = HashMap::new();
                if let Some(ref search) = play.media_search {
                    if let Some(ref name) = search.media_name {
                        map.insert("MediaName".to_string(), crate::SlotValue {
                            value: name.clone(),
                            resolved: None,
                            status: crate::SlotStatus::Ok,
                        });
                    }
                }
                map
            }
            SiriIntent::SetVolume(vol) => {
                let mut map = HashMap::new();
                if let Some(level) = vol.volume_level {
                    map.insert("Level".to_string(), crate::SlotValue {
                        value: (level * 100.0).to_string(),
                        resolved: None,
                        status: crate::SlotStatus::Ok,
                    });
                }
                map
            }
            SiriIntent::SearchMedia(search) => {
                let mut map = HashMap::new();
                if let Some(ref name) = search.media_name {
                    map.insert("Query".to_string(), crate::SlotValue {
                        value: name.clone(),
                        resolved: None,
                        status: crate::SlotStatus::Ok,
                    });
                }
                map
            }
            _ => HashMap::new(),
        };
        
        VoiceRequest {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            locale: self.config.default_locale.clone(),
            request_type: crate::RequestType::Intent,
            user: crate::UserInfo {
                id: siri.user.identifier.clone().unwrap_or_default(),
                access_token: None,
                permissions: Vec::new(),
            },
            session: None,
            device: Some(crate::DeviceInfo {
                device_id: siri.device.identifier.clone(),
                device_type: match siri.device.device_type.as_deref() {
                    Some("iPhone") => crate::DeviceType::Phone,
                    Some("iPad") => crate::DeviceType::Tablet,
                    Some("AppleTV") => crate::DeviceType::Tv,
                    Some("CarPlay") => crate::DeviceType::Automotive,
                    _ => crate::DeviceType::Unknown,
                },
                capabilities: siri.device.capabilities.iter()
                    .map(|c| match c.as_str() {
                        "audio" => crate::DeviceCapability::AudioPlayer,
                        "screen" => crate::DeviceCapability::Screen,
                        "video" => crate::DeviceCapability::VideoPlayer,
                        _ => crate::DeviceCapability::AudioPlayer,
                    })
                    .collect(),
            }),
            intent: Some(crate::IntentData {
                name: intent_name.to_string(),
                slots,
                confidence: None,
            }),
        }
    }
}

#[async_trait]
impl VoiceAssistant for SiriHandler {
    fn name(&self) -> &str {
        "Siri"
    }
    
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse, VoiceAssistantError> {
        match request.request_type {
            crate::RequestType::Launch => {
                Ok(VoiceResponse::simple(
                    "What would you like to watch on Vantis Player?",
                    false,
                ))
            }
            crate::RequestType::Intent => {
                if let Some(intent) = &request.intent {
                    // Create Siri user from request
                    let siri_user = SiriUser {
                        identifier: Some(request.user.id.clone()),
                        localized_name: None,
                    };
                    
                    // Create Siri intent from IntentData
                    let siri_intent = match intent.name.as_str() {
                        "PlayMedia" => SiriIntent::PlayMedia(PlayMediaIntent {
                            media_items: None,
                            media_container: None,
                            playback_speed: None,
                            playback_queue_location: None,
                            media_search: Some(SiriMediaSearch {
                                media_type: None,
                                media_name: intent.slots.get("MediaName").map(|s| s.value.clone()),
                                artist_name: None,
                                album_name: None,
                                genre_names: None,
                                mood_names: None,
                                release_date: None,
                            }),
                        }),
                        "PauseMedia" => SiriIntent::PauseMedia(PauseMediaIntent { paused: None }),
                        "SearchMedia" => SiriIntent::SearchMedia(SearchMediaIntent {
                            media_type: None,
                            media_name: intent.slots.get("Query").map(|s| s.value.clone()),
                            artist_name: None,
                            album_name: None,
                            genre_names: None,
                        }),
                        "SetVolume" => SiriIntent::SetVolume(SetVolumeIntent {
                            volume_level: intent.slots.get("Level")
                                .and_then(|s| s.value.parse::<f64>().ok())
                                .map(|v| v / 100.0),
                            volume_relative_level: None,
                        }),
                        _ => SiriIntent::Custom(CustomIntent {
                            intent_name: intent.name.clone(),
                            parameters: HashMap::new(),
                        }),
                    };
                    
                    if let Some(handler) = self.intent_handlers.get(&intent.name) {
                        handler.process(&siri_intent, &siri_user).await
                    } else {
                        Ok(VoiceResponse::simple(
                            "I can help you play, pause, or search for content.",
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
            _ => Err(VoiceAssistantError::InvalidRequest("Unsupported request type".into())),
        }
    }
    
    async fn link_account(&self, auth_code: &str) -> Result<LinkedAccount, VoiceAssistantError> {
        Ok(LinkedAccount {
            user_id: uuid::Uuid::new_v4().to_string(),
            assistant_user_id: auth_code.to_string(),
            access_token: "siri_access_token".to_string(),
            refresh_token: None,
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

// Siri Intent Handlers

struct SiriPlayMediaHandler;

#[async_trait]
impl SiriIntentProcessor for SiriPlayMediaHandler {
    async fn process(&self, intent: &SiriIntent, _user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let media_name = match intent {
            SiriIntent::PlayMedia(play) => {
                play.media_search.as_ref()
                    .and_then(|s| s.media_name.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("that")
            }
            _ => "that",
        };
        
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

struct SiriPauseMediaHandler;

#[async_trait]
impl SiriIntentProcessor for SiriPauseMediaHandler {
    async fn process(&self, _intent: &SiriIntent, _user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError> {
        Ok(VoiceResponse::simple("Paused playback", true))
    }
}

struct SiriSearchMediaHandler;

#[async_trait]
impl SiriIntentProcessor for SiriSearchMediaHandler {
    async fn process(&self, intent: &SiriIntent, _user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let query = match intent {
            SiriIntent::SearchMedia(search) => {
                search.media_name.as_deref().unwrap_or("content")
            }
            _ => "content",
        };
        
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

struct SiriSetVolumeHandler;

#[async_trait]
impl SiriIntentProcessor for SiriSetVolumeHandler {
    async fn process(&self, intent: &SiriIntent, _user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let level = match intent {
            SiriIntent::SetVolume(vol) => {
                vol.volume_level.map(|v| (v * 100.0) as i32)
                    .unwrap_or(50)
            }
            _ => 50,
        };
        
        Ok(VoiceResponse::simple(&format!("Volume set to {} percent", level), true))
    }
}

struct SiriCustomIntentHandler;

#[async_trait]
impl SiriIntentProcessor for SiriCustomIntentHandler {
    async fn process(&self, intent: &SiriIntent, _user: &SiriUser) -> Result<VoiceResponse, VoiceAssistantError> {
        let intent_name = match intent {
            SiriIntent::Custom(c) => &c.intent_name,
            _ => "unknown",
        };
        
        Ok(VoiceResponse::simple(&format!("Processing custom intent: {}", intent_name), false))
    }
}