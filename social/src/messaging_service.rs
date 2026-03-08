//! Direct messaging service

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::{SocialError, SocialResult};
use crate::types::*;
use crate::user_service::UserService;

/// Trait for direct messaging
#[async_trait]
pub trait MessagingService: Send + Sync {
    /// Create a new direct message conversation
    async fn create_conversation(&self, participants: Vec<UserId>) -> SocialResult<Conversation>;

    /// Create a group conversation
    async fn create_group_conversation(&self, creator: UserId, name: String, members: Vec<UserId>) -> SocialResult<Conversation>;

    /// Get conversation by ID
    async fn get_conversation(&self, conversation_id: &ConversationId) -> SocialResult<Conversation>;

    /// Get all conversations for a user
    async fn get_user_conversations(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Conversation>>;

    /// Send a message in a conversation
    async fn send_message(&self, message: SendMessageRequest) -> SocialResult<Message>;

    /// Get messages in a conversation
    async fn get_messages(&self, conversation_id: &ConversationId, limit: usize, before: Option<MessageId>) -> SocialResult<Vec<Message>>;

    /// Mark messages as read
    async fn mark_as_read(&self, conversation_id: &ConversationId, user_id: &UserId, up_to: &MessageId) -> SocialResult<()>;

    /// Edit a message
    async fn edit_message(&self, message_id: &MessageId, sender_id: &UserId, new_content: String) -> SocialResult<Message>;

    /// Delete a message
    async fn delete_message(&self, message_id: &MessageId, sender_id: &UserId) -> SocialResult<()>;

    /// Add participant to group
    async fn add_participant(&self, conversation_id: &ConversationId, added_by: &UserId, new_participant: UserId) -> SocialResult<()>;

    /// Remove participant from group
    async fn remove_participant(&self, conversation_id: &ConversationId, removed_by: &UserId, participant: &UserId) -> SocialResult<()>;

    /// Leave a conversation
    async fn leave_conversation(&self, conversation_id: &ConversationId, user_id: &UserId) -> SocialResult<()>;

    /// Update group info
    async fn update_group_info(&self, conversation_id: &ConversationId, updated_by: &UserId, updates: GroupUpdate) -> SocialResult<Conversation>;

    /// Get unread count for user
    async fn get_unread_count(&self, user_id: &UserId) -> SocialResult<u64>;

    /// Subscribe to new messages
    async fn subscribe(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<MessageEvent>>;

    /// Set typing indicator
    async fn set_typing(&self, conversation_id: &ConversationId, user_id: &UserId, is_typing: bool) -> SocialResult<()>;
}

/// Request to send a message
#[derive(Debug, Clone)]
pub struct SendMessageRequest {
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    pub content: MessageContent,
    pub reply_to: Option<MessageId>,
}

/// Updates for group conversation
#[derive(Debug, Clone, Default)]
pub struct GroupUpdate {
    pub name: Option<String>,
    pub avatar: Option<String>,
}

/// Message event for real-time updates
#[derive(Debug, Clone)]
pub enum MessageEvent {
    NewMessage(Message),
    MessageEdited(Message),
    MessageDeleted(MessageId),
    UserTyping { conversation_id: ConversationId, user_id: UserId, is_typing: bool },
    ParticipantAdded { conversation_id: ConversationId, user_id: UserId },
    ParticipantRemoved { conversation_id: ConversationId, user_id: UserId },
    GroupUpdated { conversation_id: ConversationId },
}

/// Typing indicator
#[derive(Debug, Clone)]
struct TypingIndicator {
    user_id: UserId,
    conversation_id: ConversationId,
    last_updated: chrono::DateTime<Utc>,
}

/// Default in-memory implementation of MessagingService
pub struct DefaultMessagingService {
    conversations: DashMap<ConversationId, Conversation>,
    messages: DashMap<MessageId, Message>,
    user_conversations: DashMap<UserId, Vec<ConversationId>>,
    typing_indicators: RwLock<Vec<TypingIndicator>>,
    event_senders: RwLock<HashMap<UserId, broadcast::Sender<MessageEvent>>>,
    user_service: Arc<dyn UserService>,
}

impl DefaultMessagingService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self {
            conversations: DashMap::new(),
            messages: DashMap::new(),
            user_conversations: DashMap::new(),
            typing_indicators: RwLock::new(Vec::new()),
            event_senders: RwLock::new(HashMap::new()),
            user_service,
        }
    }

