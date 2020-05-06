#!/bin/sh

# script provided for convenience, to build and extract wasm output to root

#RUSTFLAGS='-C link-arg=-s' \
cargo build --bin bitswing --target=wasm32-unknown-unknown --release
mkdir -p output

mv target/wasm32-unknown-unknown/release/bitswing.wasm output/bitswing.wasm
# wasm-snip output/bitswing.wasm -o output/bitswing.wasm --snip-rust-fmt-code --snip-rust-panicking-code
# twiggy top -n 20 output/bitswing.wasm
# twiggy top -n 300 bitswing.wasm > twiggy-snip.txt

cd bridge-mock
cargo build --bin bridge-mock --target=wasm32-unknown-unknown --release
cd ..
mv bridge-mock/target/wasm32-unknown-unknown/release/bridge-mock.wasm output/bridge-mock.wasm
