# dacho/default.nix

{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = (with pkgs; [
    vulkan-validation-layers
  ]);

  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ with pkgs; lib.makeLibraryPath [
    libxkbcommon
    vulkan-loader
    wayland
  ] }";
}

