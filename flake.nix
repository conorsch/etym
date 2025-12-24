{
  description = "Dev shell for etym CLI program";
  # inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
        };


        # Defining package list outside of devshell, so it can be used in devshell & container image.
        tooling = with pkgs; [
          bashInteractive
          coreutils
          fd
          file
          glibcLocales
          gum
          jq
          just
          nix-prefetch-scripts
          shellcheck
        ];
        craneLib = crane.mkLib pkgs;
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        devShells.default = pkgs.mkShell {
          name = "etym";
          nativeBuildInputs = [ pkgs.bashInteractive ];
          buildInputs = tooling;
        };

        packages.etym = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (craneLib.filterCargoSources path type) ||
              (builtins.match ".*/tests/fixture-.*\.html$" path != null);
          };
          pname = "etym";
          version = cargoToml.package.version;
          meta = {
            description = "Queries EtymOnline.com to look up etymologies for words";
            homepage = "https://github.com/conorsch/etym";
          };
        };

        # Make the binary the default package
        packages.default = self.packages.${system}.etym;
      });
}
