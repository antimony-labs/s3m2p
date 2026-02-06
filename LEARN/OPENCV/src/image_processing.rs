//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: image_processing.rs | OPENCV/src/image_processing.rs
//! PURPOSE: Pure Rust implementations of computer vision algorithms
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════

use web_sys::ImageData;

/// Convert image to grayscale (returns RGBA with gray values)
pub fn grayscale(input: &ImageData) -> Vec<u8> {
    let data = input.data();
    let mut output = vec![0u8; data.len()];

    for i in (0..data.len()).step_by(4) {
        let r = data[i] as f32;
        let g = data[i + 1] as f32;
        let b = data[i + 2] as f32;

        // Standard luminance formula
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

        output[i] = gray;
        output[i + 1] = gray;
        output[i + 2] = gray;
        output[i + 3] = 255; // Alpha
    }

    output
}

/// Apply Gaussian blur
pub fn gaussian_blur(input: &ImageData, radius: u32) -> Vec<u8> {
    let data = input.data();
    let width = input.width() as i32;
    let height = input.height() as i32;
    let mut output = vec![0u8; data.len()];

    let kernel_size = (radius * 2 + 1) as i32;
    let sigma = radius as f32 / 2.0;

    // Generate 1D Gaussian kernel
    let mut kernel = Vec::with_capacity(kernel_size as usize);
    let mut sum = 0.0f32;

    for i in 0..kernel_size {
        let x = (i - radius as i32) as f32;
        let val = (-x * x / (2.0 * sigma * sigma)).exp();
        kernel.push(val);
        sum += val;
    }

    // Normalize kernel
    for k in kernel.iter_mut() {
        *k /= sum;
    }

    // Horizontal pass
    let mut temp = vec![0u8; data.len()];
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;

            for k in 0..kernel_size {
                let kx = x + k - radius as i32;
                let kx = kx.clamp(0, width - 1);
                let idx = ((y * width + kx) * 4) as usize;

                r_sum += data[idx] as f32 * kernel[k as usize];
                g_sum += data[idx + 1] as f32 * kernel[k as usize];
                b_sum += data[idx + 2] as f32 * kernel[k as usize];
            }

            let idx = ((y * width + x) * 4) as usize;
            temp[idx] = r_sum as u8;
            temp[idx + 1] = g_sum as u8;
            temp[idx + 2] = b_sum as u8;
            temp[idx + 3] = 255;
        }
    }

    // Vertical pass
    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;

            for k in 0..kernel_size {
                let ky = y + k - radius as i32;
                let ky = ky.clamp(0, height - 1);
                let idx = ((ky * width + x) * 4) as usize;

                r_sum += temp[idx] as f32 * kernel[k as usize];
                g_sum += temp[idx + 1] as f32 * kernel[k as usize];
                b_sum += temp[idx + 2] as f32 * kernel[k as usize];
            }

            let idx = ((y * width + x) * 4) as usize;
            output[idx] = r_sum as u8;
            output[idx + 1] = g_sum as u8;
            output[idx + 2] = b_sum as u8;
            output[idx + 3] = 255;
        }
    }

    output
}

