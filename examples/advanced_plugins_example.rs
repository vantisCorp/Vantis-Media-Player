//! Advanced Plugin System Example
//! 
//! This example demonstrates all features of the advanced plugin system:
//! - Plugin marketplace search and installation
//! - Dependency management
//! - Enhanced sandbox with permissions
//! - Hot reload with state preservation
//! - Performance monitoring
//! - Lifecycle management

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use vantisplayer::advanced_plugins::{
    AdvancedPluginManager, AdvancedPluginConfig,
    marketplace::{SearchQuery, SortBy, SortOrder, PluginCategory},
    sandbox::{PluginPermission, PermissionPolicy, ResourceLimits},
    hot_reload::{ReloadConfig, PluginState},
    lifecycle::{LifecycleConfig, LifecycleHooks},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Advanced Plugin System Example\n");
    
    // Create plugin and cache directories
    let plugin_dir = PathBuf::from("./plugins");
    let cache_dir = PathBuf::from("./cache");
    
    // Create manager with custom configuration
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
        plugin_dir.clone(),
        cache_dir.clone(),
        config,
    )?;
    
    // Start all subsystems
    println!("📡 Starting subsystems...");
    manager.start().await?;
    println!("✅ All subsystems started\n");
    
    // Example 1: Marketplace Search
    println!("=== Example 1: Marketplace Search ===\n");
    example_marketplace_search(&manager).await?;
    
    // Example 2: Dependency Management
    println!("\n=== Example 2: Dependency Management ===\n");
    example_dependency_management(&manager).await?;
    
    // Example 3: Enhanced Sandbox
    println!("\n=== Example 3: Enhanced Sandbox ===\n");
    example_enhanced_sandbox(&manager).await?;
    
    // Example 4: Hot Reload
    println!("\n=== Example 4: Hot Reload ===\n");
    example_hot_reload(&manager).await?;
    
    // Example 5: Performance Monitoring
    println!("\n=== Example 5: Performance Monitoring ===\n");
    example_performance_monitoring(&manager).await?;
    
    // Example 6: Lifecycle Management
    println!("\n=== Example 6: Lifecycle Management ===\n");
    example_lifecycle_management(&manager).await?;
    
    // Example 7: System Status
    println!("\n=== Example 7: System Status ===\n");
    example_system_status(&manager);
    
    // Keep running for a bit to demonstrate monitoring
    println!("\n⏳ Running for 30 seconds to demonstrate monitoring...");
    sleep(Duration::from_secs(30)).await;
    
    // Stop all subsystems
    println!("\n🛑 Stopping subsystems...");
    manager.stop().await?;
    println!("✅ All subsystems stopped");
    
    Ok(())
}

/// Example 1: Marketplace Search
async fn example_marketplace_search(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    println!("🔍 Searching for audio plugins...");
    
    let query = SearchQuery {
        query: "audio".to_string(),
        category: Some(PluginCategory::Audio),
        min_rating: Some(4.0),
        sort_by: SortBy::Rating,
        sort_order: SortOrder::Desc,
        limit: Some(10),
    };
    
    match manager.marketplace().search(&query).await {
        Ok(plugins) => {
            println!("✅ Found {} plugins:\n", plugins.len());
            for plugin in plugins.iter().take(5) {
                println!("  📦 {} v{}", plugin.name, plugin.version);
                println!("     Author: {}", plugin.author);
                println!("     Rating: {:.1}/5.0 ({} downloads)", plugin.rating, plugin.downloads);
                println!("     Description: {}\n", plugin.description);
            }
        }
        Err(e) => {
            println!("⚠️  Search failed (marketplace may not be available): {}", e);
        }
    }
    
    // Get featured plugins
    println!("⭐ Getting featured plugins...");
    match manager.marketplace().get_featured().await {
        Ok(featured) => {
            println!("✅ Featured plugins: {}\n", featured.len());
        }
        Err(e) => {
            println!("⚠️  Failed to get featured: {}\n", e);
        }
    }
    
    Ok(())
}

