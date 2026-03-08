//! Core types for video editing module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Unique identifier for a project
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a clip
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClipId(pub Uuid);

impl ClipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a track
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub Uuid);

impl TrackId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for an effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EffectId(pub Uuid);

impl EffectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a keyframe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyframeId(pub Uuid);

impl KeyframeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Time position in seconds
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimePosition(pub f64);

impl TimePosition {
    pub fn new(seconds: f64) -> Self {
        Self(seconds.max(0.0))
    }

    pub fn from_millis(millis: u64) -> Self {
        Self(millis as f64 / 1000.0)
    }

    pub fn from_frames(frame: u64, fps: f64) -> Self {
        Self(frame as f64 / fps)
    }

    pub fn as_seconds(&self) -> f64 {
        self.0
    }

    pub fn as_millis(&self) -> u64 {
        (self.0 * 1000.0) as u64
    }

    pub fn as_frames(&self, fps: f64) -> u64 {
        (self.0 * fps) as u64
    }
}

/// Time range with start and duration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: TimePosition,
    pub duration: TimePosition,
}

impl TimeRange {
    pub fn new(start: f64, duration: f64) -> Self {
        Self {
            start: TimePosition::new(start),
            duration: TimePosition::new(duration),
        }
    }

    pub fn end(&self) -> TimePosition {
        TimePosition::new(self.start.0 + self.duration.0)
    }

    pub fn contains(&self, position: TimePosition) -> bool {
        position >= self.start && position < self.end()
    }

    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn hd() -> Self {
        Self { width: 1280, height: 720 }
    }

    pub fn full_hd() -> Self {
        Self { width: 1920, height: 1080 }
    }

    pub fn ultra_hd() -> Self {
        Self { width: 3840, height: 2160 }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

/// Frame rate
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FrameRate(pub f64);

impl FrameRate {
    pub fn fps24() -> Self {
        Self(24.0)
    }

    pub fn fps25() -> Self {
        Self(25.0)
    }

    pub fn fps30() -> Self {
        Self(30.0)
    }

    pub fn fps60() -> Self {
        Self(60.0)
    }

    pub fn from_rational(num: u32, den: u32) -> Self {
        Self(num as f64 / den as f64)
    }
}

/// Video editing project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: ProjectSettings,
    pub timeline: Timeline,
    pub metadata: ProjectMetadata,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: ProjectId::new(),
            name: name.into(),
            created_at: now,
            updated_at: now,
            settings: ProjectSettings::default(),
            timeline: Timeline::new(),
            metadata: ProjectMetadata::default(),
        }
    }
}

/// Project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub sample_rate: u32,
    pub channels: u16,
    pub preview_quality: PreviewQuality,
    pub render_quality: RenderQuality,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            resolution: Resolution::full_hd(),
            frame_rate: FrameRate::fps30(),
            sample_rate: 48000,
            channels: 2,
            preview_quality: PreviewQuality::Medium,
            render_quality: RenderQuality::High,
        }
    }
}

/// Preview quality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreviewQuality {
    Low,
    Medium,
    High,
    Original,
}

/// Render quality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderQuality {
    Draft,
    Good,
    High,
    Ultra,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMetadata {
    pub description: Option<String>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub custom: HashMap<String, String>,
}

/// Timeline containing tracks and clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    pub duration: TimePosition,
    pub playhead: TimePosition,
    pub selection: Option<TimeRange>,
    pub markers: Vec<Marker>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            duration: TimePosition::new(0.0),
            playhead: TimePosition::new(0.0),
            selection: None,
            markers: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, track_id: TrackId) -> bool {
        let len = self.tracks.len();
        self.tracks.retain(|t| t.id != track_id);
        self.tracks.len() != len
    }

    pub fn get_track(&self, track_id: TrackId) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == track_id)
    }

    pub fn get_track_mut(&mut self, track_id: TrackId) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == track_id)
    }

    pub fn recalculate_duration(&mut self) {
        let max_end = self.tracks.iter()
            .flat_map(|t| t.clips.iter())
            .map(|c| c.timing.end())
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap_or(TimePosition::new(0.0));
        self.duration = max_end;
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Track type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
}

/// Track in timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
    pub is_muted: bool,
    pub is_locked: bool,
    pub is_visible: bool,
    pub volume: f32,
    pub opacity: f32,
}

