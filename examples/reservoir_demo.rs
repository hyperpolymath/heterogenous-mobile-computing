//! Reservoir Computing demonstration
//!
//! Run with: cargo run --example reservoir_demo

use mobile_ai_orchestrator::context::ContextManager;
use mobile_ai_orchestrator::reservoir::{encode_text, EchoStateNetwork};
use mobile_ai_orchestrator::{Query, Response, RoutingDecision};
use mobile_ai_orchestrator::types::{ResponseMetadata};

fn main() {
    println!("Reservoir Computing Demo\n");

    // Example 1: Standalone ESN
    println!("=== Example 1: Echo State Network ===");
    let mut esn = EchoStateNetwork::new(
        384,  // input size (text encoding dimension)
        1000, // reservoir size
        100,  // output size (compressed context)
        0.7,  // leak rate
        0.95, // spectral radius
    );

    // Encode and process text sequence
    let texts = vec![
        "Hello, how are you?",
        "I'm working on a Rust project",
        "Can you help with ownership?",
    ];

    println!("Processing text sequence...");
    for text in &texts {
        let encoding = encode_text(text, 384);
        let state = esn.update(&encoding);
        println!("  '{}' → state norm: {:.4}", text, vector_norm(&state));
    }

    // Example 2: Context Manager with Reservoir
    println!("\n=== Example 2: Context Manager with Reservoir ===");
    let mut cm = ContextManager::with_reservoir(true);

    println!("Adding conversation turns...");
    for (i, text) in texts.iter().enumerate() {
        let query = Query::new(*text);
        let response = Response {
            text: format!("Response to: {}", text),
            route: RoutingDecision::Local,
            confidence: 0.9,
            latency_ms: 10,
            metadata: ResponseMetadata {
                model: Some("test".to_string()),
                tokens: Some(10),
                cached: false,
            },
        };
        cm.add_turn(query, response);

        // Show reservoir state evolution
        if let Some(state) = cm.reservoir_state() {
            println!("  Turn {}: reservoir state norm = {:.4}", i + 1, vector_norm(&state));
        }
    }

    // Example 3: Context Snapshot with Reservoir
    println!("\n=== Example 3: Context Snapshot ===");
    let snapshot = cm.snapshot(10);
    println!("Project: {:?}", snapshot.project);
    println!("History size: {}", snapshot.history.len());
    if let Some(state) = snapshot.reservoir_state {
        println!("Reservoir state size: {}", state.len());
        println!("Reservoir state norm: {:.4}", vector_norm(&state));
        println!("First 10 values: {:?}", &state[..10]);
    }

    // Example 4: Reset and Restart
    println!("\n=== Example 4: Reset Reservoir ===");
    println!("State norm before reset: {:.4}", vector_norm(&cm.reservoir_state().unwrap()));
    cm.reset_reservoir();
    println!("State norm after reset: {:.4}", vector_norm(&cm.reservoir_state().unwrap()));

    println!("\n✅ Reservoir demo completed!");
}

fn vector_norm(v: &[f32]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}
