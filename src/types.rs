//! Core Types — Mobile AI Domain Models.
//!
//! This module defines the irredicible data structures used across the 
//! mobile AI framework. All types are optimized for low-overhead 
//! serialization (`serde`) and memory-efficient transfer on mobile hardware.

use serde::{Deserialize, Serialize};

/// QUERY: Represents a single user request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query {
    pub text: String,
    pub project_context: Option<String>,
    pub priority: u8, // Scale of 1-10
    pub timestamp: u64,
}

/// RESPONSE: The final output produced by the orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub text: String,
    pub route: RoutingDecision, // How the response was generated.
    pub confidence: f32,
    pub latency_ms: u64,
    pub metadata: ResponseMetadata,
}

/// ROUTING DECISION: The execution strategy chosen for a query.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoutingDecision {
    Local,   // Handled by on-device model.
    Remote,  // Dispatched to cloud API.
    Hybrid,  // Combined local/remote execution.
    Blocked, // Rejected by safety rules.
}

/// EVALUATION: The result of an expert system rule check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEvaluation {
    pub allowed: bool,
    pub reason: Option<String>,
    pub rule_id: Option<String>,
}
