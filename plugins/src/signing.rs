//! Plugin Signing and Verification System
//!
//! Provides cryptographic signing and verification for plugins using Ed25519.
//! This ensures plugin authenticity and integrity.

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use tracing::{info, debug, warn};

// ============================================================================
// Cryptographic Types
// ============================================================================

/// Ed25519 public key (32 bytes)
pub type PublicKey = [u8; 32];

/// Ed25519 signature (64 bytes)
pub type Signature = [u8; 64];

/// SHA256 hash (32 bytes)
pub type Sha256Hash = [u8; 32];

// ============================================================================
// Plugin Signature
// ============================================================================

/// Plugin signature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSignature {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Plugin version
    pub version: String,
    
    /// Signature algorithm
    pub algorithm: SignatureAlgorithm,
    
    /// Public key of the signer
    pub public_key: String, // Base64 encoded
    
    /// Signature value
    pub signature: String, // Base64 encoded
    
    /// Hash of the signed content
    pub content_hash: String, // Hex encoded SHA256
    
    /// Timestamp when signed
    pub signed_at: chrono::DateTime<chrono::Utc>,
    
    /// Optional certificate chain
    pub certificate_chain: Vec<Certificate>,
}

/// Supported signature algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureAlgorithm {
    Ed25519,
    ECDSA_P256,
    RSA_PSS,
}

/// Certificate for chain validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// Certificate ID
    pub id: String,
    
    /// Certificate subject
    pub subject: String,
    
    /// Certificate issuer
    pub issuer: String,
    
    /// Public key
    pub public_key: String,
    
    /// Valid from
    pub valid_from: chrono::DateTime<chrono::Utc>,
    
    /// Valid until
    pub valid_until: chrono::DateTime<chrono::Utc>,
    
    /// Signature from issuer
    pub signature: String,
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Whether verification passed
    pub valid: bool,
    
    /// Signer identity (if verified)
    pub signer: Option<SignerInfo>,
    
    /// Trust level
    pub trust_level: TrustLevel,
    
    /// Verification details
    pub details: Vec<VerificationDetail>,
}

/// Signer information
#[derive(Debug, Clone)]
pub struct SignerInfo {
    /// Signer ID
    pub id: String,
    
    /// Signer name
    pub name: String,
    
    /// Whether the signer is verified
    pub verified: bool,
}

/// Trust level for a plugin
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Untrusted (unsigned or invalid signature)
    Untrusted = 0,
    
    /// Self-signed (no chain to trusted root)
    SelfSigned = 1,
    
    /// Community verified
    CommunityVerified = 2,
    
    /// Official (signed by Vantis)
    Official = 3,
}

/// Verification detail
#[derive(Debug, Clone)]
pub struct VerificationDetail {
    /// Check name
    pub check: String,
    
    /// Whether the check passed
    pub passed: bool,
    
    /// Additional message
    pub message: String,
}

// ============================================================================
// Trust Store
// ============================================================================

/// Trust store for managing trusted keys
pub struct TrustStore {
    /// Trusted public keys
    trusted_keys: HashSet<String>,
    
    /// Blocked public keys
    blocked_keys: HashSet<String>,
    
    /// Certificate authorities
    certificate_authorities: Vec<TrustedCA>,
}

/// Trusted Certificate Authority
#[derive(Debug, Clone)]
pub struct TrustedCA {
    /// CA ID
    pub id: String,
    
    /// CA name
    pub name: String,
    
    /// CA public key
    pub public_key: String,
    
    /// Trust level this CA grants
    pub trust_level: TrustLevel,
}

// ============================================================================
// Plugin Verifier
// ============================================================================

/// Plugin signature verifier
pub struct PluginVerifier {
    /// Trust store
    trust_store: TrustStore,
    
    /// Whether to allow unsigned plugins
    allow_unsigned: bool,
    
    /// Minimum required trust level
    min_trust_level: TrustLevel,
}

// ============================================================================
// Implementation
// ============================================================================

