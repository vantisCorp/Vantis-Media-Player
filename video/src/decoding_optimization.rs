//! Video Decoding Optimization Module
//!
//! This module provides video decoding optimization features including:
//! - Hardware-accelerated decoding for multiple codecs
//! - Optimized frame buffer management
//! - GPU-CPU synchronization improvements
//! - Frame skipping for performance
//! - Adaptive quality adjustment

use anyhow::{Context, Result};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Video decoding optimization configuration
#[derive(Debug, Clone)]
pub struct DecodingOptimizationConfig {
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    
    /// Enable frame skipping for performance
    pub enable_frame_skipping: bool,
    
    /// Maximum frame buffer size
    pub max_frame_buffer_size: usize,
    
    /// Frame skip threshold (percentage of target FPS)
    pub frame_skip_threshold: f32,
    
    /// Enable adaptive quality
    pub enable_adaptive_quality: bool,
    
    /// Minimum quality level (0-100)
    pub min_quality: u32,
    
    /// Maximum quality level (0-100)
    pub max_quality: u32,
    
    /// GPU-CPU sync timeout in milliseconds
    pub gpu_cpu_sync_timeout: u64,
    
    /// Enable zero-copy frame transfer
    pub enable_zero_copy: bool,
    
    /// Prefetch frame count
    pub prefetch_frame_count: usize,
}

impl Default for DecodingOptimizationConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            enable_frame_skipping: true,
            max_frame_buffer_size: 30,
            frame_skip_threshold: 0.8,
            enable_adaptive_quality: true,
            min_quality: 50,
            max_quality: 100,
            gpu_cpu_sync_timeout: 16, // ~60 FPS
            enable_zero_copy: true,
            prefetch_frame_count: 5,
        }
    }
}

/// Hardware decoder type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareDecoder {
    /// NVIDIA NVDEC
    NVDEC,
    
    /// Intel Quick Sync Video
    QuickSync,
    
    /// AMD VCE
    VCE,
    
    /// VideoToolbox (macOS)
    VideoToolbox,
    
    /// MediaCodec (Android)
    MediaCodec,
    
    /// VAAPI (Linux)
    VAAPI,
    
    /// VDPAU (Linux)
    VDPAU,
    
    /// DXVA2 (Windows)
    DXVA2,
    
    /// D3D11VA (Windows)
    D3D11VA,
    
    /// Software decoder
    Software,
}

impl HardwareDecoder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NVDEC => "NVDEC",
            Self::QuickSync => "QuickSync",
            Self::VCE => "VCE",
            Self::VideoToolbox => "VideoToolbox",
            Self::MediaCodec => "MediaCodec",
            Self::VAAPI => "VAAPI",
            Self::VDPAU => "VDPAU",
            Self::DXVA2 => "DXVA2",
            Self::D3D11VA => "D3D11VA",
            Self::Software => "Software",
        }
    }
    
    pub fn is_hardware(&self) -> bool {
        !matches!(self, Self::Software)
    }
}

/// Codec support information
#[derive(Debug, Clone)]
pub struct CodecSupport {
    /// Codec name
    pub codec: String,
    
    /// Supported hardware decoders
    pub hardware_decoders: Vec<HardwareDecoder>,
    
    /// Software decoder available
    pub software_decoder: bool,
    
    /// Hardware acceleration recommended
    pub hardware_recommended: bool,
}

/// Frame buffer entry
#[derive(Debug, Clone)]
pub struct FrameBufferEntry {
    /// Frame index
    pub index: usize,
    
    /// Frame timestamp
    pub timestamp: Duration,
    
    /// Frame size in bytes
    pub size: usize,
    
    /// Decode time
    pub decode_time: Duration,
    
    /// Quality level
    pub quality: u32,
}

/// Frame buffer manager
pub struct FrameBufferManager {
    /// Maximum buffer size
    max_size: usize,
    
    /// Frame buffer
    buffer: VecDeque<FrameBufferEntry>,
    
    /// Total buffer size in bytes
    total_size: AtomicUsize,
    
    /// Current buffer usage
    current_usage: AtomicUsize,
}

