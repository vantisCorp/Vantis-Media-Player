//! Advanced Testing Suite Example
//! 
//! This example demonstrates all features of the advanced testing suite:
//! - Property-based testing with Proptest
//! - Fuzzing tests with libFuzzer
//! - Performance regression testing with Criterion
//! - Memory leak detection
//! - Concurrency stress testing with loom

use std::path::PathBuf;
use tokio::time::sleep;
use std::time::Duration;
use vantisplayer::advanced_testing::{
    AdvancedTestingSuite, AdvancedTestingConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🧪 Advanced Testing Suite Example\n");
    
    // Create output and baseline directories
    let output_dir = PathBuf::from("./test_output");
    let baseline_dir = PathBuf::from("./baselines");
    
    // Create suite with custom configuration
    let config = AdvancedTestingConfig {
        enable_property_based: true,
        enable_fuzzing: true,
        enable_performance: true,
        enable_memory: true,
        enable_concurrency: true,
        property_test_iterations: 100,  // Reduced for demo
        fuzzing_duration: 10,  // Reduced for demo
        performance_iterations: 10,  // Reduced for demo
        memory_leak_threshold: 1024 * 1024,  // 1 MB
        concurrency_threads: 8,  // Reduced for demo
        concurrency_duration: 5,  // Reduced for demo
    };
    
    let suite = AdvancedTestingSuite::with_config(
        output_dir.clone(),
        baseline_dir.clone(),
        config,
    )?;
    
    // Example 1: Property-Based Testing
    println!("=== Example 1: Property-Based Testing ===\n");
    example_property_based_testing(&suite).await?;
    
    // Example 2: Fuzzing Tests
    println!("\n=== Example 2: Fuzzing Tests ===\n");
    example_fuzzing_tests(&suite).await?;
    
    // Example 3: Performance Regression Testing
    println!("\n=== Example 3: Performance Regression Testing ===\n");
    example_performance_testing(&suite).await?;
    
    // Example 4: Memory Leak Detection
    println!("\n=== Example 4: Memory Leak Detection ===\n");
    example_memory_testing(&suite).await?;
    
    // Example 5: Concurrency Stress Testing
    println!("\n=== Example 5: Concurrency Stress Testing ===\n");
    example_concurrency_testing(&suite).await?;
    
    // Example 6: Running All Tests
    println!("\n=== Example 6: Running All Tests ===\n");
    example_run_all_tests(&suite).await?;
    
    // Example 7: Analyzing Results
    println!("\n=== Example 7: Analyzing Results ===\n");
    example_analyze_results(&suite);
    
    // Example 8: Generating Reports
    println!("\n=== Example 8: Generating Reports ===\n");
    example_generate_reports(&suite).await?;
    
    println!("\n✅ All examples completed successfully!");
    
    Ok(())
}

/// Example 1: Property-Based Testing
async fn example_property_based_testing(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("🔬 Running property-based tests...");
    
    let start = std::time::Instant::now();
    suite.run_property_based_tests().await?;
    let duration = start.elapsed();
    
    println!("✅ Property-based tests completed in {:?}", duration);
    
    // Get results
    let results = suite.get_property_based_results();
    println!("\n📊 Results:");
    for (name, result) in results.iter().take(5) {
        println!("  {} - Passed: {}, Iterations: {}", 
            name, result.passed, result.iterations);
    }
    
    Ok(())
}

/// Example 2: Fuzzing Tests
async fn example_fuzzing_tests(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("🎲 Running fuzzing tests...");
    
    let start = std::time::Instant::now();
    suite.run_fuzzing_tests().await?;
    let duration = start.elapsed();
    
    println!("✅ Fuzzing tests completed in {:?}", duration);
    
    // Get results
    let results = suite.get_fuzzing_results();
    println!("\n📊 Results:");
    for (name, result) in results.iter().take(5) {
        println!("  {} - Passed: {}, Iterations: {}, Crashes: {}", 
            name, result.passed, result.iterations, result.crashes);
    }
    
    Ok(())
}

