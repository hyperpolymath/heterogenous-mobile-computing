// SPDX-License-Identifier: PMPL-1.0-or-later
//! Orchestrator — Mobile AI Coordination Layer.
//!
//! This module implements the central pipeline for on-device AI decision
//! making. It orchestrates the interaction between symbolic safety rules,
//! neural routing models, and local/remote inference engines.
//!
//! CORE PIPELINE:
//! 1. **Evaluation**: The Expert System audits the query for safety
//!    and policy compliance.
//! 2. **Routing**: An MLP-based model decides if the query should be
//!    handled locally (SLM) or offloaded to a remote API (LLM).
//! 3. **Execution**: The chosen inference engine produces a response.
//! 4. **Persistence**: The turn is recorded in the Context Manager for
//!    long-term memory.

use crate::{
    context::ContextManager,
    expert::ExpertSystem,
    router::{Router, RouterConfig},
    types::{Query, Response, ResponseMetadata, RoutingDecision},
};

/// Orchestrator: Coordinates the full AI pipeline.
pub struct Orchestrator {
    router: Router,
    expert: ExpertSystem,
    context: ContextManager,
}

impl Orchestrator {
    /// Create a new orchestrator with default configuration.
    pub fn new() -> Self {
        Self {
            router: Router::new(RouterConfig::default()),
            expert: ExpertSystem::new(),
            context: ContextManager::new(),
        }
    }

    /// PROCESS: Executes the full coordination pipeline for a single query.
    ///
    /// HYBRID STRATEGY:
    /// - `Local`: Low-latency, privacy-preserving on-device inference.
    /// - `Remote`: High-capability cloud-based reasoning (feature-gated).
    /// - `Hybrid`: Local preprocessing (e.g. summarization) followed by remote query.
    pub fn process(&mut self, query: Query) -> Result<Response, String> {
        // Step 1: Expert system evaluation
        let eval = self.expert.evaluate(&query);
        if !eval.allowed {
            return Ok(Response {
                text: "Request blocked by safety rules".to_string(),
                route: RoutingDecision::Blocked,
                confidence: 1.0,
                latency_ms: 0,
                metadata: ResponseMetadata {
                    model: Some("expert-system".to_string()),
                    tokens: None,
                    cached: false,
                },
            });
        }

        // Step 2: Routing decision
        let (route, confidence) = self.router.route(&query);

        // Step 3: Generate response (Phase 1: placeholder)
        let response = Response {
            text: format!("Response to: {}", query.text),
            route,
            confidence,
            latency_ms: 10,
            metadata: ResponseMetadata {
                model: Some("orchestrator-phase1".to_string()),
                tokens: Some(50),
                cached: false,
            },
        };

        // Step 4: Update context
        self.context.add_turn(query, response.clone());

        Ok(response)
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}
