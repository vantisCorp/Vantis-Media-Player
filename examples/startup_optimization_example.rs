//! Startup Optimization Example
//!
//! This example demonstrates the startup optimization features of Vantis Media Player:
//! - Lazy plugin loading
//! - Parallel initialization
//! - Deferred initialization
//! - Configuration caching
//! - Startup time profiling

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use vantiscore::startup_optimization::{
    ConfigLoader, LazyPluginLoader, StartupConfig, StartupOptimizer, StartupPhase,
};

fn main() -> anyhow::Result<()> {
    println!("=== Vantis Media Player - Startup Optimization Example ===\n");

    // Example 1: Basic startup optimization
    println!("Example 1: Basic Startup Optimization");
    basic_startup_optimization()?;

    // Example 2: Lazy plugin loading
    println!("\nExample 2: Lazy Plugin Loading");
    lazy_plugin_loading_example()?;

    // Example 3: Configuration caching
    println!("\nExample 3: Configuration Caching");
    config_caching_example()?;

    // Example 4: Parallel initialization
    println!("\nExample 4: Parallel Initialization");
    parallel_initialization_example()?;

    // Example 5: Deferred initialization
    println!("\nExample 5: Deferred Initialization");
    deferred_initialization_example()?;

    // Example 6: Startup profiling
    println!("\nExample 6: Startup Profiling");
    startup_profiling_example()?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Basic startup optimization
fn basic_startup_optimization() -> anyhow::Result<()> {
    let config = StartupConfig::default();
    let optimizer = StartupOptimizer::new(config);

    // Start the startup process
    optimizer.start();

    // Add some initialization tasks
    for i in 0..3 {
        let task_num = i;
        optimizer.add_init_task(Box::new(move || {
            println!("  Initializing component {}...", task_num);
            std::thread::sleep(Duration::from_millis(10));
            println!("  Component {} initialized", task_num);
            Ok(())
        }));
    }

    // Run initialization tasks
    tokio::runtime::Runtime::new()?
        .block_on(optimizer.run_init_tasks())?;

    // Complete startup
    optimizer.complete();

    // Print startup report
    let report = optimizer.get_report();
    println!("\n{}", report);

    Ok(())
}

/// Example 2: Lazy plugin loading
fn lazy_plugin_loading_example() -> anyhow::Result<()> {
    let load_count = Arc::new(AtomicU32::new(0));

    // Create a lazy plugin loader
    let loader = LazyPluginLoader::new({
        let load_count = Arc::clone(&load_count);
        move |plugin_name| {
            println!("  Loading plugin: {}", plugin_name);
            load_count.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(10));
            Ok(())
        }
    });

    // Load plugins on demand
    println!("  Loading plugin1...");
    loader.load_plugin("plugin1")?;
    println!("  Plugin1 loaded: {}", loader.is_loaded("plugin1"));

    println!("  Loading plugin2...");
    loader.load_plugin("plugin2")?;
    println!("  Plugin2 loaded: {}", loader.is_loaded("plugin2"));

    // Try to load plugin1 again (should use cache)
    println!("  Loading plugin1 again...");
    loader.load_plugin("plugin1")?;
    println!("  Plugin1 still loaded: {}", loader.is_loaded("plugin1"));

    println!("  Total plugin loads: {}", load_count.load(Ordering::SeqCst));

    Ok(())
}

/// Example 3: Configuration caching
fn config_caching_example() -> anyhow::Result<()> {
    let load_count = Arc::new(AtomicU32::new(0));

    // Create a configuration loader
    let loader = ConfigLoader::new({
        let load_count = Arc::clone(&load_count);
        move |config_path| {
            println!("  Loading config from: {}", config_path);
            load_count.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(10));
            Ok(serde_json::json!({
                "path": config_path,
                "loaded": true
            }))
        }
    });

    // Load configurations
    println!("  Loading audio config...");
    let audio_config = loader.load_config("audio.json")?;
    println!("  Audio config: {}", audio_config);

    println!("  Loading video config...");
    let video_config = loader.load_config("video.json")?;
    println!("  Video config: {}", video_config);

    // Load audio config again (should use cache)
    println!("  Loading audio config again...");
    let audio_config_cached = loader.load_config("audio.json")?;
    println!("  Audio config (cached): {}", audio_config_cached);

    println!("  Total config loads: {}", load_count.load(Ordering::SeqCst));

    // Preload multiple configurations
    println!("\n  Preloading configurations...");
    loader.preload_configs(&["ui.json", "subtitles.json", "plugins.json"])?;

    println!("  Total config loads after preload: {}", load_count.load(Ordering::SeqCst));

    Ok(())
}

