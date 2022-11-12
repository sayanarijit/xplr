{
  description = "xplr - A hackable, minimal, fast TUI file explorer";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        rec {
          packages = flake-utils.lib.flattenTree {
            xplr = pkgs.rustPlatform.buildRustPackage rec {
              name = "xplr";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            };
          };
          defaultPackage = packages.xplr;
        }
      );
}
