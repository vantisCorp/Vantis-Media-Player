//! Device management for cloud sync
//!
//! Manages connected devices, device trust, and cross-device features:
//! - Device registration and discovery
//! - Trust levels and permissions
//! - Device capabilities detection
//! - Session management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::collections::HashMap;
use std::net::IpAddr;
use async_trait::async_trait;

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Unique device ID
    pub id: Uuid,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: DeviceType,
    /// Operating system
    pub os: String,
    /// App version
    pub app_version: String,
    /// Device capabilities
    pub capabilities: DeviceCapabilities,
    /// Trust level
    pub trust_level: TrustLevel,
    /// Registration date
    pub registered_at: DateTime<Utc>,
    /// Last seen
    pub last_seen: DateTime<Utc>,
    /// Is currently online
    pub is_online: bool,
    /// IP address (if known)
    pub ip_address: Option<String>,
    /// Device metadata
    pub metadata: HashMap<String, String>,
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    SmartTV,
    WebPlayer,
    Console,
    Iot,
    Unknown,
}

impl DeviceType {
    /// Check if device is mobile
    pub fn is_mobile(&self) -> bool {
        matches!(self, DeviceType::Mobile | DeviceType::Tablet)
    }

    /// Check if device supports full sync
    pub fn supports_full_sync(&self) -> bool {
        matches!(
            self,
            DeviceType::Desktop | DeviceType::Laptop | DeviceType::Mobile | DeviceType::Tablet
        )
    }

    /// Get default trust level for device type
    pub fn default_trust(&self) -> TrustLevel {
        match self {
            DeviceType::Desktop | DeviceType::Laptop => TrustLevel::Trusted,
            DeviceType::Mobile | DeviceType::Tablet => TrustLevel::Trusted,
            DeviceType::SmartTV => TrustLevel::Standard,
            DeviceType::WebPlayer => TrustLevel::Limited,
            DeviceType::Console => TrustLevel::Standard,
            DeviceType::Iot => TrustLevel::Limited,
            DeviceType::Unknown => TrustLevel::Untrusted,
        }
    }
}

/// Trust level for devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    /// Full access to all data
    Trusted,
    /// Standard access
    Standard,
    /// Limited access
    Limited,
    /// No sync access
    Untrusted,
}

impl TrustLevel {
    /// Check if can sync sensitive data
    pub fn can_sync_sensitive(&self) -> bool {
        matches!(self, TrustLevel::Trusted)
    }

    /// Check if can manage other devices
    pub fn can_manage_devices(&self) -> bool {
        matches!(self, TrustLevel::Trusted)
    }

    /// Check if can initiate sync
    pub fn can_sync(&self) -> bool {
        !matches!(self, TrustLevel::Untrusted)
    }
}

/// Device capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Supports video playback
    pub video_playback: bool,
    /// Supports audio playback
    pub audio_playback: bool,
    /// Supports streaming
    pub streaming: bool,
    /// Supports offline mode
    pub offline_mode: bool,
    /// Supports hardware decoding
    pub hardware_decoding: bool,
    /// Maximum video resolution
    pub max_resolution: Option<u32>,
    /// Supports HDR
    pub hdr_support: bool,
    /// Storage capacity in bytes
    pub storage_capacity: u64,
    /// Available storage
    pub available_storage: u64,
    /// Supports remote control
    pub remote_control: bool,
    /// Supports casting
    pub casting: bool,
}

impl DeviceCapabilities {
    /// Check if device can play content
    pub fn can_play(&self, resolution: u32, hdr: bool) -> bool {
        if !self.video_playback {
            return false;
        }
        
        if let Some(max) = self.max_resolution {
            if resolution > max {
                return false;
            }
        }
        
        if hdr && !self.hdr_support {
            return false;
        }
        
        true
    }

    /// Get capability score (for device ranking)
    pub fn score(&self) -> u32 {
        let mut score = 0u32;
        
        if self.video_playback { score += 10; }
        if self.audio_playback { score += 5; }
        if self.streaming { score += 5; }
        if self.offline_mode { score += 5; }
        if self.hardware_decoding { score += 10; }
        if self.hdr_support { score += 5; }
        if self.remote_control { score += 3; }
        if self.casting { score += 3; }
        
        if let Some(res) = self.max_resolution {
            score += (res / 100).min(20);
        }
        
        score
    }
}

