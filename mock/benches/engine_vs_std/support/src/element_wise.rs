//! Workload 1: a four-stage RAW chain over N records.
//!
//! `S1: A = stage1(In)`, `S2: B = stage2(A)`, `S3: C = stage3(B)`,
//! `S4: D = stage4(C)`. The engine registers four WorkUnits, each writing its
//! own column, and the engine arm runs them through `run_fused`, which folds
//! the four into one chain that keeps A, B and C in registers and writes only
//! D. The std arm is the same fused loop written by hand. This is the purest
//! test of within-fiber fusion. It is the original #660 workload; only its
//! input changed, from the record index to the harness seed.

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
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};

use crate::{
    Arm,
    ArmKind,
    Fnv1a,
    HeapBump,
    StateArm,
    arena_bytes,
    chain,
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

pub const NAME: &str = "element_wise";

// The host writes the seeded records into In before each frame; WorkUnits
// transform element-wise and never synthesise a value from the record's global
// position (each yields morsel-relative indices).
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct Inv(u32);
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
struct Av(u32);
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
struct Bv(u32);
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
struct Cv(u32);
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
struct Dv(u32);

type One<T> = Cons<Column<T>, Empty>;

#[derive(Copy, Clone)]
struct S1;
impl BuilderInput for S1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for S1 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        One<Av>,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Inv>;
    type Write = One<Av>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In was host-populated for the record count; read adds the
            // morsel start, so this is In[global i]. Av reserved + exclusively
            // written here; the morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(stage1(inp.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct S2;
impl BuilderInput for S2 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for S2 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Av>,
        One<Bv>,
        SnapNil,
        ColPtrCons<Av, ColPtrNil>,
        ColPtrCons<Bv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Av>;
    type Write = One<Bv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: S1 (ordered before by the RAW edge) wrote every record;
            // exclusive writer of Bv over reserved records.
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(stage2(a.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct S3;
impl BuilderInput for S3 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for S3 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Bv>,
        One<Cv>,
        SnapNil,
        ColPtrCons<Bv, ColPtrNil>,
        ColPtrCons<Cv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Bv>;
    type Write = One<Cv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let b = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(stage3(b.0))) };
        });
    }
}

#[derive(Copy, Clone)]
struct S4;
impl BuilderInput for S4 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for S4 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Cv>,
        One<Dv>,
        SnapNil,
        ColPtrCons<Cv, ColPtrNil>,
        ColPtrCons<Dv, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<Cv>;
    type Write = One<Dv>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let c = unsafe { ctx.reader().read::<Cv, _>(i) };
            unsafe { ctx.writer().write::<Dv, _>(i, Dv(stage4(c.0))) };
        });
    }
}

// The opt-in per-record maps: each stage's pure value-to-value transform,
// mirroring its `execute` body. Implementing `RecordOp` lets the engine fold the
// four-stage chain into one fused unit (`run_fused`) that keeps Av/Bv/Cv in
// registers and writes only Dv, the deep-single-fiber rust-pipe.
impl RecordOp for S1 {
    fn apply(&self, read: &ReadVals<'_, One<Inv>>) -> WriteVals<One<Av>> {
        rec![Av(stage1(read.get(Column::<Inv>::new()).0))]
    }
}
impl RecordOp for S2 {
    fn apply(&self, read: &ReadVals<'_, One<Av>>) -> WriteVals<One<Bv>> {
        rec![Bv(stage2(read.get(Column::<Av>::new()).0))]
    }
}
impl RecordOp for S3 {
    fn apply(&self, read: &ReadVals<'_, One<Bv>>) -> WriteVals<One<Cv>> {
        rec![Cv(stage3(read.get(Column::<Bv>::new()).0))]
    }
}
impl RecordOp for S4 {
    fn apply(&self, read: &ReadVals<'_, One<Cv>>) -> WriteVals<One<Dv>> {
        rec![Dv(stage4(read.get(Column::<Cv>::new()).0))]
    }
}

/// The scheduler of this workload over `n` records. A macro because the
/// scheduler's type is the output of the builder call sequence and is never
/// written down; the runtime arm and the startup arm expand the same sequence.
macro_rules! build {
    ($n:expr) => {{
        let n: usize = $n;
        Scheduler::builder()
            .with(Column::<Dv>::start())
            .with(Column::<Cv>::start())
            .with(Column::<Bv>::start())
            .with(Column::<Av>::start())
            .with(Column::<Inv>::start())
            .with(S1)
            .with(S2)
            .with(S3)
            .with(S4)
            .build(store(HeapBump::new(arena_bytes(5, n))), USize(n))
            .unwrap_or_else(|_| panic!("engine_vs_std: the {NAME} scheduler build failed at n={n}"))
    }};
}

/// Put the records into `In`, the bindings head (last registered).
macro_rules! load {
    ($s:expr, $records:expr, $n:expr) => {{
        let records: &[u32] = $records;
        assert_eq!(
            records.len(),
            $n,
            "engine_vs_std: the input carries a different record count"
        );
        // SAFETY: In was reserved for n records and no frame is running.
        unsafe { write_column($s.__bindings().__ptr().as_ptr() as *mut Inv, records, Inv) };
    }};
}

/// One frame: `In` marked dirty, so the incremental skip re-runs the chain
/// rather than skipping a clean frame, then the fused dispatch, which folds
/// the four units into one chain that keeps `Av`, `Bv` and `Cv` in registers.
macro_rules! frame {
    ($s:expr) => {{
        $s.mark_dirty::<Column<Inv>, _>();
        let failed = $s.run_fused().is_err();
        assert!(!failed, "engine_vs_std: the {NAME} frame failed");
    }};
}

/// Hash `Dv`, the deepest registered column (`In`, `Av`, `Bv`, `Cv`, `Dv`).
macro_rules! digest {
    ($s:expr, $n:expr) => {{
        let dv = $s
            .__bindings()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__ptr()
            .as_ptr();
        // SAFETY: Dv holds n records, written by the frame that just ran.
        unsafe { hash_column(dv, $n, |v| v.0) }
    }};
}

/// The fused std loop: `A`, `B` and `C` stay in registers and only `D` is
/// written. A zip, so the bounds checks go and the loop vectorises.
fn fused(input: &[u32], out: &mut [u32]) {
    for (d, &v) in out.iter_mut().zip(input.iter()) {
        *d = chain(v);
    }
}

/// The digest every arm must produce: FNV-1a over `D`.
pub fn reference(records: &[u32]) -> u64 {
    let mut h = Fnv1a::new();
    for &v in records {
        h.push(stage4(stage3(stage2(stage1(v)))));
    }
    h.finish()
}

/// One arm of this workload over `n` records. It has no threaded arms: one
/// fiber, nothing to spread.
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
