//! Dependency Management
//! 
//! Handles plugin dependencies including resolution, installation, and version management.

use anyhow::{Context, Result};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

/// Dependency manager
pub struct DependencyManager {
    /// Cache directory
    cache_dir: PathBuf,
    
    /// Installed dependencies
    installed: Arc<RwLock<HashMap<String, DependencyInfo>>>,
    
    /// Dependency registry
    registry: Arc<RwLock<HashMap<String, DependencyRegistryEntry>>>,
}

/// Dependency information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// Dependency name
    pub name: String,
    
    /// Dependency version
    pub version: Version,
    
    /// Dependency type
    pub dependency_type: DependencyType,
    
    /// Installation path
    pub path: PathBuf,
    
    /// Checksum
    pub checksum: String,
    
    /// Installation date
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

/// Dependency type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    /// WASM plugin
    WasmPlugin,
    
    /// Native library
    NativeLibrary,
    
    /// Data file
    DataFile,
    
    /// Configuration
    Configuration,
}

/// Dependency registry entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyRegistryEntry {
    /// Dependency name
    pub name: String,
    
    /// Available versions
    pub versions: Vec<Version>,
    
    /// Latest version
    pub latest: Version,
    
    /// Download URL template
    pub download_url: String,
    
    /// Dependency type
    pub dependency_type: DependencyType,
    
    /// Required permissions
    pub permissions: Vec<String>,
}

/// Dependency requirement
#[derive(Clone, Debug)]
pub struct DependencyRequirement {
    /// Dependency name
    pub name: String,
    
    /// Version requirement
    pub version_req: VersionReq,
    
    /// Optional flag
    pub optional: bool,
}

/// Resolution result
#[derive(Clone, Debug)]
pub struct ResolutionResult {
    /// Resolved dependencies
    pub dependencies: Vec<ResolvedDependency>,
    
    /// Conflicts
    pub conflicts: Vec<DependencyConflict>,
    
    /// Missing dependencies
    pub missing: Vec<String>,
}

/// Resolved dependency
#[derive(Clone, Debug)]
pub struct ResolvedDependency {
    /// Dependency name
    pub name: String,
    
    /// Resolved version
    pub version: Version,
    
    /// Source
    pub source: DependencySource,
}

/// Dependency source
#[derive(Clone, Debug)]
pub enum DependencySource {
    /// Already installed
    Installed,
    
    /// From registry
    Registry,
    
    /// From local file
    Local(PathBuf),
}

/// Dependency conflict
#[derive(Clone, Debug)]
pub struct DependencyConflict {
    /// Dependency name
    pub name: String,
    
    /// Conflicting requirements
    pub requirements: Vec<VersionReq>,
}

impl DependencyManager {
    /// Create a new dependency manager
    pub fn new(cache_dir: PathBuf) -> Result<Self> {
        info!("📦 Initializing Dependency Manager");
        
        // Create cache directory
        std::fs::create_dir_all(&cache_dir)?;
        
        // Load installed dependencies
        let installed = Self::load_installed(&cache_dir)?;
        
        // Load registry
        let registry = Self::load_registry(&cache_dir)?;
        
        info!("✅ Dependency manager initialized");
        info!("   - Installed: {} dependencies", installed.len());
        info!("   - Registry: {} entries", registry.len());
        
        Ok(Self {
            cache_dir,
            installed: Arc::new(RwLock::new(installed)),
            registry: Arc::new(RwLock::new(registry)),
        })
    }
    
    /// Load installed dependencies
    fn load_installed(cache_dir: &PathBuf) -> Result<HashMap<String, DependencyInfo>> {
        let installed_file = cache_dir.join("installed.json");
        
        if !installed_file.exists() {
            return Ok(HashMap::new());
        }
        
        let content = std::fs::read_to_string(&installed_file)?;
        let installed: HashMap<String, DependencyInfo> = serde_json::from_str(&content)?;
        
        Ok(installed)
    }
    
    /// Load dependency registry
    fn load_registry(cache_dir: &PathBuf) -> Result<HashMap<String, DependencyRegistryEntry>> {
        let registry_file = cache_dir.join("registry.json");
        
        if !registry_file.exists() {
            return Ok(HashMap::new());
        }
        
        let content = std::fs::read_to_string(&registry_file)?;
        let registry: HashMap<String, DependencyRegistryEntry> = serde_json::from_str(&content)?;
        
        Ok(registry)
    }
    
