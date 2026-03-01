//! Vantis UI - Liquid Glass Interface
//! 
//! Modern, frameless UI with GPU acceleration,
/// Omnibar command system, and smart controls.

use anyhow::Result;
use iced::{Application, Settings, Command, Element, Subscription};
use tracing::info;

pub mod omnibar;
pub mod controls;
pub mod library;
pub mod theme;

/// Vantis UI Application
pub struct VantisUI {
    /// Omnibar state
    omnibar: omnibar::OmnibarState,
    
    /// Controls state
    controls: controls::ControlsState,
    
    /// Library state
    library: library::LibraryState,
    
    /// Theme
    theme: theme::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// Omnibar message
    Omnibar(omnibar::Message),
    
    /// Controls message
    Controls(controls::Message),
    
    /// Library message
    Library(library::Message),
    
    /// Theme changed
    ThemeChanged(theme::Theme),
}

impl VantisUI {
    /// Create a new UI engine
    pub fn new() -> Result<Self> {
        info!("🎨 Initializing Liquid Glass UI");
        
        Ok(Self {
            omnibar: omnibar::OmnibarState::new(),
            controls: controls::ControlsState::new(),
            library: library::LibraryState::new(),
            theme: theme::Theme::Dark,
        })
    }
}

impl Application for VantisUI {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        info!("🎨 Initializing Liquid Glass UI");
        
        let ui = Self {
            omnibar: omnibar::OmnibarState::new(),
            controls: controls::ControlsState::new(),
            library: library::LibraryState::new(),
            theme: theme::Theme::Dark,
        };
        
        (ui, Command::none())
    }
    
    fn title(&self) -> String {
        "Vantis Player".to_string()
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Omnibar(msg) => {
                self.omnibar.update(msg);
            }
            Message::Controls(msg) => {
                self.controls.update(msg);
            }
            Message::Library(msg) => {
                self.library.update(msg);
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
        Command::none()
    }
    
    fn view(&self) -> Element<Self::Message> {
        iced::container(iced::text("Vantis Player UI"))
            .into()
    }
    
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
    
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

/// Run the Vantis UI
pub fn run_ui() -> Result<()> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (1920, 1080),
            decorations: false, // Frameless
            transparent: true,
            ..Default::default()
        },
        default_font: Some(include_bytes!("../assets/fonts/Inter-Regular.ttf")),
        ..Default::default()
    };
    
    VantisUI::run(settings)?;
    
    Ok(())
}