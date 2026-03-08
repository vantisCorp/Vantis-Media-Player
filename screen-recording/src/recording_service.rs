//! Recording service for screen capture

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use dashmap::DashMap;

use crate::types::*;
use crate::error::{ScreenRecordingError, ScreenRecordingResult};

/// Recording service trait
#[async_trait]
pub trait RecordingService: Send + Sync {
    /// Create a new recording session
    async fn create_session(&self, name: &str, config: RecordingConfig) -> ScreenRecordingResult<RecordingSession>;
    
    /// Start recording
    async fn start_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()>;
    
    /// Stop recording
    async fn stop_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<String>;
    
    /// Pause recording
    async fn pause_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()>;
    
    /// Resume recording
    async fn resume_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()>;
    
    /// Take screenshot during recording
    async fn take_screenshot(&self, recording_id: RecordingId) -> ScreenRecordingResult<String>;
    
    /// Get recording session
    async fn get_session(&self, recording_id: RecordingId) -> ScreenRecordingResult<RecordingSession>;
    
    /// Get all sessions
    async fn list_sessions(&self) -> ScreenRecordingResult<Vec<RecordingSession>>;
    
    /// Delete session
    async fn delete_session(&self, recording_id: RecordingId) -> ScreenRecordingResult<()>;
    
    /// Get recording statistics
    async fn get_stats(&self, recording_id: RecordingId) -> ScreenRecordingResult<RecordingStats>;
    
    /// Subscribe to recording events
    fn subscribe_to_events(&self) -> broadcast::Receiver<RecordingEvent>;
}

/// Default recording service implementation
pub struct DefaultRecordingService {
    sessions: DashMap<RecordingId, RecordingSession>,
    active_session: Arc<RwLock<Option<RecordingId>>>,
    event_sender: broadcast::Sender<RecordingEvent>,
}

impl DefaultRecordingService {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(256);
        Self {
            sessions: DashMap::new(),
            active_session: Arc::new(RwLock::new(None)),
            event_sender,
        }
    }
}

impl Default for DefaultRecordingService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RecordingService for DefaultRecordingService {
    async fn create_session(&self, name: &str, config: RecordingConfig) -> ScreenRecordingResult<RecordingSession> {
        let session = RecordingSession::new(name, config);
        self.sessions.insert(session.id, session.clone());
        Ok(session)
    }

    async fn start_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()> {
        let mut session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?
            .clone();
        
        // Check if already recording
        {
            let active = self.active_session.read().await;
            if let Some(active_id) = *active {
                if active_id == recording_id && session.status == RecordingStatus::Recording {
                    return Err(ScreenRecordingError::AlreadyRecording);
                }
            }
        }
        
        // Validate capture source
        self.validate_source(&session.config.video.source)?;
        
        // Validate output path
        self.validate_output_path(&session.config.output)?;
        
        // Update session state
        session.status = RecordingStatus::Recording;
        session.started_at = Some(chrono::Utc::now());
        self.sessions.insert(recording_id, session);
        
        // Set as active session
        {
            let mut active = self.active_session.write().await;
            *active = Some(recording_id);
        }
        
        // Send event
        let _ = self.event_sender.send(RecordingEvent::Started { recording_id });
        
        Ok(())
    }

    async fn stop_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<String> {
        let mut session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?
            .clone();
        
        if session.status != RecordingStatus::Recording && session.status != RecordingStatus::Paused {
            return Err(ScreenRecordingError::NotRecording);
        }
        
        // Generate output path
        let output_path = self.generate_output_path(&session)?;
        
        // Update session
        session.status = RecordingStatus::Completed;
        session.ended_at = Some(chrono::Utc::now());
        session.output_path = Some(output_path.clone());
        self.sessions.insert(recording_id, session);
        
        // Clear active session
        {
            let mut active = self.active_session.write().await;
            *active = None;
        }
        
        // Send event
        let _ = self.event_sender.send(RecordingEvent::Stopped { 
            recording_id, 
            output_path: output_path.clone() 
        });
        
        Ok(output_path)
    }

    async fn pause_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()> {
        let mut session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?
            .clone();
        
        if session.status != RecordingStatus::Recording {
            return Err(ScreenRecordingError::NotRecording);
        }
        
        session.status = RecordingStatus::Paused;
        self.sessions.insert(recording_id, session);
        
        let _ = self.event_sender.send(RecordingEvent::Paused { recording_id });
        
        Ok(())
    }

    async fn resume_recording(&self, recording_id: RecordingId) -> ScreenRecordingResult<()> {
        let mut session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?
            .clone();
        
        if session.status != RecordingStatus::Paused {
            return Err(ScreenRecordingError::NotRecording);
        }
        
        session.status = RecordingStatus::Recording;
        self.sessions.insert(recording_id, session);
        
        let _ = self.event_sender.send(RecordingEvent::Resumed { recording_id });
        
        Ok(())
    }

    async fn take_screenshot(&self, recording_id: RecordingId) -> ScreenRecordingResult<String> {
        let session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?;
        
        if session.status != RecordingStatus::Recording {
            return Err(ScreenRecordingError::NotRecording);
        }
        
        // Generate screenshot path
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let path = format!("{}/screenshot_{}.png", 
            session.config.output.path_pattern, 
            timestamp
        );
        
        let _ = self.event_sender.send(RecordingEvent::Screenshot { 
            recording_id, 
            path: path.clone() 
        });
        
        Ok(path)
    }

