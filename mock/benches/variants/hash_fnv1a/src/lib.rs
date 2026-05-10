//! Variant: FNV-1a 64-bit hash.
//!
//! Classic byte-stream hash. One xor + one multiply per byte. Bit-stream
//! through the input, fold each byte into the accumulator.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[bench_variant("hash_fnv1a", sizes = [64, 256, 1024, 4096, 16384])]
fn run_hash_fnv1a<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut h = FNV_OFFSET;
            for b in input.iter() {
                h ^= *b as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
