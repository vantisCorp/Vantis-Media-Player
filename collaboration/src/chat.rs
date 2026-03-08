//! Chat System for Collaborative Viewing
//! 
//! Provides real-time chat functionality for viewing sessions.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

use crate::{ParticipantId, SessionId};

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique message ID
    pub id: String,
    
    /// Session ID
    pub session_id: SessionId,
    
    /// Sender participant ID
    pub sender_id: ParticipantId,
    
    /// Sender display name
    pub sender_name: String,
    
    /// Message content
    pub content: String,
    
    /// Message type
    pub message_type: ChatMessageType,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Is edited
    pub is_edited: bool,
    
    /// Is deleted
    pub is_deleted: bool,
    
    /// Reply to message ID (if replying)
    pub reply_to: Option<String>,
    
    /// Reactions to this message
    pub reactions: Vec<MessageReaction>,
    
    /// Mentions in this message
    pub mentions: Vec<ParticipantId>,
}

/// Chat message type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMessageType {
    /// Regular user message
    User,
    /// System message (joined, left, etc.)
    System,
    /// Emote only
    Emote,
    /// Whisper (private message)
    Whisper { recipient_id: ParticipantId },
    /// Command output
    Command,
}

/// Reaction to a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    /// Reaction emoji
    pub emoji: String,
    
    /// Participant who reacted
    pub participant_id: ParticipantId,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ChatMessage {
    /// Create a new chat message
    pub fn new(
        session_id: SessionId,
        sender_id: ParticipantId,
        sender_name: String,
        content: String,
    ) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            session_id,
            sender_id,
            sender_name,
            content,
            message_type: ChatMessageType::User,
            timestamp: Utc::now(),
            is_edited: false,
            is_deleted: false,
            reply_to: None,
            reactions: Vec::new(),
            mentions: Vec::new(),
        }
    }
    
    /// Create a system message
    pub fn system(session_id: SessionId, content: String) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            session_id,
            sender_id: "system".to_string(),
            sender_name: "System".to_string(),
            content,
            message_type: ChatMessageType::System,
            timestamp: Utc::now(),
            is_edited: false,
            is_deleted: false,
            reply_to: None,
            reactions: Vec::new(),
            mentions: Vec::new(),
        }
    }
    
    /// Create a whisper (private message)
    pub fn whisper(
        session_id: SessionId,
        sender_id: ParticipantId,
        sender_name: String,
        recipient_id: ParticipantId,
        content: String,
    ) -> Self {
        Self {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            session_id,
            sender_id,
            sender_name,
            content,
            message_type: ChatMessageType::Whisper { recipient_id },
            timestamp: Utc::now(),
            is_edited: false,
            is_deleted: false,
            reply_to: None,
            reactions: Vec::new(),
            mentions: Vec::new(),
        }
    }
    
    /// Edit the message
    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
        self.is_edited = true;
    }
    
    /// Delete the message
    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.content = "[deleted]".to_string();
    }
    
    /// Add a reaction
    pub fn add_reaction(&mut self, emoji: String, participant_id: ParticipantId) {
        // Check if participant already reacted with this emoji
        let exists = self.reactions.iter().any(|r| 
            r.emoji == emoji && r.participant_id == participant_id
        );
        
        if !exists {
            self.reactions.push(MessageReaction {
                emoji,
                participant_id,
                timestamp: Utc::now(),
            });
        }
    }
    
    /// Remove a reaction
    pub fn remove_reaction(&mut self, emoji: &str, participant_id: &ParticipantId) {
        self.reactions.retain(|r| 
            !(r.emoji == emoji && r.participant_id == *participant_id)
        );
    }
    
    /// Set reply target
    pub fn reply_to(&mut self, message_id: String) {
        self.reply_to = Some(message_id);
    }
    
    /// Add mention
    pub fn add_mention(&mut self, participant_id: ParticipantId) {
        if !self.mentions.contains(&participant_id) {
            self.mentions.push(participant_id);
        }
    }
}

/// Chat manager for a session
pub struct ChatManager {
    /// Message history
    messages: VecDeque<ChatMessage>,
    
    /// Maximum messages to keep
    max_messages: usize,
    
    /// Maximum message length
    max_message_length: usize,
    
    /// Chat enabled
    enabled: bool,
}

