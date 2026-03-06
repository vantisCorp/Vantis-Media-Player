// AVX2 SIMD Optimizations (256-bit vectors)
// Inspired by VideoLAN's dav1d AVX2 optimizations

use super::SimdOps;
use std::arch::x86_64::*;

/// AVX2 SIMD operations (256-bit, 8 floats per vector)
pub struct Avx2Ops;

impl Avx2Ops {
    pub fn new() -> Self {
        Avx2Ops
    }

    #[inline]
    unsafe fn add_avx2(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 8;
        let remainder = len % 8;

        // Process 8 floats at a time (256-bit AVX2)
        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 8) as *const __m256;
            let b_ptr = b.as_ptr().add(i * 8) as *const __m256;
            let result_ptr = result.as_mut_ptr().add(i * 8) as *mut __m256;

            let a_vec = _mm256_loadu_ps(a_ptr);
            let b_vec = _mm256_loadu_ps(b_ptr);
            let result_vec = _mm256_add_ps(a_vec, b_vec);
            _mm256_storeu_ps(result_ptr, result_vec);
        }

        // Process remaining elements
        for i in 0..remainder {
            let idx = chunks * 8 + i;
            result[idx] = a[idx] + b[idx];
        }
    }

    #[inline]
    unsafe fn multiply_avx2(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 8;
        let remainder = len % 8;

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 8) as *const __m256;
            let b_ptr = b.as_ptr().add(i * 8) as *const __m256;
            let result_ptr = result.as_mut_ptr().add(i * 8) as *mut __m256;

            let a_vec = _mm256_loadu_ps(a_ptr);
            let b_vec = _mm256_loadu_ps(b_ptr);
            let result_vec = _mm256_mul_ps(a_vec, b_vec);
            _mm256_storeu_ps(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 8 + i;
            result[idx] = a[idx] * b[idx];
        }
    }

    #[inline]
    unsafe fn scale_avx2(a: &[f32], scale: f32, result: &mut [f32]) {
        let len = a.len().min(result.len());
        let chunks = len / 8;
        let remainder = len % 8;

        let scale_vec = _mm256_set1_ps(scale);

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 8) as *const __m256;
            let result_ptr = result.as_mut_ptr().add(i * 8) as *mut __m256;

            let a_vec = _mm256_loadu_ps(a_ptr);
            let result_vec = _mm256_mul_ps(a_vec, scale_vec);
            _mm256_storeu_ps(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 8 + i;
            result[idx] = a[idx] * scale;
        }
    }

    #[inline]
    unsafe fn sum_avx2(a: &[f32]) -> f32 {
        let len = a.len();
        let chunks = len / 8;
        let remainder = len % 8;

        let mut sum = _mm256_setzero_ps();

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 8) as *const __m256;
            let a_vec = _mm256_loadu_ps(a_ptr);
            sum = _mm256_add_ps(sum, a_vec);
        }

        // Horizontal sum
        sum = _mm256_hadd_ps(sum, sum);
        sum = _mm256_hadd_ps(sum, sum);
        let result = _mm256_extractf128_ps(sum, 0);
        let mut final_sum = _mm_cvtss_f32(result);

        // Process remaining elements
        for i in 0..remainder {
            let idx = chunks * 8 + i;
            final_sum += a[idx];
        }

        final_sum
    }
}

impl SimdOps for Avx2 {
    fn add(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        unsafe { Self::add_avx2(a, b, result) }
    }

    fn subtract(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] - b[i];
        }
    }

    fn multiply(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        unsafe { Self::multiply_avx2(a, b, result) }
    }

    fn divide(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] / b[i];
        }
    }

    fn scale(&self, a: &[f32], scale: f32, result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        unsafe { Self::scale_avx2(a, scale, result) }
    }

    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32 {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        let len = a.len().min(b.len());
        let mut sum = 0.0;
        for i in 0..len {
            sum += a[i] * b[i];
        }
        sum
    }

    fn sum(&self, a: &[f32]) -> f32 {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        unsafe { Self::sum_avx2(a) }
    }

    fn mean(&self, a: &[f32]) -> f32 {
        if a.is_empty() {
            return 0.0;
        }
        self.sum(a) / a.len() as f32
    }

    fn sqrt(&self, a: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("avx2"), "AVX2 not supported");
        let len = a.len().min(result.len());
        for i in 0..len {
            result[i] = a[i].sqrt();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx2_add() {
        if !is_x86_feature_detected!("avx2") {
            return; // Skip test if AVX2 not supported
        }

        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Avx2Ops::new();
        ops.add(&a, &b, &mut result);

        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_avx2_multiply() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }

        let a = vec![2.0f32; 16];
        let b = vec![3.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Avx2Ops::new();
        ops.multiply(&a, &b, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_avx2_scale() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }

        let a = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Avx2Ops::new();
        ops.scale(&a, 3.0, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_avx2_sum() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }

        let a = vec![1.0f32; 16];

        let ops = Avx2Ops::new();
        let sum = ops.sum(&a);

        assert_eq!(sum, 16.0);
    }
}