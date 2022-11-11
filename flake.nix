{
  description = "xplr - A hackable, minimal, fast TUI file explorer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { self, nixpkgs }:
    {
      packages.x86_64-linux.default =
        with import nixpkgs { system = "x86_64-linux"; };

        rustPlatform.buildRustPackage rec {
          name = "xplr";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
    };
}
