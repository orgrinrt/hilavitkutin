//! Axis D variant seq_snapshot: copy a Seq collection member to a local buffer
//! once, then fold the copy each pass (the V1 snapshot-copy shape). Seed-driven
//! Routine form (see seq_live). As N exceeds cache the copy is pure overhead.

use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::seqd::{self, SeqAlgo};

#[bench_variant(SeqAlgo, "seq_snapshot", sizes = [65536, 1048576, 4194304, 16777216, 67108864, 268435456])]
fn run_seq_snapshot<const N: usize>(
    input: &<SeqAlgo<N> as Routine>::Input,
    output: &mut <SeqAlgo<N> as Routine>::Output,
) -> FfiBenchCall {
    let state = seqd::build(*input, N);
    let r = timed! { run { seqd::run_snapshot(&state); } };
    *output = seqd::checksum(&state);
    r
}
