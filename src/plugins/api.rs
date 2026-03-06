// Plugin API - Core interface for plugins
// Inspired by VideoLAN's plugin API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Current plugin API version
pub const PLUGIN_API_VERSION: u32 = 1;

/// Plugin capability - what the plugin can do
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PluginCapability {
    // Decoders
    AudioDecoder,
    VideoDecoder,
    SubtitleDecoder,

    // Encoders
    AudioEncoder,
    VideoEncoder,
    SubtitleEncoder,

    // Filters
    AudioFilter,
    VideoFilter,
    SubtitleFilter,

    // Demuxers (Input)
    Demuxer,

    // Muxers (Output)
    Muxer,

    // Network protocols
    NetworkProtocol,

    // UI Components
    UIComponent,
    Theme,
    Visualizer,

    // Extensions
    Extension,
    Script,

    // Advanced features
    AIModel,
    HardwareAcceleration,
}

impl PluginCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluginCapability::AudioDecoder => "audio_decoder",
            PluginCapability::VideoDecoder => "video_decoder",
            PluginCapability::SubtitleDecoder => "subtitle_decoder",
            PluginCapability::AudioEncoder => "audio_encoder",
            PluginCapability::VideoEncoder => "video_encoder",
            PluginCapability::SubtitleEncoder => "subtitle_encoder",
            PluginCapability::AudioFilter => "audio_filter",
            PluginCapability::VideoFilter => "video_filter",
            PluginCapability::SubtitleFilter => "subtitle_filter",
            PluginCapability::Demuxer => "demuxer",
            PluginCapability::Muxer => "muxer",
            PluginCapability::NetworkProtocol => "network_protocol",
            PluginCapability::UIComponent => "ui_component",
            PluginCapability::Theme => "theme",
            PluginCapability::Visualizer => "visualizer",
            PluginCapability::Extension => "extension",
            PluginCapability::Script => "script",
            PluginCapability::AIModel => "ai_model",
            PluginCapability::HardwareAcceleration => "hardware_acceleration",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "audio_decoder" => Some(PluginCapability::AudioDecoder),
            "video_decoder" => Some(PluginCapability::VideoDecoder),
            "subtitle_decoder" => Some(PluginCapability::SubtitleDecoder),
            "audio_encoder" => Some(PluginCapability::AudioEncoder),
            "video_encoder" => Some(PluginCapability::VideoEncoder),
            "subtitle_encoder" => Some(PluginCapability::SubtitleEncoder),
            "audio_filter" => Some(PluginCapability::AudioFilter),
            "video_filter" => Some(PluginCapability::VideoFilter),
            "subtitle_filter" => Some(PluginCapability::SubtitleFilter),
            "demuxer" => Some(PluginCapability::Demuxer),
            "muxer" => Some(PluginCapability::Muxer),
            "network_protocol" => Some(PluginCapability::NetworkProtocol),
            "ui_component" => Some(PluginCapability::UIComponent),
            "theme" => Some(PluginCapability::Theme),
            "visualizer" => Some(PluginCapability::Visualizer),
            "extension" => Some(PluginCapability::Extension),
            "script" => Some(PluginCapability::Script),
            "ai_model" => Some(PluginCapability::AIModel),
            "hardware_acceleration" => Some(PluginCapability::HardwareAcceleration),
            _ => None,
        }
    }
}

/// Plugin metadata - information about the plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin unique identifier
    pub id: String,

    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin API version
    pub api_version: u32,

    /// Plugin description
    pub description: String,

    /// Plugin author
    pub author: String,

    /// Plugin license
    pub license: String,

    /// Plugin homepage
    pub homepage: Option<String>,

    /// Plugin capabilities
    pub capabilities: Vec<PluginCapability>,

    /// Plugin dependencies (other plugins)
    pub dependencies: Vec<String>,

    /// Supported media formats
    pub formats: Vec<String>,

    /// Plugin configuration options
    pub config_options: HashMap<String, ConfigOption>,
}

impl PluginMetadata {
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            api_version: PLUGIN_API_VERSION,
            description: String::new(),
            author: String::new(),
            license: String::new(),
            homepage: None,
            capabilities: Vec::new(),
            dependencies: Vec::new(),
            formats: Vec::new(),
            config_options: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn with_license(mut self, license: &str) -> Self {
        self.license = license.to_string();
        self
    }

