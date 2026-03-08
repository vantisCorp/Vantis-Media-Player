//! Collaboration Configuration
//! 
//! Provides configuration options for collaborative viewing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collaboration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationConfig {
    /// Enable collaborative viewing
    pub enabled: bool,
    
    /// Session settings
    pub sessions: SessionConfig,
    
    /// Chat settings
    pub chat: ChatConfig,
    
    /// Reaction settings
    pub reactions: ReactionConfig,
    
    /// Sync settings
    pub sync: SyncConfig,
    
    /// Network settings
    pub network: NetworkConfig,
    
    /// Privacy settings
    pub privacy: PrivacyConfig,
}

impl Default for CollaborationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sessions: SessionConfig::default(),
            chat: ChatConfig::default(),
            reactions: ReactionConfig::default(),
            sync: SyncConfig::default(),
            network: NetworkConfig::default(),
            privacy: PrivacyConfig::default(),
        }
    }
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Maximum concurrent sessions
    pub max_sessions: usize,
    
    /// Default maximum participants per session
    pub default_max_participants: u32,
    
    /// Maximum participant name length
    pub max_name_length: usize,
    
    /// Session inactivity timeout (minutes)
    pub inactivity_timeout_minutes: i64,
    
    /// Allow public sessions
    pub allow_public_sessions: bool,
    
    /// Allow password-protected sessions
    pub allow_password_protection: bool,
    
    /// Session cleanup interval (minutes)
    pub cleanup_interval_minutes: i64,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_sessions: 100,
            default_max_participants: 20,
            max_name_length: 32,
            inactivity_timeout_minutes: 60,
            allow_public_sessions: true,
            allow_password_protection: true,
            cleanup_interval_minutes: 10,
        }
    }
}

/// Chat configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    /// Enable chat
    pub enabled: bool,
    
    /// Maximum message length
    pub max_message_length: usize,
    
    /// Maximum messages to keep in history
    pub max_history: usize,
    
    /// Enable whispers (private messages)
    pub enable_whispers: bool,
    
    /// Enable message editing
    pub enable_editing: bool,
    
    /// Enable message deletion
    pub enable_deletion: bool,
    
    /// Enable replies
    pub enable_replies: bool,
    
    /// Enable mentions
    pub enable_mentions: bool,
    
    /// Profanity filter enabled
    pub profanity_filter: bool,
    
    /// Blocked words/phrases
    pub blocked_words: Vec<String>,
    
    /// Slow mode (seconds between messages, 0 = disabled)
    pub slow_mode_seconds: u32,
    
    /// Enable commands
    pub enable_commands: bool,
    
    /// Custom commands
    pub custom_commands: HashMap<String, String>,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_message_length: 500,
            max_history: 500,
            enable_whispers: true,
            enable_editing: true,
            enable_deletion: true,
            enable_replies: true,
            enable_mentions: true,
            profanity_filter: false,
            blocked_words: Vec::new(),
            slow_mode_seconds: 0,
            enable_commands: true,
            custom_commands: HashMap::new(),
        }
    }
}

/// Reaction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionConfig {
    /// Enable reactions
    pub enabled: bool,
    
    /// Maximum reactions to keep
    pub max_reactions: usize,
    
    /// Rate limit (reactions per minute per user)
    pub rate_limit: u32,
    
    /// Enable animated reactions
    pub enable_animated: bool,
    
    /// Enable sound reactions
    pub enable_sounds: bool,
    
    /// Allowed emojis (empty = all allowed)
    pub allowed_emojis: Vec<String>,
    
    /// Enable position-based reactions
    pub enable_position_reactions: bool,
    
    /// Reaction persistence duration (minutes, 0 = no persistence)
    pub persistence_duration_minutes: i64,
}

