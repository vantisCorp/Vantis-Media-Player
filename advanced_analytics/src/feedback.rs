//! User Feedback System
//! 
//! Provides user feedback collection capabilities including:
//! - Feedback collection
//! - Feedback categorization
//! - Feedback analysis
//! - Feedback reporting

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::AnalyticsConfig;

/// Feedback collector
#[derive(Clone)]
pub struct FeedbackCollector {
    /// Configuration
    config: AnalyticsConfig,
    /// Feedback storage
    feedback: Arc<RwLock<Vec<Feedback>>>,
    /// Feedback statistics
    stats: Arc<RwLock<FeedbackStats>>,
}

/// User feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// Feedback ID
    pub feedback_id: String,
    /// User ID
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Feedback type
    pub feedback_type: FeedbackType,
    /// Feedback category
    pub category: FeedbackCategory,
    /// Rating (1-5)
    pub rating: Option<u8>,
    /// Feedback message
    pub message: String,
    /// Feedback timestamp
    pub timestamp: DateTime<Utc>,
    /// Feedback metadata
    pub metadata: HashMap<String, String>,
    /// Screenshot (base64 encoded)
    pub screenshot: Option<String>,
    /// System information
    pub system_info: SystemInfo,
}

/// Feedback type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackType {
    /// Bug report
    BugReport,
    /// Feature request
    FeatureRequest,
    /// General feedback
    General,
    /// Rating
    Rating,
    /// Support request
    Support,
}

/// Feedback category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackCategory {
    /// User interface
    UI,
    /// Performance
    Performance,
    /// Audio
    Audio,
    /// Video
    Video,
    /// Subtitles
    Subtitles,
    /// Plugins
    Plugins,
    /// Other
    Other,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Application version
    pub app_version: String,
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// Architecture
    pub arch: String,
    /// CPU cores
    pub cpu_cores: usize,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
}

/// Feedback statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStats {
    /// Total feedback
    pub total_feedback: usize,
    /// Feedback by type
    pub feedback_by_type: HashMap<FeedbackType, usize>,
    /// Feedback by category
    pub feedback_by_category: HashMap<FeedbackCategory, usize>,
    /// Average rating
    pub average_rating: f64,
    /// Most recent feedback
    pub most_recent_feedback: Option<DateTime<Utc>>,
}

impl FeedbackCollector {
    /// Create a new feedback collector
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing Feedback Collector");

