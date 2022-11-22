{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, crate2nix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
#        overlays = [
#              rust-overlay.overlay
#              (self: super: {
#                rustc = self.rust-bin.stable.latest.default;
#                cargo = self.rust-bin.stable.latest.default;
#              })
#            ];
        pkgs = import nixpkgs { inherit system; };
        crate2nix-tools = import "${crate2nix}/tools.nix" { inherit pkgs; };
        crateName = "test-oauth-server";
        generatedCargoNix = crate2nix-tools.generatedCargoNix {
            name = crateName;
            src = ./.;
        };
        called = pkgs.callPackage "${generatedCargoNix}/default.nix" {
#          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            # Crate dependency overrides go here
#          };
        };

      in {
        packages.${crateName} = called.rootCrate.build;
        defaultPackage = self.packages.${system}.${crateName};

        # devShell = pkgs.mkShell {
        #   inputsFrom = builtins.attrValues self.packages.${system};
        #   buildInputs = [ pkgs.cargo pkgs.rust-analyzer pkgs.clippy ];
        # };
      });
}