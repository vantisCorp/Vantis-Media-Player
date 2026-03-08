//! Advanced conflict resolution strategies
//!
//! Provides intelligent conflict resolution for sync conflicts with:
//! - Automatic resolution strategies
//! - Merge capabilities
//! - User-defined rules
//! - Conflict history tracking

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use async_trait::async_trait;

use super::sync::{
    SyncConflict, ConflictResolution, ConflictVersion, SyncDataType,
};

/// Conflict resolver trait
#[async_trait]
pub trait ConflictResolver: Send + Sync {
    /// Resolve a conflict
    async fn resolve(&self, conflict: &SyncConflict) -> Result<ConflictSolution>;
    
    /// Get resolution preview
    async fn preview(&self, conflict: &SyncConflict) -> Result<ConflictPreview>;
    
    /// Check if can auto-resolve
    fn can_auto_resolve(&self, conflict: &SyncConflict) -> bool;
}

/// Solution to a conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictSolution {
    /// Conflict ID
    pub conflict_id: Uuid,
    /// Resolution strategy used
    pub resolution: ConflictResolution,
    /// Merged data (if applicable)
    pub merged_data: Option<String>,
    /// Resolution timestamp
    pub resolved_at: DateTime<Utc>,
    /// Resolution source
    pub resolved_by: ResolutionSource,
}

/// Source of resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionSource {
    /// Automatic resolution
    Automatic,
    /// User decision
    User,
    /// Rule-based resolution
    RuleBased { rule_id: String },
    /// Default strategy
    Default,
}

/// Preview of conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPreview {
    /// Conflict ID
    pub conflict_id: Uuid,
    /// Preview of local version
    pub local_preview: String,
    /// Preview of cloud version
    pub cloud_preview: String,
    /// Preview of merged version (if possible)
    pub merged_preview: Option<String>,
    /// Recommended resolution
    pub recommended: ConflictResolution,
    /// Reason for recommendation
    pub recommendation_reason: String,
}

/// Conflict resolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule priority (higher = more important)
    pub priority: u32,
    /// Data type filter (None = all types)
    pub data_type: Option<SyncDataType>,
    /// Condition for rule application
    pub condition: RuleCondition,
    /// Resolution strategy
    pub resolution: ConflictResolution,
    /// Enabled
    pub enabled: bool,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Always apply
    Always,
    /// Field matches pattern
    FieldMatches { field: String, pattern: String },
    /// Age-based condition
    NewerThan { seconds: i64 },
    /// Device-based condition
    FromDevice { device_id: String },
    /// Size-based condition
    LargerThan { bytes: u64 },
    /// Custom expression
    Custom { expression: String },
}

/// Default conflict resolver implementation
pub struct DefaultConflictResolver {
    /// Default resolution strategy
    default_strategy: ConflictResolution,
    /// Resolution rules
    rules: Vec<ConflictRule>,
    /// Conflict history
    history: VecDeque<ConflictSolution>,
    /// Maximum history size
    max_history: usize,
}

impl DefaultConflictResolver {
    /// Create a new resolver
    pub fn new(default_strategy: ConflictResolution) -> Self {
        Self {
            default_strategy,
            rules: Vec::new(),
            history: VecDeque::new(),
            max_history: 1000,
        }
    }

