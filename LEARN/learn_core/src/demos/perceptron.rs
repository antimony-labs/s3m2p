//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: perceptron.rs | LEARN/learn_core/src/demos/perceptron.rs
//! PURPOSE: Interactive perceptron demo with XOR problem visualization
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # Perceptron Demo
//!
//! Demonstrates how a single perceptron learns a linear decision boundary.
//! Shows the famous XOR problem where a single perceptron fails, and how
//! adding a hidden layer (MLP) solves it.

use crate::{Demo, ParamMeta, Rng, Vec2};

/// Dataset type for the perceptron demo
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataset {
    /// Linearly separable - two clusters
    Linear,
    /// XOR problem - not linearly separable
    Xor,
    /// Circle - inside vs outside
    Circle,
    /// Spiral - two interleaved spirals
    Spiral,
}

impl Dataset {
    pub fn name(&self) -> &'static str {
        match self {
            Dataset::Linear => "Linear",
            Dataset::Xor => "XOR",
            Dataset::Circle => "Circle",
            Dataset::Spiral => "Spiral",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Dataset::Linear,
            1 => Dataset::Xor,
            2 => Dataset::Circle,
            _ => Dataset::Spiral,
        }
    }
}

/// A data point with binary label
#[derive(Clone, Copy, Debug)]
pub struct DataPoint {
    pub pos: Vec2,
    pub label: bool, // true = class 1, false = class 0
}

/// Perceptron demo for learning decision boundaries
#[derive(Clone)]
pub struct PerceptronDemo {
    // Dataset
    pub points: Vec<DataPoint>,
    pub dataset: Dataset,
    num_points: usize,

    // Single perceptron weights
    pub w1: f32,
    pub w2: f32,
    pub bias: f32,

    // For MLP mode (hidden layer with 2-4 neurons)
    pub use_hidden_layer: bool,
    pub hidden_w: Vec<[f32; 3]>, // weights for hidden neurons [w1, w2, bias]
    pub output_w: Vec<f32>,       // weights from hidden to output (including bias)

    // Training state
    learning_rate: f32,
    pub step_count: usize,
    pub accuracy: f32,
    pub loss_history: Vec<f32>,

    // Visualization
    pub decision_boundary: Vec<Vec<f32>>, // 2D grid of predictions
    grid_resolution: usize,

    // RNG
    rng: Rng,
    seed: u64,
}

impl Default for PerceptronDemo {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            dataset: Dataset::Linear,
            num_points: 100,
            w1: 0.0,
            w2: 0.0,
            bias: 0.0,
            use_hidden_layer: false,
            hidden_w: vec![[0.0; 3]; 4],
            output_w: vec![0.0; 5], // 4 hidden + 1 bias
            learning_rate: 0.1,
            step_count: 0,
            accuracy: 0.0,
            loss_history: Vec::new(),
            decision_boundary: Vec::new(),
            grid_resolution: 30,
            rng: Rng::new(42),
            seed: 42,
        }
    }
}

impl PerceptronDemo {
    /// Generate dataset based on current type
    fn generate_data(&mut self) {
        self.points.clear();

        match self.dataset {
            Dataset::Linear => self.generate_linear(),
            Dataset::Xor => self.generate_xor(),
            Dataset::Circle => self.generate_circle(),
            Dataset::Spiral => self.generate_spiral(),
        }
    }

    fn generate_linear(&mut self) {
        for _ in 0..self.num_points {
            let x = self.rng.range(-1.0, 1.0);
            let y = self.rng.range(-1.0, 1.0);
            // Diagonal line with some margin
            let label = y > 0.3 * x + 0.1 + self.rng.range(-0.1, 0.1);
            self.points.push(DataPoint {
                pos: Vec2::new(x, y),
                label,
            });
        }
    }

    fn generate_xor(&mut self) {
        // Generate 4 clusters in XOR pattern
        let n_per_cluster = self.num_points / 4;
        let clusters = [
            (Vec2::new(-0.5, -0.5), false),
            (Vec2::new(0.5, 0.5), false),
            (Vec2::new(-0.5, 0.5), true),
            (Vec2::new(0.5, -0.5), true),
        ];

        for (center, label) in clusters.iter() {
            for _ in 0..n_per_cluster {
                let x = center.x + self.rng.range(-0.25, 0.25);
                let y = center.y + self.rng.range(-0.25, 0.25);
                self.points.push(DataPoint {
                    pos: Vec2::new(x, y),
                    label: *label,
                });
            }
        }
    }

    fn generate_circle(&mut self) {
        let inner_radius = 0.35;
        let outer_radius = 0.7;

        for _ in 0..self.num_points / 2 {
            // Inner circle (class 0)
            let angle = self.rng.range(0.0, std::f32::consts::TAU);
            let r = self.rng.range(0.0, inner_radius);
            self.points.push(DataPoint {
                pos: Vec2::new(r * angle.cos(), r * angle.sin()),
                label: false,
            });
        }

        for _ in 0..self.num_points / 2 {
            // Outer ring (class 1)
            let angle = self.rng.range(0.0, std::f32::consts::TAU);
            let r = self.rng.range(inner_radius + 0.1, outer_radius);
            self.points.push(DataPoint {
                pos: Vec2::new(r * angle.cos(), r * angle.sin()),
                label: true,
            });
        }
    }

