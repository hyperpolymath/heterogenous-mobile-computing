# Mobile AI Orchestrator

[![RSR Compliance](https://img.shields.io/badge/RSR-Bronze-cd7f32)](https://rhodium-standard.org)
[![License](https://img.shields.io/badge/license-MIT%20%2B%20Palimpsest--0.8-blue)](LICENSE.txt)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](.gitlab-ci.yml)
[![Safety](https://img.shields.io/badge/unsafe-forbidden-success)](src/lib.rs)

A hybrid AI orchestration system for constrained mobile platforms, combining on-device inference with intelligent routing to remote APIs.

## Features

🔒 **Safety-First**
- Zero `unsafe` blocks
- Type-safe by design (Rust ownership model)
- Memory-safe (compile-time guarantees)
- Formal rule-based safety layer

📱 **Mobile-Optimized**
- Designed for 2-4GB RAM devices
- Battery-aware processing
- Optimized for ARM (NEON intrinsics)
- Support for NPU/GPU acceleration (future)

🌐 **Offline-First**
- Core functionality works without internet
- Network features optional (feature flag)
- Graceful degradation
- Local SLM inference (1-3B parameters)

🧠 **Intelligent Routing**
- Automatic local vs. remote decisions
- Context-aware processing
- Project-specific state management
- Conversation history tracking

🔐 **Privacy-Preserving**
- Automatic detection of sensitive data
- Configurable blocking rules
- On-device processing by default
- Explainable routing decisions

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/Hyperpolymath/heterogenous-mobile-computing
cd heterogenous-mobile-computing

# Build (offline-first by default)
cargo build --release

# Run
./target/release/mobile-ai --help
```

### Usage

**Interactive mode:**
```bash
./target/release/mobile-ai --interactive
```

**Single query:**
```bash
./target/release/mobile-ai "How do I iterate a HashMap in Rust?"
```

**With project context:**
```bash
./target/release/mobile-ai --project oblibeny "Explain the type system"
```

**Enable network features:**
```bash
cargo build --release --features network
```

## Architecture

```
Query → Expert System → Router → Context → Inference → Response
          (Safety)      (Route)   (History)  (Local/API)
```

See [claude.md](claude.md) for comprehensive architecture documentation.

### Components

| Component | Purpose | Implementation |
|-----------|---------|----------------|
| **Expert System** | Rule-based safety | `src/expert.rs` |
| **Router** | Local/remote routing | `src/router.rs` |
| **Context Manager** | Conversation state | `src/context.rs` |
| **Orchestrator** | Main coordinator | `src/orchestrator.rs` |

## Phase 1 MVP (Implemented)

✅ Expert system (rule-based safety)
✅ Heuristic router (keyword-based routing)
✅ Context manager (in-memory history)
✅ Orchestrator (pipeline coordination)
✅ CLI interface
✅ Comprehensive tests (>90% coverage)

## Phase 2+ Features (Implemented)

✅ **Reservoir Computing** - Echo State Networks for temporal context compression
✅ **Neural Routing** - Multi-Layer Perceptron for learned routing decisions
✅ **Spiking Neural Networks** - Event-driven wake detection (ultra-low power)
✅ **Benchmarking Suite** - Performance profiling with Criterion
✅ **Example Applications** - Runnable demonstrations of all features

**Stats**: 69+ tests, 7,500+ lines of code, 10 modules

See [AUTONOMOUS_DEVELOPMENT_SUMMARY.md](AUTONOMOUS_DEVELOPMENT_SUMMARY.md) for details.

## Future Phases

**Phase 3: Integration & Training**
- [ ] Integrate MLP with router (replace heuristics)
- [ ] Replace bag-of-words with sentence-transformers
- [ ] Train reservoir on real conversation data
- [ ] Deploy SNN on DSP/NPU hardware

**Phase 4: Advanced Features**
- [ ] Mixture of Experts (specialized models)
- [ ] Bayesian decision engine
- [ ] SQLite persistence
- [ ] RAG system (document retrieval)
- [ ] Knowledge graph (project relationships)
- [ ] On-device fine-tuning
- [ ] Reinforcement learning from user feedback

## Development

### Prerequisites

- Rust 1.75+ (`rustup`)
- Just (`cargo install just`)
- Nix (optional, for reproducible builds)

### Building

```bash
# Standard build
cargo build

# Release build (optimized)
cargo build --release

# With network features
cargo build --features network

# Using Nix (reproducible)
nix build
```

### Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Benchmarks (future)
cargo bench

# RSR validation
just validate
```

### Documentation

**Core Documentation:**
- [claude.md](claude.md) - Comprehensive architecture (10,000+ words)
- [AUTONOMOUS_DEVELOPMENT_SUMMARY.md](AUTONOMOUS_DEVELOPMENT_SUMMARY.md) - Development progress and features

**Usage Guides:**
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Reservoir computing, MLP routing, SNN wake detection
- [DEPLOYMENT.md](DEPLOYMENT.md) - Platform deployment (Android, iOS, Linux mobile)
- [PERFORMANCE.md](PERFORMANCE.md) - Optimization techniques for constrained devices

**Examples:**
```bash
# Run basic usage example
cargo run --example basic_usage

# Reservoir computing demo
cargo run --example reservoir_demo

# MLP router demonstration
cargo run --example mlp_router
```

**API Documentation:**
```bash
# Generate API documentation
cargo doc --open
```

**Benchmarks:**
```bash
# Run performance benchmarks
cargo bench
```

## RSR Compliance

This project follows the **Rhodium Standard Repository** framework and achieves **Bronze-level** compliance:

✅ Type safety (Rust type system)
✅ Memory safety (ownership model, zero `unsafe`)
✅ Offline-first (network is optional)
✅ Comprehensive documentation
✅ Test coverage (>90%)
✅ Build system (`justfile`, `flake.nix`)
✅ CI/CD automation
✅ Security policy (`SECURITY.md`)
✅ Dual licensing (MIT + Palimpsest-0.8)
✅ Community governance (TPCF Perimeter 3)

See [RSR Compliance Checklist](docs/compliance-checklist.md) for details.

## Contributing

Contributions are welcome! This project operates under the **Tri-Perimeter Contribution Framework (TPCF)**.

**Current Perimeter**: 3 (Community Sandbox - fully open)

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Security

For security issues, see [SECURITY.md](SECURITY.md) or contact:
- Email: hyperpolymath@protonmail.com
- Security contact: `.well-known/security.txt`

## License

Dual-licensed under:
- [MIT License](LICENSE.txt)
- [Palimpsest License v0.8](LICENSE.txt)

You may choose either license for your use.

## Citation

If you use this work in research, please cite:

```bibtex
@software{mobile_ai_orchestrator_2025,
  author = {Bowman, Jonathan},
  title = {Mobile AI Orchestrator: Hybrid Architecture for Constrained Platforms},
  year = {2025},
  url = {https://github.com/Hyperpolymath/heterogenous-mobile-computing},
  note = {RSR Bronze-compliant, Phase 1 MVP}
}
```

## Related Projects

- **Echomesh**: Conversation context preservation across sessions
- **Oblíbený**: Safety-critical programming language with formal verification
- **UPM**: Universal Project Manager for multi-project workflows
- **CADRE**: Distributed state management via CRDTs

## Contact

**Author**: Jonathan Bowman (Hyperpolymath)
**Email**: hyperpolymath@protonmail.com
**Matrix**: (see MAINTAINERS.md)

## Acknowledgments

- **llama.cpp** for ARM-optimized LLM inference
- **Anthropic** for Claude API
- **Rust community** for safety-first tooling
- **RSR framework** for principled repository design

---

*Built with Rust 🦀 • RSR Bronze Compliant • Offline-First • Type-Safe • Memory-Safe*
