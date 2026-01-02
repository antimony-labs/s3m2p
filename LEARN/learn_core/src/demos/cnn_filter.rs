//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: cnn_filter.rs | LEARN/learn_core/src/demos/cnn_filter.rs
//! PURPOSE: CNN filter/convolution visualization demo
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # CNN Filter Demo
//!
//! Visualizes how convolutional neural networks process images:
//! - Shows the convolution sliding window operation
//! - Demonstrates various filter types (edge detection, blur, sharpen)
//! - Displays input image, filter kernel, and output feature map

use crate::{Demo, ParamMeta, Rng};

/// Predefined filter types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterType {
    EdgeHorizontal,
    EdgeVertical,
    EdgeAll,
    Sharpen,
    Blur,
    Emboss,
    Custom,
}

impl FilterType {
    pub fn name(&self) -> &'static str {
        match self {
            FilterType::EdgeHorizontal => "Edge (Horizontal)",
            FilterType::EdgeVertical => "Edge (Vertical)",
            FilterType::EdgeAll => "Edge (Sobel)",
            FilterType::Sharpen => "Sharpen",
            FilterType::Blur => "Blur (Box)",
            FilterType::Emboss => "Emboss",
            FilterType::Custom => "Custom",
        }
    }

    pub fn kernel(&self) -> [[f32; 3]; 3] {
        match self {
            FilterType::EdgeHorizontal => [
                [-1.0, -2.0, -1.0],
                [ 0.0,  0.0,  0.0],
                [ 1.0,  2.0,  1.0],
            ],
            FilterType::EdgeVertical => [
                [-1.0, 0.0, 1.0],
                [-2.0, 0.0, 2.0],
                [-1.0, 0.0, 1.0],
            ],
            FilterType::EdgeAll => [
                [-1.0, -1.0, -1.0],
                [-1.0,  8.0, -1.0],
                [-1.0, -1.0, -1.0],
            ],
            FilterType::Sharpen => [
                [ 0.0, -1.0,  0.0],
                [-1.0,  5.0, -1.0],
                [ 0.0, -1.0,  0.0],
            ],
            FilterType::Blur => {
                let v = 1.0 / 9.0;
                [[v, v, v], [v, v, v], [v, v, v]]
            },
            FilterType::Emboss => [
                [-2.0, -1.0, 0.0],
                [-1.0,  1.0, 1.0],
                [ 0.0,  1.0, 2.0],
            ],
            FilterType::Custom => [
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ],
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 7 {
            0 => FilterType::EdgeHorizontal,
            1 => FilterType::EdgeVertical,
            2 => FilterType::EdgeAll,
            3 => FilterType::Sharpen,
            4 => FilterType::Blur,
            5 => FilterType::Emboss,
            _ => FilterType::Custom,
        }
    }
}

/// Input image pattern
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImagePattern {
    Checkerboard,
    Stripes,
    Circle,
    Gradient,
    Noise,
    Letter,
}

impl ImagePattern {
    pub fn name(&self) -> &'static str {
        match self {
            ImagePattern::Checkerboard => "Checkerboard",
            ImagePattern::Stripes => "Stripes",
            ImagePattern::Circle => "Circle",
            ImagePattern::Gradient => "Gradient",
            ImagePattern::Noise => "Noise",
            ImagePattern::Letter => "Letter",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 6 {
            0 => ImagePattern::Checkerboard,
            1 => ImagePattern::Stripes,
            2 => ImagePattern::Circle,
            3 => ImagePattern::Gradient,
            4 => ImagePattern::Noise,
            _ => ImagePattern::Letter,
        }
    }
}

/// CNN Filter demo
#[derive(Clone)]
pub struct CnnFilterDemo {
    // Input image (grayscale, values 0-1)
    pub input: Vec<Vec<f32>>,
    pub input_size: usize,

    // Filter/kernel (3x3)
    pub kernel: [[f32; 3]; 3],
    pub filter_type: FilterType,

    // Output feature map
    pub output: Vec<Vec<f32>>,

    // Animation state
    pub current_x: usize,
    pub current_y: usize,
    pub animating: bool,
    animation_speed: f32,
    animation_timer: f32,

    // Current convolution result at position
    pub current_sum: f32,
    pub current_products: [[f32; 3]; 3],

    // Pattern
    pub pattern: ImagePattern,

    // RNG
    rng: Rng,
    seed: u64,
}

