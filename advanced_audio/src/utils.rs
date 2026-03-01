//! Utility functions for advanced audio processing

use anyhow::Result;
use tracing::{debug, warn};

/// Convert dB to linear scale
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Convert linear scale to dB
pub fn linear_to_db(linear: f32) -> f32 {
    20.0 * linear.log10().max(-100.0)
}

/// Apply gain to samples
pub fn apply_gain(samples: &[f32], gain_db: f32) -> Vec<f32> {
    let gain_linear = db_to_linear(gain_db);
    samples.iter().map(|&s| s * gain_linear).collect()
}

/// Apply fade in to samples
pub fn apply_fade_in(samples: &[f32], duration_samples: usize) -> Vec<f32> {
    let mut output = samples.to_vec();
    let fade_len = duration_samples.min(samples.len());
    
    for i in 0..fade_len {
        let fade_factor = i as f32 / fade_len as f32;
        output[i] *= fade_factor;
    }
    
    output
}

/// Apply fade out to samples
pub fn apply_fade_out(samples: &[f32], duration_samples: usize) -> Vec<f32> {
    let mut output = samples.to_vec();
    let fade_len = duration_samples.min(samples.len());
    let start = samples.len().saturating_sub(fade_len);
    
    for i in start..samples.len() {
        let fade_factor = (samples.len() - i) as f32 / fade_len as f32;
        output[i] *= fade_factor;
    }
    
    output
}

/// Crossfade two audio buffers
pub fn crossfade(buffer1: &[f32], buffer2: &[f32], crossfade_samples: usize) -> Vec<f32> {
    let output_len = buffer1.len() + buffer2.len() - crossfade_samples;
    let mut output = vec![0.0; output_len];
    
    // Copy first buffer
    output[..buffer1.len()].copy_from_slice(buffer1);
    
    // Crossfade region
    let crossfade_start = buffer1.len().saturating_sub(crossfade_samples);
    for i in 0..crossfade_samples.min(buffer1.len()).min(buffer2.len()) {
        let idx1 = crossfade_start + i;
        let idx2 = i;
        let fade_out = 1.0 - (i as f32 / crossfade_samples as f32);
        let fade_in = i as f32 / crossfade_samples as f32;
        output[idx1] = buffer1[idx1] * fade_out + buffer2[idx2] * fade_in;
    }
    
    // Copy second buffer after crossfade
    let copy_start = crossfade_samples.min(buffer2.len());
    let output_start = buffer1.len();
    if copy_start < buffer2.len() {
        output[output_start..].copy_from_slice(&buffer2[copy_start..]);
    }
    
    output
}

/// Normalize audio to target level
pub fn normalize(samples: &[f32], target_level_db: f32) -> Vec<f32> {
    let max_sample = samples.iter().map(|&s| s.abs()).fold(0.0_f32, f32::max);
    
    if max_sample == 0.0 {
        return samples.to_vec();
    }
    
    let current_level_db = linear_to_db(max_sample);
    let gain_db = target_level_db - current_level_db;
    
    apply_gain(samples, gain_db)
}

/// Calculate RMS level
pub fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    
    let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

/// Calculate peak level
pub fn calculate_peak(samples: &[f32]) -> f32 {
    samples.iter().map(|&s| s.abs()).fold(0.0_f32, f32::max)
}

/// Calculate dynamic range
pub fn calculate_dynamic_range(samples: &[f32]) -> f32 {
    let peak = calculate_peak(samples);
    let rms = calculate_rms(samples);
    
    if rms == 0.0 {
        return 0.0;
    }
    
    linear_to_db(peak / rms)
}

/// Apply simple low-pass filter
pub fn low_pass_filter(samples: &[f32], cutoff_freq: f32, sample_rate: f32) -> Vec<f32> {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);
    
    let mut output = vec![0.0; samples.len()];
    output[0] = samples[0];
    
    for i in 1..samples.len() {
        output[i] = alpha * samples[i] + (1.0 - alpha) * output[i - 1];
    }
    
    output
}

