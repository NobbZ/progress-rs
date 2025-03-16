{
  mkShell,
  cargo-nextest,
  cargo-audit,
  cargo-deny,
  cargo-tarpaulin,
  nil,
  pre-commit,
  reuse,
  watchexec,
  alejandra,
  rust-bin,
  rustVersion ? (builtins.fromTOML (builtins.readFile ./rust-toolchain)).toolchain.channel,
  rustfmt ? rust-bin.nightly.latest.rustfmt,
  rust_ ? rust-bin.stable.${rustVersion}.default.override {extensions = ["rust-src" "rust-analyzer"];},
}:
mkShell {
  packages = [
    # we use a nightly rustfmt, this has to be before rust!
    rustfmt

    # cargo tooling
    cargo-nextest
    cargo-audit
    cargo-deny
    cargo-tarpaulin

    # other tooling
    nil
    pre-commit
    reuse
    watchexec
    alejandra

    # actual rust
    rust_
  ];

  env.RUST_SRC_PATH = "${rust_}/lib/rustlib/src/rust/library";
}
