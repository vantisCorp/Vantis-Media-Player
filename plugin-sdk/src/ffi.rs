//! FFI interface for plugin communication.

use crate::error::{PluginError, PluginResult};
use crate::plugin::{PluginCapabilities, PluginContext, PluginInfo};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

/// FFI function pointer types.
pub type PluginCreateFn = extern "C" fn() -> *mut c_void;
pub type PluginDestroyFn = extern "C" fn(*mut c_void);
pub type PluginInfoFn = extern "C" fn(*const c_void) -> PluginInfoRaw;
pub type PluginCapabilitiesFn = extern "C" fn(*const c_void) -> PluginCapabilitiesRaw;

/// Raw plugin info for FFI.
#[repr(C)]
pub struct PluginInfoRaw {
    /// Plugin ID.
    pub id: *mut c_char,
    /// Plugin name.
    pub name: *mut c_char,
    /// Plugin version.
    pub version: *mut c_char,
    /// Plugin description.
    pub description: *mut c_char,
    /// Plugin author.
    pub author: *mut c_char,
    /// Plugin license.
    pub license: *mut c_char,
    /// Minimum host version.
    pub min_host_version: *mut c_char,
}

/// Raw plugin capabilities for FFI.
#[repr(C)]
#[derive(Default)]
pub struct PluginCapabilitiesRaw {
    /// Media playback capability.
    pub media_playback: bool,
    /// Media decoding capability.
    pub media_decoding: bool,
    /// UI extension capability.
    pub ui_extension: bool,
    /// Metadata provider capability.
    pub metadata_provider: bool,
    /// Network handler capability.
    pub network_handler: bool,
    /// Subtitle handler capability.
    pub subtitle_handler: bool,
    /// Audio processor capability.
    pub audio_processor: bool,
    /// Video processor capability.
    pub video_processor: bool,
    /// Keyboard shortcuts capability.
    pub keyboard_shortcuts: bool,
    /// Notifications capability.
    pub notifications: bool,
}

/// Plugin library wrapper for dynamic loading.
pub struct PluginLibrary {
    /// Path to the library.
    path: PathBuf,
    /// Handle to the loaded library.
    handle: Option<libloading::Library>,
    /// Plugin info (cached).
    info: Option<PluginInfo>,
    /// Plugin capabilities (cached).
    capabilities: Option<PluginCapabilities>,
}

impl PluginLibrary {
    /// Create a new plugin library wrapper.
    pub fn new(path: PathBuf) -> Self {
        PluginLibrary {
            path,
            handle: None,
            info: None,
            capabilities: None,
        }
    }

    /// Load the library.
    pub fn load(&mut self) -> PluginResult<()> {
        if self.handle.is_some() {
            return Err(PluginError::AlreadyLoaded(self.path.display().to_string()));
        }

        let library = unsafe {
            libloading::Library::new(&self.path)
                .map_err(|e| PluginError::load_failed(&self.path, e.to_string()))?
        };

        self.handle = Some(library);
        Ok(())
    }

    /// Unload the library.
    pub fn unload(&mut self) -> PluginResult<()> {
        if let Some(handle) = self.handle.take() {
            drop(handle);
        }
        Ok(())
    }

    /// Get plugin info.
    pub fn info(&self) -> PluginResult<PluginInfo> {
        if let Some(ref info) = self.info {
            return Ok(info.clone());
        }

        // Try to call the FFI function
        // For now, return a default
        Ok(PluginInfo::default())
    }

    /// Get plugin capabilities.
    pub fn capabilities(&self) -> PluginResult<PluginCapabilities> {
        if let Some(ref caps) = self.capabilities {
            return Ok(*caps);
        }

        Ok(PluginCapabilities::default())
    }

    /// Initialize the plugin.
    pub async fn initialize(&mut self, _context: &PluginContext) -> PluginResult<()> {
        // Call FFI initialize function
        Ok(())
    }

    /// Shutdown the plugin.
    pub async fn shutdown(&mut self) -> PluginResult<()> {
        // Call FFI shutdown function
        Ok(())
    }

    /// Enable the plugin.
    pub async fn enable(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Disable the plugin.
    pub async fn disable(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Check if the library is loaded.
    pub fn is_loaded(&self) -> bool {
        self.handle.is_some()
    }

    /// Get the library path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for PluginLibrary {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            drop(handle);
        }
    }
}

// FFI Helper functions

/// Convert a C string to a Rust string.
pub unsafe fn c_str_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_string())
}

/// Convert a Rust string to a C string.
pub fn string_to_c_str(s: &str) -> *mut c_char {
    let c_string = CString::new(s).unwrap_or_default();
    c_string.into_raw()
}

/// Free a C string allocated by Rust.
#[no_mangle]
pub extern "C" fn vantis_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

/// Free a plugin info struct.
#[no_mangle]
pub extern "C" fn vantis_free_plugin_info(info: PluginInfoRaw) {
    unsafe {
        if !info.id.is_null() {
            drop(CString::from_raw(info.id));
        }
        if !info.name.is_null() {
            drop(CString::from_raw(info.name));
        }
        if !info.version.is_null() {
            drop(CString::from_raw(info.version));
        }
        if !info.description.is_null() {
            drop(CString::from_raw(info.description));
        }
        if !info.author.is_null() {
            drop(CString::from_raw(info.author));
        }
        if !info.license.is_null() {
            drop(CString::from_raw(info.license));
        }
        if !info.min_host_version.is_null() {
            drop(CString::from_raw(info.min_host_version));
        }
    }
}

