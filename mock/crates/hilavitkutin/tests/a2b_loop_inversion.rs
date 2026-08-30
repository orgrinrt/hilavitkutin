//! A2b acceptance: `run` dispatches fiber-outer/morsel-inner per fiber window.
//!
//! Two disjoint one-unit pipelines land in two fibers whose plan-baked L1
//! windows differ (a 4-byte write column windows wider than an 8-byte one
//! under the same budget). Each unit records its tag once per `execute` call,
//! and `execute` runs once per (fiber, morsel), so the recorded sequence is
//! the dispatch order across the per-fiber window walks. Fiber-outer means
//! the first descriptor's fiber completes its whole window sequence before
//! the second starts: the tags form two contiguous blocks, block `i` of
//! length `ceil(records / window_i)` in descriptor order. The shared-window
//! shape this replaces would interleave both units once per uniform window.
//!
//! The second test pins the per-fiber skip refinement on a mixed carrier: an
//! accumulator fiber re-runs every frame (per-frame append is never gated)
//! while a clean `morsel_local` fiber is skipped, where the old global
//! either/or ran every unit whenever any accumulator fiber existed.
//!
//! The third pins the record-less frame: each fiber dispatches once over the
//! empty range, so a unit runs exactly once.
//!
//! Lives under `tests/` so the bare-numeric fixture values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;
use std::vec::Vec;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

thread_local! {
    static DISPATCH_ORDER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/column_dispatch.rs).
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
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Two morsels for the narrow pipeline and more for the wide one: under the
// default budget a 4-byte write column windows at 24576 / 4 = 6144 and an
// 8-byte one at 24576 / 8 = 3072, so 7000 records split 2 vs 3 ways.
const RECORDS: usize = 7000;

const NARROW_TAG: u8 = 0;
const WIDE_TAG: u8 = 1;

// Disjoint column values: a 4-byte and an 8-byte record, so the two pipelines
// land in separate fibers with different L1 windows.
#[derive(Copy, Clone)]
#[allow(dead_code)] // written for the record footprint; the test observes call tags
struct Narrow(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written for the record footprint; the test observes call tags
struct Wide(u64);

type NarrowCol = Cons<Column<Narrow>, Empty>;
type WideCol = Cons<Column<Wide>, Empty>;

struct NarrowWu;
impl BuilderInput for NarrowWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for NarrowWu {
    type Read = Empty;
    type Write = NarrowCol;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, NarrowCol, SnapNil, ColPtrNil, ColPtrCons<Narrow, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(NARROW_TAG));
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the column for the record count and the
            // plan proved this unit the exclusive writer; the morsel covers
            // only reserved records.
            unsafe { ctx.writer().write::<Narrow, _>(i, Narrow(i.0 as u32)) };
        });
    }
}

struct WideWu;
impl BuilderInput for WideWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for WideWu {
    type Read = Empty;
    type Write = WideCol;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, WideCol, SnapNil, ColPtrNil, ColPtrCons<Wide, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(WIDE_TAG));
        ctx.each().run(|i| {
            // SAFETY: as above, exclusive writer over reserved records.
            unsafe { ctx.writer().write::<Wide, _>(i, Wide(i.0 as u64)) };
        });
    }
}

