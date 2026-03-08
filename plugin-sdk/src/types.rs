//! Common types for the plugin system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEventKind {
    /// Plugin was loaded.
    Loaded,
    /// Plugin was initialized.
    Initialized,
    /// Plugin was enabled.
    Enabled,
    /// Plugin was disabled.
    Disabled,
    /// Plugin was unloaded.
    Unloaded,
    /// Plugin encountered an error.
    Error,
    /// Plugin state changed.
    StateChanged,
    /// Plugin configuration changed.
    ConfigChanged,
}

/// Plugin event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEventData {
    /// Event kind.
    pub kind: PluginEventKind,
    /// Plugin ID.
    pub plugin_id: String,
    /// Event timestamp (Unix milliseconds).
    pub timestamp: u64,
    /// Additional event data.
    pub data: HashMap<String, serde_json::Value>,
}

/// Plugin statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginStats {
    /// Number of times the plugin was loaded.
    pub load_count: u64,
    /// Number of times the plugin was enabled.
    pub enable_count: u64,
    /// Total time the plugin was active (seconds).
    pub active_time_secs: u64,
    /// Number of errors encountered.
    pub error_count: u64,
    /// Last error message.
    pub last_error: Option<String>,
    /// Memory usage in bytes.
    pub memory_usage: u64,
    /// CPU time used (milliseconds).
    pub cpu_time_ms: u64,
}

/// Plugin health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginHealth {
    /// Plugin is healthy.
    Healthy,
    /// Plugin has warnings.
    Warning,
    /// Plugin has errors.
    Error,
    /// Plugin health is unknown.
    Unknown,
}

/// Plugin diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDiagnostics {
    /// Plugin ID.
    pub plugin_id: String,
    /// Health status.
    pub health: PluginHealth,
    /// Diagnostic messages.
    pub messages: Vec<DiagnosticMessage>,
    /// Statistics.
    pub stats: PluginStats,
}

/// Diagnostic message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticMessage {
    /// Message severity.
    pub severity: DiagnosticSeverity,
    /// Message code (if any).
    pub code: Option<String>,
    /// Message text.
    pub message: String,
    /// Timestamp.
    pub timestamp: u64,
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Plugin priority for ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PluginPriority {
    /// Lowest priority.
    Lowest = 0,
    /// Low priority.
    Low = 1,
    /// Normal priority.
    Normal = 2,
    /// High priority.
    High = 3,
    /// Highest priority.
    Highest = 4,
}

impl Default for PluginPriority {
    fn default() -> Self {
        PluginPriority::Normal
    }
}

/// Plugin load strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadStrategy {
    /// Load plugins on demand.
    OnDemand,
    /// Load all plugins at startup.
    Eager,
    /// Load plugins in background.
    Lazy,
}

impl Default for LoadStrategy {
    fn default() -> Self {
        LoadStrategy::OnDemand
    }
}

/// Plugin conflict resolution strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Keep the first loaded plugin.
    FirstWins,
    /// Keep the last loaded plugin.
    LastWins,
    /// Keep the higher priority plugin.
    PriorityWins,
    /// Keep the newer version.
    NewerWins,
    /// Fail on conflict.
    Fail,
}

impl Default for ConflictResolution {
    fn default() -> Self {
        ConflictResolution::PriorityWins
    }
}

/// Plugin isolation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationLevel {
    /// No isolation (shared memory).
    None,
    /// Process isolation.
    Process,
    /// Sandbox isolation.
    Sandbox,
    /// Container isolation.
    Container,
}

impl Default for IsolationLevel {
    fn default() -> Self {
        IsolationLevel::None
    }
}

/// Plugin security context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Whether filesystem access is allowed.
    pub filesystem_access: bool,
    /// Whether network access is allowed.
    pub network_access: bool,
    /// Allowed paths for filesystem access.
    pub allowed_paths: Vec<String>,
    /// Allowed hosts for network access.
    pub allowed_hosts: Vec<String>,
    /// Maximum memory usage (bytes).
    pub max_memory: Option<u64>,
    /// Maximum CPU time (milliseconds).
    pub max_cpu_time: Option<u64>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        SecurityContext {
            filesystem_access: false,
            network_access: false,
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
            max_memory: None,
            max_cpu_time: None,
        }
    }
}

/// Plugin resource usage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory used (bytes).
    pub memory_bytes: u64,
    /// CPU time used (milliseconds).
    pub cpu_time_ms: u64,
    /// Number of threads.
    pub thread_count: u32,
    /// Number of open file descriptors.
    pub open_files: u32,
    /// Network bytes sent.
    pub network_sent: u64,
    /// Network bytes received.
    pub network_recv: u64,
}

