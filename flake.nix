{
  outputs = {parts, ...} @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;

      perSystem = {
        self',
        inputs',
        lib,
        pkgs,
        system,
        ...
      }: let
        pkgsWithOverlays = inputs.nixpkgs.legacyPackages.${system}.extend inputs.rust-overlay.overlays.default;
        rustVersion = (builtins.fromTOML (builtins.readFile ./rust-toolchain)).toolchain."channel";
        rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };
      in {
        _module.args.pkgs = pkgsWithOverlays;

        formatter = pkgs.alejandra;

        legacyPackages = {inherit rust;};

        devShells.default = pkgs.callPackage ./shell.nix {};
      };
    };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    systems.url = "github:nix-systems/default-linux";
    systems.flake = false;
  };
}
