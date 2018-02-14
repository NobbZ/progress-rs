#!/usr/bin/env zsh

set -xe

cargo test
cargo doc
cargo +nightly clippy
cargo +nightly fmt
