//! Intent handling for voice commands

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::commands::{CommandType, MediaCommand, VoiceCommand};
use crate::error::VoiceAssistantError;
use crate::response::VoiceResponse;

/// Trait for handling voice intents
#[async_trait]
pub trait IntentHandler: Send + Sync {
    /// Get the intent name this handler processes
    fn intent_name(&self) -> &str;
    
    /// Check if this handler can process the given command
    fn can_handle(&self, command: &VoiceCommand) -> bool;
    
    /// Process the voice command and return a response
    async fn handle(&self, command: VoiceCommand) -> Result<IntentResult, VoiceAssistantError>;
    
    /// Get supported slots/parameters for this intent
    fn supported_slots(&self) -> Vec<String> {
        vec![]
    }
    
    /// Validate the command before processing
    fn validate(&self, command: &VoiceCommand) -> Result<(), VoiceAssistantError> {
        Ok(())
    }
}

/// Result from processing an intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResult {
    /// Whether the intent was successfully handled
    pub success: bool,
    
    /// Response to send back to the user
    pub response: VoiceResponse,
    
    /// State changes to apply
    pub state_changes: HashMap<String, serde_json::Value>,
    
    /// Whether the session should end
    pub should_end_session: bool,
    
    /// Additional directives to execute
    pub directives: Vec<crate::response::Directive>,
}

impl IntentResult {
    /// Create a successful intent result
    pub fn success(response: VoiceResponse) -> Self {
        Self {
            success: true,
            response,
            state_changes: HashMap::new(),
            should_end_session: false,
            directives: vec![],
        }
    }
    
    /// Create a failed intent result
    pub fn failure(response: VoiceResponse) -> Self {
        Self {
            success: false,
            response,
            state_changes: HashMap::new(),
            should_end_session: false,
            directives: vec![],
        }
    }
    
    /// Add a state change
    pub fn with_state(mut self, key: String, value: serde_json::Value) -> Self {
        self.state_changes.insert(key, value);
        self
    }
    
    /// Set whether to end the session
    pub fn with_end_session(mut self, end: bool) -> Self {
        self.should_end_session = end;
        self
    }
    
    /// Add a directive
    pub fn with_directive(mut self, directive: crate::response::Directive) -> Self {
        self.directives.push(directive);
        self
    }
}

/// Registry of intent handlers
pub struct IntentRegistry {
    handlers: HashMap<String, Arc<dyn IntentHandler>>,
    default_handler: Option<Arc<dyn IntentHandler>>,
}

