//! Startup Optimization Module
//! 
//! This module provides startup time optimization features including:
//! - Lazy plugin loading
//! - Deferred initialization
//! - Parallel initialization tasks
//! - Configuration loading optimization
//! - Startup time profiling

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task::JoinSet;
use tracing::{debug, info, warn};

/// Startup optimization configuration
#[derive(Debug, Clone)]
pub struct StartupConfig {
    /// Enable lazy plugin loading
    pub lazy_plugin_loading: bool,
    
    /// Enable parallel initialization
    pub parallel_initialization: bool,
    
    /// Enable deferred initialization for non-critical components
    pub deferred_initialization: bool,
    
    /// Maximum parallel initialization tasks
    pub max_parallel_tasks: usize,
    
    /// Startup timeout in seconds
    pub startup_timeout: u64,
    
    /// Enable startup profiling
    pub enable_profiling: bool,
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self {
            lazy_plugin_loading: true,
            parallel_initialization: true,
            deferred_initialization: true,
            max_parallel_tasks: 4,
            startup_timeout: 30,
            enable_profiling: true,
        }
    }
}

/// Startup phase for tracking initialization progress
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupPhase {
    /// Configuration loading
    ConfigLoading,
    
    /// Core initialization
    CoreInit,
    
    /// Plugin loading
    PluginLoading,
    
    /// Video engine initialization
    VideoInit,
    
    /// Audio engine initialization
    AudioInit,
    
    /// UI initialization
    UIInit,
    
    /// Deferred initialization
    DeferredInit,
    
    /// Complete
    Complete,
}

impl StartupPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConfigLoading => "Config Loading",
            Self::CoreInit => "Core Initialization",
            Self::PluginLoading => "Plugin Loading",
            Self::VideoInit => "Video Engine Initialization",
            Self::AudioInit => "Audio Engine Initialization",
            Self::UIInit => "UI Initialization",
            Self::DeferredInit => "Deferred Initialization",
            Self::Complete => "Complete",
        }
    }
}

/// Startup timing information for a phase
#[derive(Debug, Clone)]
pub struct PhaseTiming {
    /// Phase name
    pub phase: StartupPhase,
    
    /// Start time
    pub start_time: Instant,
    
    /// End time
    pub end_time: Option<Instant>,
    
    /// Duration
    pub duration: Option<Duration>,
    
    /// Completed
    pub completed: bool,
}

impl PhaseTiming {
    pub fn new(phase: StartupPhase) -> Self {
        Self {
            phase,
            start_time: Instant::now(),
            end_time: None,
            duration: None,
            completed: false,
        }
    }
    
    pub fn complete(&mut self) {
        self.end_time = Some(Instant::now());
        self.duration = Some(self.end_time.unwrap() - self.start_time);
        self.completed = true;
    }
}

/// Startup profiler for measuring initialization time
#[derive(Debug, Clone)]
pub struct StartupProfiler {
    /// Enable profiling
    pub enabled: bool,
    
    /// Phase timings
    pub phases: HashMap<StartupPhase, PhaseTiming>,
    
    /// Overall start time
    pub overall_start: Option<Instant>,
    
    /// Overall end time
    pub overall_end: Option<Instant>,
    
    /// Total duration
    pub total_duration: Option<Duration>,
}

