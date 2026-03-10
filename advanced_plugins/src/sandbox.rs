//! Enhanced Sandbox
//! 
//! Provides fine-grained permission control and resource limits for plugin execution.

use anyhow::{Context, Result};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn, error};
use wasmtime::{Engine, Module, Store, Linker, Config, StoreLimits, StoreLimitsBuilder};

/// Enhanced sandbox
pub struct EnhancedSandbox {
    /// Wasmtime engine
    engine: Engine,
    
    /// Linker for host functions
    linker: Linker<SandboxState>,
    
    /// Maximum memory per plugin (in bytes)
    max_memory: usize,
    
    /// Timeout in seconds
    timeout: u64,
    
    /// Active instances
    instances: Arc<RwLock<HashMap<String, SandboxInstance>>>,
    
    /// Permission policies
    policies: Arc<RwLock<HashMap<String, PermissionPolicy>>>,
}

/// Sandbox state
pub struct SandboxState {
    /// Plugin name
    plugin_name: String,
    
    /// Resource limiter
    limiter: StoreLimits,
    
    /// Permissions
    permissions: Vec<PluginPermission>,
}

/// Sandbox instance
#[derive(Clone)]
pub struct SandboxInstance {
    /// Instance name
    pub name: String,
    
    /// Creation time
    pub created_at: Instant,
    
    /// Last activity
    pub last_activity: Instant,
    
    /// Memory usage
    pub memory_usage: usize,
    
    /// CPU time
    pub cpu_time: Duration,
    
    /// Is active
    pub active: bool,
}

/// Permission policy
#[derive(Clone, Debug)]
pub struct PermissionPolicy {
    /// Plugin name
    pub plugin_name: String,
    
    /// Allowed permissions
    pub allowed: Vec<PluginPermission>,
    
    /// Denied permissions
    pub denied: Vec<PluginPermission>,
    
    /// Resource limits
    pub limits: ResourceLimits,
}

/// Resource limits
#[derive(Clone, Debug)]
pub struct ResourceLimits {
    /// Maximum memory (bytes)
    pub max_memory: usize,
    
    /// Maximum CPU time (seconds)
    pub max_cpu_time: u64,
    
    /// Maximum file descriptors
    pub max_fds: u32,
    
    /// Maximum network connections
    pub max_connections: u32,
    
    /// Maximum execution time (seconds)
    pub max_execution_time: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: 512 * 1024 * 1024, // 512 MB
            max_cpu_time: 60,
            max_fds: 32,
            max_connections: 10,
            max_execution_time: 30,
        }
    }
}

/// Plugin permission
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PluginPermission {
    /// File system read
    FileSystemRead,
    
    /// File system write
    FileSystemWrite,
    
    /// Network access
    Network,
    
    /// Media control
    MediaControl,
    
    /// Configuration read
    ConfigRead,
    
    /// Configuration write
    ConfigWrite,
    
    /// Logging
    Logging,
    
    /// Custom permission
    Custom(String),
}