/// Device session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSession {
    /// Session ID
    pub id: Uuid,
    /// Device ID
    pub device_id: Uuid,
    /// Session token
    pub token: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
    /// Is active
    pub is_active: bool,
}

impl DeviceSession {
    /// Create a new session
    pub fn new(device_id: Uuid, duration_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            device_id,
            token: Uuid::new_v4().to_string(),
            created_at: now,
            expires_at: now + Duration::hours(duration_hours),
            last_activity: now,
            is_active: true,
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if session is valid
    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }

    /// Refresh session
    pub fn refresh(&mut self, duration_hours: i64) {
        self.expires_at = Utc::now() + Duration::hours(duration_hours);
        self.last_activity = Utc::now();
    }

    /// Revoke session
    pub fn revoke(&mut self) {
        self.is_active = false;
    }
}

/// Device manager trait
#[async_trait]
pub trait DeviceManager: Send + Sync {
    /// Register a new device
    async fn register_device(&self, info: DeviceRegistration) -> Result<Device>;
    
    /// Unregister a device
    async fn unregister_device(&self, device_id: Uuid) -> Result<()>;
    
    /// Get device by ID
    async fn get_device(&self, device_id: Uuid) -> Result<Option<Device>>;
    
    /// List all devices
    async fn list_devices(&self) -> Result<Vec<Device>>;
    
    /// Update device trust level
    async fn set_trust_level(&self, device_id: Uuid, level: TrustLevel) -> Result<()>;
    
    /// Update device capabilities
    async fn update_capabilities(&self, device_id: Uuid, capabilities: DeviceCapabilities) -> Result<()>;
    
    /// Create session for device
    async fn create_session(&self, device_id: Uuid) -> Result<DeviceSession>;
    
    /// Validate session
    async fn validate_session(&self, token: &str) -> Result<Option<DeviceSession>>;
    
    /// Revoke session
    async fn revoke_session(&self, token: &str) -> Result<()>;
    
    /// Update device last seen
    async fn update_last_seen(&self, device_id: Uuid) -> Result<()>;
}

/// Device registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegistration {
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: DeviceType,
    /// Operating system
    pub os: String,
    /// App version
    pub app_version: String,
    /// Initial capabilities
    pub capabilities: DeviceCapabilities,
    /// IP address
    pub ip_address: Option<String>,
}

/// In-memory device manager implementation
pub struct InMemoryDeviceManager {
    /// Devices
    devices: std::sync::RwLock<HashMap<Uuid, Device>>,
    /// Sessions
    sessions: std::sync::RwLock<HashMap<String, DeviceSession>>,
    /// Session duration in hours
    session_duration: i64,
}

impl InMemoryDeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            devices: std::sync::RwLock::new(HashMap::new()),
            sessions: std::sync::RwLock::new(HashMap::new()),
            session_duration: 24 * 7, // 1 week
        }
    }

    /// Set session duration
    pub fn with_session_duration(mut self, hours: i64) -> Self {
        self.session_duration = hours;
        self
    }
}

impl Default for InMemoryDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DeviceManager for InMemoryDeviceManager {
    async fn register_device(&self, info: DeviceRegistration) -> Result<Device> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let device = Device {
            id,
            name: info.name,
            device_type: info.device_type,
            os: info.os,
            app_version: info.app_version,
            capabilities: info.capabilities,
            trust_level: info.device_type.default_trust(),
            registered_at: now,
            last_seen: now,
            is_online: true,
            ip_address: info.ip_address,
            metadata: HashMap::new(),
        };
        
        self.devices.write().unwrap().insert(id, device.clone());
        