    fn broadcast_event(&self, user_id: &UserId, event: MessageEvent) {
        let senders = self.event_senders.read();
        if let Some(sender) = senders.get(user_id) {
            let _ = sender.send(event);
        }
    }

    fn broadcast_to_conversation(&self, conversation: &Conversation, event: MessageEvent) {
        for participant in &conversation.participants {
            self.broadcast_event(participant, event.clone());
        }
    }

    fn get_or_create_sender(&self, user_id: &UserId) -> broadcast::Sender<MessageEvent> {
        let mut senders = self.event_senders.write();
        senders.entry(user_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone()
    }
}

#[async_trait]
impl MessagingService for DefaultMessagingService {
    async fn create_conversation(&self, participants: Vec<UserId>) -> SocialResult<Conversation> {
        if participants.len() != 2 {
            return Err(SocialError::InvalidOperation("Direct conversation requires exactly 2 participants".to_string()));
        }

        // Verify users exist
        for user_id in &participants {
            self.user_service.get_profile(user_id).await?;
        }

        let conversation = Conversation::direct(participants[0].clone(), participants[1].clone());
        let conversation_id = conversation.id.clone();

        // Add to user conversations
        for user_id in &conversation.participants {
            let mut convs = self.user_conversations.entry(user_id.clone()).or_insert_with(Vec::new);
            convs.push(conversation_id.clone());
        }

        self.conversations.insert(conversation_id, conversation.clone());

        Ok(conversation)
    }

    async fn create_group_conversation(&self, creator: UserId, name: String, members: Vec<UserId>) -> SocialResult<Conversation> {
        // Verify all users exist
        self.user_service.get_profile(&creator).await?;
        for user_id in &members {
            self.user_service.get_profile(user_id).await?;
        }

        let conversation = Conversation::group(creator.clone(), name, members);

        // Add to user conversations
        for user_id in &conversation.participants {
            let mut convs = self.user_conversations.entry(user_id.clone()).or_insert_with(Vec::new);
            convs.push(conversation.id.clone());
        }

        let conversation_id = conversation.id.clone();
        self.conversations.insert(conversation_id, conversation.clone());

        Ok(conversation)
    }

    async fn get_conversation(&self, conversation_id: &ConversationId) -> SocialResult<Conversation> {
        self.conversations
            .get(conversation_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::ConversationNotFound(conversation_id.0.to_string()))
    }

