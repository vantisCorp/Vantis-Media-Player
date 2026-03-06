// Platform-Specific SIMD Optimizations
// Inspired by VideoLAN's dav1d decoder (80% assembly, extreme optimizations)

#[cfg(target_arch = "x86_64")]
pub mod avx2;

#[cfg(target_arch = "x86_64")]
pub mod sse42;

#[cfg(target_arch = "aarch64")]
pub mod neon;

#[cfg(target_arch = "arm")]
pub mod neon;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm"
)))]
pub mod fallback;

pub mod wrapper;

// Re-export for convenience
pub use wrapper::{get_simd_ops, UniversalSimdOps};

#[cfg(target_arch = "x86_64")]
pub use avx2::Avx2Ops;

#[cfg(target_arch = "x86_64")]
pub use sse42::Sse42Ops;

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub use neon::NeonOps;

pub use fallback::FallbackOps;

use std::arch::x86_64::*;

/// SIMD capability detection
pub struct SimdCapabilities {
    pub has_avx2: bool,
    pub has_sse42: bool,
    pub has_avx512: bool,
    pub has_neon: bool,
}

impl SimdCapabilities {
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                has_avx2: is_x86_feature_detected!("avx2"),
                has_sse42: is_x86_feature_detected!("sse4.2"),
                has_avx512: is_x86_feature_detected!("avx512f"),
                has_neon: false,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            Self {
                has_avx2: false,
                has_sse42: false,
                has_avx512: false,
                has_neon: true,
            }
        }

        #[cfg(target_arch = "arm")]
        {
            Self {
                has_avx2: false,
                has_sse42: false,
                has_avx512: false,
                has_neon: true,
            }
        }

        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "arm"
        )))]
        {
            Self {
                has_avx2: false,
                has_sse42: false,
                has_avx512: false,
                has_neon: false,
            }
        }
    }
}

impl Default for SimdCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

/// SIMD optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdLevel {
    None,
    Sse42,
    Avx2,
    Avx512,
    Neon,
}

impl SimdLevel {
    pub fn highest_available() -> Self {
        let caps = SimdCapabilities::detect();

        #[cfg(target_arch = "x86_64")]
        {
            if caps.has_avx512 {
                return SimdLevel::Avx512;
            }
            if caps.has_avx2 {
                return SimdLevel::Avx2;
            }
            if caps.has_sse42 {
                return SimdLevel::Sse42;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if caps.has_neon {
                return SimdLevel::Neon;
            }
        }

        #[cfg(target_arch = "arm")]
        {
            if caps.has_neon {
                return SimdLevel::Neon;
            }
        }

        SimdLevel::None
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SimdLevel::None => "None",
            SimdLevel::Sse42 => "SSE4.2",
            SimdLevel::Avx2 => "AVX2",
            SimdLevel::Avx512 => "AVX-512",
            SimdLevel::Neon => "ARM NEON",
        }
    }
}

/// SIMD-optimized vector operations
pub trait SimdOps {
    fn add(&self, a: &[f32], b: &[f32], result: &mut [f32]);
    fn subtract(&self, a: &[f32], b: &[f32], result: &mut [f32]);
    fn multiply(&self, a: &[f32], b: &[f32], result: &mut [f32]);
    fn divide(&self, a: &[f32], b: &[f32], result: &mut [f32]);
    fn scale(&self, a: &[f32], scale: f32, result: &mut [f32]);
    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32;
    fn sum(&self, a: &[f32]) -> f32;
    fn mean(&self, a: &[f32]) -> f32;
    fn sqrt(&self, a: &[f32], result: &mut [f32]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_capabilities_detect() {
        let caps = SimdCapabilities::detect();
        println!("AVX2: {}", caps.has_avx2);
        println!("SSE4.2: {}", caps.has_sse42);
        println!("AVX512: {}", caps.has_avx512);
        println!("NEON: {}", caps.has_neon);
    }

    #[test]
    fn test_simd_level_highest() {
        let level = SimdLevel::highest_available();
        println!("Highest SIMD level: {}", level.as_str());
    }

    #[test]
    fn test_simd_level_as_str() {
        assert_eq!(SimdLevel::None.as_str(), "None");
        assert_eq!(SimdLevel::Sse42.as_str(), "SSE4.2");
        assert_eq!(SimdLevel::Avx2.as_str(), "AVX2");
        assert_eq!(SimdLevel::Avx512.as_str(), "AVX-512");
        assert_eq!(SimdLevel::Neon.as_str(), "ARM NEON");
    }
}