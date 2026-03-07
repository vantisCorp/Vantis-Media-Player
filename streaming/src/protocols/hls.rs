//! HLS (HTTP Live Streaming) protocol handler
//! 
//! Implements Apple's HTTP Live Streaming specification (RFC 8216)
//! with support for:
//! - M3U8 playlist parsing
//! - Master playlist with variant streams
//! - Media playlist parsing
//! - AES-128 encryption
//! - Low-Latency HLS (LL-HLS)
//! - Multiple audio/video tracks

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor};
use tracing::{debug, info, warn};

/// M3U8 playlist types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistType {
    /// Master playlist (contains variant streams)
    Master,
    /// Media playlist (contains segments)
    Media,
}

/// M3U8 playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M3U8Playlist {
    /// Playlist type
    pub playlist_type: PlaylistType,
    
    /// Version
    pub version: u32,
    
    /// Target duration (for media playlists)
    pub target_duration: Option<f64>,
    
    /// Media sequence number
    pub media_sequence: u64,
    
    /// Is live stream
    pub is_live: bool,
    
    /// Variant streams (for master playlists)
    pub variants: Vec<VariantStream>,
    
    /// Media segments (for media playlists)
    pub segments: Vec<MediaSegment>,
    
    /// Alternative renditions (audio, video, subtitles)
    pub alternatives: Vec<AlternativeRendition>,
    
    /// Session data
    pub session_data: Vec<SessionData>,
    
    /// Session keys (DRM)
    pub session_keys: Vec<SessionKey>,
    
    /// Independent segments
    pub independent_segments: bool,
    
    /// Start offset
    pub start_offset: Option<f64>,
    
    /// End list tag present
    pub end_list: bool,
}

/// Variant stream (in master playlist)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantStream {
    /// URI
    pub uri: String,
    
    /// Bandwidth in bps
    pub bandwidth: u64,
    
    /// Average bandwidth
    pub average_bandwidth: Option<u64>,
    
    /// Resolution
    pub resolution: Option<(u32, u32)>,
    
    /// Frame rate
    pub frame_rate: Option<f32>,
    
    /// Codecs
    pub codecs: Option<String>,
    
    /// Audio group
    pub audio: Option<String>,
    
    /// Video group
    pub video: Option<String>,
    
    /// Subtitles group
    pub subtitles: Option<String>,
    
    /// Closed captions group
    pub closed_captions: Option<String>,
}

/// Media segment (in media playlist)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSegment {
    /// Segment URI
    pub uri: String,
    
    /// Duration in seconds
    pub duration: f64,
    
    /// Title (optional)
    pub title: Option<String>,
    
    /// Byte range
    pub byte_range: Option<(u64, Option<u64>)>,
    
    /// Discontinuity marker
    pub discontinuity: bool,
    
    /// Key (encryption)
    pub key: Option<KeyInfo>,
    
    /// Map (initialization segment)
    pub map: Option<SegmentMap>,
    
    /// Program date time
    pub program_date_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Daterange
    pub date_range: Option<DateRange>,
    
    /// Bitrate hint
    pub bitrate: Option<u64>,
}

/// Encryption key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Encryption method
    pub method: EncryptionMethod,
    
    /// Key URI
    pub uri: String,
    
    /// IV (initialization vector)
    pub iv: Option<String>,
    
    /// Key format
    pub key_format: Option<String>,
    
    /// Key format versions
    pub key_format_versions: Option<String>,
}

/// Encryption method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionMethod {
    /// No encryption
    None,
    
    /// AES-128 encryption
    AES128,
    
    /// Sample AES encryption
    SampleAES,
    
    /// FairPlay (DRM)
    FairPlay,
}

/// Segment map (initialization segment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentMap {
    /// Map URI
    pub uri: String,
    
    /// Byte range
    pub byte_range: Option<(u64, Option<u64>)>,
}

