# Vantis Advanced UI Features

## Overview

The Vantis Advanced UI module provides cutting-edge user interface components for the Vantis Media Player. This module implements advanced UI features including Picture-in-Picture mode, mini-player mode, theater mode, gesture controls, and customizable keyboard shortcuts.

## Features

### 1. Picture-in-Picture Mode

Picture-in-Picture (PiP) allows users to watch videos in a small floating window while using other applications or browsing the web.

#### Capabilities

- **Floating Window**: Small, always-on-top video window
- **Draggable**: Move the PiP window anywhere on screen
- **Resizable**: Adjust the PiP window size
- **Snap to Edges**: Automatically snap to screen edges
- **Opacity Control**: Adjust window transparency
- **Multiple Positions**: Top-left, top-right, bottom-left, bottom-right, or custom

#### Usage Example

```rust
use vantis_advanced_ui::{AdvancedUIEngine, AdvancedUIConfig, PipConfig};

// Create engine with PiP enabled
let mut config = AdvancedUIConfig::default();
config.pip.enabled = true;
config.pip.default_position = PipPosition::BottomRight;
config.pip.allow_dragging = true;
config.pip.snap_to_edges = true;

let engine = AdvancedUIEngine::new(config)?;

// Start Picture-in-Picture
engine.pip().start().await?;

// Update frame
engine.pip().update_frame(&frame).await?;

// Stop Picture-in-Picture
engine.pip().stop().await?;
```

#### Configuration Options

- `enabled`: Enable/disable PiP
- `default_position`: Default window position
- `default_size`: Default window size (width, height)
- `allow_dragging`: Allow window dragging
- `allow_resizing`: Allow window resizing
- `snap_to_edges`: Snap to screen edges
- `opacity`: Window opacity (0.0 - 1.0)

#### PiP Positions

- `TopLeft`: Top-left corner
- `TopRight`: Top-right corner
- `BottomLeft`: Bottom-left corner
- `BottomRight`: Bottom-right corner
- `Custom`: Custom position with x, y coordinates

---

### 2. Mini-Player Mode

The mini-player provides a compact player that can be displayed in a corner of the screen while browsing or using other applications.

#### Capabilities

- **Compact Design**: Small, unobtrusive player
- **Playback Controls**: Play/pause, seek, volume controls
- **Progress Bar**: Show playback progress
- **Auto-Hide Controls**: Automatically hide controls after inactivity
- **Multiple Positions**: 6 predefined positions
- **Opacity Control**: Adjust transparency

#### Usage Example

```rust
// Start mini-player
engine.mini_player().start().await?;

// Update playback state
let state = PlaybackState {
    is_playing: true,
    current_position: 45.5,
    duration: 120.0,
    volume: 0.8,
    is_muted: false,
};
engine.mini_player().update_playback_state(state).await?;

// Toggle playback
engine.mini_player().toggle_playback().await?;

// Seek to position
engine.mini_player().seek(60.0).await?;

// Set volume
engine.mini_player().set_volume(0.5).await?;

// Stop mini-player
engine.mini_player().stop().await?;
```

#### Configuration Options

- `enabled`: Enable/disable mini-player
- `default_position`: Default player position
- `show_controls`: Show playback controls
- `show_progress`: Show progress bar
- `auto_hide_controls`: Auto-hide controls after inactivity
- `auto_hide_delay`: Auto-hide delay in seconds
- `opacity`: Player opacity (0.0 - 1.0)

#### Mini-Player Positions

- `TopLeft`: Top-left corner
- `TopCenter`: Top-center
- `TopRight`: Top-right corner
- `BottomLeft`: Bottom-left corner
- `BottomCenter`: Bottom-center
- `BottomRight`: Bottom-right corner

---

### 3. Theater Mode

Theater mode provides an immersive viewing experience with dimmed background and minimal UI distractions.

#### Capabilities

- **Dimmed Background**: Darken the background for focus
- **Fullscreen Mode**: Optional fullscreen by default
- **Hide UI**: Hide all UI elements
- **Auto-Hide Controls**: Automatically hide controls
- **Adjustable Dimming**: Control background dimming level
- **Toggle Mode**: Easy toggle on/off

#### Usage Example

