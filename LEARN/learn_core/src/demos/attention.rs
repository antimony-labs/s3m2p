//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: attention.rs | LEARN/learn_core/src/demos/attention.rs
//! PURPOSE: Interactive attention mechanism visualization
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # Attention Demo
//!
//! Visualizes the attention mechanism used in Transformers:
//! - Query, Key, Value computation
//! - Attention weights (softmax of Q·K^T / sqrt(d))
//! - Weighted value aggregation
//! - Interactive token selection

use crate::{Demo, ParamMeta, Rng};

/// Example sentence for attention visualization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sentence {
    TheCatSat,
    TheAnimalDidnt,
    BankRiver,
    TranslationExample,
}

impl Sentence {
    pub fn tokens(&self) -> &'static [&'static str] {
        match self {
            Sentence::TheCatSat => &["The", "cat", "sat", "on", "the", "mat"],
            Sentence::TheAnimalDidnt => &["The", "animal", "didn't", "cross", "the", "road", "because", "it", "was", "tired"],
            Sentence::BankRiver => &["I", "went", "to", "the", "bank", "by", "the", "river"],
            Sentence::TranslationExample => &["Je", "suis", "un", "étudiant"],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Sentence::TheCatSat => "The cat sat...",
            Sentence::TheAnimalDidnt => "The animal didn't...",
            Sentence::BankRiver => "Bank/River",
            Sentence::TranslationExample => "Translation",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Sentence::TheCatSat,
            1 => Sentence::TheAnimalDidnt,
            2 => Sentence::BankRiver,
            _ => Sentence::TranslationExample,
        }
    }
}

/// Attention Demo
#[derive(Clone)]
pub struct AttentionDemo {
    // Sentence
    pub sentence: Sentence,
    pub tokens: Vec<String>,

    // Embedding dimension (simplified for visualization)
    pub embed_dim: usize,

    // Token embeddings (random for demo)
    pub embeddings: Vec<Vec<f32>>,

    // Learned weight matrices (simplified - normally these are much larger)
    pub w_query: Vec<Vec<f32>>,
    pub w_key: Vec<Vec<f32>>,
    pub w_value: Vec<Vec<f32>>,

    // Computed values per token
    pub queries: Vec<Vec<f32>>,
    pub keys: Vec<Vec<f32>>,
    pub values: Vec<Vec<f32>>,

    // Attention weights matrix [query_token][key_token]
    pub attention_weights: Vec<Vec<f32>>,

    // Output after attention (weighted values)
    pub outputs: Vec<Vec<f32>>,

    // Currently selected query token
    pub selected_query: usize,

    // Animation
    pub show_step: usize, // 0=embeddings, 1=Q/K/V, 2=attention, 3=output

    // Temperature for softmax (scaling factor)
    pub temperature: f32,

    // RNG
    rng: Rng,
    seed: u64,
}

impl Default for AttentionDemo {
    fn default() -> Self {
        Self {
            sentence: Sentence::TheCatSat,
            tokens: Vec::new(),
            embed_dim: 4,
            embeddings: Vec::new(),
            w_query: Vec::new(),
            w_key: Vec::new(),
            w_value: Vec::new(),
            queries: Vec::new(),
            keys: Vec::new(),
            values: Vec::new(),
            attention_weights: Vec::new(),
            outputs: Vec::new(),
            selected_query: 0,
            show_step: 0,
            temperature: 1.0,
            rng: Rng::new(42),
            seed: 42,
        }
    }
}

impl AttentionDemo {
    /// Initialize embeddings and weight matrices
    fn init_weights(&mut self) {
        let n_tokens = self.tokens.len();
        let d = self.embed_dim;

        // Random embeddings for each token
        self.embeddings = (0..n_tokens)
            .map(|_| (0..d).map(|_| self.rng.range(-1.0, 1.0)).collect())
            .collect();

        // Initialize Q, K, V weight matrices (d x d)
        let init_matrix = |rng: &mut Rng| -> Vec<Vec<f32>> {
            (0..d)
                .map(|_| (0..d).map(|_| rng.range(-0.5, 0.5)).collect())
                .collect()
        };

        self.w_query = init_matrix(&mut self.rng);
        self.w_key = init_matrix(&mut self.rng);
        self.w_value = init_matrix(&mut self.rng);
    }

    /// Compute Q, K, V for all tokens
    fn compute_qkv(&mut self) {
        let n_tokens = self.tokens.len();
        let d = self.embed_dim;

        self.queries = vec![vec![0.0; d]; n_tokens];
        self.keys = vec![vec![0.0; d]; n_tokens];
        self.values = vec![vec![0.0; d]; n_tokens];

        for t in 0..n_tokens {
            // Q = embedding * W_Q
            for i in 0..d {
                for j in 0..d {
                    self.queries[t][i] += self.embeddings[t][j] * self.w_query[i][j];
                    self.keys[t][i] += self.embeddings[t][j] * self.w_key[i][j];
                    self.values[t][i] += self.embeddings[t][j] * self.w_value[i][j];
                }
            }
        }
    }

