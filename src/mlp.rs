//! Multi-Layer Perceptron for router decision making
//!
//! Simple feedforward neural network for query routing.
//! Replaces heuristic-based routing with learned patterns.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Simple Multi-Layer Perceptron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLP {
    /// Input layer size
    input_size: usize,
    /// Hidden layer sizes
    hidden_sizes: Vec<usize>,
    /// Output layer size
    output_size: usize,
    /// Weights for each layer
    weights: Vec<Vec<Vec<f32>>>,
    /// Biases for each layer
    biases: Vec<Vec<f32>>,
}

impl MLP {
    /// Create a new MLP with random initialization
    ///
    /// # Arguments
    ///
    /// * `input_size` - Size of input vector
    /// * `hidden_sizes` - Sizes of hidden layers
    /// * `output_size` - Size of output vector
    ///
    /// # Examples
    ///
    /// ```
    /// use mobile_ai_orchestrator::mlp::MLP;
    ///
    /// // Create MLP: 10 inputs → 50 hidden → 20 hidden → 3 outputs
    /// let mlp = MLP::new(10, vec![50, 20], 3);
    /// ```
    pub fn new(input_size: usize, hidden_sizes: Vec<usize>, output_size: usize) -> Self {
        let mut mlp = Self {
            input_size,
            hidden_sizes: hidden_sizes.clone(),
            output_size,
            weights: Vec::new(),
            biases: Vec::new(),
        };

        // Initialize layers
        let mut layer_sizes = vec![input_size];
        layer_sizes.extend(hidden_sizes);
        layer_sizes.push(output_size);

        // Xavier initialization for weights
        let mut seed = 123u64;
        for i in 0..layer_sizes.len() - 1 {
            let rows = layer_sizes[i + 1];
            let cols = layer_sizes[i];

            let mut layer_weights = vec![vec![0.0; cols]; rows];
            let scale = (2.0 / cols as f32).sqrt();

            for row in &mut layer_weights {
                for weight in row {
                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                    let rand = ((seed / 65536) % 32768) as f32 / 32768.0;
                    *weight = (rand - 0.5) * 2.0 * scale;
                }
            }

            mlp.weights.push(layer_weights);
            mlp.biases.push(vec![0.0; rows]);
        }

        mlp
    }

    /// Forward pass through the network
    ///
    /// # Arguments
    ///
    /// * `input` - Input vector
    ///
    /// # Returns
    ///
    /// Output vector after forward pass
    ///
    /// # Panics
    ///
    /// Panics if input size doesn't match network input size
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        assert_eq!(
            input.len(),
            self.input_size,
            "Input size mismatch: expected {}, got {}",
            self.input_size,
            input.len()
        );

        let mut activation = input.to_vec();

        // Forward through each layer
        for (layer_weights, layer_biases) in self.weights.iter().zip(&self.biases) {
            let mut next_activation = vec![0.0; layer_weights.len()];

            for (i, (weights_row, bias)) in layer_weights.iter().zip(layer_biases).enumerate() {
                let mut sum = *bias;
                for (w, a) in weights_row.iter().zip(&activation) {
                    sum += w * a;
                }
                next_activation[i] = sum;
            }

            // ReLU activation for hidden layers, no activation for output
            if layer_weights.len() != self.output_size {
                for a in &mut next_activation {
                    *a = a.max(0.0); // ReLU
                }
            }

            activation = next_activation;
        }