        Ok(Self {
            config,
            feedback: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(FeedbackStats {
                total_feedback: 0,
                feedback_by_type: HashMap::new(),
                feedback_by_category: HashMap::new(),
                average_rating: 0.0,
                most_recent_feedback: None,
            })),
        })
    }

    /// Collect user feedback
    pub async fn collect(&self, feedback: Feedback) -> Result<()> {
        info!("Collecting feedback: {:?}", feedback.feedback_type);

        // Add to feedback storage
        let mut feedback_list = self.feedback.write().await;
        feedback_list.push(feedback.clone());

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_feedback += 1;
        *stats.feedback_by_type.entry(feedback.feedback_type).or_insert(0) += 1;
        *stats.feedback_by_category.entry(feedback.category).or_insert(0) += 1;
        stats.most_recent_feedback = Some(feedback.timestamp);

        // Update average rating
        if let Some(rating) = feedback.rating {
            let total_rating = stats.average_rating * (stats.total_feedback - 1) as f64;
            stats.average_rating = (total_rating + rating as f64) / stats.total_feedback as f64;
        }

        info!("Feedback collected: {}", feedback.feedback_id);
        Ok(())
    }

    /// Create a bug report
    pub async fn create_bug_report(
        &self,
        user_id: Option<String>,
        message: String,
        category: FeedbackCategory,
        screenshot: Option<String>,
    ) -> Result<String> {
        let feedback = Feedback {
            feedback_id: uuid::Uuid::new_v4().to_string(),
            user_id,
            session_id: self.config.session_id.clone(),
            feedback_type: FeedbackType::BugReport,
            category,
            rating: None,
            message,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            screenshot,
            system_info: self.collect_system_info(),
        };

        self.collect(feedback).await?;
        Ok(feedback.feedback_id.clone())
    }

    /// Create a feature request
    pub async fn create_feature_request(
        &self,
        user_id: Option<String>,
        message: String,
        category: FeedbackCategory,
    ) -> Result<String> {
        let feedback = Feedback {
            feedback_id: uuid::Uuid::new_v4().to_string(),
            user_id,
            session_id: self.config.session_id.clone(),
            feedback_type: FeedbackType::FeatureRequest,
            category,
            rating: None,
            message,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            screenshot: None,
            system_info: self.collect_system_info(),
        };

        self.collect(feedback).await?;
        Ok(feedback.feedback_id.clone())
    }

    /// Create a rating
    pub async fn create_rating(
        &self,
        user_id: Option<String>,
        rating: u8,
        message: Option<String>,
        category: FeedbackCategory,
    ) -> Result<String> {
        let feedback = Feedback {
            feedback_id: uuid::Uuid::new_v4().to_string(),
            user_id,
            session_id: self.config.session_id.clone(),
            feedback_type: FeedbackType::Rating,
            category,
            rating: Some(rating.clamp(1, 5)),
            message: message.unwrap_or_default(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            screenshot: None,
            system_info: self.collect_system_info(),
        };

        self.collect(feedback).await?;
        Ok(feedback.feedback_id.clone())
    }

    /// Get all feedback
    pub async fn get_feedback(&self) -> Vec<Feedback> {
        self.feedback.read().await.clone()
    }

    /// Get feedback by type
    pub async fn get_feedback_by_type(&self, feedback_type: FeedbackType) -> Vec<Feedback> {
        self.feedback
            .read()
            .await
            .iter()
            .filter(|f| f.feedback_type == feedback_type)
            .cloned()
            .collect()
    }

    /// Get feedback by category
    pub async fn get_feedback_by_category(&self, category: FeedbackCategory) -> Vec<Feedback> {
        self.feedback
            .read()
            .await
            .iter()
            .filter(|f| f.category == category)
            .cloned()
            .collect()
    }

    /// Get feedback statistics
    pub async fn get_stats(&self) -> FeedbackStats {
        self.stats.read().await.clone()
    }

    /// Get feedback for user
    pub async fn get_user_feedback(&self, user_id: &str) -> Vec<Feedback> {
        self.feedback
            .read()
            .await
            .iter()
            .filter(|f| f.user_id.as_deref() == Some(user_id))
            .cloned()
            .collect()
    }

    /// Collect system information
    fn collect_system_info(&self) -> SystemInfo {
        SystemInfo {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            os_version: "unknown".to_string(), // Would need platform-specific code
            arch: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get(),
            total_memory: 0, // Would need platform-specific code
            available_memory: 0,
        }
    }

    /// Flush feedback to analytics endpoint
    pub async fn flush(&self) -> Result<()> {
        if let Some(endpoint) = &self.config.analytics_endpoint {
            let feedback_list = self.feedback.read().await.clone();
            
            if feedback_list.is_empty() {
                return Ok(());
            }

            debug!("Flushing {} feedback items to {}", feedback_list.len(), endpoint);

            // In a real implementation, this would send feedback to the analytics endpoint
            // For now, we'll just clear the feedback
            
            self.feedback.write().await.clear();
            
            info!("Flushed {} feedback items", feedback_list.len());
        }

        Ok(())
    }

    /// Clear old feedback
    pub async fn clear_old_feedback(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = Utc::now() - older_than;
        
        let mut feedback = self.feedback.write().await;
        let initial_count = feedback.len();
        feedback.retain(|f| f.timestamp > cutoff);
        let removed = initial_count - feedback.len();

        info!("Cleared {} old feedback items", removed);
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feedback_collector_creation() {
        let config = AnalyticsConfig::default();
        let collector = FeedbackCollector::new(config).await;
        assert!(collector.is_ok());
    }

    #[tokio::test]
    async fn test_create_bug_report() {
        let collector = FeedbackCollector::new(AnalyticsConfig::default()).await.unwrap();
        let result = collector.create_bug_report(
            Some("user123".to_string()),
            "Test bug".to_string(),
            FeedbackCategory::UI,
            None,
        ).await;
        assert!(result.is_ok());
        
        let stats = collector.get_stats().await;
        assert_eq!(stats.total_feedback, 1);
    }

    #[tokio::test]
    async fn test_create_rating() {
        let collector = FeedbackCollector::new(AnalyticsConfig::default()).await.unwrap();
        let result = collector.create_rating(
            Some("user123".to_string()),
            5,
            Some("Great!".to_string()),
            FeedbackCategory::Other,
        ).await;
        assert!(result.is_ok());
        
        let stats = collector.get_stats().await;
        assert_eq!(stats.total_feedback, 1);
        assert_eq!(stats.average_rating, 5.0);
    }
}