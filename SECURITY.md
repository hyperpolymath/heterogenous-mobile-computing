# Security Policy

## Supported Versions

| Version | Supported          | RSR Level |
| ------- | ------------------ | --------- |
| 0.1.x   | :white_check_mark: | Bronze    |

## Security Model

This project implements a **defense-in-depth** security model:

### 1. Memory Safety (Compile-Time)

- **Zero `unsafe` blocks**: `#![forbid(unsafe_code)]` in all modules
- **Ownership system**: Rust's borrow checker prevents use-after-free, double-free, data races
- **Type safety**: Strong static typing prevents type confusion attacks
- **Bounds checking**: All array/slice access is bounds-checked

### 2. Input Validation (Runtime)

- **Expert system rules**: Automatic detection of sensitive data (API keys, passwords)
- **Length limits**: Maximum query length enforced
- **Sanitization**: User input never directly interpolated into commands
- **Rate limiting**: (Future) Prevent DoS via excessive queries

### 3. Data Protection

- **Privacy-first**: On-device processing by default
- **Optional network**: Remote APIs require explicit `--features network` flag
- **No telemetry**: Zero data collection or analytics
- **Minimal dependencies**: 2 required dependencies in Phase 1 (Bronze compliance)

### 4. Cryptographic Guarantees (Future)

- **Encrypted storage**: SQLite database encryption (Phase 2)
- **API key management**: Keychain integration (Phase 2)
- **TLS verification**: Certificate pinning for API calls (Phase 2)

## Reporting a Vulnerability

### Contact Methods

**Preferred**: Email encryption

```
Email: hyperpolymath@protonmail.com
PGP Key: (see .well-known/security.txt)
Response Time: Within 72 hours
```

**Alternative**: GitHub Security Advisory

```
URL: https://github.com/Hyperpolymath/heterogenous-mobile-computing/security/advisories/new
```

### What to Include

1. **Description**: Detailed explanation of the vulnerability
2. **Impact**: What an attacker could achieve
3. **Reproduction**: Step-by-step instructions
4. **Environment**: OS, Rust version, device details
5. **Suggested fix**: (Optional but appreciated)

### Example Report Template

```markdown
## Vulnerability: [Title]

**Severity**: [Critical/High/Medium/Low]

**Description**:
[Detailed description of the issue]

**Impact**:
- Confidentiality: [High/Medium/Low/None]
- Integrity: [High/Medium/Low/None]
- Availability: [High/Medium/Low/None]

**Reproduction**:
1. Step one
2. Step two
3. ...

**Environment**:
- OS: [e.g., Android 12, Termux]
- Rust version: [e.g., 1.75.0]
- Device: [e.g., Oppo Reno 7]

**Suggested Fix**:
[Optional]
```

## Security Update Process

### Timeline

1. **T+0 hours**: Vulnerability reported
2. **T+72 hours**: Initial response sent
3. **T+7 days**: Severity assessment complete
4. **T+30 days**: Patch developed and tested (for High/Critical)
5. **T+30 days**: Public disclosure (coordinated)

### Severity Levels

**Critical** (CVSS 9.0-10.0):
- Remote code execution
- Arbitrary memory corruption
- Bypass of all safety mechanisms

**High** (CVSS 7.0-8.9):
- Privilege escalation
- Sensitive data exposure
- Safety rule bypass

**Medium** (CVSS 4.0-6.9):
- DoS attacks
- Information disclosure (limited)
- Logic errors

**Low** (CVSS 0.1-3.9):
- Minor information leaks
- Cosmetic issues with security implications

## Known Security Limitations

### Phase 1 (Current)

1. **No encryption at rest**: Context history stored in plaintext
   - **Mitigation**: Use full-disk encryption on device
   - **Timeline**: Phase 2 will add SQLite encryption

2. **No API key protection**: Network feature (if enabled) requires hardcoded keys
   - **Mitigation**: Use environment variables, never commit keys
   - **Timeline**: Phase 2 will add keychain integration

3. **No rate limiting**: User can spam queries
   - **Impact**: Battery drain, potential DoS of local device
   - **Mitigation**: None currently
   - **Timeline**: Phase 2 will add query throttling

4. **Expert system rules not comprehensive**: May miss novel attack patterns
   - **Impact**: Sensitive data could slip through heuristics
   - **Mitigation**: Conservative allow-list approach, regular rule updates
   - **Timeline**: Ongoing improvement

### Non-Vulnerabilities (By Design)

