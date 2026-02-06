//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: neural_network.rs | LEARN/learn_core/src/demos/neural_network.rs
//! PURPOSE: TensorFlow Playground-style neural network visualization
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # Neural Network Playground
//!
//! Interactive neural network with configurable architecture:
//! - Variable number of hidden layers (1-4)
//! - Variable neurons per layer (1-8)
//! - Real-time decision boundary visualization
//! - Multiple dataset patterns

use crate::{Demo, ParamMeta, Rng, Vec2};

/// Activation function type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
    Linear,
}

impl Activation {
    pub fn name(&self) -> &'static str {
        match self {
            Activation::ReLU => "ReLU",
            Activation::Sigmoid => "Sigmoid",
            Activation::Tanh => "Tanh",
            Activation::Linear => "Linear",
        }
    }

    #[inline]
    pub fn apply(&self, x: f32) -> f32 {
        match self {
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Linear => x,
        }
    }

    #[inline]
    pub fn derivative(&self, x: f32, output: f32) -> f32 {
        match self {
            Activation::ReLU => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Activation::Sigmoid => output * (1.0 - output),
            Activation::Tanh => 1.0 - output * output,
            Activation::Linear => 1.0,
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Activation::ReLU,
            1 => Activation::Sigmoid,
            2 => Activation::Tanh,
            _ => Activation::Linear,
        }
    }
}

/// Dataset pattern for training
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NNDataset {
    Circle,
    Xor,
    Gaussian,
    Spiral,
}

impl NNDataset {
    pub fn name(&self) -> &'static str {
        match self {
            NNDataset::Circle => "Circle",
            NNDataset::Xor => "XOR",
            NNDataset::Gaussian => "Gaussian",
            NNDataset::Spiral => "Spiral",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => NNDataset::Circle,
            1 => NNDataset::Xor,
            2 => NNDataset::Gaussian,
            _ => NNDataset::Spiral,
        }
    }
}

/// A data point for training
#[derive(Clone, Copy, Debug)]
pub struct NNDataPoint {
    pub x: f32,
    pub y: f32,
    pub label: i32, // -1 or +1
}

/// Dense layer with weights and biases
#[derive(Clone, Debug)]
pub struct Layer {
    pub weights: Vec<Vec<f32>>,    // [output_size][input_size]
    pub biases: Vec<f32>,          // [output_size]
    pub activations: Vec<f32>,     // [output_size] - cached for visualization
    pub pre_activations: Vec<f32>, // [output_size] - before activation
}

impl Layer {
    fn new(input_size: usize, output_size: usize, rng: &mut Rng) -> Self {
        // Xavier initialization
        let scale = (2.0 / (input_size + output_size) as f32).sqrt();

        let mut weights = Vec::with_capacity(output_size);
        for _ in 0..output_size {
            let mut row = Vec::with_capacity(input_size);
            for _ in 0..input_size {
                row.push(rng.range(-scale, scale));
            }
            weights.push(row);
        }

        let biases = vec![0.0; output_size];
        let activations = vec![0.0; output_size];
        let pre_activations = vec![0.0; output_size];

        Self {
            weights,
            biases,
            activations,
            pre_activations,
        }
    }

    fn forward(&mut self, inputs: &[f32], activation: Activation) {
        for (i, (w_row, bias)) in self.weights.iter().zip(self.biases.iter()).enumerate() {
            let mut sum = *bias;
            for (w, x) in w_row.iter().zip(inputs.iter()) {
                sum += w * x;
            }
            self.pre_activations[i] = sum;
            self.activations[i] = activation.apply(sum);
        }
    }
}

/// Neural Network Playground demo
#[derive(Clone)]
pub struct NeuralNetworkDemo {
    // Dataset
    pub points: Vec<NNDataPoint>,
    pub dataset: NNDataset,
    num_points: usize,

