//! Variant: `checked_add().unwrap_or(MAX)` reduction.
//!
//! Lowers to ADDS + B.CS branch + fallback on aarch64. The branch
//! is normally not taken (u64 sum of byte-times-prime products won't
//! overflow in any realistic morsel size), so the branch predictor
//! saturates to "not taken" within a few iterations.
//!
//! Models a strategy-marker-free defensive pattern: same semantics
//! as saturating but goes through a Maybe-shaped path. The interest
//! is whether LLVM optimises the branch out or pays its cost.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("sat_checked", sizes = [256, 1024, 4096, 16384])]
fn run_sat_checked<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0;
            for &byte in input.iter() {
                let term = (byte as u64).wrapping_mul(0x100000001b3);
                acc = match acc.checked_add(term) {
                    Some(v) => v,
                    None => u64::MAX,
                };
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
