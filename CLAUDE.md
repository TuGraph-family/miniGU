# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MiniGU is a graph database written in Rust, designed as an educational project by the TuGraph team. It provides an interactive shell environment for graph data operations and queries using GQL (Graph Query Language).

## Common Development Commands

### Build and Run
```bash
# Start the interactive shell (debug mode)
cargo run -- shell

# Start the interactive shell (release mode)
cargo run -r -- shell

# Build the project
cargo build

# Build with all features enabled
cargo build --features std,serde,miette
```

### Testing
```bash
# Run all tests (unit and integration)
cargo test

# Run tests with features enabled
cargo test --features std,serde,miette

# Run specific test
cargo test test_name

# Run doc tests only
cargo test --doc

# Run tests using nextest (faster test runner)
cargo nextest run
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Run clippy linter
RUSTFLAGS="-Dwarnings" cargo clippy --tests --no-deps

# Check TOML files formatting
taplo fmt --check --diff
```

### Documentation
```bash
# Build documentation
cargo doc

# Build documentation with features
cargo doc --lib --no-deps --features std,serde,miette
```

## Architecture Overview

MiniGU follows a modular architecture organized as a Rust workspace:

### Core Modules (`minigu/`)
- **core**: Main database library providing the public API, session management, and procedures
- **cli**: Command-line interface entry point (`minigu-cli`)
- **catalog**: Graph metadata management (vertex/edge types, properties, indexes)
- **context**: Query execution context and graph operations
- **transaction**: Transaction management with timestamp ordering
- **storage**: Low-level storage layer with vector index support via DiskANN

### GQL Layer (`minigu/gql/`)
- **parser**: GQL query parsing with comprehensive test suite using snapshot testing
- **planner**: Query planning and optimization
- **execution**: Query execution engine

### Storage Specialization (`minigu/storage/`)
- DiskANN-based vector indexing for similarity search
- Support for both L2 and cosine distance metrics
- Database files integrated into `.minigu` directory

### Testing Infrastructure
- **minigu-test**: End-to-end testing framework
- Extensive use of `insta` for snapshot testing
- SQL logic test support for query validation
- Property-based and stress testing for storage components

## Key Design Patterns

1. **Transaction Management**: Uses timestamp-based transaction ordering with global transaction ID generation
2. **Error Handling**: Comprehensive error types using `miette` for rich error reporting
3. **Type Safety**: Strong typing throughout with logical types defined in `minigu-common`
4. **Memory Management**: Uses `Arc` and `DashMap` for thread-safe shared state
5. **Storage Abstraction**: Clear separation between storage engine and higher-level components

## Development Notes

- The project uses Rust edition 2024 with nightly features (`impl_trait_in_assoc_type`, `duration_millis_float`)
- Clippy is configured with strict rules - all warnings are treated as errors in CI
- The project follows standard Rust conventions with comprehensive test coverage
- Vector indexes have limited dimension support (check constants in storage module)