    async fn get_user_conversations(&self, user_id: &UserId, limit: usize, offset: usize) -> SocialResult<Vec<Conversation>> {
        let conv_ids = self.user_conversations
            .get(user_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut conversations: Vec<Conversation> = conv_ids
            .into_iter()
            .skip(offset)
            .take(limit)
            .filter_map(|id| self.conversations.get(&id).map(|r| r.clone()))
            .collect();

        // Sort by updated_at descending
        conversations.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(conversations)
    }

    async fn send_message(&self, message: SendMessageRequest) -> SocialResult<Message> {
        // Verify sender exists
        self.user_service.get_profile(&message.sender_id).await?;

        // Get and verify conversation
        let mut conversation = self.get_conversation(&message.conversation_id).await?;

        // Verify sender is a participant
        if !conversation.participants.contains(&message.sender_id) {
            return Err(SocialError::Unauthorized);
        }

        // Create message
        let mut new_message = Message::new(
            message.conversation_id.clone(),
            message.sender_id.clone(),
            message.content,
        );

        if let Some(reply_to) = message.reply_to {
            // Verify reply-to message exists
            if self.messages.get(&reply_to).is_some() {
                // Store reply_to in a custom field would need extending Message
            }
        }

        let message_id = new_message.id.clone();
        self.messages.insert(message_id.clone(), new_message.clone());

        // Update conversation
        conversation.updated_at = Utc::now();
        conversation.last_message = Some(new_message.clone());

        // Increment unread count for all participants except sender
        for participant in &conversation.participants {
            if participant != &message.sender_id {
                *conversation.unread_count.entry(participant.clone()).or_insert(0) += 1;
            }
        }

        self.conversations.insert(conversation.id.clone(), conversation.clone());

        // Broadcast event
        self.broadcast_to_conversation(&conversation, MessageEvent::NewMessage(new_message.clone()));

        Ok(new_message)
    }

    async fn get_messages(&self, conversation_id: &ConversationId, limit: usize, before: Option<MessageId>) -> SocialResult<Vec<Message>> {
        // Verify conversation exists
        self.get_conversation(conversation_id).await?;

        let mut messages: Vec<Message> = self.messages
            .iter()
            .filter(|e| &e.value().conversation_id == conversation_id)
            .filter(|e| e.value().deleted_at.is_none())
            .map(|e| e.value().clone())
            .collect();

        // Sort by created_at descending (newest first)
        messages.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Filter by 'before' if provided
        if let Some(before_id) = before {
            if let Some(pos) = messages.iter().position(|m| m.id == before_id) {
                messages = messages.into_iter().skip(pos + 1).collect();
            }
        }

        messages = messages.into_iter().take(limit).collect();

        Ok(messages)
    }

    async fn mark_as_read(&self, conversation_id: &ConversationId, user_id: &UserId, up_to: &MessageId) -> SocialResult<()> {
        let mut conversation = self.get_conversation(conversation_id).await?;

        // Verify user is a participant
        if !conversation.participants.contains(user_id) {
            return Err(SocialError::Unauthorized);
        }

        // Get all messages up to the specified one
        let messages = self.get_messages(conversation_id, 1000, None).await?;
        let mut read_count = 0u64;

        for msg in messages.into_iter().rev() {
            if msg.id == *up_to {
                break;
            }
            read_count += 1;
        }

        // Update unread count
        let current_unread = conversation.unread_count.entry(user_id.clone()).or_insert(0);
        *current_unread = current_unread.saturating_sub(read_count);

        // Add read receipt
        let receipt = MessageReadReceipt {
            user_id: user_id.clone(),
            read_at: Utc::now(),
        };

        // Update messages with read receipt
        if let Some((_, mut msg)) = self.messages.remove(up_to) {
            msg.read_by.push(receipt);
            self.messages.insert(up_to.clone(), msg);
        }

        self.conversations.insert(conversation_id.clone(), conversation);

        Ok(())
    }

    async fn edit_message(&self, message_id: &MessageId, sender_id: &UserId, new_content: String) -> SocialResult<Message> {
        let mut message = self.messages
            .get(message_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::MessageNotFound(message_id.0.to_string()))?;

        // Verify sender
        if &message.sender_id != sender_id {
            return Err(SocialError::Unauthorized);
        }

        // Verify message not deleted
        if message.deleted_at.is_some() {
            return Err(SocialError::InvalidOperation("Message has been deleted".to_string()));
        }

        // Update content
        message.content = MessageContent::text(new_content);
        message.edited_at = Some(Utc::now());

        self.messages.insert(message_id.clone(), message.clone());

        // Broadcast edit event
        if let Ok(conversation) = self.get_conversation(&message.conversation_id).await {
            self.broadcast_to_conversation(&conversation, MessageEvent::MessageEdited(message.clone()));
        }

        Ok(message)
    }

    async fn delete_message(&self, message_id: &MessageId, sender_id: &UserId) -> SocialResult<()> {
        let mut message = self.messages
            .get(message_id)
            .map(|r| r.clone())
            .ok_or_else(|| SocialError::MessageNotFound(message_id.0.to_string()))?;

        // Verify sender
        if &message.sender_id != sender_id {
            return Err(SocialError::Unauthorized);
        }

        message.deleted_at = Some(Utc::now());
        self.messages.insert(message_id.clone(), message.clone());

        // Broadcast delete event
        if let Ok(conversation) = self.get_conversation(&message.conversation_id).await {
            self.broadcast_to_conversation(&conversation, MessageEvent::MessageDeleted(message_id.clone()));
        }

        Ok(())
    }

    async fn add_participant(&self, conversation_id: &ConversationId, added_by: &UserId, new_participant: UserId) -> SocialResult<()> {
        let mut conversation = self.get_conversation(conversation_id).await?;

        // Must be a group
        if !conversation.is_group {
            return Err(SocialError::InvalidOperation("Cannot add participants to direct conversation".to_string()));
        }

        // Verify adder is a participant
        if !conversation.participants.contains(added_by) {
            return Err(SocialError::Unauthorized);
        }

        // Verify new participant exists
        self.user_service.get_profile(&new_participant).await?;

        // Check if already in group
        if conversation.participants.contains(&new_participant) {
            return Err(SocialError::AlreadyInGroup);
        }

        conversation.participants.push(new_participant.clone());
        conversation.updated_at = Utc::now();

        // Add to user conversations
        let mut convs = self.user_conversations.entry(new_participant.clone()).or_insert_with(Vec::new);
        convs.push(conversation_id.clone());

        self.conversations.insert(conversation_id.clone(), conversation.clone());

        // Broadcast event
        self.broadcast_to_conversation(&conversation, MessageEvent::ParticipantAdded {
            conversation_id: conversation_id.clone(),
            user_id: new_participant,
        });

        Ok(())
    }

    async fn remove_participant(&self, conversation_id: &ConversationId, removed_by: &UserId, participant: &UserId) -> SocialResult<()> {
        let mut conversation = self.get_conversation(conversation_id).await?;

        // Must be a group
        if !conversation.is_group {
            return Err(SocialError::InvalidOperation("Cannot remove participants from direct conversation".to_string()));
        }

        // Verify remover is a participant
        if !conversation.participants.contains(removed_by) {
            return Err(SocialError::Unauthorized);
        }

        // Find and remove participant
        if let Some(pos) = conversation.participants.iter().position(|p| p == participant) {
            conversation.participants.remove(pos);
        } else {
            return Err(SocialError::NotInGroup);
        }

        conversation.updated_at = Utc::now();
        self.conversations.insert(conversation_id.clone(), conversation.clone());

        // Broadcast event
        self.broadcast_to_conversation(&conversation, MessageEvent::ParticipantRemoved {
            conversation_id: conversation_id.clone(),
            user_id: participant.clone(),
        });

        Ok(())
    }

    async fn leave_conversation(&self, conversation_id: &ConversationId, user_id: &UserId) -> SocialResult<()> {
        self.remove_participant(conversation_id, user_id, user_id).await
    }

    async fn update_group_info(&self, conversation_id: &ConversationId, updated_by: &UserId, updates: GroupUpdate) -> SocialResult<Conversation> {
        let mut conversation = self.get_conversation(conversation_id).await?;

        // Must be a group
        if !conversation.is_group {
            return Err(SocialError::InvalidOperation("Not a group conversation".to_string()));
        }

        // Verify updater is a participant
        if !conversation.participants.contains(updated_by) {
            return Err(SocialError::Unauthorized);
        }

        if let Some(name) = updates.name {
            conversation.group_name = Some(name);
        }
        if let Some(avatar) = updates.avatar {
            conversation.group_avatar = Some(avatar);
        }

        conversation.updated_at = Utc::now();
        self.conversations.insert(conversation_id.clone(), conversation.clone());

        // Broadcast event
        self.broadcast_to_conversation(&conversation, MessageEvent::GroupUpdated {
            conversation_id: conversation_id.clone(),
        });

        Ok(conversation)
    }

    async fn get_unread_count(&self, user_id: &UserId) -> SocialResult<u64> {
        let conv_ids = self.user_conversations
            .get(user_id)
            .map(|r| r.clone())
            .unwrap_or_default();

        let mut total_unread = 0u64;

        for conv_id in conv_ids {
            if let Some(conv) = self.conversations.get(&conv_id) {
                total_unread += conv.unread_count.get(user_id).copied().unwrap_or(0);
            }
        }

        Ok(total_unread)
    }

    async fn subscribe(&self, user_id: &UserId) -> SocialResult<broadcast::Receiver<MessageEvent>> {
        let sender = self.get_or_create_sender(user_id);
        Ok(sender.subscribe())
    }

    async fn set_typing(&self, conversation_id: &ConversationId, user_id: &UserId, is_typing: bool) -> SocialResult<()> {
        // Verify user is in conversation
        let conversation = self.get_conversation(conversation_id).await?;
        if !conversation.participants.contains(user_id) {
            return Err(SocialError::Unauthorized);
        }

        // Update typing indicators
        {
            let mut indicators = self.typing_indicators.write();

            // Remove existing indicator for this user in this conversation
            indicators.retain(|i| !(i.user_id == *user_id && i.conversation_id == *conversation_id));

            // Add new indicator if typing
            if is_typing {
                indicators.push(TypingIndicator {
                    user_id: user_id.clone(),
                    conversation_id: conversation_id.clone(),
                    last_updated: Utc::now(),
                });
            }
        }

        // Broadcast typing event
        self.broadcast_to_conversation(&conversation, MessageEvent::UserTyping {
            conversation_id: conversation_id.clone(),
            user_id: user_id.clone(),
            is_typing,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_service::DefaultUserService;

    #[tokio::test]
    async fn test_create_conversation() {
        let user_service = Arc::new(DefaultUserService::new());
        let user1 = user_service.create_profile("user1".to_string(), "User One".to_string()).await.unwrap();
        let user2 = user_service.create_profile("user2".to_string(), "User Two".to_string()).await.unwrap();

        let msg_service = DefaultMessagingService::new(user_service);

        let conv = msg_service.create_conversation(vec![user1.id.clone(), user2.id.clone()]).await.unwrap();
        assert_eq!(conv.participants.len(), 2);
        assert!(!conv.is_group);
    }

    #[tokio::test]
    async fn test_send_message() {
        let user_service = Arc::new(DefaultUserService::new());
        let user1 = user_service.create_profile("user1".to_string(), "User One".to_string()).await.unwrap();
        let user2 = user_service.create_profile("user2".to_string(), "User Two".to_string()).await.unwrap();

        let msg_service = DefaultMessagingService::new(user_service);

        let conv = msg_service.create_conversation(vec![user1.id.clone(), user2.id.clone()]).await.unwrap();

        let msg = msg_service.send_message(SendMessageRequest {
            conversation_id: conv.id.clone(),
            sender_id: user1.id.clone(),
            content: MessageContent::text("Hello!".to_string()),
            reply_to: None,
        }).await.unwrap();

        assert_eq!(msg.content.text, Some("Hello!".to_string()));
    }

    #[tokio::test]
    async fn test_group_conversation() {
        let user_service = Arc::new(DefaultUserService::new());
        let creator = user_service.create_profile("creator".to_string(), "Creator".to_string()).await.unwrap();
        let member1 = user_service.create_profile("member1".to_string(), "Member 1".to_string()).await.unwrap();
        let member2 = user_service.create_profile("member2".to_string(), "Member 2".to_string()).await.unwrap();

        let msg_service = DefaultMessagingService::new(user_service);

        let group = msg_service.create_group_conversation(
            creator.id.clone(),
            "Test Group".to_string(),
            vec![member1.id.clone(), member2.id.clone()],
        ).await.unwrap();

        assert!(group.is_group);
        assert_eq!(group.group_name, Some("Test Group".to_string()));
        assert_eq!(group.participants.len(), 3);
    }
}