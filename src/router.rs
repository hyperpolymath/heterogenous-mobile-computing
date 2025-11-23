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
use crate::mlp::MLP;

/// Router for determining query routing strategy
#[derive(Debug, Clone)]
pub struct Router {
    config: RouterConfig,
    /// Optional MLP for learned routing (Phase 2+)
    mlp: Option<MLP>,
    /// Whether to use MLP or fallback to heuristics
    use_mlp: bool,
}

impl Router {
    /// Create a new router with default configuration
    pub fn new() -> Self {
        Self {
            config: RouterConfig::default(),
            mlp: None,
            use_mlp: false,
        }
    }

    /// Create a router with custom configuration
    pub fn with_config(config: RouterConfig) -> Self {
        Self {
            config,
            mlp: None,
            use_mlp: false,
        }
    }

    /// Create a router with a trained MLP
    pub fn with_mlp(mlp: MLP) -> Self {
        Self {
            config: RouterConfig::default(),
            mlp: Some(mlp),
            use_mlp: true,
        }
    }

    /// Load MLP from persistence
    #[cfg(feature = "persistence")]
    pub fn load_mlp(&mut self, pm: &crate::persistence::PersistenceManager, name: &str) -> Result<(), String> {
        match pm.load_mlp(name) {
            Ok(Some(mlp)) => {
                self.mlp = Some(mlp);
                self.use_mlp = true;
                Ok(())
            }
            Ok(None) => Err(format!("No MLP found with name '{}'", name)),
            Err(e) => Err(format!("Failed to load MLP: {}", e)),
        }
    }

    /// Save MLP to persistence
    #[cfg(feature = "persistence")]
    pub fn save_mlp(&self, pm: &crate::persistence::PersistenceManager, name: &str, accuracy: Option<f32>) -> Result<(), String> {
        if let Some(ref mlp) = self.mlp {
            pm.save_mlp(name, mlp, accuracy)
                .map_err(|e| format!("Failed to save MLP: {}", e))
        } else {
            Err("No MLP to save".to_string())
        }
    }

    /// Set the MLP for routing
    pub fn set_mlp(&mut self, mlp: MLP) {
        self.mlp = Some(mlp);
        self.use_mlp = true;
    }

    /// Enable or disable MLP usage (fallback to heuristics if disabled)
    pub fn set_use_mlp(&mut self, use_mlp: bool) {
        self.use_mlp = use_mlp && self.mlp.is_some();
    }

