//! Easter Eggs Module
//! 
//! Hidden features and fun surprises for users to discover.
//! Because media players should be delightful!

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Easter egg types
#[derive(Debug, Clone)]
pub enum EasterEgg {
    /// Konami code sequence
    KonamiCode { reward: KonamiReward },
    /// Hidden message in banner
    Steganography { message: String },
    /// ASCII art reveal
    AsciiArt { art: String },
    /// Secret game
    MiniGame { game_type: MiniGameType },
    /// Recruitment puzzle
    RecruitmentPuzzle { puzzle: Puzzle },
    /// Special date trigger
    DateTrigger { date: String, event: SpecialEvent },
}

#[derive(Debug, Clone)]
pub enum KonamiReward {
    /// Enable retro theme
    RetroTheme,
    /// Show credits
    SecretCredits,
    /// Unlock special visualization
    SpecialVisualization,
    /// Play secret sound
    SecretSound,
    /// Rainbow mode
    RainbowMode,
}

#[derive(Debug, Clone)]
pub enum MiniGameType {
    /// Snake game
    Snake,
    /// Pong
    Pong,
    /// Breakout
    Breakout,
    /// Memory game
    Memory,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub id: String,
    pub difficulty: u8,
    pub hint: String,
    pub solution: String,
    pub reward: String,
}

#[derive(Debug, Clone)]
pub enum SpecialEvent {
    /// Birthday celebration
    Birthday { year: u32 },
    /// Holiday special
    Holiday { name: String },
    /// Milestone celebration
    Milestone { downloads: u64 },
    /// April Fools
    AprilFools,
}

/// Easter egg manager
pub struct EasterEggManager {
    discovered: HashMap<String, Instant>,
    konami_sequence: Vec<KonamiKey>,
    konami_position: usize,
    total_discovered: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KonamiKey {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
}

impl EasterEggManager {
    const KONAMI_CODE: [KonamiKey; 10] = [
        KonamiKey::Up,
        KonamiKey::Up,
        KonamiKey::Down,
        KonamiKey::Down,
        KonamiKey::Left,
        KonamiKey::Right,
        KonamiKey::Left,
        KonamiKey::Right,
        KonamiKey::B,
        KonamiKey::A,
    ];

    pub fn new() -> Self {
        Self {
            discovered: HashMap::new(),
            konami_sequence: Vec::new(),
            konami_position: 0,
            total_discovered: 0,
        }
    }

    /// Process a key press for Konami code detection
    pub fn process_key(&mut self, key: KonamiKey) -> Option<KonamiReward> {
        if key == Self::KONAMI_CODE[self.konami_position] {
            self.konami_position += 1;
            
            if self.konami_position == Self::KONAMI_CODE.len() {
                self.konami_position = 0;
                self.discover("konami_code");
                return Some(KonamiReward::RainbowMode);
            }
        } else {
            self.konami_position = 0;
        }
        
        None
    }

    /// Mark an easter egg as discovered
    pub fn discover(&mut self, id: &str) {
        if !self.discovered.contains_key(id) {
            self.discovered.insert(id.to_string(), Instant::now());
            self.total_discovered += 1;
        }
    }

    /// Check if an easter egg has been discovered
    pub fn is_discovered(&self, id: &str) -> bool {
        self.discovered.contains_key(id)
    }

    /// Get total discovered count
    pub fn total_discovered(&self) -> u32 {
        self.total_discovered
    }

    /// Get discovery progress (out of known easter eggs)
    pub fn progress(&self) -> f32 {
        const TOTAL_EGGS: u32 = 10;
        self.total_discovered as f32 / TOTAL_EGGS as f32
    }
}

/// Hidden ASCII arts
pub mod ascii_art {
    pub const VANTIS_LOGO: &str = r#"
     ██╗   ██╗███████╗████████╗██╗   ██╗███╗   ██╗
     ██║   ██║██╔════╝╚══██╔══╝██║   ██║████╗  ██║
     ██║   ██║█████╗     ██║   ██║   ██║██╔██╗ ██║
     ╚██╗ ██╔╝██╔══╝     ██║   ██║   ██║██║╚██╗██║
      ╚████╔╝ ███████╗   ██║   ╚██████╔╝██║ ╚████║
       ╚═══╝  ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝
    "#;

