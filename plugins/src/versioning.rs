//! Plugin Versioning and Compatibility System
//!
//! Handles API version checking, compatibility matrices,
//! and breaking change detection.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};

// ============================================================================
// Version Types
// ============================================================================

/// Semantic version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    /// Major version
    pub major: u32,
    
    /// Minor version
    pub minor: u32,
    
    /// Patch version
    pub patch: u32,
    
    /// Pre-release identifier
    pub pre_release: Option<String>,
    
    /// Build metadata
    pub build: Option<String>,
}

/// API version (simplified integer)
pub type ApiVersion = u32;

/// Compatibility level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompatibilityLevel {
    /// Fully compatible - no issues expected
    Full,
    
    /// Partially compatible - some features may not work
    Partial,
    
    /// Deprecated - will be removed in future version
    Deprecated,
    
    /// Breaking changes - migration required
    Breaking,
    
    /// Incompatible - cannot be used together
    Incompatible,
}

/// Compatibility info for a plugin version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Plugin version
    pub version: SemanticVersion,
    
    /// Minimum API version required
    pub min_api_version: ApiVersion,
    
    /// Maximum API version supported
    pub max_api_version: ApiVersion,
    
    /// Compatibility matrix with other plugins
    pub plugin_compatibility: HashMap<String, VersionRange>,
    
    /// Breaking changes from previous versions
    pub breaking_changes: Vec<BreakingChange>,
    
    /// Deprecation warnings
    pub deprecations: Vec<Deprecation>,
    
    /// Migration notes
    pub migration_notes: Option<String>,
}

/// Version range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRange {
    /// Minimum version (inclusive)
    pub min: SemanticVersion,
    
    /// Maximum version (exclusive)
    pub max: Option<SemanticVersion>,
}

/// Breaking change description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    /// Change ID
    pub id: String,
    
    /// Change description
    pub description: String,
    
    /// Affected functionality
    pub affected_areas: Vec<String>,
    
    /// Migration guide URL
    pub migration_guide: Option<String>,
    
    /// Severity level
    pub severity: BreakingChangeSeverity,
}

/// Breaking change severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreakingChangeSeverity {
    /// Low impact - easy to fix
    Low,
    
    /// Medium impact - requires code changes
    Medium,
    
    /// High impact - significant refactoring needed
    High,
    
    /// Critical - complete rewrite may be needed
    Critical,
}

/// Deprecation notice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deprecation {
    /// Deprecated feature
    pub feature: String,
    
    /// Version when deprecated
    pub deprecated_in: SemanticVersion,
    
    /// Version when it will be removed
    pub removal_version: SemanticVersion,
    
    /// Replacement feature
    pub replacement: Option<String>,
    
    /// Additional message
    pub message: String,
}

// ============================================================================
// Compatibility Checker
// ============================================================================

/// Plugin compatibility checker
pub struct CompatibilityChecker {
    /// Current API version
    current_api_version: ApiVersion,
    
    /// Known plugin compatibility info
    compatibility_db: HashMap<String, Vec<CompatibilityInfo>>,
    
    /// Compatibility matrix between plugins
    plugin_matrix: HashMap<(String, String), CompatibilityLevel>,
}

// ============================================================================
// Implementation
// ============================================================================