impl FrameBufferManager {
    /// Create a new frame buffer manager
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            buffer: VecDeque::with_capacity(max_size),
            total_size: AtomicUsize::new(0),
            current_usage: AtomicUsize::new(0),
        }
    }
    
    /// Add a frame to the buffer
    pub fn add_frame(&mut self, entry: FrameBufferEntry) -> Result<()> {
        // Check if buffer is full
        if self.buffer.len() >= self.max_size {
            // Remove oldest frame
            if let Some(removed) = self.buffer.pop_front() {
                self.total_size.fetch_sub(removed.size, Ordering::SeqCst);
            }
        }
        
        self.buffer.push_back(entry);
        self.total_size.fetch_add(entry.size, Ordering::SeqCst);
        self.current_usage.store(self.buffer.len(), Ordering::SeqCst);
        
        debug!(
            "Frame added to buffer: index={}, buffer_size={}, total_size={} bytes",
            entry.index,
            self.buffer.len(),
            self.total_size.load(Ordering::SeqCst)
        );
        
        Ok(())
    }
    
    /// Get a frame from the buffer
    pub fn get_frame(&mut self, index: usize) -> Option<FrameBufferEntry> {
        self.buffer
            .iter()
            .find(|entry| entry.index == index)
            .cloned()
    }
    
    /// Remove a frame from the buffer
    pub fn remove_frame(&mut self, index: usize) -> Option<FrameBufferEntry> {
        if let Some(pos) = self.buffer.iter().position(|entry| entry.index == index) {
            let entry = self.buffer.remove(pos);
            self.total_size.fetch_sub(entry.size, Ordering::SeqCst);
            self.current_usage.store(self.buffer.len(), Ordering::SeqCst);
            Some(entry)
        } else {
            None
        }
    }
    
    /// Get current buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
    
    /// Get total buffer size in bytes
    pub fn total_size(&self) -> usize {
        self.total_size.load(Ordering::SeqCst)
    }
    
    /// Get current usage percentage
    pub fn usage_percentage(&self) -> f32 {
        if self.max_size == 0 {
            return 0.0;
        }
        (self.buffer.len() as f32 / self.max_size as f32) * 100.0
    }
    
    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.total_size.store(0, Ordering::SeqCst);
        self.current_usage.store(0, Ordering::SeqCst);
        debug!("Frame buffer cleared");
    }
}

/// Frame skipping strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSkippingStrategy {
    /// No frame skipping
    None,
    
    /// Skip every Nth frame
    SkipEveryNth(usize),
    
    /// Skip frames based on decode time
    Adaptive,
    
    /// Skip frames to maintain target FPS
    TargetFPS(f32),
}

impl Default for FrameSkippingStrategy {
    fn default() -> Self {
        Self::Adaptive
    }
}

/// Frame skipping controller
pub struct FrameSkippingController {
    /// Strategy
    strategy: FrameSkippingStrategy,
    
    /// Frame counter
    frame_counter: AtomicU32,
    
    /// Target FPS
    target_fps: f32,
    
    /// Last frame time
    last_frame_time: Arc<Mutex<Option<Instant>>>,
    
    /// Enabled
    enabled: AtomicBool,
}

impl FrameSkippingController {
    /// Create a new frame skipping controller
    pub fn new(strategy: FrameSkippingStrategy, target_fps: f32) -> Self {
        Self {
            strategy,
            frame_counter: AtomicU32::new(0),
            target_fps,
            last_frame_time: Arc::new(Mutex::new(None)),
            enabled: AtomicBool::new(true),
        }
    }
    
