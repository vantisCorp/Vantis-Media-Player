//! Advanced UI Features Example
//! 
//! This example demonstrates all advanced UI features including:
//! - Picture-in-Picture mode
//! - Mini-player mode
//! - Theater mode
//! - Gesture controls
//! - Keyboard shortcuts

use anyhow::Result;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use tracing_subscriber;
use image::RgbImage;
use std::sync::Arc;
use tokio::sync::RwLock;

use vantis_advanced_ui::{
    AdvancedUIEngine, AdvancedUIConfig,
    PipConfig, MiniPlayerConfig, TheaterModeConfig,
    GestureConfig, ShortcutConfig,
    PipPosition, MiniPlayerPosition,
    GestureType, GestureEvent, GestureData, SwipeDirection,
    ShortcutAction, Shortcut, Key, Modifier,
    PlaybackState,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=== Vantis Advanced UI Features Example ===\n");

    // Create advanced UI configuration
    let config = create_advanced_config();
    
    // Initialize the engine
    info!("Initializing Advanced UI Engine...");
    let engine = AdvancedUIEngine::new(config)?;
    info!("✓ Engine initialized successfully\n");

    // Demonstrate Picture-in-Picture
    demo_picture_in_picture(&engine).await?;

    // Demonstrate Mini-Player
    demo_mini_player(&engine).await?;

    // Demonstrate Theater Mode
    demo_theater_mode(&engine).await?;

    // Demonstrate Gesture Controls
    demo_gesture_controls(&engine).await?;

    // Demonstrate Keyboard Shortcuts
    demo_keyboard_shortcuts(&engine).await?;

    // Demonstrate Combined Features
    demo_combined_features(&engine).await?;

    info!("\n=== All demonstrations completed successfully ===");
    Ok(())
}

/// Create advanced UI configuration
fn create_advanced_config() -> AdvancedUIConfig {
    AdvancedUIConfig {
        pip: PipConfig {
            enabled: true,
            default_position: PipPosition::BottomRight,
            default_size: (320, 180),
            allow_dragging: true,
            allow_resizing: true,
            snap_to_edges: true,
            opacity: 0.95,
        },
        mini_player: MiniPlayerConfig {
            enabled: true,
            default_position: MiniPlayerPosition::BottomCenter,
            show_controls: true,
            show_progress: true,
            auto_hide_controls: true,
            auto_hide_delay: 3,
            opacity: 0.9,
        },
        theater_mode: TheaterModeConfig {
            enabled: true,
            hide_ui: true,
            dim_background: true,
            dimming_level: 0.8,
            fullscreen_by_default: false,
            show_controls_on_hover: true,
            controls_auto_hide_delay: 3,
        },
        gestures: GestureConfig {
            enabled: true,
            enable_swipe: true,
            enable_pinch: true,
            enable_tap: true,
            swipe_sensitivity: 1.0,
            pinch_sensitivity: 1.0,
            tap_duration_threshold: 300,
        },
        shortcuts: ShortcutConfig::default(),
    }
}

/// Generate test frame
fn generate_test_frame(width: u32, height: u32, color: u8) -> RgbImage {
    let mut frame = RgbImage::new(width, height);
    for pixel in frame.pixels_mut() {
        *pixel = image::Rgb([color, color, color]);
    }
    frame
}

