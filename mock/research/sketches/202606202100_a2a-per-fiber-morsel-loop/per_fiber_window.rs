//! Sketch A2a: per-fiber morsel-windowed dispatch over the GATE-2 carrier.
//!
//! Hypothesis: the per-trunk monomorphised dispatch (`RunGatedTrunk::run_trunk`)
//! can be driven once per fiber per window with a distinct `MorselRange`,
//! WITHOUT (a) a per-record indirect call and WITHOUT (b) breaking the
//! phase-gated const-DCE. `MorselRange` is a plain runtime value threaded
//! `run_trunk -> run_head`; every DCE site (`Member::IS`, `GateWith::open`) is
//! compile-time and reads neither the range nor the dirty mask, so distinct
//! per-call ranges cannot perturb codegen.
//!
//! The windowing loop lives on a temporary `Scheduler` method
//! `run_one_trunk_windowed` (see `proposed_engine_method.rs.txt`): identical to
//! the shipped `run_one_trunk` but with an inner `while start < total` loop
//! calling `run_trunk` per window. The method gets the same witness inference
//! `run_one_trunk` does (the trait obligation is byte-identical), which is why a
//! free standalone driver cannot work but a method can.
//!
//! Fixture mirrors `tests/gate2_gated_walk.rs`: two column-disjoint units, each
//! its own trunk AND its own single-member fiber, both phase 0. The sketch
//! drives fiber-outer / morsel-inner: fiber 0 (trunk 0) walks its whole window
//! sequence over `[0,total)` before fiber 1 (trunk 1). Each `execute` call
//! records `(tag, window_start)`; the assertion proves the per-fiber window
//! sequence and the fiber-outer order.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}
impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self { buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]), used: Cell::new(0) }
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
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const TOTAL: usize = 10;

#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;

thread_local! {
    // Each entry is (tag, window_start): the dispatch order across (fiber, window).
    static ORDER: RefCell<Vec<(u8, usize)>> = RefCell::new(Vec::new());
}

struct WuA;
impl BuilderInput for WuA {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for WuA {
    type Read = Empty;
    type Write = ColA;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, ColA, PtrNil, ColPtrNil, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let mut first: Option<usize> = None;
        ctx.each().run(|i| {
            if first.is_none() {
                first = Some(i.0);
            }
            unsafe { ctx.writer().write::<Av, _>(i, Av(i.0 as u32 * 10)) };
        });
        if let Some(s) = first {
            ORDER.with(|o| o.borrow_mut().push((0u8, s)));
        }
    }
}

struct WuB;
impl BuilderInput for WuB {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for WuB {
    type Read = Empty;
    type Write = ColB;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, ColB, PtrNil, ColPtrNil, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let mut first: Option<usize> = None;
        ctx.each().run(|i| {
            if first.is_none() {
                first = Some(i.0);
            }
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(i.0 as u32 * 100)) };
        });
        if let Some(s) = first {
            ORDER.with(|o| o.borrow_mut().push((1u8, s)));
        }
    }
}

#[test]
fn per_fiber_morsel_windowing_via_run_trunk() {
    ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(WuA)
        .with(WuB)
        .build(store(provider), USize(TOTAL))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Fiber 0 (trunk 0) windows of 4 over [0,10): [0,4)[4,8)[8,10) -> starts 0,4,8.
    // Fiber 1 (trunk 1) windows of 8 over [0,10): [0,8)[8,10)      -> starts 0,8.
    // Fiber-outer: fiber 0's whole window sequence runs before fiber 1's.
    scheduler.run_one_trunk_windowed::<_, _, 0>(USize(4));
    scheduler.run_one_trunk_windowed::<_, _, 1>(USize(8));

    ORDER.with(|o| {
        let observed = o.borrow();
        let expected: Vec<(u8, usize)> =
            vec![(0, 0), (0, 4), (0, 8), (1, 0), (1, 8)];
        assert_eq!(
            observed.as_slice(),
            expected.as_slice(),
            "fiber-outer / morsel-inner: each fiber walks its own window sequence \
             over [0,total) before the next fiber"
        );
    });
}
