# Contributing to Mobile AI Orchestrator

Thank you for your interest in contributing! This project follows the **Tri-Perimeter Contribution Framework (TPCF)** and adheres to the **Rhodium Standard Repository (RSR)** principles.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Tri-Perimeter Contribution Framework](#tri-perimeter-contribution-framework)
3. [Development Setup](#development-setup)
4. [Contribution Guidelines](#contribution-guidelines)
5. [Code Standards](#code-standards)
6. [Testing Requirements](#testing-requirements)
7. [Documentation](#documentation)
8. [Pull Request Process](#pull-request-process)
9. [Code of Conduct](#code-of-conduct)

## Getting Started

### Quick Links

- **Issue Tracker**: https://github.com/Hyperpolymath/heterogenous-mobile-computing/issues
- **Discussions**: https://github.com/Hyperpolymath/heterogenous-mobile-computing/discussions
- **Matrix Chat**: (see MAINTAINERS.md)
- **Security**: See [SECURITY.md](SECURITY.md)

### Ways to Contribute

- 🐛 **Bug reports**: File issues with reproduction steps
- 💡 **Feature requests**: Propose new functionality
- 📝 **Documentation**: Improve docs, examples, architecture
- 🧪 **Testing**: Add tests, improve coverage
- 🔧 **Code**: Fix bugs, implement features
- 🎨 **Design**: UI/UX improvements (future Android app)
- 📊 **Performance**: Benchmarking, optimization
- 🔐 **Security**: Audits, vulnerability reports

## Tri-Perimeter Contribution Framework

This project operates under TPCF with **three graduated trust perimeters**:

### Perimeter 3: Community Sandbox (Current - You Are Here!)

**Access Level**: Open to all
**Permissions**: Fork, PR, issues, discussions
**Review Process**: All PRs reviewed by Perimeter 1-2 contributors
**Commit Access**: No direct commits; PR-based only

**Ideal for**:
- First-time contributors
- Bug fixes
- Documentation improvements
- Minor features
- Test additions

**Progression**: Demonstrate consistent high-quality contributions → Invited to Perimeter 2

### Perimeter 2: Curated Contributors

**Access Level**: Invitation only
**Permissions**: Direct push to feature branches, PR approval rights
**Review Process**: Lightweight review from Perimeter 1
**Responsibilities**: Code review, issue triage, community support

**Requirements**:
- 5+ accepted PRs from Perimeter 3
- Demonstrated domain expertise
- Alignment with project values (RSR, safety-first, offline-first)

**Progression**: Major architectural contributions + maintainer nomination → Perimeter 1

### Perimeter 1: Core Team

**Access Level**: Invitation only
**Permissions**: Push to `main`, release management, security fixes
**Responsibilities**: Architectural decisions, security, releases, governance

**Requirements**:
- Long-term commitment
- Deep domain expertise
- Security clearance for vulnerability handling

## Development Setup

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Install Just (task runner)
cargo install just

# (Optional) Install Nix for reproducible builds
# See: https://nixos.org/download.html
```

### Fork and Clone

```bash
# Fork on GitHub UI first, then:
git clone https://github.com/YOUR_USERNAME/heterogenous-mobile-computing
cd heterogenous-mobile-computing

# Add upstream remote
git remote add upstream https://github.com/Hyperpolymath/heterogenous-mobile-computing
```

### Build and Test

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Check code formatting
cargo fmt --check

# Lint with Clippy
cargo clippy -- -D warnings

# Build documentation
cargo doc --open

# Use Just recipes
just build    # Build release
just test     # Run all tests
just validate # RSR compliance check
```

## Contribution Guidelines

### Issue First, Code Second

**Before writing code**:
1. Search existing issues/PRs to avoid duplicates
2. File an issue describing the problem/feature
3. Wait for maintainer feedback (usually < 48 hours)
4. Get approval before starting large changes

**Exception**: Obvious bugs, typos, test additions don't need issues

### Branch Naming

```bash
# Feature branches
git checkout -b feature/add-reservoir-computing

# Bug fixes
git checkout -b fix/router-crash-on-empty-query

# Documentation
git checkout -b docs/improve-architecture-guide

# Tests
git checkout -b test/add-expert-system-fuzzing
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): brief description

Longer explanation (if needed).

Fixes #123
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code change that neither fixes nor adds feature
- `perf`: Performance improvement
- `chore`: Build system, dependencies
- `security`: Security fix (do NOT commit vulnerable code first!)

**Examples**:
```
feat(router): add MLP-based routing decision

Replaces heuristic routing with a small MLP trained on
user feedback. Improves accuracy from 78% to 91%.

Implements #42

---

fix(expert): prevent panic on empty query text

Expert system crashed when query.text was empty.
Added check at entry point.

Fixes #56

---

docs(architecture): add Phase 2 reservoir computing design

Details the Echo State Network implementation planned
for conversation context compression.
```

## Code Standards

### Rust Style

**Use `rustfmt` and `clippy`**:
```bash
# Format code
cargo fmt

# Lint code
cargo clippy --all-targets --all-features -- -D warnings
```

### Safety Requirements

**Mandatory**:
- ✅ Zero `unsafe` blocks (exception requires security review + justification)
- ✅ No `unwrap()` on user input (use `?`, `unwrap_or()`, or `expect()` with justification)
- ✅ All public functions documented with `///` doc comments
- ✅ Error handling via `Result<T, E>`, not panics

**Example (Good)**:
```rust
/// Route a query to the appropriate processing backend.
///
/// # Errors
///
/// Returns `Err` if the query is blocked by safety rules.
pub fn route(&self, query: &Query) -> Result<RoutingDecision, String> {
    let eval = self.expert.evaluate(query)?;
    // ...
}
```

**Example (Bad)**:
```rust
// ❌ Missing docs
// ❌ Panics on failure
pub fn route(&self, query: &Query) -> RoutingDecision {
    self.expert.evaluate(query).unwrap()  // ❌ unwrap on user input
}
```

### Performance Guidelines

- Use `&str` instead of `String` for function parameters (unless ownership needed)
- Prefer `Vec::with_capacity()` when size is known
- Clone only when necessary (prefer borrowing)
- Benchmark performance-critical code (use `criterion`)

### Dependency Policy

**Before adding a dependency**:
1. Check if std lib can do it (e.g., don't add `itertools` for simple chains)
2. Verify crate is actively maintained (last commit < 6 months)
3. Check download count (>100k preferred)
4. Audit security (use `cargo-audit`)
5. Minimize features (only enable what's needed)

**Must-have justification for**:
- Crates with >10 transitive dependencies
- Crates using `unsafe` internally
- Crates with unclear licensing

## Testing Requirements

### Minimum Coverage

- **Unit tests**: All public functions
- **Integration tests**: Main user workflows
- **Coverage**: >80% line coverage (aim for 90%+)

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let router = Router::new();
        let query = Query::new("test");

        // Act
        let (decision, confidence) = router.route(&query);

        // Assert
        assert_eq!(decision, RoutingDecision::Local);
        assert!(confidence > 0.7);
    }

    #[test]
    #[should_panic(expected = "Invalid query")]
    fn test_error_condition() {
        // Test that function panics as expected
    }
}
```

### Test Naming

- `test_<function>_<scenario>_<expected_outcome>`
- Examples:
  - `test_router_short_query_routes_local()`
  - `test_expert_api_key_in_query_blocks()`
  - `test_context_max_history_truncates()`

### Property-Based Testing (Future)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_router_always_returns_valid_decision(query_text in "\\PC{0,1000}") {
        let router = Router::new();
        let query = Query::new(query_text);
        let (decision, _) = router.route(&query);

        // Property: decision is always one of four variants
        assert!(matches!(decision,
            RoutingDecision::Local |
            RoutingDecision::Remote |
            RoutingDecision::Hybrid |
            RoutingDecision::Blocked
        ));
    }
}
```

## Documentation

### API Documentation

**All public items must have doc comments**:

```rust
/// Brief one-line description.
///
/// Longer explanation with usage details.
///
/// # Examples
///
/// ```
/// use mobile_ai_orchestrator::Router;
/// let router = Router::new();
/// ```
///
/// # Errors
///
/// Returns `Err` if...
///
/// # Panics
///
/// Panics if... (avoid this!)
///
/// # Safety
///
/// If using unsafe (should be extremely rare)
pub fn function() -> Result<(), Error> {
    // ...
}
```

### Architecture Documentation

- Significant changes → Update `claude.md`
- New components → Add section to architecture docs
- API changes → Update examples in README

### Changelog

Add entry to `CHANGELOG.md` under `[Unreleased]`:

```markdown
## [Unreleased]

### Added
- feat(reservoir): Echo State Network for context compression (#42)

### Fixed
- fix(expert): Prevent panic on empty query (#56)

### Changed
- refactor(router): Migrate to MLP-based routing (#58)
```

## Pull Request Process

### 1. Prepare Your PR

```bash
# Update from upstream
git fetch upstream
git rebase upstream/main

# Run full test suite
just test

# Format and lint
cargo fmt
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps

# Verify RSR compliance
just validate
```

### 2. Create Pull Request

**Template** (auto-populated):

```markdown
## Description
[Brief description of changes]

## Motivation
[Why is this change needed? Link to issue]

## Changes
- [ ] Add/modify code
- [ ] Add/update tests
- [ ] Update documentation
- [ ] Update CHANGELOG.md

## Testing
[How was this tested?]

## Checklist
- [ ] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [ ] Tests pass locally (`cargo test`)
- [ ] Documentation updated (if needed)
- [ ] No new `unsafe` blocks (or justified)
- [ ] CHANGELOG.md updated

Closes #[issue number]
```

### 3. Review Process

**Timeline**:
- Initial review: Within 48 hours
- Feedback cycles: As needed
- Merge decision: Within 7 days (for small PRs)

**What reviewers check**:
- ✅ Code correctness
- ✅ Test coverage
- ✅ Performance implications
- ✅ Security considerations
- ✅ Documentation quality
- ✅ RSR compliance

**Feedback incorporation**:
- Address all comments (or explain why not)
- Push fixup commits → squash before merge
- Be open to suggestions

### 4. Merge

**Merge criteria**:
- ✅ All tests pass (CI)
- ✅ At least one Perimeter 1-2 approval
- ✅ No unresolved comments
- ✅ Up to date with `main`
- ✅ CHANGELOG updated

**Merge style**:
- **Small PRs (<200 lines)**: Squash and merge
- **Large PRs**: Rebase and merge (preserve commits)

## Code of Conduct

This project adheres to the **Contributor Covenant Code of Conduct** (see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)).

**TL;DR**:
- Be respectful
- Be inclusive
- Focus on code and ideas, not people
- Harassment will not be tolerated

**Enforcement**:
- First violation: Warning
- Second violation: Temporary ban
- Third violation: Permanent ban

Report violations to: hyperpolymath@protonmail.com

## Recognition

Contributors will be recognized in:
- `MAINTAINERS.md` (for significant contributions)
- GitHub contributors graph
- Release notes
- Research paper acknowledgments (if applicable)

### Contributor Levels

**Bronze** (1-4 merged PRs):
- Listed in acknowledgments

**Silver** (5-9 merged PRs):
- Listed in MAINTAINERS.md
- Consideration for Perimeter 2

**Gold** (10+ merged PRs):
- Strong Perimeter 2 candidate
- Potential Perimeter 1 invitation

## Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: Questions, ideas, general chat
- **Matrix**: Real-time chat (see MAINTAINERS.md)
- **Email**: hyperpolymath@protonmail.com (for sensitive topics)

### Common Questions

**Q: How do I get started?**
A: Look for issues labeled `good-first-issue` or `help-wanted`.

**Q: I found a bug but don't know how to fix it**
A: File an issue with reproduction steps. Someone else will pick it up!

**Q: Can I add a dependency?**
A: File an issue first to discuss. Dependency minimalism is a core value.

**Q: My PR was rejected. Can I appeal?**
A: Absolutely. Comment on the PR with your reasoning, or escalate to Matrix chat.

**Q: Can I work on multiple issues at once?**
A: Yes, but prefer to finish one PR before starting another (to avoid conflicts).

## License

By contributing, you agree that your contributions will be dual-licensed under MIT + Palimpsest-0.8 (same as the project).

---

*Thank you for contributing to making AI on mobile devices safer, faster, and more private!*

*Last updated: 2025-11-22*
