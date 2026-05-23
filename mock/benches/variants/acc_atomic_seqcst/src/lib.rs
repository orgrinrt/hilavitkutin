//! Variant: `AtomicU64` accumulator with SeqCst ordering.
//!
//! Each step emits a full memory barrier (`DMB ISH` on aarch64).
//! Worst case for atomic access cost in single-thread. Models a
//! WorkUnit that uses the strongest ordering "to be safe" without
//! considering the cost.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

use core::sync::atomic::{AtomicU64, Ordering};

#[bench_variant("acc_atomic_seqcst", sizes = [256, 1024, 4096, 16384])]
fn run_acc_atomic_seqcst<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let acc = AtomicU64::new(0xcbf29ce484222325);
            for &byte in input.iter() {
                let prev = acc.load(Ordering::SeqCst);
                let next = (prev ^ (byte as u64)).wrapping_mul(0x100000001b3);
                acc.store(next, Ordering::SeqCst);
            }
            let final_val = acc.load(Ordering::SeqCst);
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