/// Convert raw FFI info to PluginInfo.
impl From<PluginInfoRaw> for PluginInfo {
    fn from(raw: PluginInfoRaw) -> Self {
        unsafe {
            PluginInfo {
                id: c_str_to_string(raw.id)
                    .map(crate::plugin::PluginId::new)
                    .unwrap_or_default(),
                name: c_str_to_string(raw.name).unwrap_or_default(),
                version: c_str_to_string(raw.version).unwrap_or_default(),
                description: c_str_to_string(raw.description).unwrap_or_default(),
                author: c_str_to_string(raw.author).unwrap_or_default(),
                license: c_str_to_string(raw.license).unwrap_or_default(),
                min_host_version: c_str_to_string(raw.min_host_version)
                    .unwrap_or_else(|| "1.0.0".to_string()),
                ..Default::default()
            }
        }
    }
}

/// Convert PluginInfo to raw FFI struct.
impl From<PluginInfo> for PluginInfoRaw {
    fn from(info: PluginInfo) -> Self {
        PluginInfoRaw {
            id: string_to_c_str(info.id.as_str()),
            name: string_to_c_str(&info.name),
            version: string_to_c_str(&info.version),
            description: string_to_c_str(&info.description),
            author: string_to_c_str(&info.author),
            license: string_to_c_str(&info.license),
            min_host_version: string_to_c_str(&info.min_host_version),
        }
    }
}

/// Convert raw FFI capabilities to PluginCapabilities.
impl From<PluginCapabilitiesRaw> for PluginCapabilities {
    fn from(raw: PluginCapabilitiesRaw) -> Self {
        PluginCapabilities {
            media_playback: raw.media_playback,
            media_decoding: raw.media_decoding,
            ui_extension: raw.ui_extension,
            metadata_provider: raw.metadata_provider,
            network_handler: raw.network_handler,
            subtitle_handler: raw.subtitle_handler,
            audio_processor: raw.audio_processor,
            video_processor: raw.video_processor,
            keyboard_shortcuts: raw.keyboard_shortcuts,
            notifications: raw.notifications,
        }
    }
}

/// Convert PluginCapabilities to raw FFI struct.
impl From<PluginCapabilities> for PluginCapabilitiesRaw {
    fn from(caps: PluginCapabilities) -> Self {
        PluginCapabilitiesRaw {
            media_playback: caps.media_playback,
            media_decoding: caps.media_decoding,
            ui_extension: caps.ui_extension,
            metadata_provider: caps.metadata_provider,
            network_handler: caps.network_handler,
            subtitle_handler: caps.subtitle_handler,
            audio_processor: caps.audio_processor,
            video_processor: caps.video_processor,
            keyboard_shortcuts: caps.keyboard_shortcuts,
            notifications: caps.notifications,
        }
    }
}

/// FFI-safe result type.
#[repr(C)]
pub struct FfiResult<T> {
    /// Whether the operation was successful.
    pub success: bool,
    /// Error message if failed.
    pub error: *mut c_char,
    /// The result value.
    pub value: T,
}

impl<T> FfiResult<T> {
    /// Create a successful result.
    pub fn ok(value: T) -> Self {
        FfiResult {
            success: true,
            error: ptr::null_mut(),
            value,
        }
    }

    /// Create an error result.
    pub fn err(message: &str) -> Self {
        FfiResult {
            success: false,
            error: string_to_c_str(message),
            value: unsafe { std::mem::zeroed() },
        }
    }
}

/// FFI-safe byte buffer.
#[repr(C)]
pub struct FfiBuffer {
    /// Pointer to data.
    pub data: *mut u8,
    /// Buffer length.
    pub len: usize,
    /// Buffer capacity.
    pub cap: usize,
}

impl FfiBuffer {
    /// Create from a Vec.
    pub fn from_vec(vec: Vec<u8>) -> Self {
        let mut vec = std::mem::ManuallyDrop::new(vec);
        FfiBuffer {
            data: vec.as_mut_ptr(),
            len: vec.len(),
            cap: vec.capacity(),
        }
    }

    /// Convert to a Vec.
    pub unsafe fn to_vec(self) -> Vec<u8> {
        Vec::from_raw_parts(self.data, self.len, self.cap)
    }
}

/// Free an FFI buffer.
#[no_mangle]
pub extern "C" fn vantis_free_buffer(buf: FfiBuffer) {
    unsafe {
        if !buf.data.is_null() {
            drop(Vec::from_raw_parts(buf.data, buf.len, buf.cap));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_str_conversion() {
        let s = "test string";
        let c_str = string_to_c_str(s);
        let back = unsafe { c_str_to_string(c_str) };
        assert_eq!(back, Some(s.to_string()));
        
        // Clean up
        unsafe { vantis_free_string(c_str) };
    }

    #[test]
    fn test_capabilities_conversion() {
        let caps = PluginCapabilities::media_player();
        let raw: PluginCapabilitiesRaw = caps.into();
        let back: PluginCapabilities = raw.into();
        
        assert_eq!(back.media_playback, true);
        assert_eq!(back.media_decoding, true);
        assert_eq!(back.ui_extension, false);
    }

    #[test]
    fn test_ffi_buffer() {
        let vec = vec![1u8, 2, 3, 4, 5];
        let ffi_buf = FfiBuffer::from_vec(vec.clone());
        assert_eq!(ffi_buf.len, 5);
        
        unsafe {
            let back = ffi_buf.to_vec();
            assert_eq!(back, vec);
        }
    }

    #[test]
    fn test_ffi_result() {
        let ok: FfiResult<i32> = FfiResult::ok(42);
        assert!(ok.success);
        assert_eq!(ok.value, 42);
        
        let err: FfiResult<i32> = FfiResult::err("test error");
        assert!(!err.success);
    }
}