//! DASH (Dynamic Adaptive Streaming over HTTP) Protocol Handler
//! 
//! This module provides comprehensive support for MPEG-DASH streaming including:
//! - MPD (Media Presentation Description) parsing
//! - Period, AdaptationSet, and Representation hierarchy
//! - Segment templates and timelines
//! - Content protection (DRM) parsing
//! - Multi-period and live streaming support

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

// ============================================================================
// MPD (Media Presentation Description) Types
// ============================================================================

/// Root MPD document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPresentationDescription {
    /// MPD schema version
    pub version: Option<String>,
    /// MPD type (static or dynamic)
    pub mpd_type: MPDType,
    /// Media presentation duration
    pub duration: Option<Duration>,
    /// Minimum buffer time
    pub min_buffer_time: Duration,
    /// Maximum segment duration
    pub max_segment_duration: Option<Duration>,
    /// Maximum subsegment duration
    pub max_subsegment_duration: Option<Duration>,
    /// Profile (e.g., "urn:mpeg:dash:profile:isoff-live:2011")
    pub profiles: Vec<String>,
    /// Availability start time for live
    pub availability_start_time: Option<String>,
    /// Availability end time for live
    pub availability_end_time: Option<String>,
    /// Time shift buffer depth for live
    pub time_shift_buffer_depth: Option<Duration>,
    /// Suggested presentation delay
    pub suggested_presentation_delay: Option<Duration>,
    /// Publish time
    pub publish_time: Option<String>,
    /// Base URL for resolving relative URLs
    pub base_url: Option<String>,
    /// Periods in the presentation
    pub periods: Vec<Period>,
    /// Program information
    pub program_info: Option<ProgramInformation>,
    /// Location URLs
    pub locations: Vec<String>,
    /// Metrics
    pub metrics: Vec<Metrics>,
}

/// MPD type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MPDType {
    Static,
    Dynamic,
}

impl Default for MPDType {
    fn default() -> Self {
        Self::Static
    }
}

/// Period within MPD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// Unique identifier
    pub id: Option<String>,
    /// Start time relative to MPD start
    pub start: Option<Duration>,
    /// Duration of the period
    pub duration: Option<Duration>,
    /// Bitstream switching mode
    pub bitstream_switching: Option<bool>,
    /// Base URL
    pub base_url: Option<String>,
    /// Adaptation sets
    pub adaptation_sets: Vec<AdaptationSet>,
    /// Subset
    pub subsets: Vec<Subset>,
    /// Supplemental properties
    pub supplemental_properties: Vec<Descriptor>,
}

/// AdaptationSet within Period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationSet {
    /// Unique identifier
    pub id: Option<String>,
    /// Content type (video, audio, text, image, etc.)
    pub content_type: Option<ContentType>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Codecs string
    pub codecs: Option<String>,
    /// Language
    pub lang: Option<String>,
    /// Maximum width in pixels
    pub max_width: Option<u32>,
    /// Maximum height in pixels
    pub max_height: Option<u32>,
    /// Maximum frame rate
    pub max_frame_rate: Option<f64>,
    /// Par (pixel aspect ratio)
    pub par: Option<String>,
    /// Segment alignment
    pub segment_alignment: Option<bool>,
    /// Subsegment alignment
    pub subsegment_alignment: Option<bool>,
    /// Bitstream switching
    pub bitstream_switching: Option<bool>,
    /// Selection priority
    pub selection_priority: Option<u32>,
    /// Base URL
    pub base_url: Option<String>,
    /// Representations
    pub representations: Vec<Representation>,
    /// Content protection
    pub content_protection: Vec<ContentProtection>,
    /// Accessibility descriptors
    pub accessibility: Vec<Descriptor>,
    /// Role descriptors
    pub roles: Vec<Descriptor>,
    /// Rating descriptors
    pub ratings: Vec<Descriptor>,
    /// Viewpoint descriptors
    pub viewpoints: Vec<Descriptor>,
    /// Supplemental properties
    pub supplemental_properties: Vec<Descriptor>,
    /// Essential properties
    pub essential_properties: Vec<Descriptor>,
}

/// Content type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    Video,
    Audio,
    Text,
    Image,
    Application,
    Font,
    Unknown,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Representation within AdaptationSet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Representation {
    /// Unique identifier
    pub id: String,
    /// Bandwidth in bits per second
    pub bandwidth: u64,
    /// Width in pixels (for video)
    pub width: Option<u32>,
    /// Height in pixels (for video)
    pub height: Option<u32>,
    /// Frame rate
    pub frame_rate: Option<f64>,
    /// Sample rate (for audio)
    pub sample_rate: Option<u32>,
    /// Number of audio channels
    pub num_channels: Option<u32>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Codecs string
    pub codecs: Option<String>,
    /// SAR (sample aspect ratio)
    pub sar: Option<String>,
    /// Base URL
    pub base_url: Option<String>,
    /// Segment base
    pub segment_base: Option<SegmentBase>,
    /// Segment template
    pub segment_template: Option<SegmentTemplate>,
    /// Segment list
    pub segment_list: Option<SegmentList>,
    /// Content protection
    pub content_protection: Vec<ContentProtection>,
    /// Supplemental properties
    pub supplemental_properties: Vec<Descriptor>,
    /// Essential properties
    pub essential_properties: Vec<Descriptor>,
    /// Sub-representations
    pub sub_representations: Vec<SubRepresentation>,
}

