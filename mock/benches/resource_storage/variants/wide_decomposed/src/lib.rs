//! Axis B variant wide_decomposed: gather M=64 resource members from scattered
//! per-member columns (one cache line each), repeated; the timed region is the
//! gather. Same members + order as wide_blob, so checksums agree; only the
//! fetch locality differs (M scattered lines vs one contiguous blob).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::bwide;

#[bench_variant("wide_decomposed", sizes = [256, 1024, 16384, 262144, 4194304])]
fn run_wide_decomposed<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let state = bwide::build_dec(input);
    let r = timed! { run { bwide::run_dec(&state); } };
    *output = bwide::checksum_dec(&state);
    r
}
