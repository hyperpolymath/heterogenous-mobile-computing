//! Mobile AI Orchestrator - Phase 1 MVP
//!
//! A hybrid AI orchestration system designed for constrained mobile platforms.
//! This implementation follows the Rhodium Standard Repository (RSR) framework.
//!
//! # Architecture
//!
//! Phase 1 includes four core components:
//! - **Router**: MLP-based decision engine for local vs. API routing
//! - **Expert System**: Rule-based safety and policy enforcement
//! - **Context Manager**: SQLite-backed conversation state
//! - **Orchestrator**: Main coordination layer
//!
//! # Safety
//!
//! - Zero `unsafe` blocks
//! - Full type safety via Rust's ownership system
//! - Memory safety guaranteed by the compiler
//!
//! # Offline-First
//!
//! - Network features are optional (behind `network` feature flag)
//! - All core functionality works air-gapped
//! - Local inference prioritized over API calls
//!
//! # Features
//!
//! - `persistence` (default): SQLite-backed state persistence
//! - `network`: Enable tokio + reqwest for cloud API calls
//! - `high-perf`: ndarray + rayon for optimized matrix operations
//! - `logging`: Structured logging with tracing
//! - `fast-serde`: Binary serialization with bincode
//! - `full`: All features enabled

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub mod context;
pub mod expert;
pub mod mlp;
pub mod orchestrator;
pub mod persistence;
pub mod reservoir;
pub mod router;
pub mod sensor;
pub mod snn;
pub mod training;
pub mod types;

pub use orchestrator::Orchestrator;
pub use reservoir::EchoStateNetwork;
pub use sensor::{SensorBuffer, SensorReading, SensorType};
pub use snn::SpikingNetwork;
pub use types::{Query, Response, RoutingDecision};

/// Library version following semantic versioning
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// RSR compliance level
pub const RSR_COMPLIANCE: &str = "Bronze";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_rsr_compliance() {
        assert_eq!(RSR_COMPLIANCE, "Bronze");
    }

    #[test]
    fn test_no_unsafe() {
        // This test passes if the crate compiles with #![forbid(unsafe_code)]
        assert!(true);
    }
}
