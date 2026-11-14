//! Workload 2: a branching multi-fiber DAG.
//!
//! Two independent transforms read the same input and write store-disjoint
//! columns, then a third joins them:
//!   `X: Xv = stage1(In)`
//!   `Y: Yv = stage3(stage2(In))`   (a different transform, so the branches
//!                                    are genuinely distinct work)
//!   `Z: Zv = stage4(Xv ^ Yv)`      (the join, reads both branches)
//!
//! X and Y touch disjoint stores, so block-diagonalisation places them in
//! distinct fibers; the join depends on both via RAW edges. The engine
//! materialises Xv, Yv, and Zv and dispatches across the fibers. The std arm
//! fuses the whole DAG per element, keeping x and y in registers and writing
//! only z. This exercises dispatch across fibers (mega-dispatch territory) on
//! top of the intermediate-materialisation cost.

use core::hint::black_box;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::{
    Arm,
    ArmKind,
    Fnv1a,
    HeapBump,
    StateArm,
    arena_bytes,
    fnv1a_u32,
    hash_column,
    stage1,
    stage2,
    stage3,
    stage4,
    startup_arm,
    std_arm,
    store,
    write_column,
};

pub const NAME: &str = "branching";

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct Xv(u32);
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct Yv(u32);
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
#[allow(dead_code)] // written by Z, read back post-run as raw u32 for the hash
struct Zv(u32);

type One<T> = Cons<Column<T>, Empty>;
type Two<A, B> = Cons<Column<A>, Cons<Column<B>, Empty>>;

#[inline(always)]
fn branch_x(seed: u32) -> u32 {
    stage1(seed)
}
#[inline(always)]
fn branch_y(seed: u32) -> u32 {
    stage3(stage2(seed))
}
#[inline(always)]
fn join(x: u32, y: u32) -> u32 {
    stage4(x ^ y)
}

// Branch X: reads In, writes Xv.
struct BranchX;
impl BuilderInput for BranchX {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for BranchX {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        One<Xv>,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Xv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Inv>;
    type Write = One<Xv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for the record count; Xv reserved and
            // exclusively written here; the morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Xv, _>(i, Xv(branch_x(inp.0))) };
        });
    }
}

// Branch Y: reads In, writes Yv. Store-disjoint from X.
struct BranchY;
impl BuilderInput for BranchY {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for BranchY {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        One<Yv>,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Yv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Inv>;
    type Write = One<Yv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as BranchX, for Yv.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Yv, _>(i, Yv(branch_y(inp.0))) };
        });
    }
}

// Join Z: reads Xv and Yv, writes Zv. Ordered after both branches by the RAW
// edges the plan derives from the read set.
struct JoinZ;
impl BuilderInput for JoinZ {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for JoinZ {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Two<Xv, Yv>,
        One<Zv>,
        SnapNil,
        ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Two<Xv, Yv>;
    type Write = One<Zv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both branches (ordered before this unit by the RAW edges)
            // wrote every record the morsel covers; Zv reserved + exclusive here.
            let x = unsafe { ctx.reader().read::<Xv, _>(i) };
            let y = unsafe { ctx.reader().read::<Yv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(join(x.0, y.0))) };
        });
    }
}

/// The scheduler of this workload over `n` records; a macro for the reason
/// `element_wise` gives.
macro_rules! build {
    ($n:expr) => {{
        let n: usize = $n;
        Scheduler::builder()
            .with(Column::<Inv>::start())
            .with(Column::<Xv>::start())
            .with(Column::<Yv>::start())
            .with(Column::<Zv>::start())
            .with(BranchX)
            .with(BranchY)
            .with(JoinZ)
            .build(store(HeapBump::new(arena_bytes(4, n))), USize(n))
            .unwrap_or_else(|_| panic!("engine_vs_std: the {NAME} scheduler build failed at n={n}"))
    }};
}

/// Put the records into `In`, three tails below the head (`Zv`, `Yv`, `Xv`,
/// `In`).
macro_rules! load {
    ($s:expr, $records:expr, $n:expr) => {{
        let records: &[u32] = $records;
        assert_eq!(
            records.len(),
            $n,
            "engine_vs_std: the input carries a different record count"
        );
        let base = $s.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut Inv;
        // SAFETY: In was reserved for n records and no frame is running.
        unsafe { write_column(base, records, Inv) };
    }};
}

/// One frame on one core: `In` marked dirty so the incremental skip re-runs
/// the carrier, then `run`.
///
/// Single core is this workload's engine arm. `run_parallel` was measured here
/// (R4d, finding 202606072130) and was slower than single core at every size
/// (30.75x, 4.12x, 2.75x against 1.54x, 2.21x, 2.69x): the main-orchestrated
/// per-phase barrier dominates small frames, and trunk parallelism does not
/// remove the materialised diamond. Its parity is a later gate (per-fiber
/// diamond fusion and a worker-side barrier), so the bar stays single core.
macro_rules! frame {
    ($s:expr) => {{
        $s.mark_dirty::<Column<Inv>, _>();
        let failed = $s.run().is_err();
        assert!(!failed, "engine_vs_std: the {NAME} frame failed");
    }};
}

/// Hash `Zv`, the bindings head.
macro_rules! digest {
    ($s:expr, $n:expr) => {{
        // SAFETY: Zv holds n records, written by the frame that just ran.
        unsafe { hash_column($s.__bindings().__ptr().as_ptr(), $n) }
    }};
}

/// The fused std loop: the whole DAG per record, `x` and `y` in registers and
/// only `z` written.
fn fused(input: &[u32], out: &mut [u32]) {
    for (z, &v) in out.iter_mut().zip(input.iter()) {
        *z = join(branch_x(v), branch_y(v));
    }
}

/// The digest every arm must produce: FNV-1a over `Z`.
pub fn reference(records: &[u32]) -> u64 {
    let mut h = Fnv1a::new();
    for &v in records {
        h.push(stage4(stage1(v) ^ stage3(stage2(v))));
    }
    h.finish()
}

/// One arm of this workload over `n` records. No threaded arms, for the
/// reason `frame!` gives.
pub fn arm(kind: ArmKind, n: usize) -> Box<dyn Arm<u64>> {
    match kind {
        ArmKind::Engine => {
            Box::new(StateArm::new(
                (build!(n), n),
                |s, records| load!(s.0, records, s.1),
                |s| frame!(s.0),
                |s| digest!(s.0, s.1),
            ))
        },
        ArmKind::EngineStartup => {
            startup_arm(
                move || build!(n),
                move |s, records| {
                    load!(s, records, n);
                    frame!(s);
                    digest!(s, n)
                },
            )
        },
        ArmKind::Std => {
            std_arm(
                (Vec::new(), vec![0u32; n]),
                |b| &mut b.0,
                |b| {
                    fused(&b.0, &mut b.1);
                    black_box(&b.1);
                },
                |b| fnv1a_u32(&b.1),
            )
        },
        ArmKind::StdStartup => {
            startup_arm(
                move || (vec![0u32; n], vec![0u32; n]),
                |b, records| {
                    b.0.copy_from_slice(records);
                    fused(&b.0, &mut b.1);
                    fnv1a_u32(&b.1)
                },
            )
        },
        ArmKind::EnginePar | ArmKind::StdPar => unreachable!("refused by `Workload::arm`"),
    }
}
