//! A4 acceptance: `run_parallel` windows each fiber by its own plan-baked size.
//!
//! Same two-width fixture as `a2b_loop_inversion`, driven through the parallel
//! path instead of `run`. Two disjoint one-unit pipelines land in two fibers
//! whose plan-baked L1 windows differ (a 4-byte write column windows wider than
//! an 8-byte one under the same budget). They are column-disjoint, so phase 0
//! holds two trunks and each is dispatched by trunk rank.
//!
//! Each unit counts its `execute` calls, and `execute` runs once per (fiber,
//! morsel), so a unit's count is its fiber's morsel count. The sharp claim: the
//! two counts DIFFER, and each matches `ceil(records / that fiber's window)`,
//! the narrow column's unit by the wider window and the wide column's by the
//! narrower one. A walk that gave each fiber the other's window would produce
//! the same two counts on the wrong units, so the counts are asserted by unit.
//! The shared-window shape this replaces gave both fibers the same count, since
//! one scalar drove the whole phase.
//!
//! Counts live in atomics rather than a thread-local, because the units run on
//! pool worker threads rather than the calling thread.
//!
//! Lives under `tests/` so the bare-numeric fixture values do not trip the
//! src-tree primitive lints.

mod common;

use core::sync::atomic::{AtomicUsize, Ordering};

mod stable_provider;
use arvo::USize;
use common::TestExecutor;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::stable_provider;

static NARROW_CALLS: AtomicUsize = AtomicUsize::new(0);
static WIDE_CALLS: AtomicUsize = AtomicUsize::new(0);

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Enough records that both windows split the range more than once, so a
// per-fiber count is distinguishable from a shared one.
const RECORDS: usize = 7000;

#[derive(Copy, Clone, Default)]
struct Narrow(u32);
#[derive(Copy, Clone, Default)]
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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for NarrowWu {
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, NarrowCol, SnapNil, ColPtrNil, ColPtrCons<Narrow, ColPtrNil>>;
    type Hint = HintT;
    type Read = Empty;
    type Write = NarrowCol;

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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for WideWu {
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, WideCol, SnapNil, ColPtrNil, ColPtrCons<Wide, ColPtrNil>>;
    type Hint = HintT;
    type Read = Empty;
    type Write = WideCol;

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

    let provider = stable_provider(131072, 0);
    // Both columns start at their maximum so an uncovered record is caught rather
    // than reading as a coincidentally-correct zero.
    let scheduler = Scheduler::builder()
        .with(Column::<Narrow>::start_from(Narrow(u32::MAX)))
        .with(Column::<Wide>::start_from(Wide(u64::MAX)))
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

    let pool = TestExecutor::new();
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

    // And each count is its own fiber's morsel count. The narrow column's
    // fiber has the wider window, so it is asserted against `max(w0, w1)` and
    // the wide column's against `min(w0, w1)`. Naming the unit is the point:
    // comparing the two counts as an unordered pair would also pass a walk
    // that windowed each fiber by the other's size.
    let (wider, narrower) = (w0.max(w1), w0.min(w1));
    assert_eq!(
        narrow,
        ceil_div(RECORDS, wider),
        "the 4-byte column's fiber walks the wider window {wider}"
    );
    assert_eq!(
        wide,
        ceil_div(RECORDS, narrower),
        "the 8-byte column's fiber walks the narrower window {narrower}"
    );

    // Those morsels cover the whole range: no record still holds the start value.
    // Coverage is the claim, not the value, because `each()` yields a
    // morsel-RELATIVE index (the writer windows it to absolute), so record 3072
    // legitimately holds 0 when it opens the wide fiber's second morsel. The
    // count assertion above plus this coverage check together pin the range as
    // covered exactly once.
    let wide_base: *const Wide = scheduler.as_ref().__bindings().__ptr().as_ptr();
    let narrow_base: *const Narrow = scheduler.as_ref().__bindings().__tail().__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: both hold RECORDS reserved records; the scheduler is alive.
        let w = unsafe { (*wide_base.add(i)).0 };
        let n = unsafe { (*narrow_base.add(i)).0 };
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
