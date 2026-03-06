<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# Mobile AI Orchestrator — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              CONSUMER APP               │
                        │        (NeuroPhone, Edge Device)        │
                        └───────────────────┬─────────────────────┘
                                            │ Query
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           ORCHESTRATOR CORE             │
                        │    (Rust, Platform-Agnostic Library)    │
                        └──────────┬───────────────────┬──────────┘
                                   │                   │
                                   ▼                   ▼
                        ┌───────────────────────┐  ┌────────────────────────────────┐
                        │ EXPERT SYSTEM (SAFETY)│  │ CONTEXT MANAGER                │
                        │ - Rule-based gating   │  │ - Conversation History         │
                        │ - Privacy Filtering   │  │ - Project State Tracking       │
                        └──────────┬────────────┘  └──────────┬─────────────────────┘
                                   │                          │
                                   └────────────┬─────────────┘
                                                ▼
                        ┌─────────────────────────────────────────┐
                        │           ROUTER (DECISION)             │
                        │  ┌───────────┐  ┌───────────┐  ┌───────┐│
                        │  │ Heuristic │  │ MLP Learn │  │ ESN   ││
                        │  │ Baseline  │  │ Routing   │  │ Resvr ││
                        │  └─────┬─────┘  └───────────┘  └───────┘│
                        └────────│────────────────────────────────┘
                                 │
                                 ▼ Decision (Local vs Remote)
                        ┌───────────────────┬─────────────────────┐
                        │                   │                     │
                        ▼                   ▼                     ▼
                  ┌───────────┐       ┌───────────┐         ┌───────────┐
                  │ Local SLM │       │ Cloud API │         │  BLOCKED  │
                  │(llama.cpp)│       │ (Claude)  │         │ (Safety)  │
                  └───────────┘       └───────────┘         └───────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile / Cargo   .machine_readable/  │
                        │  Benchmarks         RSR Bronze (Cert)   │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
CORE ORCHESTRATION
  Orchestrator Logic (Rust)         ██████████ 100%    Pipeline execution stable
  Context Manager                   ██████████ 100%    History tracking verified
  Expert System (Safety)            ██████████ 100%    Rule-based gating active
  Router (Heuristic)                ██████████ 100%    Decision logic stable

NEURAL COMPONENTS (PHASE 2+)
  Reservoir Computing (ESN)         ████████░░  80%    Temporal compression stable
  MLP Learned Routing               ██████░░░░  60%    Inference active, training req
  SNN Wake Detection                ████░░░░░░  40%    Initial spikes prototyping

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard build/lint/test
  .machine_readable/                ██████████ 100%    STATE tracking active
  Performance Benchmarks            ██████████ 100%    Route decision < 50μs

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            █████████░  ~90%   Core library production-ready
```

## Key Dependencies

```
User Query ──────► Expert System ──────► Context Mgr ──────► Router
                        │                   │                 │
                        ▼                   ▼                 ▼
                  Safety Proof ────────► History ────────► Decision
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