impl StartupProfiler {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            phases: HashMap::new(),
            overall_start: None,
            overall_end: None,
            total_duration: None,
        }
    }
    
    /// Start profiling
    pub fn start(&mut self) {
        if !self.enabled {
            return;
        }
        self.overall_start = Some(Instant::now());
        info!("Startup profiling started");
    }
    
    /// Start a phase
    pub fn start_phase(&mut self, phase: StartupPhase) {
        if !self.enabled {
            return;
        }
        let timing = PhaseTiming::new(phase);
        self.phases.insert(phase, timing);
        debug!("Started phase: {}", phase.as_str());
    }
    
    /// Complete a phase
    pub fn complete_phase(&mut self, phase: StartupPhase) {
        if !self.enabled {
            return;
        }
        if let Some(timing) = self.phases.get_mut(&phase) {
            timing.complete();
            if let Some(duration) = timing.duration {
                debug!("Completed phase: {} in {:?}", phase.as_str(), duration);
            }
        }
    }
    
    /// Complete profiling
    pub fn complete(&mut self) {
        if !self.enabled {
            return;
        }
        self.overall_end = Some(Instant::now());
        self.total_duration = Some(self.overall_end.unwrap() - self.overall_start.unwrap());
        info!("Startup profiling completed");
    }
    
    /// Get total startup time
    pub fn total_time(&self) -> Option<Duration> {
        self.total_duration
    }
    
    /// Get phase duration
    pub fn phase_duration(&self, phase: StartupPhase) -> Option<Duration> {
        self.phases.get(&phase).and_then(|t| t.duration)
    }
    
    /// Get startup report
    pub fn report(&self) -> String {
        if !self.enabled {
            return "Profiling disabled".to_string();
        }
        
        let mut report = String::new();
        report.push_str("=== Startup Time Report ===\n\n");
        
        if let Some(total) = self.total_duration {
            report.push_str(&format!("Total Startup Time: {:?}\n\n", total));
        }
        
        report.push_str("Phase Breakdown:\n");
        for phase in [
            StartupPhase::ConfigLoading,
            StartupPhase::CoreInit,
            StartupPhase::PluginLoading,
            StartupPhase::VideoInit,
            StartupPhase::AudioInit,
            StartupPhase::UIInit,
            StartupPhase::DeferredInit,
        ] {
            if let Some(timing) = self.phases.get(&phase) {
                if let Some(duration) = timing.duration {
                    report.push_str(&format!(
                        "  {}: {:?}\n",
                        phase.as_str(),
                        duration
                    ));
                }
            }
        }
        
        report
    }
}

/// Initialization task for parallel execution
pub type InitTask = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

/// Startup optimizer for managing initialization process
pub struct StartupOptimizer {
    /// Configuration
    config: StartupConfig,
    
    /// Profiler
    profiler: Arc<Mutex<StartupProfiler>>,
    
    /// Initialization tasks
    init_tasks: Arc<Mutex<Vec<InitTask>>>,
    
    /// Deferred tasks
    deferred_tasks: Arc<Mutex<Vec<InitTask>>>,
    
    /// Lazy loaded plugins
    lazy_plugins: Arc<Mutex<HashMap<String, bool>>>,
}

