//! Gesture Controls
//! 
//! Provides touch gesture recognition for controlling playback and navigation.

use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, error, debug, warn};
use std::collections::HashMap;

use crate::GestureConfig;

/// Gesture controller
pub struct GestureController {
    is_initialized: Arc<RwLock<bool>>,
    
    /// Configuration
    config: Arc<RwLock<GestureConfig>>,
    
    /// Gesture recognizers
    swipe_recognizer: Arc<RwLock<SwipeRecognizer>>,
    pinch_recognizer: Arc<RwLock<PinchRecognizer>>,
    tap_recognizer: Arc<RwLock<TapRecognizer>>,
    
    /// Gesture callbacks
    callbacks: Arc<RwLock<HashMap<GestureType, GestureCallback>>>,
}

/// Gesture type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GestureType {
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
    PinchIn,
    PinchOut,
    Tap,
    DoubleTap,
    LongPress,
}

/// Gesture callback
pub type GestureCallback = Arc<dyn Fn(GestureEvent) + Send + Sync>;

/// Gesture event
#[derive(Debug, Clone)]
pub struct GestureEvent {
    /// Gesture type
    pub gesture_type: GestureType,
    
    /// Start position
    pub start_position: (f32, f32),
    
    /// End position
    pub end_position: (f32, f32),
    
    /// Velocity
    pub velocity: f32,
    
    /// Duration in milliseconds
    pub duration: u64,
    
    /// Additional data
    pub data: GestureData,
}

/// Gesture data
#[derive(Debug, Clone)]
pub enum GestureData {
    Swipe {
        distance: f32,
        direction: SwipeDirection,
    },
    Pinch {
        scale: f32,
        center: (f32, f32),
    },
    Tap {
        position: (f32, f32),
        tap_count: u32,
    },
    LongPress {
        position: (f32, f32),
        duration: u64,
    },
}

/// Swipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Touch point
#[derive(Debug, Clone, Copy)]
pub struct TouchPoint {
    pub x: f32,
    pub y: f32,
    pub timestamp: u64,
}

/// Swipe recognizer
pub struct SwipeRecognizer {
    /// Touch points
    touch_points: Vec<TouchPoint>,
    
    /// Minimum swipe distance
    min_distance: f32,
    
    /// Maximum swipe duration in milliseconds
    max_duration: u64,
    
    /// Minimum velocity
    min_velocity: f32,
}

impl SwipeRecognizer {
    /// Create a new swipe recognizer
    pub fn new(config: &GestureConfig) -> Self {
        Self {
            touch_points: Vec::new(),
            min_distance: 50.0 * config.swipe_sensitivity,
            max_duration: 500,
            min_velocity: 100.0 * config.swipe_sensitivity,
        }
    }
    
    /// Add touch point
    pub fn add_touch_point(&mut self, x: f32, y: f32, timestamp: u64) {
        self.touch_points.push(TouchPoint { x, y, timestamp });
    }
    
    /// Clear touch points
    pub fn clear(&mut self) {
        self.touch_points.clear();
    }
    
    /// Recognize swipe
    pub fn recognize(&self) -> Option<GestureEvent> {
        if self.touch_points.len() < 2 {
            return None;
        }
        
        let start = self.touch_points.first()?;
        let end = self.touch_points.last()?;
        
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let duration = end.timestamp - start.timestamp;
        
        // Check minimum distance
        if distance < self.min_distance {
            return None;
        }
        
        // Check maximum duration
        if duration > self.max_duration {
            return None;
        }
        
        // Calculate velocity
        let velocity = distance / (duration as f32 / 1000.0);
        if velocity < self.min_velocity {
            return None;
        }
        
        // Determine direction
        let direction = if dx.abs() > dy.abs() {
            if dx > 0.0 {
                SwipeDirection::Right
            } else {
                SwipeDirection::Left
            }
        } else {
            if dy > 0.0 {
                SwipeDirection::Down
            } else {
                SwipeDirection::Up
            }
        };
        
        let gesture_type = match direction {
            SwipeDirection::Left => GestureType::SwipeLeft,
            SwipeDirection::Right => GestureType::SwipeRight,
            SwipeDirection::Up => GestureType::SwipeUp,
            SwipeDirection::Down => GestureType::SwipeDown,
        };
        
        Some(GestureEvent {
            gesture_type,
            start_position: (start.x, start.y),
            end_position: (end.x, end.y),
            velocity,
            duration,
            data: GestureData::Swipe { distance, direction },
        })
    }
}

