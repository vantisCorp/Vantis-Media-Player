//! Rendering service for video export

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use dashmap::DashMap;

use crate::types::*;
use crate::error::{VideoEditingError, VideoEditingResult};

/// Render service trait
#[async_trait]
pub trait RenderService: Send + Sync {
    /// Start rendering a project
    async fn start_render(&self, project_id: ProjectId, config: RenderConfig) -> VideoEditingResult<uuid::Uuid>;
    
    /// Cancel a render job
    async fn cancel_render(&self, render_id: uuid::Uuid) -> VideoEditingResult<()>;
    
    /// Get render progress
    async fn get_progress(&self, render_id: uuid::Uuid) -> VideoEditingResult<RenderProgress>;
    
    /// Subscribe to render progress updates
    fn subscribe_to_progress(&self) -> broadcast::Receiver<RenderProgressEvent>;
    
    /// List render jobs
    async fn list_renders(&self) -> VideoEditingResult<Vec<RenderJobInfo>>;
    
    /// Get render queue position
    async fn get_queue_position(&self, render_id: uuid::Uuid) -> VideoEditingResult<usize>;
}

/// Render progress event
#[derive(Debug, Clone)]
pub struct RenderProgressEvent {
    pub render_id: uuid::Uuid,
    pub project_id: ProjectId,
    pub progress: RenderProgress,
}

/// Render job information
#[derive(Debug, Clone)]
pub struct RenderJobInfo {
    pub id: uuid::Uuid,
    pub project_id: ProjectId,
    pub project_name: String,
    pub config: RenderConfig,
    pub status: RenderStatus,
    pub progress: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Render queue
struct RenderQueue {
    jobs: Vec<RenderJob>,
    current: Option<RenderJob>,
}

impl RenderQueue {
    fn new() -> Self {
        Self {
            jobs: Vec::new(),
            current: None,
        }
    }
}

/// Internal render job
struct RenderJob {
    id: uuid::Uuid,
    project_id: ProjectId,
    config: RenderConfig,
    status: RenderStatus,
    progress: RenderProgress,
    created_at: chrono::DateTime<chrono::Utc>,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Default render service implementation
pub struct DefaultRenderService {
    projects: Arc<DashMap<ProjectId, Project>>,
    queue: Arc<RwLock<RenderQueue>>,
    progress_sender: broadcast::Sender<RenderProgressEvent>,
}

impl DefaultRenderService {
    pub fn new(projects: Arc<DashMap<ProjectId, Project>>) -> Self {
        let (progress_sender, _) = broadcast::channel(256);
        Self {
            projects,
            queue: Arc::new(RwLock::new(RenderQueue::new())),
            progress_sender,
        }
    }
}

#[async_trait]
impl RenderService for DefaultRenderService {
    async fn start_render(&self, project_id: ProjectId, config: RenderConfig) -> VideoEditingResult<uuid::Uuid> {
        let project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?;
        
        let render_id = uuid::Uuid::new_v4();
        let total_frames = (project.timeline.duration.0 * config.frame_rate.0) as u64;
        
        let job = RenderJob {
            id: render_id,
            project_id,
            config,
            status: RenderStatus::Queued,
            progress: RenderProgress {
                current_frame: 0,
                total_frames,
                current_time: TimePosition::new(0.0),
                total_time: project.timeline.duration,
                estimated_remaining: chrono::Duration::seconds(0),
                fps: 0.0,
                status: RenderStatus::Queued,
            },
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
        };
        
        let mut queue = self.queue.write().await;
        queue.jobs.push(job);
        
        Ok(render_id)
    }

    async fn cancel_render(&self, render_id: uuid::Uuid) -> VideoEditingResult<()> {
        let mut queue = self.queue.write().await;
        
        // Check if it's the current job
        if let Some(ref current) = queue.current {
            if current.id == render_id {
                let mut job = queue.current.take().unwrap();
                job.status = RenderStatus::Cancelled;
                queue.current = Some(job);
                return Ok(());
            }
        }
        
        // Check queue
        let len = queue.jobs.len();
        queue.jobs.retain(|j| j.id != render_id);
        
        if queue.jobs.len() == len {
            return Err(VideoEditingError::RenderFailed("Render job not found".to_string()));
        }
        
        Ok(())
    }

    async fn get_progress(&self, render_id: uuid::Uuid) -> VideoEditingResult<RenderProgress> {
        let queue = self.queue.read().await;
        
        // Check current job
        if let Some(ref current) = queue.current {
            if current.id == render_id {
                return Ok(current.progress.clone());
            }
        }
        
        // Check queue
        for job in &queue.jobs {
            if job.id == render_id {
                return Ok(job.progress.clone());
            }
        }
        
        Err(VideoEditingError::RenderFailed("Render job not found".to_string()))
    }

    fn subscribe_to_progress(&self) -> broadcast::Receiver<RenderProgressEvent> {
        self.progress_sender.subscribe()
    }