    /// Save installed dependencies
    async fn save_installed(&self) -> Result<()> {
        let installed = self.installed.read().await;
        let installed_file = self.cache_dir.join("installed.json");
        
        let content = serde_json::to_string_pretty(&*installed)?;
        tokio::fs::write(&installed_file, content).await?;
        
        Ok(())
    }
    
    /// Resolve dependencies
    pub async fn resolve(&self, requirements: &[DependencyRequirement]) -> Result<ResolutionResult> {
        debug!("🔍 Resolving {} dependencies", requirements.len());
        
        let mut resolved = Vec::new();
        let mut conflicts = Vec::new();
        let mut missing = Vec::new();
        
        let installed = self.installed.read().await;
        let registry = self.registry.read().await;
        
        for req in requirements {
            // Check if already installed
            if let Some(dep) = installed.get(&req.name) {
                if req.version_req.matches(&dep.version) {
                    resolved.push(ResolvedDependency {
                        name: req.name.clone(),
                        version: dep.version.clone(),
                        source: DependencySource::Installed,
                    });
                    continue;
                }
            }
            
            // Check registry
            if let Some(entry) = registry.get(&req.name) {
                // Find matching version
                if let Some(version) = entry.versions.iter()
                    .find(|v| req.version_req.matches(v))
                    .cloned() {
                    resolved.push(ResolvedDependency {
                        name: req.name.clone(),
                        version,
                        source: DependencySource::Registry,
                    });
                } else {
                    missing.push(req.name.clone());
                }
            } else {
                missing.push(req.name.clone());
            }
        }
        
        // Check for conflicts
        let mut version_map: HashMap<String, Vec<VersionReq>> = HashMap::new();
        for req in requirements {
            version_map.entry(req.name.clone())
                .or_insert_with(Vec::new)
                .push(req.version_req.clone());
        }
        
        for (name, versions) in version_map {
            if versions.len() > 1 {
                // Check if versions are compatible
                let first = &versions[0];
                let compatible = versions.iter().all(|v| {
                    // Simple check: if both are compatible with a common version
                    first == v
                });
                
                if !compatible {
                    conflicts.push(DependencyConflict {
                        name,
                        requirements: versions,
                    });
                }
            }
        }
        
        debug!("✅ Resolved {} dependencies", resolved.len());
        if !conflicts.is_empty() {
            warn!("⚠️  Found {} conflicts", conflicts.len());
        }
        if !missing.is_empty() {
            warn!("⚠️  Missing {} dependencies", missing.len());
        }
        
        Ok(ResolutionResult {
            dependencies: resolved,
            conflicts,
            missing,
        })
    }
    
    /// Install dependency
    pub async fn install(&self, name: &str, version: &Version) -> Result<DependencyInfo> {
        info!("📥 Installing dependency: {}@{}", name, version);
        
        let registry = self.registry.read().await;
        
        let entry = registry.get(name)
            .ok_or_else(|| anyhow::anyhow!("Dependency not found in registry: {}", name))?;
        
        // Check if already installed
        {
            let installed = self.installed.read().await;
            if let Some(dep) = installed.get(name) {
                if dep.version == *version {
                    debug!("✅ Dependency already installed: {}@{}", name, version);
                    return Ok(dep.clone());
                }
            }
        }
        
        // Download dependency
        let download_url = entry.download_url.replace("{version}", &version.to_string());
        let response = reqwest::get(&download_url).await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to download dependency: {}", response.status()));
        }
        
        let bytes = response.bytes().await?;
        
        // Calculate checksum
        let checksum = sha2::Sha256::digest(&bytes);
        let checksum_hex = hex::encode(checksum);
        
        // Save to cache
        let filename = format!("{}-{}.wasm", name, version);
        let path = self.cache_dir.join(&filename);
        tokio::fs::write(&path, bytes).await?;
        
        // Create dependency info
        let dep_info = DependencyInfo {
            name: name.to_string(),
            version: version.clone(),
            dependency_type: entry.dependency_type.clone(),
            path: path.clone(),
            checksum: checksum_hex,
            installed_at: chrono::Utc::now(),
        };
        
