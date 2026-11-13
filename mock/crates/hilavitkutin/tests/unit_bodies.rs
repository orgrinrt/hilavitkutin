//! The two bodies of an ordinary unit on the engine.
//!
//! A value function reads a column and two resources, one by value and one in
//! place, and what it returns lands in its write column. A positional body
//! writes a plain column listed bare, under `Written` and under `Differs` with
//! the one call. An ordinary unit's readers follow it by the plan's edges, so
//! none of the three leaves a mark for the next pass.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapCons,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasEach};
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Column, Replaceable, Resource, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Differs, ResourceRead, Written, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;

const RECORDS: usize = 600;
const SENTINEL: u32 = 0xDEAD;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Slot(u32);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Height(u32);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Seen(u32);

/// A resource of one scalar, read by value.
#[derive(Copy, Clone)]
struct Step(u32);

impl ResourceRead for Step {
    type Val<'a> = Self;

    fn read<'a>(&'a self) -> Self {
        *self
    }
}

/// A resource that holds a table, read in place.
#[derive(Copy, Clone)]
struct Table {
    marks: [u32; 2],
}

impl ResourceRead for Table {
    type Val<'a> = &'a Table;

    fn read<'a>(&'a self) -> &'a Table {
        self
    }
}

impl Replaceable for Table {}

// Neither is written by a unit, so neither takes any of a morsel's write
// budget.
impl ResourceFootprint for Step {
    const L1_BYTES: USize = USize(0);
}

impl ResourceFootprint for Table {
    const L1_BYTES: USize = USize(0);
}

type Hints = (Immediate, Atomic, Normal);

fn store() -> ArenaColumnStorage<StableProvider> {
    ArenaColumnStorage::new(StableProvider::new(1 << 16))
}

/// Fill `RECORDS` records at `base`.
///
/// # Safety
///
/// `base` is a column of `RECORDS` records and no pass is running.
unsafe fn fill<T: Copy>(base: *mut T, mut value: impl FnMut(usize) -> T) {
    for i in 0 .. RECORDS {
        // SAFETY: inside the reservation, per the contract above.
        unsafe { base.add(i).write(value(i)) };
    }
}

/// Read `RECORDS` records at `base` through `raw`.
///
/// # Safety
///
/// `base` is a column of `RECORDS` initialised records and no pass is running.
unsafe fn all<T: Copy>(base: *const T, raw: impl Fn(T) -> u32) -> Vec<u32> {
    // SAFETY: inside the reservation, per the contract above.
    (0 .. RECORDS)
        .map(|i| raw(unsafe { base.add(i).read() }))
        .collect()
}

// --- The value function over a column and two resources --------------------

type Inputs = Cons<Column<Slot>, Cons<Resource<Table>, Cons<Resource<Step>, Empty>>>;
type Heights = Cons<Column<Height>, Empty>;

#[derive(Copy, Clone)]
struct Survey;

impl BuilderInput for Survey {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Survey {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Inputs,
        Heights,
        SnapCons<Table, SnapCons<Step, SnapNil>>,
        ColPtrCons<Slot, ColPtrNil>,
        ColPtrCons<Height, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = Hints;
    type Read = Inputs;
    type Write = Heights;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let slot = read.get(Column::<Slot>::new()).0;
            let table: &Table = read.get(Resource::<Table>::default());
            let step: Step = read.get(Resource::<Step>::default());
            rec![Height(slot * step.0 + table.marks[slot as usize % 2])]
        });
    }
}

#[test]
fn a_value_function_reads_a_resource_by_value_and_one_in_place() {
    let mut scheduler = Scheduler::builder()
        .with(Column::<Slot>::start())
        .with(Column::<Height>::start())
        .with(Resource::new(Table {
            marks: [7, 11],
        }))
        .with(Resource::new(Step(3)))
        .with(Survey)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let heights = scheduler.__bindings().__tail().__tail().__ptr().as_ptr();
    let slots = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr();
    // SAFETY: before the first pass.
    unsafe { fill(slots, |i| Slot(i as u32)) };
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    // SAFETY: between passes.
    let got = unsafe { all(heights, |h: Height| h.0) };
    let want: Vec<u32> = (0 .. RECORDS as u32)
        .map(|s| s * 3 + [7, 11][s as usize % 2])
        .collect();
    assert_eq!(got, want, "each record from its slot and the two resources");

    // The table is read where it stands, so a pass after a swap reads the
    // new one.
    scheduler.replace_value(Table {
        marks: [100, 200],
    });
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    // SAFETY: between passes.
    let got = unsafe { all(heights, |h: Height| h.0) };
    let want: Vec<u32> = (0 .. RECORDS as u32)
        .map(|s| s * 3 + [100, 200][s as usize % 2])
        .collect();
    assert_eq!(got, want, "the pass after the swap read the new table");
}

