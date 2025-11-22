//! Orchestrator: Main coordination layer
//!
//! Coordinates all components:
//! - Router: decides local vs. remote
//! - Expert System: enforces safety rules
//! - Context Manager: provides conversation state
//! - Inference: executes the decision
//!
//! This is the main API for the mobile AI system.

use crate::{
    context::ContextManager,
    expert::ExpertSystem,
    router::Router,
    types::{Query, Response, ResponseMetadata, RoutingDecision},
};

/// Main orchestrator coordinating all AI components
#[derive(Debug)]
pub struct Orchestrator {
    router: Router,
    expert: ExpertSystem,
    context: ContextManager,
}

impl Orchestrator {
    /// Create a new orchestrator with default configuration
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            expert: ExpertSystem::new(),
            context: ContextManager::new(),
        }
    }

    /// Process a query through the full pipeline
    ///
    /// # Pipeline
    ///
    /// 1. Expert system evaluation (safety check)
    /// 2. Router decision (local/remote/hybrid/blocked)
    /// 3. Context retrieval (augment query with history)
    /// 4. Inference (execute the decision)
    /// 5. Context update (save conversation turn)
    ///
    /// # Returns
    ///
    /// - `Ok(Response)` if query was processed successfully
    /// - `Err(String)` if query was blocked or processing failed
    pub fn process(&mut self, query: Query) -> Result<Response, String> {
        let start = std::time::Instant::now();

        // Step 1: Expert system safety check
        let eval = self.expert.evaluate(&query);
        if !eval.allowed {
            let response = Response {
                text: format!(
                    "Query blocked by safety rules: {}",
                    eval.reason.unwrap_or_else(|| "Unknown reason".to_string())
                ),
                route: RoutingDecision::Blocked,
                confidence: 1.0,
                latency_ms: start.elapsed().as_millis() as u64,
                metadata: ResponseMetadata {
                    model: None,
                    tokens: None,
                    cached: false,
                },
            };
            return Err(response.text.clone());
        }

        // Step 2: Router decision
        let (route, confidence) = self.router.route(&query);

        // Step 3: Context snapshot
        let _context = self.context.snapshot(10);

        // Step 4: Execute based on routing decision
        let response_text = match route {
            RoutingDecision::Local => self.process_local(&query)?,
            RoutingDecision::Remote => self.process_remote(&query)?,
            RoutingDecision::Hybrid => self.process_hybrid(&query)?,
            RoutingDecision::Blocked => {
                return Err("Query was blocked".to_string());
            }
        };

        let latency = start.elapsed().as_millis() as u64;

        // Calculate tokens before moving response_text
        let tokens = estimate_tokens(&response_text);

        let response = Response {
            text: response_text,
            route,
            confidence,
            latency_ms: latency,
            metadata: ResponseMetadata {
                model: Some(self.get_model_name(route)),
                tokens: Some(tokens),
                cached: false,
            },
        };

        // Step 5: Update context
        self.context.add_turn(query, response.clone());

        Ok(response)
    }

    /// Process query locally (on-device inference)
    ///
    /// Phase 1: Mock implementation returns template responses
    /// Phase 2+: Actual llama.cpp integration
    fn process_local(&self, query: &Query) -> Result<String, String> {
        // Phase 1: Mock local inference
        // Phase 2: Replace with actual llama.cpp call
        Ok(format!(
            "[LOCAL] Processed query: '{}' (Mock response - Phase 2 will use actual SLM)",
            truncate(&query.text, 50)
        ))
    }

    /// Process query remotely (API call)
    ///
    /// Phase 1: Returns error (network feature not enabled by default)
    /// Phase 2: Actual Claude/Mistral API integration
    #[cfg(not(feature = "network"))]
    fn process_remote(&self, _query: &Query) -> Result<String, String> {
        Err("Remote processing requires 'network' feature flag. This is an offline-first build.".to_string())
    }

    #[cfg(feature = "network")]
    fn process_remote(&self, query: &Query) -> Result<String, String> {
        // Phase 2: Actual API call
        Ok(format!(
            "[REMOTE] Would call API for: '{}'",
            truncate(&query.text, 50)
        ))
    }

    /// Process query with hybrid approach (local preprocessing + remote)
    fn process_hybrid(&self, query: &Query) -> Result<String, String> {
        // Phase 1: Mock implementation
        let local_result = self.process_local(query)?;

        #[cfg(feature = "network")]
        {
            let _remote_result = self.process_remote(query)?;
            Ok(format!(
                "[HYBRID] Local preprocessing complete, would combine with remote API for: '{}'",
                truncate(&query.text, 50)
            ))
        }

        #[cfg(not(feature = "network"))]
        {
            Ok(format!(
                "[HYBRID] Offline mode: using only local processing. {}",
                local_result
            ))
        }
    }

    /// Get model name based on routing decision
    fn get_model_name(&self, route: RoutingDecision) -> String {
        match route {
            RoutingDecision::Local => "tinyllama-1.1b-q4".to_string(),
            RoutingDecision::Remote => "claude-sonnet-4.5".to_string(),
            RoutingDecision::Hybrid => "hybrid-local+remote".to_string(),
            RoutingDecision::Blocked => "none".to_string(),
        }
    }

    /// Switch project context
    pub fn switch_project(&mut self, project: impl Into<String>) {
        self.context.switch_project(project);
    }

    /// Get current project
    pub fn current_project(&self) -> Option<&str> {
        self.context.current_project()
    }

    /// Get recent conversation history
    pub fn recent_history(&self, n: usize) -> Vec<crate::types::ConversationTurn> {
        self.context.recent_history(n)
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.context.clear_history();
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Truncate string to max length with ellipsis
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

/// Estimate token count (rough heuristic: chars / 4)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let orch = Orchestrator::new();
        assert!(orch.current_project().is_none());
    }

    #[test]
    fn test_process_simple_query() {
        let mut orch = Orchestrator::new();
        let query = Query::new("Hello, how are you?");
        let result = orch.process(query);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.route, RoutingDecision::Local);
    }

    #[test]
    fn test_process_blocked_query() {
        let mut orch = Orchestrator::new();
        let query = Query::new("Here is my api_key=secret123");
        let result = orch.process(query);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(not(feature = "network"))]
    fn test_process_complex_query() {
        let mut orch = Orchestrator::new();
        let query = Query::new("Can you prove this theorem using formal verification?");
        let result = orch.process(query);

        // Without network feature, remote queries will error
        // The router will route to Remote, but processing will fail
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "network")]
    fn test_process_complex_query() {
        let mut orch = Orchestrator::new();
        let query = Query::new("Can you prove this theorem using formal verification?");
        let result = orch.process(query);
        assert!(result.is_ok());

        let response = result.unwrap();
        // Complex query should route remotely
        assert_eq!(response.route, RoutingDecision::Remote);
    }

    #[test]
    fn test_project_context() {
        let mut orch = Orchestrator::new();
        orch.switch_project("test-project");
        assert_eq!(orch.current_project(), Some("test-project"));
    }

    #[test]
    fn test_conversation_history() {
        let mut orch = Orchestrator::new();

        let query1 = Query::new("First query");
        let _result1 = orch.process(query1);

        let query2 = Query::new("Second query");
        let _result2 = orch.process(query2);

        let history = orch.recent_history(10);
        assert_eq!(history.len(), 2);
        // Most recent first
        assert!(history[0].query.text.contains("Second"));
    }

    #[test]
    fn test_clear_history() {
        let mut orch = Orchestrator::new();

        let query = Query::new("Test");
        let _result = orch.process(query);

        assert_eq!(orch.recent_history(10).len(), 1);

        orch.clear_history();
        assert_eq!(orch.recent_history(10).len(), 0);
    }

    #[test]
    fn test_truncate_helper() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world this is long", 10), "hello worl...");
    }

    #[test]
    fn test_token_estimation() {
        let text = "This is a test";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens < text.len());
    }
}