/// Pinch recognizer
pub struct PinchRecognizer {
    /// Initial distance between touch points
    initial_distance: Option<f32>,
    
    /// Current distance between touch points
    current_distance: Option<f32>,
    
    /// Center point
    center: Option<(f32, f32)>,
    
    /// Start timestamp
    start_timestamp: Option<u64>,
    
    /// Minimum pinch scale change
    min_scale_change: f32,
}

impl PinchRecognizer {
    /// Create a new pinch recognizer
    pub fn new(config: &GestureConfig) -> Self {
        Self {
            initial_distance: None,
            current_distance: None,
            center: None,
            start_timestamp: None,
            min_scale_change: 0.1 * config.pinch_sensitivity,
        }
    }
    
    /// Update with two touch points
    pub fn update(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let distance = (dx * dx + dy * dy).sqrt();
        let center = ((x1 + x2) / 2.0, (y1 + y2) / 2.0);
        
        if self.initial_distance.is_none() {
            self.initial_distance = Some(distance);
            self.center = Some(center);
            self.start_timestamp = Some(timestamp);
        }
        
        self.current_distance = Some(distance);
    }
    
    /// Clear state
    pub fn clear(&mut self) {
        self.initial_distance = None;
        self.current_distance = None;
        self.center = None;
        self.start_timestamp = None;
    }
    
    /// Recognize pinch
    pub fn recognize(&self) -> Option<GestureEvent> {
        let initial = self.initial_distance?;
        let current = self.current_distance?;
        let center = self.center?;
        let start_timestamp = self.start_timestamp?;
        
        let scale = current / initial;
        let scale_change = (scale - 1.0).abs();
        
        // Check minimum scale change
        if scale_change < self.min_scale_change {
            return None;
        }
        
        let gesture_type = if scale > 1.0 {
            GestureType::PinchOut
        } else {
            GestureType::PinchIn
        };
        
        let duration = chrono::Utc::now().timestamp_millis() as u64 - start_timestamp;
        
        Some(GestureEvent {
            gesture_type,
            start_position: center,
            end_position: center,
            velocity: scale_change / (duration as f32 / 1000.0),
            duration,
            data: GestureData::Pinch { scale, center },
        })
    }
}

/// Tap recognizer
pub struct TapRecognizer {
    /// Touch points
    touch_points: Vec<TouchPoint>,
    
    /// Maximum tap duration in milliseconds
    max_duration: u64,
    
    /// Maximum movement distance
    max_movement: f32,
    
    /// Tap count
    tap_count: u32,
    
    /// Last tap timestamp
    last_tap_timestamp: Option<u64>,
}

impl TapRecognizer {
    /// Create a new tap recognizer
    pub fn new(config: &GestureConfig) -> Self {
        Self {
            touch_points: Vec::new(),
            max_duration: config.tap_duration_threshold,
            max_movement: 10.0,
            tap_count: 0,
            last_tap_timestamp: None,
        }
    }
    
    /// Add touch point
    pub fn add_touch_point(&mut self, x: f32, y: f32, timestamp: u64) {
        self.touch_points.push(TouchPoint { x, y, timestamp });
    }
    
    /// Clear touch points
    pub fn clear(&mut self) {
        self.touch_points.clear();
    }
    
    /// Recognize tap
    pub fn recognize(&mut self) -> Option<GestureEvent> {
        if self.touch_points.is_empty() {
            return None;
        }
        
        let start = self.touch_points.first()?;
        let end = self.touch_points.last()?;
        
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let duration = end.timestamp - start.timestamp;
        
        // Check maximum duration
        if duration > self.max_duration {
            return None;
        }
        
        // Check maximum movement
        if distance > self.max_movement {
            return None;
        }
        
        // Check for double tap
        let gesture_type = if let Some(last_timestamp) = self.last_tap_timestamp {
            let time_since_last_tap = start.timestamp - last_timestamp;
            if time_since_last_tap < 300 {
                self.tap_count = 2;
                GestureType::DoubleTap
            } else {
                self.tap_count = 1;
                GestureType::Tap
            }
        } else {
            self.tap_count = 1;
            GestureType::Tap
        };
        
        self.last_tap_timestamp = Some(end.timestamp);
        
        Some(GestureEvent {
            gesture_type,
            start_position: (start.x, start.y),
            end_position: (end.x, end.y),
            velocity: 0.0,
            duration,
            data: GestureData::Tap {
                position: (start.x, start.y),
                tap_count: self.tap_count,
            },
        })
    }
}

