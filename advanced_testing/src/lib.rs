//! Vantis Advanced Testing Suite
//! 
//! This module provides advanced testing capabilities including:
//! - Property-based testing with Proptest
//! - Fuzzing tests with libFuzzer
//! - Performance regression testing
//! - Memory leak detection
//! - Concurrency stress tests

use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, debug, warn, error};

pub mod property_based;
pub mod fuzzing;
pub mod performance;
pub mod memory;
pub mod concurrency;
pub mod utils;

use property_based::PropertyBasedTestSuite;
use fuzzing::FuzzingTestSuite;
use performance::PerformanceRegressionTestSuite;
use memory::MemoryLeakDetector;
use concurrency::ConcurrencyStressTestSuite;

/// Advanced testing suite
pub struct AdvancedTestingSuite {
    /// Property-based test suite
    property_based: PropertyBasedTestSuite,
    
    /// Fuzzing test suite
    fuzzing: FuzzingTestSuite,
    
    /// Performance regression test suite
    performance: PerformanceRegressionTestSuite,
    
    /// Memory leak detector
    memory: MemoryLeakDetector,
    
    /// Concurrency stress test suite
    concurrency: ConcurrencyStressTestSuite,
    
    /// Test output directory
    output_dir: PathBuf,
    
    /// Baseline directory
    baseline_dir: PathBuf,
}

/// Advanced testing configuration
#[derive(Clone, Debug)]
pub struct AdvancedTestingConfig {
    /// Enable property-based testing
    pub enable_property_based: bool,
    
    /// Enable fuzzing
    pub enable_fuzzing: bool,
    
    /// Enable performance regression testing
    pub enable_performance: bool,
    
    /// Enable memory leak detection
    pub enable_memory: bool,
    
    /// Enable concurrency stress testing
    pub enable_concurrency: bool,
    
    /// Test iterations for property-based tests
    pub property_test_iterations: u32,
    
    /// Fuzzing duration in seconds
    pub fuzzing_duration: u64,
    
    /// Performance test iterations
    pub performance_iterations: u32,
    
    /// Memory leak threshold in bytes
    pub memory_leak_threshold: usize,
    
    /// Concurrency test threads
    pub concurrency_threads: usize,
    
    /// Concurrency test duration in seconds
    pub concurrency_duration: u64,
}

impl Default for AdvancedTestingConfig {
    fn default() -> Self {
        Self {
            enable_property_based: true,
            enable_fuzzing: true,
            enable_performance: true,
            enable_memory: true,
            enable_concurrency: true,
            property_test_iterations: 1000,
            fuzzing_duration: 60,
            performance_iterations: 100,
            memory_leak_threshold: 1024 * 1024, // 1 MB
            concurrency_threads: 16,
            concurrency_duration: 30,
        }
    }
}

impl AdvancedTestingSuite {
    /// Create a new advanced testing suite
    pub fn new(output_dir: PathBuf, baseline_dir: PathBuf) -> Result<Self> {
        Self::with_config(output_dir, baseline_dir, AdvancedTestingConfig::default())
    }
    
    /// Create with custom configuration
    pub fn with_config(
        output_dir: PathBuf,
        baseline_dir: PathBuf,
        config: AdvancedTestingConfig,
    ) -> Result<Self> {
        info!("🧪 Initializing Advanced Testing Suite");
        
        // Create directories if they don't exist
        std::fs::create_dir_all(&output_dir)?;
        std::fs::create_dir_all(&baseline_dir)?;
        
        // Initialize test suites
        let property_based = PropertyBasedTestSuite::new(config.property_test_iterations)?;
        let fuzzing = FuzzingTestSuite::new(config.fuzzing_duration)?;
        let performance = PerformanceRegressionTestSuite::new(
            baseline_dir.clone(),
            config.performance_iterations,
        )?;
        let memory = MemoryLeakDetector::new(config.memory_leak_threshold)?;
        let concurrency = ConcurrencyStressTestSuite::new(
            config.concurrency_threads,
            config.concurrency_duration,
        )?;
        
        info!("✅ Advanced testing suite initialized");
        info!("   - Property-based testing: {}", if config.enable_property_based { "Enabled" } else { "Disabled" });
        info!("   - Fuzzing: {}", if config.enable_fuzzing { "Enabled" } else { "Disabled" });
        info!("   - Performance regression: {}", if config.enable_performance { "Enabled" } else { "Disabled" });
        info!("   - Memory leak detection: {}", if config.enable_memory { "Enabled" } else { "Disabled" });
        info!("   - Concurrency stress: {}", if config.enable_concurrency { "Enabled" } else { "Disabled" });
        
        Ok(Self {
            property_based,
            fuzzing,
            performance,
            memory,
            concurrency,
            output_dir,
            baseline_dir,
        })
    }
    
    /// Get property-based test suite
    pub fn property_based(&self) -> &PropertyBasedTestSuite {
        &self.property_based
    }
    
    /// Get fuzzing test suite
    pub fn fuzzing(&self) -> &FuzzingTestSuite {
        &self.fuzzing
    }
    
    /// Get performance regression test suite
    pub fn performance(&self) -> &PerformanceRegressionTestSuite {
        &self.performance
    }
    
    /// Get memory leak detector
    pub fn memory(&self) -> &MemoryLeakDetector {
        &self.memory
    }
    
    /// Get concurrency stress test suite
    pub fn concurrency(&self) -> &ConcurrencyStressTestSuite {
        &self.concurrency
    }
    
