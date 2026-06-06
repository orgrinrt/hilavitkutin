//! ASM-dispatch-gate fixtures (D6).
//!
//! Four `#[no_mangle] #[inline(never)]` exports, each building a real
//! `Scheduler` and dispatching one representative GATE-1 shape. The gate binary
//! (`benches::bin::asm_gate`) builds this crate in release, objdumps each named
//! symbol, and runs the five-check disassembly checklist. `#[inline(never)]`
//! keeps the symbol stable for objdump; the dispatch body below it inlines, so
//! the disassembly is the real emitted dispatch, not a wrapper.
//!
//! The WorkUnit and `RecordOp` definitions mirror `engine_vs_std::element_wise`
//! and `::accumulator` (the perf-gate workloads) so the gate reads the same
//! dispatch the perf gate measures. Provider, stage, and hash helpers are reused
//! from `engine_vs_std` rather than duplicated.

use core::hint::black_box;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, PtrNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::RecordOp;

use engine_vs_std::{
    HeapBump, arena_bytes, chain, fnv1a_u32_slice, stage1, stage2, stage3, stage4, store,
};

// The fixed record count for every fixture. Small enough that the release build
// is quick, large enough that the morsel loop is a real loop.
const N: usize = 256;

// ----- the four-stage column chain (mirrors engine_vs_std::element_wise) -----

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
        EngineCtx<'frame, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for the record count; Av reserved and
            // exclusively written here over the reserved records.
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
        EngineCtx<'frame, One<Av>, One<Bv>, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
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
        EngineCtx<'frame, One<Bv>, One<Cv>, PtrNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
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
        EngineCtx<'frame, One<Cv>, One<Dv>, PtrNil, ColPtrCons<Cv, ColPtrNil>, ColPtrCons<Dv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let c = unsafe { ctx.reader().read::<Cv, _>(i) };
            unsafe { ctx.writer().write::<Dv, _>(i, Dv(stage4(c.0))) };
        });
    }
}

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

// Build + populate is shared; the dispatch call differs per fixture. A macro
// expands the build inline in each fixture so the `#[inline(never)]` symbol
// boundary stays at the fixture fn, not a shared helper.
macro_rules! chain_scheduler {
    ($sched:ident) => {
        let provider = HeapBump::new(arena_bytes(5, N));
        let mut $sched = Scheduler::builder()
            .with(Column::<Dv>::new())
            .with(Column::<Cv>::new())
            .with(Column::<Bv>::new())
            .with(Column::<Av>::new())
            .with(Column::<Inv>::new())
            .with(S1)
            .with(S2)
            .with(S3)
            .with(S4)
            .build(store(provider), USize(N))
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        // In is the bindings head (last-registered).
        // SAFETY: In reserved for N records of Inv (repr u32); scheduler alive;
        // each reserved slot written once.
        let in_base = $sched.__bindings().__ptr().as_ptr() as *mut Inv;
        for i in 0..N {
            unsafe { *in_base.add(i) = Inv(i as u32) };
        }
    };
}

/// Hash the Dv output column (deepest registered: In -> Av -> Bv -> Cv -> Dv).
///
/// SAFETY: `dv_base` points at N reserved Dv records (repr u32) in a live arena.
unsafe fn dv_hash(dv_base: *const u32) -> u64 {
    let slice = unsafe { core::slice::from_raw_parts(dv_base, N) };
    fnv1a_u32_slice(slice)
}

// ----- fixture 1: column chain via run() (morsel-outer RAW path) -----

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_column_chain() -> u64 {
    chain_scheduler!(sched);
    let _ = sched.run();
    let dv_base = sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    black_box(unsafe { dv_hash(dv_base) })
}

// ----- fixture 2: linear RecordOp chain via run_fused() (register residency) -----

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_fused_chain() -> u64 {
    chain_scheduler!(sched);
    let _ = sched.run_fused();
    let dv_base = sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    black_box(unsafe { dv_hash(dv_base) })
}

// ----- fixture 4: column chain via run() after mark_dirty (E7 run_gated path) -----

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_dirty_gated() -> u64 {
    chain_scheduler!(sched);
    // Mark the root input dirty so the dirty-gated walk re-runs the cone (the
    // E7 run_gated predicated-branch path), not a clean-frame skip.
    sched.mark_dirty::<Column<Inv>, _>();
    let _ = sched.run();
    let dv_base = sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    black_box(unsafe { dv_hash(dv_base) })
}

// ----- fixture 3: accumulator appender via run() (unit-outer never-gated path) -----

#[derive(Copy, Clone)]
struct AInv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // appended by the WU, read back post-run as raw u32 for the hash
struct AAv(u32);

type AccW = Cons<Accum<AAv>, Empty>;

struct AppendChain;
impl BuilderInput for AppendChain {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for AppendChain {
    type Read = One<AInv>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<AInv>,
        AccW,
        PtrNil,
        ColPtrCons<AInv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, AAv, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; append advances live-length under the
            // reserved capacity (= record count).
            let inp = unsafe { ctx.reader().read::<AInv, _>(i) };
            unsafe { ctx.accums().append::<AAv, _>(AAv(chain(inp.0))) };
        });
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn asm_gate_accumulator() -> u64 {
    let provider = HeapBump::new(arena_bytes(2, N));
    let mut sched = Scheduler::builder()
        .with(Column::<AInv>::new())
        .with(Accum::<AAv>::new())
        .with(AppendChain)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    // Av accumulator is the head; AInv column is one tail down.
    // SAFETY: In reserved for N records of AInv (repr u32); scheduler alive.
    let in_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut AInv;
    for i in 0..N {
        unsafe { *in_base.add(i) = AInv(i as u32) };
    }
    let _ = sched.run();
    let len = sched.__bindings().__len_cell().get().0;
    let av_base = sched.__bindings().__ptr().as_ptr() as *const u32;
    // SAFETY: the appender wrote `len` records into the Av buffer; scheduler alive.
    let slice = unsafe { core::slice::from_raw_parts(av_base, len) };
    black_box(fnv1a_u32_slice(slice))
}