#[test]
fn fibers_window_by_their_own_plan_baked_size() {
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    // 7000 * (4 + 8) bytes of column data plus plan columns.
    let provider = BumpProvider::<131072>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Narrow>::new())
        .with(Column::<Wide>::new())
        .with(NarrowWu)
        .with(WideWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Read both fibers' plan-baked windows off the descriptors (descriptor
    // order; r6_morsel_window_formula pins the formula itself). The windows
    // must differ for the per-fiber claim to discriminate from a shared one.
    let w0 = scheduler.__fiber_morsel_size(USize(0)).0;
    let w1 = scheduler.__fiber_morsel_size(USize(1)).0;
    assert!(w0 > 0 && w1 > 0, "both fibers carry plan-baked windows");
    assert_ne!(w0, w1, "the two pipelines' windows must differ");
    assert!(RECORDS > w0 && RECORDS > w1, "the record count must split both fibers");

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        // Fiber-outer: the first descriptor's fiber runs its whole window
        // sequence (ceil(RECORDS / w0) calls), then the second runs its own
        // (ceil(RECORDS / w1) calls). Two contiguous single-tag blocks; a
        // shared-window interleave would alternate the tags.
        let n0 = RECORDS.div_ceil(w0);
        let n1 = RECORDS.div_ceil(w1);
        assert_eq!(observed.len(), n0 + n1, "one execute per (fiber, morsel)");
        let first = observed[0];
        let second = observed[n0];
        assert_ne!(first, second, "each fiber forms its own block");
        assert!(
            observed[..n0].iter().all(|&t| t == first),
            "fiber 0 completes its whole window sequence before fiber 1: {observed:?}"
        );
        assert!(
            observed[n0..].iter().all(|&t| t == second),
            "fiber 1 runs after fiber 0 completed: {observed:?}"
        );
    });
}

// ----- mixed carrier: per-fiber skip -----

const COL_TAG: u8 = 10;
const ACC_TAG: u8 = 11;

#[derive(Copy, Clone)]
#[allow(dead_code)] // written for the record footprint; the test observes call tags
struct Cm(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // appended for the side effect; the test observes call tags
struct Ov(u32);

type CmCol = Cons<Column<Cm>, Empty>;
type OvAcc = Cons<Accum<Ov>, Empty>;

struct ColWu;
impl BuilderInput for ColWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ColWu {
    type Read = Empty;
    type Write = CmCol;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, CmCol, SnapNil, ColPtrNil, ColPtrCons<Cm, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(COL_TAG));
        ctx.each().run(|i| {
            // SAFETY: exclusive writer over reserved records.
            unsafe { ctx.writer().write::<Cm, _>(i, Cm(i.0 as u32)) };
        });
    }
}

struct AccWu;
impl BuilderInput for AccWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for AccWu {
    type Read = Empty;
    type Write = OvAcc;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        OvAcc,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Ov, AccPtrNil>,
    >;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(ACC_TAG));
        ctx.each().run(|_i| {
            // SAFETY: append advances live-length under the reserved capacity
            // (= record count); the frame reset zeroed it.
            unsafe { ctx.accums().append::<Ov, _>(Ov(1)) };
        });
    }
}

#[test]
fn clean_frame_skips_column_fiber_but_reruns_accumulator_fiber() {
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Cm>::new())
        .with(Accum::<Ov>::new())
        .with(ColWu)
        .with(AccWu)
        .build(store(provider), USize(100))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Cold frame: every fiber runs (100 records fit one window per fiber).
    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));
    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        assert!(observed.contains(&COL_TAG), "cold frame runs the column fiber");
        assert!(observed.contains(&ACC_TAG), "cold frame runs the accumulator fiber");
    });

    // Clean frame: no store was marked dirty, so the column fiber's RAW
    // recompute is skipped (identical output), while the accumulator fiber
    // re-runs (its buffer was reset and must be re-appended). The old global
    // either/or ran the column fiber here too because an accumulator fiber
    // existed in the carrier.
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));
    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        assert_eq!(
            observed.as_slice(),
            &[ACC_TAG],
            "a clean frame skips the clean column fiber and re-runs the accumulator fiber"
        );
    });
}

#[test]
fn record_less_frame_runs_each_fiber_exactly_once() {
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Narrow>::new())
        .with(Column::<Wide>::new())
        .with(NarrowWu)
        .with(WideWu)
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let mut observed = o.borrow().clone();
        observed.sort_unstable();
        // Each fiber dispatches once over the empty range: a unit's `execute`
        // runs exactly once (its `each()` iterates zero records).
        assert_eq!(
            observed.as_slice(),
            &[NARROW_TAG, WIDE_TAG],
            "a record-less frame runs each unit exactly once"
        );
    });
}
