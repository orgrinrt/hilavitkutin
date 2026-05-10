//! Variant: `core::ptr::copy_nonoverlapping` (lowers to `memcpy` intrinsic).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 16384;

#[bench_variant("memcpy_intrinsic", sizes = [64, 256, 1024, 4096, 16384])]
fn run_memcpy_intrinsic<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut buf = [0u8; MAX_N];
            unsafe {
                core::ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), N);
            }
            // hash the copied bytes
            let mut h: u64 = 0xcbf29ce484222325;
            for i in 0..N {
                h = (h ^ buf[i] as u64).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
