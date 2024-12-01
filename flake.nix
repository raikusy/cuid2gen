# This flake was initially generated by fh, the CLI for FlakeHub (version 0.1.18)
{
  # A helpful description of your flake
  description = "A fast and secure command-line tool for generating [CUID2](https://github.com/paralleldrive/cuid2) identifiers - Collision-resistant Unique IDs.";

  # Flake inputs
  inputs = {
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/*";

    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*";

    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # Flake outputs that other flakes can use
  outputs = {
    self,
    flake-compat,
    flake-schemas,
    nixpkgs,
    rust-overlay,
  }: let
    # Nixpkgs overlays
    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchain = final.rust-bin.stable.latest.default.override {extensions = ["rust-src"];};
      })
    ];

    # Helpers for producing system-specific outputs
    supportedSystems = ["x86_64-linux" "aarch64-darwin" "x86_64-darwin" "aarch64-linux"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit overlays system;};
        });
  in {
    # Schemas tell Nix about the structure of your flake's outputs
    schemas = flake-schemas.schemas;

    # Default outputs
    defaultNix = flake-compat.defaultNix;
    shellNix = flake-compat.shellNix;

    packages = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        # Pinned packages available in the environment
        packages = with pkgs; [
          rustToolchain
        ];
      };
    });

    # Development environments
    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        # Pinned packages available in the environment
        packages = with pkgs; [
          rustToolchain
          cargo-bloat
          cargo-edit
          cargo-outdated
          cargo-udeps
          cargo-watch
          rust-analyzer
          curl
          git
          jq
          wget
          nixpkgs-fmt
        ];

        # Environment variables
        env = {
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });
  };
}