impl SemanticVersion {
    /// Parse a version string
    pub fn parse(version: &str) -> Result<Self> {
        let version = version.trim_start_matches('v');
        
        let parts: Vec<&str> = version.split('.').collect();
        if parts.is_empty() {
            bail!("Empty version string");
        }
        
        let major = parts[0].parse()
            .map_err(|_| anyhow::anyhow!("Invalid major version"))?;
        
        let minor = if parts.len() > 1 {
            parts[1].parse()
                .map_err(|_| anyhow::anyhow!("Invalid minor version"))?
        } else {
            0
        };
        
        let patch = if parts.len() > 2 {
            let patch_str = parts[2].split('-').next().unwrap_or("0");
            patch_str.parse()
                .map_err(|_| anyhow::anyhow!("Invalid patch version"))?
        } else {
            0
        };
        
        let pre_release = if parts.len() > 2 && parts[2].contains('-') {
            Some(parts[2].split('-').nth(1).unwrap_or("").to_string())
        } else {
            None
        };
        
        let build = version.split('+').nth(1).map(|s| s.to_string());
        
        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
            build,
        })
    }
    
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }
    
    /// Convert to string
    pub fn to_string(&self) -> String {
        let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        
        if let Some(ref pre) = self.pre_release {
            s.push_str(&format!("-{}", pre));
        }
        
        if let Some(ref build) = self.build {
            s.push_str(&format!("+{}", build));
        }
        
        s
    }
    
    /// Check if this is a pre-release version
    pub fn is_prerelease(&self) -> bool {
        self.pre_release.is_some()
    }
    
    /// Check if this version is compatible with another (same major version)
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major
    }
    
    /// Get the next major version
    pub fn next_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }
    
    /// Get the next minor version
    pub fn next_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }
    
    /// Get the next patch version
    pub fn next_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl CompatibilityChecker {
    /// Create a new compatibility checker
    pub fn new(current_api_version: ApiVersion) -> Self {
        Self {
            current_api_version,
            compatibility_db: HashMap::new(),
            plugin_matrix: HashMap::new(),
        }
    }
    
    /// Register compatibility info for a plugin
    pub fn register(&mut self, info: CompatibilityInfo) {
        self.compatibility_db
            .entry(info.plugin_id.clone())
            .or_default()
            .push(info);
    }
    
    /// Set compatibility level between two plugins
    pub fn set_plugin_compatibility(
        &mut self,
        plugin_a: &str,
        plugin_b: &str,
        level: CompatibilityLevel,
    ) {
        self.plugin_matrix.insert((plugin_a.to_string(), plugin_b.to_string()), level);
        self.plugin_matrix.insert((plugin_b.to_string(), plugin_a.to_string()), level);
    }
    
    /// Check API version compatibility
    pub fn check_api_compatibility(&self, info: &CompatibilityInfo) -> CompatibilityLevel {
        if info.min_api_version > self.current_api_version {
            warn!("Plugin requires API version {} but current is {}", 
                info.min_api_version, self.current_api_version);
            return CompatibilityLevel::Incompatible;
        }
        
        if let Some(max) = info.max_api_version {
            if max < self.current_api_version {
                warn!("Plugin max API version {} is below current {}", 
                    max, self.current_api_version);
                return CompatibilityLevel::Breaking;
            }
        }
        
        if !info.breaking_changes.is_empty() {
            let has_critical = info.breaking_changes.iter()
                .any(|bc| bc.severity == BreakingChangeSeverity::Critical);
            
            if has_critical {
                return CompatibilityLevel::Breaking;
            }
            return CompatibilityLevel::Partial;
        }
        
        if !info.deprecations.is_empty() {
            return CompatibilityLevel::Deprecated;
        }
        
        CompatibilityLevel::Full
    }
    
    /// Check if a plugin version is compatible
    pub fn check_version_compatibility(
        &self,
        plugin_id: &str,
        version: &SemanticVersion,
    ) -> CompatibilityLevel {
        let versions = match self.compatibility_db.get(plugin_id) {
            Some(v) => v,
            None => {
                debug!("No compatibility info for plugin {}", plugin_id);
                return CompatibilityLevel::Full;
            }
        };
        
        // Find the best matching version info
        let best_match = versions.iter()
            .filter(|v| v.version <= *version)
            .max_by_key(|v| v.version.clone());
        
        match best_match {
            Some(info) => self.check_api_compatibility(info),
            None => {
                warn!("No compatible version found for {}@{}", plugin_id, version);
                CompatibilityLevel::Incompatible
            }
        }
    }
    
    /// Check compatibility between two plugins
    pub fn check_plugin_compatibility(
        &self,
        plugin_a: &str,
        plugin_b: &str,
    ) -> CompatibilityLevel {
        self.plugin_matrix
            .get(&(plugin_a.to_string(), plugin_b.to_string()))
            .copied()
            .unwrap_or(CompatibilityLevel::Full)
    }
    
    /// Get breaking changes for upgrading from one version to another
    pub fn get_breaking_changes(
        &self,
        plugin_id: &str,
        from_version: &SemanticVersion,
        to_version: &SemanticVersion,
    ) -> Vec<&BreakingChange> {
        let versions = match self.compatibility_db.get(plugin_id) {
            Some(v) => v,
            None => return Vec::new(),
        };
        
        versions.iter()
            .filter(|v| v.version > *from_version && v.version <= *to_version)
            .flat_map(|v| v.breaking_changes.iter())
            .collect()
    }
    
    /// Get deprecations for a version
    pub fn get_deprecations(
        &self,
        plugin_id: &str,
        version: &SemanticVersion,
    ) -> Vec<&Deprecation> {
        let versions = match self.compatibility_db.get(plugin_id) {
            Some(v) => v,
            None => return Vec::new(),
        };
        
        versions.iter()
            .filter(|v| v.version == *version)
            .flat_map(|v| v.deprecations.iter())
            .collect()
    }
    
    /// Check if migration is required
    pub fn requires_migration(
        &self,
        plugin_id: &str,
        from_version: &SemanticVersion,
        to_version: &SemanticVersion,
    ) -> bool {
        let breaking = self.get_breaking_changes(plugin_id, from_version, to_version);
        
        breaking.iter().any(|bc| {
            bc.severity == BreakingChangeSeverity::High 
                || bc.severity == BreakingChangeSeverity::Critical
        })
    }
    
    /// Get migration guide
    pub fn get_migration_guide(
        &self,
        plugin_id: &str,
        from_version: &SemanticVersion,
        to_version: &SemanticVersion,
    ) -> Option<String> {
        let versions = self.compatibility_db.get(plugin_id)?;
        
        versions.iter()
            .filter(|v| v.version > *from_version && v.version <= *to_version)
            .filter_map(|v| v.migration_notes.clone())
            .next()
    }
}

