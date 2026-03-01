//! Utility functions for AI operations
//! 
//! Provides helper functions including:
//! - Tensor operations
//! - Feature extraction
//! - Data preprocessing
//! - Post-processing utilities

use crate::{AIError, AIResult};
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb};
use ndarray::{Array4, Array3, Array2, Array1};
use serde::{Deserialize, Serialize};

/// Tensor operations utility
pub struct TensorOps;

impl TensorOps {
    /// Normalize tensor to [0, 1]
    pub fn normalize(tensor: &mut [f32]) {
        let min = tensor.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = tensor.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        if max > min {
            let range = max - min;
            for val in tensor.iter_mut() {
                *val = (*val - min) / range;
            }
        }
    }
    
    /// Denormalize tensor from [0, 1] to original range
    pub fn denormalize(tensor: &mut [f32], min: f32, max: f32) {
        let range = max - min;
        for val in tensor.iter_mut() {
            *val = *val * range + min;
        }
    }
    
    /// Apply mean and std normalization
    pub fn normalize_mean_std(tensor: &mut [f32], mean: &[f32], std: &[f32]) {
        for (i, val) in tensor.iter_mut().enumerate() {
            let m = mean.get(i % mean.len()).copied().unwrap_or(0.0);
            let s = std.get(i % std.len()).copied().unwrap_or(1.0);
            *val = (*val - m) / s;
        }
    }
    
    /// Reshape flat tensor to 4D (N, C, H, W)
    pub fn reshape_4d(tensor: &[f32], shape: [usize; 4]) -> AIResult<Array4<f32>> {
        let total = shape[0] * shape[1] * shape[2] * shape[3];
        if tensor.len() != total {
            return Err(AIError::InvalidInput(format!(
                "Tensor size mismatch: expected {}, got {}",
                total,
                tensor.len()
            )));
        }
        
        Ok(Array4::from_shape_vec(shape, tensor.to_vec())?)
    }
    
    /// Reshape flat tensor to 3D (C, H, W)
    pub fn reshape_3d(tensor: &[f32], shape: [usize; 3]) -> AIResult<Array3<f32>> {
        let total = shape[0] * shape[1] * shape[2];
        if tensor.len() != total {
            return Err(AIError::InvalidInput(format!(
                "Tensor size mismatch: expected {}, got {}",
                total,
                tensor.len()
            )));
        }
        
        Ok(Array3::from_shape_vec(shape, tensor.to_vec())?)
    }
    
    /// Reshape flat tensor to 2D (H, W)
    pub fn reshape_2d(tensor: &[f32], shape: [usize; 2]) -> AIResult<Array2<f32>> {
        let total = shape[0] * shape[1];
        if tensor.len() != total {
            return Err(AIError::InvalidInput(format!(
                "Tensor size mismatch: expected {}, got {}",
                total,
                tensor.len()
            )));
        }
        
        Ok(Array2::from_shape_vec(shape, tensor.to_vec())?)
    }
    
    /// Flatten tensor to 1D
    pub fn flatten<T, D>(tensor: &ndarray::ArrayBase<T, D>) -> Vec<f32>
    where
        T: ndarray::Data<Elem = f32>,
        D: ndarray::Dimension,
    {
        tensor.iter().cloned().collect()
    }
    
    /// Pad tensor to target size
    pub fn pad_tensor(tensor: &mut Vec<f32>, current_size: usize, target_size: usize, pad_value: f32) {
        if current_size < target_size {
            tensor.extend(std::iter::repeat(pad_value).take(target_size - current_size));
        }
    }
    
    /// Crop tensor to target size
    pub fn crop_tensor(tensor: &[f32], target_size: usize) -> Vec<f32> {
        tensor.iter().take(target_size).cloned().collect()
    }
    
    /// Calculate cosine similarity between two tensors
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        
        let mut dot_product = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;
        
        for (x, y) in a.iter().zip(b.iter()) {
            dot_product += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }
        
        let norm_a = norm_a.sqrt();
        let norm_b = norm_b.sqrt();
        
        if norm_a > 0.0 && norm_b > 0.0 {
            dot_product / (norm_a * norm_b)
        } else {
            0.0
        }
    }
    
    /// Calculate Euclidean distance between two tensors
    pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return f32::INFINITY;
        }
        
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
    
    /// Apply softmax to tensor
    pub fn softmax(tensor: &mut [f32]) {
        let max = tensor.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = tensor.iter().map(|x| (x - max).exp()).sum();
        
        for val in tensor.iter_mut() {
            *val = (*val - max).exp() / exp_sum;
        }
    }
    
    /// Apply sigmoid to tensor
    pub fn sigmoid(tensor: &mut [f32]) {
        for val in tensor.iter_mut() {
            *val = 1.0 / (1.0 + (-*val).exp());
        }
    }
    
    /// Apply ReLU activation
    pub fn relu(tensor: &mut [f32]) {
        for val in tensor.iter_mut() {
            *val = val.max(0.0);
        }
    }
    
    /// Apply tanh activation
    pub fn tanh(tensor: &mut [f32]) {
        for val in tensor.iter_mut() {
            *val = val.tanh();
        }
    }
}