/// SubRepresentation for embedded streams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRepresentation {
    /// Content type
    pub content_type: Option<ContentType>,
    /// Level
    pub level: Option<u32>,
    /// Dependency on other representations
    pub dependency_id: Option<String>,
    /// Bandwidth
    pub bandwidth: Option<u64>,
}

// ============================================================================
// Segment Types
// ============================================================================

/// Segment base for index-based segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentBase {
    /// Timescale
    pub timescale: Option<u32>,
    /// Presentation time offset
    pub presentation_time_offset: Option<u64>,
    /// Index range (byte range)
    pub index_range: Option<String>,
    /// Index range exact
    pub index_range_exact: Option<bool>,
    /// Initialization segment
    pub initialization: Option<URL>,
    /// Representation index
    pub representation_index: Option<URL>,
}

/// Segment template for template-based segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentTemplate {
    /// Timescale
    pub timescale: Option<u32>,
    /// Duration in timescale units
    pub duration: Option<u64>,
    /// Start number
    pub start_number: Option<u64>,
    /// Presentation time offset
    pub presentation_time_offset: Option<u64>,
    /// Media template URL
    pub media: Option<String>,
    /// Index template URL
    pub index: Option<String>,
    /// Initialization URL
    pub initialization: Option<String>,
    /// Bitstream switching URL
    pub bitstream_switching: Option<String>,
    /// Segment timeline
    pub segment_timeline: Option<SegmentTimeline>,
}

/// Segment timeline for live streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentTimeline {
    /// Timeline entries
    pub entries: Vec<TimelineEntry>,
}

/// Timeline entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    /// Start time
    pub t: Option<u64>,
    /// Number of consecutive segments
    pub r: Option<i64>,
    /// Duration
    pub d: u64,
}

/// Segment list for explicit segment enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentList {
    /// Timescale
    pub timescale: Option<u32>,
    /// Duration
    pub duration: Option<u64>,
    /// Start number
    pub start_number: Option<u64>,
    /// Presentation time offset
    pub presentation_time_offset: Option<u64>,
    /// Segments
    pub segments: Vec<SegmentURL>,
    /// Initialization segment
    pub initialization: Option<URL>,
}

/// Segment URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentURL {
    /// Media URL
    pub media: Option<String>,
    /// Media range (byte range)
    pub media_range: Option<String>,
    /// Index URL
    pub index: Option<String>,
    /// Index range
    pub index_range: Option<String>,
}

/// Generic URL type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URL {
    /// URL string
    pub url: String,
    /// Service location
    pub service_location: Option<String>,
    /// Byte range
    pub range: Option<String>,
}

// ============================================================================
// Content Protection (DRM)
// ============================================================================

/// Content protection descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProtection {
    /// DRM scheme ID (e.g., "urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51aa21f" for Widevine)
    pub scheme_id: String,
    /// Content protection value
    pub value: Option<String>,
    /// Key ID
    pub kid: Option<String>,
    /// Default key ID (KID) from default_KID attribute
    pub default_kid: Option<String>,
    /// Robustness level (e.g., "SW_SECURE_CRYPTO")
    pub robustness: Option<String>,
    /// Custom namespace for vendor-specific data
    pub cenc: Option<CEncParams>,
    /// PlayReady specific
    pub playready: Option<PlayReadyParams>,
    /// Widevine specific
    pub widevine: Option<WidevineParams>,
    /// ClearKey specific
    pub clearkey: Option<ClearKeyParams>,
    /// Marlin specific
    pub marlin: Option<MarlinParams>,
}

/// CENC parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CEncParams {
    /// Default KID
    pub default_kid: Option<String>,
    /// PSSH boxes
    pub pssh: Vec<PSSHBox>,
}

/// PSSH box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSSHBox {
    /// System ID
    pub system_id: String,
    /// PSSH data (base64 encoded)
    pub data: Option<String>,
}

/// PlayReady parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayReadyParams {
    /// Header XML
    pub header: Option<String>,
}

/// Widevine parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidevineParams {
    /// License server URL
    pub license_server_url: Option<String>,
    /// PSSH data
    pub pssh: Option<String>,
}

/// ClearKey parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearKeyParams {
    /// Key IDs
    pub key_ids: Vec<String>,
}

/// Marlin parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarlinParams {
    /// Content ID
    pub content_id: Option<String>,
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Program information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInformation {
    /// Title
    pub title: Option<String>,
    /// Source
    pub source: Option<String>,
    /// Copyright
    pub copyright: Option<String>,
    /// More information URL
    pub more_info: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Generic descriptor type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    /// Scheme ID
    pub scheme_id: Option<String>,
    /// Value
    pub value: Option<String>,
    /// ID
    pub id: Option<String>,
    /// Sub-element content
    pub content: Option<String>,
}

/// Subset for adaptation set combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subset {
    /// Unique identifier
    pub id: Option<String>,
    /// Adaptation set references
    pub adaptation_sets: Vec<String>,
}

/// Metrics reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    /// Metrics type
    pub metrics_type: String,
    /// Reporting URLs
    pub reporting: Vec<String>,
}

// ============================================================================
// MPD Parser
// ============================================================================

/// MPD Parser
pub struct MPDParser {
    /// Parsed MPD
    mpd: Option<MediaPresentationDescription>,
    /// Last parsing errors
    errors: Vec<String>,
}

impl MPDParser {
    /// Create new parser
    pub fn new() -> Self {
        Self {
            mpd: None,
            errors: Vec::new(),
        }
    }

