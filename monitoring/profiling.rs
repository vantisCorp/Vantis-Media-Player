//! Performance Profiling Dashboard
//! 
//! Real-time performance profiling and analysis tools for development and optimization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;

/// Profiled operation with timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfiledOperation {
    /// Unique identifier for this operation
    pub id: String,
    /// Operation name/label
    pub name: String,
    /// Category of operation (rendering, audio, io, etc.)
    pub category: OperationCategory,
    /// Start time of the operation
    pub start_time: f64,
    /// Duration in milliseconds
    pub duration_ms: f64,
    /// Whether the operation is currently active
    pub is_active: bool,
    /// Parent operation ID (for nested operations)
    pub parent_id: Option<String>,
    /// Child operation IDs
    pub children: Vec<String>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationCategory {
    Rendering,
    AudioProcessing,
    VideoDecoding,
    IoOperation,
    NetworkRequest,
    PluginExecution,
    MemoryOperation,
    UserInteraction,
    SystemCall,
    Custom,
}

/// Individual profile sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSample {
    pub timestamp: f64,
    pub operation_id: String,
    pub sample_type: SampleType,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SampleType {
    CpuTime,
    GpuTime,
    MemoryUsage,
    FrameTime,
    Custom(String),
}

/// Statistics for an operation type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationStats {
    pub operation_name: String,
    pub total_calls: u64,
    pub total_time_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub avg_time_ms: f64,
    pub percentile_50: f64,
    pub percentile_90: f64,
    pub percentile_99: f64,
}

/// Profiling session for tracking operations over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingSession {
    pub id: String,
    pub name: String,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub operations: HashMap<String, ProfiledOperation>,
    pub samples: VecDeque<ProfileSample>,
    pub config: ProfilingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub sample_rate_hz: u32,
    pub max_samples: usize,
    pub track_memory: bool,
    pub track_cpu: bool,
    pub track_gpu: bool,
    pub enabled_categories: Vec<OperationCategory>,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            sample_rate_hz: 60,
            max_samples: 10000,
            track_memory: true,
            track_cpu: true,
            track_gpu: false,
            enabled_categories: vec![
                OperationCategory::Rendering,
                OperationCategory::AudioProcessing,
                OperationCategory::VideoDecoding,
                OperationCategory::IoOperation,
            ],
        }
    }
}

/// Real-time metrics for the dashboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub gpu_usage_percent: f64,
    pub active_operations: u32,
    pub operations_per_frame: u32,
    pub last_update: f64,
}

/// Performance report with analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub session_id: String,
    pub duration_seconds: f64,
    pub total_operations: u64,
    pub operation_stats: HashMap<String, OperationStats>,
    pub hotspots: Vec<PerformanceHotspot>,
    pub recommendations: Vec<String>,
    pub summary: PerformanceSummary,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub avg_fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub avg_frame_time_ms: f64,
    pub total_cpu_time_ms: f64,
    pub total_gpu_time_ms: f64,
    pub memory_peak_mb: f64,
    pub slowest_operation: Option<String>,
    pub most_frequent_operation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub timestamp: f64,
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub timestamp: f64,
    pub user_time_ms: u64,
    pub system_time_ms: u64,
    pub idle_time_ms: u64,
    pub usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSnapshot {
    pub timestamp: f64,
    pub renderer: String,
    pub memory_used_mb: f64,
    pub memory_total_mb: f64,
    pub utilization_percent: f64,
    pub temperature_c: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceHotspot {
    pub operation_name: String,
    pub hotspot_type: HotspotType,
    pub severity: HotspotSeverity,
    pub total_time_ms: f64,
    pub occurrence_count: u64,
    pub avg_time_ms: f64,
    pub description: String,
    pub suggested_fix: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HotspotType {
    CpuBound,
    MemoryBound,
    GpuBound,
    IoBound,
    LockContention,
    MemoryLeak,
    ExcessiveAllocation,
    InefficientAlgorithm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HotspotSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Configuration for the profiling dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub update_interval_ms: u64,
    pub history_duration_seconds: f64,
    pub show_operations: bool,
    pub show_memory: bool,
    pub show_cpu: bool,
    pub show_gpu: bool,
    pub show_hotspots: bool,
    pub alert_thresholds: AlertThresholds,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            update_interval_ms: 100,
            history_duration_seconds: 60.0,
            show_operations: true,
            show_memory: true,
            show_cpu: true,
            show_gpu: false,
            show_hotspots: true,
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub fps_warning: f64,
    pub fps_critical: f64,
    pub frame_time_warning_ms: f64,
    pub frame_time_critical_ms: f64,
    pub memory_warning_mb: f64,
    pub memory_critical_mb: f64,
    pub cpu_warning_percent: f64,
    pub cpu_critical_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            fps_warning: 45.0,
            fps_critical: 30.0,
            frame_time_warning_ms: 22.0,
            frame_time_critical_ms: 33.0,
            memory_warning_mb: 500.0,
            memory_critical_mb: 1000.0,
            cpu_warning_percent: 70.0,
            cpu_critical_percent: 90.0,
        }
    }
}

