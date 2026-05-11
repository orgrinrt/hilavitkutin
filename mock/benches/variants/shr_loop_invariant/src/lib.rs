//! Variant: runtime-loaded but loop-invariant shift amount.
//!
//! The shift amount is loaded once from a `#[inline(never)]` getter,
//! then reused across all iterations. Lowers to `LSR xN, xN, xK`
//! (register-amount shift) but the register `K` is loaded once and
//! held in a register for the duration of the loop. No dep chain on
//! the shift amount.
//!
//! Isolates the cost of "amount-in-register" vs "amount-as-immediate"
//! without the dep-chain confound from rotate_strategy's variable
//! variant.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn opaque_shift_amount() -> u32 {
    let a = core::hint::black_box(13_u32);
    let b = core::hint::black_box(0_u32);
    a + b
}

#[bench_variant("shr_loop_invariant", sizes = [256, 1024, 4096, 16384])]
fn run_shr_loop_invariant<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let k = opaque_shift_amount();
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = acc ^ (v >> k);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
