# Advanced Testing Suite - Features Guide

## Overview

The Advanced Testing Suite provides comprehensive testing capabilities for the Vantis Media Player, including property-based testing, fuzzing, performance regression testing, memory leak detection, and concurrency stress testing.

## Table of Contents

1. [Property-Based Testing](#property-based-testing)
2. [Fuzzing Tests](#fuzzing-tests)
3. [Performance Regression Testing](#performance-regression-testing)
4. [Memory Leak Detection](#memory-leak-detection)
5. [Concurrency Stress Testing](#concurrency-stress-testing)
6. [Configuration](#configuration)
7. [Usage Examples](#usage-examples)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)

---

## Property-Based Testing

Property-based testing uses Proptest to test invariants and properties across a wide range of randomly generated inputs, helping find edge cases that traditional unit tests might miss.

### Features

- **Random Input Generation**: Generate random test inputs using strategies
- **Invariant Testing**: Test properties that should always hold true
- **Shrinking**: Automatically find minimal failing cases
- **Configurable Iterations**: Control the number of test iterations

### Usage

```rust
use vantisplayer::advanced_testing::AdvancedTestingSuite;

// Create testing suite
let suite = AdvancedTestingSuite::new(
    PathBuf::from("./test_output"),
    PathBuf::from("./baselines"),
)?;

// Run property-based tests
suite.run_property_based_tests().await?;

// Get results
let results = suite.get_property_based_results();
```

### Test Categories

**Core Tests:**
- Config serialization properties
- Event bus properties
- Player state properties

**Video Tests:**
- Video frame properties
- Video decoder properties

**Audio Tests:**
- Audio frame properties
- Audio volume properties

**Subtitle Tests:**
- Subtitle timing properties
- Subtitle encoding properties

**Plugin Tests:**
- Plugin permissions properties
- Plugin state properties

### Configuration

```rust
let config = AdvancedTestingConfig {
    enable_property_based: true,
    property_test_iterations: 1000,  // Number of iterations per test
    ..Default::default()
};
```

---

## Fuzzing Tests

Fuzzing tests use libFuzzer to find edge cases and security vulnerabilities through random input generation.

### Features

- **Random Input Generation**: Generate random inputs for testing
- **Crash Detection**: Detect crashes and panics
- **Unique Crash Tracking**: Track unique crash cases
- **Corpus Management**: Maintain a corpus of interesting inputs

### Usage

```rust
// Run fuzzing tests
suite.run_fuzzing_tests().await?;

// Get results
let results = suite.get_fuzzing_results();
```

### Test Categories

**Core Tests:**
- Config parsing fuzzing
- Event deserialization fuzzing

**Video Tests:**
- Video frame parsing fuzzing
- Video codec detection fuzzing

**Audio Tests:**
- Audio frame parsing fuzzing
- Audio codec detection fuzzing

**Subtitle Tests:**
- Subtitle parsing fuzzing
- Subtitle timing fuzzing

**Plugin Tests:**
- Plugin manifest parsing fuzzing
- Plugin permission check fuzzing

### Configuration

```rust
let config = AdvancedTestingConfig {
    enable_fuzzing: true,
    fuzzing_duration: 60,  // Duration in seconds
    ..Default::default()
};
```

---

## Performance Regression Testing

Performance regression testing uses Criterion to detect performance degradation across code changes.

### Features

- **Benchmarking**: Measure execution time of critical operations
- **Baseline Comparison**: Compare against baseline performance
- **Regression Detection**: Detect performance regressions
- **Statistical Analysis**: Statistical significance testing

### Usage

```rust
// Run performance regression tests
suite.run_performance_tests().await?;

// Get results
let results = suite.get_performance_results();
```

### Test Categories

**Core Tests:**
- Config parsing benchmark
- Event processing benchmark
- State updates benchmark

**Video Tests:**
- Video frame processing benchmark
- Video decoding benchmark

**Audio Tests:**
- Audio frame processing benchmark
- Audio volume adjustment benchmark

**Subtitle Tests:**
- Subtitle parsing benchmark
- Subtitle rendering benchmark

**Plugin Tests:**
- Plugin loading benchmark
- Plugin execution benchmark

### Configuration

```rust
let config = AdvancedTestingConfig {
    enable_performance: true,
    performance_iterations: 100,  // Number of iterations per benchmark
    ..Default::default()
};
```

### Baseline Management

```rust
// Save current performance as baseline
suite.save_baseline("v1.0.0").await?;

// Compare against baseline
suite.compare_against_baseline("v1.0.0").await?;
```

---

## Memory Leak Detection

Memory leak detection identifies memory leaks and memory usage issues in the codebase.

### Features

- **Memory Tracking**: Track memory allocations and deallocations
- **Leak Detection**: Detect memory leaks
- **Threshold Checking**: Check against memory leak thresholds
- **Snapshot Support**: Take and compare memory snapshots

### Usage

```rust
// Run memory leak detection tests
suite.run_memory_tests().await?;

// Get results
let results = suite.get_memory_results();

// Take memory snapshot
let snapshot = suite.take_memory_snapshot("before_test");

// Compare snapshots
let diff = suite.compare_snapshots("before_test", "after_test");
```

### Test Categories

**Core Tests:**
- Config memory test
- Event bus memory test
- State memory test

**Video Tests:**
- Video frame memory test
- Video decoder memory test

**Audio Tests:**
- Audio frame memory test
- Audio buffer memory test

**Subtitle Tests:**
- Subtitle memory test
- Subtitle cache memory test

**Plugin Tests:**
- Plugin memory test
- Plugin sandbox memory test

### Configuration

```rust
let config = AdvancedTestingConfig {
    enable_memory: true,
    memory_leak_threshold: 1024 * 1024,  // 1 MB threshold
    ..Default::default()
};
```

---

## Concurrency Stress Testing

Concurrency stress testing uses loom to test concurrent operations and identify race conditions and deadlocks.

### Features

- **Concurrent Operations**: Test operations under concurrent load
- **Race Condition Detection**: Detect race conditions
- **Deadlock Detection**: Detect deadlocks
- **Configurable Threads**: Control number of concurrent threads

### Usage

```rust
// Run concurrency stress tests
suite.run_concurrency_tests().await?;

// Get results
let results = suite.get_concurrency_results();
```

### Test Categories

**Core Tests:**
- Concurrent config access
- Concurrent event processing
- Concurrent state updates

**Video Tests:**
- Concurrent video frame processing
- Concurrent video decoding

**Audio Tests:**
- Concurrent audio frame processing
- Concurrent audio volume adjustment

**Subtitle Tests:**
- Concurrent subtitle parsing
- Concurrent subtitle rendering

**Plugin Tests:**
- Concurrent plugin loading
- Concurrent plugin execution

### Configuration

```rust
let config = AdvancedTestingConfig {
    enable_concurrency: true,
    concurrency_threads: 16,  // Number of concurrent threads
    concurrency_duration: 30,  // Duration in seconds
    ..Default::default()
};
```

---

## Configuration

The advanced testing suite can be configured with the following options:

```rust
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
```

### Default Configuration

```rust
let config = AdvancedTestingConfig::default();
```

Default values:
- `enable_property_based`: true
- `enable_fuzzing`: true
- `enable_performance`: true
- `enable_memory`: true
- `enable_concurrency`: true
- `property_test_iterations`: 1000
- `fuzzing_duration`: 60
- `performance_iterations`: 100
- `memory_leak_threshold`: 1,048,576 (1 MB)
- `concurrency_threads`: 16
- `concurrency_duration`: 30

---

## Usage Examples

### Running All Tests

```rust
use vantisplayer::advanced_testing::AdvancedTestingSuite;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create testing suite
    let suite = AdvancedTestingSuite::new(
        PathBuf::from("./test_output"),
        PathBuf::from("./baselines"),
    )?;
    
    // Run all tests
    suite.run_all().await?;
    
    // Generate report
    suite.generate_report().await?;
    
    Ok(())
}
```

### Running Specific Test Suites

```rust
// Run only property-based tests
suite.run_property_based_tests().await?;

// Run only fuzzing tests
suite.run_fuzzing_tests().await?;

// Run only performance tests
suite.run_performance_tests().await?;

// Run only memory tests
suite.run_memory_tests().await?;

// Run only concurrency tests
suite.run_concurrency_tests().await?;
```

### Custom Configuration

```rust
use vantisplayer::advanced_testing::AdvancedTestingConfig;

let config = AdvancedTestingConfig {
    enable_property_based: true,
    enable_fuzzing: false,  // Disable fuzzing
    enable_performance: true,
    enable_memory: true,
    enable_concurrency: true,
    property_test_iterations: 500,  // Fewer iterations
    fuzzing_duration: 30,
    performance_iterations: 50,
    memory_leak_threshold: 2 * 1024 * 1024,  // 2 MB threshold
    concurrency_threads: 8,  // Fewer threads
    concurrency_duration: 15,
};

let suite = AdvancedTestingSuite::with_config(
    PathBuf::from("./test_output"),
    PathBuf::from("./baselines"),
    config,
)?;
```

### Analyzing Results

```rust
// Get all results
let property_results = suite.get_property_based_results();
let fuzzing_results = suite.get_fuzzing_results();
let performance_results = suite.get_performance_results();
let memory_results = suite.get_memory_results();
let concurrency_results = suite.get_concurrency_results();

// Check for failures
for (name, result) in property_results {
    if !result.passed {
        eprintln!("Property test failed: {}", name);
        if let Some(failure) = result.failure_case {
            eprintln!("  Failure case: {}", failure);
        }
    }
}

// Check for regressions
for (name, result) in performance_results {
    if !result.passed {
        eprintln!("Performance regression detected: {}", name);
        if let Some(regression) = result.regression_pct {
            eprintln!("  Regression: {:.1}%", regression);
        }
    }
}

// Check for memory leaks
for (name, result) in memory_results {
    if !result.passed {
        eprintln!("Memory leak detected: {}", name);
        eprintln!("  Leaked: {} bytes", result.leaked);
    }
}

// Check for concurrency issues
for (name, result) in concurrency_results {
    if !result.passed {
        eprintln!("Concurrency issue detected: {}", name);
        eprintln!("  Race conditions: {}", result.race_conditions);
        eprintln!("  Deadlocks: {}", result.deadlocks);
    }
}
```

---

## Best Practices

### Property-Based Testing

1. **Test Invariants, Not Specific Values**: Focus on properties that should always hold true
2. **Use Appropriate Strategies**: Choose the right strategy for generating test inputs
3. **Keep Tests Fast**: Property tests should run quickly to allow many iterations
4. **Review Failing Cases**: Carefully analyze minimal failing cases

### Fuzzing

1. **Start with Short Durations**: Begin with short fuzzing durations and increase as needed
2. **Review Crashes**: Investigate all crashes found during fuzzing
3. **Maintain Corpus**: Keep a corpus of interesting inputs for future fuzzing runs
4. **Run Regularly**: Incorporate fuzzing into your CI/CD pipeline

### Performance Testing

1. **Establish Baselines**: Create baselines for all critical operations
2. **Run in Consistent Environment**: Ensure consistent testing environment
3. **Monitor Regressions**: Watch for performance regressions in CI/CD
4. **Profile Slow Code**: Use profiling tools to investigate performance issues

### Memory Testing

1. **Set Appropriate Thresholds**: Choose thresholds based on expected memory usage
2. **Test Long-Running Operations**: Focus on operations that run repeatedly
3. **Use Valgrind/ASan**: Complement with tools like Valgrind or AddressSanitizer
4. **Monitor in Production**: Consider adding memory monitoring in production

### Concurrency Testing

1. **Test with Multiple Threads**: Use various numbers of concurrent threads
2. **Run Multiple Times**: Concurrency tests should be run multiple times
3. **Use Loom for Verification**: Use loom for exhaustive testing of concurrent code
4. **Review Race Conditions**: Investigate all detected race conditions

---

## Troubleshooting

### Property-Based Tests Failing

**Problem**: Property tests are failing with unexpected inputs

**Solution**:
- Review the failing input and understand why it fails
- Check if the property being tested is correct
- Consider adding preconditions to filter invalid inputs
- Use shrinking to find minimal failing cases

### Fuzzing Finding Crashes

**Problem**: Fuzzing is finding crashes or panics

**Solution**:
- Investigate the crash input and reproduce the issue
- Add proper error handling and validation
- Fix the underlying bug causing the crash
- Add unit tests to prevent regression

### Performance Regressions Detected

**Problem**: Performance tests show regressions

**Solution**:
- Verify the regression is real (run multiple times)
- Profile the code to identify bottlenecks
- Review recent changes that might affect performance
- Consider optimizing the slow code or adjusting the baseline

### Memory Leaks Detected

**Problem**: Memory tests are detecting leaks

**Solution**:
- Verify the leak is real (run multiple times)
- Use tools like Valgrind or AddressSanitizer for detailed analysis
- Review code for missing deallocations or reference cycles
- Ensure proper cleanup in all code paths

### Concurrency Issues Detected

**Problem**: Concurrency tests are finding race conditions or deadlocks

**Solution**:
- Review the concurrent code for proper synchronization
- Use loom for exhaustive testing of concurrent code
- Add proper locking or use concurrent data structures
- Consider redesigning to avoid shared state

---

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Advanced Testing

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run advanced tests
        run: |
          cargo test --package vantis-advanced-testing --all-features
      - name: Upload test results
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test_output/
```

---

## Additional Resources

- **Main README**: [../README.md](../README.md)
- **API Documentation**: [../API_REFERENCE.md](../API_REFERENCE.md)
- **Testing Guide**: [../TROUBLESHOOTING.md](../TROUBLESHOOTING.md)
- **Contributing**: [../CONTRIBUTING.md](../CONTRIBUTING.md)

---

## Support

For issues or questions about the advanced testing suite:

- Open an issue on GitHub
- Check existing documentation
- Review API reference
- Join community discussions

Happy testing! 🧪