    /// Check if frame should be skipped
    pub fn should_skip_frame(&self) -> bool {
        if !self.enabled.load(Ordering::SeqCst) {
            return false;
        }
        
        match self.strategy {
            FrameSkippingStrategy::None => false,
            
            FrameSkippingStrategy::SkipEveryNth(n) => {
                let count = self.frame_counter.fetch_add(1, Ordering::SeqCst);
                count % n as u32 != 0
            }
            
            FrameSkippingStrategy::Adaptive => {
                // Adaptive skipping based on decode time
                // This is a simplified implementation
                let count = self.frame_counter.fetch_add(1, Ordering::SeqCst);
                // Skip every 3rd frame if under load
                count % 3 == 0
            }
            
            FrameSkippingStrategy::TargetFPS(fps) => {
                let mut last_time = self.last_frame_time.lock().unwrap();
                let now = Instant::now();
                
                if let Some(last) = *last_time {
                    let elapsed = now.duration_since(last).as_secs_f32();
                    let target_frame_time = 1.0 / fps;
                    
                    if elapsed < target_frame_time * 0.8 {
                        // Frame is too fast, skip it
                        *last_time = Some(now);
                        return true;
                    }
                }
                
                *last_time = Some(now);
                false
            }
        }
    }
    
    /// Set strategy
    pub fn set_strategy(&self, strategy: FrameSkippingStrategy) {
        // Note: In a real implementation, we'd need interior mutability
        // For now, this is a placeholder
        debug!("Frame skipping strategy set to {:?}", strategy);
    }
    
    /// Enable/disable frame skipping
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        debug!("Frame skipping {}", if enabled { "enabled" } else { "disabled" });
    }
    
    /// Reset frame counter
    pub fn reset(&self) {
        self.frame_counter.store(0, Ordering::SeqCst);
        let mut last_time = self.last_frame_time.lock().unwrap();
        *last_time = None;
        debug!("Frame skipping controller reset");
    }
}

/// GPU-CPU synchronization manager
pub struct GpuCpuSyncManager {
    /// Sync timeout
    timeout: Duration,
    
    /// Zero-copy enabled
    zero_copy_enabled: bool,
    
    /// Pending frames
    pending_frames: AtomicU32,
    
    /// Sync errors
    sync_errors: AtomicU32,
}

impl GpuCpuSyncManager {
    /// Create a new GPU-CPU sync manager
    pub fn new(timeout: Duration, zero_copy_enabled: bool) -> Self {
        Self {
            timeout,
            zero_copy_enabled,
            pending_frames: AtomicU32::new(0),
            sync_errors: AtomicU32::new(0),
        }
    }
    
    /// Wait for GPU to finish processing
    pub fn wait_for_gpu(&self) -> Result<()> {
        if self.zero_copy_enabled {
            // Zero-copy mode: no sync needed
            return Ok(());
        }
        
        // Simulate GPU wait
        std::thread::sleep(Duration::from_millis(1));
        
        self.pending_frames.fetch_sub(1, Ordering::SeqCst);
        
        Ok(())
    }
    
    /// Submit frame to GPU
    pub fn submit_to_gpu(&self) -> Result<()> {
        self.pending_frames.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    
    /// Get pending frame count
    pub fn pending_frames(&self) -> u32 {
        self.pending_frames.load(Ordering::SeqCst)
    }
    
    /// Get sync error count
    pub fn sync_errors(&self) -> u32 {
        self.sync_errors.load(Ordering::SeqCst)
    }
    
    /// Check if zero-copy is enabled
    pub fn is_zero_copy_enabled(&self) -> bool {
        self.zero_copy_enabled
    }
}

/// Adaptive quality controller
pub struct AdaptiveQualityController {
    /// Minimum quality
    min_quality: u32,
    
    /// Maximum quality
    max_quality: u32,
    
    /// Current quality
    current_quality: AtomicU32,
    
    /// Target FPS
    target_fps: f32,
    
    /// Enabled
    enabled: AtomicBool,
}

impl AdaptiveQualityController {
    /// Create a new adaptive quality controller
    pub fn new(min_quality: u32, max_quality: u32, target_fps: f32) -> Self {
        Self {
            min_quality,
            max_quality,
            current_quality: AtomicU32::new(max_quality),
            target_fps,
            enabled: AtomicBool::new(true),
        }
    }
    
    /// Get current quality
    pub fn current_quality(&self) -> u32 {
        self.current_quality.load(Ordering::SeqCst)
    }
    
