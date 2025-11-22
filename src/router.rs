//! Router: MLP-based decision engine for query routing
//!
//! Determines whether queries should be:
//! - Processed locally (on-device inference)
//! - Sent to remote API (Claude, Mistral, etc.)
//! - Hybrid (local preprocessing + remote)
//! - Blocked (safety/policy violations)
//!
//! Phase 1: Simple heuristic-based routing
//! Phase 2+: Actual MLP with learned weights

use crate::types::{Query, RouterConfig, RoutingDecision};

/// Router for determining query routing strategy
#[derive(Debug, Clone)]
pub struct Router {
    config: RouterConfig,
}

impl Router {
    /// Create a new router with default configuration
    pub fn new() -> Self {
        Self {
            config: RouterConfig::default(),
        }
    }

    /// Create a router with custom configuration
    pub fn with_config(config: RouterConfig) -> Self {
        Self { config }
    }

    /// Route a query to the appropriate processing path
    ///
    /// # Decision Logic (Phase 1 - Heuristic)
    ///
    /// 1. Check query length - long queries → Remote
    /// 2. Check for complex keywords → Remote
    /// 3. Check project context - known projects → Local (cached knowledge)
    /// 4. Default: Local for simple queries
    ///
    /// # Future (Phase 2+)
    ///
    /// Replace with actual MLP:
    /// - Input: query embedding + context features
    /// - Hidden layers: 2-3 layers, ~50-100 neurons each
    /// - Output: [local_score, remote_score, hybrid_score]
    /// - Decision: argmax(output)
    pub fn route(&self, query: &Query) -> (RoutingDecision, f32) {
        // Rule 1: Very long queries are complex
        if query.text.len() > self.config.max_local_length {
            return (RoutingDecision::Remote, 0.9);
        }

        // Rule 2: Check for complex reasoning keywords
        let has_complex_keyword = self
            .config
            .complex_keywords
            .iter()
            .any(|kw| query.text.to_lowercase().contains(&kw.to_lowercase()));

        if has_complex_keyword {
            return (RoutingDecision::Remote, 0.85);
        }

        // Rule 3: Very short queries are usually simple
        if query.text.len() < 50 {
            return (RoutingDecision::Local, 0.8);
        }

        // Rule 4: Questions with "how", "what", "why" - depends on length
        let is_question = query.text.contains('?');
        if is_question && query.text.len() < 200 {
            return (RoutingDecision::Local, 0.75);
        }

        // Rule 5: High priority queries with context might need hybrid
        if query.is_high_priority() && query.project_context.is_some() {
            return (RoutingDecision::Hybrid, 0.7);
        }

        // Default: local processing
        (RoutingDecision::Local, self.config.local_threshold)
    }

    /// Update router configuration
    pub fn update_config(&mut self, config: RouterConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &RouterConfig {
        &self.config
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Future Phase 2: Actual MLP implementation placeholder
#[allow(dead_code)]
struct SimpleMLP {
    // Placeholder for future MLP implementation
    // Will use ndarray or similar for matrix operations
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = Router::new();
        assert_eq!(router.config.local_threshold, 0.7);
    }

    #[test]
    fn test_short_query_routes_local() {
        let router = Router::new();
        let query = Query::new("Hello");
        let (decision, confidence) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Local);
        assert!(confidence > 0.7);
    }

    #[test]
    fn test_long_query_routes_remote() {
        let router = Router::new();
        let long_text = "a".repeat(600);
        let query = Query::new(long_text);
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Remote);
    }

    #[test]
    fn test_complex_keyword_routes_remote() {
        let router = Router::new();
        let query = Query::new("Can you prove this theorem formally?");
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Remote);
    }

    #[test]
    fn test_simple_question_routes_local() {
        let router = Router::new();
        let query = Query::new("How do I iterate a HashMap in Rust?");
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Local);
    }

    #[test]
    fn test_high_priority_with_context_routes_hybrid() {
        let router = Router::new();
        let mut query = Query::with_context(
            "Help with this bug in the type system implementation for the borrow checker",
            "project-oblibeny"
        );
        query.priority = 9;
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Hybrid);
    }

    #[test]
    fn test_config_update() {
        let mut router = Router::new();
        let mut new_config = RouterConfig::default();
        new_config.local_threshold = 0.8;
        router.update_config(new_config);
        assert_eq!(router.config().local_threshold, 0.8);
    }
}
