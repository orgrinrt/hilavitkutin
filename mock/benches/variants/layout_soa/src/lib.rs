//! Variant: SoA (struct-of-arrays) layout — column-store style.
//!
//! Fields stored in parallel arrays: pos[], vel[], mass[]. Iteration updates
//! `pos` using `mass`; `vel` is never loaded into the cache. This is the
//! column-store layout arvo's Column<T> requires.
//!
//! Same logical update as layout_aos; same input bytes; same hash output
//! (byte-exact validation).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("layout_soa", sizes = [64, 256, 1024, 4096, 16384])]
fn run_layout_soa<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let count = N / 56;
            // Reinterpret the input buffer as three parallel arrays.
            // For fair comparison: same total bytes laid out as columns.
            let p = input.as_ptr();
            let pos_base = unsafe { p as *const u64 };
            let vel_base = unsafe { p.add(count * 24) as *const u64 };
            let mass_base = unsafe { p.add(count * 48) as *const u64 };
            // (vel_base is read into pointer-math but never dereferenced; only
            // pos and mass are loaded.)
            let _ = vel_base;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..count {
                let pos0 = unsafe { pos_base.add(i * 3).read_unaligned() };
                let mass = unsafe { mass_base.add(i).read_unaligned() };
                let new_pos0 = pos0.wrapping_add(mass);
                acc = (acc ^ new_pos0).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
