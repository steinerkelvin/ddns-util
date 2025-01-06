{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = (import nixpkgs) {
            inherit system;
          };

          shellPkgs = [
            pkgs.just
            pkgs.act
          ];

          naersk' = pkgs.callPackage naersk { };
        in
        {
          # For `nix build` & `nix run`:
          packages.default = naersk'.buildPackage {
            src = ./.;
            buildInputs = [
              pkgs.openssl
            ];
            nativeBuildInputs = [
              pkgs.pkg-config
            ];
          };

          # For `nix develop`:
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.pkg-config
              # pkgs.rustc
              # pkgs.cargo
            ];
            buildInputs = [
              pkgs.openssl
            ];
            packages = shellPkgs;
          };
        }
      ) // {
      nixosModules.k-ddns = import ./nix/modules/k-ddns.nix;
    };
}
