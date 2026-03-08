//! Response types for voice assistant interactions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main response type for voice assistants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    /// Response ID
    pub id: String,
    
    /// Text to speak
    pub speech: Speech,
    
    /// Visual card to display
    pub card: Option<Card>,
    
    /// Directives to execute
    pub directives: Vec<Directive>,
    
    /// Reprompt text (for continued conversation)
    pub reprompt: Option<Speech>,
    
    /// Whether the session should end
    pub should_end_session: bool,
    
    /// Session attributes to maintain
    pub session_attributes: HashMap<String, serde_json::Value>,
    
    /// Additional metadata
    pub metadata: ResponseMetadata,
}

impl Default for VoiceResponse {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            speech: Speech::plain(""),
            card: None,
            directives: vec![],
            reprompt: None,
            should_end_session: true,
            session_attributes: HashMap::new(),
            metadata: ResponseMetadata::default(),
        }
    }
}

impl VoiceResponse {
    /// Create a simple text response
    pub fn text(message: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            speech: Speech::plain(message),
            card: None,
            directives: vec![],
            reprompt: None,
            should_end_session: true,
            session_attributes: HashMap::new(),
            metadata: ResponseMetadata::default(),
        }
    }
    
    /// Create a response with SSML
    pub fn ssml(ssml: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            speech: Speech::ssml(ssml),
            card: None,
            directives: vec![],
            reprompt: None,
            should_end_session: true,
            session_attributes: HashMap::new(),
            metadata: ResponseMetadata::default(),
        }
    }
    
    /// Add a card
    pub fn with_card(mut self, card: Card) -> Self {
        self.card = Some(card);
        self
    }
    
    /// Add a directive
    pub fn with_directive(mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self
    }
    
    /// Set reprompt
    pub fn with_reprompt(mut self, text: impl Into<String>) -> Self {
        self.reprompt = Some(Speech::plain(text));
        self
    }
    
    /// Set session end behavior
    pub fn with_end_session(mut self, end: bool) -> Self {
        self.should_end_session = end;
        self
    }
    
    /// Add session attribute
    pub fn with_attribute(mut self, key: String, value: serde_json::Value) -> Self {
        self.session_attributes.insert(key, value);
        self
    }
}

/// Speech output type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speech {
    /// Plain text version
    pub text: String,
    
    /// SSML version (Speech Synthesis Markup Language)
    pub ssml: Option<String>,
    
    /// Speech type
    pub speech_type: SpeechType,
}

impl Speech {
    /// Create plain speech
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ssml: None,
            speech_type: SpeechType::PlainText,
        }
    }
    
    /// Create SSML speech
    pub fn ssml(ssml: impl Into<String>) -> Self {
        let ssml_text = ssml.into();
        // Extract plain text from SSML
        let plain = ssml_text
            .replace("<speak>", "")
            .replace("</speak>", "")
            .replace("<break time=&quot;1s&quot;/>", " ")
            .replace("<break/>", " ")
            .replace("<p>", "")
            .replace("</p>", " ");
        
        Self {
            text: plain,
            ssml: Some(ssml_text),
            speech_type: SpeechType::SSML,
        }
    }
}

/// Type of speech output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpeechType {
    PlainText,
    SSML,
}

/// Visual card to display on device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Card type
    pub card_type: CardType,
    
    /// Card title
    pub title: String,
    
    /// Card subtitle
    pub subtitle: Option<String>,
    
    /// Card content/body
    pub content: String,
    
    /// Image for the card
    pub image: Option<CardImage>,
    
    /// Card actions (buttons, links, etc.)
    pub actions: Vec<CardAction>,
}

/// Types of display cards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CardType {
    /// Simple text card
    Simple,
    
    /// Standard card with image
    Standard,
    
    /// Link account card
    LinkAccount,
    
    /// Permission card
    AskForPermission,
    
    /// Media card
    Media,
    
    /// List card
    List,
}