/// Main profiling dashboard
pub struct ProfilingDashboard {
    config: DashboardConfig,
    current_session: Arc<RwLock<Option<ProfilingSession>>>,
    real_time_metrics: Arc<RwLock<RealTimeMetrics>>,
    operation_history: Arc<RwLock<VecDeque<ProfiledOperation>>>,
    memory_history: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    cpu_history: Arc<RwLock<VecDeque<CpuSnapshot>>>,
    gpu_history: Arc<RwLock<VecDeque<GpuSnapshot>>>,
    operation_stats: Arc<RwLock<HashMap<String, OperationStats>>>,
    hotspots: Arc<RwLock<Vec<PerformanceHotspot>>>,
    active_operations: Arc<RwLock<HashMap<String, Instant>>>,
}

impl ProfilingDashboard {
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            config,
            current_session: Arc::new(RwLock::new(None)),
            real_time_metrics: Arc::new(RwLock::new(RealTimeMetrics::default())),
            operation_history: Arc::new(RwLock::new(VecDeque::with_capacity(10000))),
            memory_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            cpu_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            gpu_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            operation_stats: Arc::new(RwLock::new(HashMap::new())),
            hotspots: Arc::new(RwLock::new(Vec::new())),
            active_operations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a new profiling session
    pub fn start_session(&self, name: &str) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = ProfilingSession {
            id: session_id.clone(),
            name: name.to_string(),
            start_time: Self::current_time(),
            end_time: None,
            operations: HashMap::new(),
            samples: VecDeque::with_capacity(self.config.history_duration_seconds as usize * 1000),
            config: ProfilingConfig::default(),
        };
        