    /// Add a resolution rule
    pub fn add_rule(&mut self, rule: ConflictRule) {
        self.rules.push(rule);
        // Sort by priority (highest first)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Remove a rule
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.retain(|r| r.id != rule_id);
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[ConflictRule] {
        &self.rules
    }

    /// Get conflict history
    pub fn get_history(&self) -> &VecDeque<ConflictSolution> {
        &self.history
    }

    /// Find applicable rule
    fn find_rule(&self, conflict: &SyncConflict) -> Option<&ConflictRule> {
        self.rules
            .iter()
            .filter(|r| r.enabled)
            .find(|r| self.rule_matches(r, conflict))
    }

    /// Check if rule matches conflict
    fn rule_matches(&self, rule: &ConflictRule, conflict: &SyncConflict) -> bool {
        // Check data type filter
        if let Some(ref data_type) = rule.data_type {
            if conflict.data_type != *data_type {
                return false;
            }
        }

        // Check condition
        match &rule.condition {
            RuleCondition::Always => true,
            RuleCondition::NewerThan { seconds } => {
                let now = Utc::now();
                let threshold = now - chrono::Duration::seconds(*seconds);
                conflict.local_version.timestamp > threshold
                    || conflict.cloud_version.timestamp > threshold
            }
            RuleCondition::FromDevice { device_id } => {
                conflict.local_version.device_id.to_string() == *device_id
                    || conflict.cloud_version.device_id.to_string() == *device_id
            }
            RuleCondition::FieldMatches { field, pattern } => {
                // Simple field matching
                conflict.item_id.contains(pattern) || pattern.contains(field)
            }
            RuleCondition::LargerThan { bytes } => {
                conflict.local_version.preview.len() as u64 > *bytes
                    || conflict.cloud_version.preview.len() as u64 > *bytes
            }
            RuleCondition::Custom { expression: _ } => {
                // Custom expressions would be evaluated here
                false
            }
        }
    }

    /// Determine best resolution strategy
    fn determine_strategy(&self, conflict: &SyncConflict) -> ConflictResolution {
        // Check rules first
        if let Some(rule) = self.find_rule(conflict) {
            return rule.resolution.clone();
        }

        // Smart defaults based on data type
        match conflict.data_type {
            SyncDataType::Settings => ConflictResolution::Merge,
            SyncDataType::Playlists => ConflictResolution::KeepBoth,
            SyncDataType::WatchHistory => ConflictResolution::KeepCloud,
            SyncDataType::Bookmarks => ConflictResolution::Merge,
            SyncDataType::Subtitles => ConflictResolution::KeepLocal,
            SyncDataType::Themes => ConflictResolution::KeepLocal,
            SyncDataType::Plugins => ConflictResolution::Merge,
            SyncDataType::Profiles => ConflictResolution::KeepCloud,
        }
    }

    /// Record resolution in history
    fn record_resolution(&mut self, solution: ConflictSolution) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(solution);
    }
}

#[async_trait]
impl ConflictResolver for DefaultConflictResolver {
    async fn resolve(&self, conflict: &SyncConflict) -> Result<ConflictSolution> {
        let resolution = self.determine_strategy(conflict);
        
        let merged_data = match &resolution {
            ConflictResolution::Merge => {
                self.merge_versions(&conflict.local_version, &conflict.cloud_version, &conflict.data_type).await?
            }
            _ => None,
        };

        Ok(ConflictSolution {
            conflict_id: conflict.id,
            resolution,
            merged_data,
            resolved_at: Utc::now(),
            resolved_by: ResolutionSource::Default,
        })
    }

    async fn preview(&self, conflict: &SyncConflict) -> Result<ConflictPreview> {
        let recommended = self.determine_strategy(conflict);
        
        let merged_preview = if matches!(recommended, ConflictResolution::Merge) {
            self.merge_versions(&conflict.local_version, &conflict.cloud_version, &conflict.data_type).await?
        } else {
            None
        };

        let recommendation_reason = match &recommended {
            ConflictResolution::KeepLocal => "Local version is newer or more complete".to_string(),
            ConflictResolution::KeepCloud => "Cloud version is the source of truth".to_string(),
            ConflictResolution::Merge => "Both versions have unique changes".to_string(),
            ConflictResolution::KeepBoth => "Items can coexist independently".to_string(),
            ConflictResolution::Manual => "Requires user decision due to complexity".to_string(),
        };

        Ok(ConflictPreview {
            conflict_id: conflict.id,
            local_preview: conflict.local_version.preview.clone(),
            cloud_preview: conflict.cloud_version.preview.clone(),
            merged_preview,
            recommended,
            recommendation_reason,
        })
    }

