//! Monitoring and analytics for Vantis Media Player
//!
//! This crate provides monitoring, profiling, and analytics capabilities.

pub mod metrics;
pub mod profiling;
pub mod sentry;
pub mod telemetry;
pub mod webhooks;

// Re-export main types
pub use metrics::*;
pub use profiling::{
    ProfiledOperation, ProfileSample, ProfilingSession, ProfilingDashboard,
    DashboardConfig, RealTimeMetrics, PerformanceReport, PerformanceSummary,
    OperationStats, MemorySnapshot, CpuSnapshot, GpuSnapshot,
    PerformanceHotspot, HotspotType, HotspotSeverity,
};
pub use sentry::*;
pub use telemetry::*;
pub use webhooks::*;