//! What the harness drives, and the per-call budget it needs.
//!
//! This is a `Routine` rather than a `ByteRoutine` for one reason: `max_call_us`.
//! With no per-call bound the harness kills a worker at 300 seconds and the
//! cell yields nothing, and a frame over millions of records is measured in
//! milliseconds rather than nanoseconds.

use crate::*;

/// The bench routine for `frame_wide`, parameterised by record count and
/// regime.
///
/// `max_call_us` is what makes this a `Routine` rather than a `ByteRoutine`:
/// with no per-call bound the harness kills a worker at 300 seconds and the
/// cell yields nothing, and a frame over millions of records is measured in
/// milliseconds rather than nanoseconds.
pub struct FrameRoutine<const N: usize, const HEAVY: bool>;

/// The heavy sweep's routine.
pub type FrameHeavy<const N: usize> = FrameRoutine<N, true>;
/// The light sweep's routine.
pub type FrameLight<const N: usize> = FrameRoutine<N, false>;

/// Nanoseconds of per-call budget per record, heavy regime.
///
/// About ten times the slowest arm measured at 65536 records on the first
/// run (52 ns a record, `hand`, 2026-11-13, four-core Xeon at 2.1GHz). The
/// preflight probe skips a variant only past ten times the bound and the
/// worker deadline is three times it times the per-pass run count, so a
/// generous value costs nothing but a later kill on a genuinely hung arm.
const HEAVY_NS_PER_RECORD: u64 = 500;
/// The same, light regime: the slowest arm was 9.2 ns a record
/// (`rust_std_par`).
const LIGHT_NS_PER_RECORD: u64 = 100;

/// The least per-call bound any cell gets, 400 milliseconds.
///
/// Forced by the harness on x86_64, not by the frame. Every timing worker
/// calibrates its counter by sleeping three times one second before its first
/// call (`counter_frequency` in `mockspace-bench-core`), and the preflight
/// probe that validation runs is killed after ten times this bound, at least
/// two seconds. Below 400 milliseconds the probe is killed during the
/// calibration and every arm is skipped as slow, whatever it costs.
// FIXME(3ec38cb5c7c8): drop the floor once the harness calibrates its counter
// without a three second sleep per worker, or gives the probe room for it.
const MAX_CALL_US_FLOOR: u64 = 400_000;

impl<const N: usize, const HEAVY: bool> FrameRoutine<N, HEAVY> {
    /// The regime this routine builds inputs for.
    pub const fn regime() -> Regime {
        if HEAVY { Regime::Heavy } else { Regime::Light }
    }
}

/// Fill one record of the input column from the seed stream.
#[inline]
fn record_from(x: &mut u64) -> u32 {
    *x ^= *x >> 30;
    *x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    (*x >> 32) as u32
}

impl<const N: usize, const HEAVY: bool> mockspace_bench_core::Routine for FrameRoutine<N, HEAVY> {
    type Input = FrameInput<N>;
    type Output = FrameOut;

    /// The by-value constructor. The harness never calls it: the override of
    /// `build_input_bytes` below is its only caller on that path, and a frame
    /// over millions of records would not fit on the stack here.
    ///
    /// `the_two_input_builders_agree` holds it to the byte image, so the two
    /// cannot drift while only one of them is reached by a run.
    fn build_input(seed: u64) -> FrameInput<N> {
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mut out = FrameInput {
            regime:  Self::regime().tag(),
            pad:     0,
            records: [0u32; N],
        };
        for r in out.records.iter_mut() {
            *r = record_from(&mut x);
        }
        out
    }

    /// Fill the byte image directly so a multi-megabyte input never lives on
    /// the stack. `ByteRoutine::build_input_bytes` is the precedent.
    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let mut buf = vec![0u8; core::mem::size_of::<FrameInput<N>>()];
        buf[0 .. 4].copy_from_slice(&Self::regime().tag().to_le_bytes());
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        for chunk in buf[8 ..].chunks_mut(4) {
            chunk.copy_from_slice(&record_from(&mut x).to_le_bytes());
        }
        buf
    }

    /// Hold the arm to the reference: the digest `record` alone produces for
    /// this input. The harness rebuilds the input from the seed for this, so
    /// it is the arm's output against an independent recomputation.
    fn validate_output(input: &FrameInput<N>, output: &FrameOut) -> Result<(), &'static str> {
        if input.regime != Self::regime().tag() {
            return Err("the input carries the other sweep's regime tag");
        }
        let expected = if HEAVY {
            reference_digest::<Heavy>(&input.records)
        } else {
            reference_digest::<Light>(&input.records)
        };
        if *output == expected {
            Ok(())
        } else {
            Err("the digest differs from the reference frame computed by `record`")
        }
    }

    fn ops_per_call(_input: &FrameInput<N>) -> u64 {
        N as u64 * STAGES_PER_RECORD
    }

    fn max_call_us(n: usize) -> Option<u64> {
        let per_record = if HEAVY { HEAVY_NS_PER_RECORD } else { LIGHT_NS_PER_RECORD };
        Some(((n as u64).saturating_mul(per_record) / 1000).max(MAX_CALL_US_FLOOR))
    }
}
