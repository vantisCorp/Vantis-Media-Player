//! Utility functions for advanced video processing

use anyhow::Result;
use image::{RgbImage, Rgb, Pixel};
use tracing::{debug};

/// Convert RGB to grayscale
pub fn rgb_to_grayscale(frame: &RgbImage) -> Vec<f32> {
    frame.pixels()
        .map(|p| {
            let r = p[0] as f32;
            let g = p[1] as f32;
            let b = p[2] as f32;
            // Standard luminance formula
            0.299 * r + 0.587 * g + 0.114 * b
        })
        .collect()
}

/// Convert grayscale to RGB
pub fn grayscale_to_rgb(grayscale: &[f32], width: u32, height: u32) -> RgbImage {
    let mut output = RgbImage::new(width, height);
    
    for (i, &value) in grayscale.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        let pixel_value = value.clamp(0.0, 255.0) as u8;
        output.put_pixel(x, y, Rgb([pixel_value, pixel_value, pixel_value]));
    }
    
    output
}

/// Resize frame using bilinear interpolation
pub fn resize_frame(frame: &RgbImage, new_width: u32, new_height: u32) -> RgbImage {
    let mut output = RgbImage::new(new_width, new_height);
    
    let x_ratio = frame.width() as f32 / new_width as f32;
    let y_ratio = frame.height() as f32 / new_height as f32;
    
    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = (x as f32 * x_ratio).floor() as u32;
            let src_y = (y as f32 * y_ratio).floor() as u32;
            
            let src_x = src_x.min(frame.width() - 1);
            let src_y = src_y.min(frame.height() - 1);
            
            let pixel = frame.get_pixel(src_x, src_y);
            output.put_pixel(x, y, *pixel);
        }
    }
    
    output
}

/// Crop frame to specified rectangle
pub fn crop_frame(frame: &RgbImage, x: u32, y: u32, width: u32, height: u32) -> Result<RgbImage> {
    if x + width > frame.width() || y + height > frame.height() {
        return Err(anyhow::anyhow!("Crop rectangle exceeds frame bounds"));
    }
    
    let mut output = RgbImage::new(width, height);
    
    for dy in 0..height {
        for dx in 0..width {
            let src_x = x + dx;
            let src_y = y + dy;
            let pixel = frame.get_pixel(src_x, src_y);
            output.put_pixel(dx, dy, *pixel);
        }
    }
    
    Ok(output)
}

/// Apply Gaussian blur
pub fn gaussian_blur(frame: &RgbImage, sigma: f32) -> RgbImage {
    let kernel = create_gaussian_kernel(sigma);
    let kernel_size = kernel.len();
    let offset = (kernel_size / 2) as i32;
    
    let mut output = RgbImage::new(frame.width(), frame.height());
    
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut weight_sum = 0.0;
            
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let px = x as i32 + kx as i32 - offset;
                    let py = y as i32 + ky as i32 - offset;
                    
                    if px >= 0 && px < frame.width() as i32 && py >= 0 && py < frame.height() as i32 {
                        let pixel = frame.get_pixel(px as u32, py as u32);
                        let weight = kernel[ky][kx];
                        
                        sum_r += pixel[0] as f32 * weight;
                        sum_g += pixel[1] as f32 * weight;
                        sum_b += pixel[2] as f32 * weight;
                        weight_sum += weight;
                    }
                }
            }
            
            let r = (sum_r / weight_sum).clamp(0.0, 255.0) as u8;
            let g = (sum_g / weight_sum).clamp(0.0, 255.0) as u8;
            let b = (sum_b / weight_sum).clamp(0.0, 255.0) as u8;
            
            output.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    output
}