impl Track {
    pub fn new(name: impl Into<String>, track_type: TrackType) -> Self {
        Self {
            id: TrackId::new(),
            name: name.into(),
            track_type,
            clips: Vec::new(),
            is_muted: false,
            is_locked: false,
            is_visible: true,
            volume: 1.0,
            opacity: 1.0,
        }
    }

    pub fn video(name: impl Into<String>) -> Self {
        Self::new(name, TrackType::Video)
    }

    pub fn audio(name: impl Into<String>) -> Self {
        Self::new(name, TrackType::Audio)
    }

    pub fn subtitle(name: impl Into<String>) -> Self {
        Self::new(name, TrackType::Subtitle)
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
        self.clips.sort_by(|a, b| {
            a.timing.start.0.partial_cmp(&b.timing.start.0).unwrap()
        });
    }

    pub fn remove_clip(&mut self, clip_id: ClipId) -> bool {
        let len = self.clips.len();
        self.clips.retain(|c| c.id != clip_id);
        self.clips.len() != len
    }

    pub fn get_clip_at(&self, position: TimePosition) -> Option<&Clip> {
        self.clips.iter().find(|c| c.timing.contains(position))
    }
}

/// Media source reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSource {
    pub path: String,
    pub duration: TimePosition,
    pub in_point: TimePosition,
    pub out_point: TimePosition,
}

impl MediaSource {
    pub fn new(path: impl Into<String>, duration: f64) -> Self {
        let duration = TimePosition::new(duration);
        Self {
            path: path.into(),
            duration,
            in_point: TimePosition::new(0.0),
            out_point: duration,
        }
    }

    pub fn trimmed_duration(&self) -> TimePosition {
        TimePosition::new(self.out_point.0 - self.in_point.0)
    }
}

/// Clip on timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,
    pub source: Option<MediaSource>,
    pub timing: TimeRange,
    pub effects: Vec<Effect>,
    pub transitions: Vec<Transition>,
    pub speed: f64,
    pub is_reversed: bool,
    pub audio_enabled: bool,
    pub video_enabled: bool,
}

impl Clip {
    pub fn new(name: impl Into<String>, start: f64, duration: f64) -> Self {
        Self {
            id: ClipId::new(),
            name: name.into(),
            source: None,
            timing: TimeRange::new(start, duration),
            effects: Vec::new(),
            transitions: Vec::new(),
            speed: 1.0,
            is_reversed: false,
            audio_enabled: true,
            video_enabled: true,
        }
    }

    pub fn from_source(source: MediaSource, start: f64) -> Self {
        let duration = source.trimmed_duration().0;
        Self {
            id: ClipId::new(),
            name: source.path.clone(),
            source: Some(source),
            timing: TimeRange::new(start, duration),
            effects: Vec::new(),
            transitions: Vec::new(),
            speed: 1.0,
            is_reversed: false,
            audio_enabled: true,
            video_enabled: true,
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn remove_effect(&mut self, effect_id: EffectId) -> bool {
        let len = self.effects.len();
        self.effects.retain(|e| e.id != effect_id);
        self.effects.len() != len
    }
}

/// Effect types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: EffectId,
    pub name: String,
    pub effect_type: EffectType,
    pub enabled: bool,
    pub parameters: HashMap<String, EffectParameter>,
    pub keyframes: Vec<Keyframe>,
}

impl Effect {
    pub fn new(name: impl Into<String>, effect_type: EffectType) -> Self {
        Self {
            id: EffectId::new(),
            name: name.into(),
            effect_type,
            enabled: true,
            parameters: HashMap::new(),
            keyframes: Vec::new(),
        }
    }

    pub fn set_parameter(&mut self, name: impl Into<String>, value: EffectParameter) {
        self.parameters.insert(name.into(), value);
    }
}

/// Effect types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    // Video effects
    ColorCorrection,
    BrightnessContrast,
    Saturation,
    HueShift,
    ColorBalance,
    Curves,
    Levels,
    
    // Transform effects
    Crop,
    Scale,
    Rotation,
    Position,
    Flip,
    
    // Blur effects
    GaussianBlur,
    MotionBlur,
    RadialBlur,
    LensBlur,
    
    // Stylize effects
    Sharpen,
    Vignette,
    FilmGrain,
    Glow,
    Emboss,
    EdgeDetect,
    
    // Time effects
    SlowMotion,
    FastForward,
    Reverse,
    FreezeFrame,
    TimeRemap,
    
