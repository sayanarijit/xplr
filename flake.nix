{
  description = "xplr - A hackable, minimal, fast TUI file explorer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = inputs@{ self, nixpkgs, ... }:
    let
      lib = nixpkgs.lib;

      darwin = [ "x86_64-darwin" "aarch64-darwin" ];
      linux = [ "x86_64-linux" "x86_64-linux-musl" "aarch64-linux" "aarch64-linux-android" "i86_64-linux" ];
      allSystems = darwin ++ linux;

      forEachSystem = systems: f: lib.genAttrs systems (system: f system);
      forAllSystems = forEachSystem allSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          # e.g. nix build .#xplr
          xplr = pkgs.rustPlatform.buildRustPackage rec {
            name = "xplr";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

          # e.g. nix build .#cross.x86_64-linux-musl.xplr --impure
          cross = forEachSystem (lib.filter (sys: sys != system) allSystems) (targetSystem:
            let
              crossPkgs = import nixpkgs { localSystem = system; crossSystem = targetSystem; };
            in
            { inherit (crossPkgs) xplr; }
          );
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

            # For cross compilation
            NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM = 1;

            buildInputs = devRequirements;
            packages = devRequirements;
          };
        }
      );
    };
}
