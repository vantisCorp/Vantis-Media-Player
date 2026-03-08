//! Plugin manifest definition and loading.

use crate::error::{PluginError, PluginResult};
use crate::plugin::PluginId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Plugin manifest definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier.
    pub id: PluginId,
    /// Human-readable plugin name.
    pub name: String,
    /// Plugin version (semver).
    pub version: String,
    /// Plugin description.
    #[serde(default)]
    pub description: String,
    /// Plugin author.
    #[serde(default)]
    pub author: String,
    /// Plugin homepage URL.
    pub homepage: Option<String>,
    /// Plugin repository URL.
    pub repository: Option<String>,
    /// Plugin license (SPDX identifier).
    #[serde(default = "default_license")]
    pub license: String,
    /// Minimum host version required.
    #[serde(default = "default_version")]
    pub min_host_version: String,
    /// Plugin entry point (relative path to library).
    #[serde(default = "default_entry")]
    pub entry: String,
    /// Plugin dependencies.
    #[serde(default)]
    pub dependencies: Vec<PluginDependency>,
    /// Plugin permissions.
    #[serde(default)]
    pub permissions: Vec<PluginPermission>,
    /// Plugin configuration schema.
    #[serde(default)]
    pub config: Option<PluginConfigSchema>,
    /// Plugin keywords/tags.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// Plugin categories.
    #[serde(default)]
    pub categories: Vec<String>,
    /// Custom metadata.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Target platforms.
    #[serde(default)]
    pub platforms: Vec<Platform>,
    /// Runtime requirements.
    #[serde(default)]
    pub requirements: Option<RuntimeRequirements>,
}

fn default_license() -> String {
    "MIT".to_string()
}

fn default_version() -> String {
    "1.0.0".to_string()
}

fn default_entry() -> String {
    "plugin.so".to_string()
}

/// Plugin dependency definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin ID.
    pub plugin_id: PluginId,
    /// Required version (semver range).
    #[serde(default = "default_version_range")]
    pub version: String,
    /// Whether the dependency is optional.
    #[serde(default)]
    pub optional: bool,
}

fn default_version_range() -> String {
    "*".to_string()
}

impl PluginDependency {
    /// Create a new required dependency.
    pub fn required(id: impl Into<String>, version: impl Into<String>) -> Self {
        PluginDependency {
            plugin_id: PluginId::new(id),
            version: version.into(),
            optional: false,
        }
    }

    /// Create an optional dependency.
    pub fn optional(id: impl Into<String>, version: impl Into<String>) -> Self {
        PluginDependency {
            plugin_id: PluginId::new(id),
            version: version.into(),
            optional: true,
        }
    }
}

/// Plugin permission definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermission {
    /// Permission name.
    pub name: String,
    /// Permission reason (shown to user).
    pub reason: Option<String>,
    /// Permission is required (vs optional).
    #[serde(default)]
    pub required: bool,
}

impl PluginPermission {
    /// Create a new permission.
    pub fn new(name: impl Into<String>) -> Self {
        PluginPermission {
            name: name.into(),
            reason: None,
            required: true,
        }
    }

    /// Add a reason for the permission.
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Mark the permission as optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

/// Plugin configuration schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigSchema {
    /// Configuration schema version.
    #[serde(default = "default_schema_version")]
    pub version: String,
    /// Configuration properties.
    pub properties: HashMap<String, ConfigProperty>,
    /// Required properties.
    #[serde(default)]
    pub required: Vec<String>,
}

fn default_schema_version() -> String {
    "1.0".to_string()
}

/// Configuration property definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProperty {
    /// Property type.
    #[serde(rename = "type")]
    pub prop_type: ConfigType,
    /// Property title.
    pub title: String,
    /// Property description.
    #[serde(default)]
    pub description: String,
    /// Default value.
    pub default: Option<serde_json::Value>,
    /// Minimum value (for numbers).
    pub minimum: Option<f64>,
    /// Maximum value (for numbers).
    pub maximum: Option<f64>,
    /// Enum values (for string enums).
    #[serde(default)]
    pub enum_values: Vec<String>,
    /// Items schema (for arrays).
    pub items: Option<Box<ConfigProperty>>,
}

/// Configuration property types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

/// Supported platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "macos")]
    MacOS,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    IOS,
    #[serde(rename = "web")]
    Web,
}

impl Platform {
    /// Get the current platform.
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        { Platform::Windows }
        #[cfg(target_os = "macos")]
        { Platform::MacOS }
        #[cfg(target_os = "linux")]
        { Platform::Linux }
        #[cfg(target_os = "android")]
        { Platform::Android }
        #[cfg(target_os = "ios")]
        { Platform::IOS }
    }

    /// Check if this platform matches the current platform.
    pub fn is_current(&self) -> bool {
        *self == Self::current()
    }
}