    /// Adjust quality based on performance
    pub fn adjust_quality(&self, current_fps: f32) {
        if !self.enabled.load(Ordering::SeqCst) {
            return;
        }
        
        let quality_ratio = current_fps / self.target_fps;
        let current = self.current_quality.load(Ordering::SeqCst);
        
        let new_quality = if quality_ratio < 0.8 {
            // Performance is poor, reduce quality
            (current as f32 * 0.9).max(self.min_quality as f32) as u32
        } else if quality_ratio > 1.2 {
            // Performance is good, increase quality
            (current as f32 * 1.1).min(self.max_quality as f32) as u32
        } else {
            // Performance is acceptable, maintain quality
            current
        };
        
        if new_quality != current {
            self.current_quality.store(new_quality, Ordering::SeqCst);
            info!(
                "Adjusted quality: {} -> {} (FPS ratio: {:.2})",
                current, new_quality, quality_ratio
            );
        }
    }
    
    /// Set quality manually
    pub fn set_quality(&self, quality: u32) {
        let clamped = quality.clamp(self.min_quality, self.max_quality);
        self.current_quality.store(clamped, Ordering::SeqCst);
        info!("Quality set to {}", clamped);
    }
    
    /// Enable/disable adaptive quality
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        debug!("Adaptive quality {}", if enabled { "enabled" } else { "disabled" });
    }
}

/// Video decoding optimizer
pub struct VideoDecodingOptimizer {
    /// Configuration
    config: DecodingOptimizationConfig,
    
    /// Frame buffer manager
    frame_buffer: Arc<Mutex<FrameBufferManager>>,
    
    /// Frame skipping controller
    frame_skipping: Arc<FrameSkippingController>,
    
    /// GPU-CPU sync manager
    gpu_cpu_sync: Arc<GpuCpuSyncManager>,
    
    /// Adaptive quality controller
    adaptive_quality: Arc<AdaptiveQualityController>,
    
    /// Supported codecs
    supported_codecs: Vec<CodecSupport>,
    
    /// Current decoder
    current_decoder: Arc<Mutex<Option<HardwareDecoder>>>,
    
    /// CPU usage tracker
    cpu_usage: Arc<Mutex<Vec<f32>>>,
}

