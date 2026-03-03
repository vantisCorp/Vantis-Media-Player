//! Keyboard Shortcuts Editor
//!
//! Allows users to customize all keyboard shortcuts in the media player.

use anyhow::Result;
use iced::{widget::{column, row, text, text_input, button, scrollable, container}, Element, Length, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keyboard shortcut editor state
pub struct ShortcutEditorState {
    /// Current shortcuts
    shortcuts: HashMap<String, Shortcut>,
    
    /// Editing state
    editing: Option<String>,
    
    /// New shortcut input
    new_shortcut: String,
    
    /// Conflict message
    conflict_message: Option<String>,
    
    /// Presets
    presets: Vec<ShortcutPreset>,
    
    /// Selected preset
    selected_preset: Option<usize>,
}

/// Keyboard shortcut
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    /// Action name
    pub action: String,
    
    /// Key combination
    pub keys: Vec<String>,
    
    /// Description
    pub description: String,
    
    /// Category
    pub category: ShortcutCategory,
}

/// Shortcut category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShortcutCategory {
    /// Playback controls
    Playback,
    
    /// Volume controls
    Volume,
    
    /// Navigation
    Navigation,
    
    /// Subtitles
    Subtitles,
    
    /// UI controls
    UI,
    
    /// Custom
    Custom,
}

/// Shortcut preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutPreset {
    /// Preset name
    pub name: String,
    
    /// Shortcuts
    pub shortcuts: HashMap<String, Vec<String>>,
}

/// Shortcut editor message
#[derive(Debug, Clone)]
pub enum Message {
    /// Edit shortcut
    EditShortcut(String),
    
    /// Save shortcut
    SaveShortcut(String),
    
    /// Cancel editing
    CancelEdit,
    
    /// New shortcut input changed
    NewShortcutChanged(String),
    
    /// Load preset
    LoadPreset(usize),
    
    /// Export shortcuts
    ExportShortcuts,
    
    /// Import shortcuts
    ImportShortcuts,
    
    /// Reset to defaults
    ResetToDefaults,
}

impl ShortcutEditorState {
    /// Create a new shortcut editor state
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();
        
        // Default shortcuts
        shortcuts.insert("play_pause".to_string(), Shortcut {
            action: "play_pause".to_string(),
            keys: vec!["Space".to_string()],
            description: "Play or pause".to_string(),
            category: ShortcutCategory::Playback,
        });
        
        shortcuts.insert("stop".to_string(), Shortcut {
            action: "stop".to_string(),
            keys: vec!["S".to_string()],
            description: "Stop playback".to_string(),
            category: ShortcutCategory::Playback,
        });
        
        shortcuts.insert("seek_forward".to_string(), Shortcut {
            action: "seek_forward".to_string(),
            keys: vec!["Right".to_string()],
            description: "Seek forward 10 seconds".to_string(),
            category: ShortcutCategory::Playback,
        });
        
        shortcuts.insert("seek_backward".to_string(), Shortcut {
            action: "seek_backward".to_string(),
            keys: vec!["Left".to_string()],
            description: "Seek backward 10 seconds".to_string(),
            category: ShortcutCategory::Playback,
        });
        
        shortcuts.insert("volume_up".to_string(), Shortcut {
            action: "volume_up".to_string(),
            keys: vec!["Up".to_string()],
            description: "Increase volume".to_string(),
            category: ShortcutCategory::Volume,
        });
        
        shortcuts.insert("volume_down".to_string(), Shortcut {
            action: "volume_down".to_string(),
            keys: vec!["Down".to_string()],
            description: "Decrease volume".to_string(),
            category: ShortcutCategory::Volume,
        });
        
        shortcuts.insert("mute".to_string(), Shortcut {
            action: "mute".to_string(),
            keys: vec!["M".to_string()],
            description: "Toggle mute".to_string(),
            category: ShortcutCategory::Volume,
        });
        
        shortcuts.insert("fullscreen".to_string(), Shortcut {
            action: "fullscreen".to_string(),
            keys: vec!["F".to_string()],
            description: "Toggle fullscreen".to_string(),
            category: ShortcutCategory::UI,
        });
        
