{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # Rust setup.
        fenixPkgs = fenix.packages.${system};
        rust-toolchain = fenixPkgs.stable.toolchain;
        rust-env = [ rust-toolchain ] ++ (with pkgs; [
          rust-analyzer
          cargo-watch
          cargo-expand
        ]);

        # Crane setup.
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
        # Source.
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          buildInputs = [ ];
        };
        # Deps.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        my-crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          inherit my-crate;

          rust_fmt = craneLib.cargoFmt {
            inherit src;
          };

          rust_doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          rust_test = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = my-crate;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = rust-env;
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
