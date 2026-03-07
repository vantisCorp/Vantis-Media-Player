//! Keybind Editor Module
//! 
//! Provides keyboard shortcut editing functionality.

use std::collections::HashMap;
use std::hash::Hash;

/// Keybind editor for managing shortcuts
#[derive(Debug, Clone)]
pub struct KeybindEditor {
    /// Is editing mode active
    pub editing: bool,
    /// Currently editing action
    pub editing_action: Option<String>,
    /// Pending keybind
    pub pending_bind: Option<KeyCombo>,
    /// All keybinds
    pub keybinds: HashMap<String, Vec<KeyCombo>>,
    /// Conflicts
    pub conflicts: Vec<KeybindConflict>,
    /// Categories
    pub categories: Vec<KeybindCategory>,
}

impl Default for KeybindEditor {
    fn default() -> Self {
        Self {
            editing: false,
            editing_action: None,
            pending_bind: None,
            keybinds: HashMap::new(),
            conflicts: Vec::new(),
            categories: vec![
                KeybindCategory::Playback,
                KeybindCategory::Navigation,
                KeybindCategory::Library,
                KeybindCategory::System,
                KeybindCategory::Developer,
            ],
        }
    }
}

impl KeybindEditor {
    /// Create a new keybind editor
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Start editing a keybind
    pub fn start_editing(&mut self, action: impl Into<String>) {
        self.editing = true;
        self.editing_action = Some(action.into());
        self.pending_bind = None;
    }
    
    /// Stop editing
    pub fn stop_editing(&mut self) {
        self.editing = false;
        self.editing_action = None;
        self.pending_bind = None;
    }
    
    /// Record a key press
    pub fn record_key(&mut self, key: KeyCode, modifiers: Modifiers) {
        if self.editing {
            self.pending_bind = Some(KeyCombo {
                key,
                modifiers,
                is_mouse: false,
            });
        }
    }
    
    /// Confirm the pending bind
    pub fn confirm_bind(&mut self) -> bool {
        if let (Some(action), Some(combo)) = (&self.editing_action, &self.pending_bind) {
            // Check for conflicts
            self.check_conflicts(action, combo);
            
            // Add the bind
            self.keybinds
                .entry(action.clone())
                .or_default()
                .push(combo.clone());
            
            self.stop_editing();
            return true;
        }
        false
    }
    
    /// Remove a keybind
    pub fn remove_bind(&mut self, action: &str, combo: &KeyCombo) {
        if let Some(bindings) = self.keybinds.get_mut(action) {
            bindings.retain(|c| c != combo);
        }
    }
    
    /// Reset to defaults
    pub fn reset_defaults(&mut self) {
        self.keybinds = default_keybinds();
        self.conflicts.clear();
    }
    
    /// Check for conflicts
    fn check_conflicts(&mut self, new_action: &str, new_combo: &KeyCombo) {
        for (action, combos) in &self.keybinds {
            if action != new_action && combos.contains(new_combo) {
                self.conflicts.push(KeybindConflict {
                    action1: action.clone(),
                    action2: new_action.to_string(),
                    combo: new_combo.clone(),
                });
            }
        }
    }
    
    /// Get binds for an action
    pub fn get_binds(&self, action: &str) -> Option<&Vec<KeyCombo>> {
        self.keybinds.get(action)
    }
}

/// Key combination
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    /// Main key
    pub key: KeyCode,
    /// Modifier keys
    pub modifiers: Modifiers,
    /// Is this a mouse binding
    pub is_mouse: bool,
}

impl KeyCombo {
    /// Create a new key combo
    pub fn new(key: KeyCode, modifiers: Modifiers) -> Self {
        Self {
            key,
            modifiers,
            is_mouse: false,
        }
    }
    
    /// Create a simple key bind (no modifiers)
    pub fn simple(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: Modifiers::default(),
            is_mouse: false,
        }
    }
    
    /// Create with Ctrl modifier
    pub fn ctrl(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: Modifiers { ctrl: true, ..Default::default() },
            is_mouse: false,
        }
    }
    
    /// Create with Alt modifier
    pub fn alt(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: Modifiers { alt: true, ..Default::default() },
            is_mouse: false,
        }
    }
    
    /// Create with Shift modifier
    pub fn shift(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: Modifiers { shift: true, ..Default::default() },
            is_mouse: false,
        }
    }
    
    /// Format for display
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        
        if self.modifiers.ctrl {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.alt {
            parts.push("Alt".to_string());
        }
        if self.modifiers.shift {
            parts.push("Shift".to_string());
        }
        if self.modifiers.super_key {
            parts.push("Super".to_string());
        }
        
        parts.push(self.key.display());
        
        parts.join(" + ")
    }
}