/// Canny edge detection
pub fn canny_edge(input: &ImageData, low_threshold: f32, high_threshold: f32) -> Vec<u8> {
    let data = input.data();
    let width = input.width() as i32;
    let height = input.height() as i32;
    let len = data.len();

    // Step 1: Convert to grayscale
    let mut gray = vec![0f32; (width * height) as usize];
    for i in 0..(width * height) as usize {
        let idx = i * 4;
        gray[i] =
            0.299 * data[idx] as f32 + 0.587 * data[idx + 1] as f32 + 0.114 * data[idx + 2] as f32;
    }

    // Step 2: Apply Gaussian blur (simplified 3x3)
    let blur_kernel = [
        1.0 / 16.0,
        2.0 / 16.0,
        1.0 / 16.0,
        2.0 / 16.0,
        4.0 / 16.0,
        2.0 / 16.0,
        1.0 / 16.0,
        2.0 / 16.0,
        1.0 / 16.0,
    ];
    let mut blurred = vec![0f32; gray.len()];
    convolve_gray(&gray, width, height, &blur_kernel, 3, &mut blurred);

    // Step 3: Sobel gradients
    let sobel_x = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let sobel_y = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

    let mut gx = vec![0f32; gray.len()];
    let mut gy = vec![0f32; gray.len()];
    convolve_gray(&blurred, width, height, &sobel_x, 3, &mut gx);
    convolve_gray(&blurred, width, height, &sobel_y, 3, &mut gy);

    // Compute magnitude and direction
    let mut magnitude = vec![0f32; gray.len()];
    let mut direction = vec![0f32; gray.len()];

    for i in 0..gray.len() {
        magnitude[i] = (gx[i] * gx[i] + gy[i] * gy[i]).sqrt();
        direction[i] = gy[i].atan2(gx[i]);
    }

    // Step 4: Non-maximum suppression
    let mut suppressed = vec![0f32; gray.len()];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = (y * width + x) as usize;
            let angle = direction[idx];
            let mag = magnitude[idx];

            // Quantize angle to 4 directions
            let (dx, dy) = if !(-std::f32::consts::FRAC_PI_8 * 3.0
                ..std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle)
            {
                (1, 0) // Horizontal
            } else if (std::f32::consts::FRAC_PI_8..std::f32::consts::FRAC_PI_8 * 3.0)
                .contains(&angle)
            {
                (1, 1) // Diagonal 45
            } else if (-std::f32::consts::FRAC_PI_8 * 3.0..-std::f32::consts::FRAC_PI_8)
                .contains(&angle)
            {
                (1, -1) // Diagonal -45
            } else {
                (0, 1) // Vertical
            };

            let idx1 = ((y + dy) * width + (x + dx)) as usize;
            let idx2 = ((y - dy) * width + (x - dx)) as usize;

            if mag >= magnitude[idx1] && mag >= magnitude[idx2] {
                suppressed[idx] = mag;
            }
        }
    }

    // Step 5: Double threshold and hysteresis
    let mut edges = vec![0u8; gray.len()];
    for i in 0..gray.len() {
        if suppressed[i] >= high_threshold {
            edges[i] = 255; // Strong edge
        } else if suppressed[i] >= low_threshold {
            edges[i] = 128; // Weak edge
        }
    }

    // Connect weak edges to strong edges
    let mut output = vec![0u8; len];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = (y * width + x) as usize;
            let out_idx = idx * 4;

            let val = if edges[idx] == 255 {
                255
            } else if edges[idx] == 128 {
                // Check if connected to strong edge
                let mut connected = false;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let ni = ((y + dy) * width + (x + dx)) as usize;
                        if edges[ni] == 255 {
                            connected = true;
                            break;
                        }
                    }
                    if connected {
                        break;
                    }
                }
                if connected {
                    255
                } else {
                    0
                }
            } else {
                0
            };

            output[out_idx] = val;
            output[out_idx + 1] = val;
            output[out_idx + 2] = val;
            output[out_idx + 3] = 255;
        }
    }

    output
}

/// Apply convolution to grayscale image
fn convolve_gray(
    input: &[f32],
    width: i32,
    height: i32,
    kernel: &[f32],
    ksize: i32,
    output: &mut [f32],
) {
    let half = ksize / 2;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0f32;
            let mut ki = 0;

            for ky in -half..=half {
                for kx in -half..=half {
                    let py = (y + ky).clamp(0, height - 1);
                    let px = (x + kx).clamp(0, width - 1);
                    let idx = (py * width + px) as usize;
                    sum += input[idx] * kernel[ki];
                    ki += 1;
                }
            }

            output[(y * width + x) as usize] = sum;
        }
    }
}

/// Simple thresholding
pub fn threshold(input: &ImageData, thresh: u8) -> Vec<u8> {
    let data = input.data();
    let mut output = vec![0u8; data.len()];

    for i in (0..data.len()).step_by(4) {
        let gray = ((data[i] as u32 + data[i + 1] as u32 + data[i + 2] as u32) / 3) as u8;
        let val = if gray > thresh { 255 } else { 0 };

        output[i] = val;
        output[i + 1] = val;
        output[i + 2] = val;
        output[i + 3] = 255;
    }

    output
}