impl PluginSignature {
    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("Failed to parse signature JSON")
    }
    
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize signature")
    }
    
    /// Verify this signature against plugin data
    pub fn verify(&self, plugin_data: &[u8]) -> Result<VerificationResult> {
        // Verify content hash
        let computed_hash = compute_sha256(plugin_data);
        let expected_hash = hex::decode(&self.content_hash)
            .context("Invalid content hash format")?;
        
        let hash_valid = computed_hash.as_slice() == expected_hash.as_slice();
        
        if !hash_valid {
            return Ok(VerificationResult {
                valid: false,
                signer: None,
                trust_level: TrustLevel::Untrusted,
                details: vec![VerificationDetail {
                    check: "content_hash".to_string(),
                    passed: false,
                    message: "Content hash mismatch".to_string(),
                }],
            });
        }
        
        // Decode public key and signature
        let public_key_bytes = base64::decode(&self.public_key)
            .context("Invalid public key format")?;
        let signature_bytes = base64::decode(&self.signature)
            .context("Invalid signature format")?;
        
        if public_key_bytes.len() != 32 {
            bail!("Invalid public key length");
        }
        
        if signature_bytes.len() != 64 {
            bail!("Invalid signature length");
        }
        
        // Verify Ed25519 signature
        let public_key: PublicKey = public_key_bytes.as_slice().try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert public key"))?;
        let signature: Signature = signature_bytes.as_slice().try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert signature"))?;
        
        let sig_valid = verify_ed25519(&public_key, plugin_data, &signature)?;
        
        Ok(VerificationResult {
            valid: sig_valid,
            signer: None, // Populated by verifier with trust store
            trust_level: if sig_valid { TrustLevel::SelfSigned } else { TrustLevel::Untrusted },
            details: vec![
                VerificationDetail {
                    check: "content_hash".to_string(),
                    passed: true,
                    message: "Hash verified".to_string(),
                },
                VerificationDetail {
                    check: "signature".to_string(),
                    passed: sig_valid,
                    message: if sig_valid { "Signature valid".to_string() } else { "Invalid signature".to_string() },
                },
            ],
        })
    }
}

impl PluginVerifier {
    /// Create a new plugin verifier
    pub fn new() -> Self {
        Self {
            trust_store: TrustStore::with_defaults(),
            allow_unsigned: false,
            min_trust_level: TrustLevel::SelfSigned,
        }
    }
    
    /// Set whether to allow unsigned plugins
    pub fn allow_unsigned(mut self, allow: bool) -> Self {
        self.allow_unsigned = allow;
        self
    }
    
    /// Set minimum trust level
    pub fn min_trust_level(mut self, level: TrustLevel) -> Self {
        self.min_trust_level = level;
        self
    }
    
    /// Verify a plugin file
    pub fn verify_plugin(&self, plugin_path: &Path, signature: Option<&PluginSignature>) -> Result<VerificationResult> {
        info!("Verifying plugin: {:?}", plugin_path);
        
        // Read plugin data
        let plugin_data = std::fs::read(plugin_path)
            .context("Failed to read plugin file")?;
        
        // Check if plugin is signed
        let Some(sig) = signature else {
            if self.allow_unsigned {
                debug!("Plugin is unsigned but unsigned plugins are allowed");
                return Ok(VerificationResult {
                    valid: true,
                    signer: None,
                    trust_level: TrustLevel::Untrusted,
                    details: vec![VerificationDetail {
                        check: "signature".to_string(),
                        passed: false,
                        message: "Plugin is unsigned".to_string(),
                    }],
                });
            } else {
                warn!("Plugin is unsigned and unsigned plugins are not allowed");
                return Ok(VerificationResult {
                    valid: false,
                    signer: None,
                    trust_level: TrustLevel::Untrusted,
                    details: vec![VerificationDetail {
                        check: "signature".to_string(),
                        passed: false,
                        message: "Plugin is unsigned".to_string(),
                    }],
                });
            }
        };
        
        // Verify signature
        let mut result = sig.verify(&plugin_data)?;
        
        // Check trust store
        if result.valid {
            let key_b64 = &sig.public_key;
            
            // Check if blocked
            if self.trust_store.is_blocked(key_b64) {
                result.valid = false;
                result.trust_level = TrustLevel::Untrusted;
                result.details.push(VerificationDetail {
                    check: "trust_store".to_string(),
                    passed: false,
                    message: "Signer is blocked".to_string(),
                });
            } else if self.trust_store.is_trusted(key_b64) {
                // Find signer info
                result.signer = self.trust_store.get_signer_info(key_b64);
                result.trust_level = self.trust_store.get_trust_level(key_b64);
                result.details.push(VerificationDetail {
                    check: "trust_store".to_string(),
                    passed: true,
                    message: "Signer is trusted".to_string(),
                });
            }
        }
        
        // Check minimum trust level
        if result.valid && result.trust_level < self.min_trust_level {
            result.valid = false;
            result.details.push(VerificationDetail {
                check: "trust_level".to_string(),
                passed: false,
                message: format!(
                    "Trust level {:?} below minimum {:?}",
                    result.trust_level, self.min_trust_level
                ),
            });
        }
        
        info!("Verification result: valid={}, trust_level={:?}", result.valid, result.trust_level);
        Ok(result)
    }
    