impl Default for CnnFilterDemo {
    fn default() -> Self {
        Self {
            input: Vec::new(),
            input_size: 16,
            kernel: FilterType::EdgeAll.kernel(),
            filter_type: FilterType::EdgeAll,
            output: Vec::new(),
            current_x: 0,
            current_y: 0,
            animating: false,
            animation_speed: 5.0,
            animation_timer: 0.0,
            current_sum: 0.0,
            current_products: [[0.0; 3]; 3],
            pattern: ImagePattern::Checkerboard,
            rng: Rng::new(42),
            seed: 42,
        }
    }
}

impl CnnFilterDemo {
    /// Generate input image based on pattern
    fn generate_input(&mut self) {
        let n = self.input_size;
        self.input = vec![vec![0.0; n]; n];

        match self.pattern {
            ImagePattern::Checkerboard => {
                let block = 4.max(n / 4);
                for y in 0..n {
                    for x in 0..n {
                        let cx = x / block;
                        let cy = y / block;
                        self.input[y][x] = if (cx + cy) % 2 == 0 { 1.0 } else { 0.0 };
                    }
                }
            }
            ImagePattern::Stripes => {
                for y in 0..n {
                    for x in 0..n {
                        self.input[y][x] = if (x / 2) % 2 == 0 { 1.0 } else { 0.0 };
                    }
                }
            }
            ImagePattern::Circle => {
                let center = n as f32 / 2.0;
                let radius = n as f32 * 0.3;
                for y in 0..n {
                    for x in 0..n {
                        let dx = x as f32 - center;
                        let dy = y as f32 - center;
                        let dist = (dx * dx + dy * dy).sqrt();
                        self.input[y][x] = if dist < radius { 1.0 } else { 0.0 };
                    }
                }
            }
            ImagePattern::Gradient => {
                for y in 0..n {
                    for x in 0..n {
                        self.input[y][x] = x as f32 / (n - 1) as f32;
                    }
                }
            }
            ImagePattern::Noise => {
                for y in 0..n {
                    for x in 0..n {
                        self.input[y][x] = self.rng.next_f32();
                    }
                }
            }
            ImagePattern::Letter => {
                // Draw a simple "A" shape
                for y in 0..n {
                    for x in 0..n {
                        let rel_y = y as f32 / n as f32;
                        let rel_x = x as f32 / n as f32;

                        // Two diagonal lines forming A
                        let left_edge = 0.5 - rel_y * 0.35;
                        let right_edge = 0.5 + rel_y * 0.35;
                        let on_left = (rel_x - left_edge).abs() < 0.08;
                        let on_right = (rel_x - right_edge).abs() < 0.08;
                        // Horizontal bar
                        let on_bar = rel_y > 0.45 && rel_y < 0.55 && rel_x > left_edge && rel_x < right_edge;

                        self.input[y][x] = if (on_left || on_right || on_bar) && rel_y > 0.1 { 1.0 } else { 0.0 };
                    }
                }
            }
        }
    }

    /// Compute full convolution output
    fn compute_output(&mut self) {
        let n = self.input_size;
        let out_size = n - 2; // Valid convolution (no padding)
        self.output = vec![vec![0.0; out_size]; out_size];

        for y in 0..out_size {
            for x in 0..out_size {
                let val = self.convolve_at(x, y);
                self.output[y][x] = val;
            }
        }
    }

    /// Perform convolution at a specific position
    fn convolve_at(&self, x: usize, y: usize) -> f32 {
        let mut sum = 0.0;
        for ky in 0..3 {
            for kx in 0..3 {
                let ix = x + kx;
                let iy = y + ky;
                if iy < self.input.len() && ix < self.input[0].len() {
                    sum += self.input[iy][ix] * self.kernel[ky][kx];
                }
            }
        }
        sum
    }

    /// Update current position computation for visualization
    fn update_current(&mut self) {
        let mut sum = 0.0;
        for ky in 0..3 {
            for kx in 0..3 {
                let ix = self.current_x + kx;
                let iy = self.current_y + ky;
                if iy < self.input.len() && ix < self.input[0].len() {
                    let product = self.input[iy][ix] * self.kernel[ky][kx];
                    self.current_products[ky][kx] = product;
                    sum += product;
                } else {
                    self.current_products[ky][kx] = 0.0;
                }
            }
        }
        self.current_sum = sum;
    }

    /// Advance animation position
    fn advance_position(&mut self) {
        let out_size = self.input_size.saturating_sub(2);
        if out_size == 0 {
            return;
        }

        self.current_x += 1;
        if self.current_x >= out_size {
            self.current_x = 0;
            self.current_y += 1;
            if self.current_y >= out_size {
                self.current_y = 0;
            }
        }
        self.update_current();
    }