/// Example 2: Dependency Management
async fn example_dependency_management(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    use vantisplayer::advanced_plugins::dependencies::{DependencyRequirement, VersionReq};
    use semver::Version;
    
    println!("📦 Managing dependencies...");
    
    // Define requirements
    let requirements = vec![
        DependencyRequirement {
            name: "example_dep".to_string(),
            version_req: VersionReq::parse(">=1.0.0")?,
            optional: false,
        },
    ];
    
    // Resolve dependencies
    println!("🔍 Resolving dependencies...");
    let result = manager.dependency_manager().resolve(&requirements).await?;
    
    println!("✅ Resolved {} dependencies", result.dependencies.len());
    
    if !result.conflicts.is_empty() {
        println!("⚠️  Conflicts: {}", result.conflicts.len());
    }
    
    if !result.missing.is_empty() {
        println!("⚠️  Missing: {:?}", result.missing);
    }
    
    // List installed dependencies
    println!("\n📋 Installed dependencies:");
    let installed = manager.dependency_manager().list_installed().await;
    if installed.is_empty() {
        println!("  (none)");
    } else {
        for dep in installed {
            println!("  - {}@{}", dep.name, dep.version);
        }
    }
    
    Ok(())
}

/// Example 3: Enhanced Sandbox
async fn example_enhanced_sandbox(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    println!("🔒 Enhanced Sandbox with permissions...");
    
    // Create permission policy
    let policy = PermissionPolicy {
        plugin_name: "example_plugin".to_string(),
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
    
    manager.sandbox().add_policy(policy);
    println!("✅ Permission policy added");
    
    // Check permissions
    println!("\n🔍 Checking permissions:");
    println!("  - Logging: {}", 
        manager.sandbox().has_permission("example_plugin", &PluginPermission::Logging));
    println!("  - Network: {}", 
        manager.sandbox().has_permission("example_plugin", &PluginPermission::Network));
    println!("  - FileSystemRead: {}", 
        manager.sandbox().has_permission("example_plugin", &PluginPermission::FileSystemRead));
    
    // Get policy
    if let Some(retrieved) = manager.sandbox().get_policy("example_plugin") {
        println!("\n📋 Policy details:");
        println!("  - Allowed permissions: {}", retrieved.allowed.len());
        println!("  - Denied permissions: {}", retrieved.denied.len());
        println!("  - Max memory: {} MB", retrieved.limits.max_memory / 1024 / 1024);
    }
    
    Ok(())
}

/// Example 4: Hot Reload
async fn example_hot_reload(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    println!("🔄 Hot Reload with state preservation...");
    
    // Save plugin state
    let state = PluginState {
        name: "example_plugin".to_string(),
        version: "1.0.0".to_string(),
        config: serde_json::json!({
            "setting1": "value1",
            "setting2": 42,
        }),
        data: {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), serde_json::json!("data1"));
            map
        },
        last_reload: chrono::Utc::now(),
    };
    
    manager.hot_reload().save_state("example_plugin", state);
    println!("✅ Plugin state saved");
    
    // Load plugin state
    if let Some(loaded_state) = manager.hot_reload().load_state("example_plugin") {
        println!("\n📦 Loaded state:");
        println!("  - Name: {}", loaded_state.name);
        println!("  - Version: {}", loaded_state.version);
        println!("  - Config: {}", loaded_state.config);
        println!("  - Data keys: {}", loaded_state.data.len());
    }
    
    // Get all states
    println!("\n📋 All saved states: {}", manager.hot_reload().get_all_states().len());
    
    // Check hot reload status
    println!("\n📊 Hot reload status:");
    println!("  - Active: {}", manager.hot_reload().is_active());
    
    Ok(())
}

/// Example 5: Performance Monitoring
async fn example_performance_monitoring(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    println!("📊 Performance Monitoring...");
    
    // Record some executions
    println!("📈 Recording plugin executions...");
    for i in 0..5 {
        let duration = Duration::from_millis(100 + i * 50);
        manager.monitor().record_execution("example_plugin", duration);
    }
    
    // Record some errors
    println!("❌ Recording errors...");
    manager.monitor().record_error("example_plugin");
    manager.monitor().record_error("example_plugin");
    
    // Update resource metrics
    println!("💾 Updating resource metrics...");
    manager.monitor().update_memory("example_plugin", 128 * 1024 * 1024); // 128 MB
    manager.monitor().update_cpu("example_plugin", 45.5); // 45.5%
    
    // Get plugin metrics
    println!("\n📊 Plugin metrics:");
    if let Some(metrics) = manager.monitor().get_metrics("example_plugin") {
        println!("  - Executions: {}", metrics.execution_count);
        println!("  - Avg time: {:?}", metrics.average_execution_time);
        println!("  - Memory: {} MB", metrics.memory_usage / 1024 / 1024);
        println!("  - CPU: {:.1}%", metrics.cpu_usage);
        println!("  - Errors: {}", metrics.error_count);
        println!("  - Peak memory: {} MB", metrics.peak_memory / 1024 / 1024);
        println!("  - Peak CPU: {:.1}%", metrics.peak_cpu);
    }
    
    // Take snapshot
    println!("\n📸 Performance snapshot:");
    let snapshot = manager.monitor().take_snapshot();
    println!("  - Active plugins: {}", snapshot.system.active_plugins);
    println!("  - Total executions: {}", snapshot.system.total_executions);
    println!("  - Total errors: {}", snapshot.system.total_errors);
    println!("  - Total memory: {} MB", snapshot.system.total_memory / 1024 / 1024);
    println!("  - Total CPU: {:.1}%", snapshot.system.total_cpu);
    
    // Export Prometheus metrics
    println!("\n📊 Prometheus metrics:");
    match manager.monitor().export_prometheus() {
        Ok(metrics) => {
            println!("  ({} bytes exported)", metrics.len());
        }
        Err(e) => {
            println!("  ⚠️  Failed to export: {}", e);
        }
    }
    
    // Get performance report
    println!("\n📋 Performance report:");
    let report = manager.monitor().get_report();
    for line in report.lines().take(10) {
        println!("  {}", line);
    }
    
    Ok(())
}