impl StartupOptimizer {
    /// Create a new startup optimizer
    pub fn new(config: StartupConfig) -> Self {
        let profiler = Arc::new(Mutex::new(StartupProfiler::new(config.enable_profiling)));
        
        Self {
            config,
            profiler,
            init_tasks: Arc::new(Mutex::new(Vec::new())),
            deferred_tasks: Arc::new(Mutex::new(Vec::new())),
            lazy_plugins: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Start the startup process
    pub fn start(&self) {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.start();
    }
    
    /// Add an initialization task
    pub fn add_init_task(&self, task: InitTask) {
        let mut tasks = self.init_tasks.lock().unwrap();
        tasks.push(task);
    }
    
    /// Add a deferred initialization task
    pub fn add_deferred_task(&self, task: InitTask) {
        let mut tasks = self.deferred_tasks.lock().unwrap();
        tasks.push(task);
    }
    
    /// Register a plugin for lazy loading
    pub fn register_lazy_plugin(&self, plugin_name: String) {
        let mut plugins = self.lazy_plugins.lock().unwrap();
        plugins.insert(plugin_name, false);
    }
    
    /// Check if a plugin is loaded
    pub fn is_plugin_loaded(&self, plugin_name: &str) -> bool {
        let plugins = self.lazy_plugins.lock().unwrap();
        plugins.get(plugin_name).copied().unwrap_or(false)
    }
    
    /// Mark a plugin as loaded
    pub fn mark_plugin_loaded(&self, plugin_name: &str) {
        let mut plugins = self.lazy_plugins.lock().unwrap();
        plugins.insert(plugin_name.to_string(), true);
    }
    
    /// Run initialization tasks
    pub async fn run_init_tasks(&self) -> Result<()> {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.start_phase(StartupPhase::CoreInit);
        drop(profiler);
        
        let tasks = {
            let mut task_list = self.init_tasks.lock().unwrap();
            std::mem::take(&mut *task_list)
        };
        
        if self.config.parallel_initialization && tasks.len() > 1 {
            self.run_parallel_tasks(tasks).await?;
        } else {
            self.run_sequential_tasks(tasks).await?;
        }
        
        let mut profiler = self.profiler.lock().unwrap();
        profiler.complete_phase(StartupPhase::CoreInit);
        
        Ok(())
    }
    
    /// Run tasks in parallel
    async fn run_parallel_tasks(&self, tasks: Vec<InitTask>) -> Result<()> {
        let max_tasks = self.config.max_parallel_tasks.min(tasks.len());
        let mut join_set = JoinSet::new();
        let mut task_iter = tasks.into_iter();
        
        // Spawn initial tasks
        for _ in 0..max_tasks {
            if let Some(task) = task_iter.next() {
                join_set.spawn(async move {
                    task().map_err(|e| {
                        warn!("Initialization task failed: {:?}", e);
                        e
                    })
                });
            }
        }
        
        // Process tasks as they complete
        while let Some(result) = join_set.join_next().await {
            result??;
            
            // Spawn next task if available
            if let Some(task) = task_iter.next() {
                join_set.spawn(async move {
                    task().map_err(|e| {
                        warn!("Initialization task failed: {:?}", e);
                        e
                    })
                });
            }
        }
        
        Ok(())
    }
    
    /// Run tasks sequentially
    async fn run_sequential_tasks(&self, tasks: Vec<InitTask>) -> Result<()> {
        for task in tasks {
            task()?;
        }
        Ok(())
    }
    
    /// Run deferred initialization tasks
    pub async fn run_deferred_tasks(&self) -> Result<()> {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.start_phase(StartupPhase::DeferredInit);
        drop(profiler);
        
        let tasks = {
            let mut task_list = self.deferred_tasks.lock().unwrap();
            std::mem::take(&mut *task_list)
        };
        
        for task in tasks {
            if let Err(e) = task() {
                warn!("Deferred task failed: {:?}", e);
            }
        }
        
        let mut profiler = self.profiler.lock().unwrap();
        profiler.complete_phase(StartupPhase::DeferredInit);
        
        Ok(())
    }
    
    /// Complete startup
    pub fn complete(&self) {
        let mut profiler = self.profiler.lock().unwrap();
        profiler.complete();
        profiler.start_phase(StartupPhase::Complete);
        profiler.complete_phase(StartupPhase::Complete);
        
        if let Some(report) = profiler.total_time() {
            info!("Startup completed in {:?}", report);
        }
    }
    
    /// Get startup report
    pub fn get_report(&self) -> String {
        let profiler = self.profiler.lock().unwrap();
        profiler.report()
    }
    
    /// Get profiler reference
    pub fn profiler(&self) -> Arc<Mutex<StartupProfiler>> {
        Arc::clone(&self.profiler)
    }
}

/// Lazy loader for plugins
pub struct LazyPluginLoader {
    /// Loaded plugins
    loaded: Arc<Mutex<HashMap<String, bool>>>,
    
    /// Load function
    load_fn: Arc<dyn Fn(&str) -> Result<()> + Send + Sync>,
}

impl LazyPluginLoader {
    /// Create a new lazy plugin loader
    pub fn new<F>(load_fn: F) -> Self
    where
        F: Fn(&str) -> Result<()> + Send + Sync + 'static,
    {
        Self {
            loaded: Arc::new(Mutex::new(HashMap::new())),
            load_fn: Arc::new(load_fn),
        }
    }
    
    /// Load a plugin if not already loaded
    pub fn load_plugin(&self, plugin_name: &str) -> Result<()> {
        {
            let loaded = self.loaded.lock().unwrap();
            if loaded.get(plugin_name).copied().unwrap_or(false) {
                debug!("Plugin {} already loaded", plugin_name);
                return Ok(());
            }
        }
        
        debug!("Lazy loading plugin: {}", plugin_name);
        (self.load_fn)(plugin_name)?;
        
        let mut loaded = self.loaded.lock().unwrap();
        loaded.insert(plugin_name.to_string(), true);
        
        info!("Plugin {} loaded successfully", plugin_name);
        Ok(())
    }
    
    /// Check if a plugin is loaded
    pub fn is_loaded(&self, plugin_name: &str) -> bool {
        let loaded = self.loaded.lock().unwrap();
        loaded.get(plugin_name).copied().unwrap_or(false)
    }
}

/// Configuration loader with caching
pub struct ConfigLoader {
    /// Cache for loaded configurations
    cache: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    
    /// Load function
    load_fn: Arc<dyn Fn(&str) -> Result<serde_json::Value> + Send + Sync>,
}

impl ConfigLoader {
    /// Create a new configuration loader
    pub fn new<F>(load_fn: F) -> Self
    where
        F: Fn(&str) -> Result<serde_json::Value> + Send + Sync + 'static,
    {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            load_fn: Arc::new(load_fn),
        }
    }
    
    /// Load configuration with caching
    pub fn load_config(&self, config_path: &str) -> Result<serde_json::Value> {
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(value) = cache.get(config_path) {
                debug!("Configuration {} loaded from cache", config_path);
                return Ok(value.clone());
            }
        }
        
        // Load from source
        debug!("Loading configuration from: {}", config_path);
        let value = (self.load_fn)(config_path)?;
        
        // Cache the result
        let mut cache = self.cache.lock().unwrap();
        cache.insert(config_path.to_string(), value.clone());
        
        info!("Configuration {} loaded and cached", config_path);
        Ok(value)
    }
    
