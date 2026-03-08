//! Plugin API interfaces for host-plugin communication.

use crate::error::PluginResult;
use crate::plugin::PluginId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

/// Host API for plugin-to-host communication.
#[derive(Clone)]
pub struct HostApi {
    /// Host version.
    version: String,
    /// Command sender for async operations.
    command_tx: Option<mpsc::Sender<HostCommand>>,
}

/// Commands that can be sent to the host.
#[derive(Debug)]
pub enum HostCommand {
    /// Log a message.
    Log { level: LogLevel, message: String },
    /// Get a configuration value.
    GetConfig { key: String, response: oneshot::Sender<Option<String>> },
    /// Set a configuration value.
    SetConfig { key: String, value: String },
    /// Register a keyboard shortcut.
    RegisterShortcut { shortcut: String, action: String },
    /// Unregister a keyboard shortcut.
    UnregisterShortcut { shortcut: String },
    /// Show a notification.
    ShowNotification { title: String, message: String },
    /// Open a URL.
    OpenUrl { url: String },
    /// Get plugin by ID.
    GetPlugin { id: PluginId, response: oneshot::Sender<Option<PluginInfo>> },
}

/// Log level for host logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Plugin info for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin ID.
    pub id: PluginId,
    /// Plugin name.
    pub name: String,
    /// Plugin version.
    pub version: String,
    /// Whether the plugin is enabled.
    pub enabled: bool,
}

impl HostApi {
    /// Create a new host API.
    pub fn new(version: String) -> Self {
        HostApi {
            version,
            command_tx: None,
        }
    }

    /// Create with command channel.
    pub fn with_channel(version: String, command_tx: mpsc::Sender<HostCommand>) -> Self {
        HostApi {
            version,
            command_tx: Some(command_tx),
        }
    }

    /// Get the host version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Log a message to the host.
    pub async fn log(&self, level: LogLevel, message: impl Into<String>) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::Log {
                level,
                message: message.into(),
            }).await;
        }
    }

    /// Get a configuration value.
    pub async fn get_config(&self, key: &str) -> Option<String> {
        if let Some(tx) = &self.command_tx {
            let (response_tx, response_rx) = oneshot::channel();
            let _ = tx.send(HostCommand::GetConfig {
                key: key.to_string(),
                response: response_tx,
            }).await;
            response_rx.await.ok().flatten()
        } else {
            None
        }
    }

    /// Set a configuration value.
    pub async fn set_config(&self, key: &str, value: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::SetConfig {
                key: key.to_string(),
                value: value.to_string(),
            }).await;
        }
    }

    /// Register a keyboard shortcut.
    pub async fn register_shortcut(&self, shortcut: &str, action: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::RegisterShortcut {
                shortcut: shortcut.to_string(),
                action: action.to_string(),
            }).await;
        }
    }

    /// Unregister a keyboard shortcut.
    pub async fn unregister_shortcut(&self, shortcut: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::UnregisterShortcut {
                shortcut: shortcut.to_string(),
            }).await;
        }
    }

    /// Show a notification.
    pub async fn show_notification(&self, title: &str, message: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::ShowNotification {
                title: title.to_string(),
                message: message.to_string(),
            }).await;
        }
    }

    /// Open a URL in the default browser.
    pub async fn open_url(&self, url: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(HostCommand::OpenUrl {
                url: url.to_string(),
            }).await;
        }
    }
}

/// Media API for media playback operations.
#[derive(Clone)]
pub struct MediaApi {
    /// Command sender for async operations.
    command_tx: Option<mpsc::Sender<MediaCommand>>,
}

/// Commands for media operations.
#[derive(Debug)]
pub enum MediaCommand {
    /// Play media.
    Play { url: String },
    /// Pause playback.
    Pause,
    /// Resume playback.
    Resume,
    /// Stop playback.
    Stop,
    /// Seek to position.
    Seek { position_ms: u64 },
    /// Set volume.
    SetVolume { volume: f32 },
    /// Get current position.
    GetPosition { response: oneshot::Sender<Option<u64>> },
    /// Get duration.
    GetDuration { response: oneshot::Sender<Option<u64>> },
    /// Get current media info.
    GetMediaInfo { response: oneshot::Sender<Option<MediaInfo>> },
    /// Add subtitle track.
    AddSubtitle { path: PathBuf },
    /// Set audio track.
    SetAudioTrack { track_index: u32 },
}

/// Media information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    /// Media title.
    pub title: Option<String>,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Media URL.
    pub url: String,
    /// Video width.
    pub width: u32,
    /// Video height.
    pub height: u32,
    /// Audio tracks.
    pub audio_tracks: Vec<TrackInfo>,
    /// Subtitle tracks.
    pub subtitle_tracks: Vec<TrackInfo>,
}