impl GestureController {
    /// Create a new gesture controller
    pub fn new(config: GestureConfig) -> Result<Self> {
        info!("Initializing gesture controller");
        
        Ok(Self {
            is_initialized: Arc::new(RwLock::new(true)),
            config: Arc::new(RwLock::new(config)),
            swipe_recognizer: Arc::new(RwLock::new(SwipeRecognizer::new(&config))),
            pinch_recognizer: Arc::new(RwLock::new(PinchRecognizer::new(&config))),
            tap_recognizer: Arc::new(RwLock::new(TapRecognizer::new(&config))),
            callbacks: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                *self.is_initialized.read().await
            })
        })
    }
    
    /// Set configuration
    pub async fn set_config(&self, config: GestureConfig) -> Result<()> {
        *self.config.write().await = config;
        info!("Gesture configuration updated");
        Ok(())
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> GestureConfig {
        self.config.read().await.clone()
    }
    
    /// Register gesture callback
    pub async fn register_callback(&self, gesture_type: GestureType, callback: GestureCallback) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.insert(gesture_type, callback);
        info!("Registered callback for {:?}", gesture_type);
    }
    
    /// Unregister gesture callback
    pub async fn unregister_callback(&self, gesture_type: GestureType) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.remove(&gesture_type);
        info!("Unregistered callback for {:?}", gesture_type);
    }
    
    /// Handle touch start
    pub async fn handle_touch_start(&self, x: f32, y: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        if config.enable_swipe {
            let mut recognizer = self.swipe_recognizer.write().await;
            recognizer.clear();
            recognizer.add_touch_point(x, y, timestamp);
        }
        
        if config.enable_tap {
            let mut recognizer = self.tap_recognizer.write().await;
            recognizer.clear();
            recognizer.add_touch_point(x, y, timestamp);
        }
        
        Ok(())
    }
    
    /// Handle touch move
    pub async fn handle_touch_move(&self, x: f32, y: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        if config.enable_swipe {
            let mut recognizer = self.swipe_recognizer.write().await;
            recognizer.add_touch_point(x, y, timestamp);
        }
        
        if config.enable_tap {
            let mut recognizer = self.tap_recognizer.write().await;
            recognizer.add_touch_point(x, y, timestamp);
        }
        
        Ok(())
    }
    
    /// Handle touch end
    pub async fn handle_touch_end(&self, x: f32, y: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        // Recognize swipe
        if config.enable_swipe {
            let recognizer = self.swipe_recognizer.write().await;
            if let Some(event) = recognizer.recognize() {
                self.dispatch_event(event).await;
            }
        }
        
        // Recognize tap
        if config.enable_tap {
            let mut recognizer = self.tap_recognizer.write().await;
            recognizer.add_touch_point(x, y, timestamp);
            if let Some(event) = recognizer.recognize() {
                self.dispatch_event(event).await;
            }
        }
        
        Ok(())
    }
    
    /// Handle pinch start
    pub async fn handle_pinch_start(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        if config.enable_pinch {
            let mut recognizer = self.pinch_recognizer.write().await;
            recognizer.clear();
            recognizer.update(x1, y1, x2, y2, timestamp);
        }
        
        Ok(())
    }
    
    /// Handle pinch move
    pub async fn handle_pinch_move(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        if config.enable_pinch {
            let mut recognizer = self.pinch_recognizer.write().await;
            recognizer.update(x1, y1, x2, y2, timestamp);
        }
        
        Ok(())
    }
    
    /// Handle pinch end
    pub async fn handle_pinch_end(&self, x1: f32, y1: f32, x2: f32, y2: f32, timestamp: u64) -> Result<()> {
        let config = self.config.read().await;
        
        if config.enable_pinch {
            let mut recognizer = self.pinch_recognizer.write().await;
            recognizer.update(x1, y1, x2, y2, timestamp);
            if let Some(event) = recognizer.recognize() {
                self.dispatch_event(event).await;
            }
            recognizer.clear();
        }
        
        Ok(())
    }
    
    /// Dispatch gesture event
    async fn dispatch_event(&self, event: GestureEvent) {
        let callbacks = self.callbacks.read().await;
        if let Some(callback) = callbacks.get(&event.gesture_type) {
            callback(event);
        }
    }
    
    /// Get configuration
    pub async fn get_config(&self) -> GestureConfig {
        let config = self.config.read().await;
        config.clone()
    }
    
    /// Set configuration
    pub async fn set_config(&self, config: GestureConfig) -> Result<()> {
        let mut current_config = self.config.write().await;
        *current_config = config;
        Ok(())
    }
    
    /// Export gesture configuration to JSON
    pub async fn export_config(&self) -> Result<String> {
        let config = self.config.read().await;
        serde_json::to_string_pretty(&*config)
            .context("Failed to serialize config to JSON")
    }
    
    /// Import gesture configuration from JSON
    pub async fn import_config(&self, json: &str) -> Result<()> {
        let config: GestureConfig = serde_json::from_str(json)
            .context("Failed to parse config JSON")?;
        
        self.set_config(config).await
    }
}

