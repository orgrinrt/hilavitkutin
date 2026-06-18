//! GATE-2 const-gated per-trunk walk: member selectivity (round 2a).
//!
//! Two column-disjoint work units that read nothing and write distinct columns:
//! both land in phase 0, and because their write sets share no column they fall
//! in distinct trunks (`WuA` at carrier position 0 is trunk 0; `WuB` at position
//! 1 is trunk 1, each its own component, id = its min member position).
//! `run_one_trunk::<_, _, TRUNK>` walks the carrier gated on the compile-time
//! grouping and runs only that trunk's member (a trunk lies wholly within one
//! phase, so the trunk id alone selects members). Each unit
//! records into its own observer, so dispatching trunk 0 must fire only `WuA`'s
//! observer and dispatching trunk 1 only `WuB`'s: every non-member position folds
//! away.
//!
//! Red first: `Scheduler::run_one_trunk` / `RunGatedTrunk` / the grouping fold
//! over `WuCons` do not exist before this round, so the file does not compile.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

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
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 4;

#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;

thread_local! {
    static OBS_A: RefCell<Vec<u32>> = RefCell::new(Vec::new());
    static OBS_B: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

// WuA: writes Column<Av>, reads nothing -> phase 0, trunk {Av}.
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
        ctx.each().run(|i| {
            unsafe { ctx.writer().write::<Av, _>(i, Av(i.0 as u32 * 10)) };
            OBS_A.with(|o| o.borrow_mut().push(i.0 as u32 * 10));
        });
    }
}

// WuB: writes Column<Bv>, reads nothing -> phase 0, trunk {Bv} (disjoint from A).
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
        ctx.each().run(|i| {
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(i.0 as u32 * 100)) };
            OBS_B.with(|o| o.borrow_mut().push(i.0 as u32 * 100));
        });
    }
}

#[test]
fn run_one_trunk_runs_only_trunk0_member() {
    OBS_A.with(|o| o.borrow_mut().clear());
    OBS_B.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(WuA)
        .with(WuB)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // Trunk 0 = WuA (carrier position 0, phase 0). Only WuA's observer fires.
    scheduler.run_one_trunk::<_, _, 0>();
    OBS_A.with(|o| assert_eq!(o.borrow().as_slice(), &[0u32, 10, 20, 30], "WuA (trunk 0) ran"));
    OBS_B.with(|o| assert!(o.borrow().is_empty(), "WuB (trunk 1) did not run"));
}

#[test]
fn run_one_trunk_runs_only_trunk1_member() {
    OBS_A.with(|o| o.borrow_mut().clear());
    OBS_B.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(WuA)
        .with(WuB)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // Trunk 1 = WuB (carrier position 1, phase 0). Only WuB's observer fires.
    scheduler.run_one_trunk::<_, _, 1>();
    OBS_B.with(|o| assert_eq!(o.borrow().as_slice(), &[0u32, 100, 200, 300], "WuB (trunk 1) ran"));
    OBS_A.with(|o| assert!(o.borrow().is_empty(), "WuA (trunk 0) did not run"));
}

#[test]
fn run_all_trunks_runs_every_trunk_once() {
    OBS_A.with(|o| o.borrow_mut().clear());
    OBS_B.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(WuA)
        .with(WuB)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // The outer dispatcher: both trunks (WuA = trunk 0, WuB = trunk 1, both
    // phase 0) run, each exactly once, every member over the whole range.
    scheduler.run_all_trunks::<_, _>();
    OBS_A.with(|o| assert_eq!(o.borrow().as_slice(), &[0u32, 10, 20, 30], "WuA (trunk 0) ran once"));
    OBS_B.with(|o| assert_eq!(o.borrow().as_slice(), &[0u32, 100, 200, 300], "WuB (trunk 1) ran once"));
}