/// Plugin cache entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCacheEntry {
    /// Plugin ID.
    pub plugin_id: String,
    /// Plugin version.
    pub version: String,
    /// Cache timestamp.
    pub timestamp: u64,
    /// Cached data.
    pub data: Vec<u8>,
}

/// Plugin message for inter-plugin communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMessage {
    /// Source plugin ID.
    pub from: String,
    /// Target plugin ID (or "broadcast").
    pub to: String,
    /// Message type.
    pub message_type: String,
    /// Message payload.
    pub payload: serde_json::Value,
    /// Message timestamp.
    pub timestamp: u64,
}

impl PluginMessage {
    /// Create a new message.
    pub fn new(from: impl Into<String>, to: impl Into<String>, message_type: impl Into<String>) -> Self {
        PluginMessage {
            from: from.into(),
            to: to.into(),
            message_type: message_type.into(),
            payload: serde_json::Value::Null,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
        }
    }

    /// Set the payload.
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    /// Create a broadcast message.
    pub fn broadcast(from: impl Into<String>, message_type: impl Into<String>) -> Self {
        Self::new(from, "broadcast", message_type)
    }
}

/// Plugin hook result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HookResult {
    /// Continue with normal processing.
    Continue,
    /// Skip normal processing.
    Skip,
    /// Replace the result with custom data.
    Replace(serde_json::Value),
    /// Return an error.
    Error(String),
}

/// Plugin hook types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HookType {
    /// Before media playback starts.
    BeforePlay,
    /// After media playback starts.
    AfterPlay,
    /// Before media playback pauses.
    BeforePause,
    /// After media playback pauses.
    AfterPause,
    /// Before media playback stops.
    BeforeStop,
    /// After media playback stops.
    AfterStop,
    /// Before seeking.
    BeforeSeek,
    /// After seeking.
    AfterSeek,
    /// Before volume change.
    BeforeVolumeChange,
    /// After volume change.
    AfterVolumeChange,
    /// Before loading media.
    BeforeLoad,
    /// After loading media.
    AfterLoad,
    /// Custom hook.
    Custom,
}

/// Plugin capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CapabilityFlags: u32 {
    /// No capabilities.
    const NONE = 0;
    /// Can handle media playback.
    const MEDIA_PLAYBACK = 1 << 0;
    /// Can decode media.
    const MEDIA_DECODING = 1 << 1;
    /// Can extend UI.
    const UI_EXTENSION = 1 << 2;
    /// Can provide metadata.
    const METADATA_PROVIDER = 1 << 3;
    /// Can handle network streams.
    const NETWORK_HANDLER = 1 << 4;
    /// Can handle subtitles.
    const SUBTITLE_HANDLER = 1 << 5;
    /// Can process audio.
    const AUDIO_PROCESSOR = 1 << 6;
    /// Can process video.
    const VIDEO_PROCESSOR = 1 << 7;
    /// Can handle keyboard shortcuts.
    const KEYBOARD_SHORTCUTS = 1 << 8;
    /// Can show notifications.
    const NOTIFICATIONS = 1 << 9;
    /// All capabilities.
    const ALL = 0xFFFF_FFFF;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_priority() {
        assert!(PluginPriority::High > PluginPriority::Normal);
        assert!(PluginPriority::Normal > PluginPriority::Low);
    }

    #[test]
    fn test_plugin_message() {
        let msg = PluginMessage::new("plugin-a", "plugin-b", "test")
            .with_payload(serde_json::json!({ "key": "value" }));

        assert_eq!(msg.from, "plugin-a");
        assert_eq!(msg.to, "plugin-b");
        assert_eq!(msg.message_type, "test");
    }

    #[test]
    fn test_security_context() {
        let ctx = SecurityContext::default();
        assert!(!ctx.filesystem_access);
        assert!(!ctx.network_access);
    }

    #[test]
    fn test_capability_flags() {
        let flags = CapabilityFlags::MEDIA_PLAYBACK | CapabilityFlags::AUDIO_PROCESSOR;
        assert!(flags.contains(CapabilityFlags::MEDIA_PLAYBACK));
        assert!(flags.contains(CapabilityFlags::AUDIO_PROCESSOR));
        assert!(!flags.contains(CapabilityFlags::VIDEO_PROCESSOR));
    }
}