1. **No sandboxing of local SLM**: Model runs in same process
   - **Rationale**: Phase 1 uses mock inference; Phase 2 will use llama.cpp (C++ FFI)
   - **Future**: Phase 3 may add WASM sandboxing for untrusted models

2. **No supply chain verification**: Dependencies not cryptographically verified
   - **Rationale**: Relying on `cargo`'s checksums and crates.io trust model
   - **Future**: Consider `cargo-vet` integration

## Security Checklist for Contributors

Before submitting a PR, ensure:

- [ ] No new `unsafe` blocks added (exception requires security review)
- [ ] No hardcoded secrets or API keys
- [ ] No `unwrap()` on user input (use `?` or `unwrap_or_default()`)
- [ ] All new expert system rules have tests
- [ ] No SQL injection vectors (if adding database code)
- [ ] No command injection vectors (if adding shell execution)
- [ ] Fuzz testing for new parsing code (if applicable)

## Threat Model

### In-Scope Threats

✅ **Malicious queries**: User attempts to extract model weights, jailbreak, etc.
✅ **Data exfiltration**: Attacker tries to leak conversation history
✅ **Privacy violations**: Sensitive data sent to remote API without consent
✅ **Resource exhaustion**: DoS via excessive/large queries
✅ **Supply chain attacks**: Compromised dependencies

### Out-of-Scope Threats

❌ **Physical access**: Attacker with unlocked device can read all data
❌ **Rooted/jailbroken devices**: OS-level compromise is out of scope
❌ **Side-channel attacks**: Timing attacks, speculative execution, etc.
❌ **Hardware attacks**: Fault injection, voltage glitching, etc.
❌ **Social engineering**: Tricking user into running malicious commands

## Secure Configuration Guide

### Recommended Settings

```toml
# Cargo.toml
[profile.release]
overflow-checks = true  # Detect integer overflow in production
strip = true            # Remove symbols to prevent reverse engineering
lto = true              # Link-time optimization
```

### Environment Variables

```bash
# Never commit these!
export CLAUDE_API_KEY="sk-..."      # Store in password manager
export MISTRAL_API_KEY="..."
export RUST_BACKTRACE=0              # Don't leak stack traces in production
```

### Runtime Flags

```bash
# Offline-only mode (most secure)
mobile-ai --offline-only "query"

# Network mode (if needed)
mobile-ai --features network "query"

# Maximum privacy
mobile-ai --no-logging --no-telemetry "query"
```

## Dependency Security

### Current Dependencies (Phase 1)

| Crate | Version | Purpose | Audit Status |
|-------|---------|---------|--------------|
| `serde` | 1.0 | Serialization | ✅ Widely audited |
| `serde_json` | 1.0 | JSON parsing | ✅ Widely audited |

### Dependency Policy

1. **Minimize dependencies**: Only add if absolutely necessary
2. **Audit all additions**: Review source code before adding
3. **Pin versions**: Exact versions in `Cargo.lock`
4. **Monitor CVEs**: Use `cargo-audit` in CI
5. **Consider alternatives**: Prefer std lib where possible

## Cryptographic Disclosure

### Current Cryptography Use

**Phase 1**: None

**Phase 2** (planned):
- **SQLite encryption**: SQLCipher (AES-256-CBC)
- **API key storage**: OS keychain (platform-specific)
- **TLS**: `rustls` (no OpenSSL dependency)

### Export Control

This software uses only weak cryptography (< 56-bit) and is not subject to U.S. export controls. Future versions may include strong cryptography and will be updated accordingly.

## Security Scorecard

| Category | Status | Evidence |
|----------|--------|----------|
| Memory safety | ✅ Pass | Zero `unsafe`, Rust compiler |
| Type safety | ✅ Pass | Rust type system |
| Input validation | ✅ Pass | Expert system tests |
| Dependency audit | ✅ Pass | 2 dependencies, widely used |
| Fuzz testing | ⚠️ Partial | Planned for Phase 2 |
| Cryptography | 🔵 N/A | No crypto in Phase 1 |
| Third-party audit | 🔵 Pending | Seeking security review |

## Hall of Fame

Contributors who responsibly disclose security vulnerabilities:

*None yet - be the first!*

## References

- **Rust Security Book**: https://anssi-fr.github.io/rust-guide/
- **OWASP Mobile Top 10**: https://owasp.org/www-project-mobile-top-10/
- **RFC 9116 (security.txt)**: https://www.rfc-editor.org/rfc/rfc9116
- **CWE Top 25**: https://cwe.mitre.org/top25/

---

*Last updated: 2025-11-22*
*Security contact: .well-known/security.txt*
