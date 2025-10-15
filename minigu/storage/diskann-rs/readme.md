# diskann-rs for MiniGU

## Overview

This directory contains a vendored and modified version of Microsoft's DiskANN Rust implementation, integrated into the MiniGU project to provide high-performance approximate nearest neighbor (ANN) search capabilities.

## Provenance

- **Original Work**: [Microsoft DiskANN](https://github.com/microsoft/DiskANN/tree/main/rust)
- **This Version**: Vendored and modified for MiniGU

## Key Modifications for MiniGU

### 1. Memory-Based Interface
- **Files**: `DiskANN/src/index/inmem_index/ann_inmem_index.rs`, `inmem_index.rs`, `model/data_store/inmem_dataset.rs`
- Added `build_from_memory()` and `insert_from_memory()` methods for direct vector ingestion
- Added `get_aligned_vector_data()` for zero-copy vector access
- Enables building indices from in-memory data without file I/O

### 2. Filter Support
- **Files**: `DiskANN/src/common/filter_mask.rs` (new), `algorithm/search/search.rs`, `index/inmem_index/inmem_index.rs`
- Introduced `FilterIndex` trait for pre-filter and post-filter search
- Pre-filter: filters during graph traversal with alternative start points
- Post-filter: filters result candidates before returning top-K results
- Enhanced search signature to include `filter_mask` and `should_pre` parameters

### 3. Distance Return
- **Files**: `DiskANN/src/index/inmem_index/inmem_index.rs`, `ann_inmem_index.rs`
- Modified search methods to return distances alongside indices
- Enables distance-aware ranking and similarity scoring


### 4. Runtime SIMD Dispatch
- **Files**: `vector/src/l2_float_distance.rs`
- Implemented runtime AVX2 detection using `is_x86_feature_detected!("avx2")`
- Added scalar fallback implementations for f32 distance calculations
- Removed compile-time requirement for `-C target-feature=+avx2`
- Automatic selection of optimal implementation based on CPU capabilities

### 5. Linux Platform Support
- **Files**: `DiskANN/src/platform/linux.rs` (adapted from Windows version)
- Created Linux-specific FileHandle implementation using standard library
- Adapted from Microsoft's `file_handle.rs` (Windows/winapi version)
- Simplified to Read-only mode for DiskANN index file access

## Modified Files Summary

| File | Modification Type | Description |
|------|------------------|-------------|
| `DiskANN/src/index/inmem_index/inmem_index.rs` | Enhanced | Memory interface, filter support, distances |
| `DiskANN/src/index/inmem_index/ann_inmem_index.rs` | Enhanced | Memory-based API, search signature |
| `DiskANN/src/model/data_store/inmem_dataset.rs` | Enhanced | build_from_memory, copy_aligned_data |
| `DiskANN/src/algorithm/search/search.rs` | Enhanced | Filter-aware search, get_init_ids |
| `DiskANN/src/common/filter_mask.rs` | New | FilterIndex trait (MiniGU original) |
| `vector/src/l2_float_distance.rs` | Enhanced | Runtime SIMD dispatch, scalar fallback |
| `DiskANN/src/platform/linux.rs` | Adapted | Linux FileHandle from Windows version |

## Licensing

- **Original License**: MIT License by Microsoft Corporation
- **Modifications**: 2025 MiniGU Contributors
- All modified files retain original Microsoft copyright notices and add MiniGU copyright
- See `LICENSE` file for full MIT License text
- Each modified source file contains detailed modification notes in its header