    /// Parse MPD from XML string
    pub fn parse(&mut self, xml: &str) -> Result<MediaPresentationDescription, String> {
        self.errors.clear();
        
        // Parse XML structure
        let mpd = self.parse_mpd_document(xml)?;
        
        self.mpd = Some(mpd.clone());
        Ok(mpd)
    }

    /// Parse MPD document
    fn parse_mpd_document(&mut self, xml: &str) -> Result<MediaPresentationDescription, String> {
        // Basic MPD structure parsing
        // In production, use quick-xml or roxmltree for proper XML parsing
        
        let mpd_type = if xml.contains("type=&quot;dynamic&quot;") || xml.contains("type='dynamic'") {
            MPDType::Dynamic
        } else {
            MPDType::Static
        };

        let duration = self.extract_duration(xml, "duration");
        let min_buffer_time = self.extract_duration(xml, "minBufferTime")
            .unwrap_or_else(|| Duration::from_secs(2));
        
        let profiles = self.extract_profiles(xml);
        let periods = self.parse_periods(xml)?;

        Ok(MediaPresentationDescription {
            version: self.extract_attribute(xml, "version"),
            mpd_type,
            duration,
            min_buffer_time,
            max_segment_duration: self.extract_duration(xml, "maxSegmentDuration"),
            max_subsegment_duration: self.extract_duration(xml, "maxSubsegmentDuration"),
            profiles,
            availability_start_time: self.extract_attribute(xml, "availabilityStartTime"),
            availability_end_time: self.extract_attribute(xml, "availabilityEndTime"),
            time_shift_buffer_depth: self.extract_duration(xml, "timeShiftBufferDepth"),
            suggested_presentation_delay: self.extract_duration(xml, "suggestedPresentationDelay"),
            publish_time: self.extract_attribute(xml, "publishTime"),
            base_url: self.extract_element_content(xml, "BaseURL"),
            periods,
            program_info: self.parse_program_info(xml),
            locations: self.parse_locations(xml),
            metrics: Vec::new(),
        })
    }

