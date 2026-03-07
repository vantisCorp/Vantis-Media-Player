//! Vantis Media Player Core
//!
//! Core primitives and shared functionality for the Vantis Media Player ecosystem.

pub mod config;
pub mod error;
pub mod events;
pub mod state;
pub mod types;

pub use config::*;
pub use error::*;
pub use events::*;
pub use state::*;
pub use types::*;

/// Core library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");