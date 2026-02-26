# miniGU WASM (Browser) Support

This repository includes an experimental browser-oriented WASM build that exposes a small JS API
for running miniGU queries against an in-memory database.

## What you get

- A WASM package crate: `minigu-wasm`
- JS-facing API (via `wasm-bindgen`):
  - `new MiniGuDb()`
  - `db.query_table(query: string): string`
  - `db.query_json(query: string): string`

Both `query_table` and `query_json` execute a **single statement** per call. If you need multiple
statements, call the API multiple times in order.

## Requirements

- Rust toolchain from `rust-toolchain.toml`
- `wasm32-unknown-unknown` target:
  - `rustup target add wasm32-unknown-unknown`

To generate JS glue code you can use either:

- `wasm-bindgen-cli` (`cargo install wasm-bindgen-cli`), or
- `wasm-pack` (`cargo install wasm-pack`)

## Build (minimal)

1) Build the WASM artifact:

```bash
cargo build -p minigu-wasm --target wasm32-unknown-unknown --release
```

2) Generate JS bindings (web target):

```bash
wasm-bindgen \
  --target web \
  --out-dir ./pkg \
  ./target/wasm32-unknown-unknown/release/minigu_wasm.wasm
```

This produces `./pkg/minigu_wasm.js` and `./pkg/minigu_wasm_bg.wasm`.

## Usage (Browser)

```js
import init, { MiniGuDb } from "./pkg/minigu_wasm.js";

await init();

const db = new MiniGuDb();

// Create a graph with sample data (catalog-modifying procedure)
db.query_json('CALL create_test_graph_data("g", 5)');

// Set current graph for subsequent queries
db.query_json("SESSION SET GRAPH g");

// Run a query
const out = db.query_json("MATCH (n:PERSON) RETURN n");
console.log(JSON.parse(out));
```

## Output format

### `query_table`

Returns a plain text table (intended for quick debugging).

### `query_json`

Returns a JSON string with the following shape:

```json
{
  "schema": [
    { "name": "...", "type": "...", "nullable": false }
  ],
  "rows": [
    [ ... ]
  ],
  "metrics_ms": {
    "parsing": 0.0,
    "planning": 0.0,
    "compiling": 0.0,
    "execution": 0.0,
    "total": 0.0
  }
}
```

`rows` is a row-major array. Complex Arrow values (e.g. `StructArray`) are converted into JSON
objects recursively.

## Limitations (important)

- **Persistence is not supported in WASM**
  - There is no on-disk database support in the browser build.
  - Storage modules that require filesystem access are not compiled for WASM.
- **`import_graph` / `export_graph` are not available in WASM**
  - These procedures are filesystem-based and are excluded from the WASM build.
- **Single-threaded runtime**
  - The WASM build uses a single-threaded runtime (no Rayon).
- **Timing metrics are best-effort**
  - `std::time::Instant` is not available on `wasm32-unknown-unknown`, so `metrics_ms` is reported as `0.0`.
- **Vector index implementation differs**
  - Native builds use DiskANN-backed vector indices.
  - WASM builds use a naive linear-scan implementation.

## CI / Verification

- Host tests:
  - `cargo test --workspace --features std,serde,miette`
- WASM compile check:
  - `cargo check -p minigu-wasm --target wasm32-unknown-unknown`
- WASM runtime tests:
  - Node:
    - `wasm-pack test --node minigu-wasm`
  - Browser (headless):
    - `wasm-pack test --headless --chrome minigu-wasm`

Note: browser tests require a local Chrome installation (for `--chrome`).
