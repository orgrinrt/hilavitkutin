//! Variant: `rotate_left` intrinsic with runtime amount.
//!
//! On aarch64, lowers to `RORV` (rotate by register-amount) plus
//! the masking arithmetic to keep the amount in range. Variable
//! shifts are slightly more expensive than const shifts on most
//! aarch64 microarchitectures, but the gap is usually small (~1
//! cycle).
//!
//! Models hash mixers that derive their rotation amount from the
//! data (e.g., SipHash). Quantifies the cost of data-dependent
//! rotation vs const-amount rotation.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("rot_variable_amount", sizes = [256, 1024, 4096, 16384])]
fn run_rot_variable_amount<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                // amount derives from previous acc bits; the predictor cannot
                // resolve it, forcing a true register-amount RORV
                let k = ((acc >> 56) & 0x3F) as u32;
                acc = acc.rotate_left(k.max(1)) ^ v;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
