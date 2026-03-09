//! Sentry integration for error tracking
//! 
//! This module provides Sentry integration for real-time error monitoring.

use anyhow::Result;
use sentry::{ClientOptions, IntoDsn};

/// Initialize Sentry for error tracking
pub fn init_sentry(environment: &str, release: &str) -> Result<()> {
    let dsn = std::env::var("SENTRY_DSN")
        .ok()
        .and_then(|d| d.into_dsn().transpose().ok())
        .flatten();

    if let Some(dsn) = dsn {
        let _guard = sentry::init((
            dsn,
            ClientOptions {
                environment: Some(environment.into()),
                release: Some(release.into()),
                ..Default::default()
            },
        ));
        
        tracing::info!("Sentry initialized for environment: {}", environment);
    } else {
        tracing::warn!("SENTRY_DSN not configured, error tracking disabled");
    }

    Ok(())
}

/// Capture an error to Sentry
pub fn capture_error(error: &anyhow::Error) {
    sentry::capture_error(error.root_cause());
}

/// Capture a message to Sentry
pub fn capture_message(message: &str, level: sentry::Level) {
    sentry::capture_message(message, level);
}

/// Add a breadcrumb to the current Sentry scope
pub fn add_breadcrumb(message: &str, category: &str) {
    sentry::add_breadcrumb(sentry::Breadcrumb {
        message: Some(message.to_string()),
        category: Some(category.to_string()),
        ..Default::default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentry_init_without_dsn() {
        // Should not fail when DSN is not set
        assert!(init_sentry("test", "1.0.0").is_ok());
    }
}