/// Demonstrate Picture-in-Picture
async fn demo_picture_in_picture(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Picture-in-Picture Demo ###\n");
    
    // Start PiP
    info!("Starting Picture-in-Picture...");
    engine.pip().start().await?;
    info!("✓ PiP started");
    
    // Get state
    let state = engine.pip().get_state().await;
    info!("PiP State:");
    info!("  Active: {}", state.is_active);
    info!("  Visible: {}", state.is_visible);
    info!("  Position: {:?}", state.position);
    info!("  Size: {}x{}", state.size.0, state.size.1);
    info!("  Opacity: {:.2}", state.opacity);
    
    // Update frame
    let frame = generate_test_frame(320, 180, 128);
    engine.pip().update_frame(&frame).await?;
    info!("✓ Frame updated");
    
    // Change position
    engine.pip().set_position(PipPosition::TopLeft).await?;
    info!("✓ Position changed to TopLeft");
    
    // Change size
    engine.pip().set_size(400, 225).await?;
    info!("✓ Size changed to 400x225");
    
    // Change opacity
    engine.pip().set_opacity(0.8).await?;
    info!("✓ Opacity changed to 0.8");
    
    // Snap to edge
    engine.pip().snap_to_edge().await?;
    info!("✓ Snapped to edge");
    
    // Hide and show
    engine.pip().hide().await?;
    info!("✓ PiP hidden");
    sleep(Duration::from_millis(500)).await;
    engine.pip().show().await?;
    info!("✓ PiP shown");
    
    // Stop PiP
    engine.pip().stop().await?;
    info!("✓ PiP stopped\n");
    
    Ok(())
}

/// Demonstrate Mini-Player
async fn demo_mini_player(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Mini-Player Demo ###\n");
    
    // Start mini-player
    info!("Starting mini-player...");
    engine.mini_player().start().await?;
    info!("✓ Mini-player started");
    
    // Get state
    let state = engine.mini_player().get_state().await;
    info!("Mini-Player State:");
    info!("  Active: {}", state.is_active);
    info!("  Visible: {}", state.is_visible);
    info!("  Position: {:?}", state.position);
    info!("  Controls Visible: {}", state.controls_visible);
    info!("  Opacity: {:.2}", state.opacity);
    
    // Update playback state
    let playback_state = PlaybackState {
        is_playing: true,
        current_position: 45.5,
        duration: 120.0,
        volume: 0.8,
        is_muted: false,
    };
    engine.mini_player().update_playback_state(playback_state).await?;
    info!("✓ Playback state updated");
    
    // Toggle playback
    engine.mini_player().toggle_playback().await?;
    info!("✓ Playback toggled");
    
    // Seek
    engine.mini_player().seek(60.0).await?;
    info!("✓ Seeked to 60.0s");
    
    // Set volume
    engine.mini_player().set_volume(0.5).await?;
    info!("✓ Volume set to 0.5");
    
    // Toggle mute
    engine.mini_player().toggle_mute().await?;
    info!("✓ Mute toggled");
    
    // Show/hide controls
    engine.mini_player().hide_controls().await?;
    info!("✓ Controls hidden");
    sleep(Duration::from_millis(500)).await;
    engine.mini_player().show_controls().await?;
    info!("✓ Controls shown");
    
    // Change position
    engine.mini_player().set_position(MiniPlayerPosition::TopRight).await?;
    info!("✓ Position changed to TopRight");
    
    // Stop mini-player
    engine.mini_player().stop().await?;
    info!("✓ Mini-player stopped\n");
    
    Ok(())
}

/// Demonstrate Theater Mode
async fn demo_theater_mode(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Theater Mode Demo ###\n");
    
    // Enter theater mode
    info!("Entering theater mode...");
    engine.theater_mode().enter().await?;
    info!("✓ Theater mode entered");
    
    // Get state
    let state = engine.theater_mode().get_state().await;
    info!("Theater Mode State:");
    info!("  Active: {}", state.is_active);
    info!("  Fullscreen: {}", state.is_fullscreen);
    info!("  UI Visible: {}", state.ui_visible);
    info!("  Controls Visible: {}", state.controls_visible);
    info!("  Dimming Level: {:.2}", state.dimming_level);
    
    // Toggle fullscreen
    engine.theater_mode().toggle_fullscreen().await?;
    info!("✓ Fullscreen toggled");
    
    // Set dimming level
    engine.theater_mode().set_dimming_level(0.9).await?;
    info!("✓ Dimming level set to 0.9");
    
    // Toggle UI
    engine.theater_mode().toggle_ui().await?;
    info!("✓ UI toggled");
    
    // Show/hide controls
    engine.theater_mode().hide_controls().await?;
    info!("✓ Controls hidden");
    sleep(Duration::from_millis(500)).await;
    engine.theater_mode().show_controls().await?;
    info!("✓ Controls shown");
    
    // Exit theater mode
    engine.theater_mode().exit().await?;
    info!("✓ Theater mode exited\n");
    
    Ok(())
}

