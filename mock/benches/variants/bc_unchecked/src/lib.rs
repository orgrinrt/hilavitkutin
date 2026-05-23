//! Variant: `get_unchecked` explicit bounds-check elision.
//!
//! Unsafe explicit elision. Floor case: if any variant runs faster
//! than this one, that's a measurement artifact, not a real win.
//! Bench framework noise floor.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bc_unchecked", sizes = [256, 1024, 4096, 16384])]
fn run_bc_unchecked<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..N {
                let byte = unsafe { *input.get_unchecked(i) };
                acc = (acc ^ (byte as u64)).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
