//! Scheduler run-walk integration test (C2 slice 2).
//!
//! Two resource-reading WorkUnits register through the builder, the
//! scheduler builds a real arena, and `run()` walks the retained unit
//! instances: each constructs its own `EngineCtx` from the arena and
//! reads its registered resource. This pins the slice-2 contract that
//! `build()` retains the registered WorkUnit instances and `run()`
//! executes them, distinct from slice 1 (which hand-built the unit list
//! and drove the walk directly).
//!
//! Slice 3 wires dependency-topological dispatch: `build()` computes the
//! execution plan from the registered bundle and `run()` dispatches in the
//! plan's topological order. Two independent readers carry no dependency
//! edge, so their topological order coincides with the registration walk
//! (the no-edge fallback the two-reader test documents); the reorder is
//! observable only when an edge exists, which the writer-before-reader test
//! pins. A cyclic registration is rejected at `build()` as
//! `BuildError::PlanFailed`.
//!
//! Lives under `tests/` so the bare byte buffer backing the test memory
//! provider does not trip the src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, PtrCons, PtrNil};
use hilavitkutin::plan::UnitMeta;
use hilavitkutin::scheduler::{BuildError, Scheduler};
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{HasResourceProvider, ResourceProviderApi};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/fiber_walk.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}

impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Two distinct resource types: the arena holds both, each unit reads its
// own. Newtypes keep the recorder a single value stream. Resources are
// `ColumnValue` now, so each derives `Copy`.
#[derive(Copy, Clone)]
struct Ra(u32);
#[derive(Copy, Clone)]
struct Rb(u32);

type ReadA = Cons<Resource<Ra>, Empty>;
type ReadB = Cons<Resource<Rb>, Empty>;

thread_local! {
    static OBSERVED: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

struct ReadAWu;

impl BuilderInput for ReadAWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ReadAWu {
    type Read = ReadA;
    type Write = Empty;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ReadA, Empty, PtrCons<Ra, PtrNil>, ColPtrNil, ColPtrNil>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let v: &Ra = ctx.resources().resource();
        OBSERVED.with(|o| o.borrow_mut().push(v.0));
    }
}

struct ReadBWu;

impl BuilderInput for ReadBWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ReadBWu {
    type Read = ReadB;
    type Write = Empty;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ReadB, Empty, PtrCons<Rb, PtrNil>, ColPtrNil, ColPtrNil>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let v: &Rb = ctx.resources().resource();
        OBSERVED.with(|o| o.borrow_mut().push(v.0));
    }
}

#[test]
fn run_walks_two_registered_units() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    // Register the resources both units read, then the two units. The two
    // readers touch disjoint resources (Ra, Rb) and neither writes, so the
    // plan's dependency graph has no edge between them. With no edge, the
    // topological order coincides with the registration walk: the builder
    // appends, so the retained list is [ReadAWu, ReadBWu] and dispatch runs
    // ReadAWu (Ra = 10) then ReadBWu (Rb = 20). The reorder is observable
    // only when an edge exists, which the writer-before-reader test covers.
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(Resource::new(Rb(20)))
        .with(ReadAWu)
        .with(ReadBWu)
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[10u32, 20u32],
            "both registered units ran in registration-list order (first-registered first), \
             each resolving its own registered resource"
        );
    });
}

#[test]
fn run_walks_single_registered_unit() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(42)))
        .with(ReadAWu)
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| assert_eq!(o.borrow().as_slice(), &[42u32]));
}

// Writer over Ra: declares `Write = {Resource<Ra>}` so the plan adds a RAW
// edge to any later reader of Ra. Read is empty, so its read bundle is
// `PtrNil`; its execute records a sentinel marker. Resource-write-pointer
// projection is a later slice, so the write declaration exists only for the
// dependency edge, not to actually write Ra.
type WriteA = Cons<Resource<Ra>, Empty>;

// Sentinel the writer records, distinct from any resource value, so the
// observed sequence names which unit ran when.
const WRITER_MARKER: u32 = 99;

struct WriteAWu;

impl BuilderInput for WriteAWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for WriteAWu {
    type Read = Empty;
    type Write = WriteA;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, WriteA, PtrNil, ColPtrNil, ColPtrNil>;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        OBSERVED.with(|o| o.borrow_mut().push(WRITER_MARKER));
    }
}

#[test]
fn run_dispatches_in_topological_order_not_registration() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    // Register the writer of Ra FIRST, then the reader of Ra LAST. The builder
    // appends, so the retained value list is [writer, reader] in registration
    // order. The plan adds a RAW edge writer to reader (the writer writes what
    // the reader reads), so the topological dispatch order is [writer, reader]:
    // the writer runs before its reader. With the producer-before-consumer
    // registration here the topological order coincides with registration; the
    // reorder away from an anti-topological registration is exercised by the
    // phase-sequential dispatch test (consumer registered first).
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(WriteAWu)
        .with(ReadAWu)
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[WRITER_MARKER, 10u32],
            "dispatch follows the plan's topological order (writer before its \
             reader), not the registration walk (which would run the \
             last-registered reader first)"
        );
    });
}