        // Update installed
        let mut installed = self.installed.write().await;
        installed.insert(name.to_string(), dep_info.clone());
        
        // Save
        self.save_installed().await?;
        
        info!("✅ Dependency installed: {}@{}", name, version);
        
        Ok(dep_info)
    }
    
    /// Uninstall dependency
    pub async fn uninstall(&self, name: &str) -> Result<()> {
        info!("🗑️  Uninstalling dependency: {}", name);
        
        let mut installed = self.installed.write().await;
        
        if let Some(dep) = installed.remove(name) {
            // Delete file
            if dep.path.exists() {
                tokio::fs::remove_file(&dep.path).await?;
            }
            
            // Save
            drop(installed);
            self.save_installed().await?;
            
            info!("✅ Dependency uninstalled: {}", name);
        } else {
            warn!("⚠️  Dependency not installed: {}", name);
        }
        
        Ok(())
    }
    
    /// Get installed dependency
    pub async fn get_installed(&self, name: &str) -> Option<DependencyInfo> {
        let installed = self.installed.read().await;
        installed.get(name).cloned()
    }
    
    /// Get all installed dependencies
    pub async fn list_installed(&self) -> Vec<DependencyInfo> {
        let installed = self.installed.read().await;
        installed.values().cloned().collect()
    }
    
    /// Update dependency
    pub async fn update(&self, name: &str) -> Result<Option<DependencyInfo>> {
        info!("🔄 Updating dependency: {}", name);
        
        let registry = self.registry.read().await;
        
        let entry = registry.get(name)
            .ok_or_else(|| anyhow::anyhow!("Dependency not found in registry: {}", name))?;
        
        let latest = &entry.latest;
        
        // Check if update needed
        {
            let installed = self.installed.read().await;
            if let Some(dep) = installed.get(name) {
                if dep.version >= *latest {
                    debug!("✅ Dependency already up to date: {}@{}", name, dep.version);
                    return Ok(None);
                }
            }
        }
        
        // Install latest version
        let dep_info = self.install(name, latest).await?;
        
        info!("✅ Dependency updated: {}@{}", name, latest);
        
        Ok(Some(dep_info))
    }
    
    /// Update all dependencies
    pub async fn update_all(&self) -> Result<Vec<DependencyInfo>> {
        info!("🔄 Updating all dependencies");
        
        let installed = self.installed.read().await;
        let names: Vec<String> = installed.keys().cloned().collect();
        drop(installed);
        
        let mut updated = Vec::new();
        
        for name in names {
            if let Some(dep) = self.update(&name).await? {
                updated.push(dep);
            }
        }
        
        info!("✅ Updated {} dependencies", updated.len());
        
        Ok(updated)
    }
    
    /// Check for updates
    pub async fn check_updates(&self) -> Result<Vec<(String, Version, Version)>> {
        debug!("🔍 Checking for dependency updates");
        
        let installed = self.installed.read().await;
        let registry = self.registry.read().await;
        
        let mut updates = Vec::new();
        
        for (name, dep) in installed.iter() {
            if let Some(entry) = registry.get(name) {
                if dep.version < entry.latest {
                    updates.push((
                        name.clone(),
                        dep.version.clone(),
                        entry.latest.clone(),
                    ));
                }
            }
        }
        
        debug!("✅ Found {} updates", updates.len());
        
        Ok(updates)
    }
    
    /// Add dependency to registry
    pub async fn add_to_registry(&self, entry: DependencyRegistryEntry) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.insert(entry.name.clone(), entry);
        Ok(())
    }
    
    /// Get cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_dependency_manager_creation() {
        let cache_dir = TempDir::new().unwrap();
        let manager = DependencyManager::new(cache_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_version_requirement() {
        let req = VersionReq::parse(">=1.0.0").unwrap();
        let v1 = Version::parse("1.0.0").unwrap();
        let v2 = Version::parse("2.0.0").unwrap();
        let v3 = Version::parse("0.9.0").unwrap();
        
        assert!(req.matches(&v1));
        assert!(req.matches(&v2));
        assert!(!req.matches(&v3));
    }
    
    #[test]
    fn test_dependency_type() {
        assert_eq!(DependencyType::WasmPlugin, DependencyType::WasmPlugin);
        assert_eq!(DependencyType::NativeLibrary, DependencyType::NativeLibrary);
    }
}