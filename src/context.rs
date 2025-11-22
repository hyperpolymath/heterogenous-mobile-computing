//! Context Manager: Conversation state and history management
//!
//! Phase 1: Simple in-memory storage with serialization
//! Phase 2: SQLite persistence
//! Phase 3: Reservoir computing integration for temporal state
//!
//! Provides:
//! - Conversation history tracking
//! - Project context switching
//! - State snapshots
//! - Context retrieval for query augmentation

use crate::types::{ContextSnapshot, ConversationTurn, Query, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum conversation history to keep in memory
const MAX_HISTORY_SIZE: usize = 100;

/// Context manager for maintaining conversation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManager {
    /// Current project context
    current_project: Option<String>,
    /// Conversation history (most recent first)
    history: Vec<ConversationTurn>,
    /// Per-project context snapshots
    project_contexts: HashMap<String, Vec<ConversationTurn>>,
}

impl ContextManager {
    /// Create a new context manager
    pub fn new() -> Self {
        Self {
            current_project: None,
            history: Vec::new(),
            project_contexts: HashMap::new(),
        }
    }

    /// Add a conversation turn to history
    pub fn add_turn(&mut self, query: Query, response: Response) {
        let turn = ConversationTurn { query, response };

        // Add to main history
        self.history.insert(0, turn.clone());

        // Trim if exceeds max size
        if self.history.len() > MAX_HISTORY_SIZE {
            self.history.truncate(MAX_HISTORY_SIZE);
        }

        // Add to project-specific history if applicable
        if let Some(ref project) = self.current_project {
            self.project_contexts
                .entry(project.clone())
                .or_insert_with(Vec::new)
                .insert(0, turn);

            // Trim project history too
            if let Some(project_history) = self.project_contexts.get_mut(project) {
                if project_history.len() > MAX_HISTORY_SIZE {
                    project_history.truncate(MAX_HISTORY_SIZE);
                }
            }
        }
    }

    /// Switch to a different project context
    pub fn switch_project(&mut self, project: impl Into<String>) {
        let project = project.into();
        self.current_project = Some(project);
    }

    /// Clear current project context
    pub fn clear_project(&mut self) {
        self.current_project = None;
    }

    /// Get current project
    pub fn current_project(&self) -> Option<&str> {
        self.current_project.as_deref()
    }

    /// Get recent conversation history
    ///
    /// Returns the N most recent turns
    pub fn recent_history(&self, n: usize) -> Vec<ConversationTurn> {
        self.history.iter().take(n).cloned().collect()
    }

    /// Get project-specific history
    pub fn project_history(&self, project: &str) -> Option<Vec<ConversationTurn>> {
        self.project_contexts.get(project).cloned()
    }

    /// Get a context snapshot for augmenting queries
    pub fn snapshot(&self, history_size: usize) -> ContextSnapshot {
        ContextSnapshot {
            project: self.current_project.clone(),
            history: self.recent_history(history_size),
            reservoir_state: None, // Phase 2: reservoir computing
        }
    }

    /// Clear all history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Clear project-specific history
    pub fn clear_project_history(&mut self, project: &str) {
        self.project_contexts.remove(project);
    }

    /// Get total conversation count
    pub fn conversation_count(&self) -> usize {
        self.history.len()
    }

    /// Get project list
    pub fn projects(&self) -> Vec<String> {
        self.project_contexts.keys().cloned().collect()
    }

    /// Serialize to JSON (for persistence)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Response, ResponseMetadata, RoutingDecision};

    fn create_test_response(text: &str) -> Response {
        Response {
            text: text.to_string(),
            route: RoutingDecision::Local,
            confidence: 0.9,
            latency_ms: 100,
            metadata: ResponseMetadata {
                model: Some("test-model".to_string()),
                tokens: Some(50),
                cached: false,
            },
        }
    }

    #[test]
    fn test_context_manager_creation() {
        let cm = ContextManager::new();
        assert_eq!(cm.conversation_count(), 0);
        assert!(cm.current_project().is_none());
    }

    #[test]
    fn test_add_turn() {
        let mut cm = ContextManager::new();
        let query = Query::new("test query");
        let response = create_test_response("test response");

        cm.add_turn(query, response);
        assert_eq!(cm.conversation_count(), 1);
    }

    #[test]
    fn test_project_switching() {
        let mut cm = ContextManager::new();
        cm.switch_project("project-1");
        assert_eq!(cm.current_project(), Some("project-1"));

        cm.clear_project();
        assert!(cm.current_project().is_none());
    }

    #[test]
    fn test_project_specific_history() {
        let mut cm = ContextManager::new();
        cm.switch_project("project-1");

        let query = Query::new("project 1 query");
        let response = create_test_response("response");
        cm.add_turn(query, response);

        let history = cm.project_history("project-1");
        assert!(history.is_some());
        assert_eq!(history.unwrap().len(), 1);
    }

    #[test]
    fn test_recent_history() {
        let mut cm = ContextManager::new();

        for i in 0..10 {
            let query = Query::new(format!("query {}", i));
            let response = create_test_response(&format!("response {}", i));
            cm.add_turn(query, response);
        }

        let recent = cm.recent_history(5);
        assert_eq!(recent.len(), 5);
        // Most recent first
        assert!(recent[0].query.text.contains("9"));
    }

    #[test]
    fn test_snapshot() {
        let mut cm = ContextManager::new();
        cm.switch_project("test-project");

        let query = Query::new("test");
        let response = create_test_response("response");
        cm.add_turn(query, response);

        let snapshot = cm.snapshot(10);
        assert_eq!(snapshot.project, Some("test-project".to_string()));
        assert_eq!(snapshot.history.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let mut cm = ContextManager::new();
        cm.switch_project("test");

        let query = Query::new("test query");
        let response = create_test_response("test response");
        cm.add_turn(query, response);

        let json = cm.to_json().unwrap();
        let restored = ContextManager::from_json(&json).unwrap();

        assert_eq!(restored.current_project(), cm.current_project());
        assert_eq!(restored.conversation_count(), cm.conversation_count());
    }

    #[test]
    fn test_max_history_limit() {
        let mut cm = ContextManager::new();

        // Add more than MAX_HISTORY_SIZE
        for i in 0..150 {
            let query = Query::new(format!("query {}", i));
            let response = create_test_response(&format!("response {}", i));
            cm.add_turn(query, response);
        }

        assert_eq!(cm.conversation_count(), MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_projects_list() {
        let mut cm = ContextManager::new();

        cm.switch_project("project-1");
        cm.add_turn(Query::new("q1"), create_test_response("r1"));

        cm.switch_project("project-2");
        cm.add_turn(Query::new("q2"), create_test_response("r2"));

        let projects = cm.projects();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&"project-1".to_string()));
        assert!(projects.contains(&"project-2".to_string()));
    }
}
