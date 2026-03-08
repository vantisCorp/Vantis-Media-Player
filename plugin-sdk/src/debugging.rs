//! Plugin Debugging Tools
//!
//! Comprehensive debugging tools for Vantis plugin development:
//! - Plugin state inspection
//! - Message logging and tracing
//! - Breakpoint support
//! - Memory inspection
//! - Performance profiling per-plugin

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;

// ============================================================================
// Debug Log Types
// ============================================================================

/// Log level for plugin debug messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebugLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for DebugLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

impl PartialOrd for DebugLogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DebugLogLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use DebugLogLevel::*;
        let order = |l: &Self| match l {
            Trace => 0,
            Debug => 1,
            Info => 2,
            Warn => 3,
            Error => 4,
        };
        order(self).cmp(&order(other))
    }
}

/// A debug log message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLogMessage {
    pub plugin_id: String,
    pub level: DebugLogLevel,
    pub message: String,
    pub timestamp_ms: u64,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub context: HashMap<String, String>,
}

impl DebugLogMessage {
    pub fn new(plugin_id: &str, level: DebugLogLevel, message: &str) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            level,
            message: message.to_string(),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            file: None,
            line: None,
            context: HashMap::new(),
        }
    }
    
    pub fn with_location(mut self, file: &str, line: u32) -> Self {
        self.file = Some(file.to_string());
        self.line = Some(line);
        self
    }
    
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }
}

// ============================================================================
// Breakpoint Support
// ============================================================================

/// A breakpoint in plugin code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: u32,
    pub plugin_id: String,
    pub file: String,
    pub line: u32,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

impl Breakpoint {
    pub fn new(id: u32, plugin_id: &str, file: &str, line: u32) -> Self {
        Self {
            id,
            plugin_id: plugin_id.to_string(),
            file: file.to_string(),
            line,
            condition: None,
            hit_count: 0,
            enabled: true,
        }
    }
    
    pub fn with_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }
}

/// Breakpoint hit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointHit {
    pub breakpoint: Breakpoint,
    pub state: PluginState,
    pub call_stack: Vec<StackFrame>,
    pub locals: HashMap<String, String>,
    pub timestamp_ms: u64,
}

// ============================================================================
// Plugin State Inspection
// ============================================================================

/// Snapshot of plugin state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub status: PluginStatus,
    pub memory_used: u64,
    pub cpu_time_ms: u64,
    pub messages_processed: u64,
    pub errors_count: u64,
    pub last_activity_ms: u64,
    pub exported_functions: Vec<String>,
    pub memory_regions: Vec<MemoryRegion>,
    pub call_stack: Vec<StackFrame>,
}

/// Plugin status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginStatus {
    Loading,
    Running,
    Paused,
    Error,
    Unloaded,
}

/// Stack frame information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub locals: HashMap<String, String>,
}

/// Memory region in plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub read_only: bool,
    pub contents: Option<Vec<u8>>,
}

// ============================================================================
// Debug Controller
// ============================================================================

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub max_logs_per_plugin: usize,
    pub auto_capture_state: bool,
    pub min_log_level: DebugLogLevel,
    pub track_performance: bool,
    pub break_on_exceptions: bool,
    pub break_on_unhandled: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            max_logs_per_plugin: 1000,
            auto_capture_state: true,
            min_log_level: DebugLogLevel::Debug,
            track_performance: true,
            break_on_exceptions: true,
            break_on_unhandled: false,
        }
    }
}

/// Controller for plugin debugging
pub struct PluginDebugController {
    breakpoints: Arc<RwLock<HashMap<u32, Breakpoint>>>,
    logs: Arc<RwLock<HashMap<String, VecDeque<DebugLogMessage>>>>,
    plugin_states: Arc<RwLock<HashMap<String, PluginState>>>,
    config: DebugConfig,
    next_breakpoint_id: Arc<RwLock<u32>>,
}

impl PluginDebugController {
    pub fn new(config: Option<DebugConfig>) -> Self {
        Self {
            breakpoints: Arc::new(RwLock::new(HashMap::new())),
            logs: Arc::new(RwLock::new(HashMap::new())),
            plugin_states: Arc::new(RwLock::new(HashMap::new())),
            config: config.unwrap_or_default(),
            next_breakpoint_id: Arc::new(RwLock::new(1)),
        }
    }
    
    /// Log a message from a plugin
    pub fn log(&self, message: DebugLogMessage) {
        if message.level < self.config.min_log_level {
            return;
        }
        
        let mut logs = self.logs.write();
        let plugin_logs = logs.entry(message.plugin_id.clone()).or_default();
        
        plugin_logs.push_back(message);
        
        while plugin_logs.len() > self.config.max_logs_per_plugin {
            plugin_logs.pop_front();
        }
    }
    