/// Apply simple high-pass filter
pub fn high_pass_filter(samples: &[f32], cutoff_freq: f32, sample_rate: f32) -> Vec<f32> {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = rc / (rc + dt);
    
    let mut output = vec![0.0; samples.len()];
    output[0] = samples[0];
    
    for i in 1..samples.len() {
        output[i] = alpha * (output[i - 1] + samples[i] - samples[i - 1]);
    }
    
    output
}

/// Resample audio to new sample rate
pub fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
    if from_rate == to_rate {
        return Ok(samples.to_vec());
    }
    
    let ratio = to_rate as f64 / from_rate as f64;
    let output_len = (samples.len() as f64 * ratio) as usize;
    
    let mut output = vec![0.0; output_len];
    
    // Linear interpolation
    for i in 0..output_len {
        let src_pos = i as f64 / ratio;
        let src_idx = src_pos.floor() as usize;
        let frac = src_pos - src_pos.floor() as f64;
        
        if src_idx + 1 < samples.len() {
            output[i] = (samples[src_idx] * (1.0 - frac) + samples[src_idx + 1] * frac) as f32;
        } else {
            output[i] = samples[src_idx.min(samples.len() - 1)];
        }
    }
    
    Ok(output)
}

/// Mix multiple audio buffers
pub fn mix_buffers(buffers: &[Vec<f32>]) -> Vec<f32> {
    if buffers.is_empty() {
        return Vec::new();
    }
    
    let max_len = buffers.iter().map(|b| b.len()).max().unwrap_or(0);
    let mut output = vec![0.0; max_len];
    
    for buffer in buffers {
        for (i, &sample) in buffer.iter().enumerate() {
            output[i] += sample;
        }
    }
    
    // Normalize to prevent clipping
    let max_val = output.iter().map(|&s| s.abs()).fold(0.0_f32, f32::max);
    if max_val > 1.0 {
        for sample in output.iter_mut() {
            *sample /= max_val;
        }
    }
    
    output
}

/// Split stereo buffer into left and right channels
pub fn split_stereo(samples: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut left = Vec::with_capacity(samples.len() / 2);
    let mut right = Vec::with_capacity(samples.len() / 2);
    
    for chunk in samples.chunks(2) {
        left.push(chunk[0]);
        right.push(chunk.get(1).copied().unwrap_or(0.0));
    }
    
    (left, right)
}

/// Combine left and right channels into stereo buffer
pub fn combine_stereo(left: &[f32], right: &[f32]) -> Vec<f32> {
    let mut output = Vec::with_capacity(left.len() * 2);
    
    for i in 0..left.len().max(right.len()) {
        output.push(left.get(i).copied().unwrap_or(0.0));
        output.push(right.get(i).copied().unwrap_or(0.0));
    }
    
    output
}

/// Calculate correlation between two buffers
pub fn calculate_correlation(buffer1: &[f32], buffer2: &[f32]) -> f32 {
    let len = buffer1.len().min(buffer2.len());
    if len == 0 {
        return 0.0;
    }
    
    let mean1: f32 = buffer1[..len].iter().sum::<f32>() / len as f32;
    let mean2: f32 = buffer2[..len].iter().sum::<f32>() / len as f32;
    
    let mut numerator = 0.0;
    let mut denominator1 = 0.0;
    let mut denominator2 = 0.0;
    
    for i in 0..len {
        let diff1 = buffer1[i] - mean1;
        let diff2 = buffer2[i] - mean2;
        numerator += diff1 * diff2;
        denominator1 += diff1 * diff1;
        denominator2 += diff2 * diff2;
    }
    
    let denominator = (denominator1 * denominator2).sqrt();
    if denominator == 0.0 {
        return 0.0;
    }
    
    numerator / denominator
}