impl ChatManager {
    /// Create a new chat manager
    pub fn new(max_messages: usize, max_message_length: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
            max_message_length,
            enabled: true,
        }
    }
    
    /// Add a message
    pub fn add_message(&mut self, mut message: ChatMessage) -> crate::CollaborationResult<String> {
        if !self.enabled {
            return Err(crate::CollaborationError::InvalidState("Chat is disabled".to_string()));
        }
        
        // Truncate if too long
        if message.content.len() > self.max_message_length {
            message.content.truncate(self.max_message_length);
        }
        
        let message_id = message.id.clone();
        
        // Add to history
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        
        self.messages.push_back(message);
        
        Ok(message_id)
    }
    
    /// Get all messages
    pub fn get_messages(&self) -> Vec<&ChatMessage> {
        self.messages.iter().collect()
    }
    
    /// Get messages since a timestamp
    pub fn get_messages_since(&self, since: DateTime<Utc>) -> Vec<&ChatMessage> {
        self.messages.iter()
            .filter(|m| m.timestamp > since)
            .collect()
    }
    
    /// Get messages by participant
    pub fn get_messages_by_participant(&self, participant_id: &ParticipantId) -> Vec<&ChatMessage> {
        self.messages.iter()
            .filter(|m| &m.sender_id == participant_id)
            .collect()
    }
    
    /// Get a specific message
    pub fn get_message(&self, message_id: &str) -> Option<&ChatMessage> {
        self.messages.iter().find(|m| m.id == message_id)
    }
    
    /// Edit a message
    pub fn edit_message(
        &mut self,
        message_id: &str,
        participant_id: &ParticipantId,
        new_content: String,
    ) -> crate::CollaborationResult<()> {
        let message = self.messages.iter_mut()
            .find(|m| m.id == message_id)
            .ok_or_else(|| crate::CollaborationError::InvalidState("Message not found".to_string()))?;
        
        if &message.sender_id != participant_id {
            return Err(crate::CollaborationError::NotAuthorized("Cannot edit others' messages".to_string()));
        }
        
        message.edit(new_content);
        Ok(())
    }
    
    /// Delete a message
    pub fn delete_message(
        &mut self,
        message_id: &str,
        participant_id: &ParticipantId,
    ) -> crate::CollaborationResult<()> {
        let message = self.messages.iter_mut()
            .find(|m| m.id == message_id)
            .ok_or_else(|| crate::CollaborationError::InvalidState("Message not found".to_string()))?;
        
        if &message.sender_id != participant_id {
            return Err(crate::CollaborationError::NotAuthorized("Cannot delete others' messages".to_string()));
        }
        
        message.delete();
        Ok(())
    }
    
    /// Add reaction to message
    pub fn add_reaction(
        &mut self,
        message_id: &str,
        emoji: String,
        participant_id: ParticipantId,
    ) -> crate::CollaborationResult<()> {
        let message = self.messages.iter_mut()
            .find(|m| m.id == message_id)
            .ok_or_else(|| crate::CollaborationError::InvalidState("Message not found".to_string()))?;
        
        message.add_reaction(emoji, participant_id);
        Ok(())
    }
    
    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    /// Enable/disable chat
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if chat is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new(500, 500)
    }
}

/// Chat command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatCommand {
    /// Skip intro
    SkipIntro,
    
    /// Vote to skip
    VoteSkip,
    
    /// Request sync
    RequestSync,
    
    /// Change nickname
    Nickname { new_name: String },
    
    /// Custom command
    Custom { command: String, args: Vec<String> },
}

impl ChatCommand {
    /// Parse a command from chat input
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        
        if !input.starts_with('/') {
            return None;
        }
        
        let parts: Vec<&str> = input[1..].split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }
        
        match parts[0].to_lowercase().as_str() {
            "skipintro" => Some(ChatCommand::SkipIntro),
            "voteskip" => Some(ChatCommand::VoteSkip),
            "sync" => Some(ChatCommand::RequestSync),
            "nick" | "nickname" => {
                if parts.len() > 1 {
                    Some(ChatCommand::Nickname {
                        new_name: parts[1..].join(" "),
                    })
                } else {
                    None
                }
            }
            _ => Some(ChatCommand::Custom {
                command: parts[0].to_string(),
                args: parts[1..].iter().map(|s| s.to_string()).collect(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage::new(
            "sess_1".to_string(),
            "part_1".to_string(),
            "User".to_string(),
            "Hello!".to_string(),
        );
        
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.message_type, ChatMessageType::User);
    }
    
    #[test]
    fn test_system_message() {
        let msg = ChatMessage::system("sess_1".to_string(), "User joined".to_string());
        assert_eq!(msg.message_type, ChatMessageType::System);
    }
    
    #[test]
    fn test_message_edit() {
        let mut msg = ChatMessage::new(
            "sess_1".to_string(),
            "part_1".to_string(),
            "User".to_string(),
            "Hello!".to_string(),
        );
        
        msg.edit("Hello World!".to_string());
        assert!(msg.is_edited);
        assert_eq!(msg.content, "Hello World!");
    }
    
    #[test]
    fn test_message_reaction() {
        let mut msg = ChatMessage::new(
            "sess_1".to_string(),
            "part_1".to_string(),
            "User".to_string(),
            "Hello!".to_string(),
        );
        
        msg.add_reaction("👍".to_string(), "part_2".to_string());
        assert_eq!(msg.reactions.len(), 1);
    }
    
    #[test]
    fn test_chat_command_parse() {
        let cmd = ChatCommand::parse("/skipintro");
        assert!(matches!(cmd, Some(ChatCommand::SkipIntro)));
        
        let cmd = ChatCommand::parse("/nick NewName");
        assert!(matches!(cmd, Some(ChatCommand::Nickname { .. })));
    }
}