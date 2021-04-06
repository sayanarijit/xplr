with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "xplr";
  version = "0.3.1";
  src = fetchTarball
    ("https://github.com/sayanarijit/xplr/archive/refs/tags/v0.3.1.tar.gz");
  buildInputs = [ cargo ];
  checkPhase = "";
  cargoSha256 = "sha256-IyaYkHXmqXziXNK6uYU+XNNWA8a8S8cuMxkopps/9kk=";
}

