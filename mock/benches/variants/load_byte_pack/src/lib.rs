//! Variant: byte-by-byte read + `u64::from_le_bytes` pack.
//!
//! Reads eight u8s and packs them via `u64::from_le_bytes`. Modern
//! LLVM recognises this pattern and reduces it back to a single u64
//! load on aarch64. The bench verifies whether the recognition holds
//! under realistic workload context and inlining noise.
//!
//! Floor case: if LLVM fails to recognise the pattern, this variant
//! degrades to eight scalar byte loads.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("load_byte_pack", sizes = [256, 1024, 4096, 16384])]
fn run_load_byte_pack<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let off = i * 8;
                let v = u64::from_le_bytes([
                    input[off + 0],
                    input[off + 1],
                    input[off + 2],
                    input[off + 3],
                    input[off + 4],
                    input[off + 5],
                    input[off + 6],
                    input[off + 7],
                ]);
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
