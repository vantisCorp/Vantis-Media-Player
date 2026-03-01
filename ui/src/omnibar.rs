//! Omnibar Command System
//! 
//! Universal command bar (Ctrl+K) for quick access to all functions.

use std::collections::HashMap;

/// Omnibar state
pub struct OmnibarState {
    /// Current input
    input: String,
    
    /// Is open
    is_open: bool,
    
    /// Command history
    history: Vec<String>,
    
    /// Available commands
    commands: HashMap<String, Command>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Submit,
    Open,
    Close,
}

/// Command
#[derive(Debug, Clone)]
pub struct Command {
    /// Command name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Action to execute
    pub action: Box<dyn Fn() -> () + Send + Sync>,
}

impl OmnibarState {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        
        // Register default commands
        commands.insert("play".to_string(), Command {
            name: "play".to_string(),
            description: "Resume playback".to_string(),
            action: Box::new(|| println!("Play")),
        });
        
        commands.insert("pause".to_string(), Command {
            name: "pause".to_string(),
            description: "Pause playback".to_string(),
            action: Box::new(|| println!("Pause")),
        });
        
        commands.insert("stop".to_string(), Command {
            name: "stop".to_string(),
            description: "Stop playback".to_string(),
            action: Box::new(|| println!("Stop")),
        });
        
        commands.insert("fullscreen".to_string(), Command {
            name: "fullscreen".to_string(),
            description: "Toggle fullscreen".to_string(),
            action: Box::new(|| println!("Fullscreen")),
        });
        
        commands.insert("subtitles download".to_string(), Command {
            name: "subtitles download".to_string(),
            description: "Download subtitles".to_string(),
            action: Box::new(|| println!("Download subtitles")),
        });
        
        commands.insert("settings".to_string(), Command {
            name: "settings".to_string(),
            description: "Open settings".to_string(),
            action: Box::new(|| println!("Settings")),
        });
        
        Self {
            input: String::new(),
            is_open: false,
            history: Vec::new(),
            commands,
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
            }
            Message::Submit => {
                if !self.input.is_empty() {
                    self.history.push(self.input.clone());
                    self.execute_command(&self.input);
                    self.input.clear();
                    self.is_open = false;
                }
            }
            Message::Open => {
                self.is_open = true;
            }
            Message::Close => {
                self.is_open = false;
                self.input.clear();
            }
        }
    }
    
    fn execute_command(&self, input: &str) {
        // Find matching command
        let command = self.commands.values()
            .find(|c| input.to_lowercase().contains(&c.name.to_lowercase()));
        
        if let Some(cmd) = command {
            tracing::info!("⌨️ Executing command: {}", cmd.name);
            (cmd.action)();
        } else {
            tracing::warn!("⚠️ Unknown command: {}", input);
        }
    }
    
    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Default for OmnibarState {
    fn default() -> Self {
        Self::new()
    }
}