    // Network architecture
    pub layers: Vec<Layer>,
    pub layer_sizes: Vec<usize>, // includes input (2) and output (1)
    pub activation: Activation,

    // Training
    learning_rate: f32,
    regularization: f32,
    pub step_count: usize,
    pub loss_history: Vec<f32>,
    pub accuracy: f32,
    batch_size: usize,

    // Decision boundary visualization
    pub decision_grid: Vec<Vec<f32>>, // 2D grid of outputs
    pub grid_resolution: usize,

    // RNG
    rng: Rng,
    seed: u64,
}

impl Default for NeuralNetworkDemo {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            dataset: NNDataset::Circle,
            num_points: 200,
            layers: Vec::new(),
            layer_sizes: vec![2, 4, 4, 1], // Default: 2 hidden layers with 4 neurons each
            activation: Activation::ReLU,
            learning_rate: 0.03,
            regularization: 0.0,
            step_count: 0,
            loss_history: Vec::new(),
            accuracy: 0.0,
            batch_size: 16,
            decision_grid: Vec::new(),
            grid_resolution: 25,
            rng: Rng::new(42),
            seed: 42,
        }
    }
}

impl NeuralNetworkDemo {
    /// Initialize network with current layer_sizes
    fn init_network(&mut self) {
        self.layers.clear();

        for i in 0..self.layer_sizes.len() - 1 {
            let input_size = self.layer_sizes[i];
            let output_size = self.layer_sizes[i + 1];
            self.layers
                .push(Layer::new(input_size, output_size, &mut self.rng));
        }
    }

    /// Generate dataset
    fn generate_data(&mut self) {
        self.points.clear();

        match self.dataset {
            NNDataset::Circle => self.generate_circle(),
            NNDataset::Xor => self.generate_xor(),
            NNDataset::Gaussian => self.generate_gaussian(),
            NNDataset::Spiral => self.generate_spiral(),
        }
    }

    fn generate_circle(&mut self) {
        let inner_r = 0.35;
        let outer_r = 0.7;

        for _ in 0..self.num_points / 2 {
            let angle = self.rng.range(0.0, std::f32::consts::TAU);
            let r = self.rng.range(0.0, inner_r);
            self.points.push(NNDataPoint {
                x: r * angle.cos(),
                y: r * angle.sin(),
                label: -1,
            });
        }

        for _ in 0..self.num_points / 2 {
            let angle = self.rng.range(0.0, std::f32::consts::TAU);
            let r = self.rng.range(inner_r + 0.1, outer_r);
            self.points.push(NNDataPoint {
                x: r * angle.cos(),
                y: r * angle.sin(),
                label: 1,
            });
        }
    }

    fn generate_xor(&mut self) {
        let clusters = [
            (Vec2::new(-0.5, -0.5), -1),
            (Vec2::new(0.5, 0.5), -1),
            (Vec2::new(-0.5, 0.5), 1),
            (Vec2::new(0.5, -0.5), 1),
        ];

        let per_cluster = self.num_points / 4;
        for (center, label) in clusters.iter() {
            for _ in 0..per_cluster {
                self.points.push(NNDataPoint {
                    x: center.x + self.rng.range(-0.2, 0.2),
                    y: center.y + self.rng.range(-0.2, 0.2),
                    label: *label,
                });
            }
        }
    }

    fn generate_gaussian(&mut self) {
        // Two overlapping Gaussian clusters
        for _ in 0..self.num_points / 2 {
            self.points.push(NNDataPoint {
                x: self.rng.normal_with(-0.3, 0.25),
                y: self.rng.normal_with(-0.3, 0.25),
                label: -1,
            });
        }

        for _ in 0..self.num_points / 2 {
            self.points.push(NNDataPoint {
                x: self.rng.normal_with(0.3, 0.25),
                y: self.rng.normal_with(0.3, 0.25),
                label: 1,
            });
        }

        // Clamp to bounds
        for p in &mut self.points {
            p.x = p.x.clamp(-1.0, 1.0);
            p.y = p.y.clamp(-1.0, 1.0);
        }
    }

