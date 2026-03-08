//! Clip management service

use async_trait::async_trait;
use std::sync::Arc;
use dashmap::DashMap;

use crate::types::*;
use crate::error::{VideoEditingError, VideoEditingResult};

/// Clip service trait
#[async_trait]
pub trait ClipService: Send + Sync {
    /// Add a clip to a track
    async fn add_clip(&self, project_id: ProjectId, track_id: TrackId, clip: Clip) -> VideoEditingResult<ClipId>;
    
    /// Remove a clip from a track
    async fn remove_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<()>;
    
    /// Move a clip to a new position
    async fn move_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_start: TimePosition) -> VideoEditingResult<()>;
    
    /// Trim clip start
    async fn trim_clip_start(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_start: TimePosition) -> VideoEditingResult<()>;
    
    /// Trim clip end
    async fn trim_clip_end(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_end: TimePosition) -> VideoEditingResult<()>;
    
    /// Set clip speed
    async fn set_clip_speed(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, speed: f64) -> VideoEditingResult<()>;
    
    /// Reverse clip
    async fn reverse_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<()>;
    
    /// Duplicate clip
    async fn duplicate_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<Clip>;
    
    /// Get clips in range
    async fn get_clips_in_range(&self, project_id: ProjectId, track_id: TrackId, range: TimeRange) -> VideoEditingResult<Vec<Clip>>;
    
    /// Enable/disable clip audio
    async fn set_clip_audio_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, enabled: bool) -> VideoEditingResult<()>;
    
    /// Enable/disable clip video
    async fn set_clip_video_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, enabled: bool) -> VideoEditingResult<()>;
}

/// Default clip service implementation
pub struct DefaultClipService {
    projects: Arc<DashMap<ProjectId, Project>>,
}

impl DefaultClipService {
    pub fn new(projects: Arc<DashMap<ProjectId, Project>>) -> Self {
        Self { projects }
    }
}

#[async_trait]
impl ClipService for DefaultClipService {
    async fn add_clip(&self, project_id: ProjectId, track_id: TrackId, mut clip: Clip) -> VideoEditingResult<ClipId> {
        let clip_id = clip.id;
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        // Check for overlaps
        for existing_clip in &track.clips {
            if clip.timing.overlaps(&existing_clip.timing) {
                return Err(VideoEditingError::ClipOverlap { position: clip.timing.start.0 });
            }
        }
        
        track.add_clip(clip);
        project.timeline.recalculate_duration();
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(clip_id)
    }

    async fn remove_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        if !track.remove_clip(clip_id) {
            return Err(VideoEditingError::ClipNotFound(format!("{:?}", clip_id)));
        }
        
        project.timeline.recalculate_duration();
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn move_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_start: TimePosition) -> VideoEditingResult<()> {
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
        
        let duration = track.clips[clip_index].timing.duration;
        let new_range = TimeRange::new(new_start.0, duration.0);
        
        // Check for overlaps with other clips
        for (i, existing_clip) in track.clips.iter().enumerate() {
            if i != clip_index && new_range.overlaps(&existing_clip.timing) {
                return Err(VideoEditingError::ClipOverlap { position: new_start.0 });
            }
        }
        
        track.clips[clip_index].timing.start = new_start;
        track.clips.sort_by(|a, b| a.timing.start.0.partial_cmp(&b.timing.start.0).unwrap());
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn trim_clip_start(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_start: TimePosition) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        if new_start.0 < 0.0 {
            return Err(VideoEditingError::InvalidTimeRange { start: new_start.0, end: clip.timing.end().0 });
        }
        
        if new_start.0 >= clip.timing.end().0 {
            return Err(VideoEditingError::InvalidTimeRange { start: new_start.0, end: clip.timing.end().0 });
        }
        
        let new_duration = clip.timing.end().0 - new_start.0;
        clip.timing.start = new_start;
        clip.timing.duration = TimePosition::new(new_duration);
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn trim_clip_end(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, new_end: TimePosition) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        if new_end.0 <= clip.timing.start.0 {
            return Err(VideoEditingError::InvalidTimeRange { start: clip.timing.start.0, end: new_end.0 });
        }
        
        let new_duration = new_end.0 - clip.timing.start.0;
        clip.timing.duration = TimePosition::new(new_duration);
        
        project.timeline.recalculate_duration();
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn set_clip_speed(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, speed: f64) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        if speed <= 0.0 {
            return Err(VideoEditingError::InvalidParameter("Speed must be positive".to_string()));
        }
        
        clip.speed = speed;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn reverse_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        clip.is_reversed = !clip.is_reversed;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn duplicate_clip(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId) -> VideoEditingResult<Clip> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?
            .clone();
        
        let mut new_clip = clip.clone();
        new_clip.id = ClipId::new();
        new_clip.timing.start = TimePosition::new(clip.timing.end().0 + 0.1);
        new_clip.name = format!("{} (copy)", clip.name);
        
        track.add_clip(new_clip.clone());
        project.timeline.recalculate_duration();
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(new_clip)
    }

    async fn get_clips_in_range(&self, project_id: ProjectId, track_id: TrackId, range: TimeRange) -> VideoEditingResult<Vec<Clip>> {
        let project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?;
        
        let track = project.timeline.get_track(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clips: Vec<Clip> = track.clips.iter()
            .filter(|c| c.timing.overlaps(&range))
            .cloned()
            .collect();
        
        Ok(clips)
    }

    async fn set_clip_audio_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, enabled: bool) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        clip.audio_enabled = enabled;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn set_clip_video_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, enabled: bool) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        clip.video_enabled = enabled;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }
}

