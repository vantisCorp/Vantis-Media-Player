//! Plugin lifecycle management.

use crate::error::{PluginError, PluginResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Plugin lifecycle stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginState {
    /// Plugin is discovered but not loaded.
    Discovered,
    /// Plugin is loaded into memory.
    Loaded,
    /// Plugin dependencies are resolved.
    Resolved,
    /// Plugin is initialized and ready.
    Initialized,
    /// Plugin is enabled and active.
    Enabled,
    /// Plugin is disabled.
    Disabled,
    /// Plugin is in error state.
    Error,
    /// Plugin is being unloaded.
    Unloading,
    /// Plugin has been unloaded.
    Unloaded,
}

impl fmt::Display for PluginState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginState::Discovered => write!(f, "Discovered"),
            PluginState::Loaded => write!(f, "Loaded"),
            PluginState::Resolved => write!(f, "Resolved"),
            PluginState::Initialized => write!(f, "Initialized"),
            PluginState::Enabled => write!(f, "Enabled"),
            PluginState::Disabled => write!(f, "Disabled"),
            PluginState::Error => write!(f, "Error"),
            PluginState::Unloading => write!(f, "Unloading"),
            PluginState::Unloaded => write!(f, "Unloaded"),
        }
    }
}

impl PluginState {
    /// Check if this is an active state (plugin is usable).
    pub fn is_active(&self) -> bool {
        matches!(self, PluginState::Enabled | PluginState::Initialized)
    }

    /// Check if the plugin is loaded in memory.
    pub fn is_loaded(&self) -> bool {
        matches!(
            self,
            PluginState::Loaded
                | PluginState::Resolved
                | PluginState::Initialized
                | PluginState::Enabled
                | PluginState::Disabled
        )
    }

    /// Check if the plugin can be enabled.
    pub fn can_enable(&self) -> bool {
        matches!(self, PluginState::Initialized | PluginState::Disabled)
    }

    /// Check if the plugin can be disabled.
    pub fn can_disable(&self) -> bool {
        matches!(self, PluginState::Enabled)
    }

    /// Check if the plugin can be unloaded.
    pub fn can_unload(&self) -> bool {
        matches!(
            self,
            PluginState::Loaded
                | PluginState::Resolved
                | PluginState::Initialized
                | PluginState::Disabled
                | PluginState::Error
        )
    }
}

/// State transition for plugins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// Source state.
    pub from: PluginState,
    /// Target state.
    pub to: PluginState,
    /// Timestamp of transition.
    pub timestamp: u64,
    /// Optional reason for transition.
    pub reason: Option<String>,
}

impl StateTransition {
    /// Create a new state transition.
    pub fn new(from: PluginState, to: PluginState) -> Self {
        StateTransition {
            from,
            to,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            reason: None,
        }
    }

    /// Add a reason for the transition.
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Validate the transition.
    pub fn validate(&self) -> PluginResult<()> {
        if !is_valid_transition(self.from, self.to) {
            return Err(PluginError::InvalidTransition {
                from: self.from.to_string(),
                to: self.to.to_string(),
            });
        }
        Ok(())
    }
}

/// Check if a state transition is valid.
pub fn is_valid_transition(from: PluginState, to: PluginState) -> bool {
    match (from, to) {
        // Initial loading sequence
        (PluginState::Discovered, PluginState::Loaded) => true,
        (PluginState::Loaded, PluginState::Resolved) => true,
        (PluginState::Resolved, PluginState::Initialized) => true,
        (PluginState::Initialized, PluginState::Enabled) => true,

        // Enable/disable cycle
        (PluginState::Enabled, PluginState::Disabled) => true,
        (PluginState::Disabled, PluginState::Enabled) => true,

        // Unloading
        (PluginState::Loaded, PluginState::Unloading) => true,
        (PluginState::Resolved, PluginState::Unloading) => true,
        (PluginState::Initialized, PluginState::Unloading) => true,
        (PluginState::Enabled, PluginState::Unloading) => true,
        (PluginState::Disabled, PluginState::Unloading) => true,
        (PluginState::Error, PluginState::Unloading) => true,
        (PluginState::Unloading, PluginState::Unloaded) => true,

        // Error handling
        (PluginState::Loaded, PluginState::Error) => true,
        (PluginState::Resolved, PluginState::Error) => true,
        (PluginState::Initialized, PluginState::Error) => true,
        (PluginState::Enabled, PluginState::Error) => true,
        (PluginState::Disabled, PluginState::Error) => true,

        // Recovery from error
        (PluginState::Error, PluginState::Disabled) => true,
        (PluginState::Error, PluginState::Unloaded) => true,

        // Same state (no-op)
        (from, to) if from == to => true,

        _ => false,
    }
}

/// Plugin lifecycle manager.
#[derive(Debug, Default)]
pub struct PluginLifecycle {
    /// Current state.
    state: PluginState,
    /// State history.
    history: Vec<StateTransition>,
    /// Error message if in error state.
    error_message: Option<String>,
}