/// Detect silence in audio
pub fn detect_silence(samples: &[f32], threshold_db: f32, min_duration_samples: usize) -> Vec<(usize, usize)> {
    let threshold = db_to_linear(threshold_db);
    let mut silence_regions = Vec::new();
    let mut in_silence = false;
    let mut start = 0;
    
    for (i, &sample) in samples.iter().enumerate() {
        let is_silent = sample.abs() < threshold;
        
        if is_silent && !in_silence {
            start = i;
            in_silence = true;
        } else if !is_silent && in_silence {
            let duration = i - start;
            if duration >= min_duration_samples {
                silence_regions.push((start, i));
            }
            in_silence = false;
        }
    }
    
    // Handle trailing silence
    if in_silence {
        let duration = samples.len() - start;
        if duration >= min_duration_samples {
            silence_regions.push((start, samples.len()));
        }
    }
    
    silence_regions
}

/// Remove silence from audio
pub fn remove_silence(samples: &[f32], threshold_db: f32, min_duration_samples: usize) -> Vec<f32> {
    let silence_regions = detect_silence(samples, threshold_db, min_duration_samples);
    
    if silence_regions.is_empty() {
        return samples.to_vec();
    }
    
    let mut output = Vec::new();
    let mut last_end = 0;
    
    for (start, end) in silence_regions {
        output.extend_from_slice(&samples[last_end..start]);
        last_end = end;
    }
    
    output.extend_from_slice(&samples[last_end..]);
    output
}

/// Clamp samples to prevent clipping
pub fn clamp_samples(samples: &[f32], min: f32, max: f32) -> Vec<f32> {
    samples.iter().map(|&s| s.clamp(min, max)).collect()
}

/// Apply soft clipping
pub fn soft_clip(samples: &[f32]) -> Vec<f32> {
    samples.iter().map(|&s| {
        if s > 1.0 {
            1.0 - (2.0 / (1.0 + (s - 1.0).exp()))
        } else if s < -1.0 {
            -1.0 + (2.0 / (1.0 + (-s - 1.0).exp()))
        } else {
            s
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_db_conversion() {
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.001);
        assert!((db_to_linear(-6.0) - 0.501).abs() < 0.01);
        assert!((linear_to_db(1.0) - 0.0).abs() < 0.001);
        assert!((linear_to_db(0.5) - (-6.02)).abs() < 0.1);
    }
    
    #[test]
    fn test_gain() {
        let samples = vec![0.5, 0.5, 0.5];
        let gained = apply_gain(&samples, 6.0);
        assert!((gained[0] - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_fade_in() {
        let samples = vec![1.0, 1.0, 1.0, 1.0];
        let faded = apply_fade_in(&samples, 2);
        assert!(faded[0] < faded[1]);
        assert!((faded[2] - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_rms() {
        let samples = vec![1.0, -1.0, 1.0, -1.0];
        let rms = calculate_rms(&samples);
        assert!((rms - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_peak() {
        let samples = vec![0.5, 0.8, 0.3, 0.9];
        let peak = calculate_peak(&samples);
        assert!((peak - 0.9).abs() < 0.01);
    }
    
    #[test]
    fn test_normalize() {
        let samples = vec![0.5, 0.5, 0.5];
        let normalized = normalize(&samples, 0.0);
        assert!((normalized[0] - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_split_combine_stereo() {
        let stereo = vec![0.5, 0.3, 0.7, 0.9];
        let (left, right) = split_stereo(&stereo);
        assert_eq!(left, vec![0.5, 0.7]);
        assert_eq!(right, vec![0.3, 0.9]);
        
        let combined = combine_stereo(&left, &right);
        assert_eq!(combined, stereo);
    }
    
    #[test]
    fn test_silence_detection() {
        let samples = vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0];
        let silence = detect_silence(&samples, -60.0, 2);
        assert_eq!(silence.len(), 2);
        assert_eq!(silence[0], (0, 3));
        assert_eq!(silence[1], (5, 7));
    }
}