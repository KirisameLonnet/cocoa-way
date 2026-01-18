{
  description = "Cocoa-Way - A Wayland Compositor for macOS built with Rust and Smithay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Use stable Rust with edition 2024 support
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        # Build dependencies
        buildInputs = with pkgs; [
          # Required libraries for cocoa-way
          libxkbcommon
          pixman
        ] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
          # macOS frameworks (automatically provided by stdenv on Darwin)
          pkgs.libiconv
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];

      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          # Environment variables for build
          shellHook = ''
            echo "Cocoa-Way Development Environment"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build --release    Build the compositor"
            echo "  cargo run --release      Run the compositor"
            echo "  cargo test               Run tests"
            echo ""
          '';

          # Rust-specific environment
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
        };
      }
    );
}