impl PluginLifecycle {
    /// Create a new lifecycle manager.
    pub fn new() -> Self {
        PluginLifecycle {
            state: PluginState::Discovered,
            history: Vec::new(),
            error_message: None,
        }
    }

    /// Get the current state.
    pub fn state(&self) -> PluginState {
        self.state
    }

    /// Get the state history.
    pub fn history(&self) -> &[StateTransition] {
        &self.history
    }

    /// Get the error message if in error state.
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Transition to a new state.
    pub fn transition(&mut self, to: PluginState) -> PluginResult<()> {
        self.transition_with_reason(to, None)
    }

    /// Transition to a new state with a reason.
    pub fn transition_with_reason(
        &mut self,
        to: PluginState,
        reason: Option<String>,
    ) -> PluginResult<()> {
        let transition = StateTransition::new(self.state, to);
        transition.validate()?;

        // Update error message
        if to == PluginState::Error {
            self.error_message = reason.clone();
        } else {
            self.error_message = None;
        }

        // Record transition
        let mut transition = StateTransition::new(self.state, to);
        transition.reason = reason;
        self.history.push(transition);

        // Update state
        self.state = to;

        Ok(())
    }

    /// Force a state transition (bypass validation).
    pub fn force_transition(&mut self, to: PluginState) {
        let transition = StateTransition::new(self.state, to);
        self.history.push(transition);
        self.state = to;
        self.error_message = None;
    }

    /// Set error state with a message.
    pub fn set_error(&mut self, message: impl Into<String>) {
        let message = message.into();
        let transition = StateTransition::new(self.state, PluginState::Error)
            .with_reason(message.clone());
        self.history.push(transition);
        self.state = PluginState::Error;
        self.error_message = Some(message);
    }

    /// Check if the plugin can transition to the given state.
    pub fn can_transition_to(&self, to: PluginState) -> bool {
        is_valid_transition(self.state, to)
    }

    /// Get the last transition.
    pub fn last_transition(&self) -> Option<&StateTransition> {
        self.history.last()
    }

    /// Get the time the plugin has been in the current state.
    pub fn time_in_state(&self) -> Option<std::time::Duration> {
        let last = self.history.last()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?;
        let elapsed = now.as_secs().saturating_sub(last.timestamp);
        Some(std::time::Duration::from_secs(elapsed))
    }

    /// Reset the lifecycle (clear history and return to discovered).
    pub fn reset(&mut self) {
        self.state = PluginState::Discovered;
        self.history.clear();
        self.error_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_properties() {
        assert!(PluginState::Enabled.is_active());
        assert!(PluginState::Initialized.is_active());
        assert!(!PluginState::Disabled.is_active());

        assert!(PluginState::Enabled.is_loaded());
        assert!(!PluginState::Unloaded.is_loaded());

        assert!(PluginState::Initialized.can_enable());
        assert!(PluginState::Enabled.can_disable());
    }

    #[test]
    fn test_valid_transitions() {
        assert!(is_valid_transition(PluginState::Discovered, PluginState::Loaded));
        assert!(is_valid_transition(PluginState::Loaded, PluginState::Resolved));
        assert!(is_valid_transition(PluginState::Initialized, PluginState::Enabled));
        assert!(is_valid_transition(PluginState::Enabled, PluginState::Disabled));
        
        assert!(!is_valid_transition(PluginState::Discovered, PluginState::Enabled));
        assert!(!is_valid_transition(PluginState::Loaded, PluginState::Enabled));
    }

    #[test]
    fn test_lifecycle_transitions() {
        let mut lifecycle = PluginLifecycle::new();
        assert_eq!(lifecycle.state(), PluginState::Discovered);

        lifecycle.transition(PluginState::Loaded).unwrap();
        assert_eq!(lifecycle.state(), PluginState::Loaded);

        lifecycle.transition(PluginState::Resolved).unwrap();
        lifecycle.transition(PluginState::Initialized).unwrap();
        lifecycle.transition(PluginState::Enabled).unwrap();
        assert_eq!(lifecycle.state(), PluginState::Enabled);
    }

    #[test]
    fn test_invalid_transition() {
        let mut lifecycle = PluginLifecycle::new();
        let result = lifecycle.transition(PluginState::Enabled);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_state() {
        let mut lifecycle = PluginLifecycle::new();
        lifecycle.transition(PluginState::Loaded).unwrap();
        lifecycle.set_error("Test error");

        assert_eq!(lifecycle.state(), PluginState::Error);
        assert_eq!(lifecycle.error_message(), Some("Test error"));
    }

    #[test]
    fn test_transition_validation() {
        let transition = StateTransition::new(PluginState::Discovered, PluginState::Loaded);
        assert!(transition.validate().is_ok());

        let invalid = StateTransition::new(PluginState::Discovered, PluginState::Enabled);
        assert!(invalid.validate().is_err());
    }
}