//! Media Library Browser
//! 
/// Browse and manage your media collection.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    /// Select media
    SelectMedia(String),
    
    /// Search library
    Search(String),
    
    /// Change view mode
    ChangeViewMode(ViewMode),
    
    /// Filter by type
    FilterBy(MediaType),
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Grid,
    List,
    Timeline,
}

/// Media type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Video,
    Audio,
    Image,
    All,
}

/// Media item
#[derive(Debug, Clone)]
pub struct MediaItem {
    /// Item ID
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// File path
    pub path: String,
    
    /// Media type
    pub media_type: MediaType,
    
    /// Duration (in seconds)
    pub duration: Option<f64>,
    
    /// Thumbnail
    pub thumbnail: Option<String>,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Library state
pub struct LibraryState {
    /// Media items
    items: Vec<MediaItem>,
    
    /// Selected item
    selected: Option<String>,
    
    /// Current view mode
    view_mode: ViewMode,
    
    /// Current filter
    filter: MediaType,
    
    /// Search query
    search_query: String,
}

impl LibraryState {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            view_mode: ViewMode::Grid,
            filter: MediaType::All,
            search_query: String::new(),
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectMedia(id) => {
                self.selected = Some(id);
                tracing::info!("📂 Selected media: {}", id);
            }
            Message::Search(query) => {
                self.search_query = query;
                tracing::debug!("🔍 Search: {}", query);
            }
            Message::ChangeViewMode(mode) => {
                self.view_mode = mode;
                tracing::info!("👁️ View mode: {:?}", mode);
            }
            Message::FilterBy(media_type) => {
                self.filter = media_type;
                tracing::info!("🔍 Filter: {:?}", media_type);
            }
        }
    }
    
    /// Get filtered items
    pub fn get_filtered_items(&self) -> Vec<&MediaItem> {
        self.items
            .iter()
            .filter(|item| {
                // Apply media type filter
                if self.filter != MediaType::All && item.media_type != self.filter {
                    return false;
                }
                
                // Apply search query
                if !self.search_query.is_empty() {
                    let query_lower = self.search_query.to_lowercase();
                    if !item.title.to_lowercase().contains(&query_lower) {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Add media item
    pub fn add_item(&mut self, item: MediaItem) {
        self.items.push(item);
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        Self::new()
    }
}
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Element, Length};

/// View the library
pub fn view_library(state: &LibraryState) -> Element<Message> {
    let content = column![
        // Header
        row![
            text("Media Library")
                .size(32)
                .width(Length::Fill),
            button("Refresh"),
        ]
        .spacing(10),
        Space::with_height(Length::Fixed(20.0)),
        
        // Search and filters
        row![
            text_input("Search...", &state.search_query)
                .on_input(Message::Search)
                .width(Length::Fill),
            Space::with_width(Length::Fixed(10.0)),
            button("All")
                .on_press(Message::FilterBy(MediaType::All)),
            button("Video")
                .on_press(Message::FilterBy(MediaType::Video)),
            button("Audio")
                .on_press(Message::FilterBy(MediaType::Audio)),
        ]
        .spacing(10),
        Space::with_height(Length::Fixed(20.0)),
        
        // Media items
        text(format!("{} items", state.get_filtered_items().len()))
            .size(18),
    ]
    .padding(20)
    .spacing(10);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
