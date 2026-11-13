//! The scaffolding each arm's cdylib hangs off.
//!
//! One arm is: get the records in, run the frame, read the digest out. What
//! differs between arms is only the middle step, so everything around it is
//! written once here and an arm that wanted to differ in the other two would
//! have to say so loudly.

use core::cell::RefCell;
use core::mem::ManuallyDrop;

use crate::*;

/// One arm of the bench: get the records in, run the frame, read the digest.
///
/// Loading and the digest sit outside the timed region for every arm, so what
/// is measured is the frame and nothing else. Every arm copies the records
/// into an input buffer of its own in `load`, the engine into its input
/// column and the others into a `Vec`, so at the start of every frame each
/// arm's input has just been written and is equally hot.
///
/// That holds of the input and of nothing else. The arms keep different
/// working sets between frames and the frame starts on whatever the last one
/// left: `hand` holds four buffers, `rust_nostd` thirteen, the engine thirteen
/// columns in its arena, and `rust_std` frees and reallocates its output
/// vectors every frame. The input is the one part of the cache state that is
/// the same across arms.
pub trait FrameArm {
    /// Put the frame's input records in place. Untimed.
    fn load(&mut self, records: &[u32]);
    /// Run one frame. This is the measured region.
    fn frame(&mut self);
    /// Hash the three output columns. Untimed.
    fn digest(&mut self) -> FrameOut;
}

/// A `FrameArm` built from a state value and three closures over it.
///
/// Every arm is some state plus three operations on it, and one arm's state is
/// a `Scheduler` whose type cannot be written down: it is the output of a
/// builder call sequence over thirteen columns and twelve units. Holding it in a
/// generic field and erasing the whole thing behind `Box<dyn FrameArm>` is
/// what lets the engine arms and the plain arms share one shape.
pub struct StateArm<S, L, F, D> {
    state:  S,
    load:   L,
    frame:  F,
    digest: D,
}

impl<S, L, F, D> StateArm<S, L, F, D>
where
    L: FnMut(&mut S, &[u32]),
    F: FnMut(&mut S),
    D: FnMut(&mut S) -> FrameOut,
{
    /// Assemble an arm. The bounds are here rather than on the impl so the
    /// closures' parameter types are inferred at the call site.
    pub fn new(state: S, load: L, frame: F, digest: D) -> Self {
        Self {
            state,
            load,
            frame,
            digest,
        }
    }
}

impl<S, L, F, D> FrameArm for StateArm<S, L, F, D>
where
    L: FnMut(&mut S, &[u32]),
    F: FnMut(&mut S),
    D: FnMut(&mut S) -> FrameOut,
{
    fn load(&mut self, records: &[u32]) {
        (self.load)(&mut self.state, records);
    }

    fn frame(&mut self) {
        (self.frame)(&mut self.state);
    }

    fn digest(&mut self) -> FrameOut {
        (self.digest)(&mut self.state)
    }
}

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
pub fn build_arm(kind: ArmKind, regime: Regime, n: usize) -> Box<dyn FrameArm> {
    match kind {
        ArmKind::Engine => engine::build(regime, n, false),
        ArmKind::EnginePar => engine::build(regime, n, true),
        ArmKind::EngineDropStage => engine::build_without_a2(regime, n),
        _ => plain::build(kind, regime, n),
    }
}

thread_local! {
    /// The per-process arm, built on the first call and reused.
    ///
    /// The framework sanctions this: each worker is a dedicated subprocess per
    /// variant, mode and pass, and at least one warmup call always runs before
    /// any timed batch, so a built scheduler lands in untimed territory. It is
    /// a `thread_local` rather than a `static` because a `Scheduler` is
    /// neither `Sync` nor `Unpin`, and the entry point is only ever called on
    /// the worker's own thread.
    ///
    /// `ManuallyDrop` keeps the arm alive until the process exits. A parallel
    /// engine arm has worker threads parked on its scheduler, and a
    /// thread-local destructor running at exit would free the scheduler under
    /// them. The process reclaims the memory either way.
    static ARM: ManuallyDrop<RefCell<Option<(ArmKind, Regime, usize, Box<dyn FrameArm>)>>> =
        const { ManuallyDrop::new(RefCell::new(None)) };
}

/// One call of the bench, for any arm: the whole body of every variant.
///
/// The arm is built on the first call of the process and reused. The records
/// are loaded and the digest is read outside the timed region; the `run` block
/// holds the frame and nothing else.
///
/// The key is fixed for the life of a worker process: one dylib is one
/// `ArmKind`, the worker is spawned with one `--n`, and the regime comes from
/// the bench section that spawned it. A key that changed would mean dropping a
/// scheduler whose worker threads are still parked on it, so a change is a
/// panic rather than a rebuild.
pub fn bench_call<const N: usize>(
    kind: ArmKind,
    input: &FrameInput<N>,
    output: &mut FrameOut,
) -> mockspace_bench_core::FfiBenchCall {
    let regime = Regime::from_tag(input.regime);
    ARM.with(|cell| {
        let mut slot = cell.borrow_mut();
        match slot.as_ref() {
            None => {
                *slot = Some((kind, regime, N, build_arm(kind, regime, N)));
            },
            Some((k, r, m, _)) => {
                assert!(
                    *k == kind && *r == regime && *m == N,
                    "frame_wide: this process already holds a {k:?}/{r:?}/n={m} arm and was \
                     asked for {kind:?}/{regime:?}/n={N}. One worker process runs one arm at \
                     one size under one regime."
                );
            },
        }
        let arm = &mut slot.as_mut().expect("the arm was just placed").3;
        let call = mockspace_bench_core::timed! {
            setup { arm.load(&input.records); }
            run { arm.frame(); }
        };
        *output = arm.digest();
        call
    })
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
            assert_eq!(heavy.regime, TAG_HEAVY, "seed {seed}: heavy tag");
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
            assert_eq!(light.regime, TAG_LIGHT, "seed {seed}: light tag");
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
