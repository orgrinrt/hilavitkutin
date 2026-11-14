//! Which workload a section runs, which arm a variant is, and the one call
//! every variant makes.

pub use bench_arm::{Arm, StateArm};

use crate::{accumulator, branching, element_wise, wide_parallel};

/// The four workloads, by the tag their routine writes into the input.
///
/// The input is the only channel the harness gives a variant, so the workload
/// travels in its tag: the driver resolves the section's routine, that routine
/// writes the tag, and the variant builds the workload's arm from it.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Workload {
    /// A four-stage RAW chain, one fiber.
    ElementWise,
    /// Two transforms of one input joined by a third.
    Branching,
    /// A transform feeding the accumulator append surface.
    Accumulator,
    /// Four independent heavy transforms, one trunk each.
    WideParallel,
}

impl Workload {
    /// Every workload, in the order the reporting tables list them.
    pub const ALL: [Workload; 4] = [
        Workload::ElementWise,
        Workload::Branching,
        Workload::Accumulator,
        Workload::WideParallel,
    ];

    /// The tag a routine writes for this workload.
    pub const fn tag(self) -> u32 {
        match self {
            Workload::ElementWise => 1,
            Workload::Branching => 2,
            Workload::Accumulator => 3,
            Workload::WideParallel => 4,
        }
    }

    /// The workload a tag names, if any.
    pub const fn from_tag(tag: u32) -> Option<Workload> {
        match tag {
            1 => Some(Workload::ElementWise),
            2 => Some(Workload::Branching),
            3 => Some(Workload::Accumulator),
            4 => Some(Workload::WideParallel),
            _ => None,
        }
    }

    /// The name a bench section and the ceilings use.
    pub const fn name(self) -> &'static str {
        match self {
            Workload::ElementWise => element_wise::NAME,
            Workload::Branching => branching::NAME,
            Workload::Accumulator => accumulator::NAME,
            Workload::WideParallel => wide_parallel::NAME,
        }
    }

    /// The workload a name names, if any.
    pub fn from_name(name: &str) -> Option<Workload> {
        Workload::ALL.into_iter().find(|w| w.name() == name)
    }

    /// The digest every arm of this workload must produce for `records`,
    /// computed from the kernels alone with no column materialised.
    pub fn reference(self, records: &[u32]) -> u64 {
        match self {
            Workload::ElementWise => element_wise::reference(records),
            Workload::Branching => branching::reference(records),
            Workload::Accumulator => accumulator::reference(records),
            Workload::WideParallel => wide_parallel::reference(records),
        }
    }

    /// Whether the workload has threaded arms. Only the workloads with
    /// several trunks or a record-split carrier do; `element_wise` is one
    /// fiber, and `branching` measured slower threaded than single core, so
    /// the design's bar for it is the single-core one.
    pub const fn has_parallel(self) -> bool {
        matches!(self, Workload::Accumulator | Workload::WideParallel)
    }

    /// Build one arm of this workload over `n` records.
    pub fn arm(self, kind: ArmKind, n: usize) -> Box<dyn Arm<u64>> {
        if matches!(kind, ArmKind::EnginePar | ArmKind::StdPar) && !self.has_parallel() {
            panic!(
                "engine_vs_std: `{}` has no threaded arm; the manifest lists one where the \
                 workload has none",
                self.name()
            );
        }
        match self {
            Workload::ElementWise => element_wise::arm(kind, n),
            Workload::Branching => branching::arm(kind, n),
            Workload::Accumulator => accumulator::arm(kind, n),
            Workload::WideParallel => wide_parallel::arm(kind, n),
        }
    }
}

/// Which arm a variant cdylib is.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArmKind {
    /// The engine on one core: `Scheduler::run`, or `run_fused` where the
    /// workload's units fold into one chain.
    Engine,
    /// The engine across cores: `Scheduler::run_parallel` on `BenchExecutor`.
    EnginePar,
    /// The optimal single-threaded std loop.
    Std,
    /// The optimal std loop threaded over the same core budget as
    /// `EnginePar`, on a `Crew` of workers parked between frames.
    StdPar,
    /// The engine's get-ready: a fresh scheduler built per call. The digest
    /// then runs it once, untimed, so the arm is validated like the others.
    EngineStartup,
    /// std's get-ready: the buffers its loop writes, allocated per call. The
    /// digest then runs the loop into them, untimed.
    StdStartup,
}

/// One call of the bench, for any arm of any workload: the whole body of every
/// variant.
///
/// The arm is built on the first call of the process and reused, keyed by its
/// kind, its workload and the record count (`bench_arm::call`).
pub fn bench_call<const N: usize>(
    kind: ArmKind,
    input: &bench_arm::TaggedRecords<N>,
    output: &mut u64,
) -> mockspace_bench_core::FfiBenchCall {
    let workload = Workload::from_tag(input.tag).unwrap_or_else(|| {
        panic!(
            "engine_vs_std: the input carries no workload tag ({})",
            input.tag
        )
    });
    bench_arm::call(
        (kind, workload, N),
        &input.records,
        || workload.arm(kind, N),
        output,
    )
}

/// A get-ready arm: `ready` makes the state a frame starts from and is the
/// timed region, `finish` runs one frame on it and digests the result.
///
/// The startup arms of every workload share this shape. The previous call's
/// state is dropped before the next is made, inside the timed region, as a
/// consumer rebuilding its pipeline would.
pub fn startup_arm<S: 'static>(
    mut ready: impl FnMut() -> S + 'static,
    mut finish: impl FnMut(&mut S, &[u32]) -> u64 + 'static,
) -> Box<dyn Arm<u64>> {
    Box::new(StateArm::new(
        (Vec::<u32>::new(), None::<S>),
        |s: &mut (Vec<u32>, Option<S>), records: &[u32]| {
            s.0.clear();
            s.0.extend_from_slice(records);
        },
        move |s: &mut (Vec<u32>, Option<S>)| {
            s.1 = None;
            s.1 = Some(ready());
        },
        move |s: &mut (Vec<u32>, Option<S>)| {
            let state = s.1.as_mut().expect("a frame ran before the digest");
            finish(state, &s.0)
        },
    ))
}

/// A std arm over plain buffers: the records copied into an input buffer of
/// its own in `load`, `run` the measured loop from it into the output buffers,
/// `digest` over those.
pub fn std_arm<B: 'static>(
    buffers: B,
    mut input: impl FnMut(&mut B) -> &mut Vec<u32> + 'static,
    run: impl FnMut(&mut B) + 'static,
    digest: impl FnMut(&mut B) -> u64 + 'static,
) -> Box<dyn Arm<u64>> {
    Box::new(StateArm::new(
        buffers,
        move |b: &mut B, records: &[u32]| {
            let v = input(b);
            v.clear();
            v.extend_from_slice(records);
        },
        run,
        digest,
    ))
}
