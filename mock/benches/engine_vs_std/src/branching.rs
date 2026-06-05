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
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};

use crate::{
    HeapBump, WorkloadMeasure, arena_bytes, bench, fnv1a_u32_slice, stage1, stage2, stage3, stage4,
    store,
};

pub const NAME: &str = "branching";

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Xv(u32);
#[derive(Copy, Clone)]
struct Yv(u32);
#[derive(Copy, Clone)]
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchX {
    type Read = One<Inv>;
    type Write = One<Xv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Xv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Xv, ColPtrNil>>;
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchY {
    type Read = One<Inv>;
    type Write = One<Yv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Yv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Yv, ColPtrNil>>;
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for JoinZ {
    type Read = Two<Xv, Yv>;
    type Write = One<Zv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        Two<Xv, Yv>,
        One<Zv>,
        PtrNil,
        ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
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

pub fn measure(n: usize, warmup: usize, iters: usize) -> WorkloadMeasure {
    let eng_startup = bench(warmup, iters, || {
        let provider = HeapBump::new(arena_bytes(4, n));
        let scheduler = Scheduler::builder()
            .with(Column::<Inv>::new())
            .with(Column::<Xv>::new())
            .with(Column::<Yv>::new())
            .with(Column::<Zv>::new())
            .with(BranchX)
            .with(BranchY)
            .with(JoinZ)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        black_box(&scheduler);
    });
    let std_startup = bench(warmup, iters, || {
        let in_buf: Vec<u32> = vec![0u32; n];
        let z: Vec<u32> = vec![0u32; n];
        black_box(&in_buf);
        black_box(&z);
    });

    // ----- runtime engine -----
    let provider = HeapBump::new(arena_bytes(4, n));
    let mut sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Xv>::new())
        .with(Column::<Yv>::new())
        .with(Column::<Zv>::new())
        .with(BranchX)
        .with(BranchY)
        .with(JoinZ)
        .build(store(provider), USize(n))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Zv is the head (last-registered), In is three tails down (Zv -> Yv -> Xv
    // -> In).
    // SAFETY: In's buffer was reserved for N records of Inv (repr u32); the
    // scheduler is alive; each reserved slot is written once.
    let in_base = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut Inv;
    for i in 0..n {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    let eng_runtime = bench(warmup, iters, || {
        let r = sched.run();
        black_box(&r);
    });
    let _ = sched.run();
    let zv_base = sched.__bindings().__ptr().as_ptr();
    let eng_hash = {
        // SAFETY: Zv holds N reserved records; the scheduler (hence storage) is
        // alive.
        let slice = unsafe { core::slice::from_raw_parts(zv_base as *const u32, n) };
        fnv1a_u32_slice(slice)
    };

    // ----- runtime std: fuse the whole DAG per element, write only z -----
    let in_buf: Vec<u32> = (0..n as u32).collect();
    let mut z_out: Vec<u32> = vec![0u32; n];
    let std_runtime = bench(warmup, iters, || {
        for (z, &inv) in z_out.iter_mut().zip(in_buf.iter()) {
            let x = branch_x(inv);
            let y = branch_y(inv);
            *z = join(x, y);
        }
        black_box(&z_out);
    });
    let std_hash = fnv1a_u32_slice(&z_out);

    WorkloadMeasure {
        name: NAME,
        n,
        eng_startup,
        std_startup,
        eng_runtime,
        std_runtime,
        checksum_ok: eng_hash == std_hash,
        eng_hash,
        std_hash,
    }
}
