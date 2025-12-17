# Mobile AI Orchestrator - Roadmap

**Last Updated**: 2025-12-17
**Current Version**: 0.1.0
**RSR Compliance**: Bronze

---

## Current Status: Phase 1 Complete

### Completed (v0.1.0)

| Component | Status | Tests | Coverage |
|-----------|--------|-------|----------|
| Expert System | Complete | 8 | >90% |
| Router (Heuristic) | Complete | 7 | >90% |
| Context Manager | Complete | 9 | >90% |
| Orchestrator | Complete | 7 | >90% |
| CLI Interface | Complete | - | Manual |
| Type System | Complete | 10 | 100% |
| Reservoir Computing | Complete | 8 | >90% |
| MLP Router | Complete | 8 | >90% |
| SNN Module | Complete | 8 | >90% |
| Persistence | Complete | 9 | >90% |
| Training | Complete | 6 | >90% |

**Total**: 92 unit tests + 2 doc tests = **94 tests passing**

### Recent Security Fixes (2025-12-17)

- [x] Fixed license inconsistency across SCM files
  - `guix.scm`: Now dual-licensed (MIT + AGPL-3.0-or-later)
  - `flake.nix`: Now dual-licensed (MIT + AGPL-3.0-or-later)
  - `Cargo.toml`: Now matches LICENSE.txt (MIT OR AGPL-3.0-or-later)
- [x] Fixed flaky SNN tests (`test_lif_neuron_reset`, `test_spiking_network_reset`)
- [x] All HTTPS URLs verified
- [x] No MD5/SHA1 hash usage (security compliant)
- [x] No hardcoded secrets detected

---

## Phase 2: Production Hardening

**Target**: v0.2.0
**Goal**: Silver RSR Compliance

### Security Enhancements

| Task | Priority | Status |
|------|----------|--------|
| Encrypted SQLite (SQLCipher) | High | Planned |
| API key management (OS keychain) | High | Planned |
| Rate limiting implementation | Medium | Planned |
| TLS certificate pinning | Medium | Planned |
| cargo-vet integration | Medium | Planned |
| Fuzz testing (cargo-fuzz) | Medium | Planned |

### Performance Optimizations

| Task | Priority | Status |
|------|----------|--------|
| Reservoir state persistence | High | Planned |
| MLP weight caching | High | Planned |
| Query batching | Medium | Planned |
| Lazy loading for large contexts | Medium | Planned |

### Features

| Task | Priority | Status |
|------|----------|--------|
| RAG system with vector database | High | Planned |
| Knowledge graph for project relationships | High | Planned |
| Cross-session context continuity | High | Planned |
| Proactive context loading | Medium | Planned |

---

## Phase 3: Advanced AI

**Target**: v0.3.0
**Goal**: Maintain Silver RSR

### Features

| Task | Priority | Status |
|------|----------|--------|
| Mixture of Experts (MoE) | High | Planned |
| Bayesian decision engine | High | Planned |
| Background monitoring daemon | Medium | Planned |
| Proactive assistance triggers | Medium | Planned |
| Context inference via graph walks | Medium | Planned |

### Integration

| Task | Priority | Status |
|------|----------|--------|
| Claude API integration | High | Planned |
| Mistral API fallback | Medium | Planned |
| Local llama.cpp inference | High | Planned |
| Model switching based on context | Medium | Planned |

---

## Phase 4: Mobile Deployment

**Target**: v1.0.0
**Goal**: Gold RSR Compliance

### Mobile-Specific

| Task | Priority | Status |
|------|----------|--------|
| Android ARM64 optimization | High | Planned |
| iOS support (via FFI) | Medium | Planned |
| Battery-aware scheduling | High | Planned |
| WASM sandboxing for models | Medium | Planned |

### Production Features

| Task | Priority | Status |
|------|----------|--------|
| SNN wake-word detection | Medium | Planned |
| Reinforcement learning personalization | Low | Planned |
| On-device fine-tuning | Low | Planned |
| Federated learning support | Low | Planned |

---

## Build System Roadmap

### Package Management

| System | Status | File |
|--------|--------|------|
| Cargo | Complete | `Cargo.toml` |
| Nix | Complete | `flake.nix` |
| Guix | Complete | `guix.scm` |
| Docker/Containerfile | Planned | - |

### CI/CD

| Platform | Status | File |
|----------|--------|------|
| GitLab CI | Complete | `.gitlab-ci.yml` |
| GitHub Actions | Complete | `.github/workflows/` |
| Codecov | Complete | Integrated |
| SLSA 3 Provenance | Complete | `generator-generic-ossf-slsa3-publish.yml` |

---

## Security Roadmap

### Phase 1 (Current)

- [x] Zero `unsafe` blocks (`#![forbid(unsafe_code)]`)
- [x] Memory safety (Rust ownership)
- [x] Input validation (Expert system)
- [x] Minimal dependencies (4 core)
- [x] `cargo audit` integration
- [x] SECURITY.md + security.txt

### Phase 2 (Planned)

- [ ] Encrypted storage (SQLCipher)
- [ ] Keychain integration (OS-native)
- [ ] Rate limiting
- [ ] TLS pinning
- [ ] cargo-vet

### Phase 3+ (Future)

- [ ] Formal verification (Rust model checking)
- [ ] Third-party security audit
- [ ] WASM sandboxing
- [ ] Hardware security module support

---

## RSR Compliance Targets

| Version | Target Level | Key Requirements |
|---------|--------------|------------------|
| 0.1.0 | Bronze | Type safety, memory safety, offline-first |
| 0.2.0 | Silver | Encryption, rate limiting, >95% coverage |
| 1.0.0 | Gold | Formal verification, third-party audit |

---

## Known Limitations (Phase 1)

1. **No encryption at rest**: Context history stored in plaintext
   - Mitigation: Use full-disk encryption
   - Timeline: Phase 2

2. **No API key protection**: Requires environment variables
   - Mitigation: Never commit keys
   - Timeline: Phase 2 (keychain integration)

3. **No rate limiting**: Potential battery drain
   - Mitigation: None currently
   - Timeline: Phase 2

4. **Expert rules not comprehensive**: May miss novel attack patterns
   - Mitigation: Conservative allow-list
   - Timeline: Ongoing improvement

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to help with roadmap items.

**Priority areas**:
1. Security enhancements (Phase 2)
2. Test coverage improvements
3. Documentation
4. Cross-platform testing

---

## Version History

| Version | Date | Milestone |
|---------|------|-----------|
| 0.1.0 | 2025-11-22 | Phase 1 MVP |
| 0.2.0 | TBD | Production Hardening |
| 0.3.0 | TBD | Advanced AI |
| 1.0.0 | TBD | Mobile Deployment |

---

*This roadmap is updated regularly. For the latest status, check the GitHub/GitLab issue tracker.*
