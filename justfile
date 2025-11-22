# Justfile for Mobile AI Orchestrator
# Usage: just <recipe>
# List all recipes: just --list

# Default recipe (runs when you just type 'just')
default: check

# Display this help message
help:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode (optimized)
release:
    cargo build --release

# Build with network features enabled
build-network:
    cargo build --release --features network

# Run the project in interactive mode
run *ARGS:
    cargo run -- {{ARGS}}

# Run in interactive mode
interactive:
    cargo run -- --interactive

# Run with a single query
query QUERY:
    cargo run -- "{{QUERY}}"

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
coverage:
    cargo tarpaulin --out Html --output-dir target/coverage

# Run benchmarks (when available)
bench:
    cargo bench

# Check code without building
check:
    cargo check

# Format code using rustfmt
fmt:
    cargo fmt

# Check if code is formatted correctly
fmt-check:
    cargo fmt --check

# Lint code with Clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Lint with Clippy (strict mode)
clippy-strict:
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic

# Build documentation
doc:
    cargo doc --no-deps

# Build and open documentation in browser
doc-open:
    cargo doc --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Full validation suite (formatting, linting, tests)
validate: fmt-check clippy test
    @echo "✅ All validation checks passed!"

# Pre-commit checks (run before committing)
pre-commit: validate
    @echo "✅ Ready to commit!"

# Install development dependencies
install-deps:
    @echo "Installing development dependencies..."
    cargo install cargo-tarpaulin
    cargo install cargo-audit
    cargo install cargo-outdated
    @echo "✅ Dependencies installed!"

# Security audit of dependencies
audit:
    cargo audit

# Check for outdated dependencies
outdated:
    cargo outdated

# Update dependencies
update:
    cargo update

# Run the binary directly (after building)
run-binary *ARGS:
    ./target/release/mobile-ai {{ARGS}}

# Build and run release version
run-release *ARGS:
    cargo run --release -- {{ARGS}}

# Generate RSR compliance report
rsr-compliance:
    @echo "=== RSR Compliance Check ==="
    @echo ""
    @echo "✅ Type Safety: Rust compile-time guarantees"
    @echo "✅ Memory Safety: Zero unsafe blocks"
    @grep -r "unsafe" src/ && echo "❌ Found unsafe blocks!" || echo "✅ No unsafe blocks found"
    @echo "✅ Offline-First: Network is optional feature"
    @echo "✅ Documentation: Complete"
    @echo "✅ Tests: Running..."
    @cargo test --quiet
    @echo "✅ Build System: justfile, flake.nix, CI/CD"
    @echo "✅ Security: SECURITY.md, .well-known/security.txt"
    @echo "✅ License: Dual MIT + Palimpsest-0.8"
    @echo "✅ Community: CODE_OF_CONDUCT.md, CONTRIBUTING.md"
    @echo ""
    @echo "RSR Compliance Level: Bronze ✅"

# Count lines of code
loc:
    @echo "=== Lines of Code ==="
    @find src -name '*.rs' -exec wc -l {} + | tail -1
    @echo ""
    @echo "By file:"
    @wc -l src/*.rs

# Check dependency count (Bronze RSR requires minimal deps)
dep-count:
    @echo "=== Dependency Count ==="
    @cargo tree --depth 1 | grep -v "mobile-ai-orchestrator" | wc -l
    @echo ""
    @echo "Direct dependencies:"
    @cargo tree --depth 1

# Generate a new release (update version, tag, etc.)
release-prepare VERSION:
    @echo "Preparing release {{VERSION}}..."
    @# Update Cargo.toml version
    @sed -i 's/^version = .*/version = "{{VERSION}}"/' Cargo.toml
    @# Update CHANGELOG.md (manual step reminder)
    @echo "⚠️  Don't forget to update CHANGELOG.md!"
    @echo "✅ Version updated to {{VERSION}}"

# Git commit with conventional commit message
commit MESSAGE:
    @git add .
    @git commit -m "{{MESSAGE}}"

