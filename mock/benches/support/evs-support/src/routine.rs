//! What the harness drives, and the per-call budget it needs.

use bench_arm::{MAX_CALL_US_FLOOR, TaggedRecords, tagged_bytes, tagged_records};

use crate::Workload;

/// The routine of one workload at `N` records: the workload's tag in the
/// input, the record column seeded by the harness, and the reference digest
/// every arm is held to. `W` is the workload's tag.
pub struct Evs<const N: usize, const W: u32>;

/// Nanoseconds of per-call budget per record.
///
/// The heaviest call is `wide_parallel` single threaded: four chains of eight
/// rounds of the four-stage kernel per record, about thirty nanoseconds a
/// record on one core. Ten times that leaves room for a slow machine. The
/// preflight probe skips a variant only past ten times the bound, so a
/// generous value costs nothing but a later kill on a hung arm.
const NS_PER_RECORD: u64 = 300;

impl<const N: usize, const W: u32> mockspace_bench_core::Routine for Evs<N, W> {
    type Input = TaggedRecords<N>;
    type Output = u64;

    /// The by-value constructor. The harness reaches `build_input_bytes`
    /// instead, and a column of millions of records would not fit on the stack
    /// here; `bench-arm` holds the two to one image.
    fn build_input(seed: u64) -> TaggedRecords<N> {
        tagged_records::<N>(W, seed)
    }

    fn build_input_bytes(seed: u64) -> Vec<u8> {
        tagged_bytes::<N>(W, seed)
    }

    /// Hold the arm to the reference: the digest the kernels alone produce
    /// for this input, recomputed by the harness from the seed.
    fn validate_output(input: &TaggedRecords<N>, output: &u64) -> Result<(), &'static str> {
        if input.tag != W {
            return Err("the input carries another workload's tag");
        }
        let Some(workload) = Workload::from_tag(W) else {
            return Err("the routine names no workload");
        };
        if *output == workload.reference(&input.records) {
            Ok(())
        } else {
            Err("the digest differs from the reference computed from the kernels alone")
        }
    }

    fn ops_per_call(_input: &TaggedRecords<N>) -> u64 {
        N as u64
    }

    fn max_call_us(n: usize) -> Option<u64> {
        Some(((n as u64).saturating_mul(NS_PER_RECORD) / 1000).max(MAX_CALL_US_FLOOR))
    }
}
