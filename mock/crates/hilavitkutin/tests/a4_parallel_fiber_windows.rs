//! A4 acceptance: `run_parallel` windows each fiber by its own plan-baked size.
//!
//! Same two-width fixture as `a2b_loop_inversion`, driven through the parallel
//! path instead of `run`. Two disjoint one-unit pipelines land in two fibers
//! whose plan-baked L1 windows differ (a 4-byte write column windows wider than
//! an 8-byte one under the same budget), and they are column-disjoint so phase 0
//! holds two trunks and the dispatch takes the ordinary trunk-rank branch rather
//! than the head+tail one.
//!
//! Each unit counts its `execute` calls, and `execute` runs once per (fiber,
//! morsel), so a unit's count is its fiber's morsel count. The sharp claim: the
//! two counts DIFFER, and each matches `ceil(records / that fiber's window)`.
//! The shared-window shape this replaces gave both fibers the same count, since
//! one scalar drove the whole phase.
//!
//! Counts live in atomics rather than a thread-local, because the units run on
//! pool worker threads rather than the calling thread.
//!
//! Lives under `tests/` so the bare-numeric fixture values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

static NARROW_CALLS: AtomicUsize = AtomicUsize::new(0);
static WIDE_CALLS: AtomicUsize = AtomicUsize::new(0);

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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Enough records that both windows split the range more than once, so a
// per-fiber count is distinguishable from a shared one.
const RECORDS: usize = 7000;

#[derive(Copy, Clone)]
#[allow(dead_code)] // written for the record footprint; read back as raw u32
struct Narrow(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written for the record footprint; read back as raw u64
struct Wide(u64);

type NarrowCol = Cons<Column<Narrow>, Empty>;
type WideCol = Cons<Column<Wide>, Empty>;

type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

struct NarrowWu;
impl BuilderInput for NarrowWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for NarrowWu {
    type Read = Empty;
    type Write = NarrowCol;
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, NarrowCol, SnapNil, ColPtrNil, ColPtrCons<Narrow, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        NARROW_CALLS.fetch_add(1, Ordering::Relaxed);
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the column for the record count and the
            // plan proved this unit the exclusive writer; the morsel covers only
            // reserved records, and the writer windows to absolute.
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
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, WideCol, SnapNil, ColPtrNil, ColPtrCons<Wide, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        WIDE_CALLS.fetch_add(1, Ordering::Relaxed);
        ctx.each().run(|i| {
            // SAFETY: as above, exclusive writer over reserved records.
            unsafe { ctx.writer().write::<Wide, _>(i, Wide(i.0 as u64)) };
        });
    }
}

fn ceil_div(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

#[test]
fn parallel_fibers_window_by_their_own_plan_baked_size() {
    NARROW_CALLS.store(0, Ordering::Relaxed);
    WIDE_CALLS.store(0, Ordering::Relaxed);

    let provider = BumpProvider::<131072>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Narrow>::new())
        .with(Column::<Wide>::new())
        .with(NarrowWu)
        .with(WideWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Both fibers' plan-baked windows, read off the descriptors. They must
    // differ for a per-fiber count to discriminate from a shared one; the
    // formula itself is pinned by r6_morsel_window_formula, not here.
    let w0 = scheduler.__fiber_morsel_size(USize(0)).0;
    let w1 = scheduler.__fiber_morsel_size(USize(1)).0;
    assert!(
        w0 > 0 && w1 > 0,
        "both fibers carry a plan-baked window: {w0}, {w1}"
    );
    assert_ne!(
        w0, w1,
        "the two write widths must yield different windows for this test to bite"
    );
    assert!(
        RECORDS > w0 && RECORDS > w1,
        "records must exceed both windows to force a split"
    );

    // Poison both columns so an uncovered record is caught rather than reading
    // as a coincidentally-correct zero. Columns from head: Wide(0), Narrow(1).
    // SAFETY: both reserved for RECORDS records; the scheduler is alive.
    let wide_base = scheduler.__bindings().__ptr().as_ptr() as *mut u64;
    let narrow_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..RECORDS {
        unsafe {
            *wide_base.add(i) = u64::MAX;
            *narrow_base.add(i) = u32::MAX;
        }
    }

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    let narrow = NARROW_CALLS.load(Ordering::Relaxed);
    let wide = WIDE_CALLS.load(Ordering::Relaxed);

    // The load-bearing claim: the counts DIFFER. One scalar window driving the
    // whole phase would dispatch both fibers the same number of times.
    assert_ne!(
        narrow, wide,
        "each fiber walks its own window, so their morsel counts differ \
         (a shared phase window would give both the same count)"
    );

    // And each count is its own fiber's morsel count. Matched as an unordered
    // pair so the assertion does not depend on which descriptor slot a fiber
    // landed in.
    let mut got = [narrow, wide];
    let mut want = [ceil_div(RECORDS, w0), ceil_div(RECORDS, w1)];
    got.sort_unstable();
    want.sort_unstable();
    assert_eq!(
        got, want,
        "per-fiber morsel counts come from the windows {w0} and {w1}"
    );

    // Those morsels cover the whole range: no record still holds the poison.
    // Coverage is the claim, not the value, because `each()` yields a
    // morsel-RELATIVE index (the writer windows it to absolute), so record 3072
    // legitimately holds 0 when it opens the wide fiber's second morsel. The
    // count assertion above plus this coverage check together pin the range as
    // covered exactly once.
    let wide_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u64;
    let narrow_base = scheduler.as_ref().__bindings().__tail().__ptr().as_ptr() as *const u32;
    for i in 0..RECORDS {
        // SAFETY: both hold RECORDS reserved records; the scheduler is alive.
        let w = unsafe { *wide_base.add(i) };
        let n = unsafe { *narrow_base.add(i) };
        assert_ne!(
            w,
            u64::MAX,
            "wide rec {i} left unwritten by the window walk"
        );
        assert_ne!(
            n,
            u32::MAX,
            "narrow rec {i} left unwritten by the window walk"
        );
    }
}