impl EnhancedSandbox {
    /// Create a new enhanced sandbox
    pub fn new(max_memory: usize, timeout: u64) -> Result<Self> {
        info!("🔒 Initializing Enhanced Sandbox");
        
        // Configure Wasmtime
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_multi_memory(true);
        config.wasm_threads(true);
        config.consume_fuel(true);
        
        let engine = Engine::new(&config)?;
        
        // Create linker
        let mut linker = Linker::new(&engine);
        
        // Register host functions with permission checks
        Self::register_host_functions(&mut linker)?;
        
        info!("✅ Enhanced sandbox initialized");
        info!("   - Max memory: {} MB", max_memory / 1024 / 1024);
        info!("   - Timeout: {} seconds", timeout);
        
        Ok(Self {
            engine,
            linker,
            max_memory,
            timeout,
            instances: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Register host functions
    fn register_host_functions(linker: &mut Linker<SandboxState>) -> Result<()> {
        // Logging function
        linker.func_wrap("vantis", "log", |mut caller: wasmtime::Caller<'_, SandboxState>, level: u32, ptr: u32, len: u32| -> Result<(), anyhow::Error> {
            let state = caller.data();
            
            // Check permission
            if !state.permissions.contains(&PluginPermission::Logging) {
                return Err(anyhow::anyhow!("Permission denied: logging"));
            }
            
            let mem = match caller.get_export("memory") {
                Some(export) => export.into_memory().unwrap(),
                None => return Err(anyhow::anyhow!("failed to find memory export")),
            };
            
            let data = mem.data(&caller);
            let bytes = &data[ptr as usize..(ptr + len) as usize];
            let message = String::from_utf8_lossy(bytes);
            
            match level {
                0 => tracing::error!("[{}] {}", state.plugin_name, message),
                1 => tracing::warn!("[{}] {}", state.plugin_name, message),
                2 => tracing::info!("[{}] {}", state.plugin_name, message),
                3 => tracing::debug!("[{}] {}", state.plugin_name, message),
                _ => tracing::trace!("[{}] {}", state.plugin_name, message),
            }
            
            Ok(())
        })?;
        
        // File system read function
        linker.func_wrap("vantis", "fs_read", |caller: wasmtime::Caller<'_, SandboxState>, _path_ptr: u32, _path_len: u32| -> Result<i32, anyhow::Error> {
            let state = caller.data();
            
            // Check permission
            if !state.permissions.contains(&PluginPermission::FileSystemRead) {
                return Err(anyhow::anyhow!("Permission denied: fs_read"));
            }
            
            // Implementation would read file and return content
            Ok(0)
        })?;
        
        // File system write function
        linker.func_wrap("vantis", "fs_write", |caller: wasmtime::Caller<'_, SandboxState>, _path_ptr: u32, _path_len: u32, _data_ptr: u32, _data_len: u32| -> Result<i32, anyhow::Error> {
            let state = caller.data();
            
            // Check permission
            if !state.permissions.contains(&PluginPermission::FileSystemWrite) {
                return Err(anyhow::anyhow!("Permission denied: fs_write"));
            }
            
            // Implementation would write data to file
            Ok(0)
        })?;
        
        // Network request function
        linker.func_wrap("vantis", "http_request", |caller: wasmtime::Caller<'_, SandboxState>, _url_ptr: u32, _url_len: u32| -> Result<i32, anyhow::Error> {
            let state = caller.data();
            
            // Check permission
            if !state.permissions.contains(&PluginPermission::Network) {
                return Err(anyhow::anyhow!("Permission denied: http_request"));
            }
            
            // Implementation would make HTTP request
            Ok(0)
        })?;
        
