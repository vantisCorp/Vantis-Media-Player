//! Keyboard Shortcut Manager
//! 
//! Provides customizable keyboard shortcuts for controlling playback and navigation.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, error, debug, warn};

use crate::{ShortcutConfig, Shortcut, ShortcutAction, Key, Modifier};

/// Shortcut manager
pub struct ShortcutManager {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Configuration
    config: Arc<RwLock<ShortcutConfig>>,
    
    /// Active shortcuts
    shortcuts: Arc<RwLock<HashMap<ShortcutAction, Shortcut>>>,
    
    /// Key state tracking
    key_states: Arc<RwLock<HashMap<Key, bool>>>,
    
    /// Modifier state tracking
    modifier_states: Arc<RwLock<HashMap<Modifier, bool>>>,
    
    /// Shortcut callbacks
    callbacks: Arc<RwLock<HashMap<ShortcutAction, ShortcutCallback>>>,
}

/// Shortcut callback
pub type ShortcutCallback = Arc<dyn Fn(ShortcutAction) + Send + Sync>;

/// Key event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// Key
    pub key: Key,
    
    /// Is pressed
    pub is_pressed: bool,
    
    /// Modifiers
    pub modifiers: Vec<Modifier>,
}

impl ShortcutManager {
    /// Create a new shortcut manager
    pub fn new(config: ShortcutConfig) -> Result<Self> {
        info!("Initializing shortcut manager");
        
        // Build shortcuts map
        let mut shortcuts = HashMap::new();
        for shortcut in &config.default_shortcuts {
            shortcuts.insert(shortcut.action, shortcut.clone());
        }
        for shortcut in &config.custom_shortcuts {
            shortcuts.insert(shortcut.action, shortcut.clone());
        }
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config)),
            shortcuts: Arc::new(RwLock::new(shortcuts)),
            key_states: Arc::new(RwLock::new(HashMap::new())),
            modifier_states: Arc::new(RwLock::new(HashMap::new())),
            callbacks: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *self.is_initialized.read().await
            })
        })
    }
    
    /// Set configuration
    pub async fn set_config(&self, config: ShortcutConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Shortcut configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> ShortcutConfig {
        self.config.read().await.clone()
    }
    
    /// Register shortcut callback
    pub async fn register_callback(&self, action: ShortcutAction, callback: ShortcutCallback) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.insert(action, callback);
        info!("Registered callback for {:?}", action);
    }
    
    /// Unregister shortcut callback
    pub async fn unregister_callback(&self, action: ShortcutAction) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.remove(&action);
        info!("Unregistered callback for {:?}", action);
    }
    
    /// Add custom shortcut
    pub async fn add_shortcut(&self, shortcut: Shortcut) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.allow_customization {
            return Err(anyhow::anyhow!("Custom shortcuts are disabled"));
        }
        
        let mut shortcuts = self.shortcuts.write().await;
        shortcuts.insert(shortcut.action, shortcut.clone());
        
        info!("Added custom shortcut for {:?}", shortcut.action);
        Ok(())
    }
    
    /// Remove shortcut
    pub async fn remove_shortcut(&self, action: ShortcutAction) -> Result<()> {
        let config = self.config.read().await;
        
        // Check if it's a default shortcut
        let is_default = config.default_shortcuts.iter().any(|s| s.action == action);
        if is_default {
            return Err(anyhow::anyhow!("Cannot remove default shortcut"));
        }
        
        let mut shortcuts = self.shortcuts.write().await;
        shortcuts.remove(&action);
        
        info!("Removed shortcut for {:?}", action);
        Ok(())
    }
    
    /// Get shortcut for action
    pub async fn get_shortcut(&self, action: ShortcutAction) -> Option<Shortcut> {
        self.shortcuts.read().await.get(&action).cloned()
    }
    
    /// Get all shortcuts
    pub async fn get_all_shortcuts(&self) -> Vec<Shortcut> {
        self.shortcuts.read().await.values().cloned().collect()
    }
    
    /// Get default shortcuts
    pub async fn get_default_shortcuts(&self) -> Vec<Shortcut> {
        self.config.read().await.default_shortcuts.clone()
    }
    
    /// Get custom shortcuts
    pub async fn get_custom_shortcuts(&self) -> Vec<Shortcut> {
        self.config.read().await.custom_shortcuts.clone()
    }
    
    /// Reset to default shortcuts
    pub async fn reset_to_defaults(&self) -> Result<()> {
        let config = self.config.read().await;
        
        let mut shortcuts = self.shortcuts.write().await;
        shortcuts.clear();
        
        for shortcut in &config.default_shortcuts {
            shortcuts.insert(shortcut.action, shortcut.clone());
        }
        
        info!("Reset to default shortcuts");
        Ok(())
    }
    
    /// Handle key press
    pub async fn handle_key_press(&self, key: Key, modifiers: Vec<Modifier>) -> Result<()> {
        // Update key state
        let mut key_states = self.key_states.write().await;
        key_states.insert(key, true);
        
        // Update modifier states
        let mut modifier_states = self.modifier_states.write().await;
        for modifier in &modifiers {
            modifier_states.insert(*modifier, true);
        }
        
        // Check for shortcut match
        self.check_shortcuts(key, modifiers).await?;
        
        Ok(())
    }
    
    /// Handle key release
    pub async fn handle_key_release(&self, key: Key, modifiers: Vec<Modifier>) -> Result<()> {
        // Update key state
        let mut key_states = self.key_states.write().await;
        key_states.insert(key, false);
        
        // Update modifier states
        let mut modifier_states = self.modifier_states.write().await;
        for modifier in &modifiers {
            modifier_states.insert(*modifier, false);
        }
        
        Ok(())
    }
    
    /// Check for shortcut matches
    async fn check_shortcuts(&self, key: Key, modifiers: Vec<Modifier>) -> Result<()> {
        let shortcuts = self.shortcuts.read().await;
        let callbacks = self.callbacks.read().await;
        
        for (action, shortcut) in shortcuts.iter() {
            // Check if keys match
            if shortcut.keys.len() == 1 && shortcut.keys[0] == key {
                // Check if modifiers match
                let modifiers_match = shortcut.modifiers.iter().all(|m| modifiers.contains(m))
                    && modifiers.iter().all(|m| shortcut.modifiers.contains(m));
                
                if modifiers_match {
                    // Dispatch callback
                    if let Some(callback) = callbacks.get(action) {
                        callback(*action);
                        info!("Shortcut triggered: {:?}", action);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get key state
    pub async fn get_key_state(&self, key: Key) -> bool {
        self.key_states.read().await.get(&key).copied().unwrap_or(false)
    }
    
    /// Get modifier state
    pub async fn get_modifier_state(&self, modifier: Modifier) -> bool {
        self.modifier_states.read().await.get(&modifier).copied().unwrap_or(false)
    }
    
    /// Export shortcuts to JSON
    pub async fn export_shortcuts(&self) -> Result<String> {
        let shortcuts = self.get_all_shortcuts().await;
        serde_json::to_string(&shortcuts)
            .context("Failed to export shortcuts")
    }
    
    /// Import shortcuts from JSON
    pub async fn import_shortcuts(&self, json: &str) -> Result<()> {
        let config = self.config.read().await;
        
        if !config.allow_customization {
            return Err(anyhow::anyhow!("Custom shortcuts are disabled"));
        }
        
        let shortcuts: Vec<Shortcut> = serde_json::from_str(json)
            .context("Failed to import shortcuts")?;
        
        let mut shortcuts_map = self.shortcuts.write().await;
        for shortcut in shortcuts {
            shortcuts_map.insert(shortcut.action, shortcut);
        }
        
        info!("Imported {} shortcuts", shortcuts.len());
        Ok(())
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Space => write!(f, "Space"),
            Key::Enter => write!(f, "Enter"),
            Key::Escape => write!(f, "Escape"),
            Key::Tab => write!(f, "Tab"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Delete => write!(f, "Delete"),
            Key::Insert => write!(f, "Insert"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            Key::Up => write!(f, "Up"),
            Key::Down => write!(f, "Down"),
            Key::Left => write!(f, "Left"),
            Key::Right => write!(f, "Right"),
            Key::F1 => write!(f, "F1"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
            Key::A => write!(f, "A"),
            Key::B => write!(f, "B"),
            Key::C => write!(f, "C"),
            Key::D => write!(f, "D"),
            Key::E => write!(f, "E"),
            Key::F => write!(f, "F"),
            Key::G => write!(f, "G"),
            Key::H => write!(f, "H"),
            Key::I => write!(f, "I"),
            Key::J => write!(f, "J"),
            Key::K => write!(f, "K"),
            Key::L => write!(f, "L"),
            Key::M => write!(f, "M"),
            Key::N => write!(f, "N"),
            Key::O => write!(f, "O"),
            Key::P => write!(f, "P"),
            Key::Q => write!(f, "Q"),
            Key::R => write!(f, "R"),
            Key::S => write!(f, "S"),
            Key::T => write!(f, "T"),
            Key::U => write!(f, "U"),
            Key::V => write!(f, "V"),
            Key::W => write!(f, "W"),
            Key::X => write!(f, "X"),
            Key::Y => write!(f, "Y"),
            Key::Z => write!(f, "Z"),
            Key::Digit0 => write!(f, "0"),
            Key::Digit1 => write!(f, "1"),
            Key::Digit2 => write!(f, "2"),
            Key::Digit3 => write!(f, "3"),
            Key::Digit4 => write!(f, "4"),
            Key::Digit5 => write!(f, "5"),
            Key::Digit6 => write!(f, "6"),
            Key::Digit7 => write!(f, "7"),
            Key::Digit8 => write!(f, "8"),
            Key::Digit9 => write!(f, "9"),
        }
    }
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Control => write!(f, "Ctrl"),
            Modifier::Alt => write!(f, "Alt"),
            Modifier::Shift => write!(f, "Shift"),
            Modifier::Meta => write!(f, "Meta"),
        }
    }
}

impl std::fmt::Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        
        for modifier in &self.modifiers {
            parts.push(modifier.to_string());
        }
        
        for key in &self.keys {
            parts.push(key.to_string());
        }
        
        write!(f, "{}", parts.join(" + "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_shortcut_registration() {
        let config = ShortcutConfig::default();
        let manager = ShortcutManager::new(config).unwrap();
        
        let callback_triggered = Arc::new(RwLock::new(false));
        let callback_triggered_clone = callback_triggered.clone();
        
        let callback: ShortcutCallback = Arc::new(move |_| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    *callback_triggered_clone.write().await = true;
                })
            });
        });
        
        manager.register_callback(ShortcutAction::PlayPause, callback).await;
        
        // Trigger shortcut
        manager.handle_key_press(Key::Space, vec![]).await.unwrap();
        
        assert!(*callback_triggered.read().await);
    }
    
    #[tokio::test]
    async fn test_custom_shortcut() {
        let config = ShortcutConfig::default();
        let manager = ShortcutManager::new(config).unwrap();
        
        let custom_shortcut = Shortcut {
            action: ShortcutAction::PlayPause,
            keys: vec![Key::K],
            modifiers: vec![],
        };
        
        manager.add_shortcut(custom_shortcut).await.unwrap();
        
        let shortcut = manager.get_shortcut(ShortcutAction::PlayPause).await;
        assert!(shortcut.is_some());
        assert_eq!(shortcut.unwrap().keys[0], Key::K);
    }
    
    #[tokio::test]
    async fn test_export_import() {
        let config = ShortcutConfig::default();
        let manager = ShortcutManager::new(config).unwrap();
        
        let json = manager.export_shortcuts().await.unwrap();
        
        let manager2 = ShortcutManager::new(ShortcutConfig::default()).unwrap();
        manager2.import_shortcuts(&json).await.unwrap();
        
        assert_eq!(
            manager.get_all_shortcuts().await.len(),
            manager2.get_all_shortcuts().await.len()
        );
    }
}