    // Audio effects
    Volume,
    Fade,
    Equalizer,
    Compressor,
    Reverb,
    NoiseReduction,
    
    // Custom effect
    Custom(String),
}

/// Effect parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectParameter {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    Color(Color),
    Point(Point2D),
    Curve(Vec<Point2D>),
}

/// RGBA color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    pub fn transparent() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
}

/// 2D point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Keyframe for animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: KeyframeId,
    pub time: TimePosition,
    pub value: EffectParameter,
    pub interpolation: Interpolation,
}

impl Keyframe {
    pub fn new(time: TimePosition, value: EffectParameter) -> Self {
        Self {
            id: KeyframeId::new(),
            time,
            value,
            interpolation: Interpolation::Linear,
        }
    }
}

/// Interpolation type for keyframes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interpolation {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bezier,
    Step,
}

/// Transition between clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub id: Uuid,
    pub transition_type: TransitionType,
    pub duration: TimePosition,
    pub position: TransitionPosition,
    pub parameters: HashMap<String, EffectParameter>,
}

impl Transition {
    pub fn new(transition_type: TransitionType, duration: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            transition_type,
            duration: TimePosition::new(duration),
            position: TransitionPosition::BetweenClips,
            parameters: HashMap::new(),
        }
    }
}

/// Transition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Cut,
    Fade,
    Dissolve,
    Wipe(WipeDirection),
    Slide(SlideDirection),
    Zoom,
    Spin,
    Push(PushDirection),
    Iris,
    Custom(String),
}

/// Wipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WipeDirection {
    Left,
    Right,
    Up,
    Down,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// Slide direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Push direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PushDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Transition position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionPosition {
    StartOfClip,
    EndOfClip,
    BetweenClips,
}

/// Marker on timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub id: Uuid,
    pub position: TimePosition,
    pub name: String,
    pub color: Color,
    pub comment: Option<String>,
}

impl Marker {
    pub fn new(position: TimePosition, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            name: name.into(),
            color: Color::rgb(1.0, 0.8, 0.0),
            comment: None,
        }
    }
}

/// Undo/Redo action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryAction {
    pub id: Uuid,
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub action_type: ActionType,
    pub before: Vec<ActionData>,
    pub after: Vec<ActionData>,
}

/// Action types for undo/redo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    AddClip,
    RemoveClip,
    MoveClip,
    TrimClip,
    AddTrack,
    RemoveTrack,
    AddEffect,
    RemoveEffect,
    ModifyEffect,
    AddTransition,
    RemoveTransition,
    SetMarker,
    RemoveMarker,
    ChangeSpeed,
    Composite,
}

/// Action data for undo/redo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionData {
    Clip(Clip),
    Track(Track),
    Effect(Effect),
    Transition(Transition),
    Marker(Marker),
    TimePosition(TimePosition),
    Float(f64),
    String(String),
}

/// Render configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub output_path: String,
    pub format: OutputFormat,
    pub codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub bitrate: Option<u64>,
    pub quality: RenderQuality,
    pub hardware_acceleration: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            output_path: "output.mp4".to_string(),
            format: OutputFormat::Mp4,
            codec: VideoCodec::H264,
            audio_codec: AudioCodec::Aac,
            resolution: Resolution::full_hd(),
            frame_rate: FrameRate::fps30(),
            bitrate: None,
            quality: RenderQuality::High,
            hardware_acceleration: true,
        }
    }
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Mp4,
    Mov,
    Avi,
    Mkv,
    Webm,
    Gif,
}

/// Video codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    Vp9,
    Av1,
    ProRes,
    Dnxhd,
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    Aac,
    Mp3,
    Opus,
    Flac,
    Pcm,
    Ac3,
}

/// Render progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderProgress {
    pub current_frame: u64,
    pub total_frames: u64,
    pub current_time: TimePosition,
    pub total_time: TimePosition,
    pub estimated_remaining: Duration,
    pub fps: f64,
    pub status: RenderStatus,
}

impl RenderProgress {
    pub fn percentage(&self) -> f64 {
        if self.total_frames == 0 {
            return 0.0;
        }
        (self.current_frame as f64 / self.total_frames as f64) * 100.0
    }
}

/// Render status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderStatus {
    Queued,
    Preparing,
    Rendering,
    Encoding,
    Finalizing,
    Completed,
    Failed,
    Cancelled,
}