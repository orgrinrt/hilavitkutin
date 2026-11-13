//! Two checks of the founding record's cell bound against the trunk.
//!
//! `a_4_kib_cell_is_a_column_value`: the record says a type wider than 16
//! bytes is refused at compile time. If this file compiles, the trunk does not
//! keep that property. It compiles (see test-output.txt), so the property is
//! gone, and the refusal the record names has no expression on the trunk.
//!
//! `the_morsel_window_of_an_oversized_cell_fits_l1`: the record's morsel
//! promise is that a fiber's write set for one morsel fits the usable L1
//! (spec :406-407, `L1_usable / Σ size_of`). The trunk's window is clamped
//! below at MIN_MORSEL, so an oversized cell gets a window whose write set is
//! many times the budget, silently. Expected red.

fn store<M: hilavitkutin_api::MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

use arvo::USize;
use c1_oversized_cell::ZeroedHeap;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::ColumnValue;
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
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

/// 4 KiB, 256 times the record's 16-byte ceiling.
#[derive(Copy, Clone)]
struct Big([u64; 512]);
#[derive(Copy, Clone)]
struct BigOut([u64; 512]);

const fn is_column_value<T: ColumnValue>() -> usize {
    core::mem::size_of::<T>()
}

#[test]
fn a_4_kib_cell_is_a_column_value() {
    // The record: refused at compile time. The trunk: this line compiles.
    const WIDTH: usize = is_column_value::<Big>();
    assert_eq!(WIDTH, 4096);
}

type OneIn = Cons<Column<Big>, Empty>;
type OneOut = Cons<Column<BigOut>, Empty>;

struct Copyer;
impl BuilderInput for Copyer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Copyer {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        OneOut,
        SnapNil,
        ColPtrCons<Big, ColPtrNil>,
        ColPtrCons<BigOut, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = OneIn;
    type Write = OneOut;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<Big, _>(i) };
            unsafe { ctx.writer().write::<BigOut, _>(i, BigOut(v.0)) };
        });
    }
}

#[test]
fn the_morsel_window_of_an_oversized_cell_fits_l1() {
    let s = Scheduler::builder()
        .with(Column::<Big>::new())
        .with(Column::<BigOut>::new())
        .with(Copyer)
        .build(store(ZeroedHeap), USize(1024))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let window = s.__fiber_morsel_size(USize(0)).0;
    let write_bytes = window * core::mem::size_of::<BigOut>();
    let budget = <DefaultRunCfg as RunCfg>::L1_USABLE.0;
    println!("window {window} records, write set {write_bytes} bytes, L1 budget {budget} bytes");
    assert!(
        write_bytes <= budget,
        "a {window}-record morsel of a 4 KiB cell writes {write_bytes} bytes against an L1 budget of {budget}"
    );
}
