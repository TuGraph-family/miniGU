// Copyright (c) Microsoft Corporation. All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications:
// - Added cross-platform support for `prefetch_vector` with no-op fallback implementation for
//   non-x86_64 architectures to ensure compilation compatibility.

/// Prefetch the given vector in chunks of 64 bytes, which is a cache line size
/// NOTE: good efficiency when total_vec_size is integral multiple of 64
/// Prefetch is a performance optimization, no-op fallback on non-x86_64 architectures
/// doesn't affect functionality or correctness
#[cfg(target_arch = "x86_64")]
#[inline]
pub fn prefetch_vector<T>(vec: &[T]) {
    use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};

    let vec_ptr = vec.as_ptr() as *const i8;
    let vecsize = std::mem::size_of_val(vec);
    let max_prefetch_size = (vecsize / 64) * 64;

    for d in (0..max_prefetch_size).step_by(64) {
        unsafe {
            _mm_prefetch(vec_ptr.add(d), _MM_HINT_T0);
        }
    }
}

/// Prefetch fallback for non-x86_64 architectures
/// Prefetch is a performance optimization, no-op fallback doesn't affect functionality
#[cfg(not(target_arch = "x86_64"))]
#[inline]
pub fn prefetch_vector<T>(_vec: &[T]) {
    // No prefetch implementation for this architecture
    // Functionality remains correct, just without the prefetch optimization
}