    pub const RETRO_PLAYER: &str = r#"
    ┌─────────────────────────────────────┐
    │  ▶  ──────────────────────  ●       │
    │     ████████████░░░░░░░░░░░         │
    │  00:42 / 03:14                      │
    │  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  │
    │  ████████ ♫ ♪ ♫ ♪ ♫ ♪ ♫ ♪          │
    └─────────────────────────────────────┘
    "#;

    pub const SPACE_INVADER: &str = r#"
        ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
       █▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█
       █   ▄▄▄    ▄▄▄    ▄▄▄   █
       █  █   █  █   █  █   █  █
       █  █▀▀▀▀  █▀▀▀▀  █▀▀▀▀  █
       █  █      █      █      █
       █   ▀▄▄▄▄▄▀    ▀▄▄▄▄▄▀   █
       █▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█
    "#;

    pub const CAT: &str = r#"
      /\_/\
     ( o.o )
      > ^ <
    "#;

    pub const SHRUG: &str = r#"
       ¯\_(ツ)_/¯
    "#;
}

/// Hidden messages
pub mod hidden_messages {
    pub const WELCOME_MESSAGE: &str = "You found the secret! 🎉";
    pub const CREDITS: &str = r#"
    ╔════════════════════════════════════════╗
    ║         VANTIS MEDIA PLAYER            ║
    ║                                        ║
    ║   Created with ❤️ by Vantis Team       ║
    ║                                        ║
    ║   Special thanks to:                   ║
    ║   - Our amazing contributors           ║
    ║   - The Rust community                 ║
    ║   - Open source everywhere             ║
    ║                                        ║
    ║   "The best media player in the       ║
    ║    universe (we think)"                ║
    ╚════════════════════════════════════════╝
    "#;

    pub const JOKES: [&str; 5] = [
        "Why did the video go to therapy? It had too many issues.",
        "What's a codec's favorite food? Compressed sandwiches.",
        "Why do media players hate parties? Too much buffering.",
        "What did the subtitle say to the video? 'I'm always below you.'",
        "Why was the audio file sad? It felt compressed."
    ];
}

/// Recruitment puzzles
pub mod puzzles {
    use super::Puzzle;

    pub fn get_puzzles() -> Vec<Puzzle> {
        vec![
            Puzzle {
                id: "binary_search".to_string(),
                difficulty: 1,
                hint: "01001000 01101001 01101110 01110100".to_string(),
                solution: "Hint".to_string(),
                reward: "You found the hint! Apply at careers@vantis.dev".to_string(),
            },
            Puzzle {
                id: "hex_secret".to_string(),
                difficulty: 2,
                hint: "56 61 6e 74 69 73 5f 52 6f 63 6b 73".to_string(),
                solution: "Vantis_Rocks".to_string(),
                reward: "Impressive! We're hiring developers like you!".to_string(),
            },
            Puzzle {
                id: "base64_challenge".to_string(),
                difficulty: 3,
                hint: "V2UgbG92ZSBjdXJpb3VzIG1pbmRzIQ==".to_string(),
                solution: "We love curious minds!".to_string(),
                reward: "You're exactly who we're looking for! careers@vantis.dev".to_string(),
            },
        ]
    }
}

/// Date-based triggers
pub mod date_triggers {
    use chrono::{DateTime, Utc, Datelike};

    pub fn check_special_date() -> Option<super::SpecialEvent> {
        let now = Utc::now();
        
        // April 1st - April Fools
        if now.month() == 4 && now.day() == 1 {
            return Some(super::SpecialEvent::AprilFools);
        }
        
        // December 25 - Holiday
        if now.month() == 12 && now.day() == 25 {
            return Some(super::SpecialEvent::Holiday {
                name: "Winter Holiday".to_string(),
            });
        }
        
        // January 1 - Birthday (example)
        if now.month() == 1 && now.day() == 1 {
            return Some(super::SpecialEvent::Birthday {
                year: now.year() as u32,
            });
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_konami_code() {
        let mut manager = EasterEggManager::new();
        
        for key in EasterEggManager::KONAMI_CODE {
            let result = manager.process_key(key);
            if key == KonamiKey::A {
                assert!(result.is_some());
            } else {
                assert!(result.is_none());
            }
        }
        
        assert!(manager.is_discovered("konami_code"));
    }

    #[test]
    fn test_discovery_progress() {
        let mut manager = EasterEggManager::new();
        assert_eq!(manager.progress(), 0.0);
        
        manager.discover("test_egg");
        assert!(manager.progress() > 0.0);
    }
}