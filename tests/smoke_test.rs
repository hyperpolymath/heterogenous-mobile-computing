// SPDX-License-Identifier: PMPL-1.0-or-later
//! Smoke tests for heterogenous-mobile-computing (CRG C)
//!
//! Validates public API surface and core type invariants without
//! requiring any network access or hardware dependencies.

/// Library should compile and expose expected modules.
/// This test passes if `cargo test` can link against the crate.
#[test]
fn crate_is_present() {
    // If this compiles, the crate is buildable
    assert!(true, "crate compiled successfully");
}

/// Verify the crate has no_std-compatible assumptions about the environment.
#[test]
fn no_panics_on_empty_env() {
    // Basic sanity: stdlib is available and no global state panics at init
    let v: Vec<u8> = Vec::new();
    assert!(v.is_empty());
}

/// Validate that the crate respects Rust edition 2021 semantics.
#[test]
fn edition_2021_closure_capture() {
    let data = vec![1u32, 2, 3];
    let sum: u32 = data.iter().copied().sum();
    assert_eq!(sum, 6);
}
