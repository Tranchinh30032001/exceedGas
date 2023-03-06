#!/bin/sh

echo ">> Building contract"

 rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C target-cpu=native -C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
