//! Spiking Neural Network for ultra-low-power event detection
//!
//! Implements event-driven neural computation for:
//! - Wake word detection
//! - Context switching detection
//! - Proactive assistance triggers
//!
//! SNNs use discrete spikes instead of continuous activations,
//! enabling very low power consumption on appropriate hardware.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Leaky Integrate-and-Fire neuron model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LIFNeuron {
    /// Membrane potential
    pub potential: f32,
    /// Resting potential
    pub rest_potential: f32,
    /// Threshold for firing
    pub threshold: f32,
    /// Membrane time constant
    pub tau: f32,
    /// Refractory period counter
    pub refractory: u32,
}

impl LIFNeuron {
    /// Create a new LIF neuron
    pub fn new(threshold: f32, tau: f32) -> Self {
        Self {
            potential: 0.0,
            rest_potential: 0.0,
            threshold,
            tau,
            refractory: 0,
        }
    }

    /// Update neuron state and check for spike
    ///
    /// # Arguments
    ///
    /// * `input_current` - Incoming current from synapses
    /// * `dt` - Time step (typically 1ms)
    ///
    /// # Returns
    ///
    /// `true` if neuron spiked, `false` otherwise
    pub fn update(&mut self, input_current: f32, dt: f32) -> bool {
        // Refractory period
        if self.refractory > 0 {
            self.refractory -= 1;
            return false;
        }

        // Leaky integration: dV/dt = -(V - V_rest)/tau + I
        let dv = (-(self.potential - self.rest_potential) / self.tau + input_current) * dt;
        self.potential += dv;

        // Check for spike
        if self.potential >= self.threshold {
            self.potential = self.rest_potential;
            self.refractory = 5; // 5ms refractory period
            true
        } else {
            false
        }
    }

    /// Reset neuron to resting state
    pub fn reset(&mut self) {
        self.potential = self.rest_potential;
        self.refractory = 0;
    }
}

/// Simple Spiking Neural Network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikingNetwork {
    /// Input layer neurons
    input_neurons: Vec<LIFNeuron>,
    /// Hidden layer neurons
    hidden_neurons: Vec<LIFNeuron>,
    /// Output layer neurons
    output_neurons: Vec<LIFNeuron>,
    /// Synaptic weights (input → hidden)
    weights_ih: Vec<Vec<f32>>,
    /// Synaptic weights (hidden → output)
    weights_ho: Vec<Vec<f32>>,
    /// Spike history (for analysis)
    spike_counts: Vec<usize>,
}

impl SpikingNetwork {
    /// Create a new spiking neural network
    ///
    /// # Arguments
    ///
    /// * `n_input` - Number of input neurons
    /// * `n_hidden` - Number of hidden neurons
    /// * `n_output` - Number of output neurons
    pub fn new(n_input: usize, n_hidden: usize, n_output: usize) -> Self {
        let input_neurons = (0..n_input)
            .map(|_| LIFNeuron::new(1.0, 10.0))
            .collect();

        let hidden_neurons = (0..n_hidden)
            .map(|_| LIFNeuron::new(1.0, 10.0))
            .collect();

        let output_neurons = (0..n_output)
            .map(|_| LIFNeuron::new(1.0, 10.0))
            .collect();

        // Random sparse weights
        let mut seed = 789u64;
        let mut weights_ih = vec![vec![0.0; n_input]; n_hidden];
        for row in &mut weights_ih {
            for w in row {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let rand = ((seed / 65536) % 32768) as f32 / 32768.0;
                if rand < 0.2 {
                    // 20% connectivity
                    *w = (rand - 0.5) * 0.5;
                }
            }
        }

        let mut weights_ho = vec![vec![0.0; n_hidden]; n_output];
        for row in &mut weights_ho {
            for w in row {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let rand = ((seed / 65536) % 32768) as f32 / 32768.0;
                if rand < 0.2 {
                    *w = (rand - 0.5) * 0.5;
                }
            }
        }

        Self {
            input_neurons,
            hidden_neurons,
            output_neurons,
            weights_ih,
            weights_ho,
            spike_counts: vec![0; n_output],
        }
    }

