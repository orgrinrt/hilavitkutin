//! Variant: hand-unrolled 8-byte word loop.
//!
//! Reads u64 from source, writes u64 to dest. Tail loop for the remainder.
//! Tests whether hand-rolled word-stride loop matches the memcpy intrinsic.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 16384;

#[bench_variant("memcpy_unrolled", sizes = [64, 256, 1024, 4096, 16384])]
fn run_memcpy_unrolled<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut buf = [0u8; MAX_N];
            let words = N / 8;
            let src = input.as_ptr();
            let dst = buf.as_mut_ptr();
            for i in 0..words {
                unsafe {
                    let w = (src.add(i * 8) as *const u64).read_unaligned();
                    (dst.add(i * 8) as *mut u64).write_unaligned(w);
                }
            }
            let tail_start = words * 8;
            for i in tail_start..N {
                unsafe {
                    *dst.add(i) = *src.add(i);
                }
            }
            let mut h: u64 = 0xcbf29ce484222325;
            for i in 0..N {
                h = (h ^ buf[i] as u64).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