```rust
// Enter theater mode
engine.theater_mode().enter().await?;

// Toggle fullscreen
engine.theater_mode().toggle_fullscreen().await?;

// Adjust dimming level
engine.theater_mode().set_dimming_level(0.9).await?;

// Show/hide UI
engine.theater_mode().toggle_ui().await?;

// Show/hide controls
engine.theater_mode().show_controls().await?;
engine.theater_mode().hide_controls().await?;

// Exit theater mode
engine.theater_mode().exit().await?;
```

#### Configuration Options

- `enabled`: Enable/disable theater mode
- `hide_ui`: Hide UI elements
- `dim_background`: Dim background
- `dimming_level`: Background dimming level (0.0 - 1.0)
- `fullscreen_by_default`: Enter fullscreen by default
- `show_controls_on_hover`: Show controls on hover
- `controls_auto_hide_delay`: Controls auto-hide delay in seconds

---

### 4. Gesture Controls

Gesture controls provide touch-based navigation and playback control for touch-enabled devices.

#### Supported Gestures

- **Swipe Left**: Seek backward or previous track
- **Swipe Right**: Seek forward or next track
- **Swipe Up**: Volume up
- **Swipe Down**: Volume down
- **Pinch In**: Zoom out
- **Pinch Out**: Zoom in
- **Tap**: Play/pause
- **Double Tap**: Toggle fullscreen
- **Long Press**: Show context menu

#### Usage Example

```rust
// Register gesture callbacks
engine.gestures().register_callback(GestureType::SwipeRight, Arc::new(|event| {
    println!("Swipe right detected");
    // Handle swipe right (seek forward)
})).await;

engine.gestures().register_callback(GestureType::Tap, Arc::new(|event| {
    println!("Tap detected");
    // Handle tap (play/pause)
})).await;

// Handle touch events
engine.gestures().handle_touch_start(100.0, 100.0, 0).await?;
engine.gestures().handle_touch_move(200.0, 100.0, 100).await?;
engine.gestures().handle_touch_end(300.0, 100.0, 200).await?;
```

#### Configuration Options

- `enabled`: Enable/disable gesture controls
- `enable_swipe`: Enable swipe gestures
- `enable_pinch`: Enable pinch gestures
- `enable_tap`: Enable tap gestures
- `swipe_sensitivity`: Swipe sensitivity multiplier
- `pinch_sensitivity`: Pinch sensitivity multiplier
- `tap_duration_threshold`: Tap duration threshold in milliseconds

#### Gesture Types

- `SwipeLeft`: Swipe left gesture
- `SwipeRight`: Swipe right gesture
- `SwipeUp`: Swipe up gesture
- `SwipeDown`: Swipe down gesture
- `PinchIn`: Pinch in gesture
- `PinchOut`: Pinch out gesture
- `Tap`: Single tap gesture
- `DoubleTap`: Double tap gesture
- `LongPress`: Long press gesture

---

### 5. Keyboard Shortcuts

Customizable keyboard shortcuts provide quick access to all player functions.

#### Default Shortcuts

| Action | Shortcut |
|--------|----------|
| Play/Pause | Space |
| Seek Forward | Right |
| Seek Backward | Left |
| Volume Up | Up |
| Volume Down | Down |
| Mute | M |
| Fullscreen | F |
| Toggle Subtitles | S |
| Picture-in-Picture | Ctrl+P |
| Mini-Player | Ctrl+M |
| Theater Mode | Ctrl+T |

#### Usage Example

```rust
// Register shortcut callback
engine.shortcuts().register_callback(ShortcutAction::PlayPause, Arc::new(|action| {
    println!("Play/Pause triggered");
    // Handle play/pause
})).await;

// Add custom shortcut
let custom_shortcut = Shortcut {
    action: ShortcutAction::PlayPause,
    keys: vec![Key::K],
    modifiers: vec![],
};
engine.shortcuts().add_shortcut(custom_shortcut).await?;

// Handle key events
engine.shortcuts().handle_key_press(Key::Space, vec![]).await?;
engine.shortcuts().handle_key_release(Key::Space, vec![]).await?;

// Export shortcuts
let json = engine.shortcuts().export_shortcuts().await?;

// Import shortcuts
engine.shortcuts().import_shortcuts(&json).await?;
```

#### Configuration Options

