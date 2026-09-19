#!/bin/bash
set -euo pipefail

mkdir dist/

cargo clean
cargo build --release
cargo build --release --target wasm32-unknown-unknown

cp target/release/space-game dist/
cp target/wasm32-unknown-unknown/release/space-game.wasm dist/