    /// Get normalized output for display
    pub fn normalized_output(&self) -> Vec<Vec<f32>> {
        if self.output.is_empty() {
            return Vec::new();
        }

        // Find min/max
        let mut min_val = f32::MAX;
        let mut max_val = f32::MIN;
        for row in &self.output {
            for &val in row {
                min_val = min_val.min(val);
                max_val = max_val.max(val);
            }
        }

        let range = (max_val - min_val).max(1e-6);

        self.output.iter().map(|row| {
            row.iter().map(|&val| (val - min_val) / range).collect()
        }).collect()
    }

    /// Get kernel as flat array for display
    pub fn kernel_values(&self) -> [f32; 9] {
        let mut result = [0.0; 9];
        for y in 0..3 {
            for x in 0..3 {
                result[y * 3 + x] = self.kernel[y][x];
            }
        }
        result
    }
}

impl Demo for CnnFilterDemo {
    fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Rng::new(seed);
        self.current_x = 0;
        self.current_y = 0;
        self.animation_timer = 0.0;

        self.kernel = self.filter_type.kernel();
        self.generate_input();
        self.compute_output();
        self.update_current();
    }

    fn step(&mut self, dt: f32) {
        if self.animating {
            self.animation_timer += dt * self.animation_speed;
            if self.animation_timer >= 1.0 {
                self.animation_timer = 0.0;
                self.advance_position();
            }
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "filter" => {
                self.filter_type = FilterType::from_index(value as usize);
                self.kernel = self.filter_type.kernel();
                self.compute_output();
                self.update_current();
                true
            }
            "pattern" => {
                self.pattern = ImagePattern::from_index(value as usize);
                self.rng = Rng::new(self.seed);
                self.generate_input();
                self.compute_output();
                self.update_current();
                true
            }
            "speed" => {
                self.animation_speed = value.clamp(1.0, 20.0);
                true
            }
            "animate" => {
                self.animating = value > 0.5;
                true
            }
            "pos_x" => {
                let out_size = self.input_size.saturating_sub(2);
                self.current_x = (value as usize).min(out_size.saturating_sub(1));
                self.update_current();
                true
            }
            "pos_y" => {
                let out_size = self.input_size.saturating_sub(2);
                self.current_y = (value as usize).min(out_size.saturating_sub(1));
                self.update_current();
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "filter",
                label: "Filter Type",
                min: 0.0,
                max: 6.0,
                step: 1.0,
                default: 2.0, // EdgeAll
            },
            ParamMeta {
                name: "pattern",
                label: "Input Pattern",
                min: 0.0,
                max: 5.0,
                step: 1.0,
                default: 0.0,
            },
            ParamMeta {
                name: "speed",
                label: "Animation Speed",
                min: 1.0,
                max: 20.0,
                step: 1.0,
                default: 5.0,
            },
            ParamMeta {
                name: "animate",
                label: "Auto-Animate",
                min: 0.0,
                max: 1.0,
                step: 1.0,
                default: 0.0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_averages() {
        let mut demo = CnnFilterDemo::default();
        demo.filter_type = FilterType::Blur;
        demo.pattern = ImagePattern::Gradient;
        demo.reset(42);

        // Blur should create smooth values
        assert!(!demo.output.is_empty());
    }

    #[test]
    fn test_identity_kernel() {
        let mut demo = CnnFilterDemo::default();
        demo.filter_type = FilterType::Custom; // Custom is identity kernel
        demo.pattern = ImagePattern::Checkerboard;
        demo.reset(42);

        // Identity kernel should preserve center pixel values
        for y in 0..demo.output.len() {
            for x in 0..demo.output[0].len() {
                let expected = demo.input[y + 1][x + 1];
                assert!(
                    (demo.output[y][x] - expected).abs() < 1e-5,
                    "Identity failed at ({}, {})",
                    x, y
                );
            }
        }
    }

    #[test]
    fn test_edge_detection_finds_edges() {
        let mut demo = CnnFilterDemo::default();
        demo.filter_type = FilterType::EdgeAll;
        demo.pattern = ImagePattern::Checkerboard;
        demo.reset(42);

        // Edge detection should have non-zero values at transitions
        let has_edges = demo.output.iter().any(|row| row.iter().any(|&v| v.abs() > 0.1));
        assert!(has_edges, "Edge detection should find edges in checkerboard");
    }

    #[test]
    fn test_animation_advances() {
        let mut demo = CnnFilterDemo::default();
        demo.reset(42);
        demo.animating = true;
        demo.animation_speed = 100.0; // Fast

        let start_x = demo.current_x;
        let start_y = demo.current_y;

        // Run several steps
        for _ in 0..100 {
            demo.step(0.1);
        }

        // Position should have changed
        assert!(
            demo.current_x != start_x || demo.current_y != start_y,
            "Animation should advance position"
        );
    }
}