- `enabled`: Enable/disable custom shortcuts
- `allow_customization`: Allow user customization
- `default_shortcuts`: Default shortcut definitions
- `custom_shortcuts`: Custom shortcut definitions

#### Shortcut Actions

- `PlayPause`: Toggle playback
- `Stop`: Stop playback
- `SeekForward`: Seek forward (small)
- `SeekBackward`: Seek backward (small)
- `SeekForwardLarge`: Seek forward (large)
- `SeekBackwardLarge`: Seek backward (large)
- `VolumeUp`: Increase volume
- `VolumeDown`: Decrease volume
- `Mute`: Toggle mute
- `Fullscreen`: Toggle fullscreen
- `ToggleSubtitles`: Toggle subtitles
- `NextSubtitle`: Next subtitle
- `PreviousSubtitle`: Previous subtitle
- `NextTrack`: Next track
- `PreviousTrack`: Previous track
- `PictureInPicture`: Toggle Picture-in-Picture
- `MiniPlayer`: Toggle mini-player
- `TheaterMode`: Toggle theater mode
- `Screenshot`: Take screenshot
- `FrameStepForward`: Step forward one frame
- `FrameStepBackward`: Step backward one frame
- `SpeedUp`: Increase playback speed
- `SpeedDown`: Decrease playback speed
- `ResetSpeed`: Reset playback speed
- `ZoomIn`: Zoom in
- `ZoomOut`: Zoom out
- `ResetZoom`: Reset zoom
- `RotateLeft`: Rotate left
- `RotateRight`: Rotate right
- `FlipHorizontal`: Flip horizontally
- `FlipVertical`: Flip vertically

---

## Performance Considerations

### Resource Usage

- **Picture-in-Picture**: Minimal CPU overhead, GPU for rendering
- **Mini-Player**: Low CPU usage, efficient rendering
- **Theater Mode**: Minimal overhead, mainly UI state changes
- **Gesture Controls**: Low CPU usage, event-driven
- **Keyboard Shortcuts**: Negligible overhead

### Memory Usage

- **Picture-in-Picture**: ~10-20 MB for window and frame buffer
- **Mini-Player**: ~5-10 MB for player and controls
- **Theater Mode**: ~1-2 MB for state management
- **Gesture Controls**: ~1-2 MB for gesture recognizers
- **Keyboard Shortcuts**: <1 MB for shortcut storage

---

## Best Practices

### Picture-in-Picture

1. Use snap-to-edges for better UX
2. Set appropriate opacity for visibility
3. Enable dragging for flexibility
4. Use reasonable default size (320x180 recommended)
5. Handle window focus events properly

### Mini-Player

1. Enable auto-hide for cleaner UI
2. Show essential controls only
3. Use appropriate auto-hide delay (2-3 seconds)
4. Position in non-intrusive location
5. Update playback state regularly

### Theater Mode

1. Use appropriate dimming level (0.7-0.9 recommended)
2. Enable fullscreen for immersive experience
3. Show controls on hover for accessibility
4. Provide easy toggle on/off
5. Handle window resize events

### Gesture Controls

1. Adjust sensitivity for your content
2. Provide visual feedback for gestures
3. Register callbacks for all needed gestures
4. Handle gesture conflicts properly
5. Test on different devices

### Keyboard Shortcuts

1. Use intuitive key combinations
2. Avoid conflicts with system shortcuts
3. Provide shortcut customization
4. Document all shortcuts
5. Support export/import for backup

---

## Troubleshooting

### Picture-in-Picture Issues

**Problem**: PiP window not showing
- **Solution**: Check if PiP is enabled in config
- **Solution**: Verify window handle is created
- **Solution**: Check if frame is being updated

**Problem**: PiP window not snapping
- **Solution**: Enable snap_to_edges in config
- **Solution**: Check position coordinates
- **Solution**: Call snap_to_edge() manually

### Mini-Player Issues

**Problem**: Controls not auto-hiding
- **Solution**: Enable auto_hide_controls in config
- **Solution**: Check auto_hide_delay value
- **Solution**: Verify timer is running

**Problem**: Playback state not updating
- **Solution**: Call update_playback_state() regularly
- **Solution**: Check state values are valid
- **Solution**: Verify callback is registered

### Theater Mode Issues

**Problem**: Background not dimming
- **Solution**: Enable dim_background in config
- **Solution**: Check dimming_level value
- **Solution**: Verify theater mode is active

