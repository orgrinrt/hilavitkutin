//! The arms, each a type, and the one entry every variant calls.
//!
//! An arm says how it runs a fold and an append over owned data at a worker
//! count. [`call`] resolves the workload, builds the arm's state once per
//! process through [`crate::cache`], and times one frame with
//! `timed_calibrated!`, which repeats the frame until the counter's floor is
//! cleared, so a short fiber is timed above the counter's resolution.

use core::marker::PhantomData;

use mockspace_bench_core::{FfiBenchCall, timed_calibrated};

use crate::cache::{self, Key};
use crate::engine::{self, Drive, Op};
use crate::kernel::Kernel;
use crate::split::{Ceil, HeadTailDyn, HeadTailMid};
use crate::workload::{WorkloadVisitor, visit};
use crate::{ConvInput, ConvOutput, data, std_par, walk};

/// One measured arm.
pub trait Arm: 'static {
    const NAME: &'static str;
    /// How many walkers the arm's fold runs at a worker count of `workers`.
    /// `tests/walkers.rs` holds it to the walkers a fold runs and the workers
    /// it parks.
    fn walkers(workers: usize) -> usize;
    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static;
    fn append<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static;
}

/// The engine's `run()`: one core, the whole fiber. Takes no worker count.
pub struct Sequential;

impl Arm for Sequential {
    const NAME: &'static str = "sequential";

    fn walkers(_workers: usize) -> usize {
        1
    }

    fn fold<K: Kernel>(data: Vec<u32>, _workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        engine::fold::<K>(&data, Drive::Seq)
    }

    fn append<K: Kernel>(
        data: Vec<u32>,
        _workers: usize,
    ) -> impl FnMut(Op) -> ConvOutput + 'static {
        engine::append::<K>(&data, Drive::Seq)
    }
}

/// The engine's `run_parallel`: the N-way ceil slice on `W` cores, the path
/// today.
pub struct Nway;

impl Arm for Nway {
    const NAME: &'static str = "nway";

    fn walkers(workers: usize) -> usize {
        workers
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        engine::fold::<K>(&data, Drive::Par(Self::walkers(workers)))
    }

    fn append<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        engine::append::<K>(&data, Drive::Par(workers))
    }
}

/// The append fallback the design gives head-and-tail on a non-commutative
/// fiber: one walker, forward, on the crew.
fn sequential_fallback<K: Kernel>(data: Vec<u32>) -> impl FnMut(Op) -> ConvOutput + 'static {
    walk::append::<K, Ceil>(data, Ceil, 1)
}

/// Head-and-tail with the meeting point fixed at `n / 2`, on the crew: two
/// walkers at `W >= 2`, one at `W = 1`. Falls back to one walker on the
/// append.
pub struct HeadTail;

impl Arm for HeadTail {
    const NAME: &'static str = "head_tail";

    fn walkers(workers: usize) -> usize {
        workers.min(2)
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::fold::<K, HeadTailMid>(data, HeadTailMid::new(), Self::walkers(workers))
    }

    fn append<K: Kernel>(
        data: Vec<u32>,
        _workers: usize,
    ) -> impl FnMut(Op) -> ConvOutput + 'static {
        sequential_fallback::<K>(data)
    }
}

/// Head-and-tail with a floating meeting point, on the crew: two walkers
/// claiming morsel blocks from both ends. Falls back to one walker on the
/// append.
pub struct HeadTailDynamic;

impl Arm for HeadTailDynamic {
    const NAME: &'static str = "head_tail_dyn";

    fn walkers(workers: usize) -> usize {
        workers.min(2)
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::fold::<K, HeadTailDyn>(data, HeadTailDyn::new(), Self::walkers(workers))
    }

    fn append<K: Kernel>(
        data: Vec<u32>,
        _workers: usize,
    ) -> impl FnMut(Op) -> ConvOutput + 'static {
        sequential_fallback::<K>(data)
    }
}

/// One walker on the crew, the whole fiber forward: the single-core anchor
/// on the machinery the head-and-tail arms run on. Takes no worker count.
pub struct KernelSeq;

impl Arm for KernelSeq {
    const NAME: &'static str = "kernel_seq";

    fn walkers(_workers: usize) -> usize {
        1
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::fold::<K, Ceil>(data, Ceil, Self::walkers(workers))
    }

    fn append<K: Kernel>(
        data: Vec<u32>,
        _workers: usize,
    ) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::append::<K, Ceil>(data, Ceil, 1)
    }
}

/// The N-way ceil slice on the crew: `nway`'s split on head-and-tail's
/// machinery.
pub struct KernelNway;

impl Arm for KernelNway {
    const NAME: &'static str = "kernel_nway";

    fn walkers(workers: usize) -> usize {
        workers
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::fold::<K, Ceil>(data, Ceil, Self::walkers(workers))
    }

    fn append<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        walk::append::<K, Ceil>(data, Ceil, workers)
    }
}

/// Scoped std threads over `ceil(n / W)` chunks, spawned per frame.
pub struct StdParallel;

impl Arm for StdParallel {
    const NAME: &'static str = "std_parallel";

    fn walkers(workers: usize) -> usize {
        workers
    }

    fn fold<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        std_par::fold::<K>(data, Self::walkers(workers))
    }

    fn append<K: Kernel>(data: Vec<u32>, workers: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
        std_par::append::<K>(data, workers)
    }
}

/// Build (once per process) and time one frame of arm `A` on `input` over
/// `n` records, writing the frame's result to `out`.
pub fn call<A: Arm>(input: &ConvInput, n: usize, out: &mut ConvOutput) -> FfiBenchCall {
    visit(input.kind, Entry::<A> {
        input: *input,
        n,
        out,
        _arm: PhantomData,
    })
}

/// Build arm `A`'s state for `input` and run one frame, untimed; the
/// correctness tests' entry.
pub fn run_once<A: Arm>(input: &ConvInput, n: usize) -> ConvOutput {
    let mut out = ConvOutput::default();
    let _ = call::<A>(input, n, &mut out);
    out
}

struct Entry<'a, A> {
    input: ConvInput,
    n:     usize,
    out:   &'a mut ConvOutput,
    _arm:  PhantomData<A>,
}

impl<A: Arm> Entry<'_, A> {
    fn key(&self) -> Key {
        Key {
            arm:     A::NAME,
            seed:    self.input.seed,
            kind:    self.input.kind,
            workers: self.input.workers,
            n:       self.n,
        }
    }

    fn measure<T: FnMut(Op) -> ConvOutput + 'static>(
        self,
        make: impl FnOnce() -> T,
    ) -> FfiBenchCall {
        let key = self.key();
        let out = self.out;
        cache::with(key, make, |frame| {
            let r = timed_calibrated! {
                run {
                    core::hint::black_box(frame(Op::Frame));
                }
            };
            *out = frame(Op::Read);
            r
        })
    }
}

impl<A: Arm> WorkloadVisitor for Entry<'_, A> {
    type Out = FfiBenchCall;

    fn fold<K: Kernel>(self) -> FfiBenchCall {
        let (seed, n, w) = (self.input.seed, self.n, self.input.workers as usize);
        self.measure(move || A::fold::<K>(data::column(seed, n), w))
    }

    fn append<K: Kernel>(self) -> FfiBenchCall {
        let (seed, n, w) = (self.input.seed, self.n, self.input.workers as usize);
        self.measure(move || A::append::<K>(data::column(seed, n), w))
    }
}
