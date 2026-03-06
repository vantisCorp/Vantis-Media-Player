// ARM NEON SIMD Optimizations (128-bit vectors)
// Inspired by VideoLAN's dav1d ARM NEON optimizations

use super::SimdOps;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(target_arch = "arm")]
use std::arch::arm::*;

/// NEON SIMD operations (128-bit, 4 floats per vector)
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub struct NeonOps;

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
impl NeonOps {
    pub fn new() -> Self {
        NeonOps
    }

    #[inline]
    unsafe fn add_neon(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        // Process 4 floats at a time (128-bit NEON)
        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const float32x4_t;
            let b_ptr = b.as_ptr().add(i * 4) as *const float32x4_t;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut float32x4_t;

            let a_vec = vld1q_f32(a_ptr);
            let b_vec = vld1q_f32(b_ptr);
            let result_vec = vaddq_f32(a_vec, b_vec);
            vst1q_f32(result_ptr, result_vec);
        }

        // Process remaining elements
        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] + b[idx];
        }
    }

    #[inline]
    unsafe fn multiply_neon(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const float32x4_t;
            let b_ptr = b.as_ptr().add(i * 4) as *const float32x4_t;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut float32x4_t;

            let a_vec = vld1q_f32(a_ptr);
            let b_vec = vld1q_f32(b_ptr);
            let result_vec = vmulq_f32(a_vec, b_vec);
            vst1q_f32(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] * b[idx];
        }
    }

    #[inline]
    unsafe fn scale_neon(a: &[f32], scale: f32, result: &mut [f32]) {
        let len = a.len().min(result.len());
        let chunks = len / 4;
        let remainder = len % 4;

        let scale_vec = vdupq_n_f32(scale);

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const float32x4_t;
            let result_ptr = result.as_mut_ptr().add(i * 4) as *mut float32x4_t;

            let a_vec = vld1q_f32(a_ptr);
            let result_vec = vmulq_f32(a_vec, scale_vec);
            vst1q_f32(result_ptr, result_vec);
        }

        for i in 0..remainder {
            let idx = chunks * 4 + i;
            result[idx] = a[idx] * scale;
        }
    }

    #[inline]
    unsafe fn sum_neon(a: &[f32]) -> f32 {
        let len = a.len();
        let chunks = len / 4;
        let remainder = len % 4;

        let mut sum = vdupq_n_f32(0.0);

        for i in 0..chunks {
            let a_ptr = a.as_ptr().add(i * 4) as *const float32x4_t;
            let a_vec = vld1q_f32(a_ptr);
            sum = vaddq_f32(sum, a_vec);
        }

        // Extract and sum elements
        let mut final_sum = 0.0;
        let arr: [f32; 4] = std::mem::transmute(sum);
        final_sum += arr[0] + arr[1] + arr[2] + arr[3];

        // Process remaining elements
        for i in 0..remainder {
            let idx = chunks * 4 + i;
            final_sum += a[idx];
        }

        final_sum
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
impl SimdOps for NeonOps {
    fn add(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        unsafe { Self::add_neon(a, b, result) }
    }

    fn subtract(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] - b[i];
        }
    }

    fn multiply(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        unsafe { Self::multiply_neon(a, b, result) }
    }

    fn divide(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] / b[i];
        }
    }

    fn scale(&self, a: &[f32], scale: f32, result: &mut [f32]) {
        unsafe { Self::scale_neon(a, scale, result) }
    }

    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len());
        let mut sum = 0.0;
        for i in 0..len {
            sum += a[i] * b[i];
        }
        sum
    }

    fn sum(&self, a: &[f32]) -> f32 {
        unsafe { Self::sum_neon(a) }
    }

    fn mean(&self, a: &[f32]) -> f32 {
        if a.is_empty() {
            return 0.0;
        }
        self.sum(a) / a.len() as f32
    }

    fn sqrt(&self, a: &[f32], result: &mut [f32]) {
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
    fn test_neon_add() {
        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = NeonOps::new();
        ops.add(&a, &b, &mut result);

        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_neon_multiply() {
        let a = vec![2.0f32; 16];
        let b = vec![3.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = NeonOps::new();
        ops.multiply(&a, &b, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_neon_scale() {
        let a = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = NeonOps::new();
        ops.scale(&a, 3.0, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_neon_sum() {
        let a = vec![1.0f32; 16];

        let ops = NeonOps::new();
        let sum = ops.sum(&a);

        assert_eq!(sum, 16.0);
    }
}