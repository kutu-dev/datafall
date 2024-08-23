#This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

{
  # TODO: Add description
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = [
      "x86_64-linux"
      "aarch64-darwin"
    ];

    forAllSystems = fn:
      nixpkgs.lib.genAttrs allSystems
      (system: fn {pkgs = import nixpkgs {inherit system;};});
  in {
    formatter = forAllSystems ({pkgs}: pkgs.alejandra);

    packages = forAllSystems ({pkgs}: {
      datafall = pkgs.rustPlatform.buildRustPackage {
        pname = "datafall";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;

        buildInputs = with pkgs; [
          openssl.dev
          libadwaita
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
          ripgrep
          gtk4
          libadwaita
          librsvg
          adwaita-icon-theme
          dejavu_fonts
          wrapGAppsHook4
        ];
      };
    });

    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        name = "datafall";
        nativeBuildInputs = with pkgs; [
          rustup
          addlicense
          just
          cargo-cross

          pkg-config
          openssl
          ripgrep
          gtk4
          libadwaita
          librsvg
          adwaita-icon-theme
          dejavu_fonts
        ];
      };
    });
  };
}
