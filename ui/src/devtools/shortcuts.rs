//! Shortcut Manager Module
//! 
//! Global shortcut management for the application.

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Global shortcut manager
#[derive(Debug)]
pub struct ShortcutManager {
    /// Registered shortcuts
    shortcuts: HashMap<String, Shortcut>,
    /// Active shortcuts
    active: Vec<String>,
    /// Is enabled
    pub enabled: bool,
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self {
            shortcuts: HashMap::new(),
            active: Vec::new(),
            enabled: true,
        }
    }
}

impl ShortcutManager {
    /// Create a new shortcut manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a shortcut
    pub fn register(&mut self, shortcut: Shortcut) {
        self.shortcuts.insert(shortcut.id.clone(), shortcut);
    }
    
    /// Unregister a shortcut
    pub fn unregister(&mut self, id: &str) {
        self.shortcuts.remove(id);
    }
    
    /// Check if a key combo matches any shortcut
    pub fn check(&mut self, key: &super::keybinds::KeyCode, modifiers: &super::keybinds::Modifiers) -> Option<ShortcutAction> {
        if !self.enabled {
            return None;
        }
        
        for (id, shortcut) in &self.shortcuts {
            if shortcut.matches(key, modifiers) {
                self.active.push(id.clone());
                return Some(shortcut.action.clone());
            }
        }
        None
    }
    
    /// Get all shortcuts
    pub fn all(&self) -> impl Iterator<Item = &Shortcut> {
        self.shortcuts.values()
    }
    
    /// Get shortcuts by category
    pub fn by_category(&self, category: &str) -> Vec<&Shortcut> {
        self.shortcuts
            .values()
            .filter(|s| s.category == category)
            .collect()
    }
    
    /// Enable all shortcuts
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable all shortcuts
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// A shortcut definition
#[derive(Debug, Clone)]
pub struct Shortcut {
    /// Unique ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Category
    pub category: String,
    /// Key combination
    pub combo: super::keybinds::KeyCombo,
    /// Action to perform
    pub action: ShortcutAction,
    /// Is enabled
    pub enabled: bool,
}

impl Shortcut {
    /// Create a new shortcut
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        combo: super::keybinds::KeyCombo,
        action: ShortcutAction,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            category: "General".to_string(),
            combo,
            action,
            enabled: true,
        }
    }
    
    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Set category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }
    
    /// Check if this shortcut matches the key combo
    pub fn matches(&self, key: &super::keybinds::KeyCode, modifiers: &super::keybinds::Modifiers) -> bool {
        self.enabled
            && &self.combo.key == key
            && self.combo.modifiers.ctrl == modifiers.ctrl
            && self.combo.modifiers.alt == modifiers.alt
            && self.combo.modifiers.shift == modifiers.shift
    }
}

/// Action to perform when shortcut is triggered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutAction {
    // Playback
    PlayPause,
    Stop,
    Next,
    Previous,
    SeekForward(i32), // seconds
    SeekBackward(i32),
    VolumeUp,
    VolumeDown,
    Mute,
    Fullscreen,
    Screenshot,
    
    // Navigation
    GoHome,
    GoLibrary,
    GoDiscover,
    GoSearch,
    GoSettings,
    
    // Library
    AddToFavorites,
    CreatePlaylist,
    Refresh,
    
    // System
    OpenSettings,
    Quit,
    Minimize,
    Maximize,
    
    // Developer
    ToggleDevTools,
    Reload,
    Inspect,
    
    // Custom
    Custom(String),
}

impl ShortcutAction {
    /// Get action name
    pub fn name(&self) -> String {
        match self {
            ShortcutAction::PlayPause => "Play/Pause".to_string(),
            ShortcutAction::Stop => "Stop".to_string(),
            ShortcutAction::Next => "Next".to_string(),
            ShortcutAction::Previous => "Previous".to_string(),
            ShortcutAction::SeekForward(s) => format!("Seek Forward {}s", s),
            ShortcutAction::SeekBackward(s) => format!("Seek Backward {}s", s),
            ShortcutAction::VolumeUp => "Volume Up".to_string(),
            ShortcutAction::VolumeDown => "Volume Down".to_string(),
            ShortcutAction::Mute => "Mute".to_string(),
            ShortcutAction::Fullscreen => "Fullscreen".to_string(),
            ShortcutAction::Screenshot => "Screenshot".to_string(),
            ShortcutAction::GoHome => "Go Home".to_string(),
            ShortcutAction::GoLibrary => "Go to Library".to_string(),
            ShortcutAction::GoDiscover => "Go to Discover".to_string(),
            ShortcutAction::GoSearch => "Go to Search".to_string(),
            ShortcutAction::GoSettings => "Go to Settings".to_string(),
            ShortcutAction::AddToFavorites => "Add to Favorites".to_string(),
            ShortcutAction::CreatePlaylist => "Create Playlist".to_string(),
            ShortcutAction::Refresh => "Refresh".to_string(),
            ShortcutAction::OpenSettings => "Open Settings".to_string(),
            ShortcutAction::Quit => "Quit".to_string(),
            ShortcutAction::Minimize => "Minimize".to_string(),
            ShortcutAction::Maximize => "Maximize".to_string(),
            ShortcutAction::ToggleDevTools => "Toggle DevTools".to_string(),
            ShortcutAction::Reload => "Reload".to_string(),
            ShortcutAction::Inspect => "Inspect".to_string(),
            ShortcutAction::Custom(name) => name.clone(),
        }
    }
}

/// Global shortcut manager instance
static SHORTCUT_MANAGER: once_cell::sync::Lazy<Arc<RwLock<ShortcutManager>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(ShortcutManager::new())));

/// Get the global shortcut manager
pub fn shortcut_manager() -> Arc<RwLock<ShortcutManager>> {
    SHORTCUT_MANAGER.clone()
}

/// Register a shortcut globally
pub fn register_shortcut(shortcut: Shortcut) {
    shortcut_manager().write().register(shortcut);
}

/// Check a key combo against global shortcuts
pub fn check_shortcut(
    key: &super::keybinds::KeyCode,
    modifiers: &super::keybinds::Modifiers,
) -> Option<ShortcutAction> {
    shortcut_manager().write().check(key, modifiers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::keybinds::{KeyCode, KeyCombo, Modifiers};
    
    #[test]
    fn test_shortcut_manager() {
        let mut manager = ShortcutManager::new();
        
        let shortcut = Shortcut::new(
            "play_pause",
            "Play/Pause",
            KeyCombo::simple(KeyCode::Space),
            ShortcutAction::PlayPause,
        );
        
        manager.register(shortcut);
        assert!(manager.shortcuts.contains_key("play_pause"));
    }
    
    #[test]
    fn test_shortcut_matching() {
        let mut manager = ShortcutManager::new();
        
        let shortcut = Shortcut::new(
            "test",
            "Test",
            KeyCombo::ctrl(KeyCode::S),
            ShortcutAction::Custom("Save".to_string()),
        );
        
        manager.register(shortcut);
        
        let result = manager.check(&KeyCode::S, &Modifiers { ctrl: true, ..Default::default() });
        assert!(result.is_some());
    }
}