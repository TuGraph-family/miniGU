#!/usr/bin/env bash
set -euo pipefail

# TOML 格式检查
taplo fmt --check --diff

# 代码格式检查
cargo fmt --check

# Clippy 静态检查
cargo clippy --tests --features "${DEFAULT_FEATURES:-std,serde,miette}" --no-deps

# 构建
cargo build --features "${DEFAULT_FEATURES:-std,serde,miette}"

# 测试
cargo nextest run --features "${DEFAULT_FEATURES:-std,serde,miette}"
cargo test --features "${DEFAULT_FEATURES:-std,serde,miette}" --doc

# 文档构建
cargo doc --lib --no-deps --features "${DEFAULT_FEATURES:-std,serde,miette}"