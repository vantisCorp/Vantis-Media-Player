//! Plugin Marketplace UI
//!
//! User interface for discovering, installing, and managing plugins
//! from the plugin marketplace.

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Padding, Renderer, Theme,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::theme::Theme;

/// Plugin marketplace state
pub struct MarketplaceState {
    /// Search query
    search_query: String,
    
    /// Selected category
    selected_category: Option<String>,
    
    /// Selected sort option
    sort_option: SortOption,
    
    /// Plugin list
    plugins: Vec<PluginDisplayInfo>,
    
    /// Selected plugin
    selected_plugin: Option<PluginDisplayInfo>,
    
    /// Loading state
    loading: bool,
    
    /// Error message
    error: Option<String>,
    
    /// Installation progress
    installation_progress: Option<(String, f32)>,
}

/// Plugin display information
#[derive(Clone, Debug)]
pub struct PluginDisplayInfo {
    /// Plugin ID
    pub id: String,
    
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin author
    pub author: String,
    
    /// Plugin description
    pub description: String,
    
    /// Plugin category
    pub category: String,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Download count
    pub downloads: u64,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// Installed flag
    pub installed: bool,
    
    /// Update available flag
    pub update_available: bool,
    
    /// Enabled flag
    pub enabled: bool,
    
    /// Screenshots
    pub screenshots: Vec<String>,
    
    /// Tags
    pub tags: Vec<String>,
}

/// Sort options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortOption {
    /// Sort by popularity
    Popularity,
    
    /// Sort by rating
    Rating,
    
    /// Sort by downloads
    Downloads,
    
    /// Sort by name
    Name,
    
    /// Sort by date
    Date,
}

/// Marketplace message
#[derive(Clone, Debug)]
pub enum Message {
    /// Search query changed
    SearchQueryChanged(String),
    
    /// Category selected
    CategorySelected(Option<String>),
    
    /// Sort option changed
    SortOptionChanged(SortOption),
    
    /// Plugin selected
    PluginSelected(PluginDisplayInfo),
    
    /// Install plugin
    InstallPlugin(String),
    
    /// Uninstall plugin
    UninstallPlugin(String),
    
    /// Update plugin
    UpdatePlugin(String),
    
    /// Enable plugin
    EnablePlugin(String),
    
    /// Disable plugin
    DisablePlugin(String),
    
    /// Refresh plugin list
    Refresh,
    
    /// Close plugin details
    CloseDetails,
}

