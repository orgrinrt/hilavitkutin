//! `hand_par`: the fused frame split into one record chunk per core, the fair
//! bar for `engine_par`.

use frame_wide_support::{ArmKind, FrameInput, FrameOut, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("hand_par", sizes = [4096, 65536, 1048576, 4194304, 16777216])]
fn run<const N: usize>(input: &FrameInput<N>, output: &mut FrameOut) -> FfiBenchCall {
    bench_call(ArmKind::HandPar, input, output)
}
