//! Basic usage example of the Mobile AI Orchestrator
//!
//! Run with: cargo run --example basic_usage

use mobile_ai_orchestrator::{Orchestrator, Query};

fn main() {
    println!("Mobile AI Orchestrator - Basic Usage Example\n");

    // Create orchestrator
    let mut orch = Orchestrator::new();

    // Example 1: Simple query
    println!("=== Example 1: Simple Query ===");
    let query1 = Query::new("How do I create a HashMap in Rust?");
    match orch.process(query1) {
        Ok(response) => {
            println!("Response: {}", response.text);
            println!("Route: {:?}", response.route);
            println!("Confidence: {:.2}", response.confidence);
            println!("Latency: {}ms\n", response.latency_ms);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Complex query
    println!("=== Example 2: Complex Query ===");
    let query2 = Query::new("Can you formally prove the correctness of this sorting algorithm?");
    match orch.process(query2) {
        Ok(response) => {
            println!("Response: {}", response.text);
            println!("Route: {:?}", response.route);
        }
        Err(e) => println!("Error: {} (expected - network not enabled)\n", e),
    }

    // Example 3: Project context
    println!("=== Example 3: Project Context ===");
    orch.switch_project("rust-tutorial");
    let query3 = Query::new("What did we discuss about lifetimes?");
    match orch.process(query3) {
        Ok(response) => {
            println!("Project: {}", orch.current_project().unwrap());
            println!("Response: {}", response.text);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: Conversation history
    println!("\n=== Example 4: Conversation History ===");
    let history = orch.recent_history(3);
    println!("Recent conversations: {}", history.len());
    for (i, turn) in history.iter().enumerate() {
        println!("{}. Q: {}", i + 1, &turn.query.text[..50.min(turn.query.text.len())]);
    }

    // Example 5: Blocked query
    println!("\n=== Example 5: Safety - Blocked Query ===");
    let blocked_query = Query::new("Here's my api_key=sk-12345");
    match orch.process(blocked_query) {
        Ok(_) => println!("Should have been blocked!"),
        Err(e) => println!("Correctly blocked: {}", e),
    }

    println!("\n✅ All examples completed!");
}
