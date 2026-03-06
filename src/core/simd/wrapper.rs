// Universal SIMD wrapper - automatically selects best available implementation
// Inspired by VideoLAN's runtime CPU feature detection

use super::{SimdLevel, SimdOps};
use std::sync::OnceLock;

/// Universal SIMD operations wrapper
pub struct UniversalSimdOps {
    ops: Box<dyn SimdOps>,
    level: SimdLevel,
}

impl UniversalSimdOps {
    /// Create universal SIMD ops with best available implementation
    pub fn new() -> Self {
        Self::with_level(SimdLevel::highest_available())
    }

    /// Create universal SIMD ops with specific SIMD level
    pub fn with_level(level: SimdLevel) -> Self {
        let ops: Box<dyn SimdOps> = match level {
            #[cfg(target_arch = "x86_64")]
            SimdLevel::Avx512 => {
                // TODO: Implement AVX-512 when needed
                println!("Warning: AVX-512 not implemented, falling back to AVX2");
                Box::new(super::avx2::Avx2Ops::new())
            }

            #[cfg(target_arch = "x86_64")]
            SimdLevel::Avx2 => Box::new(super::avx2::Avx2Ops::new()),

            #[cfg(target_arch = "x86_64")]
            SimdLevel::Sse42 => Box::new(super::sse42::Sse42Ops::new()),

            #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
            SimdLevel::Neon => Box::new(super::neon::NeonOps::new()),

            _ => Box::new(super::fallback::FallbackOps::new()),
        };

        Self { ops, level }
    }

    /// Get current SIMD level
    pub fn level(&self) -> SimdLevel {
        self.level
    }

    /// Get SIMD level description
    pub fn level_description(&self) -> &amp;'static str {
        self.level.as_str()
    }
}

impl Default for UniversalSimdOps {
    fn default() -> Self {
        Self::new()
    }
}

impl SimdOps for UniversalSimdOps {
    fn add(&amp;self, a: &amp;[f32], b: &amp;[f32], result: &amp;mut [f32]) {
        self.ops.add(a, b, result)
    }

    fn subtract(&amp;self, a: &amp;[f32], b: &amp;[f32], result: &amp;mut [f32]) {
        self.ops.subtract(a, b, result)
    }

    fn multiply(&amp;self, a: &amp;[f32], b: &amp;[f32], result: &amp;mut [f32]) {
        self.ops.multiply(a, b, result)
    }

    fn divide(&amp;self, a: &amp;[f32], b: &amp;[f32], result: &amp;mut [f32]) {
        self.ops.divide(a, b, result)
    }

    fn scale(&amp;self, a: &amp;[f32], scale: f32, result: &amp;mut [f32]) {
        self.ops.scale(a, scale, result)
    }

    fn dot_product(&amp;self, a: &amp;[f32], b: &amp;[f32]) -> f32 {
        self.ops.dot_product(a, b)
    }

    fn sum(&amp;self, a: &amp;[f32]) -> f32 {
        self.ops.sum(a)
    }

    fn mean(&amp;self, a: &amp;[f32]) -> f32 {
        self.ops.mean(a)
    }

    fn sqrt(&amp;self, a: &amp;[f32], result: &amp;mut [f32]) {
        self.ops.sqrt(a, result)
    }
}

/// Global singleton for SIMD operations
static SIMD_OPS: OnceLock<UniversalSimdOps> = OnceLock::new();

/// Get global SIMD operations instance
pub fn get_simd_ops() -> &amp;'static UniversalSimdOps {
    SIMD_OPS.get_or_init(|| {
        let ops = UniversalSimdOps::new();
        println!(
            "SIMD initialized with level: {}",
            ops.level_description()
        );
        ops
    })
}

/// SIMD benchmark helper
pub fn benchmark_simd(size: usize, iterations: usize) -> Vec<(&amp;'static str, u64)> {
    use std::time::Instant;

    let mut results = Vec::new();

    // Generate test data
    let a = vec![1.0f32; size];
    let b = vec![2.0f32; size];
    let mut result = vec![0.0f32; size];

    // Benchmark each available level
    let levels = vec![
        (SimdLevel::None, "Fallback"),
        #[cfg(target_arch = "x86_64")]
        (SimdLevel::Sse42, "SSE4.2"),
        #[cfg(target_arch = "x86_64")]
        (SimdLevel::Avx2, "AVX2"),
        #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
        (SimdLevel::Neon, "NEON"),
    ];

    for (level, name) in levels {
        if level == SimdLevel::None || level == SimdLevel::highest_available() {
            let ops = UniversalSimdOps::with_level(level);

            let start = Instant::now();
            for _ in 0..iterations {
                ops.add(&amp;a, &amp;b, &amp;mut result);
            }
            let duration = start.elapsed().as_micros();

            results.push((name, duration as u64));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_simd_ops() {
        let ops = UniversalSimdOps::new();
        println!("SIMD level: {}", ops.level_description());

        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        ops.add(&amp;a, &amp;b, &amp;mut result);
        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_universal_simd_multiply() {
        let ops = UniversalSimdOps::new();

        let a = vec![2.0f32; 16];
        let b = vec![3.0f32; 16];
        let mut result = vec![0.0f32; 16];

        ops.multiply(&amp;a, &amp;b, &amp;mut result);
        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_global_simd_ops() {
        let ops = get_simd_ops();
        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        ops.add(&amp;a, &amp;b, &amp;mut result);
        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_benchmark_simd() {
        let results = benchmark_simd(1024, 1000);
        println!("Benchmark results:");
        for (name, duration) in &amp;results {
            println!("  {}: {} μs", name, duration);
        }
        assert!(!results.is_empty());
    }
}