{ pkgs ? import <nixpkgs> { } }:

with pkgs;
pkgs.mkShell {
  inputsFrom = [ (callPackage ./default.nix { }) ];
  packages = [ nixfmt-rfc-style ];
}