/// Example 6: Lifecycle Management
async fn example_lifecycle_management(manager: &AdvancedPluginManager) -> anyhow::Result<()> {
    println!("🔄 Lifecycle Management...");
    
    // Set lifecycle hooks
    let hooks = LifecycleHooks {
        plugin_name: "example_plugin".to_string(),
        on_load: Some("load_hook".to_string()),
        on_init: Some("init_hook".to_string()),
        on_start: Some("start_hook".to_string()),
        on_stop: Some("stop_hook".to_string()),
        on_unload: Some("unload_hook".to_string()),
        on_error: Some("error_hook".to_string()),
    };
    
    manager.lifecycle().set_hooks(hooks);
    println!("✅ Lifecycle hooks set");
    
    // Simulate lifecycle transitions
    println!("\n🔄 Simulating lifecycle transitions...");
    
    // Load
    manager.lifecycle().set_state("example_plugin", 
        vantisplayer::advanced_plugins::lifecycle::PluginLifecycleState::Loaded);
    println!("  - State: Loaded");
    
    // Initialize
    manager.lifecycle().set_state("example_plugin", 
        vantisplayer::advanced_plugins::lifecycle::PluginLifecycleState::Initialized);
    println!("  - State: Initialized");
    
    // Start
    manager.lifecycle().set_state("example_plugin", 
        vantisplayer::advanced_plugins::lifecycle::PluginLifecycleState::Running);
    println!("  - State: Running");
    
    // Get status
    println!("\n📊 Lifecycle status:");
    let status = manager.lifecycle().get_status();
    println!("  - Total plugins: {}", status.total);
    println!("  - Running: {}", status.running);
    println!("  - Stopped: {}", status.stopped);
    println!("  - Errored: {}", status.errored);
    
    // Get running plugins
    println!("\n🚀 Running plugins:");
    let running = manager.lifecycle().get_running_plugins();
    if running.is_empty() {
        println!("  (none)");
    } else {
        for plugin in running {
            println!("  - {}", plugin);
        }
    }
    
    Ok(())
}

/// Example 7: System Status
fn example_system_status(manager: &AdvancedPluginManager) {
    println!("📊 System Status\n");
    
    let status = manager.get_status();
    
    println!("Configuration:");
    println!("  - Marketplace: {}", if status.marketplace_enabled { "✅ Enabled" } else { "❌ Disabled" });
    println!("  - Hot Reload: {}", if status.hot_reload_enabled { "✅ Enabled" } else { "❌ Disabled" });
    println!("  - Monitoring: {}", if status.monitoring_enabled { "✅ Enabled" } else { "❌ Disabled" });
    println!("  - Sandbox: {}", if status.sandbox_enabled { "✅ Enabled" } else { "❌ Disabled" });
    println!("  - Auto-update: {}", if status.auto_update_enabled { "✅ Enabled" } else { "❌ Disabled" });
    
    println!("\nActive Status:");
    println!("  - Hot Reload: {}", if status.hot_reload_active { "🟢 Active" } else { "🔴 Inactive" });
    println!("  - Monitoring: {}", if status.monitoring_active { "🟢 Active" } else { "🔴 Inactive" });
    
    println!("\nPlugins:");
    println!("  - Total: {}", status.plugin_count);
    
    println!("\nDirectories:");
    println!("  - Plugin dir: {}", manager.plugin_dir().display());
    println!("  - Cache dir: {}", manager.cache_dir().display());
}