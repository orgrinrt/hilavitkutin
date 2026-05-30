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
//! The walk order is registration-list order: the builder prepends each
//! `.with(unit)`, so the last-registered unit runs first. The test
//! asserts and documents that honestly. Dependency-topological order
//! from the execution plan is a later slice.
//!
//! Lives under `tests/` so the bare byte buffer backing the test memory
//! provider does not trip the src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, PtrCons, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{HasResourceProvider, ResourceProviderApi};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use notko::Outcome;

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
// own. Newtypes keep the recorder a single value stream.
struct Ra(u32);
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
    let provider = BumpProvider::<512>::new();
    // Register the resources both units read, then the two units. The
    // builder prepends each WorkUnit value, so the retained list runs
    // last-registered first: ReadBWu (reads Rb = 20), then ReadAWu
    // (reads Ra = 10). The order is registration-list order, documented
    // here; dependency-topological order is a later slice.
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(Resource::new(Rb(20)))
        .with(ReadAWu)
        .with(ReadBWu)
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[20u32, 10u32],
            "both registered units ran in registration-list order (last-registered first), \
             each resolving its own registered resource"
        );
    });
}

#[test]
fn run_walks_single_registered_unit() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<512>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(42)))
        .with(ReadAWu)
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| assert_eq!(o.borrow().as_slice(), &[42u32]));
}
