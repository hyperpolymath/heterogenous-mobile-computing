// SPDX-License-Identifier: PMPL-1.0-or-later
//! Heterogenous Mobile Computing — Verified AI Framework.
//!
//! This crate implements a hybrid AI orchestration system optimized for 
//! constrained mobile platforms. It follows the Rhodium Standard 
//! Repository (RSR) framework to ensure high-assurance and 
//! safety-critical operation.
//!
//! ARCHITECTURE (Phase 1 MVP):
//! - **Router**: MLP-based classifier for execution path optimization.
//! - **Expert System**: Symbolic rule engine for policy enforcement.
//! - **Context Manager**: Bitemporal conversation state using SQLite.
//! - **Orchestrator**: Master coordinator for the neural/symbolic bridge.
//!
//! SECURITY MANDATE:
//! - `#![forbid(unsafe_code)]`: Strict enforcement of Rust's memory safety.
//! - **Air-Gapped by Default**: All core functionality operates without 
//!   network access.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod context;
pub mod expert;
pub mod mlp;
pub mod orchestrator;
pub mod persistence;
pub mod router;
pub mod snn;
pub mod types;

// RE-EXPORTS: Primary types for mobile application integration.
pub use orchestrator::Orchestrator;
pub use types::{Query, Response, RoutingDecision};

/// Semantic version of the core framework.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// RSR Compliance level: Bronze.
pub const RSR_COMPLIANCE: &str = "Bronze";
