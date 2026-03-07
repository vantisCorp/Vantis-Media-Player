//! Vantis Mobile Core - Shared Library
//! 
//! This library provides a shared core for iOS and Android apps via FFI.
//! It exposes a C-compatible API for cross-platform functionality.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::OnceLock;
use std::ptr;
use serde_json;

pub mod player;
pub mod audio;
pub mod subtitles;
pub mod cloud;
pub mod events;

use events::EventCallback;

/// Global player instance
static PLAYER: OnceLock<std::sync::Mutex<player::MobilePlayer>> = OnceLock::new();

/// Global event callback
static mut EVENT_CALLBACK: Option<EventCallback> = None;

// ============================================================================
// Initialization
// ============================================================================

/// Initialize the Vantis mobile core
#[no_mangle]
pub extern "C" fn vantis_init(config_path: *const c_char) -> i32 {
    let config = if config_path.is_null() {
        None
    } else {
        unsafe {
            CStr::from_ptr(config_path).to_str().ok().map(String::from)
        }
    };
    
    let player = player::MobilePlayer::new(config);
    match PLAYER.set(std::sync::Mutex::new(player)) {
        Ok(_) => 0,
        Err(_) => -1, // Already initialized
    }
}

/// Shutdown the Vantis mobile core
#[no_mangle]
pub extern "C" fn vantis_shutdown() {
    // Player will be dropped automatically
}

// ============================================================================
// Playback Control
// ============================================================================

/// Start playing a media file
#[no_mangle]
pub extern "C" fn vantis_play(url: *const c_char) -> i32 {
    let player = match PLAYER.get() {
        Some(p) => p,
        None => return -1,
    };
    
    let url = unsafe {
        if url.is_null() {
            return -1;
        }
        match CStr::from_ptr(url).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };
    
    let mut player = player.lock().unwrap();
    player.play(&url).map(|_| 0).unwrap_or(-1)
}

/// Pause playback
#[no_mangle]
pub extern "C" fn vantis_pause() {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.pause();
    }
}

/// Resume playback
#[no_mangle]
pub extern "C" fn vantis_resume() {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.resume();
    }
}

/// Stop playback
#[no_mangle]
pub extern "C" fn vantis_stop() {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.stop();
    }
}

/// Seek to position in seconds
#[no_mangle]
pub extern "C" fn vantis_seek(seconds: f64) {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.seek(seconds);
    }
}

/// Get current playback position in seconds
#[no_mangle]
pub extern "C" fn vantis_get_position() -> f64 {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.position()
    } else {
        0.0
    }
}

/// Get media duration in seconds
#[no_mangle]
pub extern "C" fn vantis_get_duration() -> f64 {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.duration()
    } else {
        0.0
    }
}

/// Get playback state
#[no_mangle]
pub extern "C" fn vantis_get_state() -> i32 {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.state() as i32
    } else {
        0
    }
}

// ============================================================================
// Audio Control
// ============================================================================

/// Set volume (0.0 to 1.0)
#[no_mangle]
pub extern "C" fn vantis_set_volume(volume: f32) {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.set_volume(volume);
    }
}

/// Get current volume
#[no_mangle]
pub extern "C" fn vantis_get_volume() -> f32 {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.volume()
    } else {
        1.0
    }
}

/// Set mute state
#[no_mangle]
pub extern "C" fn vantis_set_muted(muted: bool) {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.set_muted(muted);
    }
}

/// Get mute state
#[no_mangle]
pub extern "C" fn vantis_is_muted() -> bool {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.is_muted()
    } else {
        false
    }
}

// ============================================================================
// Subtitles
// ============================================================================

/// Load subtitle file
#[no_mangle]
pub extern "C" fn vantis_load_subtitle(path: *const c_char, language: *const c_char) -> i32 {
    let path = unsafe {
        if path.is_null() {
            return -1;
        }
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };
    
    let language = if language.is_null() {
        None
    } else {
        unsafe {
            CStr::from_ptr(language).to_str().ok().map(String::from)
        }
    };
    
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.load_subtitle(&path, language.as_deref()).map(|_| 0).unwrap_or(-1)
    } else {
        -1
    }
}

/// Enable/disable subtitles
#[no_mangle]
pub extern "C" fn vantis_set_subtitle_enabled(enabled: bool) {
    if let Some(player) = PLAYER.get() {
        let mut player = player.lock().unwrap();
        player.set_subtitle_enabled(enabled);
    }
}

// ============================================================================
// Cloud Sync
// ============================================================================

/// Begin cloud synchronization
#[no_mangle]
pub extern "C" fn vantis_sync_begin() {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.begin_sync();
    }
}

/// Get sync status
#[no_mangle]
pub extern "C" fn vantis_sync_get_status() -> i32 {
    if let Some(player) = PLAYER.get() {
        let player = player.lock().unwrap();
        player.sync_status() as i32
    } else {
        0
    }
}

// ============================================================================
// Events
// ============================================================================

/// Set event callback
#[no_mangle]
pub extern "C" fn vantis_set_event_callback(callback: EventCallback) {
    unsafe {
        EVENT_CALLBACK = Some(callback);
    }
}

/// Emit an event to the callback
fn emit_event(event: &events::Event) {
    unsafe {
        if let Some(callback) = EVENT_CALLBACK {
            if let Ok(json) = serde_json::to_string(event) {
                if let Ok(c_string) = CString::new(json) {
                    callback(c_string.as_ptr());
                }
            }
        }
    }
}

// ============================================================================
// Utility
// ============================================================================

/// Get version string
#[no_mangle]
pub extern "C" fn vantis_get_version() -> *mut c_char {
    let version = env!("CARGO_PKG_VERSION");
    match CString::new(version) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string allocated by the library
#[no_mangle]
pub extern "C" fn vantis_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}