// --- The positional write under each of the three members ------------------

type Slots = Cons<Column<Slot>, Empty>;
type Sights = Cons<Column<Seen>, Empty>;

// Lay: writes twice its slot at each record by position, once for each member
// the column may be listed under.
macro_rules! lay_unit {
    ($name:ident, $member:ty) => {
        #[derive(Copy, Clone)]
        struct $name;

        impl BuilderInput for $name {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }

        impl WorkUnit<Always> for $name {
            type Ctx<'frame> = EngineCtx<
                'frame,
                Slots,
                Cons<$member, Empty>,
                SnapNil,
                ColPtrCons<Slot, ColPtrNil>,
                ColPtrCons<Height, ColPtrNil>,
                AccPtrNil,
            >;
            type Hint = Hints;
            type Read = Slots;
            type Write = Cons<$member, Empty>;

            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    // SAFETY: the plan gives this unit the column's one
                    // writer slot, and `i` is inside the morsel.
                    unsafe {
                        let slot: Slot = ctx.read::<Slot, _>(i);
                        ctx.write::<Height, _>(i, Height(slot.0 * 2));
                    }
                });
            }
        }
    };
}
lay_unit!(LayBare, Column<Height>);
lay_unit!(LayWritten, Written<Column<Height>>);
lay_unit!(LayDiffers, Differs<Column<Height>>);

#[derive(Copy, Clone)]
struct Watch;

impl BuilderInput for Watch {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Watch {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Heights,
        Sights,
        SnapNil,
        ColPtrCons<Height, ColPtrNil>,
        ColPtrCons<Seen, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = Hints;
    type Read = Heights;
    type Write = Sights;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .map(|read| rec![Seen(read.get(Column::<Height>::new()).0)]);
    }
}

macro_rules! positional_write {
    ($test:ident, $lay:ident) => {
        #[test]
        fn $test() {
            let mut scheduler = Scheduler::builder()
                .with(Column::<Slot>::start())
                .with(Column::<Height>::start())
                .with(Column::<Seen>::start())
                .with($lay)
                .with(Watch)
                .build(store(), USize(RECORDS))
                .unwrap_or_else(|_| panic!("build should succeed"));
            let seen = scheduler.__bindings().__ptr().as_ptr();
            let heights = scheduler.__bindings().__tail().__ptr().as_ptr();
            let slots = scheduler.__bindings().__tail().__tail().__ptr().as_ptr();
            // SAFETY: before the first pass.
            unsafe {
                fill(slots, |i| Slot(i as u32));
                fill(heights, |_| Height(SENTINEL));
            }
            assert!(matches!(scheduler.run(), Outcome::Ok(())));
            let want: Vec<u32> = (0 .. RECORDS as u32).map(|s| s * 2).collect();
            // SAFETY: between passes.
            unsafe {
                assert_eq!(
                    all(heights, |h: Height| h.0),
                    want,
                    "every record was written"
                );
                assert_eq!(
                    all(seen, |s: Seen| s.0),
                    want,
                    "the reader followed in the pass"
                );
            }
            // No mark was left: the second pass finds nothing changed, and
            // neither the writer nor its reader is dispatched. The two
            // columns hold two values, so a reader that ran alone would show.
            // SAFETY: between passes.
            unsafe {
                fill(heights, |_| Height(5));
                fill(seen, |_| Seen(SENTINEL));
            }
            assert!(matches!(scheduler.run(), Outcome::Ok(())));
            // SAFETY: between passes.
            unsafe {
                assert!(
                    all(heights, |h: Height| h.0).iter().all(|v| *v == 5),
                    "the writer rested"
                );
                assert!(
                    all(seen, |s: Seen| s.0).iter().all(|v| *v == SENTINEL),
                    "the reader rested",
                );
            }
        }
    };
}
positional_write!(a_positional_write_to_a_column_listed_bare, LayBare);
positional_write!(a_positional_write_to_a_column_under_written, LayWritten);
positional_write!(a_positional_write_to_a_column_under_differs, LayDiffers);
