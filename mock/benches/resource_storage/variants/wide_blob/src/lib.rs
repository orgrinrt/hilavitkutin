//! Axis B variant wide_blob: gather M=64 resource members from one contiguous
//! blob, repeated; the timed region is the gather (not a morsel loop, which
//! would drown the locality difference). Contrast with wide_decomposed.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::bwide;

#[bench_variant("wide_blob", sizes = [256, 1024, 16384, 262144, 4194304])]
fn run_wide_blob<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let state = bwide::build_blob(input);
    let r = timed! { run { bwide::run_blob(&state); } };
    *output = bwide::checksum_blob(&state);
    r
}
