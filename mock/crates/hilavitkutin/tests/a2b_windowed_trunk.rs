//! Per-fiber morsel A2b-1: `run_one_trunk_windowed` dispatches one trunk
//! fiber-outer/morsel-inner.
//!
//! A one-WU carrier is one trunk. `run_one_trunk_windowed::<_,_,0>(window)` should
//! call the trunk's monomorphised program once per morsel window over
//! `[0, record_count)`, so the WU's `execute` runs `ceil(total/window)` times, each
//! over a distinct sub-range. `ctx.each()` yields morsel-LOCAL indices (0-based
//! within the morsel), so the observable that distinguishes windowing is the
//! per-call iteration COUNT (the morsel length), not an absolute start index (which
//! would be `[0, 0, 0]`). With `total = 10` and `window = 4` the recorded lengths
//! must be `[4, 4, 2]`. That distinguishes windowed dispatch from the single
//! whole-range call `run_one_trunk` makes (which would record `[10]`).
//!
//! Lives under `tests/` so the bare-numeric fixture values do not trip the src-tree
//! primitive lints.

mod stable_provider;

use core::cell::{Cell, RefCell};
use std::vec::Vec;

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

const RECORDS: usize = 10;

thread_local! {
    // The number of records iterated in each `execute` call = that morsel's length.
    // `ctx.each()` yields morsel-LOCAL indices (0-based within the morsel), so the
    // observable that distinguishes windowing is the per-call iteration count.
    static MORSEL_LENS: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

#[derive(Copy, Clone, Default)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
struct Outv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColOut = Cons<Column<Outv>, Empty>;

struct Windowed;
impl BuilderInput for Windowed {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Windowed {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColOut,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Outv, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = OneIn;
    type Write = ColOut;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let count = Cell::new(0usize);
        ctx.each().run(|i| {
            count.set(count.get() + 1);
            // SAFETY: Inv reserved by build; Outv reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Outv, _>(i, Outv(inp.0)) };
        });
        MORSEL_LENS.with(|s| s.borrow_mut().push(count.get()));
    }
}

#[test]
fn windowed_trunk_dispatches_one_morsel_per_window() {
    MORSEL_LENS.with(|s| s.borrow_mut().clear());
    let provider = stable_provider(16384, 0);
    let mut s = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Outv>::start())
        .with(Windowed)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // Outv is the head column (registered last); Inv sits one tail down. Inv
    // holds i + 1 so a record the windows skip keeps Outv's default 0.
    let in_base = s.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0 .. RECORDS {
        // SAFETY: Inv reserves RECORDS records; the scheduler is alive; each
        // slot is written once here before the run.
        unsafe { *in_base.add(i) = Inv(i as u32 + 1) };
    }
    // Window 4 over 10 records: morsels [0,4), [4,8), [8,10); lengths [4, 4, 2].
    s.run_one_trunk_windowed::<_, _, 0>(USize(4));
    MORSEL_LENS.with(|lens| {
        assert_eq!(
            *lens.borrow(),
            std::vec![4usize, 4, 2],
            "run_one_trunk_windowed windows the trunk fiber-outer/morsel-inner: one execute per morsel, lengths [4,4,2] over 10 records"
        );
    });
    // The three windows together cover every record once, each written at its
    // absolute index.
    let out_base = s.__bindings().__ptr().as_ptr() as *const Outv;
    for i in 0 .. RECORDS {
        // SAFETY: Outv reserves RECORDS records; the scheduler is alive.
        let v = unsafe { (*out_base.add(i)).0 };
        assert_eq!(
            v,
            i as u32 + 1,
            "rec {i}: the windows cover the range, each record written at its absolute index"
        );
    }
}