/// Effect service trait
#[async_trait]
pub trait EffectService: Send + Sync {
    /// Add effect to clip
    async fn add_effect(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect: Effect) -> VideoEditingResult<EffectId>;
    
    /// Remove effect from clip
    async fn remove_effect(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId) -> VideoEditingResult<()>;
    
    /// Update effect parameter
    async fn set_effect_parameter(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, param_name: &str, value: EffectParameter) -> VideoEditingResult<()>;
    
    /// Add keyframe to effect
    async fn add_keyframe(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, keyframe: Keyframe) -> VideoEditingResult<()>;
    
    /// Remove keyframe from effect
    async fn remove_keyframe(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, keyframe_id: KeyframeId) -> VideoEditingResult<()>;
    
    /// Enable/disable effect
    async fn set_effect_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, enabled: bool) -> VideoEditingResult<()>;
    
    /// Reorder effects
    async fn reorder_effects(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_ids: Vec<EffectId>) -> VideoEditingResult<()>;
}

/// Default effect service implementation
pub struct DefaultEffectService {
    projects: Arc<DashMap<ProjectId, Project>>,
}

impl DefaultEffectService {
    pub fn new(projects: Arc<DashMap<ProjectId, Project>>) -> Self {
        Self { projects }
    }
}

#[async_trait]
impl EffectService for DefaultEffectService {
    async fn add_effect(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, mut effect: Effect) -> VideoEditingResult<EffectId> {
        let effect_id = effect.id;
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        clip.add_effect(effect);
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        
        Ok(effect_id)
    }

    async fn remove_effect(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        if track.is_locked {
            return Err(VideoEditingError::TrackLocked(track.name.clone()));
        }
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        if !clip.remove_effect(effect_id) {
            return Err(VideoEditingError::EffectNotFound(format!("{:?}", effect_id)));
        }
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn set_effect_parameter(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, param_name: &str, value: EffectParameter) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        let effect = clip.effects.iter_mut().find(|e| e.id == effect_id)
            .ok_or_else(|| VideoEditingError::EffectNotFound(format!("{:?}", effect_id)))?;
        
        effect.set_parameter(param_name, value);
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn add_keyframe(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, keyframe: Keyframe) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        let effect = clip.effects.iter_mut().find(|e| e.id == effect_id)
            .ok_or_else(|| VideoEditingError::EffectNotFound(format!("{:?}", effect_id)))?;
        
        effect.keyframes.push(keyframe);
        effect.keyframes.sort_by(|a, b| a.time.0.partial_cmp(&b.time.0).unwrap());
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn remove_keyframe(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, keyframe_id: KeyframeId) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        let effect = clip.effects.iter_mut().find(|e| e.id == effect_id)
            .ok_or_else(|| VideoEditingError::EffectNotFound(format!("{:?}", effect_id)))?;
        
        let len = effect.keyframes.len();
        effect.keyframes.retain(|k| k.id != keyframe_id);
        
        if effect.keyframes.len() == len {
            return Err(VideoEditingError::KeyframeInterpolationError("Keyframe not found".to_string()));
        }
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn set_effect_enabled(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_id: EffectId, enabled: bool) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        let effect = clip.effects.iter_mut().find(|e| e.id == effect_id)
            .ok_or_else(|| VideoEditingError::EffectNotFound(format!("{:?}", effect_id)))?;
        
        effect.enabled = enabled;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }

    async fn reorder_effects(&self, project_id: ProjectId, track_id: TrackId, clip_id: ClipId, effect_ids: Vec<EffectId>) -> VideoEditingResult<()> {
        let mut project = self.projects.get(&project_id)
            .ok_or_else(|| VideoEditingError::ProjectNotFound(format!("{:?}", project_id)))?
            .clone();
        
        let track = project.timeline.get_track_mut(track_id)
            .ok_or_else(|| VideoEditingError::TrackNotFound(format!("{:?}", track_id)))?;
        
        let clip = track.clips.iter_mut().find(|c| c.id == clip_id)
            .ok_or_else(|| VideoEditingError::ClipNotFound(format!("{:?}", clip_id)))?;
        
        // Create new ordered effects
        let mut new_effects = Vec::new();
        for effect_id in effect_ids {
            let effect = clip.effects.iter().find(|e| e.id == effect_id)
                .ok_or_else(|| VideoEditingError::EffectNotFound(format!("{:?}", effect_id)))?
                .clone();
            new_effects.push(effect);
        }
        
        clip.effects = new_effects;
        
        project.updated_at = chrono::Utc::now();
        self.projects.insert(project_id, project);
        Ok(())
    }
}