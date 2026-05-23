//! Variant: `AtomicU64` accumulator with Relaxed ordering.
//!
//! Each step does load + mul + store via the atomic. Even Relaxed
//! atomic ops on aarch64 emit a different instruction shape than
//! plain memory ops: LDR/STR exclusive or just LDR/STR with the
//! Rust runtime guarantee that the access is uninterrupted.
//!
//! Models a WorkUnit metric or counter stored as `AtomicU64` instead
//! of a plain local. Relevant to Topic 3 M3 (per-fiber inline
//! metrics) cost decisions.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

use core::sync::atomic::{AtomicU64, Ordering};

#[bench_variant("acc_atomic_relaxed", sizes = [256, 1024, 4096, 16384])]
fn run_acc_atomic_relaxed<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let acc = AtomicU64::new(0xcbf29ce484222325);
            for &byte in input.iter() {
                let prev = acc.load(Ordering::Relaxed);
                let next = (prev ^ (byte as u64)).wrapping_mul(0x100000001b3);
                acc.store(next, Ordering::Relaxed);
            }
            let final_val = acc.load(Ordering::Relaxed);
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
