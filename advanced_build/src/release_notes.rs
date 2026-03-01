//! Release Notes Generator
//! 
//! Provides automated release notes generation from:
//! - Git commits
//! - Pull requests
//! - Issues
//! - CHANGELOG.md
//! - Version history

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Release information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInfo {
    /// Version number
    pub version: String,
    /// Release date
    pub release_date: DateTime<Utc>,
    /// Commit hash
    pub commit_hash: String,
    /// Branch name
    pub branch: String,
}

/// Release notes section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotesSection {
    /// Section title
    pub title: String,
    /// Section content
    pub content: Vec<String>,
}

/// Release notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotes {
    /// Release information
    pub release_info: ReleaseInfo,
    /// Sections
    pub sections: Vec<ReleaseNotesSection>,
    /// Contributors
    pub contributors: Vec<String>,
    /// Statistics
    pub stats: ReleaseStats,
}

/// Release statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseStats {
    /// Number of commits
    pub commits: usize,
    /// Number of PRs merged
    pub pull_requests: usize,
    /// Number of issues closed
    pub issues: usize,
    /// Number of contributors
    pub contributors: usize,
    /// Lines added
    pub lines_added: usize,
    /// Lines removed
    pub lines_removed: usize,
}

/// Release notes generator
#[derive(Clone)]
pub struct ReleaseNotesGenerator {
    /// Template directory
    template_dir: PathBuf,
    /// Custom templates
    templates: HashMap<String, String>,
}

impl ReleaseNotesGenerator {
    /// Create a new release notes generator
    pub async fn new() -> Result<Self> {
        info!("Initializing Release Notes Generator");

        Ok(Self {
            template_dir: PathBuf::from("./templates/release_notes"),
            templates: HashMap::new(),
        })
    }

    /// Generate release notes
    pub async fn generate(&self, release_info: ReleaseInfo) -> Result<String> {
        info!("Generating release notes for version: {}", release_info.version);

        // Get commits since last release
        let commits = self.get_commits_since_last_release(&release_info.version).await?;

        // Get merged PRs
        let pull_requests = self.get_merged_pull_requests(&release_info.version).await?;

        // Get closed issues
        let issues = self.get_closed_issues(&release_info.version).await?;

        // Get contributors
        let contributors = self.get_contributors(&commits).await?;

        // Calculate statistics
        let stats = self.calculate_stats(&commits, &pull_requests, &issues, &contributors);

        // Categorize changes
        let sections = self.categorize_changes(&commits, &pull_requests, &issues).await?;

        let notes = ReleaseNotes {
            release_info: release_info.clone(),
            sections,
            contributors,
            stats,
        };

        // Format as markdown
        let markdown = self.format_markdown(&notes)?;

        Ok(markdown)
    }