    /// Process one time step
    ///
    /// # Arguments
    ///
    /// * `input_spikes` - Binary array indicating which input neurons spike
    /// * `dt` - Time step (typically 1ms)
    ///
    /// # Returns
    ///
    /// Vector of output spike indicators
    pub fn step(&mut self, input_spikes: &[bool], dt: f32) -> Vec<bool> {
        assert_eq!(input_spikes.len(), self.input_neurons.len());

        // Update input layer
        for (i, neuron) in self.input_neurons.iter_mut().enumerate() {
            if input_spikes[i] {
                neuron.update(2.0, dt);
            } else {
                neuron.update(0.0, dt);
            }
        }

        // Compute hidden layer currents
        let mut hidden_currents = vec![0.0; self.hidden_neurons.len()];
        for (i, neuron) in self.input_neurons.iter().enumerate() {
            if neuron.potential > 0.5 {
                // Approximate spike
                for (h, current) in hidden_currents.iter_mut().enumerate() {
                    *current += self.weights_ih[h][i];
                }
            }
        }

        // Update hidden layer
        for (neuron, &current) in self.hidden_neurons.iter_mut().zip(&hidden_currents) {
            neuron.update(current, dt);
        }

        // Compute output layer currents
        let mut output_currents = vec![0.0; self.output_neurons.len()];
        for (h, neuron) in self.hidden_neurons.iter().enumerate() {
            if neuron.potential > 0.5 {
                for (o, current) in output_currents.iter_mut().enumerate() {
                    *current += self.weights_ho[o][h];
                }
            }
        }

        // Update output layer
        let mut output_spikes = vec![false; self.output_neurons.len()];
        for (i, (neuron, &current)) in self
            .output_neurons
            .iter_mut()
            .zip(&output_currents)
            .enumerate()
        {
            if neuron.update(current, dt) {
                output_spikes[i] = true;
                self.spike_counts[i] += 1;
            }
        }

        output_spikes
    }

    /// Reset all neurons
    pub fn reset(&mut self) {
        for neuron in &mut self.input_neurons {
            neuron.reset();
        }
        for neuron in &mut self.hidden_neurons {
            neuron.reset();
        }
        for neuron in &mut self.output_neurons {
            neuron.reset();
        }
        self.spike_counts.fill(0);
    }

    /// Get spike counts for output neurons
    pub fn spike_counts(&self) -> &[usize] {
        &self.spike_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lif_neuron_creation() {
        let neuron = LIFNeuron::new(1.0, 10.0);
        assert_eq!(neuron.potential, 0.0);
        assert_eq!(neuron.threshold, 1.0);
    }

    #[test]
    fn test_lif_neuron_spike() {
        let mut neuron = LIFNeuron::new(1.0, 10.0);

        // Strong input should cause spike
        let spiked = neuron.update(5.0, 1.0);
        assert!(spiked || neuron.potential > 0.8);
    }

    #[test]
    fn test_lif_neuron_refractory() {
        let mut neuron = LIFNeuron::new(0.5, 10.0);

        // Cause a spike
        neuron.update(10.0, 1.0);

        // During refractory period, no spike
        let spiked = neuron.update(10.0, 1.0);
        assert!(!spiked);
    }

    #[test]
    fn test_lif_neuron_reset() {
        let mut neuron = LIFNeuron::new(10.0, 10.0); // High threshold to avoid spike
        neuron.update(2.0, 1.0); // Input below threshold
        // After update: dv = (-(0-0)/10 + 2.0) * 1.0 = 2.0, potential = 2.0
        assert!(neuron.potential > 0.0, "Potential should be positive after update");

        neuron.reset();
        assert_eq!(neuron.potential, 0.0);
    }

    #[test]
    fn test_spiking_network_creation() {
        let snn = SpikingNetwork::new(10, 20, 3);
        assert_eq!(snn.input_neurons.len(), 10);
        assert_eq!(snn.hidden_neurons.len(), 20);
        assert_eq!(snn.output_neurons.len(), 3);
    }

    #[test]
    fn test_spiking_network_step() {
        let mut snn = SpikingNetwork::new(10, 20, 3);
        let input = vec![true, false, true, false, false, false, false, false, false, false];

        let output = snn.step(&input, 1.0);
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_spiking_network_reset() {
        let mut snn = SpikingNetwork::new(10, 20, 3);
        let input = vec![true; 10];

        // Run for many steps with strong input to ensure spikes occur
        for _ in 0..100 {
            snn.step(&input, 1.0);
        }

        // After reset, all spike counts should be zero regardless of prior state
        snn.reset();
        assert!(
            snn.spike_counts().iter().all(|&c| c == 0),
            "Spike counts should be zero after reset"
        );
    }

    #[test]
    fn test_spiking_network_serialization() {
        let snn = SpikingNetwork::new(10, 20, 3);
        let json = serde_json::to_string(&snn).unwrap();
        let _deserialized: SpikingNetwork = serde_json::from_str(&json).unwrap();
    }
}