    /// Clear cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        debug!("Configuration cache cleared");
    }
    
    /// Preload configurations
    pub fn preload_configs(&self, config_paths: &[&str]) -> Result<()> {
        info!("Preloading {} configurations", config_paths.len());
        
        for path in config_paths {
            self.load_config(path)?;
        }
        
        info!("Configuration preloading complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_startup_config_default() {
        let config = StartupConfig::default();
        assert!(config.lazy_plugin_loading);
        assert!(config.parallel_initialization);
        assert!(config.deferred_initialization);
        assert_eq!(config.max_parallel_tasks, 4);
        assert_eq!(config.startup_timeout, 30);
        assert!(config.enable_profiling);
    }

    #[test]
    fn test_phase_timing() {
        let mut timing = PhaseTiming::new(StartupPhase::ConfigLoading);
        assert!(!timing.completed);
        assert!(timing.duration.is_none());
        
        std::thread::sleep(Duration::from_millis(10));
        timing.complete();
        
        assert!(timing.completed);
        assert!(timing.duration.is_some());
        assert!(timing.duration.unwrap() >= Duration::from_millis(10));
    }

    #[test]
    fn test_startup_profiler() {
        let mut profiler = StartupProfiler::new(true);
        profiler.start();
        
        profiler.start_phase(StartupPhase::ConfigLoading);
        std::thread::sleep(Duration::from_millis(10));
        profiler.complete_phase(StartupPhase::ConfigLoading);
        
        profiler.start_phase(StartupPhase::CoreInit);
        std::thread::sleep(Duration::from_millis(10));
        profiler.complete_phase(StartupPhase::CoreInit);
        
        profiler.complete();
        
        assert!(profiler.total_time().is_some());
        assert!(profiler.phase_duration(StartupPhase::ConfigLoading).is_some());
        assert!(profiler.phase_duration(StartupPhase::CoreInit).is_some());
        
        let report = profiler.report();
        assert!(report.contains("Startup Time Report"));
        assert!(report.contains("Config Loading"));
        assert!(report.contains("Core Initialization"));
    }

    #[test]
    fn test_startup_optimizer() {
        let config = StartupConfig::default();
        let optimizer = StartupOptimizer::new(config);
        
        optimizer.start();
        
        // Add some init tasks
        let counter = Arc::new(AtomicU32::new(0));
        for i in 0..5 {
            let counter = Arc::clone(&counter);
            optimizer.add_init_task(Box::new(move || {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }));
        }
        
        // Run tasks
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(optimizer.run_init_tasks())
            .unwrap();
        
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        
        optimizer.complete();
        
        let report = optimizer.get_report();
        assert!(report.contains("Startup Time Report"));
    }

    #[test]
    fn test_lazy_plugin_loader() {
        let load_count = Arc::new(AtomicU32::new(0));
        let load_count_clone = Arc::clone(&load_count);
        
        let loader = LazyPluginLoader::new(move |plugin_name| {
            load_count_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
        
        // Load plugin first time
        loader.load_plugin("test_plugin").unwrap();
        assert_eq!(load_count.load(Ordering::SeqCst), 1);
        assert!(loader.is_loaded("test_plugin"));
        
        // Load plugin second time (should use cache)
        loader.load_plugin("test_plugin").unwrap();
        assert_eq!(load_count.load(Ordering::SeqCst), 1); // Should not increment
    }

    #[test]
    fn test_config_loader() {
        let load_count = Arc::new(AtomicU32::new(0));
        let load_count_clone = Arc::clone(&load_count);
        
        let loader = ConfigLoader::new(move |_path| {
            load_count_clone.fetch_add(1, Ordering::SeqCst);
            Ok(serde_json::json!({"test": "value"}))
        });
        
        // Load config first time
        let config = loader.load_config("test_config.json").unwrap();
        assert_eq!(load_count.load(Ordering::SeqCst), 1);
        assert_eq!(config["test"], "value");
        
        // Load config second time (should use cache)
        let config = loader.load_config("test_config.json").unwrap();
        assert_eq!(load_count.load(Ordering::SeqCst), 1); // Should not increment
        assert_eq!(config["test"], "value");
        
        // Clear cache and load again
        loader.clear_cache();
        let config = loader.load_config("test_config.json").unwrap();
        assert_eq!(load_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_deferred_tasks() {
        let config = StartupConfig::default();
        let optimizer = StartupOptimizer::new(config);
        
        optimizer.start();
        
        // Add deferred tasks
        let counter = Arc::new(AtomicU32::new(0));
        for _ in 0..3 {
            let counter = Arc::clone(&counter);
            optimizer.add_deferred_task(Box::new(move || {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }));
        }
        
        // Run deferred tasks
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(optimizer.run_deferred_tasks())
            .unwrap();
        
        assert_eq!(counter.load(Ordering::SeqCst), 3);
        
        optimizer.complete();
    }

    #[test]
    fn test_parallel_initialization() {
        let config = StartupConfig {
            parallel_initialization: true,
            max_parallel_tasks: 2,
            ..Default::default()
        };
        
        let optimizer = StartupOptimizer::new(config);
        optimizer.start();
        
        // Add tasks that take some time
        let counter = Arc::new(AtomicU32::new(0));
        for _ in 0..4 {
            let counter = Arc::clone(&counter);
            optimizer.add_init_task(Box::new(move || {
                std::thread::sleep(Duration::from_millis(10));
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }));
        }
        
        let start = Instant::now();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(optimizer.run_init_tasks())
            .unwrap();
        let duration = start.elapsed();
        
        assert_eq!(counter.load(Ordering::SeqCst), 4);
        // With 2 parallel tasks and 4 tasks of 10ms each, should take ~20ms
        assert!(duration < Duration::from_millis(30));
        
        optimizer.complete();
    }
}