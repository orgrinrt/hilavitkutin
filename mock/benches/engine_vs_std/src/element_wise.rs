//! Workload 1: a four-stage RAW chain over N records.
//!
//! `S1: A = stage1(In)`, `S2: B = stage2(A)`, `S3: C = stage3(B)`,
//! `S4: D = stage4(C)`. The engine runs four WorkUnits, each writing its own
//! column, so all four intermediates (A, B, C, D) are materialised and walked
//! per morsel. The std arm is one fused loop that keeps A, B, C in registers
//! and writes only D. This is the purest test of within-fiber fusion: the gap
//! is dominated by intermediate-column memory traffic the fused loop never
//! pays. It is the original #660 workload, unchanged.

use core::hint::black_box;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::RecordOp;

use crate::{
    HeapBump, WorkloadMeasure, arena_bytes, bench, fnv1a_u32_slice, stage1, stage2, stage3, stage4,
    store,
};

pub const NAME: &str = "element_wise";

// The host pre-populates In[i] = i (the global record index); WorkUnits
// transform element-wise and never synthesise a value from the record's global
// position (each yields morsel-relative indices).
#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
struct Cv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by S4, read back post-run as raw u32 for the hash
struct Dv(u32);

type One<T> = Cons<Column<T>, Empty>;

#[derive(Copy, Clone)]
struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Av>, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S2 {
    type Read = One<Av>;
    type Write = One<Bv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Av>, One<Bv>, SnapNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S3 {
    type Read = One<Bv>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Bv>, One<Cv>, SnapNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S4 {
    type Read = One<Cv>;
    type Write = One<Dv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Cv>, One<Dv>, SnapNil, ColPtrCons<Cv, ColPtrNil>, ColPtrCons<Dv, ColPtrNil>>;
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
    type In = Inv;
    type Out = Av;
    fn apply(&self, x: Inv) -> Av {
        Av(stage1(x.0))
    }
}
impl RecordOp for S2 {
    type In = Av;
    type Out = Bv;
    fn apply(&self, x: Av) -> Bv {
        Bv(stage2(x.0))
    }
}
impl RecordOp for S3 {
    type In = Bv;
    type Out = Cv;
    fn apply(&self, x: Bv) -> Cv {
        Cv(stage3(x.0))
    }
}
impl RecordOp for S4 {
    type In = Cv;
    type Out = Dv;
    fn apply(&self, x: Cv) -> Dv {
        Dv(stage4(x.0))
    }
}

pub fn measure(n: usize, warmup: usize, iters: usize) -> WorkloadMeasure {
    // ----- startup: build a fresh scheduler each iteration -----
    let eng_startup = bench(warmup, iters, || {
        let provider = HeapBump::new(arena_bytes(5, n));
        let scheduler = Scheduler::builder()
            .with(Column::<Dv>::new())
            .with(Column::<Cv>::new())
            .with(Column::<Bv>::new())
            .with(Column::<Av>::new())
            .with(Column::<Inv>::new())
            .with(S1)
            .with(S2)
            .with(S3)
            .with(S4)
            .build(store(provider), USize(n))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        black_box(&scheduler);
    });
    let std_startup = bench(warmup, iters, || {
        // std get-ready: the input + output buffers the fused loop reads/writes
        // (the A/B/C intermediates live in registers).
        let in_buf: Vec<u32> = vec![0u32; n];
        let d: Vec<u32> = vec![0u32; n];
        black_box(&in_buf);
        black_box(&d);
    });

    // ----- runtime engine: build once, run many -----
    let provider = HeapBump::new(arena_bytes(5, n));
    let mut sched = Scheduler::builder()
        .with(Column::<Dv>::new())
        .with(Column::<Cv>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(S2)
        .with(S3)
        .with(S4)
        .build(store(provider), USize(n))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Host-populate In[i] = i. In is the bindings head (last-registered).
    // SAFETY: In's buffer was reserved for N records of Inv (repr u32); the
    // scheduler (hence the arena) is alive; each reserved slot is written once.
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..n {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    let eng_runtime = bench(warmup, iters, || {
        // Mark the chain's root input dirty each iteration so the incremental
        // skip (domain 16) re-runs the fused chain every frame: the bench
        // measures real recompute work, not a clean-frame skip. A consumer that
        // mutates In between frames makes exactly this call; here the input is
        // unchanged but the bench must still time the full chain.
        sched.mark_dirty::<Column<Inv>, _>();
        // Fused dispatch: the engine folds [S1, S2, S3, S4] into one ChainWu and
        // walks it, keeping Av/Bv/Cv register-resident and writing only Dv.
        let r = sched.run_fused();
        black_box(&r);
    });
    sched.mark_dirty::<Column<Inv>, _>();
    let _ = sched.run_fused();
    // Dv is the deepest registered column (In -> Av -> Bv -> Cv -> Dv).
    let dv_base = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr();
    let eng_hash = {
        // SAFETY: Dv holds N reserved records; the scheduler (hence storage) is
        // alive.
        let slice = unsafe { core::slice::from_raw_parts(dv_base as *const u32, n) };
        fnv1a_u32_slice(slice)
    };

    // ----- runtime std: alloc + fill input once, fused loop many -----
    let in_buf: Vec<u32> = (0..n as u32).collect();
    let mut d_out: Vec<u32> = vec![0u32; n];
    let std_runtime = bench(warmup, iters, || {
        // Zip so bounds checks elide and LLVM autovectorises: the optimal fused
        // single pass.
        for (d, &inv) in d_out.iter_mut().zip(in_buf.iter()) {
            let a = stage1(inv);
            let b = stage2(a);
            let c = stage3(b);
            *d = stage4(c);
        }
        black_box(&d_out);
    });
    let std_hash = fnv1a_u32_slice(&d_out);

    WorkloadMeasure {
        name: NAME,
        n,
        eng_startup,
        std_startup,
        eng_runtime,
        std_runtime,
        eng_runtime_par: None,
        std_runtime_par: None,
        checksum_ok: eng_hash == std_hash,
        eng_hash,
        std_hash,
    }
}
