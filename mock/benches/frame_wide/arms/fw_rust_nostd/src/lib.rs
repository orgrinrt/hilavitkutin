//! `rust_nostd`: one index loop per stage over slices allocated before the
//! timed region, every intermediate materialised, no allocation in the frame.

use frame_wide_support::{ArmKind, FrameInput, FrameOut, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("rust_nostd", sizes = [4096, 65536, 1048576, 4194304, 16777216])]
fn run<const N: usize>(input: &FrameInput<N>, output: &mut FrameOut) -> FfiBenchCall {
    bench_call(ArmKind::RustNostd, input, output)
}
