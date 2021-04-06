with import <nixpkgs> { };

# Run nix-build and update the src url, version and sha256 when new version

rustPlatform.buildRustPackage rec {
  name = "xplr";
  version = "0.3.3";
  src = fetchTarball
    ("https://github.com/sayanarijit/xplr/archive/refs/tags/v0.3.3.tar.gz");
  buildInputs = [ cargo ];
  checkPhase = "";
  cargoSha256 = "1wxb0ian7b9abi00i9v2wxkqiw71c3zcxrv5j89pqj5k23wzv04i";
}