    /// Compute attention weights using scaled dot-product attention
    fn compute_attention(&mut self) {
        let n_tokens = self.tokens.len();
        let d = self.embed_dim;
        let scale = (d as f32).sqrt() * self.temperature;

        self.attention_weights = vec![vec![0.0; n_tokens]; n_tokens];

        for q in 0..n_tokens {
            // Compute dot products (unnormalized attention)
            let mut scores = vec![0.0; n_tokens];
            for (k, key) in self.keys.iter().enumerate().take(n_tokens) {
                let mut dot = 0.0;
                for (i, &query_val) in self.queries[q].iter().enumerate().take(d) {
                    dot += query_val * key[i];
                }
                scores[k] = dot / scale;
            }

            // Softmax
            let max_score = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp_scores: Vec<f32> = scores.iter().map(|&s| (s - max_score).exp()).collect();
            let sum_exp: f32 = exp_scores.iter().sum();

            for (k, &exp_score) in exp_scores.iter().enumerate() {
                self.attention_weights[q][k] = exp_score / sum_exp;
            }
        }
    }

    /// Compute output as weighted sum of values
    fn compute_output(&mut self) {
        let n_tokens = self.tokens.len();
        let d = self.embed_dim;

        self.outputs = vec![vec![0.0; d]; n_tokens];

        for q in 0..n_tokens {
            for k in 0..n_tokens {
                let weight = self.attention_weights[q][k];
                for i in 0..d {
                    self.outputs[q][i] += weight * self.values[k][i];
                }
            }
        }
    }

    /// Get attention weights for the selected query token
    pub fn selected_attention(&self) -> Vec<f32> {
        if self.selected_query < self.attention_weights.len() {
            self.attention_weights[self.selected_query].clone()
        } else {
            Vec::new()
        }
    }

    /// Get token with highest attention from selected query
    pub fn most_attended_token(&self) -> Option<(usize, f32)> {
        self.selected_attention()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, &w)| (i, w))
    }

    /// Recompute everything
    fn recompute(&mut self) {
        if self.tokens.is_empty() {
            return;
        }
        self.compute_qkv();
        self.compute_attention();
        self.compute_output();
    }
}

impl Demo for AttentionDemo {
    fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Rng::new(seed);

        // Load tokens from sentence
        self.tokens = self.sentence.tokens().iter().map(|&s| s.to_string()).collect();
        self.selected_query = 0;
        self.show_step = 0;

        self.init_weights();
        self.recompute();
    }

    fn step(&mut self, _dt: f32) {
        // Auto-advance step for animation (optional)
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "sentence" => {
                self.sentence = Sentence::from_index(value as usize);
                self.tokens = self.sentence.tokens().iter().map(|&s| s.to_string()).collect();
                self.selected_query = 0;
                self.rng = Rng::new(self.seed);
                self.init_weights();
                self.recompute();
                true
            }
            "selected_query" => {
                let max_idx = self.tokens.len().saturating_sub(1);
                self.selected_query = (value as usize).min(max_idx);
                true
            }
            "temperature" => {
                self.temperature = value.clamp(0.1, 5.0);
                self.recompute();
                true
            }
            "show_step" => {
                self.show_step = (value as usize).min(3);
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "sentence",
                label: "Sentence",
                min: 0.0,
                max: 3.0,
                step: 1.0,
                default: 0.0,
            },
            ParamMeta {
                name: "selected_query",
                label: "Query Token",
                min: 0.0,
                max: 9.0,
                step: 1.0,
                default: 0.0,
            },
            ParamMeta {
                name: "temperature",
                label: "Temperature",
                min: 0.1,
                max: 5.0,
                step: 0.1,
                default: 1.0,
            },
            ParamMeta {
                name: "show_step",
                label: "Show Step",
                min: 0.0,
                max: 3.0,
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
    fn test_attention_weights_sum_to_one() {
        let mut demo = AttentionDemo::default();
        demo.reset(42);

        for q in 0..demo.attention_weights.len() {
            let sum: f32 = demo.attention_weights[q].iter().sum();
            assert!(
                (sum - 1.0).abs() < 1e-5,
                "Attention weights for token {} should sum to 1: {}",
                q, sum
            );
        }
    }

    #[test]
    fn test_temperature_affects_distribution() {
        let mut demo1 = AttentionDemo::default();
        demo1.temperature = 0.5; // Sharp
        demo1.reset(42);

        let mut demo2 = AttentionDemo::default();
        demo2.temperature = 2.0; // Smooth
        demo2.reset(42);

        // Find max attention weight
        let max1 = demo1.attention_weights[0].iter().cloned().fold(0.0f32, f32::max);
        let max2 = demo2.attention_weights[0].iter().cloned().fold(0.0f32, f32::max);

        // Lower temperature should produce more peaked distribution
        assert!(max1 > max2, "Lower temp should give sharper attention");
    }

    #[test]
    fn test_output_dimension_matches() {
        let mut demo = AttentionDemo::default();
        demo.reset(42);

        assert_eq!(demo.outputs.len(), demo.tokens.len());
        for output in &demo.outputs {
            assert_eq!(output.len(), demo.embed_dim);
        }
    }

    #[test]
    fn test_different_sentences() {
        let mut demo = AttentionDemo::default();

        for sentence in [Sentence::TheCatSat, Sentence::TheAnimalDidnt, Sentence::BankRiver] {
            demo.sentence = sentence;
            demo.reset(42);

            assert_eq!(demo.tokens.len(), sentence.tokens().len());
            assert_eq!(demo.attention_weights.len(), demo.tokens.len());
        }
    }
}