/// Create Gaussian kernel
fn create_gaussian_kernel(sigma: f32) -> Vec<Vec<f32>> {
    let size = (sigma * 3.0 * 2.0).ceil() as usize;
    let size = size.max(3).min(15); // Clamp kernel size
    let size = if size % 2 == 0 { size + 1 } else { size }; // Make odd
    
    let mut kernel = vec![vec![0.0; size]; size];
    let offset = size / 2;
    let two_sigma_sq = 2.0 * sigma * sigma;
    
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - offset as i32;
            let dy = y as i32 - offset as i32;
            let distance_sq = (dx * dx + dy * dy) as f32;
            kernel[y][x] = (-distance_sq / two_sigma_sq).exp();
        }
    }
    
    // Normalize kernel
    let sum: f32 = kernel.iter().flat_map(|row| row.iter()).sum();
    for row in kernel.iter_mut() {
        for val in row.iter_mut() {
            *val /= sum;
        }
    }
    
    kernel
}

/// Calculate optical flow between two frames (simplified)
pub fn calculate_optical_flow(frame1: &RgbImage, frame2: &RgbImage) -> Vec<(f32, f32)> {
    let gray1 = rgb_to_grayscale(frame1);
    let gray2 = rgb_to_grayscale(frame2);
    
    let width = frame1.width() as usize;
    let height = frame1.height() as usize;
    
    let mut flow = Vec::with_capacity(width * height);
    
    // Simplified block matching
    let block_size = 8;
    let search_radius = 16;
    
    for y in (0..height).step_by(block_size) {
        for x in (0..width).step_by(block_size) {
            let mut best_dx = 0.0;
            let mut best_dy = 0.0;
            let mut best_sad = f32::MAX;
            
            // Search for best match
            for dy in -search_radius..=search_radius {
                for dx in -search_radius..=search_radius {
                    let mut sad = 0.0;
                    let mut count = 0;
                    
                    for by in 0..block_size {
                        for bx in 0..block_size {
                            let src_x = x + bx;
                            let src_y = y + by;
                            let dst_x = src_x as i32 + dx;
                            let dst_y = src_y as i32 + dy;
                            
                            if dst_x >= 0 && dst_x < width as i32 && dst_y >= 0 && dst_y < height as i32 {
                                let val1 = gray1[src_y * width + src_x];
                                let val2 = gray2[dst_y as usize * width + dst_x as usize];
                                sad += (val1 - val2).abs();
                                count += 1;
                            }
                        }
                    }
                    
                    if count > 0 {
                        sad /= count as f32;
                        if sad < best_sad {
                            best_sad = sad;
                            best_dx = dx as f32;
                            best_dy = dy as f32;
                        }
                    }
                }
            }
            
            // Fill block with same flow
            for by in 0..block_size {
                for bx in 0..block_size {
                    let px = x + bx;
                    let py = y + by;
                    if px < width && py < height {
                        flow.push((best_dx, best_dy));
                    }
                }
            }
        }
    }
    
    flow
}

/// Apply motion compensation using optical flow
pub fn apply_motion_compensation(frame: &RgbImage, flow: &[(f32, f32)]) -> RgbImage {
    let mut output = RgbImage::new(frame.width(), frame.height());
    
    let width = frame.width() as usize;
    let height = frame.height() as usize;
    
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let (dx, dy) = flow.get(idx).copied().unwrap_or((0.0, 0.0));
            
            let src_x = x as f32 - dx;
            let src_y = y as f32 - dy;
            
            // Bilinear interpolation
            let x0 = src_x.floor() as i32;
            let y0 = src_y.floor() as i32;
            let x1 = x0 + 1;
            let y1 = y0 + 1;
            
            let fx = src_x - src_x.floor();
            let fy = src_y - src_y.floor();
            
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut weight = 0.0;
            
            for (sy, wy) in [(y0, 1.0 - fy), (y1, fy)] {
                for (sx, wx) in [(x0, 1.0 - fx), (x1, fx)] {
                    if sx >= 0 && sx < width as i32 && sy >= 0 && sy < height as i32 {
                        let pixel = frame.get_pixel(sx as u32, sy as u32);
                        let w = wx * wy;
                        r += pixel[0] as f32 * w;
                        g += pixel[1] as f32 * w;
                        b += pixel[2] as f32 * w;
                        weight += w;
                    }
                }
            }
            
            if weight > 0.0 {
                r /= weight;
                g /= weight;
                b /= weight;
            }
            
            let pixel = Rgb([
                r.clamp(0.0, 255.0) as u8,
                g.clamp(0.0, 255.0) as u8,
                b.clamp(0.0, 255.0) as u8,
            ]);
            
            output.put_pixel(x as u32, y as u32, pixel);
        }
    }
    
    output
}