**Problem**: Controls not hiding
- **Solution**: Enable show_controls_on_hover
- **Solution**: Check controls_auto_hide_delay
- **Solution**: Verify timer is running

### Gesture Control Issues

**Problem**: Gestures not recognized
- **Solution**: Enable gesture type in config
- **Solution**: Adjust sensitivity values
- **Solution**: Check touch event handling

**Problem**: Callbacks not triggered
- **Solution**: Register callback for gesture type
- **Solution**: Verify callback is correct type
- **Solution**: Check gesture recognition logic

### Keyboard Shortcut Issues

**Problem**: Shortcuts not working
- **Solution**: Enable shortcuts in config
- **Solution**: Check key and modifier values
- **Solution**: Verify handle_key_press() is called

**Problem**: Custom shortcuts not saving
- **Solution**: Enable allow_customization in config
- **Solution**: Check shortcut format
- **Solution**: Verify import/export works

---

## API Reference

### AdvancedUIEngine

Main engine coordinating all advanced UI features.

```rust
pub struct AdvancedUIEngine {
    pip: Arc<PictureInPicture>,
    mini_player: Arc<MiniPlayer>,
    theater_mode: Arc<TheaterMode>,
    gestures: Arc<GestureController>,
    shortcuts: Arc<ShortcutManager>,
}
```

#### Methods

- `new(config: AdvancedUIConfig) -> Result<Self>`
- `pip(&self) -> &PictureInPicture`
- `mini_player(&self) -> &MiniPlayer`
- `theater_mode(&self) -> &TheaterMode`
- `gestures(&self) -> &GestureController`
- `shortcuts(&self) -> &ShortcutManager`

### PictureInPicture

Picture-in-Picture controller.

```rust
pub struct PictureInPicture {
    config: Arc<RwLock<PipConfig>>,
    position: Arc<RwLock<PipPosition>>,
    size: Arc<RwLock<(u32, u32)>>,
    is_active: Arc<RwLock<bool>>,
    // ...
}
```

#### Methods

- `new(config: PipConfig) -> Result<Self>`
- `set_config(&self, config: PipConfig) -> Result<()>`
- `get_config(&self) -> PipConfig`
- `start(&self) -> Result<()>`
- `stop(&self) -> Result<()>`
- `show(&self) -> Result<()>`
- `hide(&self) -> Result<()>`
- `set_position(&self, position: PipPosition) -> Result<()>`
- `get_position(&self) -> PipPosition`
- `set_size(&self, width: u32, height: u32) -> Result<()>`
- `get_size(&self) -> (u32, u32)`
- `set_opacity(&self, opacity: f32) -> Result<()>`
- `get_opacity(&self) -> f32`
- `update_frame(&self, frame: &RgbImage) -> Result<()>`
- `get_current_frame(&self) -> Option<RgbImage>`
- `get_state(&self) -> PipState`
- `snap_to_edge(&self) -> Result<()>`

### MiniPlayer

Mini-player controller.

```rust
pub struct MiniPlayer {
    config: Arc<RwLock<MiniPlayerConfig>>,
    position: Arc<RwLock<MiniPlayerPosition>>,
    playback_state: Arc<RwLock<PlaybackState>>,
    // ...
}
```

#### Methods

- `new(config: MiniPlayerConfig) -> Result<Self>`
- `set_config(&self, config: MiniPlayerConfig) -> Result<()>`
- `get_config(&self) -> MiniPlayerConfig`
- `start(&self) -> Result<()>`
- `stop(&self) -> Result<()>`
- `show(&self) -> Result<()>`
- `hide(&self) -> Result<()>`
- `set_position(&self, position: MiniPlayerPosition) -> Result<()>`
- `get_position(&self) -> MiniPlayerPosition`
- `update_frame(&self, frame: &RgbImage) -> Result<()>`
- `get_current_frame(&self) -> Option<RgbImage>`
- `update_playback_state(&self, state: PlaybackState) -> Result<()>`
- `get_playback_state(&self) -> PlaybackState`
- `toggle_playback(&self) -> Result<()>`
- `seek(&self, position: f64) -> Result<()>`
- `set_volume(&self, volume: f32) -> Result<()>`
- `toggle_mute(&self) -> Result<()>`
- `show_controls(&self) -> Result<()>`
- `hide_controls(&self) -> Result<()>`
- `get_state(&self) -> MiniPlayerState`

