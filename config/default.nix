{ lib, ... }:

{
  dirs = rec {
    data = "../data";
    cache = data + "/cache";
  };

  libraries = [rec {
    name = "Movies";
    type = name;
    folders = [ (lib.toLower name) ];
  }];

  apiKeys = import ./keys.nix;
}