/// Calculate frame statistics
pub fn calculate_frame_stats(frame: &RgbImage) -> FrameStats {
    let mut sum_r = 0.0;
    let mut sum_g = 0.0;
    let mut sum_b = 0.0;
    let mut sum_sq_r = 0.0;
    let mut sum_sq_g = 0.0;
    let mut sum_sq_b = 0.0;
    let mut min_r = 255.0;
    let mut min_g = 255.0;
    let mut min_b = 255.0;
    let mut max_r = 0.0;
    let mut max_g = 0.0;
    let mut max_b = 0.0;
    
    let pixel_count = (frame.width() * frame.height()) as f32;
    
    for pixel in frame.pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        
        sum_r += r;
        sum_g += g;
        sum_b += b;
        
        sum_sq_r += r * r;
        sum_sq_g += g * g;
        sum_sq_b += b * b;
        
        min_r = min_r.min(r);
        min_g = min_g.min(g);
        min_b = min_b.min(b);
        
        max_r = max_r.max(r);
        max_g = max_g.max(g);
        max_b = max_b.max(b);
    }
    
    let mean_r = sum_r / pixel_count;
    let mean_g = sum_g / pixel_count;
    let mean_b = sum_b / pixel_count;
    
    let std_r = ((sum_sq_r / pixel_count) - mean_r * mean_r).sqrt().max(0.0);
    let std_g = ((sum_sq_g / pixel_count) - mean_g * mean_g).sqrt().max(0.0);
    let std_b = ((sum_sq_b / pixel_count) - mean_b * mean_b).sqrt().max(0.0);
    
    FrameStats {
        mean: (mean_r, mean_g, mean_b),
        std: (std_r, std_g, std_b),
        min: (min_r, min_g, min_b),
        max: (max_r, max_g, max_b),
        pixel_count,
    }
}

/// Frame statistics
#[derive(Debug, Clone)]
pub struct FrameStats {
    /// Mean RGB values
    pub mean: (f32, f32, f32),
    
    /// Standard deviation of RGB values
    pub std: (f32, f32, f32),
    
    /// Minimum RGB values
    pub min: (f32, f32, f32),
    
    /// Maximum RGB values
    pub max: (f32, f32, f32),
    
    /// Total pixel count
    pub pixel_count: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rgb_to_grayscale() {
        let mut frame = RgbImage::new(10, 10);
        for pixel in frame.pixels_mut() {
            *pixel = Rgb([100, 150, 200]);
        }
        
        let gray = rgb_to_grayscale(&frame);
        assert_eq!(gray.len(), 100);
        
        // Check approximate value
        let expected = 0.299 * 100.0 + 0.587 * 150.0 + 0.114 * 200.0;
        assert!((gray[0] - expected).abs() < 0.1);
    }
    
    #[test]
    fn test_resize_frame() {
        let mut frame = RgbImage::new(100, 100);
        for pixel in frame.pixels_mut() {
            *pixel = Rgb([255, 0, 0]);
        }
        
        let resized = resize_frame(&frame, 50, 50);
        assert_eq!(resized.dimensions(), (50, 50));
    }
    
    #[test]
    fn test_crop_frame() {
        let mut frame = RgbImage::new(100, 100);
        for pixel in frame.pixels_mut() {
            *pixel = Rgb([255, 0, 0]);
        }
        
        let cropped = crop_frame(&frame, 10, 10, 50, 50).unwrap();
        assert_eq!(cropped.dimensions(), (50, 50));
    }
}