// Two units that form a mutual data dependency: CycleAWu reads Ra and writes
// Rb; CycleBWu reads Rb and writes Ra. The plan adds RAW edges in both
// directions (each writes what the other reads), so `topo_sort` cannot place
// every unit. These never run; only the build-time plan rejection is
// exercised, so their execute bodies are empty.
struct CycleAWu;

impl BuilderInput for CycleAWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for CycleAWu {
    type Read = ReadA;
    type Write = ReadB;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ReadA, ReadB, PtrCons<Ra, PtrNil>, ColPtrNil, ColPtrNil>;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

struct CycleBWu;

impl BuilderInput for CycleBWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for CycleBWu {
    type Read = ReadB;
    type Write = ReadA;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ReadB, ReadA, PtrCons<Rb, PtrNil>, ColPtrNil, ColPtrNil>;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

#[test]
fn build_rejects_a_cyclic_registration() {
    let provider = BumpProvider::<8192>::new();
    // CycleAWu (reads Ra, writes Rb) and CycleBWu (reads Rb, writes Ra) form a
    // dependency cycle. `build` computes the plan before draining the store
    // arena; `compute_execution_plan` returns `PlanError::Cycle`, so `build`
    // returns `BuildError::PlanFailed` and allocates nothing.
    let result = Scheduler::builder()
        .with(Resource::new(Ra(1)))
        .with(Resource::new(Rb(2)))
        .with(CycleAWu)
        .with(CycleBWu)
        .build(store(provider), USize(0));

    assert!(matches!(result, Outcome::Err(BuildError::PlanFailed)));
}

#[test]
fn build_rejects_anti_topological_registration() {
    let provider = BumpProvider::<8192>::new();
    // Register the reader of Ra FIRST, then the writer of Ra LAST. The builder
    // appends, so the retained carrier order is [reader, writer]: slot 0 reads
    // Ra, slot 1 writes Ra. The plan adds a RAW edge writer to reader (the
    // writer writes what the reader reads), which is a back-edge in carrier
    // space (source slot 1 >= destination slot 0). The static dispatch walk
    // follows carrier order directly, so this carrier would run the reader
    // before its writer. `build` rejects it at the precondition,
    // before any allocation, naming the offending slots: producer (writer) at
    // slot 1, consumer (reader) at slot 0.
    let result = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(ReadAWu)
        .with(WriteAWu)
        .build(store(provider), USize(0));

    match result {
        Outcome::Err(BuildError::NonTopologicalRegistration {
            producer,
            consumer,
            recommended,
        }) => {
            assert_eq!(
                (producer, consumer),
                (USize(1), USize(0)),
                "the gate names producer slot 1 (writer) registered after consumer \
                 slot 0 (reader)"
            );
            // The recommended order is a valid topological order: the writer
            // (slot 1) precedes the reader (slot 0) in the named sequence.
            let order = recommended.as_slice();
            let pos_writer = order.iter().position(|s| *s == USize(1));
            let pos_reader = order.iter().position(|s| *s == USize(0));
            assert!(
                pos_writer.is_some() && pos_reader.is_some(),
                "recommended order names both carrier slots, got {:?}",
                recommended
            );
            assert!(
                pos_writer < pos_reader,
                "recommended order places the writer (slot 1) before the reader \
                 (slot 0), got {:?}",
                recommended
            );
        }
        Outcome::Err(other) => panic!(
            "an acyclic registration whose carrier order runs a reader before its \
             writer is rejected with NonTopologicalRegistration (not another \
             BuildError), got {:?}",
            other
        ),
        Outcome::Ok(_) => panic!(
            "an acyclic registration whose carrier order runs a reader before its \
             writer must be rejected, not build successfully"
        ),
    }
}

#[test]
fn build_store_backs_the_plan_unit_meta() {
    // Writer-of-Ra registered before reader-of-Ra: the plan's topological
    // order is [writer, reader] (the same order the dispatch reorder test
    // pins). `build` store-backs the plan; read the `unit_meta` column back
    // out of the scheduler's storage through the `PlanHandle` and confirm it
    // carries that order. Before store-backing, the column is unreserved
    // (count zero), so the round-trip assertions fail.
    let provider = BumpProvider::<8192>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(WriteAWu)
        .with(ReadAWu)
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let handle = scheduler.__plan_handle();
    let storage = scheduler.__storage();
    let id = handle.unit_meta_id();

    // Two units registered, so the unit-meta column holds two records.
    assert_eq!(handle.unit_count(), USize(2));
    assert_eq!(storage.count(id), USize(2));

    // Read the store-backed unit-meta column. Topo step 0 is the writer
    // (registration index 0, since the builder appends), step 1 is the
    // reader (registration index 1): the dependency-topological dispatch
    // order, round-tripped through the store.
    // SAFETY: `build` reserved this column for `unit_count` `UnitMeta` records
    // and copied the plan's unit-meta prefix in; the scheduler (and its
    // storage) is alive for this read, and no writer aliases the frozen plan
    // columns.
    let meta0 = unsafe { *storage.column_ptr::<UnitMeta>(id) };
    let meta1 = unsafe { *storage.column_ptr::<UnitMeta>(id).add(1) };
    assert_eq!(
        meta0.id.index(),
        USize(0),
        "topo step 0 is the writer (registration index 0)"
    );
    assert_eq!(
        meta1.id.index(),
        USize(1),
        "topo step 1 is the reader (registration index 1)"
    );
}
