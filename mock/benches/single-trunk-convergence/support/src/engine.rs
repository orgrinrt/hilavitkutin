//! The engine arms: one fiber of one unit, run by the scheduler.
//!
//! The fold is an accumulator unit: it reduces its record slice with
//! `ctx.reduce()` into a stack-local partial and appends the partial to an
//! `Accum`, so `run()` leaves one partial per morsel window and
//! `run_parallel` one per window per core, merged after the frame in core
//! sequence. The fold's result is the wrapping sum of the partials, the same
//! whatever the split. The append unit appends one transformed value per
//! record, so its output sequence is the result.
//!
//! Both units are accumulator writers, so the scheduler dispatches them
//! unit-outer, and `run_parallel` takes the N-way ceil slice of the record
//! range across its cores (`worker_accum_unit_outer`). This is the path the
//! bench calls `nway`.

use core::marker::PhantomData;
use core::pin::Pin;

use arvo::{Bool, USize};
use engine_vs_std::executor::BenchExecutor;
use engine_vs_std::{HeapBump, arena_bytes, store};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    HasAccumWriter,
    HasColumnReader,
    HasEach,
    HasReduce,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::ConvOutput;
use crate::kernel::{Kernel, fold_step};

/// One record value.
#[derive(Copy, Clone)]
#[repr(transparent)]
struct Val(u32);

/// One fold partial.
#[derive(Copy, Clone)]
#[repr(transparent)]
struct Part(u64);

/// One appended value.
#[derive(Copy, Clone)]
#[repr(transparent)]
struct Out(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW<T> = Cons<Accum<T>, Empty>;

/// The commutative fold over one column.
struct FoldUnit<K>(PhantomData<K>);

impl<K: Kernel> BuilderInput for FoldUnit<K> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<K: Kernel> WorkUnit<Always> for FoldUnit<K> {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Val>,
        AccW<Part>,
        SnapNil,
        ColPtrCons<Val, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Part, AccPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Val>;
    type Write = AccW<Part>;

    const COMMUTATIVE: Bool = Bool::TRUE;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // `EachApi` and `ReduceApi` both name `run` on the engine context;
        // only the one this unit uses is in scope.
        use hilavitkutin_api::context::ReduceApi as _;
        let partial = ctx.reduce().run(0u64, |acc, i| {
            // SAFETY: the column is host-filled for the record count; the
            // read adds the window start.
            let v = unsafe { ctx.reader().read::<Val, _>(i) };
            fold_step::<K>(acc, v.0)
        });
        // SAFETY: at most one partial per window, far under the reserved
        // capacity of one per record.
        unsafe { ctx.accums().append::<Part, _>(Part(partial)) };
    }
}

/// The order-sensitive append: one transformed value per record.
struct AppendUnit<K>(PhantomData<K>);

impl<K: Kernel> BuilderInput for AppendUnit<K> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<K: Kernel> WorkUnit<Always> for AppendUnit<K> {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Val>,
        AccW<Out>,
        SnapNil,
        ColPtrCons<Val, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Out, AccPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Val>;
    type Write = AccW<Out>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        use hilavitkutin_api::context::EachApi as _;
        ctx.each().run(|i| {
            // SAFETY: as in the fold; one append per record, within the
            // reserved capacity of one per record.
            let v = unsafe { ctx.reader().read::<Val, _>(i) };
            unsafe { ctx.accums().append::<Out, _>(Out(K::apply(v.0))) };
        });
    }
}

/// How the scheduler runs the frame.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Drive {
    /// `run()`: one core, the whole fiber.
    Seq,
    /// `run_parallel` on an executor reporting this many workers.
    Par(usize),
}

/// What a built arm is asked to do.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    /// Run one frame. Returns the zero output.
    Frame,
    /// Read the last frame's result.
    Read,
}

macro_rules! build_scheduler {
    ($unit:expr, $acc:ty, $data:expr) => {{
        let data: &[u32] = $data;
        let n = data.len();
        // One u32 column and one accumulator of up to 8 bytes per record.
        let provider = HeapBump::new(arena_bytes(3, n));
        let sched = Scheduler::builder()
            .with(Column::<Val>::new())
            .with(Accum::<$acc>::new())
            .with($unit)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| {
                panic!("the engine refused to build a one-unit fiber over {n} records")
            });
        // The accumulator is the head (last registered); the column is one
        // tail down. SAFETY: the column reserves `n` records of `Val`, the
        // scheduler is alive, each slot is written once.
        let base = sched.__bindings().__tail().__ptr().as_ptr() as *mut Val;
        for (i, &x) in data.iter().enumerate() {
            unsafe { *base.add(i) = Val(x) };
        }
        Box::pin(sched)
    }};
}

macro_rules! frame {
    ($sched:expr, $drive:expr, $pool:expr) => {
        match $drive {
            // SAFETY: `run` borrows the scheduler mutably and does not move
            // it; the pin is kept for `run_parallel`, which this arm never
            // calls.
            Drive::Seq => {
                let _ = unsafe { Pin::get_unchecked_mut($sched.as_mut()) }.run();
            },
            Drive::Par(_) => {
                let _ = $sched.as_mut().run_parallel(&$pool);
            },
        }
    };
}

fn executor(drive: Drive) -> BenchExecutor {
    match drive {
        Drive::Seq => BenchExecutor::with_workers(1),
        Drive::Par(w) => BenchExecutor::with_workers(w),
    }
}

/// The fold arm over `data`, driven as `drive`.
pub fn fold<K: Kernel>(data: &[u32], drive: Drive) -> impl FnMut(Op) -> ConvOutput + 'static {
    let n = data.len();
    let mut sched = build_scheduler!(FoldUnit::<K>(PhantomData), Part, data);
    let pool = executor(drive);
    move |op| {
        match op {
            Op::Frame => {
                frame!(sched, drive, pool);
                ConvOutput::default()
            },
            Op::Read => {
                let b = sched.as_ref().get_ref().__bindings();
                let len = b.__len_cell().get().0;
                let p = b.__ptr().as_ptr() as *const u64;
                // SAFETY: the frame wrote `len` partials at [0, len).
                let parts = unsafe { core::slice::from_raw_parts(p, len) };
                ConvOutput {
                    value: parts.iter().fold(0u64, |a, &x| a.wrapping_add(x)),
                    count: n as u64,
                }
            },
        }
    }
}

/// The append arm over `data`, driven as `drive`.
pub fn append<K: Kernel>(data: &[u32], drive: Drive) -> impl FnMut(Op) -> ConvOutput + 'static {
    let mut sched = build_scheduler!(AppendUnit::<K>(PhantomData), Out, data);
    let pool = executor(drive);
    move |op| {
        match op {
            Op::Frame => {
                frame!(sched, drive, pool);
                ConvOutput::default()
            },
            Op::Read => {
                let b = sched.as_ref().get_ref().__bindings();
                let len = b.__len_cell().get().0;
                let p = b.__ptr().as_ptr() as *const u32;
                // SAFETY: the frame appended `len` values at [0, len).
                let out = unsafe { core::slice::from_raw_parts(p, len) };
                ConvOutput {
                    value: crate::data::hash(out),
                    count: len as u64,
                }
            },
        }
    }
}