/// Image for card display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImage {
    /// Small image URL (typically 720x480)
    pub small_image_url: Option<String>,
    
    /// Large image URL (typically 1200x800)
    pub large_image_url: Option<String>,
    
    /// Accessibility text
    pub accessibility_text: String,
}

/// Action button on a card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAction {
    /// Action type
    pub action_type: ActionType,
    
    /// Button text
    pub title: String,
    
    /// URL for link actions
    pub url: Option<String>,
    
    /// Permission to request
    pub permission: Option<String>,
}

/// Types of card actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    /// Open URL
    OpenUrl,
    
    /// Link account
    LinkAccount,
    
    /// Request permission
    RequestPermission,
    
    /// Launch app
    LaunchApp,
    
    /// Custom action
    Custom,
}

/// Directive to execute on the device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directive {
    /// Directive type
    pub directive_type: DirectiveType,
    
    /// Directive payload
    pub payload: serde_json::Value,
}

impl Directive {
    /// Create a play directive
    pub fn play(media_url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            directive_type: DirectiveType::Play,
            payload: serde_json::json!({
                "mediaUrl": media_url.into(),
                "title": title.into(),
            }),
        }
    }
    
    /// Create a stop directive
    pub fn stop() -> Self {
        Self {
            directive_type: DirectiveType::Stop,
            payload: serde_json::json!({}),
        }
    }
    
    /// Create a pause directive
    pub fn pause() -> Self {
        Self {
            directive_type: DirectiveType::Pause,
            payload: serde_json::json!({}),
        }
    }
    
    /// Create a resume directive
    pub fn resume() -> Self {
        Self {
            directive_type: DirectiveType::Resume,
            payload: serde_json::json!({}),
        }
    }
    
    /// Create a seek directive
    pub fn seek(position_ms: u64) -> Self {
        Self {
            directive_type: DirectiveType::Seek,
            payload: serde_json::json!({
                "positionMs": position_ms,
            }),
        }
    }
    
    /// Create a set volume directive
    pub fn set_volume(volume: u8) -> Self {
        Self {
            directive_type: DirectiveType::SetVolume,
            payload: serde_json::json!({
                "volume": volume,
            }),
        }
    }
    
    /// Create a clear queue directive
    pub fn clear_queue() -> Self {
        Self {
            directive_type: DirectiveType::ClearQueue,
            payload: serde_json::json!({}),
        }
    }
}

/// Types of directives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectiveType {
    /// Play media
    Play,
    
    /// Stop playback
    Stop,
    
    /// Pause playback
    Pause,
    
    /// Resume playback
    Resume,
    
    /// Seek to position
    Seek,
    
    /// Set volume
    SetVolume,
    
    /// Clear queue
    ClearQueue,
    
    /// Next track
    Next,
    
    /// Previous track
    Previous,
    
    /// Display text
    DisplayText,
    
    /// Play audio
    AudioPlayerPlay,
    
    /// Hint
    Hint,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    
    /// API version
    pub api_version: String,
    
    /// Source of the response
    pub source: String,
    
    /// Locale of the response
    pub locale: String,
    
    /// Additional properties
    pub additional_properties: HashMap<String, String>,
}

impl Default for ResponseMetadata {
    fn default() -> Self {
        Self {
            processing_time_ms: 0,
            api_version: "1.0".to_string(),
            source: "VantisVoice".to_string(),
            locale: "en-US".to_string(),
            additional_properties: HashMap::new(),
        }
    }
}