    /// Get logs for a plugin
    pub fn get_logs(&self, plugin_id: &str) -> Vec<DebugLogMessage> {
        self.logs.read()
            .get(plugin_id)
            .map(|l| l.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Add a breakpoint
    pub fn add_breakpoint(&self, plugin_id: &str, file: &str, line: u32) -> Breakpoint {
        let id = {
            let mut next_id = self.next_breakpoint_id.write();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let breakpoint = Breakpoint::new(id, plugin_id, file, line);
        self.breakpoints.write().insert(id, breakpoint.clone());
        breakpoint
    }
    
    /// Remove a breakpoint
    pub fn remove_breakpoint(&self, id: u32) -> Option<Breakpoint> {
        self.breakpoints.write().remove(&id)
    }
    
    /// Get all breakpoints for a plugin
    pub fn get_breakpoints(&self, plugin_id: &str) -> Vec<Breakpoint> {
        self.breakpoints.read()
            .values()
            .filter(|bp| bp.plugin_id == plugin_id)
            .cloned()
            .collect()
    }
    
    /// Check if a location has a breakpoint
    pub fn check_breakpoint(&self, plugin_id: &str, file: &str, line: u32) -> Option<Breakpoint> {
        self.breakpoints.read()
            .values()
            .find(|bp| {
                bp.plugin_id == plugin_id &&
                bp.file == file &&
                bp.line == line &&
                bp.enabled
            })
            .cloned()
    }
    
    /// Update plugin state
    pub fn update_state(&self, state: PluginState) {
        self.plugin_states.write().insert(state.plugin_id.clone(), state);
    }
    
    /// Get plugin state
    pub fn get_state(&self, plugin_id: &str) -> Option<PluginState> {
        self.plugin_states.read().get(plugin_id).cloned()
    }
    
    /// Get all plugin states
    pub fn get_all_states(&self) -> Vec<PluginState> {
        self.plugin_states.read().values().cloned().collect()
    }
}

/// A debug session snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSession {
    pub id: String,
    pub created_at: String,
    pub breakpoints: Vec<Breakpoint>,
    pub plugin_states: Vec<PluginState>,
    pub recent_logs: Vec<DebugLogMessage>,
}

/// Trait for plugin debug implementations
pub trait PluginDebugger: Send + Sync {
    fn on_breakpoint(&self, hit: &BreakpointHit);
    fn on_log(&self, message: &DebugLogMessage);
    fn on_state_change(&self, plugin_id: &str, old_status: PluginStatus, new_status: PluginStatus);
    fn on_exception(&self, plugin_id: &str, exception: &str);
}

/// Logger for plugin debugging
pub struct PluginDebugLogger {
    controller: Arc<PluginDebugController>,
    plugin_id: String,
}

impl PluginDebugLogger {
    pub fn new(controller: Arc<PluginDebugController>, plugin_id: &str) -> Self {
        Self {
            controller,
            plugin_id: plugin_id.to_string(),
        }
    }
    
    pub fn trace(&self, message: &str) {
        self.controller.log(DebugLogMessage::new(&self.plugin_id, DebugLogLevel::Trace, message));
    }
    
    pub fn debug(&self, message: &str) {
        self.controller.log(DebugLogMessage::new(&self.plugin_id, DebugLogLevel::Debug, message));
    }
    
    pub fn info(&self, message: &str) {
        self.controller.log(DebugLogMessage::new(&self.plugin_id, DebugLogLevel::Info, message));
    }
    
    pub fn warn(&self, message: &str) {
        self.controller.log(DebugLogMessage::new(&self.plugin_id, DebugLogLevel::Warn, message));
    }
    
    pub fn error(&self, message: &str) {
        self.controller.log(DebugLogMessage::new(&self.plugin_id, DebugLogLevel::Error, message));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_debug_log_message() {
        let msg = DebugLogMessage::new("test-plugin", DebugLogLevel::Info, "Test message");
        assert_eq!(msg.plugin_id, "test-plugin");
        assert_eq!(msg.level, DebugLogLevel::Info);
    }
    
    #[test]
    fn test_breakpoint() {
        let bp = Breakpoint::new(1, "test-plugin", "main.rs", 42);
        assert_eq!(bp.id, 1);
        assert_eq!(bp.plugin_id, "test-plugin");
        assert!(bp.enabled);
    }
    
    #[test]
    fn test_debug_controller_logging() {
        let controller = PluginDebugController::new(None);
        
        controller.log(DebugLogMessage::new("plugin1", DebugLogLevel::Info, "Message 1"));
        controller.log(DebugLogMessage::new("plugin1", DebugLogLevel::Debug, "Message 2"));
        
        let logs = controller.get_logs("plugin1");
        assert_eq!(logs.len(), 2);
    }
    
    #[test]
    fn test_debug_controller_breakpoints() {
        let controller = PluginDebugController::new(None);
        
        let bp = controller.add_breakpoint("plugin1", "main.rs", 10);
        assert_eq!(bp.id, 1);
        
        let breakpoints = controller.get_breakpoints("plugin1");
        assert_eq!(breakpoints.len(), 1);
        
        let found = controller.check_breakpoint("plugin1", "main.rs", 10);
        assert!(found.is_some());
    }
}