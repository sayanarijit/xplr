with import <nixpkgs> {};

# Run nix-build and update the src url, version and sha256 when new version

rustPlatform.buildRustPackage rec {
  name = "xplr";
  version = "0.4.2";
  src = fetchTarball
    ("https://github.com/sayanarijit/xplr/archive/refs/tags/v0.4.2.tar.gz");
  buildInputs = [ cargo ];
  checkPhase = "";
  cargoSha256 = "0000000000000000000000000000000000000000000000000000";
}
