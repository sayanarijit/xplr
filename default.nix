with import <nixpkgs> { };

# Update the src url, version and sha256 when new version

rustPlatform.buildRustPackage rec {
  name = "xplr";
  version = "0.3.2";
  src = fetchTarball
    ("https://github.com/sayanarijit/xplr/archive/refs/tags/v0.3.2.tar.gz");
  buildInputs = [ cargo ];
  checkPhase = "";
  cargoSha256 = "sha256-IyaYkHXmqXziXNK6uYU+XNNWA8a8S8cuMxkopps/9kk=";
}

