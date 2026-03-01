// Example: Using keyboard shortcuts in Vantis Media Player
//
// This example demonstrates how to set up and use keyboard shortcuts
// for controlling playback programmatically.

use vantis_core::Player;
use vantis_core::input::{Keyboard, KeyCode, KeyEvent};

fn main() {
    // Create a new player instance
    let mut player = Player::new();
    
    // Load a video file
    player.load("examples/sample_video.mp4")
        .expect("Failed to load video");
    
    println!("Vantis Media Player - Keyboard Shortcuts Example");
    println!("===============================================\n");
    println!("Keyboard Shortcuts:");
    println!("  SPACE     - Play/Pause");
    println!("  RIGHT     - Seek forward 5 seconds");
    println!("  LEFT      - Seek backward 5 seconds");
    println!("  UP        - Volume up");
    println!("  DOWN      - Volume down");
    println!("  M         - Mute/Unmute");
    println!("  F         - Toggle fullscreen");
    println!("  S         - Toggle subtitles");
    println!("  Q         - Quit");
    println!("\nPress any key to start...");
    
    // Set up keyboard input
    let mut keyboard = Keyboard::new();
    
    // Main event loop
    loop {
        if let Some(event) = keyboard.next_event() {
            match event {
                KeyEvent::Pressed(KeyCode::Space) => {
                    // Toggle play/pause
                    if player.is_playing() {
                        player.pause();
                        println!("\n[PAUSED]");
                    } else {
                        player.play();
                        println!("\n[PLAYING]");
                    }
                }
                
                KeyEvent::Pressed(KeyCode::Right) => {
                    // Seek forward 5 seconds
                    let current_time = player.current_position();
                    player.seek(current_time + 5.0);
                    println!("\n[SEEK +5s] Position: {:.1}s", player.current_position());
                }
                
                KeyEvent::Pressed(KeyCode::Left) => {
                    // Seek backward 5 seconds
                    let current_time = player.current_position();
                    player.seek((current_time - 5.0).max(0.0));
                    println!("\n[SEEK -5s] Position: {:.1}s", player.current_position());
                }
                
                KeyEvent::Pressed(KeyCode::Up) => {
                    // Volume up
                    let current_volume = player.volume();
                    let new_volume = (current_volume + 0.1).min(1.0);
                    player.set_volume(new_volume);
                    println!("\n[VOLUME UP] {:.0}%", new_volume * 100.0);
                }
                
                KeyEvent::Pressed(KeyCode::Down) => {
                    // Volume down
                    let current_volume = player.volume();
                    let new_volume = (current_volume - 0.1).max(0.0);
                    player.set_volume(new_volume);
                    println!("\n[VOLUME DOWN] {:.0}%", new_volume * 100.0);
                }
                
                KeyEvent::Pressed(KeyCode::M) => {
                    // Toggle mute
                    let is_muted = player.is_muted();
                    player.set_mute(!is_muted);
                    println!("\n[MUTE] {}", if !is_muted { "ON" } else { "OFF" });
                }
                
                KeyEvent::Pressed(KeyCode::F) => {
                    // Toggle fullscreen
                    player.toggle_fullscreen();
                    println!("\n[FULLSCREEN] {}", if player.is_fullscreen() { "ON" } else { "OFF" });
                }
                
                KeyEvent::Pressed(KeyCode::S) => {
                    // Toggle subtitles
                    if player.has_subtitles() {
                        player.toggle_subtitles();
                        println!("\n[SUBTITLES] {}", if player.subtitles_enabled() { "ON" } else { "OFF" });
                    } else {
                        println!("\n[SUBTITLES] No subtitles available");
                    }
                }
                
                KeyEvent::Pressed(KeyCode::Q) => {
                    // Quit
                    println!("\n[QUIT]");
                    break;
                }
                
                _ => {}
            }
            
            // Display current status
            display_status(&player);
        }
    }
    
    println!("\nThank you for using Vantis Media Player!");
}

fn display_status(player: &Player) {
    println!(
        "Status: {} | Time: {:.1}s / {:.1}s | Volume: {:.0}% | Muted: {} | Subs: {}",
        if player.is_playing() { "PLAYING" } else { "PAUSED" },
        player.current_position(),
        player.duration(),
        player.volume() * 100.0,
        player.is_muted(),
        player.subtitles_enabled()
    );
}