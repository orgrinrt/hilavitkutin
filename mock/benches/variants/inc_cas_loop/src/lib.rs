//! Variant: load + compare_exchange_weak loop per increment.
//!
//! The "naive" atomic increment shape before LSE / fetch_add intrinsics.
//! In single-thread non-contended code the CAS succeeds first try, so
//! the extra cost is one redundant load + one branch per increment.
//!
//! Models what Topic 6 axis I would degrade to if the substrate used
//! a generic CAS-loop pattern (e.g. behind a trait) instead of the
//! direct fetch_add intrinsic.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

use core::sync::atomic::{AtomicU64, Ordering};

#[bench_variant("inc_cas_loop", sizes = [256, 1024, 4096, 16384])]
fn run_inc_cas_loop<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let counter = AtomicU64::new(0);
            for _ in input.iter() {
                let mut current = counter.load(Ordering::Relaxed);
                loop {
                    match counter.compare_exchange_weak(
                        current,
                        current + 1,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(actual) => current = actual,
                    }
                }
            }
            let final_val = counter.load(Ordering::Relaxed);
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