impl VersionRange {
    /// Create a new version range
    pub fn new(min: SemanticVersion, max: Option<SemanticVersion>) -> Self {
        Self { min, max }
    }
    
    /// Check if a version is within this range
    pub fn contains(&self, version: &SemanticVersion) -> bool {
        if version < &self.min {
            return false;
        }
        
        if let Some(ref max) = self.max {
            if version >= max {
                return false;
            }
        }
        
        true
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semantic_version_parse() {
        let v = SemanticVersion::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
    
    #[test]
    fn test_semantic_version_prerelease() {
        let v = SemanticVersion::parse("1.0.0-beta.1").unwrap();
        assert!(v.is_prerelease());
        assert_eq!(v.pre_release, Some("beta.1".to_string()));
    }
    
    #[test]
    fn test_semantic_version_comparison() {
        let v1 = SemanticVersion::parse("1.0.0").unwrap();
        let v2 = SemanticVersion::parse("2.0.0").unwrap();
        
        assert!(v1 < v2);
        assert!(v1.is_compatible_with(&v1));
        assert!(!v1.is_compatible_with(&v2));
    }
    
    #[test]
    fn test_version_range() {
        let range = VersionRange::new(
            SemanticVersion::parse("1.0.0").unwrap(),
            Some(SemanticVersion::parse("2.0.0").unwrap()),
        );
        
        assert!(range.contains(&SemanticVersion::parse("1.5.0").unwrap()));
        assert!(!range.contains(&SemanticVersion::parse("2.0.0").unwrap()));
        assert!(!range.contains(&SemanticVersion::parse("0.9.0").unwrap()));
    }
    
    #[test]
    fn test_compatibility_checker() {
        let checker = CompatibilityChecker::new(1);
        
        let info = CompatibilityInfo {
            plugin_id: "test-plugin".to_string(),
            version: SemanticVersion::parse("1.0.0").unwrap(),
            min_api_version: 1,
            max_api_version: 1,
            plugin_compatibility: HashMap::new(),
            breaking_changes: vec![],
            deprecations: vec![],
            migration_notes: None,
        };
        
        assert_eq!(checker.check_api_compatibility(&info), CompatibilityLevel::Full);
    }
    
    #[test]
    fn test_breaking_change_detection() {
        let mut checker = CompatibilityChecker::new(1);
        
        let info = CompatibilityInfo {
            plugin_id: "test-plugin".to_string(),
            version: SemanticVersion::parse("2.0.0").unwrap(),
            min_api_version: 1,
            max_api_version: 1,
            plugin_compatibility: HashMap::new(),
            breaking_changes: vec![BreakingChange {
                id: "BC001".to_string(),
                description: "API changed".to_string(),
                affected_areas: vec!["core".to_string()],
                migration_guide: None,
                severity: BreakingChangeSeverity::High,
            }],
            deprecations: vec![],
            migration_notes: None,
        };
        
        checker.register(info);
        
        let from = SemanticVersion::parse("1.0.0").unwrap();
        let to = SemanticVersion::parse("2.0.0").unwrap();
        
        assert!(checker.requires_migration("test-plugin", &from, &to));
    }
}