impl MarketplaceState {
    /// Create a new marketplace state
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            selected_category: None,
            sort_option: SortOption::Popularity,
            plugins: Vec::new(),
            selected_plugin: None,
            loading: false,
            error: None,
            installation_progress: None,
        }
    }
    
    /// Update marketplace state
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                self.filter_plugins();
            }
            
            Message::CategorySelected(category) => {
                self.selected_category = category;
                self.filter_plugins();
            }
            
            Message::SortOptionChanged(sort_option) => {
                self.sort_option = sort_option;
                self.sort_plugins();
            }
            
            Message::PluginSelected(plugin) => {
                self.selected_plugin = Some(plugin);
            }
            
            Message::InstallPlugin(id) => {
                self.install_plugin(id);
            }
            
            Message::UninstallPlugin(id) => {
                self.uninstall_plugin(id);
            }
            
            Message::UpdatePlugin(id) => {
                self.update_plugin(id);
            }
            
            Message::EnablePlugin(id) => {
                self.enable_plugin(id);
            }
            
            Message::DisablePlugin(id) => {
                self.disable_plugin(id);
            }
            
            Message::Refresh => {
                self.refresh_plugins();
            }
            
            Message::CloseDetails => {
                self.selected_plugin = None;
            }
        }
    }
    
    /// Filter plugins based on search query and category
    fn filter_plugins(&mut self) {
        self.sort_plugins();
    }
    
    /// Sort plugins based on selected sort option
    fn sort_plugins(&mut self) {
        match self.sort_option {
            SortOption::Popularity => {
                self.plugins.sort_by(|a, b| b.downloads.cmp(&a.downloads));
            }
            SortOption::Rating => {
                self.plugins.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
            }
            SortOption::Downloads => {
                self.plugins.sort_by(|a, b| b.downloads.cmp(&a.downloads));
            }
            SortOption::Name => {
                self.plugins.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortOption::Date => {
                self.plugins.sort_by(|a, b| a.name.cmp(&b.name));
            }
        }
    }
    
    /// Install a plugin
    fn install_plugin(&mut self, id: String) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.installed = true;
            plugin.enabled = true;
        }
    }
    
    /// Uninstall a plugin
    fn uninstall_plugin(&mut self, id: String) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.installed = false;
            plugin.enabled = false;
        }
    }
    
    /// Update a plugin
    fn update_plugin(&mut self, id: String) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.update_available = false;
        }
    }
    
    /// Enable a plugin
    fn enable_plugin(&mut self, id: String) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = true;
        }
    }
    
    /// Disable a plugin
    fn disable_plugin(&mut self, id: String) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = false;
        }
    }
    
    /// Refresh plugin list
    fn refresh_plugins(&mut self) {
        self.plugins = vec![
            PluginDisplayInfo {
                id: "audio-enhancer".to_string(),
                name: "Audio Enhancer".to_string(),
                version: "1.2.0".to_string(),
                author: "Vantis Team".to_string(),
                description: "Enhance audio quality with advanced processing".to_string(),
                category: "Audio".to_string(),
                rating: 4.5,
                downloads: 12500,
                file_size: 1024 * 1024,
                installed: false,
                update_available: false,
                enabled: false,
                screenshots: vec![],
                tags: vec!["audio".to_string(), "enhancement".to_string()],
            },
            PluginDisplayInfo {
                id: "video-upscaler".to_string(),
                name: "Video Upscaler".to_string(),
                version: "2.0.1".to_string(),
                author: "Vantis Team".to_string(),
                description: "AI-powered video upscaling to 4K".to_string(),
                category: "Video".to_string(),
                rating: 4.8,
                downloads: 25000,
                file_size: 5 * 1024 * 1024,
                installed: true,
                update_available: true,
                enabled: true,
                screenshots: vec![],
                tags: vec!["video".to_string(), "ai".to_string(), "upscaling".to_string()],
            },
            PluginDisplayInfo {
                id: "subtitle-sync".to_string(),
                name: "Subtitle Sync".to_string(),
                version: "1.0.5".to_string(),
                author: "Community".to_string(),
                description: "Automatically synchronize subtitles with audio".to_string(),
                category: "Subtitles".to_string(),
                rating: 4.2,
                downloads: 8700,
                file_size: 512 * 1024,
                installed: false,
                update_available: false,
                enabled: false,
                screenshots: vec![],
                tags: vec!["subtitles".to_string(), "sync".to_string()],
            },
        ];
        
        self.filter_plugins();
    }
}

impl Default for MarketplaceState {
    fn default() -> Self {
        Self::new()
    }
}

