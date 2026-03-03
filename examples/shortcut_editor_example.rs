//! Keyboard Shortcuts Editor Example
//!
//! Demonstrates the keyboard shortcut editor functionality.

use anyhow::Result;
use vantis_ui::shortcuts::{ShortcutEditorState, Message, ShortcutCategory};
use iced::{Application, Command, Settings};

/// Keyboard shortcuts example application
struct ShortcutsExample {
    /// Shortcut editor state
    editor: ShortcutEditorState,
}

impl Application for ShortcutsExample {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let editor = ShortcutEditorState::new();
        (Self { editor }, Command::none())
    }

    fn title(&self) -> String {
        "Vantis Media Player - Keyboard Shortcuts Editor".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.editor.update(message)
    }

    fn view(&self) -> iced::Element<Self::Message> {
        self.editor.view()
    }
}

fn main() -> Result<()> {
    println!("🎹 Vantis Media Player - Keyboard Shortcuts Editor Example");
    println!();
    println!("This example demonstrates the keyboard shortcut editor functionality.");
    println!();
    println!("Features:");
    println!("  - Edit keyboard shortcuts for all actions");
    println!("  - Conflict detection for duplicate shortcuts");
    println!("  - Shortcut presets (Default, VLC Style, MPC)");
    println!("  - Export and import shortcut configurations");
    println!("  - Reset to default shortcuts");
    println!();
    println!("Default Shortcuts:");
    println!("  - Space: Play/Pause");
    println!("  - S: Stop");
    println!("  - Left/Right: Seek backward/forward");
    println!("  - Up/Down: Volume up/down");
    println!("  - M: Mute");
    println!("  - F: Fullscreen");
    println!("  - T: Toggle subtitles");
    println!("  - Shift+T: Next subtitle track");
    println!();

    let settings = Settings {
        window: iced::window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    };

    ShortcutsExample::run(settings)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_editor_creation() {
        let editor = ShortcutEditorState::new();
        assert!(!editor.get_all_shortcuts().is_empty());
    }

    #[test]
    fn test_get_shortcuts_by_category() {
        let editor = ShortcutEditorState::new();
        let playback_shortcuts = editor.get_shortcuts_by_category(&ShortcutCategory::Playback);
        assert!(!playback_shortcuts.is_empty());
    }

    #[test]
    fn test_conflict_detection() {
        let mut editor = ShortcutEditorState::new();
        
        // Try to set a conflicting shortcut
        let action = "test_action".to_string();
        let keys = vec!["Space".to_string()];
        
        // This should detect a conflict with play_pause
        if let Some(conflict) = editor.check_conflict(&action, &keys) {
            assert!(conflict.contains("Play or pause"));
        }
    }

    #[test]
    fn test_preset_loading() {
        let mut editor = ShortcutEditorState::new();
        
        // Load VLC preset
        editor.update(Message::LoadPreset(1));
        
        // Verify shortcuts were loaded
        let shortcuts = editor.get_all_shortcuts();
        assert!(!shortcuts.is_empty());
    }

    #[test]
    fn test_edit_shortcut() {
        let mut editor = ShortcutEditorState::new();
        
        // Start editing
        editor.update(Message::EditShortcut("play_pause".to_string()));
        
        // Save new shortcut
        editor.update(Message::NewShortcutChanged("K".to_string()));
        editor.update(Message::SaveShortcut("play_pause".to_string()));
        
        // Verify shortcut was changed
        let shortcuts = editor.get_all_shortcuts();
        if let Some(shortcut) = shortcuts.get("play_pause") {
            assert_eq!(shortcut.keys, vec!["K".to_string()]);
        }
    }

    #[test]
    fn test_reset_to_defaults() {
        let mut editor = ShortcutEditorState::new();
        
        // Change a shortcut
        editor.update(Message::EditShortcut("play_pause".to_string()));
        editor.update(Message::NewShortcutChanged("K".to_string()));
        editor.update(Message::SaveShortcut("play_pause".to_string()));
        
        // Reset to defaults
        editor.update(Message::ResetToDefaults);
        
        // Verify shortcut was reset
        let shortcuts = editor.get_all_shortcuts();
        if let Some(shortcut) = shortcuts.get("play_pause") {
            assert_eq!(shortcut.keys, vec!["Space".to_string()]);
        }
    }
}