/// Feature extractor utility
pub struct FeatureExtractor {
    config: crate::AIConfig,
}

impl FeatureExtractor {
    /// Create a new feature extractor
    pub fn new(config: crate::AIConfig) -> AIResult<Self> {
        Ok(Self { config })
    }
    
    /// Convert image to tensor (NCHW format)
    pub fn image_to_tensor(&self, image: &DynamicImage) -> AIResult<Vec<f32>> {
        let rgb = image.to_rgb8();
        let (width, height) = rgb.dimensions();
        
        // Normalize to [0, 1]
        let mut tensor = Vec::with_capacity(3 * width as usize * height as usize);
        
        for c in 0..3 {
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb.get_pixel(x, y);
                    let value = pixel[c] as f32 / 255.0;
                    tensor.push(value);
                }
            }
        }
        
        Ok(tensor)
    }
    
    /// Convert tensor to image
    pub fn tensor_to_image(&self, tensor: &[f32]) -> AIResult<DynamicImage> {
        // Assume tensor is in CHW format
        let channels = 3;
        let total_pixels = tensor.len() / channels;
        let height = (total_pixels as f32).sqrt() as u32;
        let width = total_pixels as u32 / height;
        
        let mut img = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let r = (tensor[idx].clamp(0.0, 1.0) * 255.0) as u8;
                let g = (tensor[idx + total_pixels].clamp(0.0, 1.0) * 255.0) as u8;
                let b = (tensor[idx + 2 * total_pixels].clamp(0.0, 1.0) * 255.0) as u8;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        Ok(DynamicImage::ImageRgb8(img))
    }
    
    /// Extract color histogram
    pub fn extract_color_histogram(&self, image: &DynamicImage, bins: usize) -> AIResult<Vec<f32>> {
        let rgb = image.to_rgb8();
        let mut histogram = vec![0.0; bins * 3];
        
        for pixel in rgb.pixels() {
            for c in 0..3 {
                let bin = (pixel[c] as usize * bins) / 256;
                histogram[c * bins + bin.min(bins - 1)] += 1.0;
            }
        }
        
        // Normalize
        let total = rgb.pixels().count() as f32;
        for val in histogram.iter_mut() {
            *val /= total;
        }
        
        Ok(histogram)
    }
    
    /// Extract texture features (GLCM)
    pub fn extract_texture_features(&self, image: &DynamicImage) -> AIResult<Vec<f32>> {
        let gray = image.to_luma8();
        let (width, height) = gray.dimensions();
        
        // Calculate GLCM (Gray-Level Co-occurrence Matrix)
        let levels = 16;
        let mut glcm = vec![0.0; levels * levels];
        
        for y in 0..height.saturating_sub(1) {
            for x in 0..width.saturating_sub(1) {
                let p1 = (gray.get_pixel(x, y)[0] * levels / 256) as usize;
                let p2 = (gray.get_pixel(x + 1, y)[0] * levels / 256) as usize;
                glcm[p1 * levels + p2] += 1.0;
            }
        }
        
        // Normalize
        let total: f32 = glcm.iter().sum();
        for val in glcm.iter_mut() {
            *val /= total;
        }
        
        // Calculate texture features
        let mut features = Vec::new();
        
        // Contrast
        let mut contrast = 0.0;
        for i in 0..levels {
            for j in 0..levels {
                contrast += glcm[i * levels + j] * ((i as f32 - j as f32).powi(2));
            }
        }
        features.push(contrast);
        
        // Homogeneity
        let mut homogeneity = 0.0;
        for i in 0..levels {
            for j in 0..levels {
                homogeneity += glcm[i * levels + j] / (1.0 + (i as f32 - j as f32).abs());
            }
        }
        features.push(homogeneity);
        
        // Energy
        let energy: f32 = glcm.iter().map(|&v| v * v).sum();
        features.push(energy);
        
        Ok(features)
    }
    
    /// Extract edge features
    pub fn extract_edge_features(&self, image: &DynamicImage) -> AIResult<Vec<f32>> {
        let gray = image.to_luma8();
        let (width, height) = gray.dimensions();
        
        let mut edge_count = 0;
        let mut total = 0;
        
        for y in 1..height.saturating_sub(1) {
            for x in 1..width.saturating_sub(1) {
                let center = gray.get_pixel(x, y)[0] as f32;
                let left = gray.get_pixel(x - 1, y)[0] as f32;
                let right = gray.get_pixel(x + 1, y)[0] as f32;
                let top = gray.get_pixel(x, y - 1)[0] as f32;
                let bottom = gray.get_pixel(x, y + 1)[0] as f32;
                
                let gx = (right - left).abs();
                let gy = (bottom - top).abs();
                let magnitude = (gx * gx + gy * gy).sqrt();
                
                if magnitude > 30.0 {
                    edge_count += 1;
                }
                total += 1;
            }
        }
        
        let edge_density = if total > 0 {
            edge_count as f32 / total as f32
        } else {
            0.0
        };
        
        Ok(vec![edge_density])
    }
    
    /// Extract audio features (MFCC-like)
    pub fn extract_audio_features(&self, samples: &[f32], sample_rate: u32) -> AIResult<Vec<f32>> {
        let mut features = Vec::new();
        
        // Calculate energy
        let energy: f32 = samples.iter().map(|&s| s * s).sum::<f32>() / samples.len() as f32;
        features.push(energy);
        
        // Calculate zero-crossing rate
        let zero_crossings = samples
            .windows(2)
            .filter(|w| w[0] * w[1] < 0.0)
            .count() as f32;
        let zcr = zero_crossings / samples.len() as f32;
        features.push(zcr);
        
        // Calculate spectral centroid (simplified)
        let mut sum_weighted = 0.0;
        let mut sum_weights = 0.0;
        
        for (i, &sample) in samples.iter().enumerate() {
            let frequency = (i as f32 * sample_rate as f32) / samples.len() as f32;
            let magnitude = sample.abs();
            sum_weighted += frequency * magnitude;
            sum_weights += magnitude;
        }
        
        let spectral_centroid = if sum_weights > 0.0 {
            sum_weighted / sum_weights
        } else {
            0.0
        };
        features.push(spectral_centroid);
        
        // Calculate spectral rolloff (simplified)
        let mut cumulative_energy = 0.0;
        let total_energy: f32 = samples.iter().map(|&s| s * s).sum();
        let rolloff_index = samples.iter().enumerate().find(|(_, &s)| {
            cumulative_energy += s * s;
            cumulative_energy >= 0.85 * total_energy
        });
        
        let spectral_rolloff = if let Some((i, _)) = rolloff_index {
            (i as f32 * sample_rate as f32) / samples.len() as f32
        } else {
            sample_rate as f32 / 2.0
        };
        features.push(spectral_rolloff);
        
        Ok(features)
    }
    
    /// Extract temporal features
    pub fn extract_temporal_features(&self, sequence: &[f32]) -> AIResult<Vec<f32>> {
        let mut features = Vec::new();
        
        if sequence.is_empty() {
            return Ok(features);
        }
        
        // Mean
        let mean: f32 = sequence.iter().sum::<f32>() / sequence.len() as f32;
        features.push(mean);
        
        // Variance
        let variance: f32 = sequence
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / sequence.len() as f32;
        features.push(variance);
        
        // Standard deviation
        features.push(variance.sqrt());
        
        // Min and max
        let min = sequence.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = sequence.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        features.push(min);
        features.push(max);
        
        // Range
        features.push(max - min);
        
        // Skewness (simplified)
        let skewness: f32 = sequence
            .iter()
            .map(|&x| (x - mean).powi(3))
            .sum::<f32>() / (sequence.len() as f32 * variance.powf(1.5));
        features.push(skewness);
        
        // Kurtosis (simplified)
        let kurtosis: f32 = sequence
            .iter()
            .map(|&x| (x - mean).powi(4))
            .sum::<f32>() / (sequence.len() as f32 * variance.powi(2));
        features.push(kurtosis);
        
        Ok(features)
    }
    
    /// Preprocess audio for model input
    pub fn preprocess_audio(&self, samples: &[f32], target_length: usize) -> AIResult<Vec<f32>> {
        let mut processed = samples.to_vec();
        
        // Normalize
        let max = processed.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if max > 0.0 {
            for val in processed.iter_mut() {
                *val /= max;
            }
        }
        
        // Pad or crop to target length
        if processed.len() < target_length {
            processed.extend(std::iter::repeat(0.0).take(target_length - processed.len()));
        } else {
            processed.truncate(target_length);
        }
        
        Ok(processed)
    }
    
    /// Preprocess image for model input
    pub fn preprocess_image(&self, image: &DynamicImage, target_size: (u32, u32)) -> AIResult<Vec<f32>> {
        // Resize image
        let resized = image.resize_exact(
            target_size.0,
            target_size.1,
            image::imageops::FilterType::Lanczos3,
        );
        
        // Convert to tensor
        let mut tensor = self.image_to_tensor(&resized)?;
        
        // Normalize with ImageNet mean and std
        let mean = [0.485, 0.456, 0.406];
        let std = [0.229, 0.224, 0.225];
        TensorOps::normalize_mean_std(&mut tensor, &mean, &std);
        
        Ok(tensor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tensor_ops_normalize() {
        let mut tensor = vec![0.0, 0.5, 1.0];
        TensorOps::normalize(&mut tensor);
        assert!((tensor[0] - 0.0).abs() < 0.001);
        assert!((tensor[2] - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = TensorOps::cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_extract_temporal_features() {
        let extractor = FeatureExtractor::new(crate::AIConfig::default()).unwrap();
        let sequence = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let features = extractor.extract_temporal_features(&sequence).unwrap();
        assert!(!features.is_empty());
    }
}