    fn can_auto_resolve(&self, conflict: &SyncConflict) -> bool {
        // Check if any rule applies
        if self.find_rule(conflict).is_some() {
            return true;
        }

        // Check data type for auto-resolvability
        matches!(
            conflict.data_type,
            SyncDataType::Settings
                | SyncDataType::WatchHistory
                | SyncDataType::Bookmarks
        )
    }
}

impl DefaultConflictResolver {
    /// Merge two versions
    async fn merge_versions(
        &self,
        local: &ConflictVersion,
        cloud: &ConflictVersion,
        data_type: &SyncDataType,
    ) -> Result<Option<String>> {
        // Parse as JSON if possible
        let local_json: Option<serde_json::Value> = serde_json::from_str(&local.preview).ok();
        let cloud_json: Option<serde_json::Value> = serde_json::from_str(&cloud.preview).ok();

        match (local_json, cloud_json) {
            (Some(local_val), Some(cloud_val)) => {
                // Merge JSON objects
                let merged = self.merge_json(&local_val, &cloud_val, data_type);
                Ok(Some(serde_json::to_string(&merged)?))
            }
            _ => {
                // Cannot merge non-JSON data
                Ok(None)
            }
        }
    }

    /// Merge JSON values
    fn merge_json(
        &self,
        local: &serde_json::Value,
        cloud: &serde_json::Value,
        data_type: &SyncDataType,
    ) -> serde_json::Value {
        match (local, cloud) {
            (serde_json::Value::Object(local_obj), serde_json::Value::Object(cloud_obj)) => {
                let mut merged = serde_json::Map::new();
                
                // Merge all keys from both objects
                for (key, value) in local_obj {
                    if let Some(cloud_value) = cloud_obj.get(key) {
                        // Recursively merge nested objects
                        merged.insert(
                            key.clone(),
                            self.merge_json(value, cloud_value, data_type),
                        );
                    } else {
                        merged.insert(key.clone(), value.clone());
                    }
                }
                
                // Add keys only in cloud
                for (key, value) in cloud_obj {
                    if !merged.contains_key(key) {
                        merged.insert(key.clone(), value.clone());
                    }
                }
                
                serde_json::Value::Object(merged)
            }
            (serde_json::Value::Array(local_arr), serde_json::Value::Array(cloud_arr)) => {
                // Merge arrays based on data type
                match data_type {
                    SyncDataType::Playlists | SyncDataType::Bookmarks => {
                        // Concatenate and deduplicate
                        let mut merged = local_arr.clone();
                        for item in cloud_arr {
                            if !merged.contains(item) {
                                merged.push(item.clone());
                            }
                        }
                        serde_json::Value::Array(merged)
                    }
                    _ => {
                        // Use cloud array (newer wins)
                        serde_json::Value::Array(cloud_arr.clone())
                    }
                }
            }
            // For primitive values, prefer newer (cloud) for most types
            _ => cloud.clone(),
        }
    }
}

impl Default for DefaultConflictResolver {
    fn default() -> Self {
        Self::new(ConflictResolution::KeepCloud)
    }
}

/// Three-way merge resolver for complex conflicts
pub struct ThreeWayMergeResolver {
    /// Base version for comparison
    base_versions: HashMap<String, String>,
}

impl ThreeWayMergeResolver {
    /// Create a new three-way merge resolver
    pub fn new() -> Self {
        Self {
            base_versions: HashMap::new(),
        }
    }

    /// Set base version for an item
    pub fn set_base(&mut self, item_id: String, base_data: String) {
        self.base_versions.insert(item_id, base_data);
    }

