//! Expert System: Rule-based safety and policy enforcement
//!
//! Implements the deterministic safety layer that:
//! - Enforces privacy rules
//! - Blocks unsafe queries
//! - Validates resource constraints
//! - Provides explainable decisions
//!
//! All rules are explicit and auditable.

use crate::types::{Query, RuleEvaluation};

/// Rule-based expert system for safety enforcement
#[derive(Debug, Clone)]
pub struct ExpertSystem {
    rules: Vec<Rule>,
}

/// A single rule in the expert system
#[derive(Debug, Clone)]
struct Rule {
    id: String,
    description: String,
    predicate: fn(&Query) -> bool,
    action: RuleAction,
}

/// Action to take when a rule triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleAction {
    /// Block the query entirely
    Block,
    /// Warn but allow
    Warn,
    /// Log for audit
    Log,
}

impl ExpertSystem {
    /// Create a new expert system with default rules
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
        }
    }

    /// Create an expert system with custom rules
    pub fn with_rules(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    /// Evaluate a query against all rules
    ///
    /// Returns the first blocking rule encountered, or allows if none match
    pub fn evaluate(&self, query: &Query) -> RuleEvaluation {
        for rule in &self.rules {
            if (rule.predicate)(query) {
                match rule.action {
                    RuleAction::Block => {
                        return RuleEvaluation {
                            allowed: false,
                            reason: Some(rule.description.clone()),
                            rule_id: Some(rule.id.clone()),
                        };
                    }
                    RuleAction::Warn | RuleAction::Log => {
                        // Continue evaluating, but log this for audit
                        continue;
                    }
                }
            }
        }

        // No blocking rules triggered
        RuleEvaluation {
            allowed: true,
            reason: None,
            rule_id: None,
        }
    }

    /// Default safety rules
    fn default_rules() -> Vec<Rule> {
        vec![
            Rule {
                id: "PRIVACY_001".to_string(),
                description: "Block queries containing potential API keys".to_string(),
                predicate: contains_api_key_pattern,
                action: RuleAction::Block,
            },
            Rule {
                id: "PRIVACY_002".to_string(),
                description: "Block queries with potential passwords".to_string(),
                predicate: contains_password_pattern,
                action: RuleAction::Block,
            },
            Rule {
                id: "SAFETY_001".to_string(),
                description: "Block queries requesting harmful instructions".to_string(),
                predicate: contains_harmful_request,
                action: RuleAction::Block,
            },
            Rule {
                id: "RESOURCE_001".to_string(),
                description: "Warn on extremely long queries (>5000 chars)".to_string(),
                predicate: is_extremely_long,
                action: RuleAction::Warn,
            },
        ]
    }

    /// Add a custom rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Get all rule IDs
    pub fn rule_ids(&self) -> Vec<String> {
        self.rules.iter().map(|r| r.id.clone()).collect()
    }
}

impl Default for ExpertSystem {
    fn default() -> Self {
        Self::new()
    }
}

// Rule predicates

/// Check if query contains API key patterns
fn contains_api_key_pattern(query: &Query) -> bool {
    let text = query.text.to_lowercase();
    // Simple heuristic: "api_key", "secret", "token" followed by "="
    (text.contains("api_key") || text.contains("secret") || text.contains("token"))
        && text.contains('=')
}

/// Check if query contains password patterns
fn contains_password_pattern(query: &Query) -> bool {
    let text = query.text.to_lowercase();
    (text.contains("password") || text.contains("passwd")) && text.contains('=')
}

/// Check if query requests harmful instructions
fn contains_harmful_request(query: &Query) -> bool {
    let text = query.text.to_lowercase();
    let harmful_keywords = ["hack", "exploit", "bypass security", "steal"];
    harmful_keywords
        .iter()
        .any(|kw| text.contains(kw))
}

/// Check if query is extremely long
fn is_extremely_long(query: &Query) -> bool {
    query.text.len() > 5000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expert_system_creation() {
        let expert = ExpertSystem::new();
        assert!(!expert.rules.is_empty());
    }

    #[test]
    fn test_allow_normal_query() {
        let expert = ExpertSystem::new();
        let query = Query::new("How do I write a for loop in Rust?");
        let eval = expert.evaluate(&query);
        assert!(eval.allowed);
        assert!(eval.reason.is_none());
    }

    #[test]
    fn test_block_api_key() {
        let expert = ExpertSystem::new();
        let query = Query::new("Here's my api_key=sk-1234567890");
        let eval = expert.evaluate(&query);
        assert!(!eval.allowed);
        assert!(eval.reason.is_some());
        assert_eq!(eval.rule_id, Some("PRIVACY_001".to_string()));
    }

    #[test]
    fn test_block_password() {
        let expert = ExpertSystem::new();
        let query = Query::new("My password=hunter2");
        let eval = expert.evaluate(&query);
        assert!(!eval.allowed);
        assert!(eval.reason.is_some());
    }

    #[test]
    fn test_block_harmful_request() {
        let expert = ExpertSystem::new();
        let query = Query::new("How do I hack into a system?");
        let eval = expert.evaluate(&query);
        assert!(!eval.allowed);
        assert!(eval.reason.is_some());
    }

    #[test]
    fn test_rule_ids() {
        let expert = ExpertSystem::new();
        let ids = expert.rule_ids();
        assert!(ids.contains(&"PRIVACY_001".to_string()));
        assert!(ids.contains(&"SAFETY_001".to_string()));
    }

    #[test]
    fn test_edge_case_empty_query() {
        let expert = ExpertSystem::new();
        let query = Query::new("");
        let eval = expert.evaluate(&query);
        assert!(eval.allowed);
    }

    #[test]
    fn test_case_insensitive_detection() {
        let expert = ExpertSystem::new();
        let query = Query::new("API_KEY=secret123");
        let eval = expert.evaluate(&query);
        assert!(!eval.allowed);
    }
}
