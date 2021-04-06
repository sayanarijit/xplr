with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "xplr";
  version = "1.0";
  src = fetchTarball ("https://github.com/sayanarijit/xplr/tarball/main");
  buildInputs = [ cargo ];

  checkPhase = "";
  cargoSha256 = "sha256-IyaYkHXmqXziXNK6uYU+XNNWA8a8S8cuMxkopps/9kk=";

}