    /// Run all tests
    pub async fn run_all(&self) -> Result<TestResults> {
        info!("🚀 Running all advanced tests");
        
        let mut results = TestResults::default();
        
        // Run property-based tests
        if let Err(e) = self.run_property_based_tests().await {
            error!("Property-based tests failed: {}", e);
            results.property_based_failed = true;
        } else {
            results.property_based_passed = true;
        }
        
        // Run fuzzing tests
        if let Err(e) = self.run_fuzzing_tests().await {
            error!("Fuzzing tests failed: {}", e);
            results.fuzzing_failed = true;
        } else {
            results.fuzzing_passed = true;
        }
        
        // Run performance regression tests
        if let Err(e) = self.run_performance_tests().await {
            error!("Performance tests failed: {}", e);
            results.performance_failed = true;
        } else {
            results.performance_passed = true;
        }
        
        // Run memory leak detection
        if let Err(e) = self.run_memory_tests().await {
            error!("Memory tests failed: {}", e);
            results.memory_failed = true;
        } else {
            results.memory_passed = true;
        }
        
        // Run concurrency stress tests
        if let Err(e) = self.run_concurrency_tests().await {
            error!("Concurrency tests failed: {}", e);
            results.concurrency_failed = true;
        } else {
            results.concurrency_passed = true;
        }
        
        info!("✅ All advanced tests completed");
        
        Ok(results)
    }
    
    /// Run property-based tests
    pub async fn run_property_based_tests(&self) -> Result<()> {
        info!("🔬 Running property-based tests");
        
        self.property_based.run_all().await?;
        
        info!("✅ Property-based tests passed");
        
        Ok(())
    }
    
    /// Run fuzzing tests
    pub async fn run_fuzzing_tests(&self) -> Result<()> {
        info!("🎲 Running fuzzing tests");
        
        self.fuzzing.run_all().await?;
        
        info!("✅ Fuzzing tests passed");
        
        Ok(())
    }
    
    /// Run performance regression tests
    pub async fn run_performance_tests(&self) -> Result<()> {
        info!("⚡ Running performance regression tests");
        
        self.performance.run_all().await?;
        
        info!("✅ Performance regression tests passed");
        
        Ok(())
    }
    
    /// Run memory leak detection
    pub async fn run_memory_tests(&self) -> Result<()> {
        info!("💧 Running memory leak detection");
        
        self.memory.run_all().await?;
        
        info!("✅ Memory leak detection passed");
        
        Ok(())
    }
    
    /// Run concurrency stress tests
    pub async fn run_concurrency_tests(&self) -> Result<()> {
        info!("🔄 Running concurrency stress tests");
        
        self.concurrency.run_all().await?;
        
        info!("✅ Concurrency stress tests passed");
        
        Ok(())
    }
    
    /// Get output directory
    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }
    
    /// Get baseline directory
    pub fn baseline_dir(&self) -> &PathBuf {
        &self.baseline_dir
    }
}

/// Test results
#[derive(Clone, Debug, Default)]
pub struct TestResults {
    /// Property-based tests passed
    pub property_based_passed: bool,
    
    /// Property-based tests failed
    pub property_based_failed: bool,
    
    /// Fuzzing tests passed
    pub fuzzing_passed: bool,
    
    /// Fuzzing tests failed
    pub fuzzing_failed: bool,
    
    /// Performance tests passed
    pub performance_passed: bool,
    
    /// Performance tests failed
    pub performance_failed: bool,
    
    /// Memory tests passed
    pub memory_passed: bool,
    
    /// Memory tests failed
    pub memory_failed: bool,
    
    /// Concurrency tests passed
    pub concurrency_passed: bool,
    
    /// Concurrency tests failed
    pub concurrency_failed: bool,
}

impl TestResults {
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.property_based_passed
            && self.fuzzing_passed
            && self.performance_passed
            && self.memory_passed
            && self.concurrency_passed
    }
    
    /// Get total passed count
    pub fn passed_count(&self) -> usize {
        let mut count = 0;
        if self.property_based_passed { count += 1; }
        if self.fuzzing_passed { count += 1; }
        if self.performance_passed { count += 1; }
        if self.memory_passed { count += 1; }
        if self.concurrency_passed { count += 1; }
        count
    }
    
    /// Get total failed count
    pub fn failed_count(&self) -> usize {
        let mut count = 0;
        if self.property_based_failed { count += 1; }
        if self.fuzzing_failed { count += 1; }
        if self.performance_failed { count += 1; }
        if self.memory_failed { count += 1; }
        if self.concurrency_failed { count += 1; }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_advanced_testing_suite_creation() {
        let output_dir = TempDir::new().unwrap();
        let baseline_dir = TempDir::new().unwrap();
        
        let suite = AdvancedTestingSuite::new(
            output_dir.path().to_path_buf(),
            baseline_dir.path().to_path_buf(),
        );
        
        assert!(suite.is_ok());
    }
    
    #[test]
    fn test_default_config() {
        let config = AdvancedTestingConfig::default();
        
        assert!(config.enable_property_based);
        assert!(config.enable_fuzzing);
        assert!(config.enable_performance);
        assert!(config.enable_memory);
        assert!(config.enable_concurrency);
        assert_eq!(config.property_test_iterations, 1000);
        assert_eq!(config.fuzzing_duration, 60);
    }
    
    #[test]
    fn test_test_results() {
        let mut results = TestResults::default();
        
        assert_eq!(results.passed_count(), 0);
        assert_eq!(results.failed_count(), 0);
        assert!(!results.all_passed());
        
        results.property_based_passed = true;
        results.fuzzing_passed = true;
        results.performance_passed = true;
        results.memory_passed = true;
        results.concurrency_passed = true;
        
        assert_eq!(results.passed_count(), 5);
        assert_eq!(results.failed_count(), 0);
        assert!(results.all_passed());
    }
}