//! Two-frame append through the single-core entry and the fused entry.
//!
//! One unit maps a record column (`RecordOp`) and also appends one value per
//! record to an accumulator. `run` executes the unit's body, so each frame
//! leaves one appended value per record in the accumulator. `run_fused` folds
//! the carrier into a chain of `RecordOp` maps and runs only the maps: the
//! unit's body, with its appends, never runs, and nothing refuses the carrier
//! at compile time because `FuseCarrier` bounds a unit on `RecordOp` and `Copy`
//! and not on its write set. `run_fused` makes the same per-frame accumulator
//! reset as `run` and `run_parallel`, so a refusal from an earlier `run` frame
//! does not carry into a fused frame: the third test refuses through `run` and
//! then expects `Ok` from `run_fused`.
//!
//! The first test holds the behaviour the second is compared against, so the
//! second cannot fail for the harness's sake. The second asserts the same live
//! length through `run_fused` and fails today; it is ignored with ticket
//! `0fb716c5a463`, which owns making the fused entry refuse the carrier or run
//! the body and reset. Drop the `#[ignore]` when that ticket closes (a refusal turns this
//! into a compile-fail pair).
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::RecordOp;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Records per frame, and the accumulator's reserved capacity (the record
// count, which is what `build` reserves).
const RECORDS: usize = 2;

#[derive(Copy, Clone)]
struct InV(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct OutV(u32);
#[derive(Copy, Clone)]
struct Log(u32);

type ReadIn = Cons<Column<InV>, Empty>;
type WriteBoth = Cons<Column<OutV>, Cons<Accum<Log>, Empty>>;

// Maps `InV` to `OutV` and appends one `Log` per record.
#[derive(Copy, Clone)]
struct MapAndLogWu;

impl BuilderInput for MapAndLogWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapAndLogWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadIn,
        WriteBoth,
        SnapNil,
        ColPtrCons<InV, ColPtrNil>,
        ColPtrCons<OutV, ColPtrNil>,
        AccPtrCons<'frame, Log, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadIn;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `InV` is reserved for the record range and read only
            // here; `OutV` is this unit's exclusive write column; the unit holds
            // the exclusive appender slot for `Log`, and the accumulator is
            // reserved for the record count, one append per record.
            let v = unsafe { ctx.reader().read::<InV, _>(i) };
            unsafe { ctx.writer().write::<OutV, _>(i, OutV(v.0 + 1)) };
            unsafe { ctx.accums().append::<Log, _>(Log(i.0 as u32)) };
        });
    }
}

impl RecordOp for MapAndLogWu {
    type In = InV;
    type Out = OutV;

    fn apply(&self, x: InV) -> OutV {
        OutV(x.0 + 1)
    }
}

// Maps `InV` to `OutV` like `MapAndLogWu` and appends two `Log`s per record, so
// a frame asks twice the reserved capacity and `run` refuses.
#[derive(Copy, Clone)]
struct MapAndLogTwiceWu;

impl BuilderInput for MapAndLogTwiceWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapAndLogTwiceWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadIn,
        WriteBoth,
        SnapNil,
        ColPtrCons<InV, ColPtrNil>,
        ColPtrCons<OutV, ColPtrNil>,
        AccPtrCons<'frame, Log, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadIn;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as in `MapAndLogWu`; the second append per record goes
            // past the reserved capacity and is refused, writing nothing.
            let v = unsafe { ctx.reader().read::<InV, _>(i) };
            unsafe { ctx.writer().write::<OutV, _>(i, OutV(v.0 + 1)) };
            unsafe { ctx.accums().append::<Log, _>(Log(i.0 as u32)) };
            unsafe { ctx.accums().append::<Log, _>(Log(i.0 as u32)) };
        });
    }
}

impl RecordOp for MapAndLogTwiceWu {
    type In = InV;
    type Out = OutV;

    fn apply(&self, x: InV) -> OutV {
        OutV(x.0 + 1)
    }
}

macro_rules! scheduler {
    () => {
        scheduler!(MapAndLogWu)
    };
    ($wu:expr) => {{
        let provider = StableProvider::new(16384);
        // `Log` is registered last, so its accumulator is the bindings head.
        Scheduler::builder()
            .with(Column::<InV>::new())
            .with(Column::<OutV>::new())
            .with(Accum::<Log>::new())
            .with($wu)
            .build(store(provider), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"))
    }};
}

// A frame through `run` that refuses, then a frame through `run_fused`, which
// runs only the map and appends nothing: the fused frame resets the
// accumulator and returns its own verdict, `Ok`.
#[test]
fn run_fused_after_a_refusing_run_returns_ok() {
    let mut scheduler = scheduler!(MapAndLogTwiceWu);
    assert!(
        matches!(scheduler.run(), Outcome::Err(_)),
        "two appends per record into a record-count reservation are refused",
    );
    assert!(
        matches!(scheduler.run_fused(), Outcome::Ok(())),
        "the fused frame appended nothing, so it reports no refusal",
    );
    assert_eq!(
        scheduler.__bindings().__len_cell().get(),
        USize(0),
        "the fused frame started from a reset accumulator",
    );
}

#[test]
fn run_appends_one_value_per_record_each_frame() {
    let mut scheduler = scheduler!();
    for frame in 0 .. 2 {
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        assert_eq!(
            scheduler.__bindings().__len_cell().get(),
            USize(RECORDS),
            "frame {frame}: one append per record through `run`",
        );
        let base = scheduler.__bindings().__ptr().as_ptr();
        // SAFETY: `RECORDS` values were appended this frame into a buffer reset
        // at frame start; records `[0, RECORDS)` are initialised.
        let logged =
            [unsafe { core::ptr::read(base.add(0)) }, unsafe { core::ptr::read(base.add(1)) }];
        assert_eq!(
            [logged[0].0, logged[1].0],
            [0u32, 1u32],
            "frame {frame}: the appended values land in record order",
        );
    }
}

#[test]
#[ignore = "catalogue: run_fused runs only the map and never the body of a RecordOp unit that also appends; needs the fused entry to refuse the carrier, or to run the body; tracked #run_fused_does_not_refuse_a_recordop_unit_that_writes_an_acc"]
fn run_fused_appends_one_value_per_record_each_frame() {
    let mut scheduler = scheduler!();
    for frame in 0 .. 2 {
        assert!(matches!(scheduler.run_fused(), Outcome::Ok(())));
        assert_eq!(
            scheduler.__bindings().__len_cell().get(),
            USize(RECORDS),
            "frame {frame}: one append per record through `run_fused`, as through `run`",
        );
    }
}