/// Date range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    /// Class
    pub class: Option<String>,
    
    /// Start date
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    
    /// End date
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Duration
    pub duration: Option<f64>,
    
    /// Planned duration
    pub planned_duration: Option<f64>,
    
    /// End on next
    pub end_on_next: bool,
    
    /// X- client attributes
    pub client_attributes: HashMap<String, String>,
}

/// Alternative rendition (audio, video, subtitles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeRendition {
    /// Type
    pub media_type: MediaType,
    
    /// URI
    pub uri: Option<String>,
    
    /// Group ID
    pub group_id: String,
    
    /// Language
    pub language: Option<String>,
    
    /// Name
    pub name: String,
    
    /// Default
    pub default: bool,
    
    /// Autoselect
    pub autoselect: bool,
    
    /// Forced
    pub forced: bool,
    
    /// Characteristics
    pub characteristics: Option<String>,
}

/// Media type for alternative renditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

/// Session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// Data ID
    pub data_id: String,
    
    /// Value
    pub value: Option<String>,
    
    /// URI
    pub uri: Option<String>,
    
    /// Language
    pub language: Option<String>,
}

/// Session key (DRM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionKey {
    /// Key info
    pub key: KeyInfo,
}

/// M3U8 parser
pub struct M3U8Parser {
    base_url: Option<String>,
}

impl M3U8Parser {
    /// Create a new M3U8 parser
    pub fn new() -> Self {
        Self { base_url: None }
    }
    