    fn generate_spiral(&mut self) {
        let turns = 1.5;

        for i in 0..self.num_points / 2 {
            let t = i as f32 / (self.num_points / 2) as f32;
            let angle = t * std::f32::consts::TAU * turns;
            let r = t * 0.7 + 0.1;
            let noise = self.rng.range(-0.05, 0.05);

            self.points.push(NNDataPoint {
                x: r * angle.cos() + noise,
                y: r * angle.sin() + noise,
                label: -1,
            });

            let angle2 = angle + std::f32::consts::PI;
            self.points.push(NNDataPoint {
                x: r * angle2.cos() + noise,
                y: r * angle2.sin() + noise,
                label: 1,
            });
        }
    }

    /// Forward pass through network
    fn forward(&mut self, x: f32, y: f32) -> f32 {
        if self.layers.is_empty() {
            return 0.0;
        }

        let mut inputs = vec![x, y];
        let num_layers = self.layers.len();

        for (i, layer) in self.layers.iter_mut().enumerate() {
            let is_output = i == num_layers - 1;
            let act = if is_output {
                Activation::Tanh
            } else {
                self.activation
            };
            layer.forward(&inputs, act);
            inputs = layer.activations.clone();
        }

        inputs.first().copied().unwrap_or(0.0)
    }

    /// Get prediction without mutating (for grid computation)
    pub fn predict(&self, x: f32, y: f32) -> f32 {
        if self.layers.is_empty() {
            return 0.0;
        }

        let mut inputs = vec![x, y];

        for (i, layer) in self.layers.iter().enumerate() {
            let is_output = i == self.layers.len() - 1;
            let act = if is_output {
                Activation::Tanh
            } else {
                self.activation
            };

            let mut outputs = Vec::with_capacity(layer.weights.len());
            for (w_row, bias) in layer.weights.iter().zip(layer.biases.iter()) {
                let mut sum = *bias;
                for (w, inp) in w_row.iter().zip(inputs.iter()) {
                    sum += w * inp;
                }
                outputs.push(act.apply(sum));
            }
            inputs = outputs;
        }

        inputs.first().copied().unwrap_or(0.0)
    }

    /// Update decision boundary grid
    fn update_grid(&mut self) {
        let n = self.grid_resolution;
        self.decision_grid.clear();

        for iy in 0..n {
            let mut row = Vec::with_capacity(n);
            let y = (iy as f32 / (n - 1) as f32) * 2.0 - 1.0;

            for ix in 0..n {
                let x = (ix as f32 / (n - 1) as f32) * 2.0 - 1.0;
                row.push(self.predict(x, y));
            }
            self.decision_grid.push(row);
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
                let pred = self.predict(p.x, p.y);
                (pred > 0.0 && p.label > 0) || (pred <= 0.0 && p.label <= 0)
            })
            .count();

        correct as f32 / self.points.len() as f32
    }

    /// Compute mean squared error loss
    fn compute_loss(&self) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        let sum: f32 = self
            .points
            .iter()
            .map(|p| {
                let pred = self.predict(p.x, p.y);
                let target = p.label as f32;
                (pred - target) * (pred - target)
            })
            .sum();

        sum / self.points.len() as f32
    }

    /// Get layer activations for visualization (call after forward pass)
    pub fn get_layer_activations(&self) -> Vec<Vec<f32>> {
        self.layers.iter().map(|l| l.activations.clone()).collect()
    }

    /// Get all weights for visualization
    pub fn get_weights(&self) -> Vec<Vec<Vec<f32>>> {
        self.layers.iter().map(|l| l.weights.clone()).collect()
    }

    /// Set number of hidden layers (1-4)
    pub fn set_hidden_layers(&mut self, count: usize) {
        let count = count.clamp(1, 4);
        let neurons = if self.layer_sizes.len() > 2 {
            self.layer_sizes[1]
        } else {
            4
        };

        self.layer_sizes = vec![2];
        for _ in 0..count {
            self.layer_sizes.push(neurons);
        }
        self.layer_sizes.push(1);

        self.init_network();
        self.step_count = 0;
        self.loss_history.clear();
    }

    /// Set neurons per hidden layer (1-8)
    pub fn set_neurons_per_layer(&mut self, neurons: usize) {
        let neurons = neurons.clamp(1, 8);

        // Keep first (input=2) and last (output=1)
        for i in 1..self.layer_sizes.len() - 1 {
            self.layer_sizes[i] = neurons;
        }

        self.init_network();
        self.step_count = 0;
        self.loss_history.clear();
    }
}

