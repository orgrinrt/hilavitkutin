//! Variant: explicit `ptr::read_unaligned::<u64>` loads.
//!
//! Forces the unaligned-load codegen path. On aarch64 the hardware
//! still permits unaligned loads efficiently, but LLVM may emit
//! defensive byte-shuffle sequences when alignment is statically
//! unknown.
//!
//! Contrasts with `load_aligned` to surface any codegen difference
//! when alignment is or is not asserted.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("load_unaligned", sizes = [256, 1024, 4096, 16384])]
fn run_load_unaligned<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let mut acc: u64 = 0xcbf29ce484222325;
            let base = input.as_ptr();
            for i in 0..words {
                let v = unsafe { core::ptr::read_unaligned(base.add(i * 8) as *const u64) };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
