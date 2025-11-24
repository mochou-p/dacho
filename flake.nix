{
  description = "nightly rust, x11/wayland, vulkan, glslang";

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
      in
      {
        devShells.default = with pkgs; mkShell {
          # compile time only dependencies (rust)
          nativeBuildInputs = [
            openssl
            pkg-config
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
                extensions = [ "clippy-preview" ];
              })
            )
          ];

          # compile/runtime dependencies
          buildInputs = [
            glslang                  # shader compiler
            libxkbcommon             # keyboard
            vulkan-loader
            vulkan-tools-lunarg      # vkconfig (dynamic validation layers)
            vulkan-validation-layers # for vkconfig
            wayland
          ];

          # runtime links
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath [
              libxkbcommon
              vulkan-loader
              vulkan-validation-layers
              wayland
            ] }"

            export VK_LAYER_PATH="$VK_LAYER_PATH:${vulkan-validation-layers}/share/vulkan/explicit_layer.d"
          '';
        };
      }
    );
}