    /// Perform three-way merge
    pub fn merge(
        &self,
        item_id: &str,
        local: &str,
        cloud: &str,
    ) -> Result<MergeResult> {
        let base = self.base_versions.get(item_id);

        match base {
            Some(base_data) => {
                // True three-way merge
                let local_diff = self.diff(base_data, local);
                let cloud_diff = self.diff(base_data, cloud);

                // Check for conflicts in diffs
                if self.conflicts_exist(&local_diff, &cloud_diff) {
                    Ok(MergeResult::Conflict {
                        local_changes: local_diff,
                        cloud_changes: cloud_diff,
                    })
                } else {
                    // Apply both sets of changes
                    let merged = self.apply_changes(base_data, &local_diff, &cloud_diff);
                    Ok(MergeResult::Merged { data: merged })
                }
            }
            None => {
                // No base, cannot do three-way merge
                Ok(MergeResult::NoBase)
            }
        }
    }

    /// Calculate diff between two strings
    fn diff(&self, old: &str, new: &str) -> Vec<DiffEntry> {
        // Simplified diff - in production would use proper diff algorithm
        if old == new {
            return Vec::new();
        }

        vec![DiffEntry {
            operation: DiffOp::Replace,
            position: 0,
            old_value: old.to_string(),
            new_value: new.to_string(),
        }]
    }

    /// Check if two diffs conflict
    fn conflicts_exist(&self, local: &[DiffEntry], cloud: &[DiffEntry]) -> bool {
        for local_entry in local {
            for cloud_entry in cloud {
                if local_entry.position == cloud_entry.position {
                    return true;
                }
            }
        }
        false
    }

    /// Apply changes to base
    fn apply_changes(&self, base: &str, local: &[DiffEntry], cloud: &[DiffEntry]) -> String {
        // Simplified - just apply cloud changes
        for entry in cloud {
            if entry.operation == DiffOp::Replace {
                return entry.new_value.clone();
            }
        }
        base.to_string()
    }
}

impl Default for ThreeWayMergeResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a merge operation
#[derive(Debug, Clone)]
pub enum MergeResult {
    /// Successfully merged
    Merged { data: String },
    /// Conflict detected
    Conflict {
        local_changes: Vec<DiffEntry>,
        cloud_changes: Vec<DiffEntry>,
    },
    /// No base version available
    NoBase,
}

/// Diff entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEntry {
    /// Operation
    pub operation: DiffOp,
    /// Position in document
    pub position: usize,
    /// Old value
    pub old_value: String,
    /// New value
    pub new_value: String,
}

/// Diff operation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiffOp {
    Insert,
    Delete,
    Replace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_resolver() {
        let resolver = DefaultConflictResolver::new(ConflictResolution::KeepCloud);
        
        let conflict = SyncConflict {
            id: Uuid::new_v4(),
            data_type: SyncDataType::Settings,
            item_id: "settings.json".to_string(),
            local_version: ConflictVersion {
                timestamp: Utc::now(),
                hash: "abc".to_string(),
                preview: r#"{"volume": 80}"#.to_string(),
                device_id: Uuid::new_v4(),
            },
            cloud_version: ConflictVersion {
                timestamp: Utc::now(),
                hash: "def".to_string(),
                preview: r#"{"volume": 70, "brightness": 50}"#.to_string(),
                device_id: Uuid::new_v4(),
            },
            detected_at: Utc::now(),
            resolution: ConflictResolution::Merge,
        };

        assert!(resolver.can_auto_resolve(&conflict));
        
        let solution = resolver.resolve(&conflict).await.unwrap();
        assert_eq!(solution.resolution, ConflictResolution::Merge);
    }

    #[test]
    fn test_conflict_rule() {
        let rule = ConflictRule {
            id: "rule-1".to_string(),
            name: "Always keep cloud for watch history".to_string(),
            priority: 100,
            data_type: Some(SyncDataType::WatchHistory),
            condition: RuleCondition::Always,
            resolution: ConflictResolution::KeepCloud,
            enabled: true,
        };

        assert!(rule.enabled);
        assert_eq!(rule.priority, 100);
    }

    #[test]
    fn test_three_way_merge() {
        let resolver = ThreeWayMergeResolver::new();
        
        let result = resolver.merge("test", "local", "cloud").unwrap();
        assert!(matches!(result, MergeResult::NoBase));
    }
}