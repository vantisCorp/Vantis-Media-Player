// Example: Custom UI components in Vantis Media Player
//
// This example demonstrates how to create custom UI components
// using the Liquid Glass framework.

use vantis_ui::{Window, Widget, Button, Slider, ProgressBar, Label, Container};
use vantis_core::Player;
use iced::{Application, Command, Element, Settings, Subscription, executor};

#[derive(Debug, Clone)]
enum Message {
    PlayPause,
    Stop,
    Seek(f32),
    VolumeChanged(f32),
    Fullscreen,
    SubtitlesToggle,
    Tick,
}

struct CustomUI {
    player: Player,
    is_playing: bool,
    volume: f32,
    position: f32,
    duration: f32,
    show_subtitles: bool,
}

impl CustomUI {
    fn new() -> Self {
        let player = Player::new();
        player.load("examples/sample_video.mp4").unwrap();
        
        Self {
            player,
            is_playing: false,
            volume: 0.8,
            position: 0.0,
            duration: 0.0,
            show_subtitles: true,
        }
    }
}

impl Application for CustomUI {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Vantis Media Player - Custom UI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayPause => {
                if self.is_playing {
                    self.player.pause();
                    self.is_playing = false;
                } else {
                    self.player.play();
                    self.is_playing = true;
                }
            }
            
            Message::Stop => {
                self.player.stop();
                self.is_playing = false;
                self.position = 0.0;
            }
            
            Message::Seek(value) => {
                let seek_time = (value as f64 / 100.0) * self.duration;
                self.player.seek(seek_time);
                self.position = seek_time;
            }
            
            Message::VolumeChanged(value) => {
                self.volume = value;
                self.player.set_volume(value as f64);
            }
            
            Message::Fullscreen => {
                self.player.toggle_fullscreen();
            }
            
            Message::SubtitlesToggle => {
                self.show_subtitles = !self.show_subtitles;
                if self.show_subtitles {
                    self.player.enable_subtitles();
                } else {
                    self.player.disable_subtitles();
                }
            }
            
            Message::Tick => {
                if self.is_playing {
                    self.position = self.player.current_position();
                    self.duration = self.player.duration();
                }
            }
        }
        
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Main container
        Container::new(
            iced::Column::new()
                .spacing(20)
                .padding(20)
                // Title
                .push(
                    Label::new("Vantis Media Player")
                        .size(32)
                        .style(iced::theme::Text::Color(iced::Color::WHITE))
                )
                // Video area (placeholder)
                .push(
                    Container::new(
                        Label::new("Video Display Area")
                            .size(24)
                            .style(iced::theme::Text::Color(iced::Color::WHITE))
                    )
                    .width(iced::Length::Fill)
                    .height(iced::Length::Units(400))
                    .style(iced::theme::Container::Box {
                        background: Some(iced::Color::BLACK.into()),
                        border_radius: 8.0,
                        border_width: 2.0,
                        border_color: iced::Color::from_rgb(100, 100, 100),
                    })
                )
                // Progress bar
                .push(
                    ProgressBar::new(0.0..=100.0, (self.position / self.duration * 100.0) as f32)
                        .width(iced::Length::Fill)
                )
                // Seek slider
                .push(
                    Slider::new(0.0..=100.0, (self.position / self.duration * 100.0) as f32, Message::Seek)
                        .width(iced::Length::Fill)
                )
                // Time display
                .push(
                    Container::new(
                        Label::new(format!(
                            "{:.2} / {:.2}",
                            self.position,
                            self.duration
                        ))
                        .style(iced::theme::Text::Color(iced::Color::WHITE))
                    )
                    .width(iced::Length::Fill)
                )
                // Control buttons
                .push(
                    iced::Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                Label::new(if self.is_playing { "⏸ Pause" } else { "▶ Play" })
                                    .style(iced::theme::Text::Color(iced::Color::WHITE))
                            )
                            .on_press(Message::PlayPause)
                            .style(iced::theme::Button::Primary)
                        )
                        .push(
                            Button::new(
                                Label::new("⏹ Stop")
                                    .style(iced::theme::Text::Color(iced::Color::WHITE))
                            )
                            .on_press(Message::Stop)
                            .style(iced::theme::Button::Destructive)
                        )
                        .push(
                            Button::new(
                                Label::new("⛶ Fullscreen")
                                    .style(iced::theme::Text::Color(iced::Color::WHITE))
                            )
                            .on_press(Message::Fullscreen)
                            .style(iced::theme::Button::Secondary)
                        )
                        .push(
                            Button::new(
                                Label::new(if self.show_subtitles { "📝 Subs On" } else { "📝 Subs Off" })
                                    .style(iced::theme::Text::Color(iced::Color::WHITE))
                            )
                            .on_press(Message::SubtitlesToggle)
                            .style(iced::theme::Button::Secondary)
                        )
                )
                // Volume control
                .push(
                    iced::Row::new()
                        .spacing(10)
                        .push(
                            Label::new("Volume:")
                                .style(iced::theme::Text::Color(iced::Color::WHITE))
                        )
                        .push(
                            Slider::new(0.0..=1.0, self.volume, Message::VolumeChanged)
                                .width(iced::Length::Units(200))
                        )
                        .push(
                            Label::new(format!("{:.0}%", self.volume * 100.0))
                                .style(iced::theme::Text::Color(iced::Color::WHITE))
                        )
                )
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(iced::theme::Container::Box {
            background: Some(iced::Color::from_rgb(30, 30, 30).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
        })
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(100))
            .map(|_| Message::Tick)
    }
}

fn main() {
    println!("Vantis Media Player - Custom UI Example");
    println!("=========================================\n");
    
    CustomUI::run(Settings {
        window: iced::window::Settings {
            size: (1280, 720),
            ..Default::default()
        },
        ..Default::default()
    });
}