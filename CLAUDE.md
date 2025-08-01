# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

MiniGU is a Rust-based graph database and GQL (Graph Query Language) parser/execution engine designed for educational purposes. It consists of:

1. **GQL Parser** - Comprehensive parser for Graph Query Language with full AST support
2. **Graph Database Engine** - Memory-based graph storage with ACID transactions and WAL
3. **Vector Index Integration** - DiskANN-rs submodule for vector similarity search
4. **Interactive Shell** - CLI environment for query execution and development

## Build Commands

### Standard Development
```bash
# Build all workspace members
cargo build

# Build in release mode
cargo build --release

# Run interactive shell
cargo run -- shell
cargo run -r -- shell  # release mode

# Run tests
cargo test
cargo nextest run  # if cargo-nextest is installed

# Run specific test package
cargo test --package minigu-test
```

### Code Quality and Formatting
```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy --tests --features "std,serde,miette" --no-deps

# Check TOML formatting
taplo fmt --check --diff

# Run full CI pipeline locally
./scripts/run_ci.sh
```

### Documentation
```bash
# Build documentation
cargo doc --lib --no-deps --features "std,serde,miette"
```

## Architecture

### Workspace Structure
- **minigu/main**: Public API and core database engine
- **minigu-cli**: Interactive shell and command-line interface
- **minigu-test**: System-level integration tests using SQL Logic Test format
- **minigu/parser**: GQL parser with comprehensive AST and snapshot testing
- **minigu/binder**: Statement binding and semantic analysis
- **minigu/catalog**: Schema and metadata management with memory-based storage
- **minigu/storage**: Graph storage layer with vector index integration
- **minigu/execution**: Query execution engine with operator-based architecture
- **minigu/ir**: Intermediate representation for bound queries and logical plans
- **minigu/planner**: Query planning and optimization
- **minigu/context**: Session and transaction context management
- **DiskANN-rs**: Submodule providing vector similarity search capabilities

### Key Components

#### Parser Layer
- Logos-based lexer with comprehensive token support
- Recursive descent parser with precedence handling
- Full GQL AST with extensive snapshot testing (insta crate)
- Support for graph patterns, value expressions, and catalog operations

#### Storage Layer
- Memory-based graph storage with adjacency lists
- ACID transaction support with Write-Ahead Logging (WAL)
- Vector index integration via DiskANN-rs for similarity search
- Support for both OLTP (transaction processing) and OLAP (analytical processing)

#### Execution Engine
- Operator-based execution with iterators
- Support for graph pattern matching, filtering, projection, and aggregation
- Procedure call framework for extensible operations
- Built-in procedures for graph creation and data import/export

## Development Notes

### Rust Configuration
- Uses Rust 2024 edition with nightly toolchain (nightly-2025-01-17)
- Strict linting enabled via workspace-level clippy configuration
- Custom rustfmt configuration with 120-character line width
- Workspace-based dependency management for consistent versions

### Testing Strategy
- Comprehensive unit tests with snapshot testing via insta crate
- System-level tests using SQL Logic Test format in minigu-test
- Integration tests covering parser, execution, and storage layers
- CI pipeline with cross-platform testing (Ubuntu, macOS, Windows)

### Code Style
- Uses rustfmt with custom configuration (120 char lines, group imports)
- Clippy lints configured at workspace level with strict warnings
- TOML formatting enforced via taplo
- Spell checking via typos crate

### Key Dependencies
- **Parser**: logos (lexer), winnow (parsing utilities), miette (error reporting)
- **Storage**: rayon (parallelism), dashmap (concurrent maps), crossbeam-skiplist
- **Testing**: insta (snapshot testing), sqllogictest, libtest-mimic
- **CLI**: clap (argument parsing), rustyline (interactive shell)
- **Serialization**: serde, postcard, bincode for various formats

### Vector Index (DiskANN-rs)
- Rust port of Microsoft's DiskANN library
- Supports both in-memory and disk-based indices
- Integrated into storage layer for graph-vector hybrid operations
- Separate stable toolchain for compatibility with C bindings