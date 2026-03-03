//! Navigation System
//!
//! Provides navigation between different UI views.

use iced::widget::{button, container, row, text, Space};
use iced::{Element, Length};

/// Navigation view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavView {
    /// Library view
    Library,
    
    /// Marketplace view
    Marketplace,
    
    /// Settings view
    Settings,
    
    /// Shortcuts view
    Shortcuts,
}

/// Navigation state
pub struct NavigationState {
    /// Current view
    current_view: NavView,
}

impl NavigationState {
    /// Create a new navigation state
    pub fn new() -> Self {
        Self {
            current_view: NavView::Library,
        }
    }
    
    /// Set current view
    pub fn set_view(&mut self, view: NavView) {
        self.current_view = view;
    }
    
    /// Get current view
    pub fn current_view(&self) -> NavView {
        self.current_view
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation message
#[derive(Debug, Clone)]
pub enum Message {
    /// Navigate to view
    NavigateTo(NavView),
}

/// View navigation bar
pub fn view_navigation(state: &NavigationState) -> Element<Message> {
    container(
        row![
            button("Library")
                .on_press(Message::NavigateTo(NavView::Library)),
            Space::with_width(Length::Fixed(10.0)),
            button("Marketplace")
                .on_press(Message::NavigateTo(NavView::Marketplace)),
            Space::with_width(Length::Fixed(10.0)),
            button("Settings")
                .on_press(Message::NavigateTo(NavView::Settings)),
            Space::with_width(Length::Fixed(10.0)),
            button("Shortcuts")
                .on_press(Message::NavigateTo(NavView::Shortcuts)),
        ]
        .spacing(10)
        .padding(10),
    )
    .width(Length::Fill)
    .into()
}