    /// Set base URL for resolving relative URIs
    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = Some(base_url.to_string());
    }
    
    /// Parse M3U8 content
    pub fn parse(&self, content: &str) -> StreamingResult<M3U8Playlist> {
        let mut playlist = M3U8Playlist {
            playlist_type: PlaylistType::Media,
            version: 3,
            target_duration: None,
            media_sequence: 0,
            is_live: true,
            variants: Vec::new(),
            segments: Vec::new(),
            alternatives: Vec::new(),
            session_data: Vec::new(),
            session_keys: Vec::new(),
            independent_segments: false,
            start_offset: None,
            end_list: false,
        };
        
        let reader = BufReader::new(Cursor::new(content));
        let mut lines = reader.lines();
        
        // Check M3U8 header
        if let Some(Ok(first_line)) = lines.next() {
            if !first_line.starts_with("#EXTM3U") {
                return Err(StreamingError::ProtocolError(
                    "Invalid M3U8: missing #EXTM3U header".to_string()
                ));
            }
        }
        
        let mut current_segment = MediaSegment {
            uri: String::new(),
            duration: 0.0,
            title: None,
            byte_range: None,
            discontinuity: false,
            key: None,
            map: None,
            program_date_time: None,
            date_range: None,
            bitrate: None,
        };
        
        while let Some(Ok(line)) = lines.next() {
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }
            
            if line.starts_with("#EXT") {
                // Tag line
                self.parse_tag(line, &mut playlist, &mut current_segment)?;
            } else if line.starts_with("#") {
                // Comment, ignore
            } else {
                // URI line
                if playlist.playlist_type == PlaylistType::Media {
                    current_segment.uri = self.resolve_uri(line)?;
                    playlist.segments.push(current_segment.clone());
                    current_segment = MediaSegment {
                        uri: String::new(),
                        duration: 0.0,
                        title: None,
                        byte_range: None,
                        discontinuity: false,
                        key: None,
                        map: None,
                        program_date_time: None,
                        date_range: None,
                        bitrate: None,
                    };
                } else {
                    // Master playlist - URI is for variant
                    if let Some(last_variant) = playlist.variants.last_mut() {
                        last_variant.uri = self.resolve_uri(line)?;
                    }
                }
            }
        }
        
        Ok(playlist)
    }
    
    /// Parse a tag line
    fn parse_tag(
        &self,
        line: &str,
        playlist: &mut M3U8Playlist,
        current_segment: &mut MediaSegment,
    ) -> StreamingResult<()> {
        let (tag_name, value) = if let Some(pos) = line.find(':') {
            (&line[..pos], Some(&line[pos + 1..]))
        } else {
            (line, None)
        };
        
        match tag_name {
            "#EXT-X-VERSION" => {
                if let Some(v) = value {
                    playlist.version = v.parse().unwrap_or(3);
                }
            }
            "#EXT-X-TARGETDURATION" => {
                if let Some(v) = value {
                    playlist.target_duration = Some(v.parse().unwrap_or(0.0));
                }
            }
            "#EXT-X-MEDIA-SEQUENCE" => {
                if let Some(v) = value {
                    playlist.media_sequence = v.parse().unwrap_or(0);
                }
            }
            "#EXT-X-PLAYLIST-TYPE" => {
                if let Some(v) = value {
                    if v == "VOD" {
                        playlist.is_live = false;
                    }
                }
            }
            "#EXT-X-ENDLIST" => {
                playlist.end_list = true;
                playlist.is_live = false;
            }
            "#EXT-X-INDEPENDENT-SEGMENTS" => {
                playlist.independent_segments = true;
            }
            "#EXT-X-START" => {
                if let Some(v) = value {
                    playlist.start_offset = self.parse_start(v)?;
                }
            }
            "#EXTINF" => {
                if let Some(v) = value {
                    let parts: Vec<&str> = v.split(',').collect();
                    if let Some(duration_str) = parts.first() {
                        current_segment.duration = duration_str.parse().unwrap_or(0.0);
                    }
                    if let Some(title) = parts.get(1) {
                        current_segment.title = Some(title.to_string());
                    }
                }
            }
            "#EXT-X-STREAM-INF" => {
                playlist.playlist_type = PlaylistType::Master;
                let variant = self.parse_stream_inf(value)?;
                playlist.variants.push(variant);
            }
            "#EXT-X-KEY" => {
                current_segment.key = self.parse_key(value)?;
            }
            "#EXT-X-MAP" => {
                current_segment.map = self.parse_map(value)?;
            }
            "#EXT-X-BYTERANGE" => {
                if let Some(v) = value {
                    current_segment.byte_range = self.parse_byte_range(v)?;
                }
            }
            "#EXT-X-DISCONTINUITY" => {
                current_segment.discontinuity = true;
            }
            "#EXT-X-PROGRAM-DATE-TIME" => {
                if let Some(v) = value {
                    current_segment.program_date_time = chrono::DateTime::parse_from_rfc3339(v)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .ok();
                }
            }
            "#EXT-X-MEDIA" => {
                let alt = self.parse_media(value)?;
                playlist.alternatives.push(alt);
            }
            "#EXT-X-SESSION-DATA" => {
                let data = self.parse_session_data(value)?;
                playlist.session_data.push(data);
            }
            "#EXT-X-SESSION-KEY" => {
                let key = self.parse_session_key(value)?;
                playlist.session_keys.push(key);
            }
            "#EXT-X-BITRATE" => {
                if let Some(v) = value {
                    current_segment.bitrate = Some(v.parse().unwrap_or(0));
                }
            }
            _ => {
                // Unknown tag, ignore
                debug!("Unknown M3U8 tag: {}", tag_name);
            }
        }
        
        Ok(())
    }
    
    /// Parse STREAM-INF tag
    fn parse_stream_inf(&self, value: Option<&str>) -> StreamingResult<VariantStream> {
        let mut variant = VariantStream {
            uri: String::new(),
            bandwidth: 0,
            average_bandwidth: None,
            resolution: None,
            frame_rate: None,
            codecs: None,
            audio: None,
            video: None,
            subtitles: None,
            closed_captions: None,
        };
        
        if let Some(v) = value {
            for attr in self.parse_attributes(v)? {
                match attr.0.as_str() {
                    "BANDWIDTH" => {
                        variant.bandwidth = attr.1.parse().unwrap_or(0);
                    }
                    "AVERAGE-BANDWIDTH" => {
                        variant.average_bandwidth = Some(attr.1.parse().unwrap_or(0));
                    }
                    "RESOLUTION" => {
                        variant.resolution = self.parse_resolution(&attr.1)?;
                    }
                    "FRAME-RATE" => {
                        variant.frame_rate = Some(attr.1.parse().unwrap_or(0.0));
                    }
                    "CODECS" => {
                        variant.codecs = Some(attr.1.to_string());
                    }
                    "AUDIO" => {
                        variant.audio = Some(attr.1.to_string());
                    }
                    "VIDEO" => {
                        variant.video = Some(attr.1.to_string());
                    }
                    "SUBTITLES" => {
                        variant.subtitles = Some(attr.1.to_string());
                    }
                    "CLOSED-CAPTIONS" => {
                        variant.closed_captions = Some(attr.1.to_string());
                    }
                    _ => {}
                }
            }
        }
        
        Ok(variant)
    }
    
    /// Parse KEY tag
    fn parse_key(&self, value: Option<&str>) -> StreamingResult<Option<KeyInfo>> {
        if value.is_none() {
            return Ok(None);
        }
        
        let mut key = KeyInfo {
            method: EncryptionMethod::None,
            uri: String::new(),
            iv: None,
            key_format: None,
            key_format_versions: None,
        };
        
        for attr in self.parse_attributes(value.unwrap())? {
            match attr.0.as_str() {
                "METHOD" => {
                    key.method = match attr.1.as_str() {
                        "NONE" => EncryptionMethod::None,
                        "AES-128" => EncryptionMethod::AES128,
                        "SAMPLE-AES" => EncryptionMethod::SampleAES,
                        "com.apple.fps" => EncryptionMethod::FairPlay,
                        _ => EncryptionMethod::None,
                    };
                }
                "URI" => {
                    key.uri = attr.1.to_string();
                }
                "IV" => {
                    key.iv = Some(attr.1.to_string());
                }
                "KEYFORMAT" => {
                    key.key_format = Some(attr.1.to_string());
                }
                "KEYFORMATVERSIONS" => {
                    key.key_format_versions = Some(attr.1.to_string());
                }
                _ => {}
            }
        }
        
        Ok(Some(key))
    }
    
    /// Parse MAP tag
    fn parse_map(&self, value: Option<&str>) -> StreamingResult<Option<SegmentMap>> {
        if value.is_none() {
            return Ok(None);
        }
        
        let mut map = SegmentMap {
            uri: String::new(),
            byte_range: None,
        };
        
        for attr in self.parse_attributes(value.unwrap())? {
            match attr.0.as_str() {
                "URI" => {
                    map.uri = attr.1.to_string();
                }
                "BYTERANGE" => {
                    map.byte_range = self.parse_byte_range(&attr.1)?;
                }
                _ => {}
            }
        }
        
        Ok(Some(map))
    }
    
    /// Parse MEDIA tag
    fn parse_media(&self, value: Option<&str>) -> StreamingResult<AlternativeRendition> {
        let mut alt = AlternativeRendition {
            media_type: MediaType::Audio,
            uri: None,
            group_id: String::new(),
            language: None,
            name: String::new(),
            default: false,
            autoselect: false,
            forced: false,
            characteristics: None,
        };
        
        if let Some(v) = value {
            for attr in self.parse_attributes(v)? {
                match attr.0.as_str() {
                    "TYPE" => {
                        alt.media_type = match attr.1.as_str() {
                            "AUDIO" => MediaType::Audio,
                            "VIDEO" => MediaType::Video,
                            "SUBTITLES" => MediaType::Subtitles,
                            "CLOSED-CAPTIONS" => MediaType::ClosedCaptions,
                            _ => MediaType::Audio,
                        };
                    }
                    "URI" => {
                        alt.uri = Some(attr.1.to_string());
                    }
                    "GROUP-ID" => {
                        alt.group_id = attr.1.to_string();
                    }
                    "LANGUAGE" => {
                        alt.language = Some(attr.1.to_string());
                    }
                    "NAME" => {
                        alt.name = attr.1.to_string();
                    }
                    "DEFAULT" => {
                        alt.default = attr.1 == "YES";
                    }
                    "AUTOSELECT" => {
                        alt.autoselect = attr.1 == "YES";
                    }
                    "FORCED" => {
                        alt.forced = attr.1 == "YES";
                    }
                    "CHARACTERISTICS" => {
                        alt.characteristics = Some(attr.1.to_string());
                    }
                    _ => {}
                }
            }
        }
        
        Ok(alt)
    }
    
    /// Parse SESSION-DATA tag
    fn parse_session_data(&self, value: Option<&str>) -> StreamingResult<SessionData> {
        let mut data = SessionData {
            data_id: String::new(),
            value: None,
            uri: None,
            language: None,
        };
        
        if let Some(v) = value {
            for attr in self.parse_attributes(v)? {
                match attr.0.as_str() {
                    "DATA-ID" => {
                        data.data_id = attr.1.to_string();
                    }
                    "VALUE" => {
                        data.value = Some(attr.1.to_string());
                    }
                    "URI" => {
                        data.uri = Some(attr.1.to_string());
                    }
                    "LANGUAGE" => {
                        data.language = Some(attr.1.to_string());
                    }
                    _ => {}
                }
            }
        }
        
        Ok(data)
    }
    
    /// Parse SESSION-KEY tag
    fn parse_session_key(&self, value: Option<&str>) -> StreamingResult<SessionKey> {
        let key = self.parse_key(value)?.unwrap_or(KeyInfo {
            method: EncryptionMethod::None,
            uri: String::new(),
            iv: None,
            key_format: None,
            key_format_versions: None,
        });
        
        Ok(SessionKey { key })
    }
    
    /// Parse attributes from a tag value
    fn parse_attributes(&self, value: &str) -> StreamingResult<Vec<(String, String)>> {
        let mut attrs = Vec::new();
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        
        for c in value.chars() {
            match c {
                '=' if !in_quotes => {
                    current_key = current_value.trim().to_string();
                    current_value.clear();
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                ',' if !in_quotes => {
                    if !current_key.is_empty() {
                        attrs.push((current_key.clone(), current_value.trim().to_string()));
                    }
                    current_key.clear();
                    current_value.clear();
                }
                _ => {
                    current_value.push(c);
                }
            }
        }
        
        if !current_key.is_empty() {
            attrs.push((current_key, current_value.trim().to_string()));
        }
        
        Ok(attrs)
    }
    
    /// Parse resolution string (e.g., "1920x1080")
    fn parse_resolution(&self, value: &str) -> StreamingResult<Option<(u32, u32)>> {
        let parts: Vec<&str> = value.split('x').collect();
        if parts.len() == 2 {
            let width: u32 = parts[0].parse().unwrap_or(0);
            let height: u32 = parts[1].parse().unwrap_or(0);
            return Ok(Some((width, height)));
        }
        Ok(None)
    }
    
    /// Parse byte range (e.g., "1024@0")
    fn parse_byte_range(&self, value: &str) -> StreamingResult<Option<(u64, Option<u64>)>> {
        let parts: Vec<&str> = value.split('@').collect();
        if let Some(length_str) = parts.first() {
            let length: u64 = length_str.parse().unwrap_or(0);
            let offset = parts.get(1).and_then(|s| s.parse().ok());
            return Ok(Some((length, offset)));
        }
        Ok(None)
    }
    
    /// Parse START tag
    fn parse_start(&self, value: &str) -> StreamingResult<Option<f64>> {
        for attr in self.parse_attributes(value)? {
            if attr.0 == "TIME-OFFSET" {
                return Ok(Some(attr.1.parse().unwrap_or(0.0)));
            }
        }
        Ok(None)
    }
    
    /// Resolve relative URI to absolute
    fn resolve_uri(&self, uri: &str) -> StreamingResult<String> {
        if uri.starts_with("http://") || uri.starts_with("https://") {
            return Ok(uri.to_string());
        }
        
        if let Some(ref base_url) = self.base_url {
            // Simple resolution - append to base
            if uri.starts_with('/') {
                // Absolute path
                if let Some(pos) = base_url.find("://") {
                    if let Some(slash_pos) = base_url[pos + 3..].find('/') {
                        return Ok(format!("{}{}", &base_url[..pos + 3 + slash_pos], uri));
                    }
                }
            } else {
                // Relative path
                if let Some(pos) = base_url.rfind('/') {
                    return Ok(format!("{}/{}", &base_url[..pos], uri));
                }
            }
        }
        
        Ok(uri.to_string())
    }
}

