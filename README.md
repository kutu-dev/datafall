<div align="center">
<img src="./data/icons/icon.svg" alt="Logo of datafall" width="200">
<h1>Nubosa</h1>

![Linux logo](https://img.shields.io/badge/Linux-%2301A1EE?style=flat&logo=linux&logoColor=FFFFFF)
![macOS logo](https://img.shields.io/badge/macOS-000000?style=flat&logo=apple&labelColor=000000)
![Rust stable logo](https://img.shields.io/badge/Rust-stable-%23F74B00?style=flat&logo=rust)
![Static Badge](https://img.shields.io/badge/Nix-devShell-%235073BE?style=flat&logo=nixos&logoColor=FFFFFF)

An Adwaita application for multithreded HTTP file downloading.
</div>

## Installing

### On a system with Nix available
DataFall is provided as a package on a Flake, just add it to your inputs and use the package when neeeded:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    datafall.url = "github:kutu-dev/datafall":
    datafall.inputs.nixpkgs.follows = "nixpkgs";
  };

# Your output function...

}
```

### On a generic Unix system
Manual compilation is needed.

## Compiling

### On a system with Nix available
The flake file included in this repo should make a working environment, just run:
```sh
nix develop
cargo run
```

### On a generic Unix system
You need to have installed on your system the following packages:
- `pkg-config`
- `open-ssl`
- `gtk4`
- `libadwaita`
- `librsvg`
- `adwaita-icon-theme`
- `dejavu_fonts`

And manually compile the crate with:
```sh
cargo run
```

## Note for Windows users
I have personally tried and failed to compile this program for Windows, unfortunately the GTK support outside the GNOME ecosystem is really lacking so cross-compiling with [`cross`](https://github.com/cross-rs/cross/tree/main) has been a near impossible task. I don't develop on Windows myself so `MSYS2` is not a viable option on my side and in any case it will probably also give lots of issues. 

## Acknowledgements
- Created with :heart: by [Jorge "Kutu" Dobón Blanco](https://dobon.dev).
