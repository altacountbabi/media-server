{ pkgs ? import <nixpkgs> { } }:

with pkgs;
let manifest = (lib.importTOML ./Cargo.toml).package;
in rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;

  nativeBuildInputs = [ openssl pkg-config rustPlatform.bindgenHook ];
  buildInputs = [ openssl ffmpeg_7-headless ];
}