    async fn get_session(&self, recording_id: RecordingId) -> ScreenRecordingResult<RecordingSession> {
        self.sessions.get(&recording_id)
            .map(|s| s.clone())
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))
    }

    async fn list_sessions(&self) -> ScreenRecordingResult<Vec<RecordingSession>> {
        Ok(self.sessions.iter().map(|s| s.clone()).collect())
    }

    async fn delete_session(&self, recording_id: RecordingId) -> ScreenRecordingResult<()> {
        self.sessions.remove(&recording_id)
            .map(|_| ())
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))
    }

    async fn get_stats(&self, recording_id: RecordingId) -> ScreenRecordingResult<RecordingStats> {
        let session = self.sessions.get(&recording_id)
            .ok_or_else(|| ScreenRecordingError::RecordingNotFound(format!("{:?}", recording_id)))?;
        Ok(session.stats.clone())
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<RecordingEvent> {
        self.event_sender.subscribe()
    }
}

impl DefaultRecordingService {
    fn validate_source(&self, source: &CaptureSource) -> ScreenRecordingResult<()> {
        match source {
            CaptureSource::Screen { screen_index } => {
                if *screen_index > 10 { // Reasonable limit
                    return Err(ScreenRecordingError::ScreenNotFound(format!("Screen {}", screen_index)));
                }
            }
            CaptureSource::Window { window_id, .. } => {
                if *window_id == 0 {
                    return Err(ScreenRecordingError::WindowNotFound("Invalid window ID".to_string()));
                }
            }
            CaptureSource::Webcam { device_index } => {
                if *device_index > 10 {
                    return Err(ScreenRecordingError::WebcamNotFound(format!("Device {}", device_index)));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_output_path(&self, config: &OutputConfig) -> ScreenRecordingResult<()> {
        // Check if path is valid
        if config.path_pattern.is_empty() {
            return Err(ScreenRecordingError::OutputDirectoryNotFound("Path is empty".to_string()));
        }
        Ok(())
    }

    fn generate_output_path(&self, session: &RecordingSession) -> ScreenRecordingResult<String> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let extension = match session.config.output.format {
            OutputFormat::Mp4 => "mp4",
            OutputFormat::Mkv => "mkv",
            OutputFormat::Mov => "mov",
            OutputFormat::Webm => "webm",
            OutputFormat::Gif => "gif",
        };
        
        let filename = if session.config.output.naming.include_counter {
            format!("{}_{}.{}", session.name, timestamp, extension)
        } else {
            format!("{}.{}", session.name, extension)
        };
        
        Ok(format!("{}/{}", session.config.output.path_pattern, filename))
    }
}

/// Capture device service trait
#[async_trait]
pub trait CaptureDeviceService: Send + Sync {
    /// List available screens
    async fn list_screens(&self) -> ScreenRecordingResult<Vec<CaptureDevice>>;
    
    /// List available windows
    async fn list_windows(&self) -> ScreenRecordingResult<Vec<CaptureDevice>>;
    
    /// List available webcams
    async fn list_webcams(&self) -> ScreenRecordingResult<Vec<CaptureDevice>>;
    
    /// List available microphones
    async fn list_microphones(&self) -> ScreenRecordingResult<Vec<CaptureDevice>>;
    
    /// Get default devices
    async fn get_default_devices(&self) -> ScreenRecordingResult<DefaultDevices>;
}

/// Default devices
#[derive(Debug, Clone)]
pub struct DefaultDevices {
    pub screen: Option<CaptureDevice>,
    pub webcam: Option<CaptureDevice>,
    pub microphone: Option<CaptureDevice>,
}

/// Default capture device service implementation
pub struct DefaultCaptureDeviceService;

impl DefaultCaptureDeviceService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultCaptureDeviceService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CaptureDeviceService for DefaultCaptureDeviceService {
    async fn list_screens(&self) -> ScreenRecordingResult<Vec<CaptureDevice>> {
        // Return mock screens - in real implementation would query system
        Ok(vec![
            CaptureDevice {
                id: SourceId::new(),
                name: "Primary Monitor".to_string(),
                device_type: CaptureDeviceType::Screen,
                is_default: true,
                resolution: Some(Resolution::full_hd()),
                frame_rates: vec![30, 60],
            }
        ])
    }

    async fn list_windows(&self) -> ScreenRecordingResult<Vec<CaptureDevice>> {
        Ok(Vec::new())
    }

    async fn list_webcams(&self) -> ScreenRecordingResult<Vec<CaptureDevice>> {
        Ok(Vec::new())
    }

    async fn list_microphones(&self) -> ScreenRecordingResult<Vec<CaptureDevice>> {
        Ok(vec![
            CaptureDevice {
                id: SourceId::new(),
                name: "Default Microphone".to_string(),
                device_type: CaptureDeviceType::Microphone,
                is_default: true,
                resolution: None,
                frame_rates: Vec::new(),
            }
        ])
    }

    async fn get_default_devices(&self) -> ScreenRecordingResult<DefaultDevices> {
        let screens = self.list_screens().await?;
        let webcams = self.list_webcams().await?;
        let microphones = self.list_microphones().await?;
        
        Ok(DefaultDevices {
            screen: screens.into_iter().find(|d| d.is_default),
            webcam: webcams.into_iter().find(|d| d.is_default),
            microphone: microphones.into_iter().find(|d| d.is_default),
        })
    }
}