        shortcuts.insert("toggle_subtitles".to_string(), Shortcut {
            action: "toggle_subtitles".to_string(),
            keys: vec!["T".to_string()],
            description: "Toggle subtitles".to_string(),
            category: ShortcutCategory::Subtitles,
        });
        
        shortcuts.insert("next_subtitle".to_string(), Shortcut {
            action: "next_subtitle".to_string(),
            keys: vec!["Shift+T".to_string()],
            description: "Next subtitle track".to_string(),
            category: ShortcutCategory::Subtitles,
        });
        
        // Default presets
        let presets = vec![
            ShortcutPreset {
                name: "Default".to_string(),
                shortcuts: HashMap::new(), // Will be populated
            },
            ShortcutPreset {
                name: "VLC Style".to_string(),
                shortcuts: {
                    let mut map = HashMap::new();
                    map.insert("play_pause".to_string(), vec!["Space".to_string()]);
                    map.insert("stop".to_string(), vec!["S".to_string()]);
                    map.insert("seek_forward".to_string(), vec!["Right".to_string()]);
                    map.insert("seek_backward".to_string(), vec!["Left".to_string()]);
                    map.insert("volume_up".to_string(), vec!["Ctrl+Up".to_string()]);
                    map.insert("volume_down".to_string(), vec!["Ctrl+Down".to_string()]);
                    map.insert("mute".to_string(), vec!["M".to_string()]);
                    map.insert("fullscreen".to_string(), vec!["F".to_string()]);
                    map
                },
            },
            ShortcutPreset {
                name: "Media Player Classic".to_string(),
                shortcuts: {
                    let mut map = HashMap::new();
                    map.insert("play_pause".to_string(), vec!["Space".to_string()]);
                    map.insert("stop".to_string(), vec!["S".to_string()]);
                    map.insert("seek_forward".to_string(), vec!["Right".to_string()]);
                    map.insert("seek_backward".to_string(), vec!["Left".to_string()]);
                    map.insert("volume_up".to_string(), vec!["Up".to_string()]);
                    map.insert("volume_down".to_string(), vec!["Down".to_string()]);
                    map.insert("mute".to_string(), vec!["Ctrl+M".to_string()]);
                    map.insert("fullscreen".to_string(), vec!["Enter".to_string()]);
                    map
                },
            },
        ];
        
