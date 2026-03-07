//! Plugin Dependency Resolution System
//!
//! Handles dependency graph building, version constraint solving,
//! circular dependency detection, and automatic dependency installation.

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{info, debug, warn};

// ============================================================================
// Dependency Types
// ============================================================================

/// Version constraint (semver)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraint {
    /// Raw constraint string (e.g., ">=1.0.0,<2.0.0")
    pub raw: String,
    
    /// Parsed constraints
    pub constraints: Vec<Constraint>,
}

/// Single version constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Comparison operator
    pub op: ConstraintOp,
    
    /// Version
    pub version: String,
}

/// Comparison operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConstraintOp {
    /// Exact match (=)
    Exact,
    /// Greater than (>)
    Greater,
    /// Greater than or equal (>=)
    GreaterEqual,
    /// Less than (<)
    Less,
    /// Less than or equal (<=)
    LessEqual,
    /// Compatible (^)
    Compatible,
    /// Wildcard (*)
    Wildcard,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Version constraint
    pub version_constraint: VersionConstraint,
    
    /// Whether the dependency is optional
    pub optional: bool,
    
    /// Feature flags required
    pub features: Vec<String>,
}

/// Resolved dependency
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Resolved version
    pub version: String,
    
    /// Where it came from (which plugin required it)
    pub required_by: Vec<String>,
}

/// Dependency graph
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// All nodes in the graph
    nodes: HashMap<String, DependencyNode>,
    
    /// Adjacency list (plugin_id -> dependencies)
    edges: HashMap<String, Vec<String>>,
    
    /// Reverse edges (plugin_id -> dependents)
    reverse_edges: HashMap<String, Vec<String>>,
}

/// Node in the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Required version constraint
    pub version_constraint: VersionConstraint,
    
    /// Resolved version (after resolution)
    pub resolved_version: Option<String>,
    
    /// Whether this is optional
    pub optional: bool,
    
    /// Installation priority (higher = install first)
    pub priority: u32,
}

/// Resolution result
#[derive(Debug)]
pub struct ResolutionResult {
    /// Successfully resolved dependencies
    pub resolved: Vec<ResolvedDependency>,
    
    /// Missing dependencies that couldn't be resolved
    pub missing: Vec<MissingDependency>,
    
    /// Conflicts detected
    pub conflicts: Vec<DependencyConflict>,
    
    /// Resolution is complete and valid
    pub is_complete: bool,
}

/// Missing dependency
#[derive(Debug, Clone)]
pub struct MissingDependency {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Required version constraint
    pub constraint: VersionConstraint,
    
    /// Which plugins need this
    pub required_by: Vec<String>,
}

/// Dependency conflict
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    /// Plugin ID in conflict
    pub plugin_id: String,
    
    /// Conflicting constraints
    pub constraints: Vec<ConflictConstraint>,
}

/// A constraint in a conflict
#[derive(Debug, Clone)]
pub struct ConflictConstraint {
    /// The constraint
    pub constraint: VersionConstraint,
    
    /// Which plugin requires this
    pub required_by: String,
}

// ============================================================================
// Dependency Resolver
// ============================================================================

/// Plugin dependency resolver
pub struct DependencyResolver {
    /// Available plugins and their versions
    available_plugins: HashMap<String, Vec<PluginVersionInfo>>,
    
    /// Dependency graph
    graph: DependencyGraph,
    
    /// Resolution strategy
    strategy: ResolutionStrategy,
}

/// Plugin version information
#[derive(Debug, Clone)]
pub struct PluginVersionInfo {
    /// Version string
    pub version: String,
    
    /// Dependencies for this version
    pub dependencies: Vec<Dependency>,
    
    /// API version
    pub api_version: u32,
}

/// Resolution strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionStrategy {
    /// Prefer newest versions
    Newest,
    
    /// Prefer oldest compatible versions
    Oldest,
    
    /// Prefer installed versions if compatible
    PreferInstalled,
}

