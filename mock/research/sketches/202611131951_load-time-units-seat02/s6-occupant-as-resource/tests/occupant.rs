//! Seat 02 phase two. Question: on the trunk engine, can code that arrives
//! after `build` change what a compiled unit computes, with the unit set
//! fixed, and does the plan see the change as a plan-affecting parameter
//! (the reading of "PlanStage fired on plugin load" consistent with R6)?
//!
//! The occupant is a C-ABI function pointer held in a `Resource` that is
//! `PlanAffecting`. The host unit declares that resource in its read set, so
//! the plan orders it by its access set like any other reader. Swapping the
//! occupant is `replace_resource`, which seeds plan-dirty (engine design
//! :97-99, :893-899).
//!
//! The context hands a unit one record at a time and no window slice
//! (hilavitkutin-api/src/context.rs:117, :134), so the host copies the morsel
//! into scratch and back, as seat 01's a4 had to. That copy is scaffolding
//! the trunk forces; it is not part of the claim.

use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    BatchApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use s6_occupant_as_resource::{occupant_v1, occupant_v2};

struct ZeroedHeap;
unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}
impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        unsafe {
            std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        unsafe {
            std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}

fn store<M: MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

type Kernel = extern "C" fn(*const i64, *mut i64, usize);

/// The occupant: a C-ABI kernel, `Copy + 'static`, so a column value and a
/// resource like any other.
#[derive(Copy, Clone)]
struct Occupant(Kernel);
impl hilavitkutin_api::run_cfg::PlanAffecting for Occupant {}
impl hilavitkutin_api::footprint::ResourceFootprint for Occupant {
    const L1_BYTES: USize = USize(0);
}

#[derive(Copy, Clone)]
struct A(i64);
#[derive(Copy, Clone)]
struct B(i64);

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);
const N: usize = 20_000;

thread_local! {
    static SEEN: RefCell<Vec<(i64, i64)>> = RefCell::new(Vec::new());
    static HOST_RUNS: RefCell<u32> = RefCell::new(0);
}

struct Loader;
impl BuilderInput for Loader {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Loader {
    type Ctx<'f> = EngineCtx<'f, Empty, Cons<Column<A>, Empty>, SnapNil, ColPtrNil, ColPtrCons<A, ColPtrNil>>;
    type Hint = Hints;
    type Read = Empty;
    type Write = Cons<Column<A>, Empty>;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| unsafe {
            ctx.writer().write::<A, _>(i, A((i.0 as i64 % 97) - 40));
        });
    }
}

type HostRead = Cons<Resource<Occupant>, Cons<Column<A>, Empty>>;
type HostWrite = Cons<Column<B>, Empty>;

/// The compiled host unit. Its access set is the build's; what it runs is
/// whatever occupant the resource holds this frame.
struct Host;
impl BuilderInput for Host {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Host {
    type Ctx<'f> = EngineCtx<
        'f,
        HostRead,
        HostWrite,
        SnapCons<Occupant, SnapNil>,
        ColPtrCons<A, ColPtrNil>,
        ColPtrCons<B, ColPtrNil>,
    >;
    type Hint = Hints;
    type Read = HostRead;
    type Write = HostWrite;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        HOST_RUNS.with(|c| *c.borrow_mut() += 1);
        let occ: &Occupant = ctx.resources().resource();
        let kernel = occ.0;
        BatchApi::run(ctx.batch(), |_, len| {
            let n = len.0;
            let a: Vec<i64> = (0 .. n).map(|i| unsafe { ctx.reader().read::<A, _>(USize(i)).0 }).collect();
            let mut out = vec![0i64; n];
            // One call a morsel across the C ABI.
            kernel(a.as_ptr(), out.as_mut_ptr(), n);
            for (i, v) in out.iter().enumerate() {
                unsafe { ctx.writer().write::<B, _>(USize(i), B(*v)) };
            }
        });
    }
}

struct Observer;
impl BuilderInput for Observer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
type ObsRead = Cons<Column<A>, Cons<Column<B>, Empty>>;
impl WorkUnit<Always> for Observer {
    type Ctx<'f> = EngineCtx<'f, ObsRead, Empty, SnapNil, ColPtrCons<A, ColPtrCons<B, ColPtrNil>>, ColPtrNil>;
    type Hint = Hints;
    type Read = ObsRead;
    type Write = Empty;

    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        EachApi::run(ctx.each(), |i| {
            let r = ctx.reader();
            let t = unsafe { (r.read::<A, _>(i).0, r.read::<B, _>(i).0) };
            SEEN.with(|s| s.borrow_mut().push(t));
        });
    }
}

fn seen_take() -> Vec<(i64, i64)> {
    SEEN.with(|s| std::mem::take(&mut *s.borrow_mut()))
}

#[test]
fn an_occupant_swapped_after_build_changes_the_next_frame_and_opens_the_plan_band_once() {
    seen_take();
    HOST_RUNS.with(|c| *c.borrow_mut() = 0);
    let mut s = Scheduler::builder()
        .with(Resource::new(Occupant(occupant_v1)))
        .with(Column::<A>::new())
        .with(Column::<B>::new())
        .with(Loader)
        .with(Host)
        .with(Observer)
        .build(store(ZeroedHeap), USize(N))
        .unwrap_or_else(|_| panic!("build"));

    // Frame 1: cold, occupant v1.
    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(s.__plan_band().0, "the cold frame enters the plan band");
    let f1 = seen_take();
    assert_eq!(f1.len(), N, "frame 1 observer saw every record");
    assert!(f1.iter().all(|&(a, b)| b == a + 1), "frame 1 ran occupant v1 on every record");
    let runs_after_1 = HOST_RUNS.with(|c| *c.borrow());

    // Frame 2: nothing changed.
    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(!s.__plan_band().0, "a clean frame does not enter the plan band");
    let runs_after_2 = HOST_RUNS.with(|c| *c.borrow());
    let f2 = seen_take();
    println!(
        "frame 2 (clean): host runs {} -> {}, observer records {}",
        runs_after_1,
        runs_after_2,
        f2.len()
    );

    // A new occupant arrives after build: the "plugin load".
    s.replace_resource(Occupant(occupant_v2));

    // Frame 3: the plan band opens once, the host reruns with v2.
    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(s.__plan_band().0, "an occupant swap opens the plan band on the next frame");
    let runs_after_3 = HOST_RUNS.with(|c| *c.borrow());
    assert!(runs_after_3 > runs_after_2, "the host unit reran after the swap");
    let f3 = seen_take();
    println!(
        "frame 3 (after swap): host runs {} -> {}, observer records {}",
        runs_after_2,
        runs_after_3,
        f3.len()
    );
    assert_eq!(f3.len(), N, "frame 3 observer saw every record");
    assert!(f3.iter().all(|&(a, b)| b == a * 3), "frame 3 ran occupant v2 on every record");

    // Frame 4: the bit was consumed.
    assert!(matches!(s.run(), Outcome::Ok(())));
    assert!(!s.__plan_band().0, "one swap buys one plan band");
}