    /// Extract attribute value from XML
    fn extract_attribute(&self, xml: &str, name: &str) -> Option<String> {
        // Look for attribute patterns: name="value" or name='value'
        let patterns = [
            format!(r#"{}="([^"]*)""#, name),
            format!(r#"{}='([^']*)'"#, name),
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(&pattern) {
                if let Some(caps) = re.captures(xml) {
                    if let Some(m) = caps.get(1) {
                        return Some(m.as_str().to_string());
                    }
                }
            }
        }
        None
    }

    /// Extract duration from ISO 8601 duration string
    fn extract_duration(&self, xml: &str, name: &str) -> Option<Duration> {
        let value = self.extract_attribute(xml, name)?;
        self.parse_iso8601_duration(&value)
    }

    /// Parse ISO 8601 duration (PT1H30M5.5S format)
    fn parse_iso8601_duration(&self, s: &str) -> Option<Duration> {
        if s.starts_with("PT") {
            let s = s.strip_prefix("PT")?;
            let mut hours = 0u64;
            let mut minutes = 0u64;
            let mut seconds = 0.0f64;
            
            // Parse hours
            if let Some(h_end) = s.find('H') {
                hours = s[..h_end].parse().ok()?;
                let remainder = &s[h_end + 1..];
                
                // Parse minutes
                if let Some(m_end) = remainder.find('M') {
                    minutes = remainder[..m_end].parse().ok()?;
                    let remainder = &remainder[m_end + 1..];
                    
                    // Parse seconds
                    if let Some(s_end) = remainder.find('S') {
                        seconds = remainder[..s_end].parse().ok()?;
                    }
                } else if let Some(s_end) = remainder.find('S') {
                    seconds = remainder[..s_end].parse().ok()?;
                }
            } else if let Some(m_end) = s.find('M') {
                minutes = s[..m_end].parse().ok()?;
                let remainder = &s[m_end + 1..];
                if let Some(s_end) = remainder.find('S') {
                    seconds = remainder[..s_end].parse().ok()?;
                }
            } else if let Some(s_end) = s.find('S') {
                seconds = s[..s_end].parse().ok()?;
            }
            
            let total_seconds = (hours * 3600) + (minutes * 60) + seconds as u64;
            return Some(Duration::from_secs(total_seconds));
        }
        None
    }

    /// Extract profiles list
    fn extract_profiles(&self, xml: &str) -> Vec<String> {
        if let Some(profiles_str) = self.extract_attribute(xml, "profiles") {
            profiles_str.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Extract element content
    fn extract_element_content(&self, xml: &str, tag: &str) -> Option<String> {
        let open_tag = format!("<{}", tag);
        let close_tag = format!("</{}>", tag);
        
        let start = xml.find(&open_tag)?;
        let content_start = xml[start..].find('>')? + start + 1;
        let content_end = xml.find(&close_tag)?;
        
        Some(xml[content_start..content_end].trim().to_string())
    }

    /// Parse periods
    fn parse_periods(&mut self, xml: &str) -> Result<Vec<Period>, String> {
        let mut periods = Vec::new();
        
        // Find all Period elements
        let period_pattern = regex::Regex::new(r"<Period[^>]*>(.*?)</Period>")
            .map_err(|e| e.to_string())?;
        
        for caps in period_pattern.captures_iter(xml) {
            if let (Some(period_tag), Some(period_content)) = (caps.get(0), caps.get(1)) {
                let period = self.parse_period(period_tag.as_str(), period_content.as_str())?;
                periods.push(period);
            }
        }
        
        if periods.is_empty() {
            // Try self-closing periods
            let self_closing = regex::Regex::new(r"<Period[^/]*/>")
                .map_err(|e| e.to_string())?;
            
            for caps in self_closing.captures_iter(xml) {
                if let Some(period_tag) = caps.get(0) {
                    let period = self.parse_period(period_tag.as_str(), "")?;
                    periods.push(period);
                }
            }
        }
        
        Ok(periods)
    }

    /// Parse a single period
    fn parse_period(&mut self, tag: &str, content: &str) -> Result<Period, String> {
        Ok(Period {
            id: self.extract_attribute(tag, "id"),
            start: self.extract_duration(tag, "start"),
            duration: self.extract_duration(tag, "duration"),
            bitstream_switching: self.extract_attribute(tag, "bitstreamSwitching")
                .and_then(|s| s.parse().ok()),
            base_url: if content.contains("<BaseURL>") {
                self.extract_element_content(content, "BaseURL")
            } else {
                None
            },
            adaptation_sets: self.parse_adaptation_sets(content)?,
            subsets: Vec::new(),
            supplemental_properties: Vec::new(),
        })
    }

    /// Parse adaptation sets
    fn parse_adaptation_sets(&mut self, period_content: &str) -> Result<Vec<AdaptationSet>, String> {
        let mut adaptation_sets = Vec::new();
        
        let as_pattern = regex::Regex::new(r"<AdaptationSet[^>]*>(.*?)</AdaptationSet>")
            .map_err(|e| e.to_string())?;
        
        for caps in as_pattern.captures_iter(period_content) {
            if let (Some(as_tag), Some(as_content)) = (caps.get(0), caps.get(1)) {
                let adaptation_set = self.parse_adaptation_set(as_tag.as_str(), as_content.as_str())?;
                adaptation_sets.push(adaptation_set);
            }
        }
        
        // Self-closing adaptation sets
        if adaptation_sets.is_empty() {
            let self_closing = regex::Regex::new(r"<AdaptationSet[^/]*/>")
                .map_err(|e| e.to_string())?;
            
            for caps in self_closing.captures_iter(period_content) {
                if let Some(as_tag) = caps.get(0) {
                    let adaptation_set = self.parse_adaptation_set(as_tag.as_str(), "")?;
                    adaptation_sets.push(adaptation_set);
                }
            }
        }
        
        Ok(adaptation_sets)
    }

    /// Parse a single adaptation set
    fn parse_adaptation_set(&mut self, tag: &str, content: &str) -> Result<AdaptationSet, String> {
        let content_type = self.extract_attribute(tag, "contentType")
            .and_then(|s| match s.to_lowercase().as_str() {
                "video" => Some(ContentType::Video),
                "audio" => Some(ContentType::Audio),
                "text" => Some(ContentType::Text),
                "image" => Some(ContentType::Image),
                "application" => Some(ContentType::Application),
                "font" => Some(ContentType::Font),
                _ => Some(ContentType::Unknown),
            });
        
        // Try to infer content type from MIME type if not specified
        let content_type = content_type.or_else(|| {
            self.extract_attribute(tag, "mimeType").and_then(|mt| {
                if mt.starts_with("video/") {
                    Some(ContentType::Video)
                } else if mt.starts_with("audio/") {
                    Some(ContentType::Audio)
                } else if mt.starts_with("text/") {
                    Some(ContentType::Text)
                } else if mt.starts_with("image/") {
                    Some(ContentType::Image)
                } else {
                    None
                }
            })
        });
        
        Ok(AdaptationSet {
            id: self.extract_attribute(tag, "id"),
            content_type,
            mime_type: self.extract_attribute(tag, "mimeType"),
            codecs: self.extract_attribute(tag, "codecs"),
            lang: self.extract_attribute(tag, "lang"),
            max_width: self.extract_attribute(tag, "maxWidth").and_then(|s| s.parse().ok()),
            max_height: self.extract_attribute(tag, "maxHeight").and_then(|s| s.parse().ok()),
            max_frame_rate: self.extract_attribute(tag, "maxFrameRate").and_then(|s| self.parse_frame_rate(&s)),
            par: self.extract_attribute(tag, "par"),
            segment_alignment: self.extract_attribute(tag, "segmentAlignment").and_then(|s| s.parse().ok()),
            subsegment_alignment: self.extract_attribute(tag, "subsegmentAlignment").and_then(|s| s.parse().ok()),
            bitstream_switching: self.extract_attribute(tag, "bitstreamSwitching").and_then(|s| s.parse().ok()),
            selection_priority: self.extract_attribute(tag, "selectionPriority").and_then(|s| s.parse().ok()),
            base_url: if content.contains("<BaseURL>") {
                self.extract_element_content(content, "BaseURL")
            } else {
                None
            },
            representations: self.parse_representations(content)?,
            content_protection: self.parse_content_protection(content)?,
            accessibility: Vec::new(),
            roles: self.parse_roles(content),
            ratings: Vec::new(),
            viewpoints: Vec::new(),
            supplemental_properties: Vec::new(),
            essential_properties: Vec::new(),
        })
    }

    /// Parse frame rate string (e.g., "30", "30000/1001")
    fn parse_frame_rate(&self, s: &str) -> Option<f64> {
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() == 2 {
                let num: f64 = parts[0].parse().ok()?;
                let den: f64 = parts[1].parse().ok()?;
                return Some(num / den);
            }
        }
        s.parse().ok()
    }

    /// Parse representations
    fn parse_representations(&mut self, as_content: &str) -> Result<Vec<Representation>, String> {
        let mut representations = Vec::new();
        
        let rep_pattern = regex::Regex::new(r"<Representation[^>]*>(.*?)</Representation>")
            .map_err(|e| e.to_string())?;
        
        for caps in rep_pattern.captures_iter(as_content) {
            if let (Some(rep_tag), Some(rep_content)) = (caps.get(0), caps.get(1)) {
                let representation = self.parse_representation(rep_tag.as_str(), rep_content.as_str())?;
                representations.push(representation);
            }
        }
        
        // Self-closing representations
        let self_closing = regex::Regex::new(r"<Representation[^/]*/>")
            .map_err(|e| e.to_string())?;
        
        for caps in self_closing.captures_iter(as_content) {
            if let Some(rep_tag) = caps.get(0) {
                let representation = self.parse_representation(rep_tag.as_str(), "")?;
                representations.push(representation);
            }
        }
        
        Ok(representations)
    }

    /// Parse a single representation
    fn parse_representation(&mut self, tag: &str, content: &str) -> Result<Representation, String> {
        let id = self.extract_attribute(tag, "id")
            .ok_or_else(|| "Representation missing id attribute".to_string())?;
        
        let bandwidth = self.extract_attribute(tag, "bandwidth")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        
        Ok(Representation {
            id,
            bandwidth,
            width: self.extract_attribute(tag, "width").and_then(|s| s.parse().ok()),
            height: self.extract_attribute(tag, "height").and_then(|s| s.parse().ok()),
            frame_rate: self.extract_attribute(tag, "frameRate").and_then(|s| self.parse_frame_rate(&s)),
            sample_rate: self.extract_attribute(tag, "audioSamplingRate").and_then(|s| s.parse().ok()),
            num_channels: self.extract_attribute(tag, "numChannels").and_then(|s| s.parse().ok()),
            mime_type: self.extract_attribute(tag, "mimeType"),
            codecs: self.extract_attribute(tag, "codecs"),
            sar: self.extract_attribute(tag, "sar"),
            base_url: if content.contains("<BaseURL>") {
                self.extract_element_content(content, "BaseURL")
            } else {
                None
            },
            segment_base: self.parse_segment_base(content),
            segment_template: self.parse_segment_template(content),
            segment_list: self.parse_segment_list(content),
            content_protection: self.parse_content_protection(content)?,
            supplemental_properties: Vec::new(),
            essential_properties: Vec::new(),
            sub_representations: Vec::new(),
        })
    }

    /// Parse segment base
    fn parse_segment_base(&self, content: &str) -> Option<SegmentBase> {
        let sb_pattern = regex::Regex::new(r"<SegmentBase[^>]*>").ok()?;
        let caps = sb_pattern.captures(content)?;
        let tag = caps.get(0)?.as_str();
        
        Some(SegmentBase {
            timescale: self.extract_attribute(tag, "timescale").and_then(|s| s.parse().ok()),
            presentation_time_offset: self.extract_attribute(tag, "presentationTimeOffset").and_then(|s| s.parse().ok()),
            index_range: self.extract_attribute(tag, "indexRange"),
            index_range_exact: self.extract_attribute(tag, "indexRangeExact").and_then(|s| s.parse().ok()),
            initialization: self.parse_initialization(content),
            representation_index: None,
        })
    }

    /// Parse segment template
    fn parse_segment_template(&self, content: &str) -> Option<SegmentTemplate> {
        let st_pattern = regex::Regex::new(r"<SegmentTemplate[^>]*>").ok()?;
        let caps = st_pattern.captures(content)?;
        let tag = caps.get(0)?.as_str();
        
        Some(SegmentTemplate {
            timescale: self.extract_attribute(tag, "timescale").and_then(|s| s.parse().ok()),
            duration: self.extract_attribute(tag, "duration").and_then(|s| s.parse().ok()),
            start_number: self.extract_attribute(tag, "startNumber").and_then(|s| s.parse().ok()),
            presentation_time_offset: self.extract_attribute(tag, "presentationTimeOffset").and_then(|s| s.parse().ok()),
            media: self.extract_attribute(tag, "media"),
            index: self.extract_attribute(tag, "index"),
            initialization: self.extract_attribute(tag, "initialization"),
            bitstream_switching: self.extract_attribute(tag, "bitstreamSwitching"),
            segment_timeline: self.parse_segment_timeline(content),
        })
    }

    /// Parse segment list
    fn parse_segment_list(&self, content: &str) -> Option<SegmentList> {
        let sl_pattern = regex::Regex::new(r"<SegmentList[^>]*>(.*?)</SegmentList>").ok()?;
        let caps = sl_pattern.captures(content)?;
        let tag = caps.get(0)?.as_str();
        let sl_content = caps.get(1)?.as_str();
        
        let segments = self.parse_segment_urls(sl_content);
        
        Some(SegmentList {
            timescale: self.extract_attribute(tag, "timescale").and_then(|s| s.parse().ok()),
            duration: self.extract_attribute(tag, "duration").and_then(|s| s.parse().ok()),
            start_number: self.extract_attribute(tag, "startNumber").and_then(|s| s.parse().ok()),
            presentation_time_offset: self.extract_attribute(tag, "presentationTimeOffset").and_then(|s| s.parse().ok()),
            segments,
            initialization: self.parse_initialization(content),
        })
    }

    /// Parse segment URLs
    fn parse_segment_urls(&self, content: &str) -> Vec<SegmentURL> {
        let mut urls = Vec::new();
        
        let su_pattern = regex::Regex::new(r"<SegmentURL[^/]*/>");
        if let Ok(pattern) = su_pattern {
            for caps in pattern.captures_iter(content) {
                if let Some(tag) = caps.get(0) {
                    urls.push(SegmentURL {
                        media: self.extract_attribute(tag.as_str(), "media"),
                        media_range: self.extract_attribute(tag.as_str(), "mediaRange"),
                        index: self.extract_attribute(tag.as_str(), "index"),
                        index_range: self.extract_attribute(tag.as_str(), "indexRange"),
                    });
                }
            }
        }
        
        urls
    }

    /// Parse initialization segment
    fn parse_initialization(&self, content: &str) -> Option<URL> {
        let init_pattern = regex::Regex::new(r"<Initialization[^/]*/>").ok()?;
        let caps = init_pattern.captures(content)?;
        let tag = caps.get(0)?.as_str();
        
        Some(URL {
            url: self.extract_attribute(tag, "sourceURL")?,
            service_location: self.extract_attribute(tag, "range"),
            range: self.extract_attribute(tag, "range"),
        })
    }

    /// Parse segment timeline
    fn parse_segment_timeline(&self, content: &str) -> Option<SegmentTimeline> {
        let stl_pattern = regex::Regex::new(r"<SegmentTimeline>(.*?)</SegmentTimeline>").ok()?;
        let caps = stl_pattern.captures(content)?;
        let stl_content = caps.get(1)?.as_str();
        
        let mut entries = Vec::new();
        let s_pattern = regex::Regex::new(r"<S[^/]*/>").ok()?;
        
        for caps in s_pattern.captures_iter(stl_content) {
            if let Some(tag) = caps.get(0) {
                let tag_str = tag.as_str();
                entries.push(TimelineEntry {
                    t: self.extract_attribute(tag_str, "t").and_then(|s| s.parse().ok()),
                    r: self.extract_attribute(tag_str, "r").and_then(|s| s.parse().ok()),
                    d: self.extract_attribute(tag_str, "d").and_then(|s| s.parse().ok())?,
                });
            }
        }
        
        Some(SegmentTimeline { entries })
    }

    /// Parse content protection elements
    fn parse_content_protection(&mut self, content: &str) -> Result<Vec<ContentProtection>, String> {
        let mut protections = Vec::new();
        
        let cp_pattern = regex::Regex::new(r"<ContentProtection[^>]*>(.*?)</ContentProtection>")
            .map_err(|e| e.to_string())?;
        
        for caps in cp_pattern.captures_iter(content) {
            if let (Some(tag), Some(_cp_content)) = (caps.get(0), caps.get(1)) {
                let tag_str = tag.as_str();
                
                if let Some(scheme_id) = self.extract_attribute(tag_str, "schemeIdUri") {
                    let protection = ContentProtection {
                        scheme_id,
                        value: self.extract_attribute(tag_str, "value"),
                        kid: self.extract_attribute(tag_str, "kid"),
                        default_kid: self.extract_attribute(tag_str, "default_KID"),
                        robustness: None,
                        cenc: None,
                        playready: None,
                        widevine: None,
                        clearkey: None,
                        marlin: None,
                    };
                    protections.push(protection);
                }
            }
        }
        
        // Self-closing content protection
        let self_closing = regex::Regex::new(r"<ContentProtection[^/]*/>")
            .map_err(|e| e.to_string())?;
        
        for caps in self_closing.captures_iter(content) {
            if let Some(tag) = caps.get(0) {
                let tag_str = tag.as_str();
                
                if let Some(scheme_id) = self.extract_attribute(tag_str, "schemeIdUri") {
                    let protection = ContentProtection {
                        scheme_id,
                        value: self.extract_attribute(tag_str, "value"),
                        kid: self.extract_attribute(tag_str, "kid"),
                        default_kid: self.extract_attribute(tag_str, "default_KID"),
                        robustness: None,
                        cenc: None,
                        playready: None,
                        widevine: None,
                        clearkey: None,
                        marlin: None,
                    };
                    protections.push(protection);
                }
            }
        }
        
        Ok(protections)
    }

    /// Parse role elements
    fn parse_roles(&self, content: &str) -> Vec<Descriptor> {
        let mut roles = Vec::new();
        
        let role_pattern = regex::Regex::new(r"<Role[^/]*/>");
        if let Ok(pattern) = role_pattern {
            for caps in pattern.captures_iter(content) {
                if let Some(tag) = caps.get(0) {
                    roles.push(Descriptor {
                        scheme_id: self.extract_attribute(tag.as_str(), "schemeIdUri"),
                        value: self.extract_attribute(tag.as_str(), "value"),
                        id: None,
                        content: None,
                    });
                }
            }
        }
        
        roles
    }

    /// Parse program information
    fn parse_program_info(&self, xml: &str) -> Option<ProgramInformation> {
        let pi_pattern = regex::Regex::new(r"<ProgramInformation[^>]*>(.*?)</ProgramInformation>").ok()?;
        let caps = pi_pattern.captures(xml)?;
        let content = caps.get(1)?.as_str();
        
        Some(ProgramInformation {
            title: self.extract_element_content(content, "Title"),
            source: self.extract_element_content(content, "Source"),
            copyright: self.extract_element_content(content, "Copyright"),
            more_info: None,
            metadata: HashMap::new(),
        })
    }

    /// Parse location URLs
    fn parse_locations(&self, xml: &str) -> Vec<String> {
        let mut locations = Vec::new();
        
        let loc_pattern = regex::Regex::new(r"<Location>(.*?)</Location>");
        if let Ok(pattern) = loc_pattern {
            for caps in pattern.captures_iter(xml) {
                if let Some(loc) = caps.get(1) {
                    locations.push(loc.as_str().trim().to_string());
                }
            }
        }
        
        locations
    }

    /// Get last parsing errors
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

impl Default for MPDParser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DASH Handler
// ============================================================================

/// DASH stream handler
pub struct DASHHandler {
    /// MPD parser
    parser: MPDParser,
    /// Parsed MPD
    mpd: Option<MediaPresentationDescription>,
    /// HTTP client
    client: reqwest::blocking::Client,
    /// Current period index
    current_period: usize,
    /// Current adaptation set per content type
    current_adaptation_sets: HashMap<ContentType, usize>,
    /// Current representation per adaptation set
    current_representations: HashMap<String, String>,
    /// Segment cache
    segment_cache: HashMap<String, Vec<u8>>,
}

impl DASHHandler {
    /// Create new DASH handler
    pub fn new() -> Self {
        Self {
            parser: MPDParser::new(),
            mpd: None,
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::blocking::Client::new()),
            current_period: 0,
            current_adaptation_sets: HashMap::new(),
            current_representations: HashMap::new(),
            segment_cache: HashMap::new(),
        }
    }

    /// Load MPD from URL
    pub fn load_mpd(&mut self, url: &str) -> Result<MediaPresentationDescription, String> {
        let response = self.client
            .get(url)
            .send()
            .map_err(|e| format!("Failed to fetch MPD: {}", e))?;
        
        let xml = response.text()
            .map_err(|e| format!("Failed to read MPD response: {}", e))?;
        
        let mpd = self.parser.parse(&xml)?;
        self.mpd = Some(mpd.clone());
        
        // Initialize default representations
        self.initialize_representations();
        
        Ok(mpd)
    }

    /// Load MPD from XML string
    pub fn parse_mpd(&mut self, xml: &str) -> Result<MediaPresentationDescription, String> {
        let mpd = self.parser.parse(xml)?;
        self.mpd = Some(mpd.clone());
        self.initialize_representations();
        Ok(mpd)
    }

    /// Initialize default representations
    fn initialize_representations(&mut self) {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                for (as_idx, adaptation_set) in period.adaptation_sets.iter().enumerate() {
                    if let Some(content_type) = adaptation_set.content_type {
                        self.current_adaptation_sets.insert(content_type, as_idx);
                    }
                    
                    // Select highest bandwidth representation by default
                    if let Some(highest_bw) = adaptation_set.representations
                        .iter()
                        .max_by_key(|r| r.bandwidth)
                    {
                        self.current_representations.insert(
                            highest_bw.id.clone(),
                            highest_bw.id.clone(),
                        );
                    }
                }
            }
        }
    }

    /// Get MPD
    pub fn mpd(&self) -> Option<&MediaPresentationDescription> {
        self.mpd.as_ref()
    }

    /// Get all video representations
    pub fn get_video_representations(&self) -> Vec<&Representation> {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                return period.adaptation_sets
                    .iter()
                    .filter(|as_| as_.content_type == Some(ContentType::Video))
                    .flat_map(|as_| as_.representations.iter())
                    .collect();
            }
        }
        Vec::new()
    }

    /// Get all audio representations
    pub fn get_audio_representations(&self) -> Vec<&Representation> {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                return period.adaptation_sets
                    .iter()
                    .filter(|as_| as_.content_type == Some(ContentType::Audio))
                    .flat_map(|as_| as_.representations.iter())
                    .collect();
            }
        }
        Vec::new()
    }

    /// Select video representation by ID
    pub fn select_video_representation(&mut self, id: &str) -> Result<(), String> {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                for adaptation_set in &period.adaptation_sets {
                    if adaptation_set.content_type == Some(ContentType::Video) {
                        if adaptation_set.representations.iter().any(|r| r.id == id) {
                            self.current_representations.insert("video".to_string(), id.to_string());
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(format!("Video representation '{}' not found", id))
    }

    /// Select audio representation by ID
    pub fn select_audio_representation(&mut self, id: &str) -> Result<(), String> {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                for adaptation_set in &period.adaptation_sets {
                    if adaptation_set.content_type == Some(ContentType::Audio) {
                        if adaptation_set.representations.iter().any(|r| r.id == id) {
                            self.current_representations.insert("audio".to_string(), id.to_string());
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(format!("Audio representation '{}' not found", id))
    }

    /// Get segment URL for a given position
    pub fn get_segment_url(&self, representation_id: &str, segment_number: u64) -> Option<String> {
        let mpd = self.mpd.as_ref()?;
        let period = mpd.periods.first()?;
        
        // Find the representation
        for adaptation_set in &period.adaptation_sets {
            for rep in &adaptation_set.representations {
                if rep.id == representation_id {
                    return self.build_segment_url(mpd, period, adaptation_set, rep, segment_number);
                }
            }
        }
        
        None
    }

    /// Build segment URL from template
    fn build_segment_url(
        &self,
        mpd: &MediaPresentationDescription,
        period: &Period,
        adaptation_set: &AdaptationSet,
        representation: &Representation,
        segment_number: u64,
    ) -> Option<String> {
        // Get segment template from representation, adaptation set, or period
        let template = representation.segment_template.as_ref()
            .or_else(|| adaptation_set.segment_template.as_ref());
        
        if let Some(template) = template {
            let media_template = template.media.as_ref()?;
            
            // Replace template placeholders
            let mut url = media_template.clone();
            url = url.replace("$RepresentationID$", &representation.id);
            url = url.replace("$Bandwidth$", &representation.bandwidth.to_string());
            url = url.replace("$Number$", &segment_number.to_string());
            
            // Handle $Number%0Xd$ format
            let number_pattern = regex::Regex::new(r"\$Number%(\d+)d\$").ok()?;
            if let Some(caps) = number_pattern.captures(&url) {
                if let Some(digits) = caps.get(1) {
                    let width: usize = digits.as_str().parse().ok()?;
                    let formatted = format!("{:0>width$}", segment_number, width = width);
                    url = number_pattern.replace(&url, &formatted).to_string();
                }
            }
            
            // Prepend base URLs
            let mut base = String::new();
            if let Some(ref mpd_base) = mpd.base_url {
                base.push_str(mpd_base);
            }
            if let Some(ref period_base) = period.base_url {
                base.push_str(period_base);
            }
            if let Some(ref as_base) = adaptation_set.base_url {
                base.push_str(as_base);
            }
            if let Some(ref rep_base) = representation.base_url {
                base.push_str(rep_base);
            }
            
            if !base.is_empty() {
                return Some(format!("{}{}", base, url));
            }
            
            return Some(url);
        }
        
        // For SegmentList, get URL from segment list
        if let Some(ref segment_list) = representation.segment_list {
            let idx = (segment_number.saturating_sub(segment_list.start_number.unwrap_or(1))) as usize;
            if let Some(segment) = segment_list.segments.get(idx) {
                if let Some(ref media) = segment.media {
                    return Some(media.clone());
                }
            }
        }
        
        // For SegmentBase or BaseURL, return the base URL
        if let Some(ref base_url) = representation.base_url {
            return Some(base_url.clone());
        }
        
        None
    }

    /// Download a segment
    pub fn download_segment(&mut self, url: &str) -> Result<Vec<u8>, String> {
        // Check cache first
        if let Some(cached) = self.segment_cache.get(url) {
            return Ok(cached.clone());
        }
        
        let response = self.client
            .get(url)
            .send()
            .map_err(|e| format!("Failed to download segment: {}", e))?;
        
        let data = response.bytes()
            .map_err(|e| format!("Failed to read segment data: {}", e))?
            .to_vec();
        
        // Cache segment
        self.segment_cache.insert(url.to_string(), data.clone());
        
        Ok(data)
    }

    /// Get available bitrates for video
    pub fn get_available_video_bitrates(&self) -> Vec<u64> {
        self.get_video_representations()
            .iter()
            .map(|r| r.bandwidth)
            .collect()
    }

    /// Get content protection information
    pub fn get_content_protection(&self) -> Vec<&ContentProtection> {
        if let Some(ref mpd) = self.mpd {
            if let Some(period) = mpd.periods.first() {
                return period.adaptation_sets
                    .iter()
                    .flat_map(|as_| as_.content_protection.iter())
                    .collect();
            }
        }
        Vec::new()
    }

    /// Check if stream is live
    pub fn is_live(&self) -> bool {
        self.mpd.as_ref().map(|m| m.mpd_type == MPDType::Dynamic).unwrap_or(false)
    }

    /// Get stream duration (for VOD)
    pub fn duration(&self) -> Option<Duration> {
        self.mpd.as_ref().and_then(|m| m.duration)
    }

    /// Clear segment cache
    pub fn clear_cache(&mut self) {
        self.segment_cache.clear();
    }
}

impl Default for DASHHandler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Resolve relative URL against base URL
pub fn resolve_url(base: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") {
        return relative.to_string();
    }
    
    if relative.starts_with("//") {
        if let Ok(parsed) = url::Url::parse(base) {
            return format!("{}:{}", parsed.scheme(), relative);
        }
    }
    
    if let Ok(base_url) = url::Url::parse(base) {
        if let Ok(resolved) = base_url.join(relative) {
            return resolved.to_string();
        }
    }
    
    format!("{}{}", base.trim_end_matches('/'), relative)
}

/// Check if URL is a DASH manifest
pub fn is_dash_url(url: &str) -> bool {
    url.to_lowercase().ends_with(".mpd") || 
        url.contains(".mpd?") ||
        url.contains("manifest(format=mpd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso8601_duration() {
        let parser = MPDParser::new();
        
        assert_eq!(parser.parse_iso8601_duration("PT1H30M5S"), Some(Duration::from_secs(5405)));
        assert_eq!(parser.parse_iso8601_duration("PT30S"), Some(Duration::from_secs(30)));
        assert_eq!(parser.parse_iso8601_duration("PT1M"), Some(Duration::from_secs(60)));
        assert_eq!(parser.parse_iso8601_duration("PT1H"), Some(Duration::from_secs(3600)));
    }

    #[test]
    fn test_parse_frame_rate() {
        let parser = MPDParser::new();
        
        assert_eq!(parser.parse_frame_rate("30"), Some(30.0));
        assert_eq!(parser.parse_frame_rate("30000/1001"), Some(29.97));
        assert_eq!(parser.parse_frame_rate("60"), Some(60.0));
    }

    #[test]
    fn test_is_dash_url() {
        assert!(is_dash_url("https://example.com/stream.mpd"));
        assert!(is_dash_url("https://example.com/stream.mpd?token=abc"));
        assert!(!is_dash_url("https://example.com/stream.m3u8"));
    }

    #[test]
    fn test_resolve_url() {
        assert_eq!(
            resolve_url("https://example.com/path/", "segment.ts"),
            "https://example.com/path/segment.ts"
        );
        assert_eq!(
            resolve_url("https://example.com/path/", "https://other.com/segment.ts"),
            "https://other.com/segment.ts"
        );
    }
}