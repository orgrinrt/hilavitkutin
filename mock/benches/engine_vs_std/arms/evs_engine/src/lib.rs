//! `engine`: the workload on the engine, one core.

use bench_arm::TaggedRecords;
use evs_support::{ArmKind, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("evs_engine", sizes = [4096, 65536, 1048576, 4194304])]
fn run<const N: usize>(input: &TaggedRecords<N>, output: &mut u64) -> FfiBenchCall {
    bench_call(ArmKind::Engine, input, output)
}