/// Demonstrate Gesture Controls
async fn demo_gesture_controls(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Gesture Controls Demo ###\n");
    
    // Register gesture callbacks
    let swipe_right_count = Arc::new(RwLock::new(0));
    let swipe_right_count_clone = swipe_right_count.clone();
    
    engine.gestures().register_callback(GestureType::SwipeRight, Arc::new(move |event| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *swipe_right_count_clone.write().await += 1;
                info!("Swipe Right detected: distance={:.2}, velocity={:.2}",
                    match &event.data {
                        GestureData::Swipe { distance, .. } => *distance,
                        _ => 0.0,
                    },
                    event.velocity
                );
            })
        });
    })).await;
    
    let tap_count = Arc::new(RwLock::new(0));
    let tap_count_clone = tap_count.clone();
    
    engine.gestures().register_callback(GestureType::Tap, Arc::new(move |event| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *tap_count_clone.write().await += 1;
                info!("Tap detected: position={:?}",
                    match &event.data {
                        GestureData::Tap { position, .. } => position,
                        _ => &(0.0, 0.0),
                    }
                );
            })
        });
    })).await;
    
    info!("✓ Gesture callbacks registered");
    
    // Simulate swipe right
    info!("Simulating swipe right...");
    engine.gestures().handle_touch_start(100.0, 100.0, 0).await?;
    engine.gestures().handle_touch_move(200.0, 100.0, 100).await?;
    engine.gestures().handle_touch_move(300.0, 100.0, 200).await?;
    engine.gestures().handle_touch_end(400.0, 100.0, 300).await?;
    sleep(Duration::from_millis(100)).await;
    
    // Simulate tap
    info!("Simulating tap...");
    engine.gestures().handle_touch_start(100.0, 100.0, 0).await?;
    engine.gestures().handle_touch_end(100.0, 100.0, 100).await?;
    sleep(Duration::from_millis(100)).await;
    
    // Simulate pinch
    info!("Simulating pinch out...");
    engine.gestures().handle_pinch_start(100.0, 100.0, 200.0, 200.0, 0).await?;
    engine.gestures().handle_pinch_move(100.0, 100.0, 250.0, 200.0, 100).await?;
    engine.gestures().handle_pinch_end(100.0, 100.0, 300.0, 200.0, 200).await?;
    sleep(Duration::from_millis(100)).await;
    
    info!("✓ Gesture controls demonstrated\n");
    
    Ok(())
}