impl IntentRegistry {
    /// Create a new intent registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            default_handler: None,
        }
    }
    
    /// Register an intent handler
    pub fn register(&mut self, handler: Arc<dyn IntentHandler>) {
        self.handlers.insert(handler.intent_name().to_string(), handler);
    }
    
    /// Set the default handler for unmatched intents
    pub fn set_default(&mut self, handler: Arc<dyn IntentHandler>) {
        self.default_handler = Some(handler);
    }
    
    /// Get a handler for an intent
    pub fn get_handler(&self, intent_name: &str) -> Option<Arc<dyn IntentHandler>> {
        self.handlers.get(intent_name).cloned()
    }
    
    /// Find a handler that can process the command
    pub fn find_handler(&self, command: &VoiceCommand) -> Option<Arc<dyn IntentHandler>> {
        // First, try to find by command type
        let intent_name = match &command.command_type {
            CommandType::Media(media) => media_to_intent(media),
            CommandType::Library(library) => library_to_intent(library),
            CommandType::System(system) => system_to_intent(system),
            CommandType::Query(query) => query_to_intent(query),
            CommandType::Custom(name) => name.clone(),
        };
        
        if let Some(handler) = self.handlers.get(&intent_name) {
            if handler.can_handle(command) {
                return Some(handler.clone());
            }
        }
        
        // Fall back to checking all handlers
        for handler in self.handlers.values() {
            if handler.can_handle(command) {
                return Some(handler.clone());
            }
        }
        
        // Fall back to default handler
        self.default_handler.clone()
    }
    
    /// Process a command using the appropriate handler
    pub async fn process(&self, command: VoiceCommand) -> Result<IntentResult, VoiceAssistantError> {
        if let Some(handler) = self.find_handler(&command) {
            handler.validate(&command)?;
            handler.handle(command).await
        } else {
            Err(VoiceAssistantError::IntentNotRecognized(
                format!("No handler found for command: {:?}", command.command_type)
            ))
        }
    }
    
    /// List all registered intent names
    pub fn list_intents(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

impl Default for IntentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions to map commands to intent names
fn media_to_intent(media: &MediaCommand) -> String {
    match media {
        MediaCommand::Play { .. } => "PlayMedia".to_string(),
        MediaCommand::Pause => "PauseMedia".to_string(),
        MediaCommand::Resume => "ResumeMedia".to_string(),
        MediaCommand::Stop => "StopMedia".to_string(),
        MediaCommand::Next => "NextMedia".to_string(),
        MediaCommand::Previous => "PreviousMedia".to_string(),
        MediaCommand::Seek { .. } => "SeekMedia".to_string(),
        MediaCommand::SetVolume { .. } => "SetVolume".to_string(),
        MediaCommand::Mute => "Mute".to_string(),
        MediaCommand::Unmute => "Unmute".to_string(),
        MediaCommand::ToggleMute => "ToggleMute".to_string(),
        MediaCommand::Shuffle { .. } => "SetShuffle".to_string(),
        MediaCommand::Repeat { .. } => "SetRepeat".to_string(),
        MediaCommand::FastForward { .. } => "FastForward".to_string(),
        MediaCommand::Rewind { .. } => "Rewind".to_string(),
        MediaCommand::AddToQueue { .. } => "AddToQueue".to_string(),
        MediaCommand::ClearQueue => "ClearQueue".to_string(),
    }
}

fn library_to_intent(library: &crate::commands::LibraryCommand) -> String {
    match library {
        crate::commands::LibraryCommand::AddToFavorites { .. } => "AddToFavorites".to_string(),
        crate::commands::LibraryCommand::RemoveFromFavorites { .. } => "RemoveFromFavorites".to_string(),
        crate::commands::LibraryCommand::CreatePlaylist { .. } => "CreatePlaylist".to_string(),
        crate::commands::LibraryCommand::DeletePlaylist { .. } => "DeletePlaylist".to_string(),
        crate::commands::LibraryCommand::AddToPlaylist { .. } => "AddToPlaylist".to_string(),
        crate::commands::LibraryCommand::RemoveFromPlaylist { .. } => "RemoveFromPlaylist".to_string(),
        crate::commands::LibraryCommand::SharePlaylist { .. } => "SharePlaylist".to_string(),
    }
}

fn system_to_intent(system: &crate::commands::SystemCommand) -> String {
    match system {
        crate::commands::SystemCommand::SetOutputDevice { .. } => "SetOutputDevice".to_string(),
        crate::commands::SystemCommand::ToggleSubtitles { .. } => "ToggleSubtitles".to_string(),
        crate::commands::SystemCommand::SetVideoQuality { .. } => "SetVideoQuality".to_string(),
        crate::commands::SystemCommand::ToggleFullscreen => "ToggleFullscreen".to_string(),
        crate::commands::SystemCommand::SetSleepTimer { .. } => "SetSleepTimer".to_string(),
        crate::commands::SystemCommand::CancelSleepTimer => "CancelSleepTimer".to_string(),
    }
}

fn query_to_intent(query: &crate::commands::QueryCommand) -> String {
    match query {
        crate::commands::QueryCommand::GetCurrentPlayback => "GetCurrentPlayback".to_string(),
        crate::commands::QueryCommand::GetQueue => "GetQueue".to_string(),
        crate::commands::QueryCommand::SearchMedia { .. } => "SearchMedia".to_string(),
        crate::commands::QueryCommand::GetPlaylistInfo { .. } => "GetPlaylistInfo".to_string(),
        crate::commands::QueryCommand::GetArtistInfo { .. } => "GetArtistInfo".to_string(),
        crate::commands::QueryCommand::GetRecommendations { .. } => "GetRecommendations".to_string(),
        crate::commands::QueryCommand::WhatsPlaying => "WhatsPlaying".to_string(),
        crate::commands::QueryCommand::Help { .. } => "Help".to_string(),
    }
}

/// Base intent handler implementation
pub struct BaseIntentHandler {
    intent_name: String,
    supported_slots: Vec<String>,
}

impl BaseIntentHandler {
    pub fn new(intent_name: impl Into<String>) -> Self {
        Self {
            intent_name: intent_name.into(),
            supported_slots: vec![],
        }
    }
    
    pub fn with_slots(mut self, slots: Vec<String>) -> Self {
        self.supported_slots = slots;
        self
    }
}

#[async_trait]
impl IntentHandler for BaseIntentHandler {
    fn intent_name(&self) -> &str {
        &self.intent_name
    }
    
    fn can_handle(&self, _command: &VoiceCommand) -> bool {
        true
    }
    
    async fn handle(&self, _command: VoiceCommand) -> Result<IntentResult, VoiceAssistantError> {
        Ok(IntentResult::success(VoiceResponse::default()))
    }
    
    fn supported_slots(&self) -> Vec<String> {
        self.supported_slots.clone()
    }
}