        *self.current_session.write() = Some(session);
        session_id
    }

    /// End the current profiling session
    pub fn end_session(&self) -> Option<ProfilingSession> {
        let mut session_guard = self.current_session.write();
        if let Some(ref mut session) = *session_guard {
            session.end_time = Some(Self::current_time());
            Some(session.clone())
        } else {
            None
        }
    }

    /// Begin profiling an operation
    pub fn begin_operation(&self, name: &str, category: OperationCategory) -> String {
        let operation_id = uuid::Uuid::new_v4().to_string();
        let operation = ProfiledOperation {
            id: operation_id.clone(),
            name: name.to_string(),
            category,
            start_time: Self::current_time(),
            duration_ms: 0.0,
            is_active: true,
            parent_id: None,
            children: Vec::new(),
            metadata: HashMap::new(),
        };

        // Add to session if active
        if let Some(ref mut session) = *self.current_session.write() {
            session.operations.insert(operation_id.clone(), operation.clone());
        }

        // Track timing
        self.active_operations.write().insert(operation_id.clone(), Instant::now());
        
        operation_id
    }

    /// End profiling an operation
    pub fn end_operation(&self, operation_id: &str) -> Option<f64> {
        let start_instant = self.active_operations.write().remove(operation_id)?;
        let duration_ms = start_instant.elapsed().as_secs_f64() * 1000.0;

        // Update operation in session
        if let Some(ref mut session) = *self.current_session.write() {
            if let Some(op) = session.operations.get_mut(operation_id) {
                op.duration_ms = duration_ms;
                op.is_active = false;
            }
        }

        // Update stats
        self.update_operation_stats(operation_id, duration_ms);

        // Check for hotspots
        self.check_for_hotspot(operation_id, duration_ms);

        Some(duration_ms)
    }

    /// Update operation statistics
    fn update_operation_stats(&self, operation_id: &str, duration_ms: f64) {
        // Get operation name from session
        let operation_name = {
            let session = self.current_session.read();
            session.as_ref()
                .and_then(|s| s.operations.get(operation_id))
                .map(|op| op.name.clone())
                .unwrap_or_else(|| "unknown".to_string())
        };

        let mut stats = self.operation_stats.write();
        let entry = stats.entry(operation_name.clone()).or_insert(OperationStats {
            operation_name: operation_name.clone(),
            ..Default::default()
        });

        entry.total_calls += 1;
        entry.total_time_ms += duration_ms;
        entry.min_time_ms = if entry.min_time_ms == 0.0 { duration_ms } else { entry.min_time_ms.min(duration_ms) };
        entry.max_time_ms = entry.max_time_ms.max(duration_ms);
        entry.avg_time_ms = entry.total_time_ms / entry.total_calls as f64;
    }

    /// Check if an operation is a performance hotspot
    fn check_for_hotspot(&self, operation_id: &str, duration_ms: f64) {
        if duration_ms > 16.0 { // More than one frame at 60fps
            let operation_name = {
                let session = self.current_session.read();
                session.as_ref()
                    .and_then(|s| s.operations.get(operation_id))
                    .map(|op| op.name.clone())
                    .unwrap_or_else(|| "unknown".to_string())
            };

            let severity = if duration_ms > 100.0 {
                HotspotSeverity::Critical
            } else if duration_ms > 33.0 {
                HotspotSeverity::High
            } else if duration_ms > 22.0 {
                HotspotSeverity::Medium
            } else {
                HotspotSeverity::Low
            };

            let hotspot = PerformanceHotspot {
                operation_name: operation_name.clone(),
                hotspot_type: HotspotType::CpuBound,
                severity,
                total_time_ms: duration_ms,
                occurrence_count: 1,
                avg_time_ms: duration_ms,
                description: format!("Operation '{}' took {:.2}ms", operation_name, duration_ms),
                suggested_fix: "Consider optimizing this operation or moving it to a background thread".to_string(),
            };

            self.hotspots.write().push(hotspot);
        }
    }

    /// Record a memory snapshot
    pub fn record_memory_snapshot(&self, snapshot: MemorySnapshot) {
        let mut history = self.memory_history.write();
        history.push_back(snapshot);
        
        // Update real-time metrics
        self.real_time_metrics.write().memory_usage_mb = history.back()
            .map(|s| s.current_usage as f64 / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        // Trim history
        while history.len() > 1000 {
            history.pop_front();
        }
    }

    /// Record a CPU snapshot
    pub fn record_cpu_snapshot(&self, snapshot: CpuSnapshot) {
        let mut history = self.cpu_history.write();
        history.push_back(snapshot.clone());
        
        // Update real-time metrics
        self.real_time_metrics.write().cpu_usage_percent = snapshot.usage_percent;

        // Trim history
        while history.len() > 1000 {
            history.pop_front();
        }
    }

    /// Record a GPU snapshot
    pub fn record_gpu_snapshot(&self, snapshot: GpuSnapshot) {
        let mut history = self.gpu_history.write();
        history.push_back(snapshot.clone());
        
        // Update real-time metrics
        self.real_time_metrics.write().gpu_usage_percent = snapshot.utilization_percent;

        // Trim history
        while history.len() > 1000 {
            history.pop_front();
        }
    }

    /// Update frame metrics
    pub fn update_frame_metrics(&self, frame_time_ms: f64) {
        let mut metrics = self.real_time_metrics.write();
        metrics.frame_time_ms = frame_time_ms;
        metrics.fps = if frame_time_ms > 0.0 { 1000.0 / frame_time_ms } else { 0.0 };
        metrics.last_update = Self::current_time();
    }

    /// Get current real-time metrics
    pub fn get_real_time_metrics(&self) -> RealTimeMetrics {
        self.real_time_metrics.read().clone()
    }

    /// Get current hotspots
    pub fn get_hotspots(&self) -> Vec<PerformanceHotspot> {
        self.hotspots.read().clone()
    }

    /// Get operation statistics
    pub fn get_operation_stats(&self) -> HashMap<String, OperationStats> {
        self.operation_stats.read().clone()
    }

    /// Generate a performance report
    pub fn generate_report(&self) -> PerformanceReport {
        let session = self.current_session.read().clone();
        let stats = self.operation_stats.read().clone();
        let hotspots = self.hotspots.read().clone();

        let (session_id, duration_seconds, total_operations) = session
            .map(|s| {
                let duration = s.end_time
                    .unwrap_or_else(Self::current_time) - s.start_time;
                (s.id, duration, s.operations.len() as u64)
            })
            .unwrap_or_else(|| (String::new(), 0.0, 0));

        // Find slowest and most frequent operations
        let slowest_operation = stats.values()
            .max_by(|a, b| a.avg_time_ms.partial_cmp(&b.avg_time_ms).unwrap_or(std::cmp::Ordering::Equal))
            .map(|s| s.operation_name.clone());

        let most_frequent_operation = stats.values()
            .max_by_key(|s| s.total_calls)
            .map(|s| s.operation_name.clone());

        // Generate recommendations based on hotspots
        let recommendations: Vec<String> = hotspots.iter()
            .map(|h| h.suggested_fix.clone())
            .take(5)
            .collect();

        let summary = PerformanceSummary {
            avg_fps: self.real_time_metrics.read().fps,
            min_fps: 0.0, // Would need frame history
            max_fps: 60.0,
            avg_frame_time_ms: self.real_time_metrics.read().frame_time_ms,
            total_cpu_time_ms: stats.values().map(|s| s.total_time_ms).sum(),
            total_gpu_time_ms: 0.0,
            memory_peak_mb: self.memory_history.read().iter()
                .map(|s| s.peak_usage as f64 / (1024.0 * 1024.0))
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(0.0),
            slowest_operation,
            most_frequent_operation,
        };

        PerformanceReport {
            session_id,
            duration_seconds,
            total_operations,
            operation_stats: stats,
            hotspots,
            recommendations,
            summary,
        }
    }

    /// Clear all profiling data
    pub fn clear(&self) {
        *self.current_session.write() = None;
        self.operation_history.write().clear();
        self.memory_history.write().clear();
        self.cpu_history.write().clear();
        self.gpu_history.write().clear();
        self.operation_stats.write().clear();
        self.hotspots.write().clear();
        self.active_operations.write().clear();
        *self.real_time_metrics.write() = RealTimeMetrics::default();
    }

    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiling_dashboard_creation() {
        let dashboard = ProfilingDashboard::new(DashboardConfig::default());
        let session_id = dashboard.start_session("test_session");
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_operation_profiling() {
        let dashboard = ProfilingDashboard::new(DashboardConfig::default());
        dashboard.start_session("test");
        
        let op_id = dashboard.begin_operation("test_op", OperationCategory::Rendering);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let duration = dashboard.end_operation(&op_id).unwrap();
        
        assert!(duration >= 10.0);
        
        let stats = dashboard.get_operation_stats();
        assert!(stats.contains_key("test_op"));
    }

    #[test]
    fn test_real_time_metrics() {
        let dashboard = ProfilingDashboard::new(DashboardConfig::default());
        
        dashboard.update_frame_metrics(16.67);
        let metrics = dashboard.get_real_time_metrics();
        
        assert!((metrics.fps - 60.0).abs() < 1.0);
        assert!((metrics.frame_time_ms - 16.67).abs() < 0.1);
    }
}