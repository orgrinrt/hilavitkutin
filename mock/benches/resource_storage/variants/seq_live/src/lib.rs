//! Axis D variant seq_live: fold a Seq collection member in place from its
//! column each pass (the V0 live-stream shape). Seed-driven Routine form: N is
//! the Seq element count, the payload is heap-allocated from the seed in untimed
//! setup, so payload size has no stack-array ceiling.

use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::seqd::{self, SeqAlgo};

#[bench_variant(SeqAlgo, "seq_live", sizes = [65536, 1048576, 4194304, 16777216, 67108864, 268435456])]
fn run_seq_live<const N: usize>(
    input: &<SeqAlgo<N> as Routine>::Input,
    output: &mut <SeqAlgo<N> as Routine>::Output,
) -> FfiBenchCall {
    let state = seqd::build(*input, N);
    let r = timed! { run { seqd::run_live(&state); } };
    *output = seqd::checksum(&state);
    r
}
