//! Variant: SipHash-2-4 style ARX mixer (simplified one-pass).
//!
//! Add-Rotate-Xor pattern per block. Two compression rounds per block.
//! Not byte-exact SipHash (which has paired state + key schedule), but the
//! same per-byte work cost.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const SEED_A: u64 = 0xcbf29ce484222325;
const SEED_B: u64 = 0x9e3779b185ebca87;

#[inline(always)]
fn round(a: &mut u64, b: &mut u64) {
    *a = a.wrapping_add(*b);
    *b = b.rotate_left(13) ^ *a;
    *a = a.rotate_left(32);
}

#[bench_variant("hash_sip", sizes = [64, 256, 1024, 4096, 16384])]
fn run_hash_sip<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut a = SEED_A;
            let mut b = SEED_B;
            let blocks = N / 8;
            let p = input.as_ptr();
            for i in 0..blocks {
                let block = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                b ^= block;
                round(&mut a, &mut b);
                round(&mut a, &mut b);
                a ^= block;
            }
            // finaliser
            round(&mut a, &mut b);
            round(&mut a, &mut b);
            output.copy_from_slice(&(a ^ b).to_le_bytes());
        }
    }
}
