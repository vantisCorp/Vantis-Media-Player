//! Intent handling system

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use super::commands::{CommandIntent, CommandResult, EntityValue, VoiceCommand};

/// Intent handler trait
#[async_trait]
pub trait IntentHandler: Send + Sync {
    /// Check if this handler can handle the intent
    fn can_handle(&self, intent: &CommandIntent) -> bool;
    
    /// Execute the intent
    async fn execute(&self, command: &VoiceCommand) -> Result<CommandResult>;
    
    /// Get handler name
    fn name(&self) -> &str;
}

/// Intent registry
pub struct IntentRegistry {
    handlers: HashMap<CommandIntent, Arc<dyn IntentHandler>>,
}

impl IntentRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a handler for an intent
    pub fn register(&mut self, intent: CommandIntent, handler: Arc<dyn IntentHandler>) {
        self.handlers.insert(intent, handler);
    }
    
    /// Get handler for intent
    pub fn get_handler(&self, intent: &CommandIntent) -> Option<Arc<dyn IntentHandler>> {
        self.handlers.get(intent).cloned()
    }
    
    /// Process a command
    pub async fn process(&self, command: &VoiceCommand) -> Result<CommandResult> {
        if let Some(handler) = self.get_handler(&command.intent) {
            handler.execute(command).await
        } else {
            Ok(CommandResult::failure("No handler registered for this command"))
        }
    }
}

impl Default for IntentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Intent types for different voice platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    /// Alexa-style intent
    Alexa {
        name: String,
        slots: HashMap<String, SlotValue>,
    },
    
    /// Google Assistant-style intent
    Google {
        name: String,
        params: HashMap<String, ParamValue>,
    },
    
    /// Siri shortcut intent
    Siri {
        identifier: String,
        parameters: HashMap<String, SiriParam>,
    },
    
    /// Local/custom intent
    Local {
        intent: CommandIntent,
        entities: HashMap<String, EntityValue>,
    },
}

/// Alexa slot value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotValue {
    pub name: String,
    pub value: String,
    pub resolutions: Vec<String>,
}

/// Google parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamValue {
    pub name: String,
    pub value: serde_json::Value,
}

/// Siri parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiriParam {
    pub name: String,
    pub value: serde_json::Value,
}

use serde::{Deserialize, Serialize};

/// Intent matcher for natural language processing
pub struct IntentMatcher {
    patterns: Vec<IntentPattern>,
}

/// Pattern for matching intents
#[derive(Debug, Clone)]
struct IntentPattern {
    intent: CommandIntent,
    patterns: Vec<String>,
    entities: Vec<String>,
}

impl IntentMatcher {
    /// Create a new matcher
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }
    
    /// Match text to an intent
    pub fn match_text(&self, text: &str) -> Option<(CommandIntent, HashMap<String, EntityValue>)> {
        let text_lower = text.to_lowercase();
        
        for pattern in &self.patterns {
            for p in &pattern.patterns {
                if self.pattern_matches(p, &text_lower) {
                    let entities = self.extract_entities(&pattern.entities, &text_lower);
                    return Some((pattern.intent.clone(), entities));
                }
            }
        }
        
        None
    }
    
    fn pattern_matches(&self, pattern: &str, text: &str) -> bool {
        // Simple contains check - would be more sophisticated in production
        text.contains(pattern)
    }
    
    fn extract_entities(&self, entity_names: &[String], text: &str) -> HashMap<String, EntityValue> {
        let mut entities = HashMap::new();
        
        for name in entity_names {
            match name.as_str() {
                "time" | "duration" => {
                    // Extract time: "5 minutes", "30 seconds", etc.
                    if let Some(value) = self.extract_duration(text) {
                        entities.insert(name.clone(), EntityValue::String(value));
                    }
                }
                "number" | "volume" => {
                    if let Some(num) = self.extract_number(text) {
                        entities.insert(name.clone(), EntityValue::Number(num));
                    }
                }
                "title" | "name" | "query" => {
                    // Extract the rest as title
                    entities.insert(name.clone(), EntityValue::String(text.to_string()));
                }
                _ => {}
            }
        }
        
        entities
    }
    
    fn extract_duration(&self, text: &str) -> Option<String> {
        // Simple duration extraction
        let words: Vec<&str> = text.split_whitespace().collect();
        for i in 0..words.len().saturating_sub(1) {
            if let Ok(num) = words[i].parse::<f64>() {
                let unit = words[i + 1];
                if unit.contains("second") || unit.contains("sec") {
                    return Some(format!("{}s", num));
                } else if unit.contains("minute") || unit.contains("min") {
                    return Some(format!("{}m", num));
                } else if unit.contains("hour") {
                    return Some(format!("{}h", num));
                }
            }
        }
        None
    }
    
    fn extract_number(&self, text: &str) -> Option<f64> {
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            if let Ok(num) = word.parse::<f64>() {
                return Some(num);
            }
            // Convert word numbers
            match word {
                "one" => return Some(1.0),
                "two" => return Some(2.0),
                "three" => return Some(3.0),
                "four" => return Some(4.0),
                "five" => return Some(5.0),
                "ten" => return Some(10.0),
                "twenty" => return Some(20.0),
                "fifty" => return Some(50.0),
                "hundred" => return Some(100.0),
                _ => {}
            }
        }
        None
    }
    
    fn default_patterns() -> Vec<IntentPattern> {
        vec![
            IntentPattern {
                intent: CommandIntent::Play,
                patterns: vec!["play", "start playing", "begin playback"],
                entities: vec!["query".to_string()],
            },
            IntentPattern {
                intent: CommandIntent::Pause,
                patterns: vec!["pause", "stop playing"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Resume,
                patterns: vec!["resume", "continue", "keep playing"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Stop,
                patterns: vec!["stop"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Next,
                patterns: vec!["next", "skip", "next track", "next song"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Previous,
                patterns: vec!["previous", "go back", "last track", "previous track"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::SetVolume,
                patterns: vec!["set volume", "volume to"],
                entities: vec!["volume".to_string()],
            },
            IntentPattern {
                intent: CommandIntent::IncreaseVolume,
                patterns: vec!["volume up", "turn up", "louder", "increase volume"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::DecreaseVolume,
                patterns: vec!["volume down", "turn down", "quieter", "decrease volume"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Mute,
                patterns: vec!["mute", "silence"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Unmute,
                patterns: vec!["unmute", "turn sound on"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::SeekTo,
                patterns: vec!["seek to", "skip to", "go to", "jump to"],
                entities: vec!["time".to_string()],
            },
            IntentPattern {
                intent: CommandIntent::WhatIsPlaying,
                patterns: vec!["what is playing", "what's playing", "currently playing"],
                entities: vec![],
            },
            IntentPattern {
                intent: CommandIntent::Help,
                patterns: vec!["help", "what can you do"],
                entities: vec![],
            },
        ]
    }
}

impl Default for IntentMatcher {
    fn default() -> Self {
        Self::new()
    }
}