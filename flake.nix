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

    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        name = "datafall";
        nativeBuildInputs = with pkgs; [
          rustup
          pkg-config
          openssl
          ripgrep
          gtk4
        ];
      };
    });
  };
}