/// View the marketplace
pub fn view_marketplace(state: &MarketplaceState) -> Element<Message> {
    let content = column![
        // Header
        view_header(state),
        Space::with_height(Length::Fixed(20.0)),
        
        // Search and filters
        view_search_and_filters(state),
        Space::with_height(Length::Fixed(20.0)),
        
        // Plugin list or details
        if let Some(ref plugin) = state.selected_plugin {
            view_plugin_details(plugin)
        } else {
            view_plugin_list(state)
        },
    ]
    .padding(20)
    .spacing(10);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// View the header
fn view_header(state: &MarketplaceState) -> Element<Message> {
    row![
        text("Plugin Marketplace")
            .size(32)
            .width(Length::Fill),
        button("Refresh")
            .on_press(Message::Refresh),
    ]
    .spacing(10)
    .into()
}

/// View search and filters
fn view_search_and_filters(state: &MarketplaceState) -> Element<Message> {
    row![
        text_input("Search plugins...", &state.search_query)
            .on_input(Message::SearchQueryChanged)
            .width(Length::Fill),
        Space::with_width(Length::Fixed(10.0)),
        button("All")
            .on_press(Message::CategorySelected(None)),
        button("Audio")
            .on_press(Message::CategorySelected(Some("Audio".to_string()))),
        button("Video")
            .on_press(Message::CategorySelected(Some("Video".to_string()))),
        button("Subtitles")
            .on_press(Message::CategorySelected(Some("Subtitles".to_string()))),
    ]
    .spacing(10)
    .into()
}

/// View plugin list
fn view_plugin_list(state: &MarketplaceState) -> Element<Message> {
    let plugins: Vec<Element<Message>> = state
        .plugins
        .iter()
        .map(|plugin| view_plugin_card(plugin))
        .collect();
    
    scrollable(
        column(plugins)
            .spacing(10)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// View plugin card
fn view_plugin_card(plugin: &PluginDisplayInfo) -> Element<Message> {
    let plugin_clone = plugin.clone();
    
    let status_badge = if plugin.installed {
        if plugin.update_available {
            button("Update Available")
                .on_press(Message::UpdatePlugin(plugin.id.clone()))
        } else {
            button(if plugin.enabled { "Enabled" } else { "Disabled" })
                .on_press(if plugin.enabled {
                    Message::DisablePlugin(plugin.id.clone())
                } else {
                    Message::EnablePlugin(plugin.id.clone())
                })
        }
    } else {
        button("Install")
            .on_press(Message::InstallPlugin(plugin.id.clone()))
    };
    
    container(
        row![
            // Plugin info
            column![
                text(&plugin.name)
                    .size(20)
                    .width(Length::Fill),
                text(&plugin.author)
                    .size(14)
                    .width(Length::Fill),
                text(&plugin.description)
                    .size(14)
                    .width(Length::Fill),
                row![
                    text(format!("⭐ {:.1}", plugin.rating)),
                    Space::with_width(Length::Fixed(10.0)),
                    text(format!("📥 {}", plugin.downloads)),
                    Space::with_width(Length::Fixed(10.0)),
                    text(format!("📦 {} MB", plugin.file_size / (1024 * 1024))),
                ]
                .spacing(10),
            ]
            .spacing(5)
            .width(Length::Fill),
            
            // Actions
            column![
                status_badge,
                button("Details")
                    .on_press(Message::PluginSelected(plugin_clone)),
            ]
            .spacing(10),
        ]
        .spacing(20)
        .padding(15),
    )
    .width(Length::Fill)
    .padding(10)
    .into()
}

/// View plugin details
fn view_plugin_details(plugin: &PluginDisplayInfo) -> Element<Message> {
    let plugin_clone = plugin.clone();
    
    let action_button = if plugin.installed {
        if plugin.update_available {
            button("Update")
                .on_press(Message::UpdatePlugin(plugin.id.clone()))
        } else {
            button(if plugin.enabled { "Disable" } else { "Enable" })
                .on_press(if plugin.enabled {
                    Message::DisablePlugin(plugin.id.clone())
                } else {
                    Message::EnablePlugin(plugin.id.clone())
                })
        }
    } else {
        button("Install")
            .on_press(Message::InstallPlugin(plugin.id.clone()))
    };
    
    scrollable(
        column![
            // Back button
            button("← Back")
                .on_press(Message::CloseDetails),
            Space::with_height(Length::Fixed(20.0)),
            
            // Plugin name and version
            text(&plugin.name)
                .size(32)
                .width(Length::Fill),
            text(format!("v{}", plugin.version))
                .size(18)
                .width(Length::Fill),
            Space::with_height(Length::Fixed(20.0)),
            
            // Author and category
            row![
                text(format!("By {}", plugin.author)),
                Space::with_width(Length::Fixed(20.0)),
                text(&plugin.category),
            ]
            .spacing(10),
            Space::with_height(Length::Fixed(20.0)),
            
            // Description
            text("Description")
                .size(20)
                .width(Length::Fill),
            text(&plugin.description)
                .width(Length::Fill),
            Space::with_height(Length::Fixed(20.0)),
            
            // Stats
            row![
                text(format!("⭐ Rating: {:.1}", plugin.rating)),
                Space::with_width(Length::Fixed(20.0)),
                text(format!("📥 Downloads: {}", plugin.downloads)),
                Space::with_width(Length::Fixed(20.0)),
                text(format!("📦 Size: {} MB", plugin.file_size / (1024 * 1024))),
            ]
            .spacing(10),
            Space::with_height(Length::Fixed(20.0)),
            
            // Tags
            if !plugin.tags.is_empty() {
                column![
                    text("Tags")
                        .size(20)
                        .width(Length::Fill),
                    row(plugin.tags.iter().map(|tag| {
                        text(tag)
                            .size(14)
                            .padding(5)
                    }).collect::<Vec<_>>())
                    .spacing(10),
                ]
                .spacing(10)
                .into()
            } else {
                Space::with_height(Length::Fixed(0.0)).into()
            },
            Space::with_height(Length::Fixed(20.0)),
            
            // Actions
            row![
                action_button,
                if plugin.installed {
                    button("Uninstall")
                        .on_press(Message::UninstallPlugin(plugin.id.clone()))
                } else {
                    Space::with_width(Length::Fixed(0.0)).into()
                },
            ]
            .spacing(10),
        ]
        .spacing(10)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}