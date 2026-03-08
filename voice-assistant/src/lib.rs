//! Voice Assistant Integration for Vantis Media Player
//!
//! This module provides integration with popular voice assistants:
//! - Amazon Alexa
//! - Google Assistant
//! - Apple Siri
//!
//! Each integration handles voice commands for media control,
//! including play, pause, seek, and content discovery.

pub mod alexa;
pub mod google;
pub mod siri;
pub mod commands;
pub mod intent;
pub mod response;
pub mod config;
pub mod error;

pub use error::VoiceAssistantError;
pub use commands::{VoiceCommand, MediaCommand, PlaybackState};
pub use intent::{IntentHandler, IntentResult};
pub use response::{VoiceResponse, Card, Directive};
pub use config::VoiceAssistantConfig;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Main trait for voice assistant integrations
#[async_trait]
pub trait VoiceAssistant: Send + Sync {
    /// Get the name of the voice assistant
    fn name(&self) -> &str;
    
    /// Process an incoming voice request
    async fn process_request(&self, request: VoiceRequest) -> Result<VoiceResponse, VoiceAssistantError>;
    
    /// Handle account linking
    async fn link_account(&self, auth_code: &str) -> Result<LinkedAccount, VoiceAssistantError>;
    
    /// Unlink an account
    async fn unlink_account(&self, user_id: &str) -> Result<(), VoiceAssistantError>;
    
    /// Check if the service is healthy
    async fn health_check(&self) -> Result<bool, VoiceAssistantError>;
}

/// Incoming voice request from any assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceRequest {
    /// Unique request ID
    pub id: String,
    /// Timestamp of the request
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// The locale of the user (e.g., "en-US")
    pub locale: String,
    /// The type of request
    pub request_type: RequestType,
    /// User identification
    pub user: UserInfo,
    /// Session information (for multi-turn conversations)
    pub session: Option<SessionInfo>,
    /// Device information
    pub device: Option<DeviceInfo>,
    /// Original intent/query
    pub intent: Option<IntentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Launch,
    Intent,
    SessionEnded,
    AccountLink,
    Permission,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub access_token: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub new: bool,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_type: DeviceType,
    pub capabilities: Vec<DeviceCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    SmartSpeaker,
    Phone,
    Tablet,
    Tv,
    Automotive,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCapability {
    AudioPlayer,
    Screen,
    VideoPlayer,
    Geolocation,
    Reminders,
    Notifications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentData {
    pub name: String,
    pub slots: std::collections::HashMap<String, SlotValue>,
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotValue {
    pub value: String,
    pub resolved: Option<String>,
    pub status: SlotStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlotStatus {
    Ok,
    None,
    Ambiguous,
}

/// Linked account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccount {
    pub user_id: String,
    pub assistant_user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Voice Assistant Manager - coordinates all voice assistant integrations
pub struct VoiceAssistantManager {
    alexa: Option<alexa::AlexaHandler>,
    google: Option<google::GoogleHandler>,
    siri: Option<siri::SiriHandler>,
    config: VoiceAssistantConfig,
}

impl VoiceAssistantManager {
    pub fn new(config: VoiceAssistantConfig) -> Self {
        Self {
            alexa: if config.alexa.enabled {
                Some(alexa::AlexaHandler::new(config.alexa.clone()))
            } else {
                None
            },
            google: if config.google.enabled {
                Some(google::GoogleHandler::new(config.google.clone()))
            } else {
                None
            },
            siri: if config.siri.enabled {
                Some(siri::SiriHandler::new(config.siri.clone()))
            } else {
                None
            },
            config,
        }
    }
    
    /// Route request to appropriate assistant handler
    pub async fn route_request(
        &self,
        assistant: AssistantType,
        request: VoiceRequest,
    ) -> Result<VoiceResponse, VoiceAssistantError> {
        match assistant {
            AssistantType::Alexa => {
                self.alexa
                    .as_ref()
                    .ok_or(VoiceAssistantError::NotEnabled("Alexa".into()))?
                    .process_request(request)
                    .await
            }
            AssistantType::Google => {
                self.google
                    .as_ref()
                    .ok_or(VoiceAssistantError::NotEnabled("Google Assistant".into()))?
                    .process_request(request)
                    .await
            }
            AssistantType::Siri => {
                self.siri
                    .as_ref()
                    .ok_or(VoiceAssistantError::NotEnabled("Siri".into()))?
                    .process_request(request)
                    .await
            }
        }
    }
    
    /// Get all enabled assistants
    pub fn enabled_assistants(&self) -> Vec<AssistantType> {
        let mut assistants = Vec::new();
        if self.alexa.is_some() {
            assistants.push(AssistantType::Alexa);
        }
        if self.google.is_some() {
            assistants.push(AssistantType::Google);
        }
        if self.siri.is_some() {
            assistants.push(AssistantType::Siri);
        }
        assistants
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssistantType {
    Alexa,
    Google,
    Siri,
}