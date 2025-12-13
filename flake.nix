{
  description = ''
    dacho - rust vulkan game engine

    1. building/running -> `nix develop` (default shell)
    - for building: nightly rust & clippy, x11/wayland
    - for running:  glslang, xkb, vulkan

    2. debugging -> `nix develop .#debug`
    - inherits default shell + vkconfig-gui
  '';

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs     =    import nixpkgs { inherit system overlays; };

        defaultShell = with pkgs; {
          nativeBuildInputs = [
            openssl
            pkg-config
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
                extensions = [ "clippy-preview" ];
              })
            )

            wayland
          ];

          buildInputs = [
            glslang
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath [
              libxkbcommon
              vulkan-loader
            ] }"
          '';
        };
      in
      {
        devShells = with pkgs; {
          default = mkShell (defaultShell // {
            shellHook = defaultShell.shellHook + ''
              echo
              echo -e "\x1b[42;30;1m                     default shell                     \x1b[0m"
              echo -e "\x1b[40;32m build project   \x1b[2m:\x1b[22;92;1m cargo build                         \x1b[0m"
              echo -e "\x1b[40;32m compile shaders \x1b[2m:\x1b[22;92;1m ./examples/usage/compile_shaders.sh \x1b[0m"
              echo -e "\x1b[40;32m run example     \x1b[2m:\x1b[22;92;1m ./target/debug/usage                \x1b[0m"
              echo
            '';
          });

          debug = mkShell (defaultShell // {
            nativeBuildInputs = defaultShell.nativeBuildInputs ++ [
              vulkan-tools-lunarg
            ];

            shellHook = defaultShell.shellHook + ''
              echo
              echo -e "\x1b[43;30;1m                      debug shell                      \x1b[0m"
              echo -e "\x1b[40;33m build project   \x1b[2m:\x1b[22;93;1m cargo build                         \x1b[0m"
              echo -e "\x1b[40;33m compile shaders \x1b[2m:\x1b[22;93;1m ./examples/usage/compile_shaders.sh \x1b[0m"
              echo -e "\x1b[40;33m debug vulkan    \x1b[2m:\x1b[22;93;1m vkconfig-gui &                      \x1b[0m"
              echo -e "\x1b[40;33m run example     \x1b[2m:\x1b[22;93;1m ./target/debug/usage                \x1b[0m"
              echo
            '';
          });
        };
      }
    );
}