/// Track information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    /// Track index.
    pub index: u32,
    /// Track language.
    pub language: Option<String>,
    /// Track title.
    pub title: Option<String>,
    /// Whether this is the default track.
    pub is_default: bool,
}

impl MediaApi {
    /// Create a new media API.
    pub fn new() -> Self {
        MediaApi { command_tx: None }
    }

    /// Create with command channel.
    pub fn with_channel(command_tx: mpsc::Sender<MediaCommand>) -> Self {
        MediaApi { command_tx: Some(command_tx) }
    }

    /// Play media from a URL.
    pub async fn play(&self, url: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::Play { url: url.to_string() }).await;
        }
    }

    /// Pause playback.
    pub async fn pause(&self) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::Pause).await;
        }
    }

    /// Resume playback.
    pub async fn resume(&self) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::Resume).await;
        }
    }

    /// Stop playback.
    pub async fn stop(&self) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::Stop).await;
        }
    }

    /// Seek to a position.
    pub async fn seek(&self, position_ms: u64) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::Seek { position_ms }).await;
        }
    }

    /// Set volume (0.0 to 1.0).
    pub async fn set_volume(&self, volume: f32) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::SetVolume { volume }).await;
        }
    }

    /// Get current playback position.
    pub async fn get_position(&self) -> Option<u64> {
        if let Some(tx) = &self.command_tx {
            let (response_tx, response_rx) = oneshot::channel();
            let _ = tx.send(MediaCommand::GetPosition { response: response_tx }).await;
            response_rx.await.ok().flatten()
        } else {
            None
        }
    }

    /// Get media duration.
    pub async fn get_duration(&self) -> Option<u64> {
        if let Some(tx) = &self.command_tx {
            let (response_tx, response_rx) = oneshot::channel();
            let _ = tx.send(MediaCommand::GetDuration { response: response_tx }).await;
            response_rx.await.ok().flatten()
        } else {
            None
        }
    }

    /// Get current media info.
    pub async fn get_media_info(&self) -> Option<MediaInfo> {
        if let Some(tx) = &self.command_tx {
            let (response_tx, response_rx) = oneshot::channel();
            let _ = tx.send(MediaCommand::GetMediaInfo { response: response_tx }).await;
            response_rx.await.ok().flatten()
        } else {
            None
        }
    }

    /// Add a subtitle track.
    pub async fn add_subtitle(&self, path: PathBuf) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::AddSubtitle { path }).await;
        }
    }

    /// Set the audio track.
    pub async fn set_audio_track(&self, track_index: u32) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(MediaCommand::SetAudioTrack { track_index }).await;
        }
    }
}

impl Default for MediaApi {
    fn default() -> Self {
        Self::new()
    }
}

/// UI API for UI extensions.
#[derive(Clone)]
pub struct UiApi {
    /// Command sender for async operations.
    command_tx: Option<mpsc::Sender<UiCommand>>,
}

/// Commands for UI operations.
#[derive(Debug)]
pub enum UiCommand {
    /// Show a panel.
    ShowPanel { panel_id: String },
    /// Hide a panel.
    HidePanel { panel_id: String },
    /// Add a menu item.
    AddMenuItem { menu: String, item: MenuItem },
    /// Remove a menu item.
    RemoveMenuItem { menu: String, item_id: String },
    /// Show a dialog.
    ShowDialog { dialog: DialogConfig },
    /// Close a dialog.
    CloseDialog { dialog_id: String },
    /// Update status bar.
    UpdateStatusBar { section: String, content: String },
    /// Register a settings page.
    RegisterSettingsPage { page: SettingsPage },
    /// Add a sidebar item.
    AddSidebarItem { item: SidebarItem },
    /// Remove a sidebar item.
    RemoveSidebarItem { item_id: String },
}

/// Menu item definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    /// Item ID.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Keyboard shortcut.
    pub shortcut: Option<String>,
    /// Icon name.
    pub icon: Option<String>,
    /// Action to perform when clicked.
    pub action: String,
    /// Whether the item is enabled.
    pub enabled: bool,
    /// Whether the item is checked (for checkbox items).
    pub checked: Option<bool>,
}

/// Dialog configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogConfig {
    /// Dialog ID.
    pub id: String,
    /// Dialog title.
    pub title: String,
    /// Dialog content (HTML or markdown).
    pub content: String,
    /// Dialog buttons.
    pub buttons: Vec<DialogButton>,
    /// Dialog width.
    pub width: Option<u32>,
    /// Dialog height.
    pub height: Option<u32>,
    /// Whether the dialog is modal.
    pub modal: bool,
}

/// Dialog button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogButton {
    /// Button ID.
    pub id: String,
    /// Button label.
    pub label: String,
    /// Button style (default, primary, danger).
    pub style: DialogButtonStyle,
}

