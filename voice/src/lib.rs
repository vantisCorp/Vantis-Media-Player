//! Vantis Voice Control Module
//! 
//! Provides voice control integration for Vantis Media Player,
//! supporting Alexa, Google Assistant, Siri, and local voice recognition.

pub mod commands;
pub mod intents;
pub mod providers;
pub mod recognition;
pub mod synthesis;
pub mod config;

pub use commands::{VoiceCommand, CommandResult};
pub use intents::{Intent, IntentHandler};
pub use providers::{VoiceProvider, VoiceAssistant};
pub use recognition::SpeechRecognizer;
pub use synthesis::SpeechSynthesizer;
pub use config::VoiceConfig;

/// Voice control error types
#[derive(Debug, thiserror::Error)]
pub enum VoiceError {
    #[error("Recognition failed: {0}")]
    RecognitionError(String),
    
    #[error("Synthesis failed: {0}")]
    SynthesisError(String),
    
    #[error("Command not understood: {0}")]
    CommandNotUnderstood(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Not available: {0}")]
    NotAvailable(String),
}

/// Result type for voice operations
pub type VoiceResult<T> = Result<T, VoiceError>;