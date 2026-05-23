//! Exponential moving-average batch update with cfg-gated SIMD.
//!
//! Topic 5 axes H and audit-2 M4. The EMA formula uses the standard
//! `ema_new = ema_old * 7/8 + measured * 1/8` shape carried via
//! `BlendFactor`-typed constants (`NORM_7_OVER_8`, `NORM_1_OVER_8`)
//! rooted on the arvo numeric foundation.
//!
//! Three implementations live behind `cfg(target_feature = ...)`
//! gates: NEON (aarch64), SSE2 (x86_64), and a scalar fallback. AVX2,
//! AVX-512, SVE expansion is bench-driven and lives in BACKLOG per
//! Topic 5 axis H. Per audit-2 m5: gating uses `target_feature`, not
//! bare `target_arch`. A consumer compiling for x86_64 without SSE2
//! (rare; pre-Pentium 4) falls back through the scalar path.
//!
//! `BlendFactor` is a domain alias rooted on
//! `arvo::UFixed<{IBits(0)}, {FBits(16)}, Hot>`. Per the arvo
//! design discipline (toolbox not policer; consumer owns the
//! domain name), the alias lives here, not in arvo. The arvo BACKLOG
//! catalogues a separate signed `Norm` (`I=1, F=14`) for the
//! eigenvector consumer; the two shapes coexist because the EMA
//! domain wants unsigned `[0, 1]` and eigenvector decomposition
//! wants signed `[-1, 1)`. Keeping the alias consumer-side preserves
//! arvo's "no semantic domain aliases at L0" stance.
//!
//! The intrinsics bodies in the NEON / SSE2 paths defer to the
//! scalar routine for now; the bench-validated 7-instruction NEON
//! kernel lives in
//! `mock/research/sketches/202605101036-ema-vectorisation/` (WORKS,
//! 7-instruction body). Pass 7 ports the sketch into this file's
//! NEON arm with a parity test against scalar; the structural shape
//! (cfg gates, signature, BlendFactor-typed constants) freezes here.

use arvo::{Hot, UFixed, fbits, ibits};

/// Per-axis EMA sample width. Topic 5 audit-2 + Pass 7 follow-up.
///
/// 64-bit unsigned integer fixed-point: large enough for nanosecond
/// timing samples (the typical AdaptAxis sample carrier), for record
/// counts up to 2^64, and for bytes-allocated metrics on host
/// platforms with 64-bit address spaces. `USize` was rejected at
/// post-megaround review: USize is platform-pointer-width and an EMA
/// sample is not pointer-shaped (a 32-bit consumer would silently
/// halve the sample width with no signal). `Sample` pins the width
/// explicitly.
pub type Sample = UFixed<{ ibits(64) }, { fbits(0) }, Hot>;

/// Sixteen-bit fractional blend factor in `[0, 1]`.
///
/// Hot strategy: lowers to `u16` storage at codegen. Sixteen
/// fractional bits give roughly five decimal digits of precision,
/// which is sufficient for the EMA formula's rounding-error budget
/// per Topic 5 audit-2 M4.
pub type BlendFactor = UFixed<{ ibits(0) }, { fbits(16) }, Hot>;

/// `BlendFactor` representation of `7 / 8`. Topic 5 audit-2 M4.
///
/// Raw bit pattern `0xE000` in sixteen fractional bits. Pairs with
/// `NORM_1_OVER_8` such that the two sum to exactly `1.0`
/// (`0x10000` in the same repr), preserving the EMA invariant.
pub const NORM_7_OVER_8: BlendFactor = BlendFactor::from_raw(0xE000);

/// `BlendFactor` representation of `1 / 8`. Topic 5 audit-2 M4.
///
/// Sums with `NORM_7_OVER_8` to exactly `1.0` (`0x10000` in the
/// 16-bit fractional repr).
pub const NORM_1_OVER_8: BlendFactor = BlendFactor::from_raw(0x2000);

/// EMA batch update over `Sample` slices.
///
/// `dst[i] = dst[i] * NORM_7_OVER_8 + src[i] * NORM_1_OVER_8`
/// applied across the slice. Lanes are independent; intermediate
/// multiplies use the Hot UFixed container; final adds saturate at
/// the upper bound.
///
/// **STUB**: body lands in Pass-7-or-later wiring of runtime
/// megaround `202605101036`. The current scalar / NEON / SSE2
/// arms all delegate to a no-op fallback. Consumers integrating
/// against this function today get the structural shape; the
/// bench-validated kernel arrives with the executor wiring.
///
/// Cfg-gated SIMD paths short-circuit when the right ISA extension
/// is available; the scalar fallback always typechecks and runs.
/// The aarch64-NEON kernel is bench-validated (7-instruction body
/// in the round's sketch); the x86_64-SSE2 kernel is structural
/// pending bench rounds in Pass 7.
#[inline]
pub fn ema_update(dst: &mut [Sample], src: &[Sample]) {
    debug_assert_eq!(
        dst.len(),
        src.len(),
        "ema_update: dst and src slices must have equal length"
    );

    #[cfg(target_feature = "neon")]
    {
        ema_update_neon(dst, src);
        return;
    }

    #[cfg(all(not(target_feature = "neon"), target_feature = "sse2"))]
    {
        ema_update_sse2(dst, src);
        return;
    }

    #[cfg(not(any(target_feature = "neon", target_feature = "sse2")))]
    {
        ema_update_scalar(dst, src);
    }
}

/// Scalar fallback. Pass 7 wires the bench-validated body against
/// the 7-instruction NEON sketch's behavioural parity. The shape
/// freezes here as a structural stub; the EMA call shape (USize
/// slices, no allocation, in-place mutation, sum-to-one
/// `NORM_7_OVER_8 + NORM_1_OVER_8` constants) is the contract Pass
/// 7 fills with the lane-wise mul-add body.
#[inline]
fn ema_update_scalar(_dst: &mut [Sample], _src: &[Sample]) {
    // Pass 7 wires the bench-validated body. Until then, the EMA
    // batch update is a no-op; the surrounding adapt subsystem
    // tolerates a flat metrics stream because `AdaptWu`'s anomaly
    // detection thresholds are tunable independently.
    let _ = NORM_7_OVER_8;
    let _ = NORM_1_OVER_8;
}

/// NEON-gated kernel. Bench-validated 7-instruction body lands at
/// Pass 7; for now this arm falls back to scalar so the cfg shape
/// freezes without a half-shipped intrinsics body.
#[cfg(target_feature = "neon")]
#[inline]
fn ema_update_neon(dst: &mut [Sample], src: &[Sample]) {
    ema_update_scalar(dst, src);
}

/// SSE2-gated kernel. Pass 7 ports the parity-checked intrinsics
/// body; falls back to scalar until then.
#[cfg(target_feature = "sse2")]
#[inline]
fn ema_update_sse2(dst: &mut [Sample], src: &[Sample]) {
    ema_update_scalar(dst, src);
}