    /// Extract features from query for MLP input
    fn extract_features(&self, query: &Query) -> Vec<f32> {
        let mut features = vec![0.0; 384];

        // Feature 0: Normalized query length
        features[0] = (query.text.len() as f32 / 1000.0).min(1.0);

        // Feature 1: Word count normalized
        features[1] = (query.text.split_whitespace().count() as f32 / 100.0).min(1.0);

        // Feature 2: Has question mark
        features[2] = if query.text.contains('?') { 1.0 } else { 0.0 };

        // Feature 3: Priority normalized
        features[3] = query.priority as f32 / 10.0;

        // Feature 4: Has project context
        features[4] = if query.project_context.is_some() { 1.0 } else { 0.0 };

        // Feature 5-9: Complex keyword indicators
        for (i, keyword) in self.config.complex_keywords.iter().enumerate().take(5) {
            features[5 + i] = if query.text.to_lowercase().contains(&keyword.to_lowercase()) {
                1.0
            } else {
                0.0
            };
        }

        // Feature 10: Uppercase ratio (might indicate emphasis/urgency)
        let uppercase_count = query.text.chars().filter(|c| c.is_uppercase()).count();
        features[10] = if !query.text.is_empty() {
            uppercase_count as f32 / query.text.len() as f32
        } else {
            0.0
        };

        // Feature 11: Punctuation density
        let punct_count = query.text.chars().filter(|c| c.is_ascii_punctuation()).count();
        features[11] = if !query.text.is_empty() {
            punct_count as f32 / query.text.len() as f32
        } else {
            0.0
        };

        // Features 12-19: Query type indicators
        features[12] = if query.text.to_lowercase().starts_with("how") { 1.0 } else { 0.0 };
        features[13] = if query.text.to_lowercase().starts_with("what") { 1.0 } else { 0.0 };
        features[14] = if query.text.to_lowercase().starts_with("why") { 1.0 } else { 0.0 };
        features[15] = if query.text.to_lowercase().starts_with("when") { 1.0 } else { 0.0 };
        features[16] = if query.text.to_lowercase().starts_with("where") { 1.0 } else { 0.0 };
        features[17] = if query.text.to_lowercase().starts_with("who") { 1.0 } else { 0.0 };
        features[18] = if query.text.to_lowercase().starts_with("can") { 1.0 } else { 0.0 };
        features[19] = if query.text.to_lowercase().starts_with("should") { 1.0 } else { 0.0 };

        // Features 20-379: Simple bag-of-words encoding (placeholder for better embedding)
        // In production, replace with sentence-transformers or similar
        let words: Vec<&str> = query.text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate().take(360) {
            // Simple hash-based encoding
            let hash = word.chars().map(|c| c as u32).sum::<u32>() % 360;
            features[20 + hash as usize] += 1.0 / words.len() as f32;
        }

        // Features 380-383: Metadata
        // Timestamp normalized to [0,1] based on time of day (seconds since midnight / 86400)
        features[380] = ((query.timestamp % 86400) as f32 / 86400.0).min(1.0);
        features[381] = if query.is_high_priority() { 1.0 } else { 0.0 };
        features[382] = if query.text.len() > 200 { 1.0 } else { 0.0 }; // Long query indicator
        features[383] = if query.text.contains("error") || query.text.contains("bug") {
            1.0
        } else {
            0.0
        }; // Debugging indicator

        features
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
    /// # Phase 2+ (MLP-based)
    ///
    /// When MLP is available and enabled:
    /// - Input: query embedding + context features (384-dim)
    /// - Hidden layers: 2-3 layers, ~50-100 neurons each
    /// - Output: [local_score, remote_score, hybrid_score]
    /// - Decision: argmax(output)
    pub fn route(&self, query: &Query) -> (RoutingDecision, f32) {
        // Phase 2+: Try MLP routing first
        if self.use_mlp && self.mlp.is_some() {
            return self.route_with_mlp(query);
        }

        // Phase 1: Fallback to heuristic routing
        self.route_heuristic(query)
    }

    /// MLP-based routing
    fn route_with_mlp(&self, query: &Query) -> (RoutingDecision, f32) {
        if let Some(ref mlp) = self.mlp {
            // Extract features
            let features = self.extract_features(query);

            // Forward pass
            let logits = mlp.forward(&features);

            // Softmax to get probabilities
            let probs = MLP::softmax(&logits);

            // Get decision (argmax)
            let decision_idx = MLP::argmax(&probs);
            let confidence = probs[decision_idx];

            let decision = match decision_idx {
                0 => RoutingDecision::Local,
                1 => RoutingDecision::Remote,
                2 => RoutingDecision::Hybrid,
                _ => RoutingDecision::Local, // Fallback
            };

            (decision, confidence)
        } else {
            // Shouldn't reach here, but fallback just in case
            self.route_heuristic(query)
        }
    }

    /// Heuristic-based routing (Phase 1)
    fn route_heuristic(&self, query: &Query) -> (RoutingDecision, f32) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mlp::MLP;

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

    #[test]
    fn test_router_with_mlp() {
        // Create a simple MLP for testing
        let mlp = MLP::new(384, vec![100, 50], 3);
        let router = Router::with_mlp(mlp);

        let query = Query::new("What is Rust?");
        let (decision, confidence) = router.route(&query);

        // Should use MLP routing (decision can be anything, but confidence should be in [0,1])
        assert!(confidence >= 0.0 && confidence <= 1.0);
        // Decision should be one of the valid types
        assert!(matches!(
            decision,
            RoutingDecision::Local
                | RoutingDecision::Remote
                | RoutingDecision::Hybrid
                | RoutingDecision::Blocked
        ));
    }

    #[test]
    fn test_router_mlp_fallback() {
        let mut router = Router::new();
        // No MLP set, should use heuristics
        assert!(!router.use_mlp);

        let query = Query::new("Short query");
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Local);

        // Set MLP
        let mlp = MLP::new(384, vec![100, 50], 3);
        router.set_mlp(mlp);
        assert!(router.use_mlp);

        // Disable MLP
        router.set_use_mlp(false);
        assert!(!router.use_mlp);

        // Should fallback to heuristics
        let (decision, _) = router.route(&query);
        assert_eq!(decision, RoutingDecision::Local);
    }

    #[test]
    fn test_feature_extraction() {
        let router = Router::new();
        let query = Query::new("How do I iterate a HashMap in Rust?");

        let features = router.extract_features(&query);

        // Check feature vector size
        assert_eq!(features.len(), 384);

        // Check that some basic features are set correctly
        assert!(features[2] > 0.0); // Has question mark
        assert!(features[12] > 0.0); // Starts with "how"

        // Check normalization
        assert!(features.iter().all(|&f| f >= 0.0 && f <= 1.0));
    }

    #[test]
    #[cfg(feature = "persistence")]
    fn test_mlp_persistence_integration() {
        use crate::persistence::PersistenceManager;

        let pm = PersistenceManager::new_in_memory().unwrap();

        // Create router with MLP
        let mlp = MLP::new(384, vec![100, 50], 3);
        let mut router = Router::with_mlp(mlp);

        // Save MLP
        router.save_mlp(&pm, "test_router", Some(0.85)).unwrap();

        // Create new router and load MLP
        let mut new_router = Router::new();
        new_router.load_mlp(&pm, "test_router").unwrap();

        assert!(new_router.use_mlp);

        // Test that it can route
        let query = Query::new("Test query");
        let (decision, confidence) = new_router.route(&query);
        assert!(confidence >= 0.0 && confidence <= 1.0);
        assert!(matches!(
            decision,
            RoutingDecision::Local
                | RoutingDecision::Remote
                | RoutingDecision::Hybrid
                | RoutingDecision::Blocked
        ));
    }
}