/// Demonstrate Keyboard Shortcuts
async fn demo_keyboard_shortcuts(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Keyboard Shortcuts Demo ###\n");
    
    // Register shortcut callbacks
    let play_pause_count = Arc::new(RwLock::new(0));
    let play_pause_count_clone = play_pause_count.clone();
    
    engine.shortcuts().register_callback(ShortcutAction::PlayPause, Arc::new(move |action| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *play_pause_count_clone.write().await += 1;
                info!("Shortcut triggered: {:?}", action);
            })
        });
    })).await;
    
    let fullscreen_count = Arc::new(RwLock::new(0));
    let fullscreen_count_clone = fullscreen_count.clone();
    
    engine.shortcuts().register_callback(ShortcutAction::Fullscreen, Arc::new(move |action| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *fullscreen_count_clone.write().await += 1;
                info!("Shortcut triggered: {:?}", action);
            })
        });
    })).await;
    
    info!("✓ Shortcut callbacks registered");
    
    // Get all shortcuts
    let shortcuts = engine.shortcuts().get_all_shortcuts().await;
    info!("Available shortcuts ({}):", shortcuts.len());
    for shortcut in shortcuts {
        info!("  {:?}: {}", shortcut.action, shortcut);
    }
    
    // Simulate key press
    info!("Simulating Space key press...");
    engine.shortcuts().handle_key_press(Key::Space, vec![]).await?;
    engine.shortcuts().handle_key_release(Key::Space, vec![]).await?;
    sleep(Duration::from_millis(100)).await;
    
    // Simulate Ctrl+F key press
    info!("Simulating Ctrl+F key press...");
    engine.shortcuts().handle_key_press(Key::F, vec![Modifier::Control]).await?;
    engine.shortcuts().handle_key_release(Key::F, vec![Modifier::Control]).await?;
    sleep(Duration::from_millis(100)).await;
    
    // Add custom shortcut
    info!("Adding custom shortcut...");
    let custom_shortcut = Shortcut {
        action: ShortcutAction::PlayPause,
        keys: vec![Key::K],
        modifiers: vec![],
    };
    engine.shortcuts().add_shortcut(custom_shortcut).await?;
    info!("✓ Custom shortcut added");
    
    // Simulate custom shortcut
    info!("Simulating K key press...");
    engine.shortcuts().handle_key_press(Key::K, vec![]).await?;
    engine.shortcuts().handle_key_release(Key::K, vec![]).await?;
    sleep(Duration::from_millis(100)).await;
    
    // Export shortcuts
    info!("Exporting shortcuts...");
    let json = engine.shortcuts().export_shortcuts().await?;
    info!("✓ Shortcuts exported ({} bytes)", json.len());
    
    // Import shortcuts
    info!("Importing shortcuts...");
    let engine2 = AdvancedUIEngine::new(create_advanced_config())?;
    engine2.shortcuts().import_shortcuts(&json).await?;
    info!("✓ Shortcuts imported");
    
    // Reset to defaults
    info!("Resetting to defaults...");
    engine.shortcuts().reset_to_defaults().await?;
    info!("✓ Reset to defaults\n");
    
    Ok(())
}

/// Demonstrate Combined Features
async fn demo_combined_features(&engine: &AdvancedUIEngine) -> Result<()> {
    info!("### Combined Features Demo ###\n");
    
    // Start Picture-in-Picture
    engine.pip().start().await?;
    info!("✓ PiP started");
    
    // Start mini-player
    engine.mini_player().start().await?;
    info!("✓ Mini-player started");
    
    // Enter theater mode
    engine.theater_mode().enter().await?;
    info!("✓ Theater mode entered");
    
    // Update playback state
    let playback_state = PlaybackState {
        is_playing: true,
        current_position: 30.0,
        duration: 120.0,
        volume: 0.7,
        is_muted: false,
    };
    engine.mini_player().update_playback_state(playback_state).await?;
    info!("✓ Playback state updated");
    
    // Update PiP frame
    let frame = generate_test_frame(320, 180, 150);
    engine.pip().update_frame(&frame).await?;
    info!("✓ PiP frame updated");
    
    // Adjust theater mode dimming
    engine.theater_mode().set_dimming_level(0.85).await?;
    info!("✓ Dimming level adjusted");
    
    // Show all states
    info!("\nCombined States:");
    info!("  PiP Active: {}", engine.pip().get_state().await.is_active);
    info!("  Mini-Player Active: {}", engine.mini_player().get_state().await.is_active);
    info!("  Theater Mode Active: {}", engine.theater_mode().get_state().await.is_active);
    
    // Stop all features
    engine.pip().stop().await?;
    engine.mini_player().stop().await?;
    engine.theater_mode().exit().await?;
    info!("\n✓ All features stopped\n");
    
    Ok(())
}