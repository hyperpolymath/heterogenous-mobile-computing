// SPDX-License-Identifier: PMPL-1.0-or-later
//! Multi-Layer Perceptron (MLP) — Neural Routing Kernel.
//!
//! This module implements a standard feedforward neural network designed 
//! to classify incoming queries for execution path optimization.
//!
//! ARCHITECTURE:
//! - Input: 384-dimensional feature vector (from `router.rs`).
//! - Hidden Layers: Reconfigurable depth and width (ReLU activation).
//! - Output: 3-dimensional logit vector [Local, Remote, Hybrid].
//!
//! DESIGN PILLARS:
//! 1. **Zero Unsafe**: Entirely memory-safe implementation using native Rust vectors.
//! 2. **Xavier Initialization**: Scaled random weights to ensure stable gradient 
//!    flow across layers.
//! 3. **Persistence**: Fully serializable via `serde` for on-device model storage.

use serde::{Deserialize, Serialize};

/// MLP: The neural network container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLP {
    input_size: usize,
    hidden_sizes: Vec<usize>,
    output_size: usize,
    weights: Vec<Vec<Vec<f32>>>, // [Layer][Row][Col]
    biases: Vec<Vec<f32>>,
}

impl MLP {
    /// Create a new MLP with given layer sizes.
    pub fn new(input_size: usize, hidden_sizes: Vec<usize>, output_size: usize) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut prev_size = input_size;

        // Initialize weights and biases
        for &hidden_size in &hidden_sizes {
            let mut layer_weights = vec![vec![0.0; prev_size]; hidden_size];
            let mut seed = 42u64;

            // Xavier initialization
            let limit = (6.0 / (prev_size + hidden_size) as f32).sqrt();
            for row in &mut layer_weights {
                for w in row {
                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                    let rand = ((seed / 65536) % 32768) as f32 / 32768.0;
                    *w = (rand - 0.5) * 2.0 * limit;
                }
            }

            weights.push(layer_weights);
            biases.push(vec![0.0; hidden_size]);
            prev_size = hidden_size;
        }

        // Output layer
        let mut output_weights = vec![vec![0.0; prev_size]; output_size];
        let mut seed = 42u64;
        let limit = (6.0 / (prev_size + output_size) as f32).sqrt();
        for row in &mut output_weights {
            for w in row {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let rand = ((seed / 65536) % 32768) as f32 / 32768.0;
                *w = (rand - 0.5) * 2.0 * limit;
            }
        }

        weights.push(output_weights);
        biases.push(vec![0.0; output_size]);

        Self {
            input_size,
            hidden_sizes,
            output_size,
            weights,
            biases,
        }
    }

    /// FORWARD: Computes the network output for a given input vector.
    /// Applies ReLU activation to hidden layers and returns raw logits.
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        let mut activation = input.to_vec();

        // Forward pass through all layers
        for (i, layer_weights) in self.weights.iter().enumerate() {
            let is_output = i == self.weights.len() - 1;
            let mut next_activation = self.biases[i].clone();

            // Matrix-vector multiplication
            for (j, weights_row) in layer_weights.iter().enumerate() {
                let mut sum = 0.0;
                for (k, w) in weights_row.iter().enumerate() {
                    sum += w * activation[k];
                }
                next_activation[j] += sum;
            }

            // Apply activation function
            if !is_output {
                // ReLU for hidden layers
                activation = next_activation.iter().map(|&x| x.max(0.0)).collect();
            } else {
                // Linear for output layer
                activation = next_activation;
            }
        }

        activation
    }

    /// SOFTMAX: Normalizes logits into a probability distribution.
    /// Returns a vector where `sum(values) == 1.0`.
    pub fn softmax(values: &[f32]) -> Vec<f32> {
        let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exp_values: Vec<f32> = values.iter().map(|v| (v - max).exp()).collect();
        let sum: f32 = exp_values.iter().sum();

        if sum > 0.0 {
            exp_values.iter().map(|e| e / sum).collect()
        } else {
            exp_values
        }
    }

    /// Compute loss and gradients via backpropagation.
    pub fn backward(&self, input: &[f32], target: &[f32]) -> (f32, Vec<Vec<Vec<f32>>>) {
        let output = self.forward(input);

        // Cross-entropy loss
        let mut loss = 0.0;
        for (o, t) in output.iter().zip(target.iter()) {
            let o = o.clamp(1e-6, 1.0 - 1e-6);
            loss -= t * o.ln();
        }

        // Placeholder gradients (proper backprop deferred to Phase 2)
        let gradients = vec![vec![vec![0.0; input.len()]; self.output_size]; self.weights.len()];

        (loss, gradients)
    }

    /// Update weights using gradients.
    pub fn update(&mut self, _gradients: &[Vec<Vec<f32>>], _learning_rate: f32) {
        // Phase 2 implementation
    }

    /// Argmax: Return the index of the maximum value.
    pub fn argmax(values: &[f32]) -> usize {
        values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
