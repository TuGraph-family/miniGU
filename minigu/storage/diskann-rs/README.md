# diskann-rs for MiniGU

## Overview

This directory contains a vendored and modified version of Microsoft's DiskANN Rust implementation, integrated into the MiniGU project to provide high-performance approximate nearest neighbor (ANN) search capabilities.

## Provenance

- **Original Work**: [Microsoft DiskANN](https://github.com/microsoft/DiskANN/tree/main/rust)
- **This Version**: Vendored and modified for MiniGU

## Key Modifications for MiniGU

### 1. Memory-Based Interface
- **Files**: `diskann/src/index/inmem_index/ann_inmem_index.rs`, `inmem_index.rs`, `model/data_store/inmem_dataset.rs`
- Added `build_from_memory()` and `insert_from_memory()` methods for direct vector ingestion
- Added `get_aligned_vector_data()` for zero-copy vector access
- Enables building indices from in-memory data without file I/O

### 2. Filter Support
- **Files**: `diskann/src/common/filter_mask.rs` (new), `algorithm/search/search.rs`, `index/inmem_index/inmem_index.rs`
- Introduced `FilterIndex` trait for pre-filter and post-filter search
- Pre-filter: filters during graph traversal with alternative start points
- Post-filter: filters result candidates before returning top-K results
- Enhanced search signature to include `filter_mask` and `should_pre` parameters

### 3. Distance Return
- **Files**: `diskann/src/index/inmem_index/inmem_index.rs`, `ann_inmem_index.rs`
- Modified search methods to return distances alongside indices
- Enables distance-aware ranking and similarity scoring


### 4. Runtime SIMD Dispatch
- **Files**: `vector/src/l2_float_distance.rs`
- Implemented runtime AVX2 detection using `is_x86_feature_detected!("avx2")`
- Added scalar fallback implementations for f32 distance calculations
- Removed compile-time requirement for `-C target-feature=+avx2`
- Automatic selection of optimal implementation based on CPU capabilities

### 5. Cross-Platform Vector Prefetching Support
- **Files**: `vector/src/utils.rs`
- Added cross-platform `prefetch_vector` implementation with architecture-specific variants
- Provided no-op fallback for non-x86_64 architectures to ensure compilation compatibility
- Enables vector library to build successfully on all platforms while maintaining performance optimization benefits on x86_64

### 6. Cross-Platform Timer Simplification
- **Files**: `diskann/src/utils/timer.rs`
- Replaced platform-specific CPU cycle measurement with standard library Instant implementation
- Removed libc dependency and assembly instructions (_rdtsc) for cycle counting
- Simplified Timer structure by eliminating process handle and cycle tracking fields
- Maintained core timing functionality while ensuring cross-platform compatibility
- Eliminated platform-specific panics, enabling use on all supported architectures

## Modified Files Summary

| File | Modification Type | Description |
|------|------------------|-------------|
| `diskann/src/index/inmem_index/inmem_index.rs` | Enhanced | Memory interface, filter support, distances |
| `diskann/src/index/inmem_index/ann_inmem_index.rs` | Enhanced | Memory-based API, search signature |
| `diskann/src/model/data_store/inmem_dataset.rs` | Enhanced | build_from_memory, copy_aligned_data |
| `diskann/src/algorithm/search/search.rs` | Enhanced | Filter-aware search, get_init_ids |
| `diskann/src/common/filter_mask.rs` | New | FilterIndex trait (MiniGU original) |
| `vector/src/l2_float_distance.rs` | Enhanced | Runtime SIMD dispatch, scalar fallback |
| `vector/src/utils.rs` | Enhanced | Cross-platform prefetch_vector with no-op fallback |
| `diskann/src/utils/timer.rs` | Simplified for Cross-platform | Replaced platform-specific CPU cycle measurement with standard Instant implementation |

## Platform Performance Matrix

The following table outlines how diskann-rs performs across different platforms and architectures. All platforms maintain full functionality with varying performance characteristics.

| Architecture | Distance Calculation | Memory Prefetch | Performance Impact | Notes |
|------------------------|----------------------|-----------------|-------------------|-------|
| **x86_64 + AVX2** | AVX2 SIMD (vectorized) | `_mm_prefetch` optimized | ⭐⭐⭐⭐⭐ (Best) | Optimal performance with SIMD and prefetch |
| **x86_64 (no AVX2)** | Scalar fallback | `_mm_prefetch` optimized | ⭐⭐⭐⭐ (Good) | Runtime dispatch automatically chooses scalar |
| **Other architectures** | Scalar fallback | No-op (disabled) | ⭐⭐⭐ (Fair) | Fully functional, no SIMD optimization |

## Licensing

- **Original License**: MIT License by Microsoft Corporation
- **Modifications**: 2025 MiniGU Contributors
- All modified files retain original Microsoft copyright notices and add MiniGU copyright
- See `LICENSE` file for full MIT License text
- Each modified source file contains detailed modification notes in its header