/// Gesture preset
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GesturePreset {
    /// Preset name
    pub name: String,
    
    /// Preset description
    pub description: String,
    
    /// Swipe sensitivity
    pub swipe_sensitivity: f32,
    
    /// Tap sensitivity
    pub tap_sensitivity: f32,
    
    /// Minimum swipe distance
    pub min_swipe_distance: u32,
    
    /// Tap duration threshold
    pub tap_duration_threshold: u32,
}

impl Default for GesturePreset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default gesture configuration".to_string(),
            swipe_sensitivity: 0.7,
            tap_sensitivity: 0.8,
            min_swipe_distance: 50,
            tap_duration_threshold: 300,
        }
    }
}

/// Gesture preset manager
pub struct GesturePresetManager {
    presets: Arc<RwLock<HashMap<String, GesturePreset>>>,
}

impl GesturePresetManager {
    /// Create a new preset manager
    pub fn new() -> Self {
        info!("👋 Initializing Gesture Preset Manager");
        
        let presets = Self::create_default_presets();
        
        Self {
            presets: Arc::new(RwLock::new(presets)),
        }
    }
    
    /// Create default presets
    fn create_default_presets() -> HashMap<String, GesturePreset> {
        let mut presets = HashMap::new();
        
        // Sensitive preset
        presets.insert(
            "sensitive".to_string(),
            GesturePreset {
                name: "Sensitive".to_string(),
                description: "High sensitivity for quick gestures".to_string(),
                swipe_sensitivity: 0.9,
                tap_sensitivity: 0.95,
                min_swipe_distance: 30,
                tap_duration_threshold: 200,
            },
        );
        
        // Normal preset
        presets.insert(
            "normal".to_string(),
            GesturePreset {
                name: "Normal".to_string(),
                description: "Normal gesture sensitivity".to_string(),
                swipe_sensitivity: 0.7,
                tap_sensitivity: 0.8,
                min_swipe_distance: 50,
                tap_duration_threshold: 300,
            },
        );
        
        // Relaxed preset
        presets.insert(
            "relaxed".to_string(),
            GesturePreset {
                name: "Relaxed".to_string(),
                description: "Low sensitivity for casual use".to_string(),
                swipe_sensitivity: 0.5,
                tap_sensitivity: 0.6,
                min_swipe_distance: 80,
                tap_duration_threshold: 400,
            },
        );
        
        presets
    }
    
    /// Get all presets
    pub async fn get_presets(&self) -> Vec<GesturePreset> {
        let presets = self.presets.read().await;
        presets.values().cloned().collect()
    }
    
    /// Get preset by name
    pub async fn get_preset(&self, name: &str) -> Option<GesturePreset> {
        let presets = self.presets.read().await;
        presets.get(name).cloned()
    }
    
    /// Apply preset to controller
    pub async fn apply_preset(&self, name: &str, controller: &GestureController) -> Result<()> {
        let preset = self.get_preset(name).await
            .context(format!("Preset '{}' not found", name))?;
        
        let config = GestureConfig {
            swipe_sensitivity: preset.swipe_sensitivity,
            tap_sensitivity: preset.tap_sensitivity,
            min_swipe_distance: preset.min_swipe_distance,
            tap_duration_threshold: preset.tap_duration_threshold,
            ..Default::default()
        };
        
        controller.set_config(config).await?;
        
        info!("👋 Applied preset: {}", name);
        Ok(())
    }
    
