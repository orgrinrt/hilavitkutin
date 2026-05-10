//! Variant: xxh3-style block-mixed hash (simplified).
//!
//! Reads 8 bytes at a time, multiplies into the accumulator with two primes,
//! rotates between pairs. Block-parallel mixing pattern from xxhash family.
//! Not byte-exact xxhash3 (which has more state + finaliser), but the same
//! shape of work per byte.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const SEED: u64 = 0xcbf29ce484222325;
const PRIME_A: u64 = 0x9e3779b185ebca87;
const PRIME_B: u64 = 0xbf58476d1ce4e5b9;

#[bench_variant("hash_xxh", sizes = [64, 256, 1024, 4096, 16384])]
fn run_hash_xxh<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut h = SEED;
            let blocks = N / 8;
            let p = input.as_ptr();
            for i in 0..blocks {
                let block = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                let mixed = (block ^ PRIME_A).wrapping_mul(PRIME_B);
                h = h.rotate_left(31).wrapping_add(mixed).wrapping_mul(PRIME_A);
            }
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