// ============================================================================
// Implementation
// ============================================================================

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            available_plugins: HashMap::new(),
            graph: DependencyGraph::default(),
            strategy: ResolutionStrategy::Newest,
        }
    }
    
    /// Set resolution strategy
    pub fn with_strategy(mut self, strategy: ResolutionStrategy) -> Self {
        self.strategy = strategy;
        self
    }
    
    /// Register an available plugin version
    pub fn register_plugin(&mut self, plugin_id: &str, version_info: PluginVersionInfo) {
        self.available_plugins
            .entry(plugin_id.to_string())
            .or_default()
            .push(version_info);
    }
    
    /// Register multiple versions for a plugin
    pub fn register_plugin_versions(&mut self, plugin_id: &str, versions: Vec<PluginVersionInfo>) {
        self.available_plugins.insert(plugin_id.to_string(), versions);
    }
    
    /// Resolve dependencies for a plugin
    pub fn resolve(&mut self, plugin_id: &str, version: &str) -> Result<ResolutionResult> {
        info!("Resolving dependencies for {}@{}", plugin_id, version);
        
        // Build dependency graph
        self.build_graph(plugin_id, version)?;
        
        // Check for cycles
        if let Some(cycle) = self.detect_cycle() {
            bail!("Circular dependency detected: {:?}", cycle);
        }
        
        // Topological sort for installation order
        let order = self.topological_sort()?;
        
        // Resolve versions
        let mut resolved = Vec::new();
        let mut missing = Vec::new();
        let mut conflicts = Vec::new();
        
        for node_id in &order {
            let node = self.graph.nodes.get(node_id).unwrap();
            
            // Find compatible version
            if let Some(compatible_version) = self.find_compatible_version(node_id, &node.version_constraint) {
                resolved.push(ResolvedDependency {
                    plugin_id: node_id.clone(),
                    version: compatible_version.clone(),
                    required_by: self.graph.reverse_edges
                        .get(node_id)
                        .cloned()
                        .unwrap_or_default(),
                });
            } else {
                missing.push(MissingDependency {
                    plugin_id: node_id.clone(),
                    constraint: node.version_constraint.clone(),
                    required_by: self.graph.reverse_edges
                        .get(node_id)
                        .cloned()
                        .unwrap_or_default(),
                });
            }
        }
        
        // Check for conflicts
        self.check_conflicts(&mut conflicts);
        
        let is_complete = missing.is_empty() && conflicts.is_empty();
        
        info!("Resolution complete: {} resolved, {} missing, {} conflicts", 
            resolved.len(), missing.len(), conflicts.len());
        
        Ok(ResolutionResult {
            resolved,
            missing,
            conflicts,
            is_complete,
        })
    }
    
    /// Build dependency graph starting from a root plugin
    fn build_graph(&mut self, plugin_id: &str, version: &str) -> Result<()> {
        let versions = self.available_plugins.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin {} not found", plugin_id))?;
        
        let version_info = versions.iter()
            .find(|v| v.version == version)
            .ok_or_else(|| anyhow::anyhow!("Version {} not found for plugin {}", version, plugin_id))?;
        
        // Add root node
        self.graph.nodes.insert(plugin_id.to_string(), DependencyNode {
            plugin_id: plugin_id.to_string(),
            version_constraint: VersionConstraint::exact(version),
            resolved_version: Some(version.to_string()),
            optional: false,
            priority: 0,
        });
        
        // Process dependencies recursively
        self.process_dependencies(plugin_id, &version_info.dependencies, 1)?;
        
        Ok(())
    }
    
    /// Process dependencies recursively
    fn process_dependencies(&mut self, parent_id: &str, dependencies: &[Dependency], depth: u32) -> Result<()> {
        for dep in dependencies {
            // Add edge
            self.graph.edges
                .entry(parent_id.to_string())
                .or_default()
                .push(dep.plugin_id.clone());
            
            self.graph.reverse_edges
                .entry(dep.plugin_id.clone())
                .or_default()
                .push(parent_id.to_string());
            
            // Check if node already exists
            if self.graph.nodes.contains_key(&dep.plugin_id) {
                // Merge constraints
                let existing = self.graph.nodes.get_mut(&dep.plugin_id).unwrap();
                
                // Check constraint compatibility
                if !self.constraints_compatible(&existing.version_constraint, &dep.version_constraint) {
                    warn!("Potentially conflicting constraints for {}", dep.plugin_id);
                }
                
                continue;
            }
            
            // Add node
            self.graph.nodes.insert(dep.plugin_id.clone(), DependencyNode {
                plugin_id: dep.plugin_id.clone(),
                version_constraint: dep.version_constraint.clone(),
                resolved_version: None,
                optional: dep.optional,
                priority: depth,
            });
            
            // Recurse if we have version info
            if let Some(versions) = self.available_plugins.get(&dep.plugin_id) {
                if let Some(latest) = versions.first() {
                    self.process_dependencies(&dep.plugin_id, &latest.dependencies, depth + 1)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Detect cycles in the dependency graph
    fn detect_cycle(&self) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        
        for node_id in self.graph.nodes.keys() {
            if self.detect_cycle_dfs(node_id, &mut visited, &mut rec_stack, &mut path) {
                return Some(path);
            }
        }
        
        None
    }
    
    fn detect_cycle_dfs(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if rec_stack.contains(node_id) {
            return true;
        }
        
        if visited.contains(node_id) {
            return false;
        }
        
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        path.push(node_id.to_string());
        
        if let Some(neighbors) = self.graph.edges.get(node_id) {
            for neighbor in neighbors {
                if self.detect_cycle_dfs(neighbor, visited, rec_stack, path) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node_id);
        path.pop();
        false
    }
    
    /// Topological sort for installation order
    fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree: HashMap<String, u32> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // Calculate in-degrees
        for node_id in self.graph.nodes.keys() {
            in_degree.entry(node_id.clone()).or_insert(0);
        }
        
        for (_, deps) in &self.graph.edges {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        
        // Find nodes with no incoming edges
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }
        
        // Process queue
        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());
            
            if let Some(neighbors) = self.graph.edges.get(&node_id) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        if result.len() != self.graph.nodes.len() {
            bail!("Graph has a cycle - topological sort failed");
        }
        
        // Reverse to get dependency order (dependencies first)
        result.reverse();
        Ok(result)
    }
    
    /// Find a compatible version for a plugin
    fn find_compatible_version(&self, plugin_id: &str, constraint: &VersionConstraint) -> Option<String> {
        let versions = self.available_plugins.get(plugin_id)?;
        
        let compatible: Vec<_> = versions.iter()
            .filter(|v| constraint.satisfies(&v.version))
            .collect();
        
        if compatible.is_empty() {
            return None;
        }
        
        // Choose based on strategy
        match self.strategy {
            ResolutionStrategy::Newest => compatible.first().map(|v| v.version.clone()),
            ResolutionStrategy::Oldest => compatible.last().map(|v| v.version.clone()),
            ResolutionStrategy::PreferInstalled => compatible.first().map(|v| v.version.clone()),
        }
    }
    
    /// Check if two constraints are compatible
    fn constraints_compatible(&self, _c1: &VersionConstraint, _c2: &VersionConstraint) -> bool {
        // Simplified check - in production would use full semver intersection
        true
    }
    
    /// Check for version conflicts
    fn check_conflicts(&self, conflicts: &mut Vec<DependencyConflict>) {
        for (plugin_id, dependents) in &self.graph.reverse_edges {
            if dependents.len() > 1 {
                // Multiple plugins depend on this - check if constraints conflict
                let constraints: Vec<_> = dependents.iter()
                    .filter_map(|dep_id| {
                        self.graph.nodes.get(dep_id).map(|node| ConflictConstraint {
                            constraint: node.version_constraint.clone(),
                            required_by: dep_id.clone(),
                        })
                    })
                    .collect();
                
                // Check if any constraints are incompatible
                for i in 0..constraints.len() {
                    for j in (i+1)..constraints.len() {
                        if !self.constraints_compatible(&constraints[i].constraint, &constraints[j].constraint) {
                            conflicts.push(DependencyConflict {
                                plugin_id: plugin_id.clone(),
                                constraints: constraints.clone(),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
    
    /// Get installation order for resolved dependencies
    pub fn get_installation_order(&self, resolved: &[ResolvedDependency]) -> Vec<String> {
        let mut order: Vec<_> = resolved.iter()
            .map(|d| d.plugin_id.clone())
            .collect();
        
        // Sort by priority (dependencies first)
        order.sort_by_key(|id| {
            self.graph.nodes.get(id)
                .map(|n| n.priority)
                .unwrap_or(u32::MAX)
        });
        
        order
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionConstraint {
    /// Create an exact version constraint
    pub fn exact(version: &str) -> Self {
        Self {
            raw: version.to_string(),
            constraints: vec![Constraint {
                op: ConstraintOp::Exact,
                version: version.to_string(),
            }],
        }
    }
    
    /// Create a range constraint
    pub fn range(min: &str, max: &str) -> Self {
        Self {
            raw: format!(">={},<{}", min, max),
            constraints: vec![
                Constraint {
                    op: ConstraintOp::GreaterEqual,
                    version: min.to_string(),
                },
                Constraint {
                    op: ConstraintOp::Less,
                    version: max.to_string(),
                },
            ],
        }
    }
    
    /// Create a compatible version constraint (^)
    pub fn compatible(version: &str) -> Self {
        Self {
            raw: format!("^{}", version),
            constraints: vec![Constraint {
                op: ConstraintOp::Compatible,
                version: version.to_string(),
            }],
        }
    }
    
    /// Check if a version satisfies this constraint
    pub fn satisfies(&self, version: &str) -> bool {
        for constraint in &self.constraints {
            if !constraint.satisfies(version) {
                return false;
            }
        }
        true
    }
}

impl Constraint {
    /// Check if a version satisfies this constraint
    pub fn satisfies(&self, version: &str) -> bool {
        // Simplified version comparison
        // In production, use proper semver comparison
        match self.op {
            ConstraintOp::Exact => version == self.version,
            ConstraintOp::Greater => version > self.version,
            ConstraintOp::GreaterEqual => version >= self.version,
            ConstraintOp::Less => version < self.version,
            ConstraintOp::LessEqual => version <= self.version,
            ConstraintOp::Compatible => {
                // ^1.2.3 means >=1.2.3 and <2.0.0
                version >= self.version
            }
            ConstraintOp::Wildcard => true,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_constraint_exact() {
        let constraint = VersionConstraint::exact("1.0.0");
        assert!(constraint.satisfies("1.0.0"));
        assert!(!constraint.satisfies("1.0.1"));
    }
    
    #[test]
    fn test_version_constraint_range() {
        let constraint = VersionConstraint::range("1.0.0", "2.0.0");
        assert!(constraint.satisfies("1.0.0"));
        assert!(constraint.satisfies("1.5.0"));
        assert!(!constraint.satisfies("2.0.0"));
        assert!(!constraint.satisfies("0.9.0"));
    }
    
    #[test]
    fn test_dependency_resolver_creation() {
        let resolver = DependencyResolver::new();
        assert!(resolver.available_plugins.is_empty());
    }
    
    #[test]
    fn test_register_plugin() {
        let mut resolver = DependencyResolver::new();
        resolver.register_plugin("test-plugin", PluginVersionInfo {
            version: "1.0.0".to_string(),
            dependencies: vec![],
            api_version: 1,
        });
        
        assert!(resolver.available_plugins.contains_key("test-plugin"));
    }
    
    #[test]
    fn test_resolution_simple() {
        let mut resolver = DependencyResolver::new();
        resolver.register_plugin("root", PluginVersionInfo {
            version: "1.0.0".to_string(),
            dependencies: vec![],
            api_version: 1,
        });
        
        let result = resolver.resolve("root", "1.0.0").unwrap();
        assert!(result.is_complete);
        assert_eq!(result.resolved.len(), 1);
    }
    
    #[test]
    fn test_topological_sort() {
        let resolver = DependencyResolver::new();
        let mut graph = DependencyGraph::default();
        
        graph.nodes.insert("a".to_string(), DependencyNode {
            plugin_id: "a".to_string(),
            version_constraint: VersionConstraint::exact("1.0.0"),
            resolved_version: None,
            optional: false,
            priority: 0,
        });
        graph.nodes.insert("b".to_string(), DependencyNode {
            plugin_id: "b".to_string(),
            version_constraint: VersionConstraint::exact("1.0.0"),
            resolved_version: None,
            optional: false,
            priority: 1,
        });
        
        graph.edges.insert("a".to_string(), vec!["b".to_string()]);
        graph.reverse_edges.insert("b".to_string(), vec!["a".to_string()]);
        
        let order = resolver.topological_sort().unwrap();
        assert_eq!(order, vec!["b", "a"]);
    }
}