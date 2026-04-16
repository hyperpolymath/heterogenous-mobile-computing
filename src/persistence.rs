// SPDX-License-Identifier: PMPL-1.0-or-later
//! SQLite-based persistence for conversation state and model weights.
//!
//! This module provides durable storage for:
//! - Conversation history
//! - Reservoir computing state
//! - MLP weights (trained models)
//! - SNN weights
//! - User preferences and configuration

#![forbid(unsafe_code)]

#[cfg(feature = "persistence")]
use rusqlite::{Connection, Result as SqlResult, params};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::types::{Query, Response, ConversationTurn};
use crate::reservoir::EchoStateNetwork;
use crate::mlp::MLP;

/// Database schema version for migrations
const SCHEMA_VERSION: i32 = 1;

/// Persistence layer for conversation state and models
#[cfg(feature = "persistence")]
pub struct PersistenceManager {
    conn: Connection,
}

#[cfg(feature = "persistence")]
impl PersistenceManager {
    /// Create a new persistence manager with SQLite backend
    pub fn new<P: AsRef<Path>>(db_path: P) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;

        let manager = PersistenceManager { conn };
        manager.initialize_schema()?;

        Ok(manager)
    }

    /// Create in-memory database (for testing)
    pub fn new_in_memory() -> SqlResult<Self> {
        let conn = Connection::open_in_memory()?;

        let manager = PersistenceManager { conn };
        manager.initialize_schema()?;

        Ok(manager)
    }

    /// Initialize database schema
    fn initialize_schema(&self) -> SqlResult<()> {
        // Metadata table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Check schema version
        let version: Result<i32, _> = self.conn.query_row(
            "SELECT value FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        );

        if version.is_err() {
            // First time setup
            self.conn.execute(
                "INSERT INTO metadata (key, value) VALUES ('schema_version', ?1)",
                params![SCHEMA_VERSION.to_string()],
            )?;
        }

        // Conversations table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project TEXT,
                query_text TEXT NOT NULL,
                query_priority INTEGER NOT NULL,
                query_timestamp INTEGER NOT NULL,
                response_text TEXT NOT NULL,
                response_route TEXT NOT NULL,
                response_confidence REAL NOT NULL,
                response_timestamp INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;

        // Index for project-based queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_conversations_project
             ON conversations(project)",
            [],
        )?;

        // Index for timestamp-based queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_conversations_timestamp
             ON conversations(query_timestamp DESC)",
            [],
        )?;

        // Reservoir states table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS reservoir_states (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project TEXT,
                state_json TEXT NOT NULL,
                saved_at INTEGER NOT NULL,
                UNIQUE(project)
            )",
            [],
        )?;

        // Model weights table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS model_weights (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_type TEXT NOT NULL,
                model_name TEXT NOT NULL,
                weights_json TEXT NOT NULL,
                trained_at INTEGER NOT NULL,
                accuracy REAL,
                UNIQUE(model_type, model_name)
            )",
            [],
        )?;

        // Configuration table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Save a conversation turn
    pub fn save_turn(&self, project: Option<&str>, turn: &ConversationTurn) -> SqlResult<i64> {
        let now = current_timestamp();

        self.conn.execute(
            "INSERT INTO conversations (
                project, query_text, query_priority, query_timestamp,
                response_text, response_route, response_confidence,
                response_timestamp, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                project,
                turn.query.text,
                turn.query.priority,
                turn.query.timestamp,
                turn.response.text,
                format!("{:?}", turn.response.route),
                turn.response.confidence,
                turn.response.latency_ms as i64,
                now,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Load recent conversation history for a project
    pub fn load_history(&self, project: Option<&str>, limit: usize) -> SqlResult<Vec<ConversationTurn>> {
        let (query, params_vec): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(proj) = project {
            (
                "SELECT query_text, query_priority, query_timestamp,
                        response_text, response_route, response_confidence,
                        response_timestamp
                 FROM conversations
                 WHERE project = ?1
                 ORDER BY query_timestamp DESC
                 LIMIT ?2".to_string(),
                vec![Box::new(proj.to_string()), Box::new(limit as i64)],
            )
        } else {
            (
                "SELECT query_text, query_priority, query_timestamp,
                        response_text, response_route, response_confidence,
                        response_timestamp
                 FROM conversations
                 WHERE project IS NULL
                 ORDER BY query_timestamp DESC
                 LIMIT ?1".to_string(),
                vec![Box::new(limit as i64)],
            )
        };

        let mut stmt = self.conn.prepare(&query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

        let turns = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(ConversationTurn::from_row(row))
        })?;

        let mut result = Vec::new();
        for turn in turns {
            result.push(turn?);
        }

        // Reverse to get chronological order (oldest first)
        result.reverse();

        Ok(result)
    }

    /// Save reservoir state for a project
    pub fn save_reservoir_state(&self, project: Option<&str>, esn: &EchoStateNetwork) -> SqlResult<()> {
        let state_json = serde_json::to_string(&esn)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let now = current_timestamp();

        self.conn.execute(
            "INSERT OR REPLACE INTO reservoir_states (project, state_json, saved_at)
             VALUES (?1, ?2, ?3)",
            params![project, state_json, now],
        )?;

        Ok(())
    }

    /// Load reservoir state for a project
    pub fn load_reservoir_state(&self, project: Option<&str>) -> SqlResult<Option<EchoStateNetwork>> {
        let result: Result<String, _> = self.conn.query_row(
            "SELECT state_json FROM reservoir_states WHERE project = ?1",
            params![project],
            |row| row.get(0),
        );

        match result {
            Ok(json) => {
                let esn: EchoStateNetwork = serde_json::from_str(&json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?;
                Ok(Some(esn))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Save trained MLP model
    pub fn save_mlp(&self, name: &str, mlp: &MLP, accuracy: Option<f32>) -> SqlResult<()> {
        let weights_json = serde_json::to_string(&mlp)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let now = current_timestamp();

        self.conn.execute(
            "INSERT OR REPLACE INTO model_weights (model_type, model_name, weights_json, trained_at, accuracy)
             VALUES ('mlp', ?1, ?2, ?3, ?4)",
            params![name, weights_json, now, accuracy],
        )?;

        Ok(())
    }

    /// Load trained MLP model
    pub fn load_mlp(&self, name: &str) -> SqlResult<Option<MLP>> {
        let result: Result<String, _> = self.conn.query_row(
            "SELECT weights_json FROM model_weights WHERE model_type = 'mlp' AND model_name = ?1",
            params![name],
            |row| row.get(0),
        );

        match result {
            Ok(json) => {
                let mlp: MLP = serde_json::from_str(&json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?;
                Ok(Some(mlp))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Get conversation count for a project
    pub fn conversation_count(&self, project: Option<&str>) -> SqlResult<usize> {
        let count: i64 = if let Some(proj) = project {
            self.conn.query_row(
                "SELECT COUNT(*) FROM conversations WHERE project = ?1",
                params![proj],
                |row| row.get(0),
            )?
        } else {
            self.conn.query_row(
                "SELECT COUNT(*) FROM conversations WHERE project IS NULL",
                [],
                |row| row.get(0),
            )?
        };

        Ok(count as usize)
    }

    /// Clear all conversation history (for privacy/testing)
    pub fn clear_history(&self, project: Option<&str>) -> SqlResult<usize> {
        let count = if let Some(proj) = project {
            self.conn.execute(
                "DELETE FROM conversations WHERE project = ?1",
                params![proj],
            )?
        } else {
            self.conn.execute(
                "DELETE FROM conversations WHERE project IS NULL",
                [],
            )?
        };

        Ok(count)
    }

    /// Vacuum database to reclaim space
    pub fn vacuum(&self) -> SqlResult<()> {
        self.conn.execute("VACUUM", [])?;
        Ok(())
    }

    /// Get database file size (if not in-memory)
    pub fn database_size(&self) -> SqlResult<u64> {
        let page_count: i64 = self.conn.query_row(
            "PRAGMA page_count",
            [],
            |row| row.get(0),
        )?;

        let page_size: i64 = self.conn.query_row(
            "PRAGMA page_size",
            [],
            |row| row.get(0),
        )?;

        Ok((page_count * page_size) as u64)
    }
}

// Helper for ConversationTurn construction from SQLite row
impl ConversationTurn {
    #[cfg(feature = "persistence")]
    fn from_row(row: &rusqlite::Row) -> Self {
        use crate::types::{Query, Response, RoutingDecision, ResponseMetadata};

        let query_text: String = row.get(0).expect("TODO: handle error");
        let query_priority: u8 = row.get(1).expect("TODO: handle error");
        let query_timestamp: u64 = row.get(2).expect("TODO: handle error");

        let response_text: String = row.get(3).expect("TODO: handle error");
        let response_route_str: String = row.get(4).expect("TODO: handle error");
        let response_confidence: f32 = row.get(5).expect("TODO: handle error");
        let latency_ms: i64 = row.get(6).expect("TODO: handle error");

        // Parse routing decision
        let route = match response_route_str.as_str() {
            "Local" => RoutingDecision::Local,
            "Remote" => RoutingDecision::Remote,
            "Hybrid" => RoutingDecision::Hybrid,
            _ => RoutingDecision::Blocked,
        };

        ConversationTurn {
            query: Query {
                text: query_text,
                project_context: None, // Not stored in simple schema
                priority: query_priority,
                timestamp: query_timestamp,
            },
            response: Response {
                text: response_text,
                route,
                confidence: response_confidence,
                latency_ms: latency_ms as u64,
                metadata: ResponseMetadata {
                    model: None,
                    tokens: None,
                    cached: false,
                },
            },
        }
    }
}

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("TODO: handle error")
        .as_secs()
}

// No-op implementation when persistence is disabled
#[cfg(not(feature = "persistence"))]
pub struct PersistenceManager;

#[cfg(not(feature = "persistence"))]
impl PersistenceManager {
    pub fn new<P: AsRef<Path>>(_db_path: P) -> Result<Self, String> {
        Err("Persistence feature not enabled".to_string())
    }

    pub fn new_in_memory() -> Result<Self, String> {
        Err("Persistence feature not enabled".to_string())
    }
}

#[cfg(all(test, feature = "persistence"))]
mod tests {
    use super::*;
    use crate::types::{Query, Response, RoutingDecision, ResponseMetadata};

    #[test]
    fn test_persistence_manager_creation() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");
        assert_eq!(pm.conversation_count(None).expect("TODO: handle error"), 0);
    }

    #[test]
    fn test_save_and_load_turn() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        let query = Query::new("What is Rust?");
        let response = Response {
            text: "Rust is a systems programming language.".to_string(),
            route: RoutingDecision::Local,
            confidence: 0.9,
            latency_ms: 5,
            metadata: ResponseMetadata {
                model: Some("local-model".to_string()),
                tokens: Some(10),
                cached: false,
            },
        };

        let turn = ConversationTurn {
            query: query.clone(),
            response: response.clone(),
        };

        pm.save_turn(None, &turn).expect("TODO: handle error");

        let history = pm.load_history(None, 10).expect("TODO: handle error");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].query.text, query.text);
        assert_eq!(history[0].response.text, response.text);
    }

    #[test]
    fn test_project_isolation() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        let turn1 = ConversationTurn {
            query: Query::new("Project A query"),
            response: Response {
                text: "Project A response".to_string(),
                route: RoutingDecision::Local,
                confidence: 0.9,
                latency_ms: 5,
                metadata: ResponseMetadata {
                    model: None,
                    tokens: Some(10),
                    cached: false,
                },
            },
        };

        let turn2 = ConversationTurn {
            query: Query::new("Project B query"),
            response: Response {
                text: "Project B response".to_string(),
                route: RoutingDecision::Remote,
                confidence: 0.8,
                latency_ms: 10,
                metadata: ResponseMetadata {
                    model: None,
                    tokens: Some(20),
                    cached: false,
                },
            },
        };

        pm.save_turn(Some("project_a"), &turn1).expect("TODO: handle error");
        pm.save_turn(Some("project_b"), &turn2).expect("TODO: handle error");

        let history_a = pm.load_history(Some("project_a"), 10).expect("TODO: handle error");
        let history_b = pm.load_history(Some("project_b"), 10).expect("TODO: handle error");

        assert_eq!(history_a.len(), 1);
        assert_eq!(history_b.len(), 1);
        assert_eq!(history_a[0].query.text, "Project A query");
        assert_eq!(history_b[0].query.text, "Project B query");
    }

    #[test]
    fn test_reservoir_persistence() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        let mut esn = EchoStateNetwork::new(384, 1000, 100, 0.7, 0.95);

        // Update state to make it non-default
        let input = vec![0.5; 384];
        esn.update(&input);

        pm.save_reservoir_state(Some("test_project"), &esn).expect("TODO: handle error");

        let loaded = pm.load_reservoir_state(Some("test_project")).expect("TODO: handle error");
        assert!(loaded.is_some());

        // Verify we can use the loaded ESN
        let mut loaded_esn = loaded.expect("TODO: handle error");
        let output = loaded_esn.output();
        assert_eq!(output.len(), 100);
    }

    #[test]
    fn test_mlp_persistence() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        let mlp = MLP::new(384, vec![100, 50], 3);
        pm.save_mlp("router", &mlp, Some(0.85)).expect("TODO: handle error");

        let loaded = pm.load_mlp("router").expect("TODO: handle error");
        assert!(loaded.is_some());

        // Verify we can use the loaded MLP
        let loaded_mlp = loaded.expect("TODO: handle error");
        let input = vec![0.5; 384];
        let output = loaded_mlp.forward(&input);
        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_clear_history() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        for i in 0..10 {
            let turn = ConversationTurn {
                query: Query::new(&format!("Query {}", i)),
                response: Response {
                    text: format!("Response {}", i),
                    route: RoutingDecision::Local,
                    confidence: 0.9,
                    latency_ms: 5,
                    metadata: ResponseMetadata {
                        model: None,
                        tokens: Some(10),
                        cached: false,
                    },
                },
            };
            pm.save_turn(None, &turn).expect("TODO: handle error");
        }

        assert_eq!(pm.conversation_count(None).expect("TODO: handle error"), 10);

        pm.clear_history(None).expect("TODO: handle error");
        assert_eq!(pm.conversation_count(None).expect("TODO: handle error"), 0);
    }

    #[test]
    fn test_history_limit() {
        let pm = PersistenceManager::new_in_memory().expect("TODO: handle error");

        let base_timestamp = current_timestamp();
        for i in 0..100 {
            let mut query = Query::new(&format!("Query {}", i));
            // Set explicit timestamp to ensure ordering
            query.timestamp = base_timestamp + i as u64;

            let turn = ConversationTurn {
                query,
                response: Response {
                    text: format!("Response {}", i),
                    route: RoutingDecision::Local,
                    confidence: 0.9,
                    latency_ms: 5,
                    metadata: ResponseMetadata {
                        model: None,
                        tokens: Some(10),
                        cached: false,
                    },
                },
            };
            pm.save_turn(None, &turn).expect("TODO: handle error");
        }

        let history = pm.load_history(None, 10).expect("TODO: handle error");
        assert_eq!(history.len(), 10);

        // Should get most recent 10 (90-99) in chronological order (oldest first)
        assert_eq!(history[0].query.text, "Query 90");
        assert_eq!(history[9].query.text, "Query 99");
    }
}
