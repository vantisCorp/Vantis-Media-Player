# Advanced Plugin System - Features Guide

## Overview

The Advanced Plugin System extends the basic WASM plugin system with enterprise-grade features including a plugin marketplace, dependency management, enhanced sandbox with fine-grained permissions, hot-reload with state preservation, performance monitoring, and comprehensive lifecycle management.

## Table of Contents

1. [Plugin Marketplace](#plugin-marketplace)
2. [Dependency Management](#dependency-management)
3. [Enhanced Sandbox](#enhanced-sandbox)
4. [Hot Reload](#hot-reload)
5. [Performance Monitoring](#performance-monitoring)
6. [Lifecycle Management](#lifecycle-management)
7. [Configuration](#configuration)
8. [Usage Examples](#usage-examples)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Plugin Marketplace

The plugin marketplace provides a centralized repository for discovering, searching, installing, and updating plugins.

### Features

- **Search**: Search plugins by name, category, rating, and popularity
- **Categories**: Audio, Video, Subtitles, UI, Integration, Utility, Theme, Other
- **Installation**: Download and install plugins with automatic checksum verification
- **Updates**: Check for and install plugin updates automatically
- **Reviews**: Read and submit plugin reviews
- **Featured**: Browse featured and popular plugins

### Usage

```rust
use vantisplayer::advanced_plugins::AdvancedPluginManager;
use vantisplayer::advanced_plugins::marketplace::{SearchQuery, SortBy, SortOrder};

// Create manager
let manager = AdvancedPluginManager::new(
    PathBuf::from("/path/to/plugins"),
    PathBuf::from("/path/to/cache"),
)?;

// Search for plugins
let query = SearchQuery {
    query: "audio".to_string(),
    category: Some(PluginCategory::Audio),
    min_rating: Some(4.0),
    sort_by: SortBy::Rating,
    sort_order: SortOrder::Desc,
    limit: Some(20),
};

let results = manager.marketplace().search(&query).await?;

// Get plugin info
let plugin = manager.marketplace().get_plugin("plugin_id").await?;

// Download plugin
let dest = PathBuf::from("/path/to/plugins/plugin.wasm");
manager.marketplace().download_plugin("plugin_id", &dest).await?;

// Get featured plugins
let featured = manager.marketplace().get_featured().await?;

// Get popular plugins
let popular = manager.marketplace().get_popular(10).await?;

// Check for updates
let updates = manager.marketplace().check_updates().await?;
```

### Plugin Categories

- **Audio**: Audio processing, effects, visualization
- **Video**: Video processing, effects, filters
- **Subtitles**: Subtitle parsing, synchronization, translation
- **UI**: User interface components, themes
- **Integration**: External service integrations
- **Utility**: Helper functions, utilities
- **Theme**: Visual themes, skins
- **Other**: Miscellaneous plugins

### Plugin Permissions

Plugins can request the following permissions:

- `FileSystem`: File system access
- `Network`: Network access
- `MediaControl`: Media playback control
- `ConfigAccess`: Configuration access
- `Logging`: Logging access
- `Custom(String)`: Custom permissions

---

## Dependency Management

The dependency manager handles plugin dependencies including resolution, installation, and version management using semantic versioning.

### Features

- **Resolution**: Automatically resolve dependency conflicts
- **Installation**: Download and install dependencies
- **Updates**: Check for and install dependency updates
- **Version Management**: Semantic versioning support
- **Registry**: Centralized dependency registry

### Usage

```rust
use vantisplayer::advanced_plugins::dependencies::{DependencyRequirement, VersionReq};
use semver::Version;

// Define requirements
let requirements = vec![
    DependencyRequirement {
        name: "dep1".to_string(),
        version_req: VersionReq::parse(">=1.0.0")?,
        optional: false,
    },
    DependencyRequirement {
        name: "dep2".to_string(),
        version_req: VersionReq::parse("^2.0.0")?,
        optional: true,
    },
];

// Resolve dependencies
let result = manager.dependency_manager().resolve(&requirements).await?;

// Check for conflicts
if !result.conflicts.is_empty() {
    for conflict in result.conflicts {
        eprintln!("Conflict: {}", conflict.name);
    }
}

// Install dependencies
for dep in result.dependencies {
    let version = Version::parse("1.0.0")?;
    manager.dependency_manager().install(&dep.name, &version).await?;
}

// List installed dependencies
let installed = manager.dependency_manager().list_installed().await;

// Update all dependencies
let updated = manager.dependency_manager().update_all().await?;

// Check for updates
let updates = manager.dependency_manager().check_updates().await?;
```

### Semantic Versioning

The dependency manager uses semantic versioning (semver) for version requirements:

- `^1.2.3`: Compatible with 1.2.3 and above, but less than 2.0.0
- `~1.2.3`: Compatible with 1.2.3 and above, but less than 1.3.0
- `>=1.2.3`: Version 1.2.3 or higher
- `>1.2.3`: Version greater than 1.2.3
- `1.2.3`: Exact version 1.2.3

---

## Enhanced Sandbox

The enhanced sandbox provides fine-grained permission control and resource limits for plugin execution.

### Features

- **Permission System**: Granular permission control
- **Resource Limits**: Memory, CPU, file descriptor limits
- **Fuel Consumption**: Execution time limits
- **Permission Policies**: Define allowed and denied permissions per plugin

### Usage

```rust
use vantisplayer::advanced_plugins::sandbox::{PluginPermission, PermissionPolicy, ResourceLimits};

// Create permission policy
let policy = PermissionPolicy {
    plugin_name: "test_plugin".to_string(),
    allowed: vec![
        PluginPermission::Logging,
        PluginPermission::FileSystemRead,
        PluginPermission::MediaControl,
    ],
    denied: vec![
        PluginPermission::Network,
        PluginPermission::FileSystemWrite,
    ],
    limits: ResourceLimits {
        max_memory: 256 * 1024 * 1024, // 256 MB
        max_cpu_time: 30,
        max_fds: 16,
        max_connections: 5,
        max_execution_time: 15,
    },
};

// Add policy to sandbox
manager.sandbox().add_policy(policy);

// Load plugin with permissions
let permissions = vec![
    PluginPermission::Logging,
    PluginPermission::FileSystemRead,
];

let wasm_bytes = std::fs::read("plugin.wasm")?;
manager.sandbox().load_plugin("test_plugin", &wasm_bytes, permissions).await?;

// Check permission
if manager.sandbox().has_permission("test_plugin", &PluginPermission::Network) {
    // Allow network access
}

// Get instance info
if let Some(instance) = manager.sandbox().get_instance("test_plugin") {
    println!("Memory usage: {} bytes", instance.memory_usage);
    println!("CPU time: {:?}", instance.cpu_time);
}

// Cleanup inactive instances
let cleaned = manager.sandbox().cleanup_inactive(Duration::from_secs(300)).await?;
```

### Permissions

- `FileSystemRead`: Read files from file system
- `FileSystemWrite`: Write files to file system
- `Network`: Make network requests
- `MediaControl`: Control media playback
- `ConfigRead`: Read configuration
- `ConfigWrite`: Write configuration
- `Logging`: Write logs
- `Custom(String)`: Custom permission

### Resource Limits

- `max_memory`: Maximum memory in bytes (default: 512 MB)
- `max_cpu_time`: Maximum CPU time in seconds (default: 60)
- `max_fds`: Maximum file descriptors (default: 32)
- `max_connections`: Maximum network connections (default: 10)
- `max_execution_time`: Maximum execution time in seconds (default: 30)

---

## Hot Reload

The hot reload manager provides automatic plugin reloading with state preservation and minimal downtime.

### Features

- **File Watching**: Automatic detection of plugin changes
- **State Preservation**: Save and restore plugin state on reload
- **Event System**: Subscribe to reload events
- **Manual Trigger**: Manually trigger reload for specific plugins
- **Debouncing**: Prevent excessive reloads

### Usage

```rust
use vantisplayer::advanced_plugins::hot_reload::{ReloadConfig, PluginState};

// Start hot reload
manager.start().await?;

// Subscribe to reload events
let mut receiver = manager.hot_reload().subscribe()?;

tokio::spawn(async move {
    while let Some(event) = receiver.recv().await {
        match event {
            ReloadEvent::Reloaded { name, old_version, new_version } => {
                println!("Plugin {} reloaded: {} -> {}", name, old_version, new_version);
            }
            ReloadEvent::Failed { name, error } => {
                eprintln!("Failed to reload {}: {}", name, error);
            }
            ReloadEvent::Added { name } => {
                println!("Plugin added: {}", name);
            }
            ReloadEvent::Removed { name } => {
                println!("Plugin removed: {}", name);
            }
        }
    }
});

// Save plugin state before reload
let state = PluginState {
    name: "test_plugin".to_string(),
    version: "1.0.0".to_string(),
    config: serde_json::json!({"key": "value"}),
    data: HashMap::new(),
    last_reload: chrono::Utc::now(),
};

manager.hot_reload().save_state("test_plugin", state);

// Load plugin state after reload
if let Some(state) = manager.hot_reload().load_state("test_plugin") {
    println!("Restored state: {:?}", state.config);
}

// Manually trigger reload
manager.hot_reload().trigger_reload("test_plugin").await?;

// Stop hot reload
manager.stop().await?;
```

### Reload Configuration

```rust
let config = ReloadConfig {
    enabled: true,
    interval: 5,              // Check every 5 seconds
    debounce: 500,            // Wait 500ms before reloading
    preserve_state: true,     // Save and restore state
    auto_restart: true,       // Restart on error
    max_retries: 3,           // Maximum retry attempts
};
```

---

## Performance Monitoring

The performance monitor provides metrics, profiling, and health monitoring for plugins.

### Features

- **Execution Metrics**: Track execution count, time, and errors
- **Resource Metrics**: Monitor memory and CPU usage
- **Prometheus Integration**: Export metrics in Prometheus format
- **Snapshots**: Take performance snapshots
- **Reports**: Generate performance reports

### Usage

```rust
use vantisplayer::advanced_plugins::monitoring::PerformanceMonitor;

// Start monitoring
manager.start().await?;

// Record execution
let start = std::time::Instant::now();
// ... execute plugin ...
let duration = start.elapsed();
manager.monitor().record_execution("test_plugin", duration);

// Record error
manager.monitor().record_error("test_plugin");

// Update metrics
manager.monitor().update_memory("test_plugin", 1024 * 1024); // 1 MB
manager.monitor().update_cpu("test_plugin", 45.5); // 45.5%

// Get plugin metrics
if let Some(metrics) = manager.monitor().get_metrics("test_plugin") {
    println!("Executions: {}", metrics.execution_count);
    println!("Avg time: {:?}", metrics.average_execution_time);
    println!("Memory: {} MB", metrics.memory_usage / 1024 / 1024);
    println!("CPU: {:.2}%", metrics.cpu_usage);
    println!("Errors: {}", metrics.error_count);
}

// Take snapshot
let snapshot = manager.monitor().take_snapshot();
println!("Active plugins: {}", snapshot.system.active_plugins);
println!("Total executions: {}", snapshot.system.total_executions);

// Export Prometheus metrics
let prometheus = manager.monitor().export_prometheus()?;
println!("{}", prometheus);

// Get performance report
let report = manager.monitor().get_report();
println!("{}", report);

// Reset metrics
manager.monitor().reset_metrics("test_plugin");
manager.monitor().reset_all_metrics();

// Stop monitoring
manager.stop().await?;
```

### Prometheus Metrics

The following Prometheus metrics are exported:

- `plugin_executions_total`: Total number of plugin executions
- `plugin_execution_duration_seconds`: Plugin execution duration histogram
- `plugin_memory_usage_bytes`: Plugin memory usage in bytes
- `plugin_cpu_usage_percent`: Plugin CPU usage percentage
- `plugin_errors_total`: Total number of plugin errors

---

## Lifecycle Management

The lifecycle manager handles plugin lifecycle including initialization, startup, shutdown, and state transitions.

### Features

- **State Management**: Track plugin states
- **Lifecycle Hooks**: Execute hooks at lifecycle events
- **Auto-restart**: Automatically restart failed plugins
- **Graceful Shutdown**: Clean shutdown with timeout
- **Status Monitoring**: Monitor plugin status

### Usage

```rust
use vantisplayer::advanced_plugins::lifecycle::{LifecycleConfig, LifecycleHooks};

// Load plugin
manager.lifecycle().load_plugin("test_plugin", &plugin_path).await?;

// Initialize plugin
manager.lifecycle().initialize_plugin("test_plugin").await?;

// Start plugin
let config = LifecycleConfig::default();
manager.lifecycle().start_plugin("test_plugin", &config).await?;

// Stop plugin
manager.lifecycle().stop_plugin("test_plugin", &config).await?;

// Unload plugin
manager.lifecycle().unload_plugin("test_plugin").await?;

// Handle error
manager.lifecycle().handle_error("test_plugin", "Error message".to_string(), &config).await?;

// Set lifecycle hooks
let hooks = LifecycleHooks {
    plugin_name: "test_plugin".to_string(),
    on_load: Some("load_hook".to_string()),
    on_init: Some("init_hook".to_string()),
    on_start: Some("start_hook".to_string()),
    on_stop: Some("stop_hook".to_string()),
    on_unload: Some("unload_hook".to_string()),
    on_error: Some("error_hook".to_string()),
};

manager.lifecycle().set_hooks(hooks);

// Get plugin state
if let Some(state) = manager.lifecycle().get_state("test_plugin") {
    println!("State: {:?}", state);
}

// Get running plugins
let running = manager.lifecycle().get_running_plugins();
println!("Running plugins: {:?}", running);

// Get status
let status = manager.lifecycle().get_status();
println!("Total: {}", status.total);
println!("Running: {}", status.running);
println!("Stopped: {}", status.stopped);
println!("Errored: {}", status.errored);

// Stop all plugins
manager.lifecycle().stop_all(&config).await?;

// Unload all plugins
manager.lifecycle().unload_all().await?;
```

### Lifecycle States

- `Unloaded`: Plugin not loaded
- `Loaded`: Plugin loaded but not initialized
- `Initialized`: Plugin initialized
- `Starting`: Plugin starting
- `Running`: Plugin running
- `Stopping`: Plugin stopping
- `Stopped`: Plugin stopped
- `Error(String)`: Plugin error

### Lifecycle Configuration

```rust
let config = LifecycleConfig {
    auto_start: true,           // Auto-start on load
    auto_restart: true,         // Auto-restart on error
    max_restart_attempts: 3,    // Maximum restart attempts
    shutdown_timeout: 30,       // Shutdown timeout in seconds
    graceful_shutdown: true,    // Graceful shutdown
};
```

---

## Configuration

### Advanced Plugin Configuration

```rust
use vantisplayer::advanced_plugins::AdvancedPluginConfig;

let config = AdvancedPluginConfig {
    enable_marketplace: true,
    marketplace_url: "https://plugins.vantis.io".to_string(),
    enable_hot_reload: true,
    hot_reload_interval: 5,
    enable_monitoring: true,
    monitoring_interval: 10,
    enable_sandbox: true,
    sandbox_timeout: 30,
    max_plugin_memory: 512,
    enable_auto_update: true,
    auto_update_interval: 24,
};

let manager = AdvancedPluginManager::with_config(
    PathBuf::from("/path/to/plugins"),
    PathBuf::from("/path/to/cache"),
    config,
)?;
```

### Configuration Options

- `enable_marketplace`: Enable plugin marketplace (default: true)
- `marketplace_url`: Marketplace URL (default: "https://plugins.vantis.io")
- `enable_hot_reload`: Enable hot reload (default: true)
- `hot_reload_interval`: Hot reload check interval in seconds (default: 5)
- `enable_monitoring`: Enable performance monitoring (default: true)
- `monitoring_interval`: Monitoring interval in seconds (default: 10)
- `enable_sandbox`: Enable sandbox (default: true)
- `sandbox_timeout`: Sandbox timeout in seconds (default: 30)
- `max_plugin_memory`: Maximum memory per plugin in MB (default: 512)
- `enable_auto_update`: Enable auto-update (default: true)
- `auto_update_interval`: Auto-update check interval in hours (default: 24)

---

## Usage Examples

### Complete Example

```rust
use vantisplayer::advanced_plugins::AdvancedPluginManager;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create manager
    let manager = AdvancedPluginManager::new(
        PathBuf::from("./plugins"),
        PathBuf::from("./cache"),
    )?;
    
    // Start all subsystems
    manager.start().await?;
    
    // Search and install plugin
    let query = SearchQuery {
        query: "audio".to_string(),
        ..Default::default()
    };
    
    let results = manager.marketplace().search(&query).await?;
    if let Some(plugin) = results.first() {
        let dest = PathBuf::from("./plugins").join(format!("{}.wasm", plugin.id));
        manager.marketplace().download_plugin(&plugin.id, &dest).await?;
        
        // Load plugin
        let wasm_bytes = tokio::fs::read(&dest).await?;
        manager.sandbox().load_plugin(&plugin.id, &wasm_bytes, plugin.permissions.clone()).await?;
        
        // Initialize and start
        manager.lifecycle().load_plugin(&plugin.id, &dest).await?;
        manager.lifecycle().initialize_plugin(&plugin.id).await?;
        manager.lifecycle().start_plugin(&plugin.id, &LifecycleConfig::default()).await?;
    }
    
    // Monitor performance
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            let snapshot = manager.monitor().take_snapshot();
            println!("Active plugins: {}", snapshot.system.active_plugins);
        }
    });
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    
    // Stop all subsystems
    manager.stop().await?;
    
    Ok(())
}
```

---

## Best Practices

### 1. Permission Management

- Always use the minimum required permissions
- Review plugin permissions before installation
- Use permission policies to restrict access
- Regularly audit plugin permissions

### 2. Resource Limits

- Set appropriate memory limits for each plugin
- Monitor resource usage regularly
- Use resource limits to prevent runaway plugins
- Adjust limits based on plugin behavior

### 3. Hot Reload

- Enable state preservation for critical plugins
- Use debouncing to prevent excessive reloads
- Test hot reload in development before production
- Monitor reload events for errors

### 4. Performance Monitoring

- Monitor execution times regularly
- Set up alerts for high error rates
- Use Prometheus metrics for long-term monitoring
- Review performance reports periodically

### 5. Lifecycle Management

- Use graceful shutdown for clean exits
- Set appropriate shutdown timeouts
- Monitor plugin states regularly
- Handle errors appropriately

### 6. Dependency Management

- Keep dependencies up to date
- Use semantic versioning for requirements
- Resolve conflicts before installation
- Review dependency changes

---

## Troubleshooting

### Plugin Won't Load

**Problem**: Plugin fails to load

**Solutions**:
1. Check plugin file exists and is readable
2. Verify WASM file is valid
3. Check permissions are sufficient
4. Review sandbox limits
5. Check error logs

### Hot Reload Not Working

**Problem**: Hot reload not detecting changes

**Solutions**:
1. Verify hot reload is enabled
2. Check file watcher permissions
3. Verify plugin directory is correct
4. Check for file system events
5. Review hot reload logs

### Performance Issues

**Problem**: Plugin performance degradation

**Solutions**:
1. Check resource usage metrics
2. Review execution times
3. Check for memory leaks
4. Adjust resource limits
5. Optimize plugin code

### Permission Denied Errors

**Problem**: Plugin gets permission denied errors

**Solutions**:
1. Review plugin permissions
2. Check permission policies
3. Verify required permissions are granted
4. Review sandbox configuration
5. Check host function implementations

### Dependency Conflicts

**Problem**: Dependency resolution fails

**Solutions**:
1. Review dependency requirements
2. Check for version conflicts
3. Update dependency registry
4. Use compatible versions
5. Review conflict messages

---

## API Reference

See `docs/API_REFERENCE.md` for complete API documentation.

---

## Additional Resources

- [Plugin Development Guide](PLUGIN_DEVELOPMENT.md)
- [API Reference](docs/API_REFERENCE.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)
- [Examples](examples/)