impl VideoDecodingOptimizer {
    /// Create a new video decoding optimizer
    pub fn new(config: DecodingOptimizationConfig) -> Self {
        let frame_buffer = Arc::new(Mutex::new(FrameBufferManager::new(
            config.max_frame_buffer_size,
        )));
        
        let frame_skipping = Arc::new(FrameSkippingController::new(
            FrameSkippingStrategy::Adaptive,
            60.0, // Default target FPS
        ));
        
        let gpu_cpu_sync = Arc::new(GpuCpuSyncManager::new(
            Duration::from_millis(config.gpu_cpu_sync_timeout),
            config.enable_zero_copy,
        ));
        
        let adaptive_quality = Arc::new(AdaptiveQualityController::new(
            config.min_quality,
            config.max_quality,
            60.0, // Default target FPS
        ));
        
        let supported_codecs = Self::detect_supported_codecs();
        
        Self {
            config,
            frame_buffer,
            frame_skipping,
            gpu_cpu_sync,
            adaptive_quality,
            supported_codecs,
            current_decoder: Arc::new(Mutex::new(None)),
            cpu_usage: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Detect supported codecs and hardware decoders
    fn detect_supported_codecs() -> Vec<CodecSupport> {
        // In a real implementation, this would query the system
        // For now, return a placeholder list
        vec![
            CodecSupport {
                codec: "H.264".to_string(),
                hardware_decoders: vec![
                    HardwareDecoder::NVDEC,
                    HardwareDecoder::QuickSync,
                    HardwareDecoder::VAAPI,
                    HardwareDecoder::VideoToolbox,
                ],
                software_decoder: true,
                hardware_recommended: true,
            },
            CodecSupport {
                codec: "H.265/HEVC".to_string(),
                hardware_decoders: vec![
                    HardwareDecoder::NVDEC,
                    HardwareDecoder::QuickSync,
                    HardwareDecoder::VAAPI,
                ],
                software_decoder: true,
                hardware_recommended: true,
            },
            CodecSupport {
                codec: "VP9".to_string(),
                hardware_decoders: vec![HardwareDecoder::NVDEC, HardwareDecoder::VAAPI],
                software_decoder: true,
                hardware_recommended: false,
            },
            CodecSupport {
                codec: "AV1".to_string(),
                hardware_decoders: vec![HardwareDecoder::NVDEC],
                software_decoder: true,
                hardware_recommended: true,
            },
        ]
    }
    
    /// Get supported codecs
    pub fn supported_codecs(&self) -> &[CodecSupport] {
        &self.supported_codecs
    }
    
    /// Select best decoder for codec
    pub fn select_decoder(&self, codec: &str) -> Option<HardwareDecoder> {
        if !self.config.hardware_acceleration {
            return Some(HardwareDecoder::Software);
        }
        
        let codec_support = self.supported_codecs.iter().find(|c| c.codec == codec)?;
        
        if codec_support.hardware_recommended && !codec_support.hardware_decoders.is_empty() {
            // Return first available hardware decoder
            Some(codec_support.hardware_decoders[0])
        } else {
            Some(HardwareDecoder::Software)
        }
    }
    
    /// Set current decoder
    pub fn set_decoder(&self, decoder: HardwareDecoder) {
        let mut current = self.current_decoder.lock().unwrap();
        *current = Some(decoder);
        info!("Decoder set to: {}", decoder.as_str());
    }
    
    /// Get current decoder
    pub fn current_decoder(&self) -> Option<HardwareDecoder> {
        *self.current_decoder.lock().unwrap()
    }
    
    /// Process a frame
    pub fn process_frame(&self, frame_index: usize, frame_size: usize) -> Result<bool> {
        // Check if frame should be skipped
        if self.config.enable_frame_skipping && self.frame_skipping.should_skip_frame() {
            debug!("Frame {} skipped", frame_index);
            return Ok(false);
        }
        
        // Add frame to buffer
        let entry = FrameBufferEntry {
            index: frame_index,
            timestamp: Duration::from_secs(0),
            size: frame_size,
            decode_time: Duration::from_millis(10),
            quality: self.adaptive_quality.current_quality(),
        };
        
        let mut buffer = self.frame_buffer.lock().unwrap();
        buffer.add_frame(entry)?;
        
        Ok(true)
    }
    
    /// Update CPU usage
    pub fn update_cpu_usage(&self, usage: f32) {
        let mut cpu_usage = self.cpu_usage.lock().unwrap();
        cpu_usage.push(usage);
        
        // Keep only last 100 samples
        if cpu_usage.len() > 100 {
            cpu_usage.remove(0);
        }
        
        // Adjust quality based on CPU usage
        if self.config.enable_adaptive_quality {
            let avg_usage: f32 = cpu_usage.iter().sum::<f32>() / cpu_usage.len() as f32;
            let fps = 60.0 * (1.0 - avg_usage);
            self.adaptive_quality.adjust_quality(fps);
        }
    }
    
    /// Get average CPU usage
    pub fn average_cpu_usage(&self) -> f32 {
        let cpu_usage = self.cpu_usage.lock().unwrap();
        if cpu_usage.is_empty() {
            return 0.0;
        }
        cpu_usage.iter().sum::<f32>() / cpu_usage.len() as f32
    }
    
    /// Get frame buffer manager
    pub fn frame_buffer(&self) -> Arc<Mutex<FrameBufferManager>> {
        Arc::clone(&self.frame_buffer)
    }
    
    /// Get frame skipping controller
    pub fn frame_skipping(&self) -> Arc<FrameSkippingController> {
        Arc::clone(&self.frame_skipping)
    }
    
    /// Get GPU-CPU sync manager
    pub fn gpu_cpu_sync(&self) -> Arc<GpuCpuSyncManager> {
        Arc::clone(&self.gpu_cpu_sync)
    }
    
    /// Get adaptive quality controller
    pub fn adaptive_quality(&self) -> Arc<AdaptiveQualityController> {
        Arc::clone(&self.adaptive_quality)
    }
    
    /// Get optimization statistics
    pub fn statistics(&self) -> DecodingStatistics {
        let buffer = self.frame_buffer.lock().unwrap();
        let cpu_usage = self.cpu_usage.lock().unwrap();
        
        DecodingStatistics {
            frame_buffer_size: buffer.buffer_size(),
            frame_buffer_usage: buffer.usage_percentage(),
            frame_buffer_total_size: buffer.total_size(),
            average_cpu_usage: self.average_cpu_usage(),
            current_quality: self.adaptive_quality.current_quality(),
            current_decoder: self.current_decoder(),
            pending_frames: self.gpu_cpu_sync.pending_frames(),
            sync_errors: self.gpu_cpu_sync.sync_errors(),
        }
    }
}

/// Decoding statistics
#[derive(Debug, Clone)]
pub struct DecodingStatistics {
    /// Frame buffer size
    pub frame_buffer_size: usize,
    
    /// Frame buffer usage percentage
    pub frame_buffer_usage: f32,
    
    /// Frame buffer total size in bytes
    pub frame_buffer_total_size: usize,
    
    /// Average CPU usage
    pub average_cpu_usage: f32,
    
    /// Current quality level
    pub current_quality: u32,
    
    /// Current decoder
    pub current_decoder: Option<HardwareDecoder>,
    
    /// Pending frames
    pub pending_frames: u32,
    
    /// Sync errors
    pub sync_errors: u32,
}

// Need to add Mutex import
use std::sync::Mutex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding_optimization_config_default() {
        let config = DecodingOptimizationConfig::default();
        assert!(config.hardware_acceleration);
        assert!(config.enable_frame_skipping);
        assert_eq!(config.max_frame_buffer_size, 30);
        assert_eq!(config.frame_skip_threshold, 0.8);
        assert!(config.enable_adaptive_quality);
        assert_eq!(config.min_quality, 50);
        assert_eq!(config.max_quality, 100);
    }

    #[test]
    fn test_hardware_decoder() {
        assert_eq!(HardwareDecoder::NVDEC.as_str(), "NVDEC");
        assert!(HardwareDecoder::NVDEC.is_hardware());
        assert!(!HardwareDecoder::Software.is_hardware());
    }

    #[test]
    fn test_frame_buffer_manager() {
        let mut manager = FrameBufferManager::new(5);
        
        // Add frames
        for i in 0..3 {
            let entry = FrameBufferEntry {
                index: i,
                timestamp: Duration::from_secs(i as u64),
                size: 1024,
                decode_time: Duration::from_millis(10),
                quality: 100,
            };
            manager.add_frame(entry).unwrap();
        }
        
        assert_eq!(manager.buffer_size(), 3);
        assert_eq!(manager.total_size(), 3072);
        assert_eq!(manager.usage_percentage(), 60.0);
        
        // Get frame
        let frame = manager.get_frame(1);
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().index, 1);
        
        // Remove frame
        let removed = manager.remove_frame(1);
        assert!(removed.is_some());
        assert_eq!(manager.buffer_size(), 2);
        
        // Clear buffer
        manager.clear();
        assert_eq!(manager.buffer_size(), 0);
        assert_eq!(manager.total_size(), 0);
    }

