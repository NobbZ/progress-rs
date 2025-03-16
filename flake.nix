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
        rustfmt = pkgs.rust-bin.nightly.latest.rustfmt;
      in {
        _module.args.pkgs = pkgsWithOverlays;

        formatter = pkgs.alejandra;

        legacyPackages = {inherit rust;};

        devShells.default = pkgs.mkShell {
          packages = [rustfmt] ++ builtins.attrValues {
            inherit (pkgs) cargo-nextest cargo-audit cargo-deny cargo-tarpaulin;
            inherit (pkgs) nil pre-commit reuse;
            inherit (pkgs) watchexec;
            inherit (pkgs) cmake pkg-config;
            inherit rust;
          };

          env.RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
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
