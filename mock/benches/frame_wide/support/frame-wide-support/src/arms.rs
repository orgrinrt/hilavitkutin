//! The scaffolding each arm's cdylib hangs off.
//!
//! One arm is: get the records in, run the frame, read the digest out. What
//! differs between arms is only the middle step, so everything around it is
//! written once, in `bench-arm`, and an arm that wanted to differ in the other
//! two would have to say so loudly.
//!
//! Loading and the digest sit outside the timed region for every arm, so what
//! is measured is the frame and nothing else. Every arm copies the records
//! into an input buffer of its own in `load`, the engine into its input column
//! and the others into a `Vec`, so at the start of every frame each arm's input
//! has just been written and is equally hot. That holds of the input and of
//! nothing else: `hand` holds four buffers between frames, `rust_nostd`
//! thirteen, the engine thirteen columns in its arena, and `rust_std` frees and
//! reallocates its output vectors every frame.

pub use bench_arm::{Arm, StateArm};

use crate::*;

/// Which arm a variant cdylib is.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArmKind {
    /// `Scheduler::run`, single core.
    Engine,
    /// `Scheduler::run_parallel` on a `ThreadPoolApi` executor.
    EnginePar,
    /// The hand-written fused path, single threaded.
    Hand,
    /// The hand-written fused path over the same core budget.
    HandPar,
    /// The obvious loop over preallocated slices, core only.
    RustNostd,
    /// The idiomatic std shape.
    RustStd,
    /// The idiomatic std shape threaded with `std::thread::scope`.
    RustStdPar,
    /// The negative control: the hand path with one pipeline stage dropped.
    /// Never listed in the manifest; it exists to be rejected.
    DropStage,
    /// The engine negative control: the engine builder call sequence with one
    /// `Map` omitted. Never listed in the manifest; it exists to be rejected.
    EngineDropStage,
}

/// Build one arm for one regime and record count.
pub fn build_arm(kind: ArmKind, regime: Regime, n: usize) -> Box<dyn Arm<FrameOut>> {
    match kind {
        ArmKind::Engine => engine::build(regime, n, false),
        ArmKind::EnginePar => engine::build(regime, n, true),
        ArmKind::EngineDropStage => engine::build_without_a2(regime, n),
        _ => plain::build(kind, regime, n),
    }
}

/// One call of the bench, for any arm: the whole body of every variant.
///
/// The arm is built on the first call of the process and reused, keyed by its
/// kind, the regime the section's tag names and the record count, so a worker
/// asked for a second arm panics rather than dropping a scheduler whose worker
/// threads are parked on it (`bench_arm::call`).
pub fn bench_call<const N: usize>(
    kind: ArmKind,
    input: &FrameInput<N>,
    output: &mut FrameOut,
) -> mockspace_bench_core::FfiBenchCall {
    let regime = Regime::from_tag(input.tag);
    bench_arm::call(
        (kind, regime, N),
        &input.records,
        || build_arm(kind, regime, N),
        output,
    )
}

#[cfg(test)]
mod tests {
    use mockspace_bench_core::Routine;

    use super::*;

    /// How many records the round-trip runs over. Small, because
    /// `build_input` puts the whole column on the stack.
    const RT: usize = 4096;

    /// Decode the record column out of a byte image the way the harness does.
    fn records_from_bytes(bytes: &[u8]) -> Vec<u32> {
        bytes[8 ..]
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect()
    }

    /// The two input builders have to produce the same input.
    ///
    /// Only `build_input_bytes` is reached by a run, so a drift in
    /// `build_input` would not show anywhere: the doc comment asserting it is
    /// correct would be the only thing saying so. Both regimes, several seeds,
    /// and the tag as well as the records, since the tag is what tells a
    /// variant which sweep it is running under.
    #[test]
    fn the_two_input_builders_agree() {
        for seed in [0u64, 1, 7, 0x9E37_79B9_7F4A_7C15, u64::MAX] {
            let heavy = <FrameHeavy<RT> as Routine>::build_input(seed);
            let heavy_bytes = <FrameHeavy<RT> as Routine>::build_input_bytes(seed);
            assert_eq!(heavy.tag, TAG_HEAVY, "seed {seed}: heavy tag");
            assert_eq!(
                u32::from_le_bytes([
                    heavy_bytes[0],
                    heavy_bytes[1],
                    heavy_bytes[2],
                    heavy_bytes[3],
                ]),
                TAG_HEAVY,
                "seed {seed}: heavy byte-image tag"
            );
            assert_eq!(
                heavy.records.as_slice(),
                records_from_bytes(&heavy_bytes).as_slice(),
                "seed {seed}: heavy records"
            );

            let light = <FrameLight<RT> as Routine>::build_input(seed);
            let light_bytes = <FrameLight<RT> as Routine>::build_input_bytes(seed);
            assert_eq!(light.tag, TAG_LIGHT, "seed {seed}: light tag");
            assert_eq!(
                light.records.as_slice(),
                records_from_bytes(&light_bytes).as_slice(),
                "seed {seed}: light records"
            );
        }
    }

    /// The control for the one above: a stream that differs has to be caught.
    ///
    /// Without it, two builders that both returned a zero column would pass.
    #[test]
    fn two_seeds_do_not_produce_the_same_records() {
        let a = <FrameHeavy<RT> as Routine>::build_input(1);
        let b = <FrameHeavy<RT> as Routine>::build_input(2);
        assert_ne!(a.records.as_slice(), b.records.as_slice());
    }
}