        Ok(())
    }
    
    /// Create permission policy
    pub fn create_policy(&self, plugin_name: &str, allowed: Vec<PluginPermission>, denied: Vec<PluginPermission>) -> PermissionPolicy {
        PermissionPolicy {
            plugin_name: plugin_name.to_string(),
            allowed,
            denied,
            limits: ResourceLimits::default(),
        }
    }
    
    /// Add permission policy
    pub fn add_policy(&self, policy: PermissionPolicy) {
        let name = policy.plugin_name.clone();
        let mut policies = self.policies.write();
        policies.insert(name.clone(), policy);
        debug!("✅ Added permission policy for: {}", name);
    }
    
    /// Get permission policy
    pub fn get_policy(&self, plugin_name: &str) -> Option<PermissionPolicy> {
        let policies = self.policies.read();
        policies.get(plugin_name).cloned()
    }
    
    /// Load plugin into sandbox
    pub async fn load_plugin(&self, name: &str, wasm_bytes: &[u8], permissions: Vec<PluginPermission>) -> Result<()> {
        info!("🔒 Loading plugin into sandbox: {}", name);
        
        // Get or create policy
        let policy = self.get_policy(name).unwrap_or_else(|| {
            self.create_policy(name, permissions.clone(), Vec::new())
        });
        
        // Build store limits
        let limiter = StoreLimitsBuilder::new()
            .memory_size(self.max_memory)
            .build();
        
        // Create store with state
        let mut store = Store::new(&self.engine, SandboxState {
            plugin_name: name.to_string(),
            limiter,
            permissions: policy.allowed.clone(),
        });
        
        // Set fuel limit for timeout
        store.set_fuel(self.timeout * 1_000_000)?; // 1 million fuel per second
        
        // Enable resource limiter
        store.limiter(|state| &mut state.limiter);
        
        // Compile module
        let module = Module::from_binary(&self.engine, wasm_bytes)?;
        
        // Instantiate
        let _instance = self.linker.instantiate(&mut store, &module)?;
        
        // Create instance record
        let instance = SandboxInstance {
            name: name.to_string(),
            created_at: Instant::now(),
            last_activity: Instant::now(),
            memory_usage: 0,
            cpu_time: Duration::ZERO,
            active: true,
        };
        
        // Store instance
        let mut instances = self.instances.write();
        instances.insert(name.to_string(), instance);
        
        info!("✅ Plugin loaded into sandbox: {}", name);
        
        Ok(())
    }
    
    /// Unload plugin from sandbox
    pub async fn unload_plugin(&self, name: &str) -> Result<()> {
        info!("🔓 Unloading plugin from sandbox: {}", name);
        
        let mut instances = self.instances.write();
        if instances.remove(name).is_some() {
            debug!("✅ Plugin unloaded from sandbox: {}", name);
        } else {
            warn!("⚠️  Plugin not found in sandbox: {}", name);
        }
        
        Ok(())
    }
    
    /// Get instance
    pub fn get_instance(&self, name: &str) -> Option<SandboxInstance> {
        let instances = self.instances.read();
        instances.get(name).cloned()
    }
    
    /// Get all instances
    pub fn get_instances(&self) -> Vec<SandboxInstance> {
        let instances = self.instances.read();
        instances.values().cloned().collect()
    }
    
    /// Check if plugin has permission
    pub fn has_permission(&self, plugin_name: &str, permission: &PluginPermission) -> bool {
        if let Some(policy) = self.get_policy(plugin_name) {
            // Check denied first
            if policy.denied.contains(permission) {
                return false;
            }
            // Check allowed
            policy.allowed.contains(permission)
        } else {
            false
        }
    }
    
    /// Update instance activity
    pub fn update_activity(&self, name: &str) {
        let mut instances = self.instances.write();
        if let Some(instance) = instances.get_mut(name) {
            instance.last_activity = Instant::now();
        }
    }
    
    /// Clean up inactive instances
    pub async fn cleanup_inactive(&self, timeout: Duration) -> usize {
        let mut instances = self.instances.write();
        let now = Instant::now();
        
        let mut to_remove = Vec::new();
        for (name, instance) in instances.iter() {
            if now.duration_since(instance.last_activity) > timeout {
                to_remove.push(name.clone());
            }
        }
        
        for name in &to_remove {
            instances.remove(name);
            debug!("🧹 Cleaned up inactive instance: {}", name);
        }
        
        to_remove.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sandbox_creation() {
        let sandbox = EnhancedSandbox::new(512 * 1024 * 1024, 30);
        assert!(sandbox.is_ok());
    }
    
    #[test]
    fn test_permission_policy() {
        let sandbox = EnhancedSandbox::new(512 * 1024 * 1024, 30).unwrap();
        let policy = sandbox.create_policy(
            "test_plugin",
            vec![PluginPermission::Logging, PluginPermission::FileSystemRead],
            vec![PluginPermission::Network],
        );
        
        assert_eq!(policy.plugin_name, "test_plugin");
        assert_eq!(policy.allowed.len(), 2);
        assert_eq!(policy.denied.len(), 1);
    }
    
    #[test]
    fn test_permission_check() {
        let sandbox = EnhancedSandbox::new(512 * 1024 * 1024, 30).unwrap();
        let policy = sandbox.create_policy(
            "test_plugin",
            vec![PluginPermission::Logging],
            vec![],
        );
        
        sandbox.add_policy(policy);
        
        assert!(sandbox.has_permission("test_plugin", &PluginPermission::Logging));
        assert!(!sandbox.has_permission("test_plugin", &PluginPermission::Network));
    }
    
    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory, 512 * 1024 * 1024);
        assert_eq!(limits.max_cpu_time, 60);
    }
}