    pub fn with_capability(mut self, capability: PluginCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    pub fn with_format(mut self, format: &str) -> Self {
        self.formats.push(format.to_string());
        self
    }

    pub fn with_config_option(mut self, key: &str, option: ConfigOption) -> Self {
        self.config_options.insert(key.to_string(), option);
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Plugin ID cannot be empty".to_string());
        }

        if self.name.is_empty() {
            return Err("Plugin name cannot be empty".to_string());
        }

        if self.api_version != PLUGIN_API_VERSION {
            return Err(format!(
                "Plugin API version mismatch: expected {}, got {}",
                PLUGIN_API_VERSION, self.api_version
            ));
        }

        if self.capabilities.is_empty() {
            return Err("Plugin must have at least one capability".to_string());
        }

        Ok(())
    }
}

/// Configuration option for plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub description: String,
    pub value_type: ConfigValueType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<serde_json::Value>>,
}

/// Configuration value type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValueType {
    String,
    Integer,
    Float,
    Boolean,
    Enum,
    List,
}

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    Unloaded,
    Loaded,
    Active,
    Error,
}

/// Plugin information - runtime information
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub metadata: PluginMetadata,
    pub state: PluginState,
    pub load_time: std::time::Duration,
    pub error_message: Option<String>,
}

/// Plugin trait - must be implemented by all plugins
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Get plugin configuration
    fn get_config(&self, key: &str) -> Option<serde_json::Value> {
        None
    }

    /// Set plugin configuration
    fn set_config(&mut self, key: &str, value: serde_json::Value) -> Result<(), String> {
        Err("Configuration not supported".to_string())
    }

    /// Process data (for filters, decoders, encoders)
    fn process(&mut self, input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(input.to_vec())
    }

    /// Get plugin statistics
    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }
}

/// Default plugin implementation (no-op)
pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn metadata(&self) -> &PluginMetadata {
        static METADATA: std::sync::OnceLock<PluginMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| {
            PluginMetadata::new("default", "Default Plugin", "1.0.0")
                .with_description("Default no-op plugin")
                .with_author("Vantis Team")
                .with_license("MIT")
                .with_capability(PluginCapability::Extension)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_as_str() {
        assert_eq!(PluginCapability::AudioDecoder.as_str(), "audio_decoder");
        assert_eq!(PluginCapability::VideoDecoder.as_str(), "video_decoder");
    }

    #[test]
    fn test_capability_from_str() {
        assert_eq!(
            PluginCapability::from_str("audio_decoder"),
            Some(PluginCapability::AudioDecoder)
        );
        assert_eq!(PluginCapability::from_str("unknown"), None);
    }

    #[test]
    fn test_plugin_metadata_builder() {
        let metadata = PluginMetadata::new("test", "Test Plugin", "1.0.0")
            .with_description("A test plugin")
            .with_author("Test Author")
            .with_license("MIT")
            .with_capability(PluginCapability::AudioDecoder)
            .with_format("mp3");

        assert_eq!(metadata.id, "test");
        assert_eq!(metadata.name, "Test Plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.description, "A test plugin");
        assert_eq!(metadata.author, "Test Author");
        assert_eq!(metadata.license, "MIT");
        assert!(metadata.capabilities.contains(&PluginCapability::AudioDecoder));
        assert!(metadata.formats.contains(&"mp3".to_string()));
    }

    #[test]
    fn test_plugin_metadata_validate() {
        let mut metadata = PluginMetadata::new("test", "Test Plugin", "1.0.0")
            .with_capability(PluginCapability::AudioDecoder);

        assert!(metadata.validate().is_ok());

        metadata.id = String::new();
        assert!(metadata.validate().is_err());

        metadata.id = "test".to_string();
        metadata.capabilities = Vec::new();
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_default_plugin() {
        let plugin = DefaultPlugin;
        let metadata = plugin.metadata();
        assert_eq!(metadata.id, "default");
        assert_eq!(metadata.name, "Default Plugin");
    }
}