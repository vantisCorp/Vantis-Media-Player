//! Webhook integration for notifications
//!
//! Supports Discord and Slack webhooks for alerts and notifications.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Webhook configuration
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub discord_url: Option<String>,
    pub slack_url: Option<String>,
}

/// Discord webhook payload
#[derive(Debug, Serialize)]
struct DiscordWebhook {
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
    fields: Vec<DiscordField>,
}

#[derive(Debug, Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool,
}

/// Slack webhook payload
#[derive(Debug, Serialize)]
struct SlackWebhook {
    text: String,
    attachments: Vec<SlackAttachment>,
}

#[derive(Debug, Serialize)]
struct SlackAttachment {
    color: String,
    title: String,
    text: String,
    fields: Vec<SlackField>,
}

#[derive(Debug, Serialize)]
struct SlackField {
    title: String,
    value: String,
    short: bool,
}

/// Webhook notification types
#[derive(Debug, Clone)]
pub enum Notification {
    /// New release published
    Release { version: String, changelog: String },
    /// Security alert
    SecurityAlert { severity: String, message: String },
    /// CI/CD status
    CiStatus { status: String, branch: String, commit: String },
    /// Error report
    Error { component: String, error: String },
}

/// Webhook client
pub struct WebhookClient {
    config: WebhookConfig,
    http: reqwest::Client,
}

impl WebhookClient {
    /// Create a new webhook client
    pub fn new(config: WebhookConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self { config, http })
    }

    /// Send notification to all configured webhooks
    pub async fn notify(&self, notification: Notification) -> Result<()> {
        if let Some(ref url) = self.config.discord_url {
            self.send_discord(url, &notification).await?;
        }

        if let Some(ref url) = self.config.slack_url {
            self.send_slack(url, &notification).await?;
        }

        Ok(())
    }

    async fn send_discord(&self, url: &str, notification: &Notification) -> Result<()> {
        let webhook = self.build_discord_payload(notification);
        
        self.http
            .post(url)
            .json(&webhook)
            .send()
            .await?;

        Ok(())
    }

    async fn send_slack(&self, url: &str, notification: &Notification) -> Result<()> {
        let webhook = self.build_slack_payload(notification);
        
        self.http
            .post(url)
            .json(&webhook)
            .send()
            .await?;

        Ok(())
    }

    fn build_discord_payload(&self, notification: &Notification) -> DiscordWebhook {
        match notification {
            Notification::Release { version, changelog } => DiscordWebhook {
                content: Some("🚀 New Release!".to_string()),
                embeds: vec![DiscordEmbed {
                    title: format!("Version {}", version),
                    description: changelog.clone(),
                    color: 0x00FF00, // Green
                    fields: vec![],
                }],
            },
            Notification::SecurityAlert { severity, message } => DiscordWebhook {
                content: Some("⚠️ Security Alert".to_string()),
                embeds: vec![DiscordEmbed {
                    title: format!("{} Severity Alert", severity),
                    description: message.clone(),
                    color: 0xFF0000, // Red
                    fields: vec![],
                }],
            },
            Notification::CiStatus { status, branch, commit } => DiscordWebhook {
                content: None,
                embeds: vec![DiscordEmbed {
                    title: format!("CI: {}", status),
                    description: String::new(),
                    color: if status == "success" { 0x00FF00 } else { 0xFF0000 },
                    fields: vec![
                        DiscordField { name: "Branch".into(), value: branch.clone(), inline: true },
                        DiscordField { name: "Commit".into(), value: commit.clone(), inline: true },
                    ],
                }],
            },
            Notification::Error { component, error } => DiscordWebhook {
                content: Some("❌ Error Occurred".to_string()),
                embeds: vec![DiscordEmbed {
                    title: component.clone(),
                    description: error.clone(),
                    color: 0xFF6600, // Orange
                    fields: vec![],
                }],
            },
        }
    }

    fn build_slack_payload(&self, notification: &Notification) -> SlackWebhook {
        match notification {
            Notification::Release { version, changelog } => SlackWebhook {
                text: "🚀 New Release!".to_string(),
                attachments: vec![SlackAttachment {
                    color: "good".to_string(),
                    title: format!("Version {}", version),
                    text: changelog.clone(),
                    fields: vec![],
                }],
            },
            Notification::SecurityAlert { severity, message } => SlackWebhook {
                text: "⚠️ Security Alert".to_string(),
                attachments: vec![SlackAttachment {
                    color: "danger".to_string(),
                    title: format!("{} Severity Alert", severity),
                    text: message.clone(),
                    fields: vec![],
                }],
            },
            Notification::CiStatus { status, branch, commit } => SlackWebhook {
                text: format!("CI: {}", status),
                attachments: vec![SlackAttachment {
                    color: if status == "success" { "good".to_string() } else { "danger".to_string() },
                    title: "Build Status".to_string(),
                    text: String::new(),
                    fields: vec![
                        SlackField { title: "Branch".into(), value: branch.clone(), short: true },
                        SlackField { title: "Commit".into(), value: commit.clone(), short: true },
                    ],
                }],
            },
            Notification::Error { component, error } => SlackWebhook {
                text: "❌ Error Occurred".to_string(),
                attachments: vec![SlackAttachment {
                    color: "warning".to_string(),
                    title: component.clone(),
                    text: error.clone(),
                    fields: vec![],
                }],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_build() {
        let notification = Notification::Release {
            version: "1.0.0".to_string(),
            changelog: "Initial release".to_string(),
        };

        let config = WebhookConfig {
            discord_url: None,
            slack_url: None,
        };

        let client = WebhookClient::new(config).unwrap();
        let payload = client.build_discord_payload(&notification);
        assert!(payload.content.is_some());
    }
}