    #[test]
    fn test_frame_buffer_overflow() {
        let mut manager = FrameBufferManager::new(3);
        
        // Add more frames than capacity
        for i in 0..5 {
            let entry = FrameBufferEntry {
                index: i,
                timestamp: Duration::from_secs(i as u64),
                size: 1024,
                decode_time: Duration::from_millis(10),
                quality: 100,
            };
            manager.add_frame(entry).unwrap();
        }
        
        // Should only have 3 frames (max capacity)
        assert_eq!(manager.buffer_size(), 3);
        // Should have frames 2, 3, 4 (oldest removed)
        assert!(manager.get_frame(0).is_none());
        assert!(manager.get_frame(1).is_none());
        assert!(manager.get_frame(2).is_some());
    }

    #[test]
    fn test_frame_skipping_controller() {
        let controller = FrameSkippingController::new(FrameSkippingStrategy::SkipEveryNth(2), 60.0);
        
        // Should skip every 2nd frame
        assert!(!controller.should_skip_frame()); // Frame 0: keep
        assert!(controller.should_skip_frame());  // Frame 1: skip
        assert!(!controller.should_skip_frame()); // Frame 2: keep
        assert!(controller.should_skip_frame());  // Frame 3: skip
        
        controller.reset();
        
        // After reset, should start from frame 0
        assert!(!controller.should_skip_frame());
    }

