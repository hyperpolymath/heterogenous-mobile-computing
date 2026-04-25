// SPDX-License-Identifier: PMPL-1.0-or-later
//! Core Types — Mobile AI Domain Models.
//!
//! This module defines the irredicible data structures used across the 
//! mobile AI framework. All types are optimized for low-overhead 
//! serialization (`serde`) and memory-efficient transfer on mobile hardware.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// QUERY: Represents a single user request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query {
    pub text: String,
    pub project_context: Option<String>,
    pub priority: u8, // Scale of 1-10
    pub timestamp: u64,
}

impl Query {
    /// Create a new query with default priority and current timestamp.
    pub fn new(text: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock invariant: time is after UNIX_EPOCH (1970-01-01)")
            .as_secs();

        Self {
            text: text.into(),
            project_context: None,
            priority: 5,
            timestamp,
        }
    }
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

/// CONVERSATION TURN: A paired query-response interaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationTurn {
    pub query: Query,
    pub response: Response,
}

/// RESPONSE METADATA: Additional information about how a response was produced.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMetadata {
    pub model: Option<String>,
    pub tokens: Option<u32>,
    pub cached: bool,
}

/// CONTEXT SNAPSHOT: A frozen state of the conversation context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnapshot {
    pub project: Option<String>,
    pub history: Vec<ConversationTurn>,
    pub reservoir_state: Option<Vec<f32>>,
}
