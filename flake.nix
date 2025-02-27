# dacho/flake.nix

{
  description = "rust nightly";

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
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
                extensions = [
                  "clippy-preview"
                  "rust-analyzer-preview"
                  "rust-src"
                  "rustc-codegen-cranelift-preview"
                ];
              })
            )
          ];
        };
      }
    );
}