    /// Get commits since last release
    async fn get_commits_since_last_release(&self, version: &str) -> Result<Vec<CommitInfo>> {
        debug!("Getting commits since version: {}", version);

        // Get last release tag
        let last_tag = self.get_last_release_tag().await?;

        // Get commits
        let range = if let Some(tag) = last_tag {
            format!("{}..HEAD", tag)
        } else {
            "HEAD".to_string()
        };

        let output = std::process::Command::new("git")
            .args([
                "log",
                &range,
                "--pretty=format:%H|%an|%ae|%ad|%s",
                "--date=iso",
            ])
            .output()
            .context("Failed to get git log")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut commits = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                commits.push(CommitInfo {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    email: parts[2].to_string(),
                    date: parts[3].to_string(),
                    message: parts[4].to_string(),
                });
            }
        }

        Ok(commits)
    }

    /// Get merged pull requests
    async fn get_merged_pull_requests(&self, version: &str) -> Result<Vec<PRInfo>> {
        debug!("Getting merged PRs for version: {}", version);

        // Try to get from GitHub CLI
        let output = std::process::Command::new("gh")
            .args([
                "pr",
                "list",
                "--state",
                "merged",
                "--limit",
                "100",
                "--json",
                "number,title,author,mergedAt,body",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let prs: Vec<PRInfo> = serde_json::from_str(&stdout)
                    .unwrap_or_default();
                return Ok(prs);
            }
        }

        Ok(Vec::new())
    }

    /// Get closed issues
    async fn get_closed_issues(&self, version: &str) -> Result<Vec<IssueInfo>> {
        debug!("Getting closed issues for version: {}", version);

        // Try to get from GitHub CLI
        let output = std::process::Command::new("gh")
            .args([
                "issue",
                "list",
                "--state",
                "closed",
                "--limit",
                "100",
                "--json",
                "number,title,author,closedAt,body",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let issues: Vec<IssueInfo> = serde_json::from_str(&stdout)
                    .unwrap_or_default();
                return Ok(issues);
            }
        }

        Ok(Vec::new())
    }

    /// Get contributors
    async fn get_contributors(&self, commits: &[CommitInfo]) -> Result<Vec<String>> {
        let mut contributors: HashMap<String, usize> = HashMap::new();

        for commit in commits {
            *contributors.entry(commit.author.clone()).or_insert(0) += 1;
        }

        let mut sorted: Vec<String> = contributors
            .into_iter()
            .map(|(name, _)| name)
            .collect();

        sorted.sort();

        Ok(sorted)
    }

    /// Calculate statistics
    fn calculate_stats(
        &self,
        commits: &[CommitInfo],
        pull_requests: &[PRInfo],
        issues: &[IssueInfo],
        contributors: &[String],
    ) -> ReleaseStats {
        ReleaseStats {
            commits: commits.len(),
            pull_requests: pull_requests.len(),
            issues: issues.len(),
            contributors: contributors.len(),
            lines_added: 0, // Would need git diff --shortstat
            lines_removed: 0,
        }
    }

    /// Categorize changes
    async fn categorize_changes(
        &self,
        commits: &[CommitInfo],
        pull_requests: &[PRInfo],
        issues: &[IssueInfo],
    ) -> Result<Vec<ReleaseNotesSection>> {
        let mut sections = Vec::new();

        // Categorize commits
        let mut features = Vec::new();
        let mut fixes = Vec::new();
        let mut improvements = Vec::new();
        let mut breaking = Vec::new();
        let mut docs = Vec::new();
        let mut tests = Vec::new();
        let mut other = Vec::new();

        for commit in commits {
            let message = commit.message.to_lowercase();
            
            if message.starts_with("feat:") || message.starts_with("feature:") {
                features.push(commit.message.clone());
            } else if message.starts_with("fix:") || message.starts_with("bugfix:") {
                fixes.push(commit.message.clone());
            } else if message.starts_with("improve:") || message.starts_with("refactor:") {
                improvements.push(commit.message.clone());
            } else if message.contains("breaking") || message.contains("break!") {
                breaking.push(commit.message.clone());
            } else if message.starts_with("docs:") || message.starts_with("doc:") {
                docs.push(commit.message.clone());
            } else if message.starts_with("test:") || message.starts_with("tests:") {
                tests.push(commit.message.clone());
            } else {
                other.push(commit.message.clone());
            }
        }

        // Add PRs
        for pr in pull_requests {
            let title = pr.title.to_lowercase();
            if title.starts_with("feat") || title.starts_with("feature") {
                features.push(format!("{} (#{} by @{})", pr.title, pr.number, pr.author));
            } else if title.starts_with("fix") || title.starts_with("bugfix") {
                fixes.push(format!("{} (#{} by @{})", pr.title, pr.number, pr.author));
            }
        }

        // Create sections
        if !breaking.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "⚠️ Breaking Changes".to_string(),
                content: breaking,
            });
        }

        if !features.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "✨ New Features".to_string(),
                content: features,
            });
        }

        if !fixes.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "🐛 Bug Fixes".to_string(),
                content: fixes,
            });
        }

        if !improvements.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "🚀 Improvements".to_string(),
                content: improvements,
            });
        }

        if !docs.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "📚 Documentation".to_string(),
                content: docs,
            });
        }

        if !tests.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "✅ Tests".to_string(),
                content: tests,
            });
        }

        if !other.is_empty() {
            sections.push(ReleaseNotesSection {
                title: "🔧 Other Changes".to_string(),
                content: other,
            });
        }

        Ok(sections)
    }

    /// Format as markdown
    fn format_markdown(&self, notes: &ReleaseNotes) -> Result<String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str(&format!("# Release {}\n\n", notes.release_info.version));
        markdown.push_str(&format!(
            "**Released:** {}\n\n",
            notes.release_info.release_date.format("%Y-%m-%d")
        ));

        // Statistics
        markdown.push_str("## 📊 Statistics\n\n");
        markdown.push_str(&format!("- **Commits:** {}\n", notes.stats.commits));
        markdown.push_str(&format!("- **Pull Requests:** {}\n", notes.stats.pull_requests));
        markdown.push_str(&format!("- **Issues:** {}\n", notes.stats.issues));
        markdown.push_str(&format!("- **Contributors:** {}\n\n", notes.stats.contributors));

        // Sections
        for section in &notes.sections {
            markdown.push_str(&format!("## {}\n\n", section.title));
            for item in &section.content {
                markdown.push_str(&format!("- {}\n", item));
            }
            markdown.push_str("\n");
        }

        // Contributors
        if !notes.contributors.is_empty() {
            markdown.push_str("## 👥 Contributors\n\n");
            for contributor in &notes.contributors {
                markdown.push_str(&format!("- @{}\n", contributor));
            }
            markdown.push_str("\n");
        }

        // Footer
        markdown.push_str("---\n\n");
        markdown.push_str(&format!(
            "**Commit:** `{}`\n",
            notes.release_info.commit_hash
        ));
        markdown.push_str(&format!("**Branch:** `{}`\n", notes.release_info.branch));

        Ok(markdown)
    }

    /// Get last release tag
    async fn get_last_release_tag(&self) -> Result<Option<String>> {
        let output = std::process::Command::new("git")
            .args(["tag", "--sort=-version:refname"])
            .output()
            .context("Failed to get git tags")?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let tags: Vec<&str> = stdout.lines().collect();

        Ok(tags.first().map(|s| s.to_string()))
    }

    /// Load custom template
    pub fn load_template(&mut self, name: &str, template: String) {
        self.templates.insert(name.to_string(), template);
    }
}

/// Commit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
}

/// Pull request information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRInfo {
    pub number: u64,
    pub title: String,
    pub author: String,
    #[serde(rename = "mergedAt")]
    pub merged_at: Option<String>,
    pub body: Option<String>,
}

/// Issue information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueInfo {
    pub number: u64,
    pub title: String,
    pub author: String,
    #[serde(rename = "closedAt")]
    pub closed_at: Option<String>,
    pub body: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_release_notes_generator_creation() {
        let generator = ReleaseNotesGenerator::new().await;
        assert!(generator.is_ok());
    }

    #[test]
    fn test_release_info_serialization() {
        let info = ReleaseInfo {
            version: "1.0.0".to_string(),
            release_date: Utc::now(),
            commit_hash: "abc123".to_string(),
            branch: "main".to_string(),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: ReleaseInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.version, deserialized.version);
    }
}