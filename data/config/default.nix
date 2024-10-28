{ lib, ... }:

{
  dataDir = "/home/real/projects/media-server/data";

  libraries = [rec {
    name = "Movies";
    type = name;
    folders = [ (lib.toLower name) ];
  }];
}