        activation
    }

    /// Softmax activation for output layer
    ///
    /// Converts raw scores to probabilities
    pub fn softmax(values: &[f32]) -> Vec<f32> {
        let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = values.iter().map(|&v| (v - max).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.iter().map(|&e| e / sum).collect()
    }

    /// Get the index of the maximum value
    pub fn argmax(values: &[f32]) -> usize {
        values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Simple training via gradient descent (very basic)
    ///
    /// This is a placeholder - in production use a proper training framework
    /// like `tch-rs` (PyTorch bindings) or `burn` (pure Rust)
    pub fn train_step(
        &mut self,
        input: &[f32],
        target: &[f32],
        learning_rate: f32,
    ) -> f32 {
        // Forward pass
        let output = self.forward(input);

        // Compute MSE loss
        let loss: f32 = output
            .iter()
            .zip(target)
            .map(|(o, t)| (o - t).powi(2))
            .sum::<f32>()
            / output.len() as f32;

        // Simplified gradient descent (not proper backprop)
        // In production: use automatic differentiation
        for layer_weights in &mut self.weights {
            for row in layer_weights {
                for weight in row {
                    *weight *= 0.9999; // Simple weight decay
                }
            }
        }

        loss
    }

    /// Get input size
    pub fn input_size(&self) -> usize {
        self.input_size
    }

    /// Get output size
    pub fn output_size(&self) -> usize {
        self.output_size
    }

    /// Backward pass (simplified backpropagation)
    /// Returns (loss, gradients)
    pub fn backward(&self, input: &[f32], target: &[f32]) -> (f32, Vec<Vec<Vec<f32>>>) {
        // Forward pass to get activations
        let mut activations = vec![input.to_vec()];
        let mut current = input.to_vec();

        for (layer_weights, layer_biases) in self.weights.iter().zip(&self.biases) {
            let mut next = vec![0.0; layer_weights.len()];

            for (i, (weights_row, bias)) in layer_weights.iter().zip(layer_biases).enumerate() {
                let mut sum = *bias;
                for (w, a) in weights_row.iter().zip(&current) {
                    sum += w * a;
                }
                next[i] = sum;
            }

            // ReLU for hidden layers, linear for output
            if layer_weights.len() != self.output_size {
                for a in &mut next {
                    *a = a.max(0.0);
                }
            }

            activations.push(next.clone());
            current = next;
        }

        // Compute loss (cross-entropy for classification)
        let output = &activations[activations.len() - 1];
        let mut loss = 0.0;
        for (o, t) in output.iter().zip(target) {
            loss += -t * o.max(1e-7).ln();
        }

        // Backward pass (simplified)
        let mut weight_gradients = vec![vec![vec![0.0; 0]; 0]; self.weights.len()];
        let mut bias_gradients = vec![vec![0.0; 0]; self.biases.len()];

        // Initialize with same structure as weights/biases
        for (i, layer_weights) in self.weights.iter().enumerate() {
            weight_gradients[i] = vec![vec![0.0; layer_weights[0].len()]; layer_weights.len()];
            bias_gradients[i] = vec![0.0; self.biases[i].len()];
        }

        // Output layer gradient
        let mut delta: Vec<f32> = output
            .iter()
            .zip(target)
            .map(|(o, t)| o - t)
            .collect();

        // Backpropagate through layers
        for layer_idx in (0..self.weights.len()).rev() {
            let prev_activation = &activations[layer_idx];

            // Compute weight gradients
            for (i, row_gradient) in weight_gradients[layer_idx].iter_mut().enumerate() {
                for (j, grad) in row_gradient.iter_mut().enumerate() {
                    *grad = delta[i] * prev_activation[j];
                }
            }

            // Compute bias gradients
            for (i, grad) in bias_gradients[layer_idx].iter_mut().enumerate() {
                *grad = delta[i];
            }

            // Propagate delta to previous layer
            if layer_idx > 0 {
                let mut new_delta = vec![0.0; prev_activation.len()];

                for (i, weights_row) in self.weights[layer_idx].iter().enumerate() {
                    for (j, &weight) in weights_row.iter().enumerate() {
                        new_delta[j] += delta[i] * weight;
                    }
                }

                // Apply ReLU derivative
                let pre_activation = &activations[layer_idx];
                for (i, d) in new_delta.iter_mut().enumerate() {
                    if pre_activation[i] <= 0.0 {
                        *d = 0.0;
                    }
                }

                delta = new_delta;
            }
        }

        (loss, weight_gradients)
    }

    /// Update weights using gradients
    pub fn update(&mut self, gradients: &[Vec<Vec<f32>>], learning_rate: f32) {
        for (layer_idx, layer_gradients) in gradients.iter().enumerate() {
            for (i, row_gradients) in layer_gradients.iter().enumerate() {
                for (j, &gradient) in row_gradients.iter().enumerate() {
                    self.weights[layer_idx][i][j] -= learning_rate * gradient;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlp_creation() {
        let mlp = MLP::new(10, vec![20], 3);
        assert_eq!(mlp.input_size(), 10);
        assert_eq!(mlp.output_size(), 3);
    }

    #[test]
    fn test_mlp_forward() {
        let mlp = MLP::new(10, vec![20], 3);
        let input = vec![1.0; 10];
        let output = mlp.forward(&input);
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_softmax() {
        let values = vec![1.0, 2.0, 3.0];
        let probs = MLP::softmax(&values);

        // Should sum to 1.0
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);

        // Should be monotonic (higher input → higher prob)
        assert!(probs[2] > probs[1]);
        assert!(probs[1] > probs[0]);
    }

    #[test]
    fn test_argmax() {
        let values = vec![0.1, 0.8, 0.3];
        assert_eq!(MLP::argmax(&values), 1);

        let values2 = vec![5.0, 2.0, 1.0];
        assert_eq!(MLP::argmax(&values2), 0);
    }

    #[test]
    fn test_mlp_train_step() {
        let mut mlp = MLP::new(5, vec![10], 2);
        let input = vec![1.0; 5];
        let target = vec![0.0, 1.0];

        let loss = mlp.train_step(&input, &target, 0.01);
        assert!(loss >= 0.0);
    }

    #[test]
    #[should_panic(expected = "Input size mismatch")]
    fn test_mlp_forward_wrong_size() {
        let mlp = MLP::new(10, vec![20], 3);
        let wrong_input = vec![1.0; 5];
        mlp.forward(&wrong_input);
    }

    #[test]
    fn test_mlp_serialization() {
        let mlp = MLP::new(10, vec![20], 3);
        let json = serde_json::to_string(&mlp).unwrap();
        let deserialized: MLP = serde_json::from_str(&json).unwrap();

        assert_eq!(mlp.input_size, deserialized.input_size);
        assert_eq!(mlp.output_size, deserialized.output_size);
    }

    #[test]
    fn test_mlp_multi_layer() {
        let mlp = MLP::new(10, vec![50, 30, 20], 3);
        assert_eq!(mlp.weights.len(), 4); // 3 hidden + 1 output
        assert_eq!(mlp.biases.len(), 4);

        let input = vec![0.5; 10];
        let output = mlp.forward(&input);
        assert_eq!(output.len(), 3);
    }
}
