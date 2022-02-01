#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "`dirname $0`"
cargo install cargo-witgen
cargo install --git https://github.com/willemneal/witx-bindgen --rev afaa3451aebb665e40c28031fe6176374bce6760 wit-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/status_message.wasm ./res/
#wasm-opt -Oz --output ./res/status_message.wasm ./res/status_message.wasm

cargo witgen generate --prefix-file ../../near-sdk/witgen.wit
wit-bindgen js-near -i ./witgen.wit --out-dir res/