# TEST-NEEDS.md — heterogenous-mobile-computing

## CRG Grade: C — ACHIEVED 2026-04-04

## Current Test State

| Category | Count | Notes |
|----------|-------|-------|
| Smoke tests | 3 | tests/smoke_test.rs — crate compilation, stdlib, edition 2021 |
| Benchmarks | 1 dir | benches/ present |

## What's Covered

- [x] Crate links without panic (`smoke_test.rs`)
- [x] Basic allocation and iterator semantics

## Still Missing (for CRG B+)

- [ ] Unit tests for Router, Expert, Orchestrator modules
- [ ] Property tests for context serialization
- [ ] Integration tests for sensor/reservoir pipeline

## Run Tests

```bash
cargo test
```
