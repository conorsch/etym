{
  description = "PyO3 Python bindings for etym";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        python = pkgs.python311;
      in
      {
        packages.etym-py = python.pkgs.buildPythonPackage {
          pname = "etym-py";
          version = "0.1.0";
          format = "pyproject";

          src = ./.;

          cargoDeps = pkgs.rustPlatform.importCargoLock {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            rustToolchain
            pkgs.rustPlatform.cargoSetupHook
            pkgs.rustPlatform.maturinBuildHook
            python.pkgs.hatchling
            python.pkgs.hatch-maturin-build
          ];

          pythonImportsCheck = [ "etym" ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.just
            pkgs.maturin
            python
            python.pkgs.hatchling
            python.pkgs.pytest
          ];
        };
      }
    );
}