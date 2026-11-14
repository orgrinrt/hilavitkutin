//! GATE-2 G-e: full per-trunk dispatch through `Scheduler::run` (round 2b).
//!
//! A producer (reads `In`, writes `Mid`) and a consumer (reads `Mid`, writes
//! `Out`) form a read-after-write chain, so the const grouping puts them in
//! distinct phases (the producer is the waist). The re-pointed single-core
//! `run` drives the outer per-trunk dispatcher: it must run the producer's phase
//! before the consumer's, or the consumer would read the start value left in `Mid`.
//! `Out[i] == (In[i] + 1) * 10` confirms the consumer saw the producer's output,
//! i.e. the dispatcher's phase loop ordered the trunks correctly and each WU's
//! members ran over the whole range.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

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
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use stable_provider::stable_provider;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

const N: usize = 64;

#[derive(Copy, Clone, Default)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
struct Mid(u32);
#[derive(Copy, Clone, Default)]
struct Out(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type ColMid = Cons<Column<Mid>, Empty>;
type ColOut = Cons<Column<Out>, Empty>;

// Producer: Mid = In + 1. Reads In, writes Mid. Phase 0 (the waist).
struct Producer;
impl BuilderInput for Producer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Producer {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        ColMid,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Mid, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColIn;
    type Write = ColMid;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Mid reserved + exclusive; morsel covers
            // this slice; reader/writer map relative -> absolute.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Mid, _>(i, Mid(inp.0 + 1)) };
        });
    }
}

// Consumer: Out = Mid * 10. Reads Mid, writes Out. Phase 1 (after the waist).
struct Consumer;
impl BuilderInput for Consumer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Consumer {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColMid,
        ColOut,
        SnapNil,
        ColPtrCons<Mid, ColPtrNil>,
        ColPtrCons<Out, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColMid;
    type Write = ColOut;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Mid produced this frame by the producer phase; Out reserved
            // + exclusive; morsel covers this slice.
            let mid = unsafe { ctx.reader().read::<Mid, _>(i) };
            unsafe { ctx.writer().write::<Out, _>(i, Out(mid.0 * 10)) };
        });
    }
}

#[test]
fn run_drives_two_phase_chain_in_order() {
    let provider = stable_provider(16384, 0);
    let mut scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Mid>::start_from(Mid(u32::MAX)))
        .with(Column::<Out>::start_from(Out(u32::MAX)))
        .with(Producer)
        .with(Consumer)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // In[i] = i is host-populated. Mid and Out start at u32::MAX so a wrong phase
    // order (consumer before producer) or a range gap surfaces as garbage.
    // SAFETY: In reserved for N records of u32; scheduler alive; each slot
    // written once here before the run.
    let in_base = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe {
            *in_base.add(i) = i as u32;
        }
    }

    let _ = scheduler.run::<_, _>();

    // Out[i] = (In[i] + 1) * 10. If the consumer ran before the producer, it
    // would have read the start value in Mid and Out would not match.
    let base = scheduler.__bindings().__ptr().as_ptr() as *const Out;
    for i in 0 .. N {
        // SAFETY: Out holds N reserved records; scheduler alive.
        let v = unsafe { (*base.add(i)).0 };
        assert_eq!(
            v,
            (i as u32 + 1) * 10,
            "rec {i}: consumer (phase 1) saw producer (phase 0) output; dispatcher ordered phases"
        );
    }
}
