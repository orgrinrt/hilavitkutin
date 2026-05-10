//! Variant: naive byte loop.
//!
//! Reads one byte, writes one byte. LLVM often pattern-matches this and
//! lowers to memcpy anyway, but on some optimisers it stays a byte loop.
//! Useful comparison for whether the pattern-match fires under our build.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 16384;

#[bench_variant("memcpy_naive", sizes = [64, 256, 1024, 4096, 16384])]
fn run_memcpy_naive<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut buf = [0u8; MAX_N];
            for i in 0..N {
                buf[i] = input[i];
            }
            let mut h: u64 = 0xcbf29ce484222325;
            for i in 0..N {
                h = (h ^ buf[i] as u64).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
