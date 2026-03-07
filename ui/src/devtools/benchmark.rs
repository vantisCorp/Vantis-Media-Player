//! Benchmark Module
//! 
//! Performance benchmarking utilities for development.

use std::time::{Duration, Instant};

/// Simple benchmark utility
#[derive(Debug)]
pub struct Benchmark {
    /// Benchmark name
    pub name: String,
    /// Start time
    start: Option<Instant>,
    /// Results
    results: Vec<BenchmarkResult>,
    /// Iteration count
    iterations: usize,
}

impl Benchmark {
    /// Create a new benchmark
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: None,
            results: Vec::new(),
            iterations: 0,
        }
    }
    
    /// Start the benchmark
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }
    
    /// Stop the benchmark and record result
    pub fn stop(&mut self) -> Duration {
        if let Some(start) = self.start.take() {
            let duration = start.elapsed();
            self.results.push(BenchmarkResult {
                iteration: self.iterations,
                duration,
            });
            self.iterations += 1;
            duration
        } else {
            Duration::ZERO
        }
    }
    
    /// Run a closure and measure its execution time
    pub fn measure<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.start();
        let result = f();
        self.stop();
        result
    }
    
    /// Run a closure multiple times and return average
    pub fn measure_iterations<F>(&mut self, iterations: usize, mut f: F) -> Duration
    where
        F: FnMut(),
    {
        for _ in 0..iterations {
            self.measure(&mut f);
        }
        self.average()
    }
    
    /// Get all results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
    
    /// Get average duration
    pub fn average(&self) -> Duration {
        if self.results.is_empty() {
            return Duration::ZERO;
        }
        
        let total: Duration = self.results.iter().map(|r| r.duration).sum();
        total / self.results.len() as u32
    }
    
    /// Get minimum duration
    pub fn min(&self) -> Duration {
        self.results
            .iter()
            .map(|r| r.duration)
            .min()
            .unwrap_or(Duration::ZERO)
    }
    
    /// Get maximum duration
    pub fn max(&self) -> Duration {
        self.results
            .iter()
            .map(|r| r.duration)
            .max()
            .unwrap_or(Duration::ZERO)
    }
    
    /// Get standard deviation
    pub fn std_dev(&self) -> Duration {
        if self.results.len() < 2 {
            return Duration::ZERO;
        }
        
        let avg = self.average();
        let variance: f64 = self.results
            .iter()
            .map(|r| {
                let diff = (r.duration.as_secs_f64() - avg.as_secs_f64()).powi(2);
                diff
            })
            .sum::<f64>() / (self.results.len() - 1) as f64;
        
        Duration::from_secs_f64(variance.sqrt())
    }
    
    /// Generate a report
    pub fn report(&self) -> BenchmarkReport {
        BenchmarkReport {
            name: self.name.clone(),
            iterations: self.iterations,
            average: self.average(),
            min: self.min(),
            max: self.max(),
            std_dev: self.std_dev(),
            results: self.results.clone(),
        }
    }
    
    /// Clear results
    pub fn clear(&mut self) {
        self.results.clear();
        self.iterations = 0;
    }
}

/// Single benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Iteration number
    pub iteration: usize,
    /// Duration
    pub duration: Duration,
}

/// Benchmark report
#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    /// Benchmark name
    pub name: String,
    /// Total iterations
    pub iterations: usize,
    /// Average duration
    pub average: Duration,
    /// Minimum duration
    pub min: Duration,
    /// Maximum duration
    pub max: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// All results
    pub results: Vec<BenchmarkResult>,
}

impl std::fmt::Display for BenchmarkReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== {} ===", self.name)?;
        writeln!(f, "Iterations: {}", self.iterations)?;
        writeln!(f, "Average: {:?}", self.average)?;
        writeln!(f, "Min: {:?}", self.min)?;
        writeln!(f, "Max: {:?}", self.max)?;
        writeln!(f, "Std Dev: {:?}", self.std_dev)?;
        
        if self.average > Duration::from_millis(100) {
            writeln!(f, "⚠️  Performance warning: Average > 100ms")?;
        }
        
        Ok(())
    }
}

/// Macro for quick benchmarking
#[macro_export]
macro_rules! bench {
    ($name:expr, $code:block) => {{
        let mut bench = $crate::devtools::Benchmark::new($name);
        bench.measure(|| $code);
        bench.report()
    }};
}

/// Macro for benchmarking multiple iterations
#[macro_export]
macro_rules! bench_iterations {
    ($name:expr, $iterations:expr, $code:block) => {{
        let mut bench = $crate::devtools::Benchmark::new($name);
        bench.measure_iterations($iterations, || $code);
        bench.report()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_basic() {
        let mut bench = Benchmark::new("test");
        bench.start();
        std::thread::sleep(Duration::from_millis(10));
        let duration = bench.stop();
        
        assert!(duration >= Duration::from_millis(10));
        assert_eq!(bench.results.len(), 1);
    }
    
    #[test]
    fn test_benchmark_measure() {
        let mut bench = Benchmark::new("test");
        let result = bench.measure(|| 42);
        
        assert_eq!(result, 42);
        assert_eq!(bench.iterations, 1);
    }
    
    #[test]
    fn test_benchmark_average() {
        let mut bench = Benchmark::new("test");
        
        bench.measure(|| std::thread::sleep(Duration::from_millis(5)));
        bench.measure(|| std::thread::sleep(Duration::from_millis(5)));
        bench.measure(|| std::thread::sleep(Duration::from_millis(5)));
        
        let avg = bench.average();
        assert!(avg >= Duration::from_millis(4));
        assert!(avg <= Duration::from_millis(10));
    }
    
    #[test]
    fn test_benchmark_report() {
        let mut bench = Benchmark::new("test");
        bench.measure(|| {});
        
        let report = bench.report();
        assert_eq!(report.name, "test");
        assert_eq!(report.iterations, 1);
    }
}