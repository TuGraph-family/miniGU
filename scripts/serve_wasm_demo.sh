#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${1:-4173}"

cd "$ROOT_DIR"

cargo build -p minigu-wasm --target wasm32-unknown-unknown --release
rm -rf docs/wasm-demo/pkg
wasm-bindgen \
  --target web \
  --out-dir docs/wasm-demo/pkg \
  target/wasm32-unknown-unknown/release/minigu_wasm.wasm

cd docs/wasm-demo
python3 -m http.server "$PORT"
