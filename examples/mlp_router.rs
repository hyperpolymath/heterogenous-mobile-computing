//! MLP Router example - showing how to use neural network for routing
//!
//! Run with: cargo run --example mlp_router

use mobile_ai_orchestrator::mlp::MLP;
use mobile_ai_orchestrator::reservoir::encode_text;

fn main() {
    println!("MLP Router Example\n");

    // Create MLP for routing decisions
    // Input: 384-dim text encoding
    // Hidden: 100 → 50 neurons
    // Output: 3 classes (Local, Remote, Hybrid)
    let mlp = MLP::new(384, vec![100, 50], 3);

    println!("=== MLP Architecture ===");
    println!("Input size: {}", mlp.input_size());
    println!("Output size: {}", mlp.output_size());
    println!("Hidden layers: 100 → 50");

    // Example queries
    let queries = vec![
        "How do I iterate a HashMap?",
        "Can you formally prove this theorem?",
        "Help me debug this complex multi-threaded race condition",
    ];

    println!("\n=== Routing Decisions ===");
    for query in queries {
        // Encode query
        let encoding = encode_text(query, 384);

        // Forward through MLP
        let scores = mlp.forward(&encoding);

        // Apply softmax to get probabilities
        let probs = MLP::softmax(&scores);

        // Get decision
        let decision = MLP::argmax(&probs);

        let labels = ["Local", "Remote", "Hybrid"];
        println!("\nQuery: '{}'", query);
        println!("Probabilities:");
        for (i, (label, prob)) in labels.iter().zip(&probs).enumerate() {
            let marker = if i == decision { "→" } else { " " };
            println!("  {} {}: {:.3}", marker, label, prob);
        }
        println!("Decision: {}", labels[decision]);
    }

    // Example: Training step (simplified)
    println!("\n=== Training Example ===");
    let mut trainable_mlp = MLP::new(10, vec![20], 3);
    let input = vec![1.0; 10];
    let target = vec![0.0, 1.0, 0.0]; // Correct answer: Remote

    let loss_before = trainable_mlp.train_step(&input, &target, 0.01);
    println!("Loss before training: {:.4}", loss_before);

    // Train for a few steps
    for _ in 0..100 {
        trainable_mlp.train_step(&input, &target, 0.01);
    }

    let loss_after = trainable_mlp.train_step(&input, &target, 0.01);
    println!("Loss after 100 steps: {:.4}", loss_after);
    println!("Improvement: {:.4}", loss_before - loss_after);

    println!("\n✅ MLP router example completed!");
    println!("\nNote: In production, train on real user feedback data");
    println!("Collect: (query, user-corrected routing decision)");
    println!("Train: offline, deploy weights via model update");
}
