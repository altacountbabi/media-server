let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib stdenv;
in (import "$CONFIG") { inherit pkgs lib; }