        Self {
            shortcuts,
            editing: None,
            new_shortcut: String::new(),
            conflict_message: None,
            presets,
            selected_preset: Some(0),
        }
    }
    
    /// Update the shortcut editor state
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EditShortcut(action) => {
                self.editing = Some(action.clone());
                if let Some(shortcut) = self.shortcuts.get(&action) {
                    self.new_shortcut = shortcut.keys.join("+");
                }
                Command::none()
            }
            
            Message::SaveShortcut(action) => {
                if let Some(shortcut) = self.shortcuts.get_mut(&action) {
                    let keys: Vec<String> = self.new_shortcut
                        .split('+')
                        .map(|s| s.trim().to_string())
                        .collect();
                    
                    // Check for conflicts
                    if let Some(conflict) = self.check_conflict(&action, &keys) {
                        self.conflict_message = Some(format!(
                            "Conflict with: {}",
                            conflict
                        ));
                    } else {
                        shortcut.keys = keys;
                        self.conflict_message = None;
                    }
                }
                self.editing = None;
                self.new_shortcut = String::new();
                Command::none()
            }
            
            Message::CancelEdit => {
                self.editing = None;
                self.new_shortcut = String::new();
                self.conflict_message = None;
                Command::none()
            }
            
            Message::NewShortcutChanged(value) => {
                self.new_shortcut = value;
                Command::none()
            }
            
            Message::LoadPreset(index) => {
                if let Some(preset) = self.presets.get(index) {
                    for (action, keys) in &preset.shortcuts {
                        if let Some(shortcut) = self.shortcuts.get_mut(action) {
                            shortcut.keys = keys.clone();
                        }
                    }
                    self.selected_preset = Some(index);
                }
                Command::none()
            }
            
            Message::ExportShortcuts => {
                // Export shortcuts to file
                Command::none()
            }
            
            Message::ImportShortcuts => {
                // Import shortcuts from file
                Command::none()
            }
            
            Message::ResetToDefaults => {
                self.shortcuts = Self::new().shortcuts;
                Command::none()
            }
        }
    }
    
    /// Check for shortcut conflicts
    fn check_conflict(&self, action: &str, keys: &[String]) -> Option<String> {
        for (other_action, shortcut) in &self.shortcuts {
            if other_action != action && shortcut.keys == keys {
                return Some(shortcut.description.clone());
            }
        }
        None
    }
    
    /// Get shortcuts by category
    pub fn get_shortcuts_by_category(&self, category: &ShortcutCategory) -> Vec<&Shortcut> {
        self.shortcuts
            .values()
            .filter(|s| &s.category == category)
            .collect()
    }
    
    /// Get all shortcuts
    pub fn get_all_shortcuts(&self) -> &HashMap<String, Shortcut> {
        &self.shortcuts
    }
    
    /// View the shortcut editor
    pub fn view(&self) -> Element<Message> {
        let mut content = column![];
        
        // Title
        content = content.push(
            text("Keyboard Shortcuts")
                .size(24)
        );
        
        // Presets
        let mut preset_row = row![];
        preset_row = preset_row.push(text("Presets: "));
        for (i, preset) in self.presets.iter().enumerate() {
            let is_selected = self.selected_preset == Some(i);
            preset_row = preset_row.push(
                button(preset.name.clone())
                    .on_press(Message::LoadPreset(i))
            );
        }
        content = content.push(preset_row.spacing(10));
        
        // Action buttons
        let mut action_row = row![];
        action_row = action_row.push(
            button("Export")
                .on_press(Message::ExportShortcuts)
        );
        action_row = action_row.push(
            button("Import")
                .on_press(Message::ImportShortcuts)
        );
        action_row = action_row.push(
            button("Reset to Defaults")
                .on_press(Message::ResetToDefaults)
        );
        content = content.push(action_row.spacing(10));
        
        // Conflict message
        if let Some(message) = &self.conflict_message {
            content = content.push(
                text(message)
                    .size(14)
                    .style(|theme| iced::theme::Text::Color(iced::Color::from_rgb(1.0, 0.0, 0.0)))
            );
        }
        
        // Shortcuts by category
        let categories = vec![
            ShortcutCategory::Playback,
            ShortcutCategory::Volume,
            ShortcutCategory::Navigation,
            ShortcutCategory::Subtitles,
            ShortcutCategory::UI,
        ];
        
        for category in categories {
            let shortcuts = self.get_shortcuts_by_category(&category);
            if !shortcuts.is_empty() {
                content = content.push(
                    text(format!("{:?}", category))
                        .size(18)
                );
                
                for shortcut in shortcuts {
                    let action = shortcut.action.clone();
                    let is_editing = self.editing.as_ref() == Some(&action);
                    
                    if is_editing {
                        let mut edit_row = row![];
                        edit_row = edit_row.push(text(&shortcut.description));
                        edit_row = edit_row.push(
                            text_input("Shortcut", &self.new_shortcut)
                                .on_input(Message::NewShortcutChanged)
                        );
                        edit_row = edit_row.push(
                            button("Save")
                                .on_press(Message::SaveShortcut(action.clone()))
                        );
                        edit_row = edit_row.push(
                            button("Cancel")
                                .on_press(Message::CancelEdit)
                        );
                        content = content.push(edit_row.spacing(10));
                    } else {
                        let mut shortcut_row = row![];
                        shortcut_row = shortcut_row.push(text(&shortcut.description));
                        shortcut_row = shortcut_row.push(text(shortcut.keys.join(" + ")));
                        shortcut_row = shortcut_row.push(
                            button("Edit")
                                .on_press(Message::EditShortcut(action.clone()))
                        );
                        content = content.push(shortcut_row.spacing(10));
                    }
                }
            }
        }
        
        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for ShortcutEditorState {
    fn default() -> Self {
        Self::new()
    }
}