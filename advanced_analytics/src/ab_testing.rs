//! A/B Testing Framework
//! 
//! Provides A/B testing capabilities including:
//! - Experiment management
//! - Variant assignment
//! - Conversion tracking
//! - Statistical analysis

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::AnalyticsConfig;

/// A/B testing engine
#[derive(Clone)]
pub struct ABTestingEngine {
    /// Configuration
    config: AnalyticsConfig,
    /// Experiments
    experiments: Arc<RwLock<HashMap<String, Experiment>>>,
    /// User assignments
    assignments: Arc<RwLock<HashMap<String, HashMap<String, String>>>>, // user_id -> experiment_id -> variant_id
    /// Conversions
    conversions: Arc<RwLock<HashMap<String, HashMap<String, usize>>>>, // experiment_id -> variant_id -> count
}

/// Experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    /// Experiment ID
    pub experiment_id: String,
    /// Experiment name
    pub name: String,
    /// Experiment description
    pub description: String,
    /// Variants
    pub variants: Vec<Variant>,
    /// Experiment status
    pub status: ExperimentStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Traffic allocation (0.0 to 1.0)
    pub traffic_allocation: f64,
    /// Target criteria
    pub target_criteria: Option<TargetCriteria>,
    /// Experiment metadata
    pub metadata: HashMap<String, String>,
}

/// Variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    /// Variant ID
    pub variant_id: String,
    /// Variant name
    pub name: String,
    /// Variant description
    pub description: String,
    /// Traffic allocation (0.0 to 1.0)
    pub traffic_allocation: f64,
    /// Variant configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Experiment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    /// Draft
    Draft,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Completed
    Completed,
    /// Cancelled
    Cancelled,
}

/// Target criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCriteria {
    /// User segments
    pub user_segments: Vec<String>,
    /// Minimum user count
    pub min_users: Option<usize>,
    /// Maximum user count
    pub max_users: Option<usize>,
    /// Required attributes
    pub required_attributes: HashMap<String, String>,
}

