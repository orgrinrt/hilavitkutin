//! Variant: Acquire-load + Release-store atomic ops.
//!
//! Each chunk: load(Acquire) + fetch_add(Release). Models the per-morsel
//! progress counter for a cross-thread reader (peer worker watching progress
//! during head+tail convergence). On aarch64 emits `ldar` + `stlr` pair.

use core::sync::atomic::{AtomicU64, Ordering};
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("atomic_acquire", sizes = [64, 256, 1024, 4096, 16384])]
fn run_atomic_acquire<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let counter = AtomicU64::new(0);
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let p = input.as_ptr();
            for i in 0..chunks {
                let chunk = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                let observed = counter.load(Ordering::Acquire);
                let prev = counter.fetch_add(chunk, Ordering::Release);
                acc = (acc ^ prev ^ observed).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
