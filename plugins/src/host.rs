//! Host Functions for WASM Plugins
//! 
/// Functions exposed from the host to WASM plugins.

use anyhow::Result;
use tracing::debug;
use wasmtime::{Caller, Linker};

use super::HostState;

/// Register host functions that plugins can call
pub fn register_host_functions(linker: &mut Linker<HostState>) -> Result<()> {
    // Logging functions
    linker.func_wrap("vantis", "log", log)?;
    linker.func_wrap("vantis", "log_info", log_info)?;
    linker.func_wrap("vantis", "log_warn", log_warn)?;
    linker.func_wrap("vantis", "log_error", log_error)?;
    
    // Media control functions
    linker.func_wrap("vantis", "play", play)?;
    linker.func_wrap("vantis", "pause", pause)?;
    linker.func_wrap("vantis", "stop", stop)?;
    linker.func_wrap("vantis", "seek", seek)?;
    
    // Utility functions
    linker.func_wrap("vantis", "get_time", get_time)?;
    linker.func_wrap("vantis", "sleep", sleep)?;
    
    debug!("🔗 Host functions registered");
    
    Ok(())
}

/// Log message
fn log(mut caller: Caller<HostState>, ptr: u32, len: u32) -> Result<(), anyhow::Error> {
    let memory = caller.get_export("memory")
        .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?
        .into_memory()
        .ok_or_else(|| anyhow::anyhow!("Memory export is not a memory"))?;
    
    let data = memory.data(&caller);
    let bytes = &data[ptr as usize..(ptr + len) as usize];
    let message = String::from_utf8_lossy(bytes);
    
    debug!("[Plugin:{}] {}", caller.data().plugin_name, message);
    
    Ok(())
}

/// Log info message
fn log_info(mut caller: Caller<HostState>, ptr: u32, len: u32) -> Result<(), anyhow::Error> {
    let memory = caller.get_export("memory")
        .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?
        .into_memory()
        .ok_or_else(|| anyhow::anyhow!("Memory export is not a memory"))?;
    
    let data = memory.data(&caller);
    let bytes = &data[ptr as usize..(ptr + len) as usize];
    let message = String::from_utf8_lossy(bytes);
    
    tracing::info!("[Plugin:{}] {}", caller.data().plugin_name, message);
    
    Ok(())
}

/// Log warning message
fn log_warn(mut caller: Caller<HostState>, ptr: u32, len: u32) -> Result<(), anyhow::Error> {
    let memory = caller.get_export("memory")
        .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?
        .into_memory()
        .ok_or_else(|| anyhow::anyhow!("Memory export is not a memory"))?;
    
    let data = memory.data(&caller);
    let bytes = &data[ptr as usize..(ptr + len) as usize];
    let message = String::from_utf8_lossy(bytes);
    
    tracing::warn!("[Plugin:{}] {}", caller.data().plugin_name, message);
    
    Ok(())
}

/// Log error message
fn log_error(mut caller: Caller<HostState>, ptr: u32, len: u32) -> Result<(), anyhow::Error> {
    let memory = caller.get_export("memory")
        .ok_or_else(|| anyhow::anyhow!("Failed to find memory export"))?
        .into_memory()
        .ok_or_else(|| anyhow::anyhow!("Memory export is not a memory"))?;
    
    let data = memory.data(&caller);
    let bytes = &data[ptr as usize..(ptr + len) as usize];
    let message = String::from_utf8_lossy(bytes);
    
    tracing::error!("[Plugin:{}] {}", caller.data().plugin_name, message);
    
    Ok(())
}

/// Play media
fn play(caller: Caller<HostState>) -> Result<(), anyhow::Error> {
    debug!("[Plugin:{}] Command: play", caller.data().plugin_name);
    Ok(())
}

/// Pause media
fn pause(caller: Caller<HostState>) -> Result<(), anyhow::Error> {
    debug!("[Plugin:{}] Command: pause", caller.data().plugin_name);
    Ok(())
}

/// Stop media
fn stop(caller: Caller<HostState>) -> Result<(), anyhow::Error> {
    debug!("[Plugin:{}] Command: stop", caller.data().plugin_name);
    Ok(())
}

/// Seek to position
fn seek(caller: Caller<HostState>, position_ms: u64) -> Result<(), anyhow::Error> {
    debug!("[Plugin:{}] Command: seek({}ms)", caller.data().plugin_name, position_ms);
    Ok(())
}

/// Get current time
fn get_time(_caller: Caller<HostState>) -> Result<u64, anyhow::Error> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    Ok(time)
}

/// Sleep for specified milliseconds
fn sleep(caller: Caller<HostState>, ms: u32) -> Result<(), anyhow::Error> {
    debug!("[Plugin:{}] Sleep: {}ms", caller.data().plugin_name, ms);
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    Ok(())
}