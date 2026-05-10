//! Sketch — EMA vectorisation, cfg-gated SIMD per target_feature.
//!
//! Three paths:
//! - aarch64 + neon: 4-lane u32 NEON.
//! - x86_64 + sse2: 4-lane u32 SSE2.
//! - fallback: scalar loop.
//!
//! Each is `cfg(target_feature = "...")`-gated, not `cfg(target_arch)`,
//! per audit-2 m5. EMA shape: `new = (old * 7 + sample) / 8`,
//! equivalent to `Norm = UFixed<0, 32, S>` arithmetic with alpha = 1/8.

#![no_std]
#![allow(dead_code)]

const LANES: usize = 4;

/// Scalar implementation — always available, used when no target feature
/// matches.
#[inline(never)]
pub fn ema_update_batch_scalar(olds: &mut [u32; LANES], samples: &[u32; LANES]) {
    for i in 0..LANES {
        let o = olds[i] as u64;
        let s = samples[i] as u64;
        // (old * 7 + sample) / 8
        let new = ((o * 7) + s) >> 3;
        olds[i] = new as u32;
    }
}

// ---------- aarch64 NEON path ----------

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(never)]
pub fn ema_update_batch_simd(olds: &mut [u32; LANES], samples: &[u32; LANES]) {
    use core::arch::aarch64::*;
    unsafe {
        // Load four u32s into a v4u32.
        let o = vld1q_u32(olds.as_ptr());
        let s = vld1q_u32(samples.as_ptr());
        // Compute o * 7 lane-wise, then + s, then >> 3.
        // (no fused-mul-add for unsigned-int u32x4 lane-wise, so do
        // explicit shift-add: o*7 = (o<<3) - o.)
        let o8 = vshlq_n_u32(o, 3); // o << 3
        let o7 = vsubq_u32(o8, o); // o*8 - o = o*7
        let sum = vaddq_u32(o7, s); // + sample
        let new = vshrq_n_u32(sum, 3); // / 8
        vst1q_u32(olds.as_mut_ptr(), new);
    }
}

// ---------- x86_64 SSE2 path ----------

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[inline(never)]
pub fn ema_update_batch_simd(olds: &mut [u32; LANES], samples: &[u32; LANES]) {
    use core::arch::x86_64::*;
    unsafe {
        let o = _mm_loadu_si128(olds.as_ptr() as *const __m128i);
        let s = _mm_loadu_si128(samples.as_ptr() as *const __m128i);
        // o*7 = (o<<3) - o.
        let o8 = _mm_slli_epi32(o, 3);
        let o7 = _mm_sub_epi32(o8, o);
        let sum = _mm_add_epi32(o7, s);
        let new = _mm_srli_epi32(sum, 3);
        _mm_storeu_si128(olds.as_mut_ptr() as *mut __m128i, new);
    }
}

// ---------- scalar fallback re-export when neither feature is on ----------

#[cfg(not(any(
    all(target_arch = "aarch64", target_feature = "neon"),
    all(target_arch = "x86_64", target_feature = "sse2"),
)))]
#[inline(never)]
pub fn ema_update_batch_simd(olds: &mut [u32; LANES], samples: &[u32; LANES]) {
    ema_update_batch_scalar(olds, samples)
}

// ---------- dispatcher (always picks the cfg-active impl) ----------

#[inline(never)]
pub fn ema_update_batch(olds: &mut [u32; LANES], samples: &[u32; LANES]) {
    ema_update_batch_simd(olds, samples)
}
