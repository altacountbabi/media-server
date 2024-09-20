{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            surrealdb = prev.surrealdb.overrideAttrs (oldAttrs: rec {
              version = "2.0.1";

              src = prev.fetchFromGitHub {
                owner = "surrealdb";
                repo = "surrealdb";
                rev = "v${version}";
                hash = "sha256-JFkTD/MGvak8EuDEABGH1xLykSNj4rtnnENAruls6W8=";
              };

              cargoDeps = oldAttrs.cargoDeps.overrideAttrs (pkgs.lib.const {
                name = "${oldAttrs.pname}-vendor.tar.gz";
                inherit src;
                outputHash =
                  "sha256-7gwhQHKahiW/2+IMgFH1VyFjLh8dY/uejHthKzp24Uc=";
              });
            });
          })
        ];
        pkgs = import nixpkgs {
          config = { allowUnfree = true; };
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml;
      in {
        devShells.default = pkgs.mkShell {
          packages = (with pkgs; [ trunk surrealdb ]) ++ [ rustToolchain ];
        };
      });
}