    fn generate_spiral(&mut self) {
        let turns = 1.5;

        for i in 0..self.num_points / 2 {
            let t = i as f32 / (self.num_points / 2) as f32;
            let angle = t * std::f32::consts::TAU * turns;
            let r = t * 0.8 + 0.1;
            let noise = self.rng.range(-0.05, 0.05);

            // Spiral 0
            self.points.push(DataPoint {
                pos: Vec2::new(
                    r * angle.cos() + noise,
                    r * angle.sin() + noise,
                ),
                label: false,
            });

            // Spiral 1 (offset by PI)
            let angle2 = angle + std::f32::consts::PI;
            self.points.push(DataPoint {
                pos: Vec2::new(
                    r * angle2.cos() + noise,
                    r * angle2.sin() + noise,
                ),
                label: true,
            });
        }
    }

    /// Initialize weights randomly
    fn init_weights(&mut self) {
        // Single perceptron
        self.w1 = self.rng.range(-0.5, 0.5);
        self.w2 = self.rng.range(-0.5, 0.5);
        self.bias = self.rng.range(-0.5, 0.5);

        // Hidden layer (4 neurons)
        for h in 0..4 {
            self.hidden_w[h] = [
                self.rng.range(-0.5, 0.5),
                self.rng.range(-0.5, 0.5),
                self.rng.range(-0.5, 0.5),
            ];
        }

        // Output weights
        for i in 0..5 {
            self.output_w[i] = self.rng.range(-0.5, 0.5);
        }
    }

    /// Sigmoid activation
    #[inline]
    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Forward pass for single perceptron
    fn predict_perceptron(&self, x: f32, y: f32) -> f32 {
        Self::sigmoid(self.w1 * x + self.w2 * y + self.bias)
    }

    /// Forward pass for MLP
    fn predict_mlp(&self, x: f32, y: f32) -> f32 {
        // Hidden layer activations
        let mut hidden = [0.0f32; 4];
        for (h, hw) in self.hidden_w.iter().enumerate() {
            hidden[h] = Self::sigmoid(hw[0] * x + hw[1] * y + hw[2]);
        }

        // Output
        let mut sum = self.output_w[4]; // bias
        for (h, &act) in hidden.iter().enumerate() {
            sum += self.output_w[h] * act;
        }
        Self::sigmoid(sum)
    }

    /// Get prediction based on current mode
    pub fn predict(&self, x: f32, y: f32) -> f32 {
        if self.use_hidden_layer {
            self.predict_mlp(x, y)
        } else {
            self.predict_perceptron(x, y)
        }
    }

    /// Update decision boundary grid
    fn update_boundary(&mut self) {
        let n = self.grid_resolution;
        self.decision_boundary.clear();

        for iy in 0..n {
            let mut row = Vec::with_capacity(n);
            let y = (iy as f32 / (n - 1) as f32) * 2.0 - 1.0;

            for ix in 0..n {
                let x = (ix as f32 / (n - 1) as f32) * 2.0 - 1.0;
                row.push(self.predict(x, y));
            }
            self.decision_boundary.push(row);
        }
    }

    /// Compute accuracy
    fn compute_accuracy(&self) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        let correct: usize = self
            .points
            .iter()
            .filter(|p| {
                let pred = self.predict(p.pos.x, p.pos.y);
                (pred > 0.5) == p.label
            })
            .count();

        correct as f32 / self.points.len() as f32
    }

    /// Compute binary cross-entropy loss
    fn compute_loss(&self) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        let sum: f32 = self
            .points
            .iter()
            .map(|p| {
                let pred = self.predict(p.pos.x, p.pos.y).clamp(1e-7, 1.0 - 1e-7);
                let y = if p.label { 1.0 } else { 0.0 };
                -(y * pred.ln() + (1.0 - y) * (1.0 - pred).ln())
            })
            .sum();

        sum / self.points.len() as f32
    }
}

impl Demo for PerceptronDemo {
    fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Rng::new(seed);
        self.step_count = 0;
        self.loss_history.clear();