/// Harris corner detection (simplified)
pub fn harris_corners(input: &ImageData, threshold: f32) -> Vec<u8> {
    let data = input.data();
    let width = input.width() as i32;
    let height = input.height() as i32;

    // Convert to grayscale
    let mut gray = vec![0f32; (width * height) as usize];
    for i in 0..(width * height) as usize {
        let idx = i * 4;
        gray[i] =
            0.299 * data[idx] as f32 + 0.587 * data[idx + 1] as f32 + 0.114 * data[idx + 2] as f32;
    }

    // Compute gradients
    let sobel_x = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let sobel_y = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

    let mut ix = vec![0f32; gray.len()];
    let mut iy = vec![0f32; gray.len()];
    convolve_gray(&gray, width, height, &sobel_x, 3, &mut ix);
    convolve_gray(&gray, width, height, &sobel_y, 3, &mut iy);

    // Copy original to output
    let mut output = data.to_vec();

    // Compute Harris response and mark corners
    let k = 0.04f32;
    for y in 2..height - 2 {
        for x in 2..width - 2 {
            // Sum of products in 5x5 window
            let mut sxx = 0.0f32;
            let mut syy = 0.0f32;
            let mut sxy = 0.0f32;

            for wy in -2..=2 {
                for wx in -2..=2 {
                    let idx = ((y + wy) * width + (x + wx)) as usize;
                    sxx += ix[idx] * ix[idx];
                    syy += iy[idx] * iy[idx];
                    sxy += ix[idx] * iy[idx];
                }
            }

            // Harris response
            let det = sxx * syy - sxy * sxy;
            let trace = sxx + syy;
            let r = det - k * trace * trace;

            // Mark corners
            if r > threshold * 1000.0 {
                let out_idx = ((y * width + x) * 4) as usize;
                // Draw a green dot
                output[out_idx] = 0;
                output[out_idx + 1] = 255;
                output[out_idx + 2] = 0;
            }
        }
    }

    output
}

/// Simple blob detection
pub fn simple_blob_detection(input: &ImageData) -> Vec<u8> {
    let data = input.data();
    let width = input.width() as i32;
    let height = input.height() as i32;

    // Threshold to binary
    let mut binary = vec![false; (width * height) as usize];
    for i in 0..(width * height) as usize {
        let idx = i * 4;
        let gray = (data[idx] as u32 + data[idx + 1] as u32 + data[idx + 2] as u32) / 3;
        binary[i] = gray > 128;
    }

    // Copy original
    let mut output = data.to_vec();

    // Simple connected component labeling (not fully implemented for brevity)
    // Mark bright blobs with circles
    for y in 10..height - 10 {
        for x in 10..width - 10 {
            let idx = (y * width + x) as usize;
            if binary[idx] {
                // Check if center of a blob (simplified)
                let mut count = 0;
                for dy in -5..=5 {
                    for dx in -5..=5 {
                        let ni = ((y + dy) * width + (x + dx)) as usize;
                        if binary[ni] {
                            count += 1;
                        }
                    }
                }

                if count > 80 {
                    // Draw a circle marker
                    let out_idx = ((y * width + x) * 4) as usize;
                    output[out_idx] = 255;
                    output[out_idx + 1] = 0;
                    output[out_idx + 2] = 255;
                }
            }
        }
    }

    output
}

/// Find and draw contours
pub fn find_contours(input: &ImageData, threshold: u8) -> Vec<u8> {
    let data = input.data();
    let width = input.width() as i32;
    let height = input.height() as i32;

    // Threshold to binary
    let mut binary = vec![false; (width * height) as usize];
    for i in 0..(width * height) as usize {
        let idx = i * 4;
        let gray = (data[idx] as u32 + data[idx + 1] as u32 + data[idx + 2] as u32) / 3;
        binary[i] = gray > threshold as u32;
    }

    // Create dark background with contours
    let mut output = vec![0u8; data.len()];

    // Simple edge detection for contours
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = (y * width + x) as usize;
            let out_idx = idx * 4;

            // Check if this is an edge pixel (binary pixel with non-binary neighbor)
            if binary[idx] {
                let mut is_edge = false;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let ni = ((y + dy) * width + (x + dx)) as usize;
                        if !binary[ni] {
                            is_edge = true;
                            break;
                        }
                    }
                    if is_edge {
                        break;
                    }
                }

                if is_edge {
                    output[out_idx] = 0;
                    output[out_idx + 1] = 255;
                    output[out_idx + 2] = 128;
                }
            }

            output[out_idx + 3] = 255;
        }
    }

    output
}

