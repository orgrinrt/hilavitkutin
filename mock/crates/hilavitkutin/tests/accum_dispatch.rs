//! Accumulator append round-trip integration test (column data plane, round 2).
//!
//! A WorkUnit declares `Accum<Av>` in its Write set and appends three values
//! during `execute`. The scheduler builds with a record count covering the
//! appends, runs, and the test reads the accumulator buffer back through the
//! binding's hidden accessors to assert the three values landed at `[0, 3)` in
//! append order and the live-length advanced to 3. This pins the round-2
//! contract: an `Accum<T>` reserves a real buffer (sized by the build-time
//! record count), the append accessor writes at the live offset and advances a
//! `Cell` live-length under `&self`, and the accumulator bundle dispatches
//! through `run`.
//!
//! Red first: before the `Accum<T>` marker, the `AccumBinding` drain arm, the
//! `AccumSelector` / `AccumProject` projection, the `AccumWriterApi` accessor,
//! and the lifted `RunFiber` bound exist, an appending WorkUnit cannot
//! satisfy the dispatch bound and the file does not compile; once the bundle is
//! projected but before the drain reserves a real buffer, the append writes
//! through a dangling placeholder. Both precede the green round-trip.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::AccumRefused;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Accum;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use notko::lend::Exhausted;
use stable_provider::StableProvider;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// The capacity the accumulator reserves (>= the three appends). The live
// length grows from zero as the unit appends, never reaching the capacity, so
// the unwritten tail records stay reserved-but-unread.
const CAP: usize = 4;

// Accumulator value: a Copy newtype over u32, so the blanket `ColumnValue`
// applies.
#[derive(Copy, Clone)]
struct Av(u32);

type AccW = Cons<Accum<Av>, Empty>;

// Appender: writes nothing, appends three values to `Accum<Av>`. The appends
// are fixed (not driven by the morsel) so the live length and append order are
// directly assertable.
struct AppendWu;

impl BuilderInput for AppendWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for AppendWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccW,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // The drain initialised the live length to zero before the frame.
        assert_eq!(
            ctx.accums().len::<Av, _>().0,
            0,
            "live length starts at zero"
        );
        // `build` reserved the `Accum<Av>` buffer for the record count
        // (>= 3) and the plan proved this unit the exclusive appender; the
        // three appends stay within the reserved capacity. Each append advances
        // the live length, read back through the `len` accessor.
        ctx.accums().append::<Av, _>(Av(100));
        assert_eq!(ctx.accums().len::<Av, _>().0, 1, "len advances per append");
        ctx.accums().append::<Av, _>(Av(101));
        ctx.accums().append::<Av, _>(Av(102));
        assert_eq!(
            ctx.accums().len::<Av, _>().0,
            3,
            "len reflects all three appends"
        );
    }
}

// The small capacity the over-capacity test reserves. The appending unit issues
// more appends than this, so the append at index `CAP_SMALL` violates the
// reserved-capacity contract.
const CAP_SMALL: usize = 2;

// Over-capacity appender: appends five values to an `Accum<Av>` reserved to
// `CAP_SMALL`. Appending past the reserved capacity is refused: the first
// `CAP_SMALL` appends land, the rest write nothing, and the run returns the
// refusal.
struct OverAppendWu;

impl BuilderInput for OverAppendWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for OverAppendWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccW,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // `build` reserved `CAP_SMALL` records; the appender holds the
        // exclusive slot. Appending past the reserved capacity is refused and
        // writes nothing, so the appends from index `CAP_SMALL` on never reach
        // the buffer.
        ctx.accums().append::<Av, _>(Av(200));
        ctx.accums().append::<Av, _>(Av(201));
        ctx.accums().append::<Av, _>(Av(202));
        ctx.accums().append::<Av, _>(Av(203));
        ctx.accums().append::<Av, _>(Av(204));
    }
}

// Appending past the reserved capacity is refused, never a silent drop and never
// a panic: `OverAppendWu` appends `CAP_SMALL + 3` times into a `CAP_SMALL`
// accumulator, the first `CAP_SMALL` land, the rest write nothing, and the run
// returns the refusal with every append it was asked for.
#[test]
fn accum_append_past_reserved_capacity_is_refused_as_a_value() {
    let provider = StableProvider::new(8192, 0);
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(OverAppendWu)
        .build(store(provider), USize(CAP_SMALL))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    let expected = AccumRefused::new(USize(0), Exhausted {
        wanted: CAP_SMALL + 3,
        had:    CAP_SMALL,
    });
    match result {
        Outcome::Err(got) => assert_eq!(got, expected, "the refusal is the run's output"),
        Outcome::Ok(()) => panic!("the run returned Ok past the reserved capacity"),
    }

    let bindings = scheduler.__bindings();
    assert_eq!(
        bindings.__len_cell().get(),
        USize(CAP_SMALL),
        "only the first CAP_SMALL appends landed"
    );
    let base = bindings.__ptr().as_ptr();
    // SAFETY: the drain reserved `CAP_SMALL` records and both were appended.
    let landed = unsafe { [core::ptr::read(base.add(0)), core::ptr::read(base.add(1))] };
    assert_eq!(
        landed.map(|a| a.0),
        [200, 201],
        "the accepted records are the first appends, in order"
    );
}

#[test]
fn accum_appends_land_in_order_with_live_length_through_run() {
    let provider = StableProvider::new(8192, 0);
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(AppendWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    // The single registered store value is the accumulator, so the bindings
    // head is the `AccumBinding<Av, _>`. Read its live length and buffer prefix
    // back through the hidden accessors.
    let bindings = scheduler.__bindings();
    assert_eq!(
        bindings.__len_cell().get(),
        USize(3),
        "three appends advanced the live length to 3"
    );

    let base = bindings.__ptr().as_ptr();
    // SAFETY: the drain reserved `CAP` records for `Av` and the unit appended
    // three; records `[0, 3)` are initialised.
    let v0 = unsafe { core::ptr::read(base.add(0)) };
    let v1 = unsafe { core::ptr::read(base.add(1)) };
    let v2 = unsafe { core::ptr::read(base.add(2)) };
    assert_eq!(
        [v0.0, v1.0, v2.0],
        [100u32, 101u32, 102u32],
        "the three appended values landed at indices [0, 3) in append order"
    );
}
