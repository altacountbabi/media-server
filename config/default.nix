{ lib, ... }:

{
  libraries = [rec {
    name = "Movies";
    type = name;
    folders = [ (lib.toLower name) ];
  }];
  apiKeys = import ./keys.nix;
}