    async fn list_renders(&self) -> VideoEditingResult<Vec<RenderJobInfo>> {
        let queue = self.queue.read().await;
        let mut jobs = Vec::new();
        
        if let Some(ref current) = queue.current {
            let project = self.projects.get(&current.project_id);
            jobs.push(RenderJobInfo {
                id: current.id,
                project_id: current.project_id,
                project_name: project.map(|p| p.name.clone()).unwrap_or_default(),
                config: current.config.clone(),
                status: current.status,
                progress: current.progress.percentage(),
                created_at: current.created_at,
                started_at: current.started_at,
                completed_at: current.completed_at,
            });
        }
        
        for job in &queue.jobs {
            let project = self.projects.get(&job.project_id);
            jobs.push(RenderJobInfo {
                id: job.id,
                project_id: job.project_id,
                project_name: project.map(|p| p.name.clone()).unwrap_or_default(),
                config: job.config.clone(),
                status: job.status,
                progress: job.progress.percentage(),
                created_at: job.created_at,
                started_at: job.started_at,
                completed_at: job.completed_at,
            });
        }
        
        Ok(jobs)
    }

    async fn get_queue_position(&self, render_id: uuid::Uuid) -> VideoEditingResult<usize> {
        let queue = self.queue.read().await;
        
        // Check if it's the current job
        if let Some(ref current) = queue.current {
            if current.id == render_id {
                return Ok(0);
            }
        }
        
        // Find in queue
        for (i, job) in queue.jobs.iter().enumerate() {
            if job.id == render_id {
                return Ok(i + 1); // +1 because current job is position 0
            }
        }
        
        Err(VideoEditingError::RenderFailed("Render job not found".to_string()))
    }
}

/// Undo/Redo service trait
#[async_trait]
pub trait HistoryService: Send + Sync {
    /// Perform undo
    async fn undo(&self, project_id: ProjectId) -> VideoEditingResult<HistoryAction>;
    
    /// Perform redo
    async fn redo(&self, project_id: ProjectId) -> VideoEditingResult<HistoryAction>;
    
    /// Check if undo is available
    async fn can_undo(&self, project_id: ProjectId) -> bool;
    
    /// Check if redo is available
    async fn can_redo(&self, project_id: ProjectId) -> bool;
    
    /// Get undo history
    async fn get_undo_history(&self, project_id: ProjectId) -> VideoEditingResult<Vec<HistoryAction>>;
    
    /// Get redo history
    async fn get_redo_history(&self, project_id: ProjectId) -> VideoEditingResult<Vec<HistoryAction>>;
    
    /// Clear history
    async fn clear_history(&self, project_id: ProjectId) -> VideoEditingResult<()>;
}

/// Default history service implementation
pub struct DefaultHistoryService {
    projects: Arc<DashMap<ProjectId, Project>>,
    undo_stacks: DashMap<ProjectId, Vec<HistoryAction>>,
    redo_stacks: DashMap<ProjectId, Vec<HistoryAction>>,
    max_history: usize,
}

impl DefaultHistoryService {
    pub fn new(projects: Arc<DashMap<ProjectId, Project>>) -> Self {
        Self {
            projects,
            undo_stacks: DashMap::new(),
            redo_stacks: DashMap::new(),
            max_history: 100,
        }
    }
    
    pub fn push_action(&self, project_id: ProjectId, action: HistoryAction) {
        let mut stack = self.undo_stacks.entry(project_id).or_insert(Vec::new());
        stack.push(action);
        
        // Limit stack size
        if stack.len() > self.max_history {
            stack.remove(0);
        }
        
        // Clear redo stack on new action
        self.redo_stacks.remove(&project_id);
    }
}

#[async_trait]
impl HistoryService for DefaultHistoryService {
    async fn undo(&self, project_id: ProjectId) -> VideoEditingResult<HistoryAction> {
        let mut stack = self.undo_stacks.get_mut(&project_id)
            .ok_or(VideoEditingError::UndoStackEmpty)?;
        
        let action = stack.pop().ok_or(VideoEditingError::UndoStackEmpty)?;
        
        // Add to redo stack
        let mut redo_stack = self.redo_stacks.entry(project_id).or_insert(Vec::new());
        redo_stack.push(action.clone());
        
        Ok(action)
    }

    async fn redo(&self, project_id: ProjectId) -> VideoEditingResult<HistoryAction> {
        let mut stack = self.redo_stacks.get_mut(&project_id)
            .ok_or(VideoEditingError::RedoStackEmpty)?;
        
        let action = stack.pop().ok_or(VideoEditingError::RedoStackEmpty)?;
        
        // Add back to undo stack
        let mut undo_stack = self.undo_stacks.entry(project_id).or_insert(Vec::new());
        undo_stack.push(action.clone());
        
        Ok(action)
    }

    async fn can_undo(&self, project_id: ProjectId) -> bool {
        self.undo_stacks.get(&project_id)
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    async fn can_redo(&self, project_id: ProjectId) -> bool {
        self.redo_stacks.get(&project_id)
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    async fn get_undo_history(&self, project_id: ProjectId) -> VideoEditingResult<Vec<HistoryAction>> {
        Ok(self.undo_stacks.get(&project_id)
            .map(|s| s.iter().rev().cloned().collect())
            .unwrap_or_default())
    }

    async fn get_redo_history(&self, project_id: ProjectId) -> VideoEditingResult<Vec<HistoryAction>> {
        Ok(self.redo_stacks.get(&project_id)
            .map(|s| s.iter().rev().cloned().collect())
            .unwrap_or_default())
    }

    async fn clear_history(&self, project_id: ProjectId) -> VideoEditingResult<()> {
        self.undo_stacks.remove(&project_id);
        self.redo_stacks.remove(&project_id);
        Ok(())
    }
}