//! Project management service

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;

use crate::types::*;
use crate::error::{VideoEditingError, VideoEditingResult};

/// Project service trait
#[async_trait]
pub trait ProjectService: Send + Sync {
    /// Create a new project
    async fn create_project(&self, name: &str) -> VideoEditingResult<Project>;
    
    /// Open an existing project
    async fn open_project(&self, id: ProjectId) -> VideoEditingResult<Project>;
    
    /// Save a project
    async fn save_project(&self, project: &Project) -> VideoEditingResult<()>;
    
    /// Close a project
    async fn close_project(&self, id: ProjectId) -> VideoEditingResult<()>;
    
    /// Delete a project
    async fn delete_project(&self, id: ProjectId) -> VideoEditingResult<()>;
    
    /// List all projects
    async fn list_projects(&self) -> VideoEditingResult<Vec<ProjectInfo>>;
    
    /// Export project to file
    async fn export_project(&self, id: ProjectId, path: &str) -> VideoEditingResult<()>;
    
    /// Import project from file
    async fn import_project(&self, path: &str) -> VideoEditingResult<Project>;
    
    /// Get project settings
    async fn get_settings(&self, id: ProjectId) -> VideoEditingResult<ProjectSettings>;
    
    /// Update project settings
    async fn update_settings(&self, id: ProjectId, settings: ProjectSettings) -> VideoEditingResult<()>;
}

/// Project information for listing
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub id: ProjectId,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub duration: f64,
    pub track_count: usize,
    pub clip_count: usize,
}

/// Default project service implementation
pub struct DefaultProjectService {
    projects: DashMap<ProjectId, Project>,
    open_projects: Arc<RwLock<Vec<ProjectId>>>,
}