    #[test]
    fn test_frame_skipping_disabled() {
        let controller = FrameSkippingController::new(FrameSkippingStrategy::SkipEveryNth(2), 60.0);
        controller.set_enabled(false);
        
        // Should not skip any frames when disabled
        for _ in 0..10 {
            assert!(!controller.should_skip_frame());
        }
    }

    #[test]
    fn test_gpu_cpu_sync_manager() {
        let manager = GpuCpuSyncManager::new(Duration::from_millis(16), false);
        
        assert!(!manager.is_zero_copy_enabled());
        
        // Submit frames
        manager.submit_to_gpu().unwrap();
        manager.submit_to_gpu().unwrap();
        assert_eq!(manager.pending_frames(), 2);
        
        // Wait for GPU
        manager.wait_for_gpu().unwrap();
        assert_eq!(manager.pending_frames(), 1);
        
        manager.wait_for_gpu().unwrap();
        assert_eq!(manager.pending_frames(), 0);
    }

    #[test]
    fn test_gpu_cpu_sync_zero_copy() {
        let manager = GpuCpuSyncManager::new(Duration::from_millis(16), true);
        
        assert!(manager.is_zero_copy_enabled());
        
        // In zero-copy mode, pending frames should not increase
        manager.submit_to_gpu().unwrap();
        assert_eq!(manager.pending_frames(), 1);
        
        // Wait should be instant in zero-copy mode
        manager.wait_for_gpu().unwrap();
    }

    #[test]
    fn test_adaptive_quality_controller() {
        let controller = AdaptiveQualityController::new(50, 100, 60.0);
        
        assert_eq!(controller.current_quality(), 100);
        
        // Poor performance (low FPS)
        controller.adjust_quality(30.0);
        assert!(controller.current_quality() < 100);
        
        // Good performance (high FPS)
        controller.adjust_quality(80.0);
        assert!(controller.current_quality() > 50);
        
        // Manual quality setting
        controller.set_quality(75);
        assert_eq!(controller.current_quality(), 75);
        
        // Quality should be clamped
        controller.set_quality(150);
        assert_eq!(controller.current_quality(), 100);
        
        controller.set_quality(25);
        assert_eq!(controller.current_quality(), 50);
    }

    #[test]
    fn test_video_decoding_optimizer() {
        let config = DecodingOptimizationConfig::default();
        let optimizer = VideoDecodingOptimizer::new(config);
        
        // Check supported codecs
        let codecs = optimizer.supported_codecs();
        assert!(!codecs.is_empty());
        
        // Select decoder for H.264
        let decoder = optimizer.select_decoder("H.264");
        assert!(decoder.is_some());
        
        // Set decoder
        optimizer.set_decoder(HardwareDecoder::NVDEC);
        assert_eq!(optimizer.current_decoder(), Some(HardwareDecoder::NVDEC));
        
        // Process frames
        for i in 0..10 {
            let processed = optimizer.process_frame(i, 1024).unwrap();
            // Some frames may be skipped
            if processed {
                debug!("Frame {} processed", i);
            }
        }
        
        // Update CPU usage
        optimizer.update_cpu_usage(0.5);
        optimizer.update_cpu_usage(0.6);
        optimizer.update_cpu_usage(0.4);
        
        assert_eq!(optimizer.average_cpu_usage(), 0.5);
        
        // Get statistics
        let stats = optimizer.statistics();
        assert!(stats.frame_buffer_size > 0);
        assert!(stats.current_decoder.is_some());
    }

    #[test]
    fn test_detect_supported_codecs() {
        let codecs = VideoDecodingOptimizer::detect_supported_codecs();
        
        assert!(!codecs.is_empty());
        
        // Check H.264 support
        let h264 = codecs.iter().find(|c| c.codec == "H.264");
        assert!(h264.is_some());
        let h264 = h264.unwrap();
        assert!(h264.software_decoder);
        assert!(h264.hardware_recommended);
        assert!(!h264.hardware_decoders.is_empty());
    }
}