/// Example 3: Performance Regression Testing
async fn example_performance_testing(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("⚡ Running performance regression tests...");
    
    let start = std::time::Instant::now();
    suite.run_performance_tests().await?;
    let duration = start.elapsed();
    
    println!("✅ Performance tests completed in {:?}", duration);
    
    // Get results
    let results = suite.get_performance_results();
    println!("\n📊 Results:");
    for (name, result) in results.iter().take(5) {
        println!("  {} - Passed: {}, Mean: {}μs, StdDev: {}μs", 
            name, result.passed, result.mean_time / 1000, result.std_dev / 1000);
    }
    
    Ok(())
}

/// Example 4: Memory Leak Detection
async fn example_memory_testing(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("💧 Running memory leak detection tests...");
    
    let start = std::time::Instant::now();
    suite.run_memory_tests().await?;
    let duration = start.elapsed();
    
    println!("✅ Memory tests completed in {:?}", duration);
    
    // Get results
    let results = suite.get_memory_results();
    println!("\n📊 Results:");
    for (name, result) in results.iter().take(5) {
        println!("  {} - Passed: {}, Leaked: {} bytes", 
            name, result.passed, result.leaked);
    }
    
    Ok(())
}

/// Example 5: Concurrency Stress Testing
async fn example_concurrency_testing(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("🔀 Running concurrency stress tests...");
    
    let start = std::time::Instant::now();
    suite.run_concurrency_tests().await?;
    let duration = start.elapsed();
    
    println!("✅ Concurrency tests completed in {:?}", duration);
    
    // Get results
    let results = suite.get_concurrency_results();
    println!("\n📊 Results:");
    for (name, result) in results.iter().take(5) {
        println!("  {} - Passed: {}, Operations: {}, Threads: {}", 
            name, result.passed, result.operations, result.threads);
    }
    
    Ok(())
}

/// Example 6: Running All Tests
async fn example_run_all_tests(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("🚀 Running all tests...");
    
    let start = std::time::Instant::now();
    suite.run_all().await?;
    let duration = start.elapsed();
    
    println!("✅ All tests completed in {:?}", duration);
    
    Ok(())
}

/// Example 7: Analyzing Results
fn example_analyze_results(suite: &AdvancedTestingSuite) {
    println!("📈 Analyzing test results...");
    
    // Property-based results
    let property_results = suite.get_property_based_results();
    let property_passed = property_results.values().filter(|r| r.passed).count();
    let property_total = property_results.len();
    println!("\nProperty-Based Tests: {}/{} passed", property_passed, property_total);
    
    // Fuzzing results
    let fuzzing_results = suite.get_fuzzing_results();
    let fuzzing_passed = fuzzing_results.values().filter(|r| r.passed).count();
    let fuzzing_total = fuzzing_results.len();
    println!("Fuzzing Tests: {}/{} passed", fuzzing_passed, fuzzing_total);
    
    // Performance results
    let performance_results = suite.get_performance_results();
    let performance_passed = performance_results.values().filter(|r| r.passed).count();
    let performance_total = performance_results.len();
    println!("Performance Tests: {}/{} passed", performance_passed, performance_total);
    
    // Memory results
    let memory_results = suite.get_memory_results();
    let memory_passed = memory_results.values().filter(|r| r.passed).count();
    let memory_total = memory_results.len();
    println!("Memory Tests: {}/{} passed", memory_passed, memory_total);
    
    // Concurrency results
    let concurrency_results = suite.get_concurrency_results();
    let concurrency_passed = concurrency_results.values().filter(|r| r.passed).count();
    let concurrency_total = concurrency_results.len();
    println!("Concurrency Tests: {}/{} passed", concurrency_passed, concurrency_total);
    
    // Overall
    let total_passed = property_passed + fuzzing_passed + performance_passed + memory_passed + concurrency_passed;
    let total_tests = property_total + fuzzing_total + performance_total + memory_total + concurrency_total;
    let percentage = if total_tests > 0 {
        (total_passed as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    println!("\n📊 Overall: {}/{} tests passed ({:.1}%)", total_passed, total_tests, percentage);
}

/// Example 8: Generating Reports
async fn example_generate_reports(suite: &AdvancedTestingSuite) -> anyhow::Result<()> {
    println!("📄 Generating test reports...");
    
    // Generate comprehensive report
    suite.generate_report().await?;
    
    println!("✅ Reports generated successfully");
    println!("   - Check test_output/ directory for detailed reports");
    
    Ok(())
}