### TheaterMode

Theater mode controller.

```rust
pub struct TheaterMode {
    config: Arc<RwLock<TheaterModeConfig>>,
    is_active: Arc<RwLock<bool>>,
    is_fullscreen: Arc<RwLock<bool>>,
    // ...
}
```

#### Methods

- `new(config: TheaterModeConfig) -> Result<Self>`
- `set_config(&self, config: TheaterModeConfig) -> Result<()>`
- `get_config(&self) -> TheaterModeConfig`
- `enter(&self) -> Result<()>`
- `exit(&self) -> Result<()>`
- `toggle(&self) -> Result<()>`
- `enter_fullscreen(&self) -> Result<()>`
- `exit_fullscreen(&self) -> Result<()>`
- `toggle_fullscreen(&self) -> Result<()>`
- `show_ui(&self) -> Result<()>`
- `hide_ui(&self) -> Result<()>`
- `toggle_ui(&self) -> Result<()>`
- `show_controls(&self) -> Result<()>`
- `hide_controls(&self) -> Result<()>`
- `set_dimming_level(&self, level: f32) -> Result<()>`
- `get_dimming_level(&self) -> f32`
- `get_state(&self) -> TheaterModeState`

### GestureController

Gesture controller.

```rust
pub struct GestureController {
    config: Arc<RwLock<GestureConfig>>,
    swipe_recognizer: Arc<RwLock<SwipeRecognizer>>,
    pinch_recognizer: Arc<RwLock<PinchRecognizer>>,
    tap_recognizer: Arc<RwLock<TapRecognizer>>,
    // ...
}
```

#### Methods

- `new(config: GestureConfig) -> Result<Self>`
- `set_config(&self, config: GestureConfig) -> Result<()>`
- `get_config(&self) -> GestureConfig`
- `register_callback(&self, gesture_type: GestureType, callback: GestureCallback)`
- `unregister_callback(&self, gesture_type: GestureType)`
- `handle_touch_start(&self, x: f32, y: f32, timestamp: u64) -> Result<()>`
- `handle_touch_move(&self, x: f32, y: f32, timestamp: u64) -> Result<()>`
- `handle_touch_end(&self, x: f32, y: f32, timestamp: u64) -> Result<()>`
- `handle_pinch_start(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()>`
- `handle_pinch_move(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()>`
- `handle_pinch_end(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()>`

### ShortcutManager

Shortcut manager.

```rust
pub struct ShortcutManager {
    config: Arc<RwLock<ShortcutConfig>>,
    shortcuts: Arc<RwLock<HashMap<ShortcutAction, Shortcut>>>,
    key_states: Arc<RwLock<HashMap<Key, bool>>>,
    // ...
}
```

#### Methods

- `new(config: ShortcutConfig) -> Result<Self>`
- `set_config(&self, config: ShortcutConfig) -> Result<()>`
- `get_config(&self) -> ShortcutConfig`
- `register_callback(&self, action: ShortcutAction, callback: ShortcutCallback)`
- `unregister_callback(&self, action: ShortcutAction)`
- `add_shortcut(&self, shortcut: Shortcut) -> Result<()>`
- `remove_shortcut(&self, action: ShortcutAction) -> Result<()>`
- `get_shortcut(&self, action: ShortcutAction) -> Option<Shortcut>`
- `get_all_shortcuts(&self) -> Vec<Shortcut>`
- `get_default_shortcuts(&self) -> Vec<Shortcut>`
- `get_custom_shortcuts(&self) -> Vec<Shortcut>`
- `reset_to_defaults(&self) -> Result<()>`
- `handle_key_press(&self, key: Key, modifiers: Vec<Modifier>) -> Result<()>`
- `handle_key_release(&self, key: Key, modifiers: Vec<Modifier>) -> Result<()>`
- `get_key_state(&self, key: Key) -> bool`
- `get_modifier_state(&self, modifier: Modifier) -> bool`
- `export_shortcuts(&self) -> Result<String>`
- `import_shortcuts(&self, json: &str) -> Result<()>`

---

## Examples

See `examples/advanced_ui_example.rs` for comprehensive examples of all advanced UI features.

---

## License

MIT License - See LICENSE file for details.