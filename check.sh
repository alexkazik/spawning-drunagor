#!/bin/sh
set -e # exit on error
set -x # show commands executed
cargo build --target wasm32-unknown-unknown --release --no-default-features
cargo build --target wasm32-unknown-unknown --release
cargo clippy --target wasm32-unknown-unknown --release
cargo +nightly clippy --target wasm32-unknown-unknown --release
cargo +nightly fmt --all
