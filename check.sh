#!/bin/sh
set -e # exit on error
set -x # show commands executed
cargo build --target wasm32-unknown-unknown --release --no-default-features
cargo build --target wasm32-unknown-unknown --release
cargo clippy --target wasm32-unknown-unknown --release --no-default-features -- -D warnings
cargo clippy --target wasm32-unknown-unknown --release -- -D warnings
cargo build --bin copy-and-link
for width in 500 400 300 200 150 130 110
do
  cargo +nightly fmt --all -- --config max_width=$width
done
cargo +nightly fmt --all
