//! Variant: AoS (array-of-structs) layout.
//!
//! Records are `Particle { pos: [u64; 3], vel: [u64; 3], mass: u64 }` =
//! 56 bytes each. Iteration updates `pos` using `mass` only (skips `vel`),
//! but reads all 56 bytes per record because the layout interleaves fields.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[repr(C)]
struct Particle {
    pos: [u64; 3],
    vel: [u64; 3],
    mass: u64,
}

#[bench_variant("layout_aos", sizes = [64, 256, 1024, 4096, 16384])]
fn run_layout_aos<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            // Each Particle is 56 bytes; pack as many as fit in N.
            let count = N / 56;
            let mut acc: u64 = 0xcbf29ce484222325;
            let p = input.as_ptr() as *const Particle;
            for i in 0..count {
                let particle = unsafe { p.add(i).read_unaligned() };
                // Update pos using mass; vel is read into the cache line but
                // never used. This wastes 24 bytes per record on the AoS path.
                let new_pos0 = particle.pos[0].wrapping_add(particle.mass);
                acc = (acc ^ new_pos0).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
