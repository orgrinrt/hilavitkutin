//! Variant: per-iteration shift amount loaded from input.
//!
//! Each iteration's shift amount derives from THIS iteration's input
//! byte (no dep on previous step). Variable per iteration, but no
//! latency chain on the shift amount itself.
//!
//! Isolates the cost of "amount is independent per iteration" from
//! the dep-chain case in rotate_strategy's variable variant.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("shr_per_iter", sizes = [256, 1024, 4096, 16384])]
fn run_shr_per_iter<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                // shift amount comes from this iteration's v (NOT prev acc),
                // so each iteration is independent on the amount dimension
                let k = ((v >> 56) & 0x3F) as u32;
                acc = acc ^ (v >> k.max(1));
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