    /// Add custom preset
    pub async fn add_preset(&self, name: String, preset: GesturePreset) -> Result<()> {
        let mut presets = self.presets.write().await;
        presets.insert(name.clone(), preset);
        
        info!("👋 Added preset: {}", name);
        Ok(())
    }
    
    /// Remove preset
    pub async fn remove_preset(&self, name: &str) -> Result<()> {
        let mut presets = self.presets.write().await;
        if presets.remove(name).is_some() {
            info!("👋 Removed preset: {}", name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Preset '{}' not found", name))
        }
    }
    
    /// Export preset to JSON
    pub async fn export_preset(&self, name: &str) -> Result<String> {
        let preset = self.get_preset(name).await
            .context(format!("Preset '{}' not found", name))?;
        
        serde_json::to_string_pretty(&preset)
            .context("Failed to serialize preset to JSON")
    }
    
    /// Import preset from JSON
    pub async fn import_preset(&self, name: String, json: &str) -> Result<()> {
        let preset: GesturePreset = serde_json::from_str(json)
            .context("Failed to parse preset JSON")?;
        
        self.add_preset(name, preset).await
    }
}

impl Default for GesturePresetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_swipe_recognition() {
        let config = GestureConfig::default();
        let controller = GestureController::new(config).unwrap();
        
        // Simulate swipe right
        controller.handle_touch_start(100.0, 100.0, 0).await.unwrap();
        controller.handle_touch_move(200.0, 100.0, 100).await.unwrap();
        controller.handle_touch_move(300.0, 100.0, 200).await.unwrap();
        controller.handle_touch_end(400.0, 100.0, 300).await.unwrap();
        
        // Swipe should be recognized
        // (In real implementation, callback would be triggered)
    }
    
    #[tokio::test]
    async fn test_tap_recognition() {
        let config = GestureConfig::default();
        let controller = GestureController::new(config).unwrap();
        
        // Simulate tap
        controller.handle_touch_start(100.0, 100.0, 0).await.unwrap();
        controller.handle_touch_end(100.0, 100.0, 100).await.unwrap();
        
        // Tap should be recognized
        // (In real implementation, callback would be triggered)
    }
    
    #[tokio::test]
    async fn test_preset_manager() {
        let manager = GesturePresetManager::new();
        let presets = manager.get_presets().await;
        
        assert_eq!(presets.len(), 3);
        assert!(presets.iter().any(|p| p.name == "Normal"));
    }
    
    #[tokio::test]
    async fn test_apply_preset() {
        let preset_manager = GesturePresetManager::new();
        let config = GestureConfig::default();
        let controller = GestureController::new(config).unwrap();
        
        assert!(preset_manager.apply_preset("sensitive", &controller).await.is_ok());
        
        let applied_config = controller.get_config().await;
        assert_eq!(applied_config.swipe_sensitivity, 0.9);
    }
    
    #[tokio::test]
    async fn test_add_custom_preset() {
        let manager = GesturePresetManager::new();
        
        let custom_preset = GesturePreset {
            name: "Custom".to_string(),
            description: "Custom preset".to_string(),
            swipe_sensitivity: 0.75,
            tap_sensitivity: 0.85,
            min_swipe_distance: 60,
            tap_duration_threshold: 350,
        };
        
        assert!(manager.add_preset("my_custom".to_string(), custom_preset).await.is_ok());
        
        let loaded = manager.get_preset("my_custom").await;
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Custom");
    }
    
    #[tokio::test]
    async fn test_export_import_config() {
        let config = GestureConfig::default();
        let controller = GestureController::new(config).unwrap();
        
        let json = controller.export_config().await;
        assert!(json.is_ok());
        
        // Modify and import back
        let modified_json = json.unwrap();
        assert!(controller.import_config(&modified_json).await.is_ok());
    }
    
    #[tokio::test]
    async fn test_export_import_preset() {
        let manager = GesturePresetManager::new();
        
        let json = manager.export_preset("normal").await;
        assert!(json.is_ok());
        
        let loaded = manager.import_preset("imported_normal".to_string(), &json.unwrap()).await;
        assert!(loaded.is_ok());
    }
}