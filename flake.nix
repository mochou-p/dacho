{
  description = ''
    dacho - rust vulkan game engine

    1. building/running -> `nix develop` (default shell)
    + nightly rust, clippy
    + x11, wayland
    + xkb, libudev
    + glslang, vulkan

    2. debugging -> `nix develop .#debug` (inherits the default shell)
    + vkconfig, renderdoc
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

            systemd
            wayland
          ];

          buildInputs = [
            glslang
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath [
              libxkbcommon
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
            ] }"
          '';
        };

        defaultShellMessage = ''
          echo -e "\n  \x1b[100m          \x1b[1mwelcome to the \x1b[32mdefault dacho shell\x1b[39m!\x1b[22m           \x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mbuild the example\x1b[30;100m▊\x1b[0;40;1m cargo build --release              \x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mcompile shaders  \x1b[30;100m▊\x1b[0;40;1m ./examples/usage/compile_shaders.sh\x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mrun the example  \x1b[30;100m▊\x1b[0;40;1m ./target/release/usage             \x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[90m▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔\x1b[0m"
        '';
        debugShellMessage = ''
          echo -e "\n  \x1b[100m           \x1b[1mwelcome to the \x1b[33mdebug dacho shell\x1b[39m!\x1b[22m            \x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mbuild the example\x1b[30;100m▊\x1b[0;40;1m cargo build                        \x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mcompile shaders  \x1b[30;100m▊\x1b[0;40;1m ./examples/usage/compile_shaders.sh\x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mprepare vk layers\x1b[30;100m▊\x1b[0;40;1m vkconfig-gui &                     \x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[40;90m▎\x1b[39mdebug graphics   \x1b[30;100m▊\x1b[0;40;1m qrenderdoc                         \x1b[30;100m▊\x1b[0m"
          echo -e "  \x1b[90m▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔\x1b[0m"
        '';
      in
      {
        devShells = with pkgs; {
          default = mkShell (defaultShell // {
            shellHook = defaultShell.shellHook + defaultShellMessage;
          });

          debug = mkShell (defaultShell // {
            nativeBuildInputs = defaultShell.nativeBuildInputs ++ [
              renderdoc
              vulkan-tools-lunarg
            ];

            shellHook = defaultShell.shellHook + ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath [
                vulkan-validation-layers
              ] }"

              export VK_LAYER_PATH="$VK_LAYER_PATH:${vulkan-validation-layers}/share/vulkan/explicit_layer.d"

              ${debugShellMessage}
            '';
          });
        };
      }
    );
}