/// Example 4: Parallel initialization
fn parallel_initialization_example() -> anyhow::Result<()> {
    let config = StartupConfig {
        parallel_initialization: true,
        max_parallel_tasks: 3,
        ..Default::default()
    };

    let optimizer = StartupOptimizer::new(config);
    optimizer.start();

    // Add tasks that take some time
    let task_count = Arc::new(AtomicU32::new(0));
    for i in 0..6 {
        let task_num = i;
        let task_count = Arc::clone(&task_count);
        optimizer.add_init_task(Box::new(move || {
            println!("  Starting task {}...", task_num);
            std::thread::sleep(Duration::from_millis(50));
            task_count.fetch_add(1, Ordering::SeqCst);
            println!("  Task {} completed", task_num);
            Ok(())
        }));
    }

    let start = std::time::Instant::now();
    tokio::runtime::Runtime::new()?
        .block_on(optimizer.run_init_tasks())?;
    let duration = start.elapsed();

    optimizer.complete();

    println!("\n  Tasks completed: {}", task_count.load(Ordering::SeqCst));
    println!("  Total time: {:?}", duration);
    println!("  Expected time with 3 parallel tasks: ~100ms");

    let report = optimizer.get_report();
    println!("\n{}", report);

    Ok(())
}

/// Example 5: Deferred initialization
fn deferred_initialization_example() -> anyhow::Result<()> {
    let config = StartupConfig::default();
    let optimizer = StartupOptimizer::new(config);
    optimizer.start();

    // Add critical initialization tasks
    println!("  Running critical initialization...");
    optimizer.add_init_task(Box::new(|| {
        println!("    Initializing core systems...");
        std::thread::sleep(Duration::from_millis(10));
        Ok(())
    }));

    tokio::runtime::Runtime::new()?
        .block_on(optimizer.run_init_tasks())?;

    // Add deferred tasks
    println!("  Adding deferred tasks...");
    for i in 0..3 {
        let task_num = i;
        optimizer.add_deferred_task(Box::new(move || {
            println!("    Running deferred task {}...", task_num);
            std::thread::sleep(Duration::from_millis(10));
            Ok(())
        }));
    }

    // Run deferred tasks later
    println!("  Running deferred tasks...");
    tokio::runtime::Runtime::new()?
        .block_on(optimizer.run_deferred_tasks())?;

    optimizer.complete();

    let report = optimizer.get_report();
    println!("\n{}", report);

    Ok(())
}

/// Example 6: Startup profiling
fn startup_profiling_example() -> anyhow::Result<()> {
    let config = StartupConfig {
        enable_profiling: true,
        parallel_initialization: true,
        ..Default::default()
    };

    let optimizer = StartupOptimizer::new(config);
    let profiler = optimizer.profiler();

    // Start profiling
    optimizer.start();

    // Simulate different startup phases
    println!("  Phase 1: Configuration loading");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::ConfigLoading);
    }
    std::thread::sleep(Duration::from_millis(20));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::ConfigLoading);
    }

    println!("  Phase 2: Core initialization");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::CoreInit);
    }
    std::thread::sleep(Duration::from_millis(30));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::CoreInit);
    }

    println!("  Phase 3: Plugin loading");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::PluginLoading);
    }
    std::thread::sleep(Duration::from_millis(25));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::PluginLoading);
    }

    println!("  Phase 4: Video engine initialization");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::VideoInit);
    }
    std::thread::sleep(Duration::from_millis(35));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::VideoInit);
    }

    println!("  Phase 5: Audio engine initialization");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::AudioInit);
    }
    std::thread::sleep(Duration::from_millis(20));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::AudioInit);
    }

    println!("  Phase 6: UI initialization");
    {
        let mut p = profiler.lock().unwrap();
        p.start_phase(StartupPhase::UIInit);
    }
    std::thread::sleep(Duration::from_millis(15));
    {
        let mut p = profiler.lock().unwrap();
        p.complete_phase(StartupPhase::UIInit);
    }

    // Complete profiling
    optimizer.complete();

    // Print detailed report
    let report = optimizer.get_report();
    println!("\n{}", report);

    // Calculate phase breakdown
    let profiler = profiler.lock().unwrap();
    if let Some(total) = profiler.total_time() {
        println!("\nPhase Breakdown:");
        for phase in [
            StartupPhase::ConfigLoading,
            StartupPhase::CoreInit,
            StartupPhase::PluginLoading,
            StartupPhase::VideoInit,
            StartupPhase::AudioInit,
            StartupPhase::UIInit,
        ] {
            if let Some(duration) = profiler.phase_duration(phase) {
                let percentage = (duration.as_millis() as f64 / total.as_millis() as f64) * 100.0;
                println!("  {}: {:?} ({:.1}%)", phase.as_str(), duration, percentage);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_startup_optimization() {
        let result = basic_startup_optimization();
        assert!(result.is_ok());
    }

    #[test]
    fn test_lazy_plugin_loading() {
        let result = lazy_plugin_loading_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_caching() {
        let result = config_caching_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parallel_initialization() {
        let result = parallel_initialization_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_deferred_initialization() {
        let result = deferred_initialization_example();
        assert!(result.is_ok());
    }

    #[test]
    fn test_startup_profiling() {
        let result = startup_profiling_example();
        assert!(result.is_ok());
    }
}