/// Runtime requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeRequirements {
    /// Minimum memory required in MB.
    pub min_memory_mb: Option<u32>,
    /// Minimum CPU cores.
    pub min_cpu_cores: Option<u32>,
    /// Required GPU features.
    #[serde(default)]
    pub gpu_features: Vec<String>,
    /// Required system libraries.
    #[serde(default)]
    pub system_libraries: Vec<String>,
}

impl PluginManifest {
    /// Create a new manifest with the given ID and name.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        PluginManifest {
            id: PluginId::new(id),
            name: name.into(),
            version: "1.0.0".to_string(),
            description: String::new(),
            author: String::new(),
            homepage: None,
            repository: None,
            license: "MIT".to_string(),
            min_host_version: "1.0.0".to_string(),
            entry: "plugin.so".to_string(),
            dependencies: Vec::new(),
            permissions: Vec::new(),
            config: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            metadata: HashMap::new(),
            platforms: Vec::new(),
            requirements: None,
        }
    }

    /// Load manifest from a file.
    pub fn load(path: &Path) -> PluginResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::load_failed(path, e.to_string()))?;
        
        let manifest: PluginManifest = serde_json::from_str(&content)
            .map_err(|e| PluginError::InvalidManifest(e.to_string()))?;
        
        Ok(manifest)
    }

    /// Save manifest to a file.
    pub fn save(&self, path: &Path) -> PluginResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Parse manifest from JSON string.
    pub fn from_json(json: &str) -> PluginResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| PluginError::InvalidManifest(e.to_string()))
    }

    /// Convert manifest to JSON string.
    pub fn to_json(&self) -> PluginResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(PluginError::Json)
    }

    /// Set the version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Add a dependency.
    pub fn with_dependency(mut self, dep: PluginDependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Add a permission.
    pub fn with_permission(mut self, perm: PluginPermission) -> Self {
        self.permissions.push(perm);
        self
    }

    /// Check if a permission is granted.
    pub fn has_permission(&self, name: &str) -> bool {
        self.permissions.iter().any(|p| p.name == name)
    }

    /// Check if a dependency is present.
    pub fn has_dependency(&self, id: &PluginId) -> bool {
        self.dependencies.iter().any(|d| &d.plugin_id == id)
    }

    /// Check if this platform is supported.
    pub fn supports_current_platform(&self) -> bool {
        if self.platforms.is_empty() {
            return true; // All platforms supported if none specified
        }
        self.platforms.iter().any(|p| p.is_current())
    }

    /// Validate the manifest.
    pub fn validate(&self) -> PluginResult<()> {
        // Validate version format
        if semver::Version::parse(&self.version).is_err() {
            return Err(PluginError::InvalidManifest(
                format!("Invalid version format: {}", self.version)
            ));
        }

        // Validate min_host_version
        if semver::Version::parse(&self.min_host_version).is_err() {
            return Err(PluginError::InvalidManifest(
                format!("Invalid min_host_version format: {}", self.min_host_version)
            ));
        }

        // Validate dependencies
        for dep in &self.dependencies {
            if semver::VersionReq::parse(&dep.version).is_err() {
                return Err(PluginError::InvalidManifest(
                    format!("Invalid dependency version format: {}", dep.version)
                ));
            }
        }

        Ok(())
    }
}

impl Default for PluginManifest {
    fn default() -> Self {
        Self::new("com.example.plugin", "Example Plugin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_creation() {
        let manifest = PluginManifest::new("com.test.plugin", "Test Plugin")
            .with_version("1.0.0")
            .with_author("Test Author");

        assert_eq!(manifest.id.as_str(), "com.test.plugin");
        assert_eq!(manifest.name, "Test Plugin");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[test]
    fn test_manifest_validation() {
        let manifest = PluginManifest::new("com.test.plugin", "Test Plugin");
        assert!(manifest.validate().is_ok());

        let invalid = PluginManifest::new("com.test.plugin", "Test Plugin")
            .with_version("not-a-version");
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_dependency() {
        let dep = PluginDependency::required("com.test.dep", "1.0.0");
        assert!(!dep.optional);

        let opt = PluginDependency::optional("com.test.opt", "2.0.0");
        assert!(opt.optional);
    }

    #[test]
    fn test_permission() {
        let perm = PluginPermission::new("filesystem")
            .with_reason("Access files")
            .optional();

        assert_eq!(perm.name, "filesystem");
        assert!(perm.reason.is_some());
        assert!(!perm.required);
    }

    #[test]
    fn test_platform() {
        let current = Platform::current();
        assert!(current.is_current());
    }

    #[test]
    fn test_json_roundtrip() {
        let manifest = PluginManifest::new("com.test.plugin", "Test Plugin")
            .with_version("1.0.0")
            .with_description("A test plugin");

        let json = manifest.to_json().unwrap();
        let parsed = PluginManifest::from_json(&json).unwrap();

        assert_eq!(parsed.id, manifest.id);
        assert_eq!(parsed.name, manifest.name);
        assert_eq!(parsed.version, manifest.version);
    }
}