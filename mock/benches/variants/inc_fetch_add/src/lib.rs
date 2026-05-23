//! Variant: `fetch_add(1, Relaxed)` per increment.
//!
//! Single hardware atomic op (LDADD on aarch64 with FEAT_LSE; LL/SC
//! pair otherwise). The canonical phase-barrier counter increment
//! shape (Topic 6 axis I).
//!
//! Baseline for the atomic_inc_strategy bench.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

use core::sync::atomic::{AtomicU64, Ordering};

#[bench_variant("inc_fetch_add", sizes = [256, 1024, 4096, 16384])]
fn run_inc_fetch_add<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let counter = AtomicU64::new(0);
            for _ in input.iter() {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            let final_val = counter.load(Ordering::Relaxed);
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
