// SSE4.2 SIMD Optimizations (128-bit vectors)
// Legacy SIMD support for older x86_64 CPUs

use super::SimdOps;
use std::arch::x86_64::*;

/// SSE4.2 SIMD operations (128-bit, 4 floats per vector)
pub struct Sse42Ops;

impl Sse42Ops {
    pub fn new() -> Self {
        Sse42Ops
    }

    #[inline]
    unsafe fn add_sse42(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const __m128;
            let b_ptr = b.as_ptr().add(i * 4) as *const __m128;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut __m128;

            let a_vec = _mm_loadu_ps(a_ptr);
            let b_vec = _mm_loadu_ps(b_ptr);
            let result_vec = _mm_add_ps(a_vec, b_vec);
            _mm_storeu_ps(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] + b[idx];
        }
    }

    #[inline]
    unsafe fn multiply_sse42(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const __m128;
            let b_ptr = b.as_ptr().add(i * 4) as *const __m128;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut __m128;

            let a_vec = _mm_loadu_ps(a_ptr);
            let b_vec = _mm_loadu_ps(b_ptr);
            let result_vec = _mm_mul_ps(a_vec, b_vec);
            _mm_storeu_ps(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] * b[idx];
        }
    }

    #[inline]
    unsafe fn scale_sse42(a: &[f32], scale: f32, result: &mut [f32]) {
        let len = a.len().min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        let scale_vec = _mm_set1_ps(scale);

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const __m128;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut __m128;

            let a_vec = _mm_loadu_ps(a_ptr);
            let result_vec = _mm_mul_ps(a_vec, scale_vec);
            _mm_storeu_ps(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] * scale;
        }
    }

    #[inline]
    unsafe fn sum_sse42(a: &[f32]) -> f32 {
        let len = a.len();
        let chunks = len / 4;
        let remainder = len % 4;

        let mut sum = _mm_setzero_ps();

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const __m128;
            let a_vec = _mm_loadu_ps(a_ptr);
            sum = _mm_add_ps(sum, a_vec);
        }

        sum = _mm_hadd_ps(sum, sum);
        let high = _mm_movehl_ps(sum, sum);
        sum = _mm_add_ss(sum, high);
        let mut final_sum = _mm_cvtss_f32(sum);

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            final_sum += a[idx];
        }

        final_sum
    }
}

impl SimdOps for Sse42Ops {
    fn add(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        unsafe { Self::add_sse42(a, b, result) }
    }

    fn subtract(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] - b[i];
        }
    }

    fn multiply(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        unsafe { Self::multiply_sse42(a, b, result) }
    }

    fn divide(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] / b[i];
        }
    }

    fn scale(&self, a: &[f32], scale: f32, result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        unsafe { Self::scale_sse42(a, scale, result) }
    }

    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32 {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        let len = a.len().min(b.len());
        let mut sum = 0.0;
        for i in 0..len {
            sum += a[i] * b[i];
        }
        sum
    }

    fn sum(&self, a: &[f32]) -> f32 {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
        unsafe { Self::sum_sse42(a) }
    }

    fn mean(&self, a: &[f32]) -> f32 {
        if a.is_empty() {
            return 0.0;
        }
        self.sum(a) / a.len() as f32
    }

    fn sqrt(&self, a: &[f32], result: &mut [f32]) {
        assert!(is_x86_feature_detected!("sse4.2"), "SSE4.2 not supported");
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
    fn test_sse42_add() {
        if !is_x86_feature_detected!("sse4.2") {
            return;
        }

        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Sse42Ops::new();
        ops.add(&a, &b, &mut result);

        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_sse42_multiply() {
        if !is_x86_feature_detected!("sse4.2") {
            return;
        }

        let a = vec![2.0f32; 16];
        let b = vec![3.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Sse42Ops::new();
        ops.multiply(&a, &b, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_sse42_scale() {
        if !is_x86_feature_detected!("sse4.2") {
            return;
        }

        let a = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = Sse42Ops::new();
        ops.scale(&a, 3.0, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_sse42_sum() {
        if !is_x86_feature_detected!("sse4.2") {
            return;
        }

        let a = vec![1.0f32; 16];

        let ops = Sse42Ops::new();
        let sum = ops.sum(&a);

        assert_eq!(sum, 16.0);
    }
}