/// Modifier keys state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
}

/// Key code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Special keys
    Space, Enter, Escape, Tab, Backspace, Delete,
    Insert, Home, End, PageUp, PageDown,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    // Symbols
    Minus, Equal, BracketLeft, BracketRight,
    Backslash, Semicolon, Quote, Comma, Period, Slash,
}

impl KeyCode {
    /// Get display string
    pub fn display(&self) -> String {
        match self {
            KeyCode::A => "A".to_string(),
            KeyCode::B => "B".to_string(),
            KeyCode::Space => "Space".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Escape => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::ArrowUp => "↑".to_string(),
            KeyCode::ArrowDown => "↓".to_string(),
            KeyCode::ArrowLeft => "←".to_string(),
            KeyCode::ArrowRight => "→".to_string(),
            KeyCode::F1 => "F1".to_string(),
            KeyCode::F2 => "F2".to_string(),
            KeyCode::F12 => "F12".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

/// Keybind conflict
#[derive(Debug, Clone)]
pub struct KeybindConflict {
    pub action1: String,
    pub action2: String,
    pub combo: KeyCombo,
}

/// Keybind category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeybindCategory {
    Playback,
    Navigation,
    Library,
    System,
    Developer,
}

impl KeybindCategory {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            KeybindCategory::Playback => "Playback",
            KeybindCategory::Navigation => "Navigation",
            KeybindCategory::Library => "Library",
            KeybindCategory::System => "System",
            KeybindCategory::Developer => "Developer",
        }
    }
}

/// Get default keybinds
fn default_keybinds() -> HashMap<String, Vec<KeyCombo>> {
    let mut map = HashMap::new();
    
    // Playback
    map.insert("play_pause".to_string(), vec![KeyCombo::simple(KeyCode::Space)]);
    map.insert("stop".to_string(), vec![KeyCombo::simple(KeyCode::Escape)]);
    map.insert("seek_forward".to_string(), vec![KeyCombo::simple(KeyCode::ArrowRight)]);
    map.insert("seek_backward".to_string(), vec![KeyCombo::simple(KeyCode::ArrowLeft)]);
    map.insert("volume_up".to_string(), vec![KeyCombo::simple(KeyCode::ArrowUp)]);
    map.insert("volume_down".to_string(), vec![KeyCombo::simple(KeyCode::ArrowDown)]);
    map.insert("mute".to_string(), vec![KeyCombo::simple(KeyCode::M)]);
    map.insert("fullscreen".to_string(), vec![KeyCombo::simple(KeyCode::F)]);
    
    // Navigation
    map.insert("go_home".to_string(), vec![KeyCombo::alt(KeyCode::Home)]);
    map.insert("go_library".to_string(), vec![KeyCombo::alt(KeyCode::Num1)]);
    map.insert("go_search".to_string(), vec![KeyCombo::ctrl(KeyCode::K)]);
    
    // System
    map.insert("settings".to_string(), vec![KeyCombo::ctrl(KeyCode::Comma)]);
    map.insert("quit".to_string(), vec![KeyCombo::ctrl(KeyCode::Q)]);
    
    // Developer
    map.insert("devtools".to_string(), vec![KeyCombo::simple(KeyCode::F12)]);
    
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keybind_editor_default() {
        let editor = KeybindEditor::new();
        assert!(!editor.editing);
    }
    
    #[test]
    fn test_keybind_editing() {
        let mut editor = KeybindEditor::new();
        editor.start_editing("test_action");
        assert!(editor.editing);
        assert_eq!(editor.editing_action, Some("test_action".to_string()));
    }
    
    #[test]
    fn test_key_combo_display() {
        let combo = KeyCombo::ctrl(KeyCode::S);
        assert_eq!(combo.display(), "Ctrl + S");
    }
}