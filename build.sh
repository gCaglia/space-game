#!/bin/bash
set -euo pipefail

# Get ROOT
ROOT=$(dirname "$(realpath $0)")

# Compile native and wasm
cargo clean
cargo build --release
cargo build --release --target wasm32-unknown-unknown

# Link
ln -sf $ROOT/target/wasm32-unknown-unknown/release/space-game.wasm site/space-game.wasm