    /// Add a trusted key
    pub fn add_trusted_key(&mut self, key: &str, signer: SignerInfo, trust_level: TrustLevel) {
        self.trust_store.add_trusted_key(key, signer, trust_level);
    }
    
    /// Block a key
    pub fn block_key(&mut self, key: &str) {
        self.trust_store.block_key(key);
    }
}

impl Default for PluginVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustStore {
    /// Create a new trust store
    pub fn new() -> Self {
        Self {
            trusted_keys: HashSet::new(),
            blocked_keys: HashSet::new(),
            certificate_authorities: Vec::new(),
        }
    }
    
    /// Create with default Vantis trusted keys
    pub fn with_defaults() -> Self {
        let mut store = Self::new();
        
        // Add Vantis official signing key
        // In production, this would be the actual Vantis public key
        store.add_trusted_key(
            "VANTIS_OFFICIAL_KEY_PLACEHOLDER",
            SignerInfo {
                id: "vantis-official".to_string(),
                name: "Vantis Team".to_string(),
                verified: true,
            },
            TrustLevel::Official,
        );
        
        store
    }
    
    /// Add a trusted key
    pub fn add_trusted_key(&mut self, key: &str, signer: SignerInfo, trust_level: TrustLevel) {
        self.trusted_keys.insert(key.to_string());
        
        // Store signer info and trust level (in production, this would be in a map)
        debug!("Added trusted key: {} -> {:?} (trust: {:?})", key, signer, trust_level);
    }
    
    /// Block a key
    pub fn block_key(&mut self, key: &str) {
        self.blocked_keys.insert(key.to_string());
        self.trusted_keys.remove(key);
        warn!("Blocked key: {}", key);
    }
    
    /// Check if a key is trusted
    pub fn is_trusted(&self, key: &str) -> bool {
        self.trusted_keys.contains(key)
    }
    
    /// Check if a key is blocked
    pub fn is_blocked(&self, key: &str) -> bool {
        self.blocked_keys.contains(key)
    }
    
    /// Get signer info for a key
    pub fn get_signer_info(&self, _key: &str) -> Option<SignerInfo> {
        // In production, this would look up in a map
        Some(SignerInfo {
            id: "unknown".to_string(),
            name: "Unknown Signer".to_string(),
            verified: false,
        })
    }
    
    /// Get trust level for a key
    pub fn get_trust_level(&self, _key: &str) -> TrustLevel {
        // In production, this would look up in a map
        TrustLevel::SelfSigned
    }
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Cryptographic Helpers
// ============================================================================

/// Compute SHA256 hash
fn compute_sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Verify Ed25519 signature
fn verify_ed25519(public_key: &PublicKey, message: &[u8], signature: &Signature) -> Result<bool> {
    use ed25519_dalek::{Signature as DalekSignature, VerifyingKey, Verifier};
    
    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|e| anyhow::anyhow!("Invalid public key: {}", e))?;
    
    let sig = DalekSignature::from_bytes(signature);
    
    match verifying_key.verify(message, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trust_store_creation() {
        let store = TrustStore::new();
        assert!(store.trusted_keys.is_empty());
        assert!(store.blocked_keys.is_empty());
    }
    
    #[test]
    fn test_trust_store_add_key() {
        let mut store = TrustStore::new();
        store.add_trusted_key(
            "test_key",
            SignerInfo {
                id: "test".to_string(),
                name: "Test".to_string(),
                verified: false,
            },
            TrustLevel::CommunityVerified,
        );
        
        assert!(store.is_trusted("test_key"));
        assert!(!store.is_blocked("test_key"));
    }
    
    #[test]
    fn test_trust_store_block_key() {
        let mut store = TrustStore::new();
        store.add_trusted_key(
            "test_key",
            SignerInfo {
                id: "test".to_string(),
                name: "Test".to_string(),
                verified: false,
            },
            TrustLevel::SelfSigned,
        );
        
        store.block_key("test_key");
        
        assert!(!store.is_trusted("test_key"));
        assert!(store.is_blocked("test_key"));
    }
    
    #[test]
    fn test_verifier_unsigned_plugin() {
        let verifier = PluginVerifier::new().allow_unsigned(true);
        let result = verifier.verify_plugin(Path::new("nonexistent.wasm"), None);
        
        // Should fail because file doesn't exist, not because unsigned
        assert!(result.is_err());
    }
    
    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Official > TrustLevel::CommunityVerified);
        assert!(TrustLevel::CommunityVerified > TrustLevel::SelfSigned);
        assert!(TrustLevel::SelfSigned > TrustLevel::Untrusted);
    }
}