impl Demo for NeuralNetworkDemo {
    fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Rng::new(seed);
        self.step_count = 0;
        self.loss_history.clear();

        self.generate_data();
        self.init_network();
        self.update_grid();
        self.accuracy = self.compute_accuracy();
    }

    fn step(&mut self, _dt: f32) {
        if self.points.is_empty() || self.layers.is_empty() {
            return;
        }

        // Mini-batch stochastic gradient descent with backpropagation
        let batch: Vec<NNDataPoint> = (0..self.batch_size)
            .map(|_| {
                let idx = self.rng.range_int(0, self.points.len() as i32) as usize;
                self.points[idx]
            })
            .collect();

        // Gradient accumulators
        let mut weight_grads: Vec<Vec<Vec<f32>>> = self
            .layers
            .iter()
            .map(|l| l.weights.iter().map(|row| vec![0.0; row.len()]).collect())
            .collect();
        let mut bias_grads: Vec<Vec<f32>> = self
            .layers
            .iter()
            .map(|l| vec![0.0; l.biases.len()])
            .collect();

        for point in &batch {
            // Forward pass (stores activations)
            self.forward(point.x, point.y);

            // Collect all layer outputs for backprop
            let mut all_outputs: Vec<Vec<f32>> = Vec::with_capacity(self.layers.len() + 1);
            all_outputs.push(vec![point.x, point.y]); // input
            for layer in &self.layers {
                all_outputs.push(layer.activations.clone());
            }

            // Output error
            let output = all_outputs.last().unwrap()[0];
            let target = point.label as f32;
            let mut deltas: Vec<Vec<f32>> = vec![Vec::new(); self.layers.len()];

            // Output layer delta
            let d_loss = 2.0 * (output - target) / self.batch_size as f32;
            let d_act = Activation::Tanh.derivative(0.0, output);
            deltas[self.layers.len() - 1] = vec![d_loss * d_act];

            // Backpropagate
            for i in (0..self.layers.len() - 1).rev() {
                let next_layer = &self.layers[i + 1];
                let next_deltas = &deltas[i + 1];

                let mut layer_deltas = Vec::with_capacity(self.layers[i].activations.len());
                for j in 0..self.layers[i].activations.len() {
                    let mut sum = 0.0;
                    for (k, next_delta) in next_deltas.iter().enumerate() {
                        sum += next_layer.weights[k][j] * next_delta;
                    }
                    let act_out = self.layers[i].activations[j];
                    let d_act = self
                        .activation
                        .derivative(self.layers[i].pre_activations[j], act_out);
                    layer_deltas.push(sum * d_act);
                }
                deltas[i] = layer_deltas;
            }

            // Accumulate gradients
            for (l, layer_deltas) in deltas.iter().enumerate() {
                let inputs = &all_outputs[l];
                for (j, &delta) in layer_deltas.iter().enumerate() {
                    for (k, &inp) in inputs.iter().enumerate() {
                        weight_grads[l][j][k] += delta * inp;
                    }
                    bias_grads[l][j] += delta;
                }
            }
        }

        // Apply gradients
        for (l, layer) in self.layers.iter_mut().enumerate() {
            for (j, weights_j) in layer.weights.iter_mut().enumerate() {
                for (k, weight) in weights_j.iter_mut().enumerate() {
                    let reg = self.regularization * *weight;
                    *weight -= self.learning_rate * (weight_grads[l][j][k] + reg);
                }
                layer.biases[j] -= self.learning_rate * bias_grads[l][j];
            }
        }

        self.step_count += 1;

        // Update metrics periodically
        if self.step_count.is_multiple_of(10) {
            self.accuracy = self.compute_accuracy();
            let loss = self.compute_loss();
            self.loss_history.push(loss);
            if self.loss_history.len() > 100 {
                self.loss_history.remove(0);
            }
            self.update_grid();
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "learning_rate" => {
                self.learning_rate = value.clamp(0.001, 0.5);
                true
            }
            "regularization" => {
                self.regularization = value.clamp(0.0, 0.01);
                true
            }
            "activation" => {
                self.activation = Activation::from_index(value as usize);
                true
            }
            "dataset" => {
                self.dataset = NNDataset::from_index(value as usize);
                self.rng = Rng::new(self.seed);
                self.generate_data();
                self.init_network();
                self.step_count = 0;
                self.loss_history.clear();
                self.update_grid();
                self.accuracy = self.compute_accuracy();
                true
            }
            "hidden_layers" => {
                self.set_hidden_layers(value as usize);
                self.update_grid();
                self.accuracy = self.compute_accuracy();
                true
            }
            "neurons" => {
                self.set_neurons_per_layer(value as usize);
                self.update_grid();
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
                max: 0.5,
                step: 0.01,
                default: 0.03,
            },
            ParamMeta {
                name: "regularization",
                label: "Regularization",
                min: 0.0,
                max: 0.01,
                step: 0.001,
                default: 0.0,
            },
            ParamMeta {
                name: "activation",
                label: "Activation",
                min: 0.0,
                max: 3.0,
                step: 1.0,
                default: 0.0,
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
                name: "hidden_layers",
                label: "Hidden Layers",
                min: 1.0,
                max: 4.0,
                step: 1.0,
                default: 2.0,
            },
            ParamMeta {
                name: "neurons",
                label: "Neurons/Layer",
                min: 1.0,
                max: 8.0,
                step: 1.0,
                default: 4.0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_learns_circle() {
        let mut demo = NeuralNetworkDemo::default();
        demo.dataset = NNDataset::Circle;
        demo.learning_rate = 0.1;
        demo.reset(42);

        let initial_loss = demo.compute_loss();

        for _ in 0..1000 {
            demo.step(0.016);
        }

        let final_loss = demo.compute_loss();
        assert!(
            final_loss < initial_loss,
            "Loss should decrease: {} -> {}",
            initial_loss,
            final_loss
        );
    }

    #[test]
    fn test_network_architecture() {
        let mut demo = NeuralNetworkDemo::default();
        demo.layer_sizes = vec![2, 4, 3, 1];
        demo.reset(42);

        assert_eq!(demo.layers.len(), 3);
        assert_eq!(demo.layers[0].weights.len(), 4); // First hidden: 4 neurons
        assert_eq!(demo.layers[1].weights.len(), 3); // Second hidden: 3 neurons
        assert_eq!(demo.layers[2].weights.len(), 1); // Output: 1 neuron
    }

    #[test]
    fn test_deterministic() {
        let mut demo1 = NeuralNetworkDemo::default();
        let mut demo2 = NeuralNetworkDemo::default();

        demo1.reset(123);
        demo2.reset(123);

        for _ in 0..10 {
            demo1.step(0.016);
            demo2.step(0.016);
        }

        // Should have same accuracy
        assert!((demo1.compute_accuracy() - demo2.compute_accuracy()).abs() < 1e-6);
    }
}