        self.init_weights();
        self.generate_data();
        self.update_boundary();
        self.accuracy = self.compute_accuracy();
    }

    fn step(&mut self, _dt: f32) {
        if self.points.is_empty() {
            return;
        }

        // Mini-batch gradient descent
        let batch_size = 16.min(self.points.len());

        if self.use_hidden_layer {
            // MLP training with backpropagation
            let mut grad_hidden = [[0.0f32; 3]; 4];
            let mut grad_output = [0.0f32; 5];

            for _ in 0..batch_size {
                let idx = self.rng.range_int(0, self.points.len() as i32) as usize;
                let p = self.points[idx];
                let x = p.pos.x;
                let y = p.pos.y;
                let target = if p.label { 1.0 } else { 0.0 };

                // Forward pass
                let mut hidden = [0.0f32; 4];
                for (h, hw) in self.hidden_w.iter().enumerate() {
                    hidden[h] = Self::sigmoid(hw[0] * x + hw[1] * y + hw[2]);
                }

                let mut out_sum = self.output_w[4];
                for (h, &act) in hidden.iter().enumerate() {
                    out_sum += self.output_w[h] * act;
                }
                let output = Self::sigmoid(out_sum);

                // Output error (d_L/d_out)
                let d_output = output - target;

                // Gradient for output weights
                for h in 0..4 {
                    grad_output[h] += d_output * hidden[h];
                }
                grad_output[4] += d_output; // bias

                // Backprop to hidden
                for h in 0..4 {
                    let d_hidden = d_output * self.output_w[h] * hidden[h] * (1.0 - hidden[h]);
                    grad_hidden[h][0] += d_hidden * x;
                    grad_hidden[h][1] += d_hidden * y;
                    grad_hidden[h][2] += d_hidden;
                }
            }

            // Apply gradients
            let scale = self.learning_rate / batch_size as f32;
            for (h, grad_h) in grad_hidden.iter().enumerate() {
                self.hidden_w[h][0] -= scale * grad_h[0];
                self.hidden_w[h][1] -= scale * grad_h[1];
                self.hidden_w[h][2] -= scale * grad_h[2];
            }
            for (i, &grad) in grad_output.iter().enumerate() {
                self.output_w[i] -= scale * grad;
            }
        } else {
            // Simple perceptron training
            let mut grad_w1 = 0.0;
            let mut grad_w2 = 0.0;
            let mut grad_b = 0.0;

            for _ in 0..batch_size {
                let idx = self.rng.range_int(0, self.points.len() as i32) as usize;
                let p = self.points[idx];
                let target = if p.label { 1.0 } else { 0.0 };
                let pred = self.predict_perceptron(p.pos.x, p.pos.y);
                let error = pred - target;

                grad_w1 += error * p.pos.x;
                grad_w2 += error * p.pos.y;
                grad_b += error;
            }

            let scale = self.learning_rate / batch_size as f32;
            self.w1 -= scale * grad_w1;
            self.w2 -= scale * grad_w2;
            self.bias -= scale * grad_b;
        }

        self.step_count += 1;

        // Update metrics every few steps
        if self.step_count.is_multiple_of(5) {
            self.accuracy = self.compute_accuracy();
            let loss = self.compute_loss();
            self.loss_history.push(loss);
            if self.loss_history.len() > 100 {
                self.loss_history.remove(0);
            }
            self.update_boundary();
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "learning_rate" => {
                self.learning_rate = value.clamp(0.001, 1.0);
                true
            }
            "dataset" => {
                self.dataset = Dataset::from_index(value as usize);
                self.rng = Rng::new(self.seed);
                self.generate_data();
                self.init_weights();
                self.step_count = 0;
                self.loss_history.clear();
                self.update_boundary();
                self.accuracy = self.compute_accuracy();
                true
            }
            "hidden_layer" => {
                self.use_hidden_layer = value > 0.5;
                self.init_weights();
                self.step_count = 0;
                self.loss_history.clear();
                self.update_boundary();
                self.accuracy = self.compute_accuracy();
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "learning_rate",
                label: "Learning Rate",
                min: 0.001,
                max: 1.0,
                step: 0.01,
                default: 0.1,
            },
            ParamMeta {
                name: "dataset",
                label: "Dataset",
                min: 0.0,
                max: 3.0,
                step: 1.0,
                default: 0.0,
            },
            ParamMeta {
                name: "hidden_layer",
                label: "Use Hidden Layer",
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
    fn test_perceptron_learns_linear() {
        let mut demo = PerceptronDemo::default();
        demo.dataset = Dataset::Linear;
        demo.learning_rate = 0.5;
        demo.reset(42);

        let initial_acc = demo.compute_accuracy();

        for _ in 0..500 {
            demo.step(0.016);
        }

        let final_acc = demo.compute_accuracy();
        assert!(
            final_acc > initial_acc || final_acc > 0.8,
            "Accuracy should improve: {} -> {}",
            initial_acc,
            final_acc
        );
    }

    #[test]
    fn test_perceptron_fails_xor() {
        let mut demo = PerceptronDemo::default();
        demo.dataset = Dataset::Xor;
        demo.use_hidden_layer = false;
        demo.learning_rate = 0.5;
        demo.reset(42);

        for _ in 0..1000 {
            demo.step(0.016);
        }

        // Single perceptron should NOT solve XOR well
        let acc = demo.compute_accuracy();
        assert!(
            acc < 0.75,
            "Perceptron should fail XOR (acc={})",
            acc
        );
    }

    #[test]
    fn test_mlp_solves_xor() {
        let mut demo = PerceptronDemo::default();
        demo.dataset = Dataset::Xor;
        demo.use_hidden_layer = true;
        demo.learning_rate = 0.5;
        demo.reset(42);

        for _ in 0..2000 {
            demo.step(0.016);
        }

        // MLP should solve XOR
        let acc = demo.compute_accuracy();
        assert!(
            acc > 0.85,
            "MLP should solve XOR (acc={})",
            acc
        );
    }
}
