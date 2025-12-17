{
  description = "Mobile AI Orchestrator - Hybrid AI system for constrained mobile platforms";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Project dependencies
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          just
        ];

        buildInputs = with pkgs; [
          # Add system libraries here if needed in future phases
          # For Phase 1, we have minimal dependencies
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";

          shellHook = ''
            echo "🦀 Mobile AI Orchestrator Development Environment"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  just --list        # List all Just recipes"
            echo "  just build         # Build the project"
            echo "  just test          # Run tests"
            echo "  just validate      # Run full validation suite"
            echo "  cargo run          # Run the application"
            echo ""
            echo "RSR Compliance: Bronze ✅"
            echo "Safety: Zero unsafe blocks 🔒"
            echo "Offline-first: Network optional 🌐"
            echo ""
          '';
        };

        # Package definition
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "mobile-ai-orchestrator";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          # Build flags
          buildFeatures = [ ]; # No network feature by default (offline-first)

          # Tests
          doCheck = true;

          meta = with pkgs.lib; {
            description = "Hybrid AI orchestration system for constrained mobile platforms";
            homepage = "https://github.com/Hyperpolymath/heterogenous-mobile-computing";
            # Dual-licensed: user may choose either MIT or AGPL-3.0-or-later
            license = with licenses; [ mit agpl3Plus ];
            maintainers = [{ name = "Jonathan Bowman"; email = "hyperpolymath@protonmail.com"; }];
            platforms = platforms.all;
          };
        };

        # Packages with different features
        packages.with-network = pkgs.rustPlatform.buildRustPackage {
          pname = "mobile-ai-orchestrator";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          buildFeatures = [ "network" ]; # Enable network features

          doCheck = true;

          meta = with pkgs.lib; {
            description = "Mobile AI Orchestrator (with network features)";
            homepage = "https://github.com/Hyperpolymath/heterogenous-mobile-computing";
            # Dual-licensed: user may choose either MIT or AGPL-3.0-or-later
            license = with licenses; [ mit agpl3Plus ];
            maintainers = [{ name = "Jonathan Bowman"; email = "hyperpolymath@protonmail.com"; }];
          };
        };

        # CI/CD checks
        checks = {
          # Formatting check
          format = pkgs.runCommand "check-format" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo fmt --check
            touch $out
          '';

          # Clippy linting
          clippy = pkgs.runCommand "clippy" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo clippy --all-targets --all-features -- -D warnings
            touch $out
          '';

          # Tests
          test = pkgs.runCommand "test" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo test
            touch $out
          '';

          # RSR compliance
          rsr = pkgs.runCommand "rsr-compliance" {
            buildInputs = [ rustToolchain pkgs.just ];
          } ''
            cd ${./.}
            just rsr-compliance
            touch $out
          '';
        };

        # App for nix run
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        # Formatter
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