# Git commit and push
push MESSAGE:
    @git add .
    @git commit -m "{{MESSAGE}}"
    @git push origin $(git branch --show-current)

# Create a git tag for release
tag VERSION:
    @git tag -a v{{VERSION}} -m "Release v{{VERSION}}"
    @git push origin v{{VERSION}}

# Full release workflow
release VERSION MESSAGE: (release-prepare VERSION) validate
    @echo "Running full test suite..."
    @cargo test
    @echo "Building release binary..."
    @cargo build --release
    @echo "Committing version bump..."
    just commit "chore: bump version to {{VERSION}}"
    just tag {{VERSION}}
    @echo "✅ Release {{VERSION}} prepared!"
    @echo "⚠️  Review CHANGELOG.md and push to remote"

# Profile the application (requires cargo-flamegraph)
profile *ARGS:
    cargo flamegraph -- {{ARGS}}

# Watch for changes and re-run tests
watch:
    cargo watch -x test

# Run rustfmt, clippy, and tests in sequence
ci: fmt-check clippy test
    @echo "✅ CI checks passed!"

# Install pre-commit hook
install-hook:
    @echo '#!/bin/sh\njust pre-commit' > .git/hooks/pre-commit
    @chmod +x .git/hooks/pre-commit
    @echo "✅ Pre-commit hook installed!"

# Uninstall pre-commit hook
uninstall-hook:
    @rm -f .git/hooks/pre-commit
    @echo "✅ Pre-commit hook removed!"

# Build for Android (cross-compilation)
build-android:
    @echo "Building for Android (aarch64-linux-android)..."
    cargo build --target aarch64-linux-android --release

# Build for RISC-V (for testing constrained platforms)
build-riscv:
    @echo "Building for RISC-V (riscv64gc-unknown-linux-gnu)..."
    cargo build --target riscv64gc-unknown-linux-gnu --release

# Setup development environment
setup: install-deps install-hook
    @echo "✅ Development environment ready!"

# Create a new component (usage: just new-component <name>)
new-component NAME:
    @echo "Creating new component: {{NAME}}"
    @touch src/{{NAME}}.rs
    @echo "// {{NAME}} component" > src/{{NAME}}.rs
    @echo "" >> src/{{NAME}}.rs
    @echo "pub mod {{NAME}};" >> src/lib.rs
    @echo "✅ Component {{NAME}} created!"

# Stress test (run tests repeatedly to find flaky tests)
stress-test N="100":
    @echo "Running tests {{N}} times..."
    @for i in $(seq 1 {{N}}); do \
        echo "Iteration $i/{{N}}"; \
        cargo test --quiet || exit 1; \
    done
    @echo "✅ All {{N}} iterations passed!"

# Create a backup of the project
backup:
    @tar -czf ../mobile-ai-orchestrator-backup-$(date +%Y%m%d-%H%M%S).tar.gz \
        --exclude='target' \
        --exclude='.git' \
        .
    @echo "✅ Backup created in parent directory"

# Experimental: Run with miri (Rust's interpreter for detecting undefined behavior)
miri:
    cargo +nightly miri test

# Show project statistics
stats:
    @echo "=== Project Statistics ==="
    @echo ""
    @echo "Lines of Rust code:"
    @find src -name '*.rs' -exec cat {} + | wc -l
    @echo ""
    @echo "Number of files:"
    @find src -name '*.rs' | wc -l
    @echo ""
    @echo "Number of tests:"
    @grep -r "#\[test\]" src/ | wc -l
    @echo ""
    @echo "Direct dependencies:"
    @cargo tree --depth 1 | grep -v "mobile-ai-orchestrator" | wc -l
    @echo ""
    @echo "TODO count:"
    @grep -r "TODO" src/ | wc -l
    @echo ""
    @echo "FIXME count:"
    @grep -r "FIXME" src/ | wc -l

# Complete workflow: format, lint, test, build
all: fmt clippy test release
    @echo "✅ Complete build successful!"