impl Default for ReactionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_reactions: 100,
            rate_limit: 30,
            enable_animated: false,
            enable_sounds: false,
            allowed_emojis: Vec::new(),
            enable_position_reactions: true,
            persistence_duration_minutes: 30,
        }
    }
}

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Sync threshold in milliseconds
    pub sync_threshold_ms: u32,
    
    /// Buffer time before play (seconds)
    pub buffer_time_seconds: u32,
    
    /// Maximum allowed latency (ms)
    pub max_latency_ms: u32,
    
    /// Auto-sync on participant join
    pub auto_sync_on_join: bool,
    
    /// Auto-pause when participant is buffering
    pub auto_pause_on_buffer: bool,
    
    /// Maximum playback rate adjustment
    pub max_rate_adjustment: f32,
    
    /// Sync check interval (ms)
    pub sync_check_interval_ms: u32,
    
    /// Enable predictive sync
    pub predictive_sync: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            sync_threshold_ms: 500,
            buffer_time_seconds: 3,
            max_latency_ms: 2000,
            auto_sync_on_join: true,
            auto_pause_on_buffer: true,
            max_rate_adjustment: 0.1,
            sync_check_interval_ms: 1000,
            predictive_sync: false,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// WebSocket port
    pub websocket_port: u16,
    
    /// Enable TLS
    pub enable_tls: bool,
    
    /// TLS certificate path
    pub tls_cert_path: Option<String>,
    
    /// TLS key path
    pub tls_key_path: Option<String>,
    
    /// Ping interval (seconds)
    pub ping_interval_seconds: u32,
    
    /// Pong timeout (seconds)
    pub pong_timeout_seconds: u32,
    
    /// Maximum message size (bytes)
    pub max_message_size: usize,
    
    /// Enable compression
    pub enable_compression: bool,
    
    /// Connection timeout (seconds)
    pub connection_timeout_seconds: u32,
    
    /// Reconnection attempts
    pub reconnection_attempts: u32,
    
    /// Reconnection delay (ms)
    pub reconnection_delay_ms: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            websocket_port: 8080,
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            ping_interval_seconds: 30,
            pong_timeout_seconds: 10,
            max_message_size: 1024 * 1024, // 1MB
            enable_compression: true,
            connection_timeout_seconds: 30,
            reconnection_attempts: 5,
            reconnection_delay_ms: 1000,
        }
    }
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Store chat history
    pub store_chat_history: bool,
    
    /// Store reaction history
    pub store_reaction_history: bool,
    
    /// Show participant IP to host
    pub show_ip_to_host: bool,
    
    /// Anonymize participant data
    pub anonymize_data: bool,
    
    /// Data retention period (days)
    pub data_retention_days: u32,
    
    /// Require login to join sessions
    pub require_login: bool,
    
    /// Allow guest participants
    pub allow_guests: bool,
    
    /// GDPR compliance mode
    pub gdpr_mode: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            store_chat_history: true,
            store_reaction_history: false,
            show_ip_to_host: false,
            anonymize_data: true,
            data_retention_days: 30,
            require_login: false,
            allow_guests: true,
            gdpr_mode: true,
        }
    }
}

impl CollaborationConfig {
    /// Load from file
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        
        let config: CollaborationConfig = if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?
        };
        
        Ok(config)
    }
    
    /// Save to file
    pub fn save(&self, path: &str) -> Result<(), ConfigError> {
        let content = if path.ends_with(".toml") {
            toml::to_string_pretty(self)
                .map_err(|e| ConfigError::SerializeError(e.to_string()))?
        } else {
            serde_json::to_string_pretty(self)
                .map_err(|e| ConfigError::SerializeError(e.to_string()))?
        };
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.sessions.default_max_participants == 0 {
            return Err(ConfigError::ValidationError(
                "default_max_participants must be greater than 0".to_string()
            ));
        }
        
        if self.chat.max_message_length == 0 {
            return Err(ConfigError::ValidationError(
                "max_message_length must be greater than 0".to_string()
            ));
        }
        
        if self.sync.sync_threshold_ms < 100 {
            return Err(ConfigError::ValidationError(
                "sync_threshold_ms should be at least 100ms".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Configuration error
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
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
        let config = CollaborationConfig::default();
        assert!(config.enabled);
        assert!(config.chat.enabled);
        assert!(config.reactions.enabled);
    }
    
    #[test]
    fn test_config_validation() {
        let config = CollaborationConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_session_config() {
        let config = SessionConfig::default();
        assert_eq!(config.default_max_participants, 20);
    }
    
    #[test]
    fn test_chat_config() {
        let config = ChatConfig::default();
        assert!(config.enable_editing);
        assert!(config.enable_deletion);
    }
}