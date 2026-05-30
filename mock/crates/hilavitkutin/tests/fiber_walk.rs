//! RunFiber walk integration test (C2 slice 1).
//!
//! Two resource-only WorkUnits read distinct resource types from a real
//! scheduler arena; `run_fiber_walk` drives the two-unit sequence and the
//! test confirms both ran, in declaration order, each resolving its own
//! registered resource. The single-unit and empty-fiber cases pin the
//! recursion entry and the `WuNil` terminator. These live under `tests/`
//! so the bare byte buffer backing the test memory provider does not trip
//! the src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, PtrCons, PtrNil};
use hilavitkutin::dispatch::fiber_walk::{run_fiber_walk, WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{HasResourceProvider, ResourceProviderApi};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_api::work_unit::{Always, WorkUnit};

// Stack-backed test memory provider (mirrors tests/engine_ctx.rs).
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
fn walk_drives_two_resource_units_in_order() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<512>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(10)))
        .with(Resource::new(Rb(20)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    let arena = scheduler.__arena();

    // Fiber unit sequence [ReadAWu, ReadBWu]: distinct Read sets, so distinct
    // per-unit Ctx GAT instantiations, the heterogeneity the walk resolves.
    let fiber = WuCons {
        head: ReadAWu,
        tail: WuCons {
            head: ReadBWu,
            tail: WuNil,
        },
    };
    run_fiber_walk(&fiber, arena, MorselRange::new(USize::ZERO, USize::ZERO));

    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[10u32, 20u32],
            "both units ran in declaration order, each resolving its own registered resource"
        );
    });
}

#[test]
fn walk_single_unit() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<512>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(42)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    let arena = scheduler.__arena();

    let fiber = WuCons {
        head: ReadAWu,
        tail: WuNil,
    };
    run_fiber_walk(&fiber, arena, MorselRange::new(USize::ZERO, USize::ZERO));

    OBSERVED.with(|o| assert_eq!(o.borrow().as_slice(), &[42u32]));
}

#[test]
fn walk_empty_fiber_is_noop() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<512>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(7)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    let arena = scheduler.__arena();

    // The `WuNil` terminator runs nothing.
    run_fiber_walk(&WuNil, arena, MorselRange::new(USize::ZERO, USize::ZERO));
    OBSERVED.with(|o| assert!(o.borrow().is_empty()));
}