        Ok(device)
    }

    async fn unregister_device(&self, device_id: Uuid) -> Result<()> {
        self.devices.write().unwrap().remove(&device_id);
        
        // Revoke all sessions for this device
        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, s| s.device_id != device_id);
        
        Ok(())
    }

    async fn get_device(&self, device_id: Uuid) -> Result<Option<Device>> {
        Ok(self.devices.read().unwrap().get(&device_id).cloned())
    }

    async fn list_devices(&self) -> Result<Vec<Device>> {
        Ok(self.devices.read().unwrap().values().cloned().collect())
    }

    async fn set_trust_level(&self, device_id: Uuid, level: TrustLevel) -> Result<()> {
        let mut devices = self.devices.write().unwrap();
        if let Some(device) = devices.get_mut(&device_id) {
            device.trust_level = level;
        }
        Ok(())
    }

    async fn update_capabilities(&self, device_id: Uuid, capabilities: DeviceCapabilities) -> Result<()> {
        let mut devices = self.devices.write().unwrap();
        if let Some(device) = devices.get_mut(&device_id) {
            device.capabilities = capabilities;
        }
        Ok(())
    }

    async fn create_session(&self, device_id: Uuid) -> Result<DeviceSession> {
        let session = DeviceSession::new(device_id, self.session_duration);
        self.sessions.write().unwrap().insert(session.token.clone(), session.clone());
        Ok(session)
    }

    async fn validate_session(&self, token: &str) -> Result<Option<DeviceSession>> {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(token) {
            if session.is_valid() {
                session.last_activity = Utc::now();
                return Ok(Some(session.clone()));
            }
        }
        Ok(None)
    }

    async fn revoke_session(&self, token: &str) -> Result<()> {
        if let Some(session) = self.sessions.write().unwrap().get_mut(token) {
            session.revoke();
        }
        Ok(())
    }

    async fn update_last_seen(&self, device_id: Uuid) -> Result<()> {
        let mut devices = self.devices.write().unwrap();
        if let Some(device) = devices.get_mut(&device_id) {
            device.last_seen = Utc::now();
            device.is_online = true;
        }
        Ok(())
    }
}

/// Device activity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceActivity {
    /// Activity ID
    pub id: Uuid,
    /// Device ID
    pub device_id: Uuid,
    /// Activity type
    pub activity_type: DeviceActivityType,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Types of device activities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceActivityType {
    Login,
    Logout,
    SyncStarted,
    SyncCompleted,
    ContentPlayed,
    ContentDownloaded,
    SettingsChanged,
    DeviceRenamed,
    TrustChanged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type() {
        assert!(DeviceType::Mobile.is_mobile());
        assert!(DeviceType::Desktop.supports_full_sync());
        assert_eq!(DeviceType::Desktop.default_trust(), TrustLevel::Trusted);
    }

    #[test]
    fn test_trust_level() {
        assert!(TrustLevel::Trusted.can_sync_sensitive());
        assert!(TrustLevel::Standard.can_sync());
        assert!(!TrustLevel::Untrusted.can_sync());
    }

    #[test]
    fn test_device_capabilities() {
        let caps = DeviceCapabilities {
            video_playback: true,
            audio_playback: true,
            max_resolution: Some(1080),
            hdr_support: false,
            ..Default::default()
        };

        assert!(caps.can_play(720, false));
        assert!(caps.can_play(1080, false));
        assert!(!caps.can_play(2160, false)); // 4K exceeds max
        assert!(!caps.can_play(1080, true)); // HDR not supported
    }

    #[test]
    fn test_device_session() {
        let device_id = Uuid::new_v4();
        let session = DeviceSession::new(device_id, 24);
        
        assert!(session.is_valid());
        assert!(!session.is_expired());
        
        let mut session = session;
        session.revoke();
        assert!(!session.is_valid());
    }

    #[tokio::test]
    async fn test_device_manager() {
        let manager = InMemoryDeviceManager::new();
        
        let registration = DeviceRegistration {
            name: "Test Device".to_string(),
            device_type: DeviceType::Desktop,
            os: "Linux".to_string(),
            app_version: "1.0.0".to_string(),
            capabilities: DeviceCapabilities::default(),
            ip_address: None,
        };
        
        let device = manager.register_device(registration).await.unwrap();
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.trust_level, TrustLevel::Trusted);
        
        let session = manager.create_session(device.id).await.unwrap();
        assert!(session.is_valid());
        
        let validated = manager.validate_session(&session.token).await.unwrap();
        assert!(validated.is_some());
    }
}