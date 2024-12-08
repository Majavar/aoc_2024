{
  description = "Advent of code 2024";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable."1.82.0".minimal.override {
          extensions = [ "rustfmt" "clippy" "rust-src" "rust-analyzer" ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell.override { stdenv = stdenvNoCC; } {
          buildInputs = [
            clojure
            jq
            perl
            python3
            rust
            swi-prolog
            sqlite
          ];
        };
      }
    );
}
