//! Variant: non-atomic load + add + store via separate atomic ops.
//!
//! `counter.store(counter.load(Relaxed) + 1, Relaxed)` per step. Not
//! actually atomic across the read-modify-write window (a concurrent
//! writer could miss this increment), but in single-thread the result
//! is correct.
//!
//! Models what a "naive but Relaxed-everywhere" implementation would
//! produce. Floor for non-atomic-equivalent cost; the CAS-loop pays
//! one more branch per step but the same effective memory traffic.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

use core::sync::atomic::{AtomicU64, Ordering};

#[bench_variant("inc_load_store", sizes = [256, 1024, 4096, 16384])]
fn run_inc_load_store<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let counter = AtomicU64::new(0);
            for _ in input.iter() {
                let v = counter.load(Ordering::Relaxed);
                counter.store(v + 1, Ordering::Relaxed);
            }
            let final_val = counter.load(Ordering::Relaxed);
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