impl ABTestingEngine {
    /// Create a new A/B testing engine
    pub async fn new(config: AnalyticsConfig) -> Result<Self> {
        info!("Initializing A/B Testing Engine");

        Ok(Self {
            config,
            experiments: Arc::new(RwLock::new(HashMap::new())),
            assignments: Arc::new(RwLock::new(HashMap::new())),
            conversions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create a new experiment
    pub async fn create_experiment(&self, experiment: Experiment) -> Result<()> {
        info!("Creating experiment: {}", experiment.experiment_id);

        let mut experiments = self.experiments.write().await;
        experiments.insert(experiment.experiment_id.clone(), experiment.clone());

        // Initialize conversions for this experiment
        let mut conversions = self.conversions.write().await;
        conversions.insert(experiment.experiment_id.clone(), HashMap::new());

        for variant in &experiment.variants {
            conversions
                .get_mut(&experiment.experiment_id)
                .unwrap()
                .insert(variant.variant_id.clone(), 0);
        }

        info!("Experiment created: {}", experiment.experiment_id);
        Ok(())
    }

    /// Get or create variant assignment for user
    pub async fn get_variant(&self, experiment_id: &str, user_id: &str) -> Result<Option<Variant>> {
        debug!("Getting variant for user {} in experiment {}", user_id, experiment_id);

        let experiments = self.experiments.read().await;
        let experiment = experiments.get(experiment_id);

        if experiment.is_none() {
            return Ok(None);
        }

        let experiment = experiment.unwrap();

        // Check if experiment is running
        if experiment.status != ExperimentStatus::Running {
            return Ok(None);
        }

        // Check traffic allocation
        if rand::random::<f64>() > experiment.traffic_allocation {
            return Ok(None);
        }

        // Check if user already has an assignment
        let assignments = self.assignments.read().await;
        if let Some(user_assignments) = assignments.get(user_id) {
            if let Some(variant_id) = user_assignments.get(experiment_id) {
                if let Some(variant) = experiment.variants.iter().find(|v| &v.variant_id == variant_id) {
                    return Ok(Some(variant.clone()));
                }
            }
        }
        drop(assignments);

        // Assign new variant
        let variant = self.assign_variant(experiment, user_id).await?;
        Ok(Some(variant))
    }

    /// Assign a variant to a user
    async fn assign_variant(&self, experiment: &Experiment, user_id: &str) -> Result<Variant> {
        let mut rng = rand::thread_rng();
        let mut cumulative = 0.0;
        let random = rand::random::<f64>();

        for variant in &experiment.variants {
            cumulative += variant.traffic_allocation;
            if random < cumulative {
                // Store assignment
                let mut assignments = self.assignments.write().await;
                assignments
                    .entry(user_id.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(experiment.experiment_id.clone(), variant.variant_id.clone());

                info!("Assigned variant {} to user {} in experiment {}", 
                    variant.variant_id, user_id, experiment.experiment_id);
                
                return Ok(variant.clone());
            }
        }

        // Fallback to first variant
        let variant = experiment.variants.first().unwrap();
        let mut assignments = self.assignments.write().await;
        assignments
            .entry(user_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(experiment.experiment_id.clone(), variant.variant_id.clone());

        Ok(variant.clone())
    }

    /// Track conversion for experiment
    pub async fn track_conversion(&self, experiment_id: &str, user_id: &str, variant_id: &str) -> Result<()> {
        debug!("Tracking conversion for user {} in experiment {} variant {}", 
            user_id, experiment_id, variant_id);

        // Verify assignment
        let assignments = self.assignments.read().await;
        let assigned_variant = assignments
            .get(user_id)
            .and_then(|a| a.get(experiment_id));

        if assigned_variant != Some(&variant_id.to_string()) {
            warn!("User {} not assigned to variant {} in experiment {}", 
                user_id, variant_id, experiment_id);
            return Ok(());
        }
        drop(assignments);

        // Increment conversion count
        let mut conversions = self.conversions.write().await;
        if let Some(experiment_conversions) = conversions.get_mut(experiment_id) {
            *experiment_conversions.entry(variant_id.to_string()).or_insert(0) += 1;
        }

        info!("Conversion tracked for user {} in experiment {} variant {}", 
            user_id, experiment_id, variant_id);
        Ok(())
    }

    /// Get experiment results
    pub async fn get_results(&self, experiment_id: &str) -> Option<ExperimentResults> {
        let experiments = self.experiments.read().await;
        let experiment = experiments.get(experiment_id)?;

        let conversions = self.conversions.read().await;
        let experiment_conversions = conversions.get(experiment_id)?;

        let assignments = self.assignments.read().await;
        let total_participants = assignments
            .values()
            .filter(|a| a.contains_key(experiment_id))
            .count();

        let total_conversions: usize = experiment_conversions.values().sum();
        let conversion_rate = if total_participants > 0 {
            total_conversions as f64 / total_participants as f64
        } else {
            0.0
        };

        let mut variant_results = HashMap::new();
        for variant in &experiment.variants {
            let participants = assignments
                .values()
                .filter(|a| a.get(experiment_id) == Some(&variant.variant_id))
                .count();
            let conversions_count = *experiment_conversions.get(&variant.variant_id).unwrap_or(&0);
            let variant_conversion_rate = if participants > 0 {
                conversions_count as f64 / participants as f64
            } else {
                0.0
            };

            variant_results.insert(
                variant.variant_id.clone(),
                VariantResult {
                    variant_id: variant.variant_id.clone(),
                    participants,
                    conversions: conversions_count,
                    conversion_rate: variant_conversion_rate,
                },
            );
        }

        Some(ExperimentResults {
            experiment_id: experiment_id.to_string(),
            total_participants,
            total_conversions,
            conversion_rate,
            variants: variant_results,
        })
    }

    /// Get all experiments
    pub async fn get_experiments(&self) -> Vec<Experiment> {
        self.experiments.read().await.values().cloned().collect()
    }

    /// Get experiment by ID
    pub async fn get_experiment(&self, experiment_id: &str) -> Option<Experiment> {
        self.experiments.read().await.get(experiment_id).cloned()
    }

    /// Update experiment status
    pub async fn update_experiment_status(&self, experiment_id: &str, status: ExperimentStatus) -> Result<()> {
        let mut experiments = self.experiments.write().await;
        if let Some(experiment) = experiments.get_mut(experiment_id) {
            experiment.status = status;
            info!("Experiment {} status updated to {:?}", experiment_id, status);
            Ok(())
        } else {
            anyhow::bail!("Experiment not found: {}", experiment_id)
        }
    }

    /// Delete experiment
    pub async fn delete_experiment(&self, experiment_id: &str) -> Result<()> {
        let mut experiments = self.experiments.write().await;
        experiments.remove(experiment_id);

        let mut conversions = self.conversions.write().await;
        conversions.remove(experiment_id);

        let mut assignments = self.assignments.write().await;
        for user_assignments in assignments.values_mut() {
            user_assignments.remove(experiment_id);
        }

        info!("Experiment {} deleted", experiment_id);
        Ok(())
    }

    /// Flush data
    pub async fn flush(&self) -> Result<()> {
        info!("Flushed A/B testing data");
        Ok(())
    }
}

/// Experiment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    /// Experiment ID
    pub experiment_id: String,
    /// Total participants
    pub total_participants: usize,
    /// Total conversions
    pub total_conversions: usize,
    /// Conversion rate
    pub conversion_rate: f64,
    /// Variant results
    pub variants: HashMap<String, VariantResult>,
}

/// Variant result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResult {
    /// Variant ID
    pub variant_id: String,
    /// Participants
    pub participants: usize,
    /// Conversions
    pub conversions: usize,
    /// Conversion rate
    pub conversion_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ab_testing_engine_creation() {
        let config = AnalyticsConfig::default();
        let engine = ABTestingEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_create_experiment() {
        let engine = ABTestingEngine::new(AnalyticsConfig::default()).await.unwrap();
        
        let experiment = Experiment {
            experiment_id: "test_exp".to_string(),
            name: "Test Experiment".to_string(),
            description: "Test".to_string(),
            variants: vec![
                Variant {
                    variant_id: "control".to_string(),
                    name: "Control".to_string(),
                    description: "Control variant".to_string(),
                    traffic_allocation: 0.5,
                    config: HashMap::new(),
                },
                Variant {
                    variant_id: "treatment".to_string(),
                    name: "Treatment".to_string(),
                    description: "Treatment variant".to_string(),
                    traffic_allocation: 0.5,
                    config: HashMap::new(),
                },
            ],
            status: ExperimentStatus::Running,
            start_time: Utc::now(),
            end_time: None,
            traffic_allocation: 1.0,
            target_criteria: None,
            metadata: HashMap::new(),
        };
        
        let result = engine.create_experiment(experiment).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_variant() {
        let engine = ABTestingEngine::new(AnalyticsConfig::default()).await.unwrap();
        
        let experiment = Experiment {
            experiment_id: "test_exp".to_string(),
            name: "Test Experiment".to_string(),
            description: "Test".to_string(),
            variants: vec![
                Variant {
                    variant_id: "control".to_string(),
                    name: "Control".to_string(),
                    description: "Control variant".to_string(),
                    traffic_allocation: 0.5,
                    config: HashMap::new(),
                },
                Variant {
                    variant_id: "treatment".to_string(),
                    name: "Treatment".to_string(),
                    description: "Treatment variant".to_string(),
                    traffic_allocation: 0.5,
                    config: HashMap::new(),
                },
            ],
            status: ExperimentStatus::Running,
            start_time: Utc::now(),
            end_time: None,
            traffic_allocation: 1.0,
            target_criteria: None,
            metadata: HashMap::new(),
        };
        
        engine.create_experiment(experiment).await.unwrap();
        
        let variant = engine.get_variant("test_exp", "user123").await;
        assert!(variant.is_some());
    }
}