{
  description = "xplr - A hackable, minimal, fast TUI file explorer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nix.url = "github:domenkozar/nix/relaxed-flakes";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, nix, ... }:
    let
      systems = [
        "x86_64-linux"
        "i686-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-linux-android"
        "aarch64-darwin"
      ];
      forAllSystems = f: builtins.listToAttrs (map (name: { inherit name; value = f name; }) systems);
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          xplr = pkgs.rustPlatform.buildRustPackage rec {
            name = "xplr";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        }
      );
      defaultPackage = forAllSystems (system: self.packages.${system}.xplr);
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          devRequirements = with pkgs; [
            gcc
            gnumake
            clippy
            rustc
            cargo
            rustfmt
            rust-analyzer
          ];
        in
        {
          default = pkgs.mkShell {
            RUST_BACKTRACE = 1;

            buildInputs = devRequirements;
            packages = devRequirements;
          };
        }
      );
    };
}