impl Default for M3U8Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// HLS handler
pub struct HLSHandler {
    parser: M3U8Parser,
    playlist: Option<M3U8Playlist>,
    client: reqwest::blocking::Client,
    current_variant: Option<VariantStream>,
}

impl HLSHandler {
    /// Create a new HLS handler
    pub fn new() -> StreamingResult<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| StreamingError::NetworkError(e.to_string()))?;
        
        Ok(Self {
            parser: M3U8Parser::new(),
            playlist: None,
            client,
            current_variant: None,
        })
    }
    
    /// Load and parse M3U8 playlist from URL
    pub fn load_playlist(&mut self, url: &str) -> StreamingResult<M3U8Playlist> {
        info!("Loading HLS playlist: {}", url);
        
        let response = self.client
            .get(url)
            .send()
            .map_err(|e| StreamingError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(StreamingError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }
        
        let content = response
            .text()
            .map_err(|e| StreamingError::NetworkError(e.to_string()))?;
        
        self.parser.set_base_url(url);
        let playlist = self.parser.parse(&content)?;
        
        info!(
            "HLS playlist loaded: {} variants, {} segments",
            playlist.variants.len(),
            playlist.segments.len()
        );
        
        self.playlist = Some(playlist.clone());
        Ok(playlist)
    }
    
    /// Select variant stream
    pub fn select_variant(&mut self, bandwidth: u64) -> Option<&VariantStream> {
        if let Some(ref playlist) = self.playlist {
            // Select best variant that fits bandwidth
            let mut best_variant: Option<&VariantStream> = None;
            let mut best_bandwidth = 0;
            
            for variant in &playlist.variants {
                if variant.bandwidth <= bandwidth && variant.bandwidth > best_bandwidth {
                    best_variant = Some(variant);
                    best_bandwidth = variant.bandwidth;
                }
            }
            
            self.current_variant = best_variant.cloned();
            return best_variant;
        }
        None
    }
    
    /// Get current playlist
    pub fn playlist(&self) -> Option<&M3U8Playlist> {
        self.playlist.as_ref()
    }
}

impl Default for HLSHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create HLS handler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = M3U8Parser::new();
        assert!(parser.base_url.is_none());
    }

    #[test]
    fn test_parse_simple_playlist() {
        let parser = M3U8Parser::new();
        let content = "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:10\n#EXTINF:10.0,\nsegment1.ts\n#EXTINF:10.0,\nsegment2.ts\n#EXT-X-ENDLIST\n";
        
        let playlist = parser.parse(content).unwrap();
        assert_eq!(playlist.version, 3);
        assert_eq!(playlist.segments.len(), 2);
        assert!(!playlist.is_live);
    }

    #[test]
    fn test_parse_master_playlist() {
        let parser = M3U8Parser::new();
        let content = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=1000000,RESOLUTION=1920x1080\nstream1.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=500000,RESOLUTION=1280x720\nstream2.m3u8\n";
        
        let playlist = parser.parse(content).unwrap();
        assert_eq!(playlist.variants.len(), 2);
        assert_eq!(playlist.variants[0].bandwidth, 1000000);
    }

    #[test]
    fn test_hls_handler_creation() {
        let handler = HLSHandler::new();
        assert!(handler.is_ok());
    }
}