/// Dialog button style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogButtonStyle {
    Default,
    Primary,
    Danger,
}

/// Settings page definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPage {
    /// Page ID.
    pub id: String,
    /// Page title.
    pub title: String,
    /// Page icon.
    pub icon: Option<String>,
    /// Settings sections.
    pub sections: Vec<SettingsSection>,
}

/// Settings section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSection {
    /// Section ID.
    pub id: String,
    /// Section title.
    pub title: String,
    /// Settings items.
    pub items: Vec<SettingsItem>,
}

/// Settings item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsItem {
    /// Item ID.
    pub id: String,
    /// Item label.
    pub label: String,
    /// Item description.
    pub description: Option<String>,
    /// Item type.
    pub item_type: SettingsItemType,
    /// Current value.
    pub value: serde_json::Value,
    /// Default value.
    pub default: serde_json::Value,
}

/// Settings item type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingsItemType {
    Text,
    Number,
    Boolean,
    Select { options: Vec<SelectOption> },
    Color,
    Path,
}

/// Select option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    /// Option value.
    pub value: String,
    /// Option label.
    pub label: String,
}

/// Sidebar item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarItem {
    /// Item ID.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Icon name.
    pub icon: Option<String>,
    /// Item order.
    pub order: u32,
    /// Whether the item is expandable.
    pub expandable: bool,
    /// Child items.
    pub children: Vec<SidebarItem>,
}

impl UiApi {
    /// Create a new UI API.
    pub fn new() -> Self {
        UiApi { command_tx: None }
    }

    /// Create with command channel.
    pub fn with_channel(command_tx: mpsc::Sender<UiCommand>) -> Self {
        UiApi { command_tx: Some(command_tx) }
    }

    /// Show a panel.
    pub async fn show_panel(&self, panel_id: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::ShowPanel { panel_id: panel_id.to_string() }).await;
        }
    }

    /// Hide a panel.
    pub async fn hide_panel(&self, panel_id: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::HidePanel { panel_id: panel_id.to_string() }).await;
        }
    }

    /// Add a menu item.
    pub async fn add_menu_item(&self, menu: &str, item: MenuItem) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::AddMenuItem { menu: menu.to_string(), item }).await;
        }
    }

    /// Remove a menu item.
    pub async fn remove_menu_item(&self, menu: &str, item_id: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::RemoveMenuItem {
                menu: menu.to_string(),
                item_id: item_id.to_string(),
            }).await;
        }
    }

    /// Show a dialog.
    pub async fn show_dialog(&self, dialog: DialogConfig) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::ShowDialog { dialog }).await;
        }
    }

    /// Close a dialog.
    pub async fn close_dialog(&self, dialog_id: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::CloseDialog { dialog_id: dialog_id.to_string() }).await;
        }
    }

    /// Update the status bar.
    pub async fn update_status_bar(&self, section: &str, content: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::UpdateStatusBar {
                section: section.to_string(),
                content: content.to_string(),
            }).await;
        }
    }

    /// Register a settings page.
    pub async fn register_settings_page(&self, page: SettingsPage) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::RegisterSettingsPage { page }).await;
        }
    }

    /// Add a sidebar item.
    pub async fn add_sidebar_item(&self, item: SidebarItem) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::AddSidebarItem { item }).await;
        }
    }

    /// Remove a sidebar item.
    pub async fn remove_sidebar_item(&self, item_id: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(UiCommand::RemoveSidebarItem { item_id: item_id.to_string() }).await;
        }
    }
}

impl Default for UiApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin API trait for custom plugin interfaces.
#[async_trait::async_trait]
pub trait PluginApi: Send + Sync {
    /// Get the API name.
    fn name(&self) -> &str;

    /// Get the API version.
    fn version(&self) -> &str;

    /// Handle a request.
    async fn handle_request(&self, method: &str, params: serde_json::Value) -> PluginResult<serde_json::Value>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_api() {
        let api = HostApi::new("1.0.0".to_string());
        assert_eq!(api.version(), "1.0.0");
    }

    #[test]
    fn test_media_api() {
        let api = MediaApi::new();
        // Basic creation test
        assert!(api.command_tx.is_none());
    }

    #[test]
    fn test_ui_api() {
        let api = UiApi::new();
        assert!(api.command_tx.is_none());
    }

    #[test]
    fn test_menu_item() {
        let item = MenuItem {
            id: "test".into(),
            label: "Test Item".into(),
            shortcut: Some("Ctrl+T".into()),
            icon: None,
            action: "test_action".into(),
            enabled: true,
            checked: None,
        };

        assert_eq!(item.id, "test");
        assert!(item.enabled);
    }
}