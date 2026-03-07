//! Experimental Features Module
//!
//! Cutting-edge features including AI Agents, DAO governance,
//! Web3 integration, and VR/AR support.
//!
//! ⚠️ These features are experimental and may change rapidly.

pub mod ai_agents;
pub mod dao;
pub mod web3;
pub mod vr_ar;

/// Feature flags for experimental features
#[derive(Debug, Clone)]
pub struct ExperimentalConfig {
    /// Enable AI agent features
    pub ai_agents: bool,
    /// Enable DAO governance
    pub dao: bool,
    /// Enable Web3 integration
    pub web3: bool,
    /// Enable VR/AR support
    pub vr_ar: bool,
    /// Allow anonymous usage data
    pub anonymous_telemetry: bool,
}

impl Default for ExperimentalConfig {
    fn default() -> Self {
        Self {
            ai_agents: false,
            dao: false,
            web3: false,
            vr_ar: false,
            anonymous_telemetry: false,
        }
    }
}

/// Warning shown when enabling experimental features
pub const EXPERIMENTAL_WARNING: &str = r#"
╔══════════════════════════════════════════════════════════════╗
║                    ⚠️  EXPERIMENTAL FEATURES                 ║
╠══════════════════════════════════════════════════════════════╣
║  You are enabling experimental features. These may:          ║
║  • Change without notice                                     ║
║  • Have bugs or security issues                              ║
║  • Not work as expected                                      ║
║                                                              ║
║  Use at your own risk. Report issues to:                     ║
║  https://github.com/vantisCorp/Vantis-Media-Player/issues    ║
╚══════════════════════════════════════════════════════════════╝
"#;