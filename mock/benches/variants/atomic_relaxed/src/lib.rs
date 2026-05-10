//! Variant: Relaxed atomic ops only.
//!
//! Each chunk: `fetch_add(Relaxed)` on a u64 counter, then mix the previous
//! value into an accumulator. Models a per-morsel progress counter increment
//! where no cross-thread happens-before is needed (the value is consumed only
//! at the next phase barrier, which uses its own Release-Acquire pair).

use core::sync::atomic::{AtomicU64, Ordering};
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("atomic_relaxed", sizes = [64, 256, 1024, 4096, 16384])]
fn run_atomic_relaxed<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let counter = AtomicU64::new(0);
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let p = input.as_ptr();
            for i in 0..chunks {
                let chunk = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                let prev = counter.fetch_add(chunk, Ordering::Relaxed);
                acc = (acc ^ prev).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
