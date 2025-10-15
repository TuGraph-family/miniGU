// Copyright (c) Microsoft Corporation.  All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications:
// - Added scalar fallbacks for f32; unified with AVX2 via const-generic API.
// - Introduced runtime AVX2 dispatch using is_x86_feature_detected!("avx2").
// - Removed upstream compile-time requirement for -C target-feature=+avx2.
// - Added tests for scalar↔AVX2 consistency and dispatch behavior.
#![warn(missing_debug_implementations, missing_docs)]

//! Distance calculation for L2 Metric

// ==================== Scalar Implementation (Universal Fallback) ====================

/// Calculate L2 squared distance using scalar operations (fallback for non-AVX2)
#[inline]
pub(crate) fn distance_l2_scalar_f32<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..N {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum
}

// ==================== AVX2 Optimized Implementation (x86_64 only) ====================

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Calculate L2 squared distance using AVX2 vector arithmetic (f32)
///
/// # Safety
/// This function requires AVX2 support. Caller must ensure CPU has AVX2 capability.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
pub(crate) unsafe fn distance_l2_avx2_f32<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    debug_assert_eq!(N % 8, 0);

    // Make sure the addresses are 32-byte aligned
    debug_assert_eq!(a.as_ptr().align_offset(32), 0);
    debug_assert_eq!(b.as_ptr().align_offset(32), 0);

    let mut sum = _mm256_setzero_ps();

    // Iterate over the elements in steps of 8
    for i in (0..N).step_by(8) {
        let a_vec = _mm256_load_ps(&a[i]);
        let b_vec = _mm256_load_ps(&b[i]);
        let diff = _mm256_sub_ps(a_vec, b_vec);
        sum = _mm256_fmadd_ps(diff, diff, sum);
    }

    let x128: __m128 = _mm_add_ps(_mm256_extractf128_ps(sum, 1), _mm256_castps256_ps128(sum));
    // ( -, -, x1+x3+x5+x7, x0+x2+x4+x6 )
    let x64: __m128 = _mm_add_ps(x128, _mm_movehl_ps(x128, x128));
    // ( -, -, -, x0+x1+x2+x3+x4+x5+x6+x7 )
    let x32: __m128 = _mm_add_ss(x64, _mm_shuffle_ps(x64, x64, 0x55));
    // Conversion to float is a no-op on x86-64
    _mm_cvtss_f32(x32)
}

// ==================== Public Interface (Runtime Dispatch) ====================
/// Calculate L2 squared distance between two f32 vectors
///
/// This function automatically selects the best implementation based on CPU capabilities:
/// - x86_64 with AVX2: Uses optimized AVX2 SIMD instructions
/// - Other platforms or CPUs without AVX2: Uses scalar fallback
#[inline(never)]
pub fn distance_l2_vector_f32<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // Safety: We've checked that AVX2 is available
            unsafe { distance_l2_avx2_f32(a, b) }
        } else {
            distance_l2_scalar_f32(a, b)
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        distance_l2_scalar_f32(a, b)
    }
}

#[cfg(test)]
mod impl_tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[repr(C, align(32))]
    struct F32Slice104([f32; 104]);

    fn get_random_f32() -> (F32Slice104, F32Slice104) {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut a = F32Slice104([0.0; 104]);
        let mut b = F32Slice104([0.0; 104]);
        for i in 0..104 {
            a.0[i] = rng.random_range(-1.0..1.0);
            b.0[i] = rng.random_range(-1.0..1.0);
        }
        (a, b)
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn scalar_vs_avx2_consistency_f32() {
        if !is_x86_feature_detected!("avx2") {
            eprintln!("Skipping: CPU doesn't support AVX2");
            return;
        }

        for _ in 0..10 {
            let (a, b) = get_random_f32();
            let scalar = distance_l2_scalar_f32(&a.0, &b.0);
            let avx2 = unsafe { distance_l2_avx2_f32(&a.0, &b.0) };
            assert_abs_diff_eq!(scalar, avx2, epsilon = 1e-4);
        }
    }

    #[test]
    fn runtime_dispatch_selects_correct_impl() {
        let (a, b) = get_random_f32();
        let dispatched = distance_l2_vector_f32(&a.0, &b.0);
        let scalar = distance_l2_scalar_f32(&a.0, &b.0);

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                let avx2 = unsafe { distance_l2_avx2_f32(&a.0, &b.0) };
                assert_abs_diff_eq!(dispatched, avx2, epsilon = 1e-6);
            } else {
                assert_abs_diff_eq!(dispatched, scalar, epsilon = 1e-6);
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            assert_abs_diff_eq!(dispatched, scalar, epsilon = 1e-6);
        }
    }
}
