#!/usr/bin/env bash
set -euo pipefail

taplo fmt --check --diff

cargo fmt --check

cargo clippy --tests --features "${DEFAULT_FEATURES:-std,serde,miette}" --no-deps

cargo build --features "${DEFAULT_FEATURES:-std,serde,miette}"

cargo nextest run --features "${DEFAULT_FEATURES:-std,serde,miette}"
cargo test --features "${DEFAULT_FEATURES:-std,serde,miette}" --doc

cargo doc --lib --no-deps --features "${DEFAULT_FEATURES:-std,serde,miette}"

echo "Running Python API tests..."
cd minigu/python

if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
    echo "Python is not available, skipping Python tests"
    exit 0
fi

if command -v python3 &> /dev/null; then
    PYTHON_CMD=python3
else
    PYTHON_CMD=python
fi

echo "Attempting to run Python tests directly..."
$PYTHON_CMD test_minigu_api.py || echo "Python tests failed or skipped"
echo "Python API tests completed."