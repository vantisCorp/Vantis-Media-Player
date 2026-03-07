//! DevTools Module
//! 
//! Development tools, debug utilities, and automation helpers.
//! Only compiled in debug builds.

#[cfg(debug_assertions)]
mod debug_overlay;
#[cfg(debug_assertions)]
mod performance_monitor;
#[cfg(debug_assertions)]
mod inspector;

mod keybinds;
mod shortcuts;
mod benchmark;

#[cfg(debug_assertions)]
pub use debug_overlay::DebugOverlay;
#[cfg(debug_assertions)]
pub use performance_monitor::PerformanceMonitor;
#[cfg(debug_assertions)]
pub use inspector::Inspector;

pub use keybinds::KeybindEditor;
pub use shortcuts::{ShortcutManager, Shortcut, ShortcutAction};
pub use benchmark::Benchmark;

/// DevTools configuration
#[derive(Debug, Clone)]
pub struct DevToolsConfig {
    /// Enable debug overlay (F12)
    pub debug_overlay: bool,
    /// Enable performance monitor
    pub performance_monitor: bool,
    /// Enable UI inspector
    pub inspector: bool,
    /// Enable console logging
    pub console_logging: bool,
    /// Log level (0=off, 1=error, 2=warn, 3=info, 4=debug, 5=trace)
    pub log_level: u8,
    /// Enable frame timing display
    pub show_fps: bool,
    /// Enable memory usage display
    pub show_memory: bool,
}

impl Default for DevToolsConfig {
    fn default() -> Self {
        Self {
            debug_overlay: true,
            performance_monitor: true,
            inspector: true,
            console_logging: true,
            log_level: 3,
            show_fps: true,
            show_memory: false,
        }
    }
}

/// DevTools manager
#[derive(Debug)]
pub struct DevTools {
    /// Configuration
    pub config: DevToolsConfig,
    /// Is dev mode enabled
    pub enabled: bool,
    
    #[cfg(debug_assertions)]
    overlay: Option<DebugOverlay>,
    
    #[cfg(debug_assertions)]
    perf_monitor: PerformanceMonitor,
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            config: DevToolsConfig::default(),
            enabled: cfg!(debug_assertions),
            
            #[cfg(debug_assertions)]
            overlay: None,
            
            #[cfg(debug_assertions)]
            perf_monitor: PerformanceMonitor::new(),
        }
    }
}

impl DevTools {
    /// Create new DevTools instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Toggle dev tools
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
    
    /// Show debug overlay
    #[cfg(debug_assertions)]
    pub fn show_overlay(&mut self) {
        self.overlay = Some(DebugOverlay::new());
    }
    
    /// Hide debug overlay
    #[cfg(debug_assertions)]
    pub fn hide_overlay(&mut self) {
        self.overlay = None;
    }
    
    /// Update performance metrics
    pub fn update(&mut self, frame_time_ms: f32) {
        #[cfg(debug_assertions)]
        {
            self.perf_monitor.record_frame(frame_time_ms);
        }
        let _ = frame_time_ms;
    }
    
    /// Get current FPS
    pub fn fps(&self) -> f32 {
        #[cfg(debug_assertions)]
        {
            return self.perf_monitor.fps();
        }
        let _ = &self.config;
        0.0
    }
}

/// Debug log macro helper
#[macro_export]
macro_rules! dev_log {
    ($level:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            match $level {
                1 => tracing::error!($($arg)*),
                2 => tracing::warn!($($arg)*),
                3 => tracing::info!($($arg)*),
                4 => tracing::debug!($($arg)*),
                5 => tracing::trace!($($arg)*),
                _ => {}
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_devtools_default() {
        let devtools = DevTools::new();
        #[cfg(debug_assertions)]
        assert!(devtools.enabled);
        
        #[cfg(not(debug_assertions))]
        assert!(!devtools.enabled);
    }
    
    #[test]
    fn test_devtools_toggle() {
        let mut devtools = DevTools::new();
        let initial = devtools.enabled;
        devtools.toggle();
        assert_eq!(devtools.enabled, !initial);
    }
}