/// Builder for creating responses
pub struct ResponseBuilder {
    response: VoiceResponse,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response: VoiceResponse::default(),
        }
    }
    
    pub fn speech(mut self, text: impl Into<String>) -> Self {
        self.response.speech = Speech::plain(text);
        self
    }
    
    pub fn ssml(mut self, ssml: impl Into<String>) -> Self {
        self.response.speech = Speech::ssml(ssml);
        self
    }
    
    pub fn card(mut self, card: Card) -> Self {
        self.response.card = Some(card);
        self
    }
    
    pub fn simple_card(self, title: impl Into<String>, content: impl Into<String>) -> Self {
        self.card(Card {
            card_type: CardType::Simple,
            title: title.into(),
            subtitle: None,
            content: content.into(),
            image: None,
            actions: vec![],
        })
    }
    
    pub fn standard_card(
        self,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        content: impl Into<String>,
        image: Option<CardImage>,
    ) -> Self {
        self.card(Card {
            card_type: CardType::Standard,
            title: title.into(),
            subtitle: Some(subtitle.into()),
            content: content.into(),
            image,
            actions: vec![],
        })
    }
    
    pub fn directive(mut self, directive: Directive) -> Self {
        self.response.directives.push(directive);
        self
    }
    
    pub fn reprompt(mut self, text: impl Into<String>) -> Self {
        self.response.reprompt = Some(Speech::plain(text));
        self
    }
    
    pub fn should_end_session(mut self, end: bool) -> Self {
        self.response.should_end_session = end;
        self
    }
    
    pub fn session_attribute(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.response.session_attributes.insert(key.into(), value);
        self
    }
    
    pub fn build(self) -> VoiceResponse {
        self.response
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common responses
pub mod responses {
    use super::*;
    
    pub fn ok() -> VoiceResponse {
        VoiceResponse::text("OK")
    }
    
    pub fn playing(title: &str, artist: &str) -> VoiceResponse {
        ResponseBuilder::new()
            .ssml(format!(
                "<speak>Playing <emphasis level=&quot;moderate&quot;>{}</emphasis> by {}</speak>",
                title, artist
            ))
            .should_end_session(true)
            .build()
    }
    
    pub fn paused() -> VoiceResponse {
        VoiceResponse::text("Paused")
    }
    
    pub fn resumed() -> VoiceResponse {
        VoiceResponse::text("Resuming playback")
    }
    
    pub fn stopped() -> VoiceResponse {
        VoiceResponse::text("Stopped")
    }
    
    pub fn volume_set(level: u8) -> VoiceResponse {
        VoiceResponse::text(format!("Volume set to {} percent", level))
    }
    
    pub fn muted() -> VoiceResponse {
        VoiceResponse::text("Muted")
    }
    
    pub fn unmuted() -> VoiceResponse {
        VoiceResponse::text("Unmuted")
    }
    
    pub fn not_found(query: &str) -> VoiceResponse {
        ResponseBuilder::new()
            .speech(format!("I couldn't find anything matching '{}'. Try a different search.", query))
            .should_end_session(false)
            .reprompt("What would you like to play?")
            .build()
    }
    
    pub fn error(message: &str) -> VoiceResponse {
        ResponseBuilder::new()
            .speech(format!("Sorry, there was an error: {}", message))
            .should_end_session(true)
            .build()
    }
    
    pub fn need_account_linking() -> VoiceResponse {
        ResponseBuilder::new()
            .speech("To use this feature, please link your Vantis account in the Alexa app.")
            .card(Card {
                card_type: CardType::LinkAccount,
                title: "Link Account".to_string(),
                subtitle: None,
                content: "Please link your Vantis Media Player account to enable voice control.".to_string(),
                image: None,
                actions: vec![],
            })
            .should_end_session(true)
            .build()
    }
    
    pub fn welcome() -> VoiceResponse {
        ResponseBuilder::new()
            .ssml("<speak>Welcome to Vantis Media Player! You can say things like 'play some rock music', 'pause', or 'what's playing'.</speak>")
            .should_end_session(false)
            .reprompt("You can ask me to play music, pause, or control playback. What would you like to do?")
            .build()
    }
    
    pub fn goodbye() -> VoiceResponse {
        VoiceResponse::text("Goodbye!")
    }
    
    pub fn help() -> VoiceResponse {
        ResponseBuilder::new()
            .speech("You can control your media by saying things like 'play some jazz', 'pause', 'next track', 'set volume to 50', or 'what's playing'.")
            .should_end_session(false)
            .reprompt("What would you like to do?")
            .build()
    }
}