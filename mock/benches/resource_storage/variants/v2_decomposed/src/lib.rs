//! Variant v2_decomposed: builds its storage layout as untimed setup (statements before
//! the `run` block), runs the morsel loop in the timed `run` block. See
//! `rsb_kernels::v2` for the layout + kernel; this cdylib is the thin
//! `#[bench_variant]` wrapper the harness loads. The payload size N (bytes) is
//! the column-record data; the resource is built from the seeded input head.
//! Output is the 8-byte FNV-1a checksum of the produced column, validated
//! byte-exact across all six variants by the harness.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::v2;

#[bench_variant("v2_decomposed", sizes = [256, 1024, 4096, 16384, 65536, 262144, 1048576])]
fn run_v2_decomposed<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    // Untimed setup: build the storage layout + columns from the seeded input.
    let state = v2::build(input);
    let r = timed! {
        run {
            // Timed: the morsel loop (member fetch + per-record combine + write).
            v2::run::<false>(&state);
        }
    };
    // Untimed teardown: checksum the produced column for cross-variant validation.
    *output = rsb_kernels::fnv1a_column(state.out as *const u32, state.n);
    r
}