/// Color tracking in HSV space
pub fn color_tracking(input: &ImageData, target_hue: u8, hue_range: u8) -> Vec<u8> {
    let data = input.data();
    let mut output = data.to_vec();

    let mut cx_sum = 0i64;
    let mut cy_sum = 0i64;
    let mut count = 0i64;

    let width = input.width() as i32;
    let height = input.height() as i32;

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let r = data[idx] as f32 / 255.0;
            let g = data[idx + 1] as f32 / 255.0;
            let b = data[idx + 2] as f32 / 255.0;

            // Convert to HSV
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;

            let h = if delta < 0.001 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta) % 6.0)
            } else if max == g {
                60.0 * (((b - r) / delta) + 2.0)
            } else {
                60.0 * (((r - g) / delta) + 4.0)
            };

            let h = if h < 0.0 { h + 360.0 } else { h };
            let h_scaled = (h / 2.0) as u8; // 0-180 range

            let s = if max < 0.001 { 0.0 } else { delta / max };
            let v = max;

            // Check if in target color range
            let hue_diff = if h_scaled > target_hue {
                (h_scaled - target_hue).min(180 - h_scaled + target_hue)
            } else {
                (target_hue - h_scaled).min(180 - target_hue + h_scaled)
            };

            if hue_diff <= hue_range && s > 0.3 && v > 0.2 {
                // Highlight matching pixels
                output[idx] = 255;
                output[idx + 1] = 0;
                output[idx + 2] = 0;

                cx_sum += x as i64;
                cy_sum += y as i64;
                count += 1;
            }
        }
    }

    // Draw crosshair at centroid
    if count > 100 {
        let cx = (cx_sum / count) as i32;
        let cy = (cy_sum / count) as i32;

        // Draw horizontal line
        for x in (cx - 20).max(0)..(cx + 20).min(width) {
            let idx = ((cy * width + x) * 4) as usize;
            output[idx] = 0;
            output[idx + 1] = 255;
            output[idx + 2] = 0;
        }

        // Draw vertical line
        for y in (cy - 20).max(0)..(cy + 20).min(height) {
            let idx = ((y * width + cx) * 4) as usize;
            output[idx] = 0;
            output[idx + 1] = 255;
            output[idx + 2] = 0;
        }
    }

    output
}

/// Simple face detection (very basic skin color detection)
pub fn simple_face_detection(input: &ImageData) -> Vec<u8> {
    let data = input.data();
    let mut output = data.to_vec();

    let width = input.width() as i32;
    let height = input.height() as i32;

    // Simple skin color detection in YCbCr-like space
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let r = data[idx] as f32;
            let g = data[idx + 1] as f32;
            let b = data[idx + 2] as f32;

            // Simple skin detection rules
            let is_skin =
                r > 95.0 && g > 40.0 && b > 20.0 && (r - g).abs() > 15.0 && r > g && r > b;

            if is_skin {
                // Tint skin regions slightly
                output[idx] = (r * 1.1).min(255.0) as u8;
                output[idx + 1] = (g * 0.9) as u8;
                output[idx + 2] = (b * 0.9) as u8;
            }
        }
    }

    // Draw a simple detection indicator
    let center_x = width / 2;
    let center_y = height / 2;
    let radius = width.min(height) / 4;

    for angle in 0..360 {
        let rad = (angle as f32) * std::f32::consts::PI / 180.0;
        let x = center_x + (radius as f32 * rad.cos()) as i32;
        let y = center_y + (radius as f32 * rad.sin()) as i32;

        if x >= 0 && x < width && y >= 0 && y < height {
            let idx = ((y * width + x) * 4) as usize;
            output[idx] = 0;
            output[idx + 1] = 200;
            output[idx + 2] = 255;
        }
    }

    output
}