impl DefaultProjectService {
    pub fn new() -> Self {
        Self {
            projects: DashMap::new(),
            open_projects: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for DefaultProjectService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProjectService for DefaultProjectService {
    async fn create_project(&self, name: &str) -> VideoEditingResult<Project> {
        let project = Project::new(name);
        self.projects.insert(project.id, project.clone());
        Ok(project)
    }

    async fn open_project(&self, id: ProjectId) -> VideoEditingResult<Project> {
        let project = self.projects.get(&id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", id)))?
            .clone();
        
        let mut open = self.open_projects.write().await;
        if !open.contains(&id) {
            open.push(id);
        }
        
        Ok(project)
    }

    async fn save_project(&self, project: &Project) -> VideoEditingResult<()> {
        self.projects.insert(project.id, project.clone());
        Ok(())
    }

    async fn close_project(&self, id: ProjectId) -> VideoEditingResult<()> {
        let mut open = self.open_projects.write().await;
        open.retain(|&project_id| project_id != id);
        Ok(())
    }

    async fn delete_project(&self, id: ProjectId) -> VideoEditingResult<()> {
        self.projects.remove(&id);
        let mut open = self.open_projects.write().await;
        open.retain(|&project_id| project_id != id);
        Ok(())
    }

    async fn list_projects(&self) -> VideoEditingResult<Vec<ProjectInfo>> {
        let projects: Vec<ProjectInfo> = self.projects.iter()
            .map(|entry| {
                let project = entry.value();
                ProjectInfo {
                    id: project.id,
                    name: project.name.clone(),
                    created_at: project.created_at,
                    updated_at: project.updated_at,
                    duration: project.timeline.duration.0,
                    track_count: project.timeline.tracks.len(),
                    clip_count: project.timeline.tracks.iter().map(|t| t.clips.len()).sum(),
                }
            })
            .collect();
        Ok(projects)
    }

    async fn export_project(&self, id: ProjectId, path: &str) -> VideoEditingResult<()> {
        let project = self.projects.get(&id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", id)))?;
        
        let json = serde_json::to_string_pretty(project.value())?;
        std::fs::write(path, json)?;
        
        Ok(())
    }

    async fn import_project(&self, path: &str) -> VideoEditingResult<Project> {
        let content = std::fs::read_to_string(path)?;
        let project: Project = serde_json::from_str(&content)?;
        self.projects.insert(project.id, project.clone());
        Ok(project)
    }

    async fn get_settings(&self, id: ProjectId) -> VideoEditingResult<ProjectSettings> {
        let project = self.projects.get(&id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", id)))?;
        Ok(project.settings.clone())
    }

    async fn update_settings(&self, id: ProjectId, settings: ProjectSettings) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", id)))?
            .clone();
        project.settings = settings;
        project.updated_at = chrono::Utc::now();
        self.projects.insert(id, project);
        Ok(())
    }
}

/// Timeline service trait
#[async_trait]
pub trait TimelineService: Send + Sync {
    /// Add a track to the timeline
    async fn add_track(&self, project_id: ProjectId, track: Track) -> VideoEditingResult<TrackId>;
    
    /// Remove a track from the timeline
    async fn remove_track(&self, project_id: ProjectId, track_id: TrackId) -> VideoEditingResult<()>;
    
    /// Move a track to a new position
    async fn move_track(&self, project_id: ProjectId, track_id: TrackId, new_index: usize) -> VideoEditingResult<()>;
    
    /// Get all tracks
    async fn get_tracks(&self, project_id: ProjectId) -> VideoEditingResult<Vec<Track>>;
    
    /// Set playhead position
    async fn set_playhead(&self, project_id: ProjectId, position: TimePosition) -> VideoEditingResult<()>;
    
    /// Get playhead position
    async fn get_playhead(&self, project_id: ProjectId) -> VideoEditingResult<TimePosition>;
    
    /// Set selection range
    async fn set_selection(&self, project_id: ProjectId, selection: Option<TimeRange>) -> VideoEditingResult<()>;
    
    /// Add marker
    async fn add_marker(&self, project_id: ProjectId, marker: Marker) -> VideoEditingResult<()>;
    
    /// Remove marker
    async fn remove_marker(&self, project_id: ProjectId, marker_id: uuid::Uuid) -> VideoEditingResult<()>;
    
    /// Split clip at position
    async fn split_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, position: TimePosition) -> VideoEditingResult<Clip>;
    
    /// Join clips
    async fn join_clips(&self, project_id: ProjectId, track_id: TrackId, clip_ids: Vec<ClipId>) -> VideoEditingResult<Clip>;
}

/// Default timeline service implementation
pub struct DefaultTimelineService {
    projects: Arc<DashMap<ProjectId, Project>>,
}

impl DefaultTimelineService {
    pub fn new(projects: Arc<DashMap<ProjectId, Project>>) -> Self {
        Self { projects }
    }
}

#[async_trait]
impl TimelineService for DefaultTimelineService {
    async fn add_track(&self, project_id: ProjectId, mut track: Track) -> VideoEditingResult<TrackId> {
        let track_id = track.id;
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        project.timeline.add_track(track);
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(track_id)
    }

    async fn remove_track(&self, project_id: ProjectId, track_id: TrackId) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        if !project.timeline.remove_track(track_id) {
            return Err(VideoEditingError::TrackNotFound(format!("{:?}", track_id)));
        }
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn move_track(&self, project_id: ProjectId, track_id: TrackId, new_index: usize) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let current_index = project.timeline.tracks.iter()
            .position(|t| t.id == track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if current_index != new_index && new_index < project.timeline.tracks.len() {
            let track = project.timeline.tracks.remove(current_index);
            project.timeline.tracks.insert(new_index, track);
            project.updated_at = chrono::Utc::now();
            self.projects.insert(project_id, project);
        }
        
        Ok(())
    }

    async fn get_tracks(&self, project_id: ProjectId) -> VideoEditingResult<Vec<Track>> {
        let project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?;
        Ok(project.timeline.tracks.clone())
    }

    async fn set_playhead(&self, project_id: ProjectId, position: TimePosition) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        project.timeline.playhead = position;
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn get_playhead(&self, project_id: ProjectId) -> VideoEditingResult<TimePosition> {
        let project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?;
        Ok(project.timeline.playhead)
    }

    async fn set_selection(&self, project_id: ProjectId, selection: Option<TimeRange>) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        project.timeline.selection = selection;
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn add_marker(&self, project_id: ProjectId, marker: Marker) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        project.timeline.markers.push(marker);
        project.timeline.markers.sort_by(|a, b| a.position.0.partial_cmp(&b.position.0).unwrap());
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn remove_marker(&self, project_id: ProjectId, marker_id: uuid::Uuid) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let len = project.timeline.markers.len();
        project.timeline.markers.retain(|m| m.id != marker_id);
        
        if project.timeline.markers.len() == len {
            return Err(VideoEditingError::ClipNotFound(format!("Marker {:?}", marker_id)));
        }
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn split_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, position: TimePosition) -> VideoEditingResult<Clip> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip_index = track.clips.iter().position(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        let clip = &track.clips[clip_index];
        if !clip.timing.contains(position) {
            return Err(VideoEditingError::InvalidOperation("Position not within clip".to_string()));
        }
        
        let relative_pos = position.0 - clip.timing.start.0;
        let first_duration = relative_pos;
        let second_duration = clip.timing.duration.0 - relative_pos;
        
        // Create second clip
        let mut second_clip = clip.clone();
        second_clip.id = ClipId::new();
        second_clip.timing = TimeRange::new(position.0, second_duration);
        second_clip.name = format!("{} (split)", clip.name);
        
        // Modify first clip
        let first_clip = &mut track.clips[clip_index];
        first_clip.timing.duration = TimePosition::new(first_duration);
        
        // Insert second clip
        track.clips.insert(clip_index + 1, second_clip.clone());
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(second_clip)
    }

    async fn join_clips(&self, project_id: ProjectId, track_id: TrackId, clip_ids: Vec<ClipId>) -> VideoEditingResult<Clip> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        // Find all clips
        let mut clips_to_join: Vec<Clip> = Vec::new();
        for &clip_id in &clip_ids {
            let clip = track.clips.iter().find(|c| c.id == clip_id)
                .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?
                .clone();
            clips_to_join.push(clip);
        }
        
        if clips_to_join.len() < 2 {
            return Err(VideoEditingError::InvalidOperation("Need at least 2 clips to join".to_string()));
        }
        
        // Sort by start time
        clips_to_join.sort_by(|a, b| a.timing.start.0.partial_cmp(&b.timing.start.0).unwrap());
        
        // Check if clips are adjacent
        for i in 0..clips_to_join.len() - 1 {
            if (clips_to_join[i].timing.end().0 - clips_to_join[i + 1].timing.start.0).abs() > 0.001 {
                return Err(VideoEditingError::InvalidOperation("Clips must be adjacent to join".to_string()));
            }
        }
        
        // Create merged clip
        let first_clip = &clips_to_join[0];
        let last_clip = &clips_to_join[clips_to_join.len() - 1];
        let total_duration = last_clip.timing.end().0 - first_clip.timing.start.0;
        
        let mut merged_clip = first_clip.clone();
        merged_clip.id = ClipId::new();
        merged_clip.timing = TimeRange::new(first_clip.timing.start.0, total_duration);
        merged_clip.name = format!("{} (joined)", first_clip.name);
        
        // Remove old clips and add merged
        track.clips.retain(|c| !clip_ids.contains(&c.id));
        track.add_clip(merged_clip.clone());
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(merged_clip)
    }
}