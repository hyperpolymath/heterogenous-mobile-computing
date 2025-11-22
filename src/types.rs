//! Core type definitions for the mobile AI orchestrator.
//!
//! All types are designed to be:
//! - Serializable (for storage/transmission)
//! - Zero-copy where possible (mobile performance)
//! - Type-safe (compile-time guarantees)

use serde::{Deserialize, Serialize};

/// A user query to be processed by the orchestrator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query {
    /// The actual query text
    pub text: String,
    /// Optional project context identifier
    pub project_context: Option<String>,
    /// Priority level (1-10, higher = more urgent)
    pub priority: u8,
    /// Timestamp of query creation (Unix timestamp)
    pub timestamp: u64,
}

impl Query {
    /// Create a new query with default settings
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            project_context: None,
            priority: 5,
            timestamp: current_timestamp(),
        }
    }

    /// Create a query with specific project context
    pub fn with_context(text: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            project_context: Some(context.into()),
            priority: 5,
            timestamp: current_timestamp(),
        }
    }

    /// Check if query is high priority (>7)
    pub fn is_high_priority(&self) -> bool {
        self.priority > 7
    }
}

/// Response from the AI system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// The generated response text
    pub text: String,
    /// Which route was used
    pub route: RoutingDecision,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Processing time in milliseconds
    pub latency_ms: u64,
    /// Metadata about the response
    pub metadata: ResponseMetadata,
}

/// Additional metadata about a response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMetadata {
    /// Model used (if local inference)
    pub model: Option<String>,
    /// Number of tokens generated
    pub tokens: Option<usize>,
    /// Whether this was cached
    pub cached: bool,
}

/// Routing decision made by the router
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoutingDecision {
    /// Process locally with on-device model
    Local,
    /// Send to remote API (Claude, Mistral, etc.)
    Remote,
    /// Hybrid approach (local preprocessing + remote)
    Hybrid,
    /// Blocked by expert system rules
    Blocked,
}

impl RoutingDecision {
    /// Check if this decision requires network
    pub fn requires_network(&self) -> bool {
        matches!(self, Self::Remote | Self::Hybrid)
    }
}

/// Expert system rule evaluation result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEvaluation {
    /// Whether the query is allowed
    pub allowed: bool,
    /// Reason for blocking (if blocked)
    pub reason: Option<String>,
    /// Which rule triggered (if any)
    pub rule_id: Option<String>,
}

/// Context snapshot from the context manager
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContextSnapshot {
    /// Current project context
    pub project: Option<String>,
    /// Recent conversation history (last N messages)
    pub history: Vec<ConversationTurn>,
    /// Temporal state from reservoir (future: Phase 2)
    pub reservoir_state: Option<Vec<f32>>,
}

/// A single turn in the conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationTurn {
    /// Query from user
    pub query: Query,
    /// Response from system
    pub response: Response,
}

/// Router configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouterConfig {
    /// Confidence threshold for local routing (0.0 - 1.0)
    pub local_threshold: f32,
    /// Maximum query length for local processing (characters)
    pub max_local_length: usize,
    /// Keywords that suggest complex reasoning
    pub complex_keywords: Vec<String>,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            local_threshold: 0.7,
            max_local_length: 500,
            complex_keywords: vec![
                "prove".to_string(),
                "verify".to_string(),
                "formal".to_string(),
                "complex".to_string(),
            ],
        }
    }
}

/// Get current Unix timestamp in seconds
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_creation() {
        let query = Query::new("test query");
        assert_eq!(query.text, "test query");
        assert_eq!(query.priority, 5);
        assert!(query.timestamp > 0);
    }

    #[test]
    fn test_query_with_context() {
        let query = Query::with_context("test", "project-1");
        assert_eq!(query.project_context, Some("project-1".to_string()));
    }

    #[test]
    fn test_priority_check() {
        let mut query = Query::new("test");
        query.priority = 8;
        assert!(query.is_high_priority());

        query.priority = 6;
        assert!(!query.is_high_priority());
    }

    #[test]
    fn test_routing_decision_network() {
        assert!(RoutingDecision::Remote.requires_network());
        assert!(RoutingDecision::Hybrid.requires_network());
        assert!(!RoutingDecision::Local.requires_network());
        assert!(!RoutingDecision::Blocked.requires_network());
    }

    #[test]
    fn test_router_config_default() {
        let config = RouterConfig::default();
        assert_eq!(config.local_threshold, 0.7);
        assert_eq!(config.max_local_length, 500);
        assert!(!config.complex_keywords.is_empty());
    }
}
