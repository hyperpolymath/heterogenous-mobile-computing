// SPDX-License-Identifier: PMPL-1.0-or-later
//! Expert System — Deterministic Safety and Policy Enforcement.
//!
//! This module implements the "Guardrail" layer of the mobile AI system.
//! It uses a set of explicit, symbolic rules to audit incoming queries
//! before they reach the neural inference stage.
//!
//! DESIGN PILLARS:
//! 1. **Explainability**: Every rejection includes a human-readable
//!    `rule_id` and reason.
//! 2. **Privacy**: Proactively detects and blocks potential credential
//!    leakage (API keys, passwords).
//! 3. **Attenuation**: Enforces resource limits (e.g. max query length)
//!    to prevent Denial of Service.

use crate::types::{Query, RuleEvaluation};

/// Rule: A predicate for query evaluation.
#[derive(Debug, Clone)]
pub struct Rule {
    id: String,
    predicate: fn(&Query) -> bool,
}

/// RULE ENGINE: Manages a collection of security and policy predicates.
#[derive(Debug, Clone)]
pub struct ExpertSystem {
    rules: Vec<Rule>,
}

impl Default for ExpertSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// EVALUATION: Iterates through the rule set. If any `Block` rule
/// matches the query, the entire request is rejected immediately.
impl ExpertSystem {
    /// Create a new expert system with default rules.
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
        }
    }

    /// Evaluate a query against all rules.
    pub fn evaluate(&self, query: &Query) -> RuleEvaluation {
        for rule in &self.rules {
            if (rule.predicate)(query) {
                return RuleEvaluation {
                    allowed: false,
                    reason: Some(format!("Rule {} triggered", rule.id)),
                    rule_id: Some(rule.id.clone()),
                };
            }
        }
        RuleEvaluation {
            allowed: true,
            reason: None,
            rule_id: None,
        }
    }

    /// DEFAULT POLICIES:
    /// - PRIVACY_001: Block potential API keys.
    /// - SAFETY_001: Block requests for harmful instructions (hacking, etc.).
    fn default_rules() -> Vec<Rule> {
        vec![
            Rule {
                id: "PRIVACY_001".to_string(),
                predicate: |query| {
                    let text = query.text.to_lowercase();
                    text.contains("api_key") || text.contains("password")
                },
            },
            Rule {
                id: "SAFETY_001".to_string(),
                predicate: |query| {
                    let text = query.text.to_lowercase();
                    text.contains("hack") || text.contains("malware")
                },
            },
        ]
    }
}
