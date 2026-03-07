//! Monitoring and Analytics Module
//!
//! This module provides monitoring, analytics, and observability features
//! for Vantis Media Player.

pub mod sentry;
pub mod telemetry;
pub mod webhooks;
pub mod metrics;

pub use sentry::*;
pub use telemetry::*;
pub use webhooks::*;
pub use metrics::*;

/// Initialize all monitoring systems
pub fn init(environment: &str, version: &str) -> anyhow::Result<()> {
    sentry::init_sentry(environment, version)?;
    
    tracing::info!("Monitoring systems initialized");
    
    Ok(())
}