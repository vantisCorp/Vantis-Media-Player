// Fallback (non-SIMD) implementation for platforms without SIMD support
use super::SimdOps;

/// Fallback operations (scalar, no SIMD)
pub struct FallbackOps;

impl FallbackOps {
    pub fn new() -> Self {
        FallbackOps
    }
}

impl SimdOps for FallbackOps {
    fn add(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] + b[i];
        }
    }

    fn subtract(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] - b[i];
        }
    }

    fn multiply(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] * b[i];
        }
    }

    fn divide(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        for i in 0..len {
            result[i] = a[i] / b[i];
        }
    }

    fn scale(&self, a: &[f32], scale: f32, result: &mut [f32]) {
        let len = a.len().min(result.len());
        for i in 0..len {
            result[i] = a[i] * scale;
        }
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
        let mut sum = 0.0;
        for i in a {
            sum += i;
        }
        sum
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
    fn test_fallback_add() {
        let a = vec![1.0f32; 16];
        let b = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = FallbackOps::new();
        ops.add(&a, &b, &mut result);

        assert_eq!(result, vec![3.0f32; 16]);
    }

    #[test]
    fn test_fallback_multiply() {
        let a = vec![2.0f32; 16];
        let b = vec![3.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = FallbackOps::new();
        ops.multiply(&a, &b, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_fallback_scale() {
        let a = vec![2.0f32; 16];
        let mut result = vec![0.0f32; 16];

        let ops = FallbackOps::new();
        ops.scale(&a, 3.0, &mut result);

        assert_eq!(result, vec![6.0f32; 16]);
    }

    #[test]
    fn test_fallback_sum() {
        let a = vec![1.0f32; 16];

        let ops = FallbackOps::new();
        let sum = ops.sum(&a);

        assert_eq!(sum, 16.0);
    }
}