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

use crate::reservoir::{encode_text, EchoStateNetwork};
use crate::types::{ContextSnapshot, ConversationTurn, Query, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum conversation history to keep in memory
const MAX_HISTORY_SIZE: usize = 100;

/// Dimension for text encoding (matches reservoir input size)
const ENCODING_DIM: usize = 384;

/// Context manager for maintaining conversation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManager {
    /// Current project context
    current_project: Option<String>,
    /// Conversation history (most recent first)
    history: Vec<ConversationTurn>,
    /// Per-project context snapshots
    project_contexts: HashMap<String, Vec<ConversationTurn>>,
    /// Reservoir for temporal context encoding (Phase 2)
    #[serde(skip)]
    reservoir: Option<EchoStateNetwork>,
}

impl ContextManager {
    /// Create a new context manager
    pub fn new() -> Self {
        Self::with_reservoir(false)
    }

    /// Create a context manager with reservoir computing enabled
    pub fn with_reservoir(enable_reservoir: bool) -> Self {
        let reservoir = if enable_reservoir {
            Some(EchoStateNetwork::new(
                ENCODING_DIM, // input size
                1000,         // reservoir size
                100,          // output size (compressed context)
                0.7,          // leak rate
                0.95,         // spectral radius
            ))
        } else {
            None
        };

        Self {
            current_project: None,
            history: Vec::new(),
            project_contexts: HashMap::new(),
            reservoir,
        }
    }

    /// Add a conversation turn to history
    pub fn add_turn(&mut self, query: Query, response: Response) {
        let turn = ConversationTurn {
            query: query.clone(),
            response: response.clone(),
        };

        // Update reservoir with query text if enabled
        if let Some(ref mut reservoir) = self.reservoir {
            let encoding = encode_text(&query.text, ENCODING_DIM);
            reservoir.update(&encoding);
        }

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
        let reservoir_state = self.reservoir.as_ref().map(|r| r.state().to_vec());

        ContextSnapshot {
            project: self.current_project.clone(),
            history: self.recent_history(history_size),
            reservoir_state,
        }
    }

    /// Get reservoir state vector (if reservoir is enabled)
    pub fn reservoir_state(&self) -> Option<Vec<f32>> {
        self.reservoir.as_ref().map(|r| r.state().to_vec())
    }

    /// Reset reservoir state (if enabled)
    pub fn reset_reservoir(&mut self) {
        if let Some(ref mut reservoir) = self.reservoir {
            reservoir.reset();
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

    #[test]
    fn test_context_manager_with_reservoir() {
        let mut cm = ContextManager::with_reservoir(true);

        // Reservoir state should initially be zeros
        let state1 = cm.reservoir_state();
        assert!(state1.is_some());
        assert_eq!(state1.as_ref().unwrap().len(), 1000);

        // Add a turn - reservoir should update
        cm.add_turn(Query::new("Hello world"), create_test_response("Hi"));

        let state2 = cm.reservoir_state();
        assert!(state2.is_some());

        // State should have changed
        assert_ne!(state1, state2);

        // Snapshot should include reservoir state
        let snapshot = cm.snapshot(5);
        assert!(snapshot.reservoir_state.is_some());
        assert_eq!(snapshot.reservoir_state.unwrap().len(), 1000);
    }

    #[test]
    fn test_reservoir_reset() {
        let mut cm = ContextManager::with_reservoir(true);

        cm.add_turn(Query::new("test"), create_test_response("response"));

        let state = cm.reservoir_state().unwrap();
        assert!(!state.iter().all(|&x| x == 0.0));

        cm.reset_reservoir();

        let state_after_reset = cm.reservoir_state().unwrap();
        assert!(state_after_reset.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_context_manager_without_reservoir() {
        let cm = ContextManager::new();

        // Without reservoir, state should be None
        assert!(cm.reservoir_state().is_none());

        let snapshot = cm.snapshot(5);
        assert!(snapshot.reservoir_state.is_none());
    }
}
