# review 1 of work/api-revision at eeb7f555: do not merge

Reviewer: pr-reviewer-senior, fresh. Canon gate passed (no canon_paths, the locked doc changelist and crate designs stand in). Suites green: hilavitkutin-api, hilavitkutin (80 suites), -kit, -api-macros. `hilavitkutin-providers --test smoke` fails to link (`undefined symbol: __start_hilavitkutin_strings`), not touched by the branch, not checked on dev: check it on dev and ticket it.

## Findings

1. `mock/crates/hilavitkutin/src/scheduler/worker.rs:199-246` with `run_parallel.rs:471` (correctness, unsafe): the reach phase cut is ignored on the unit-outer parallel path. Once a carrier holds any accumulator, every worker runs the whole carrier over its own record slice with no barrier between units, so a `Reach<Column<T>, N>` reader reads a neighbour record another core's writer is still writing: a data race, against doc changelist 5 ("every writer's whole range has landed on every core before the reader starts"). `compute_phases_waist` (`plan/grouping.rs:168`) protects only the phase-loop path. Reproduced 3 of 3: 20000 records, `FixedWorkers::<4>`, `Raise` and `Spread` from `buffered_column.rs` plus one accumulator unit appending nothing; control without the accumulator passes. Probe source below (probe 1); it belongs in `tests/buffered_column.rs`. Fix: refuse a plain-column reach in a unit-outer carrier at build, or a cross-core barrier ahead of the reader on that path.

2. `resource/bindings/column.rs:399-408` with `dispatch/engine_ctx/map.rs:59-62` (correctness, unsafe, spec): pass 1 reads uninitialised memory through a safe API. `drain_column` reserves both sides and initialises neither; in pass 1 every `map` read of a `Buffered` goes to the frozen side, which nothing has written. No supported API seeds it between `build` and the first `run`; all 29 tests in `buffered_column.rs` go in through `#[doc(hidden)] __frozen()`, and `a_store_filled_in_the_first_pass_is_read_from_the_second` fills it with 0 at `:496` first. The design (api `DESIGN.md.tmpl:648-657`) does not say what pass 1 reads. Not yours to settle in code: the coordinator is getting a provisional stamp on it and will DM you the shape. Do not build a fix for 2 until then.

3. `dispatch/engine_ctx/map.rs:187-195` (tests): the buffered write has a surviving mutant. Writing the live side only where the new value differs from the frozen one leaves `buffered_column`, `maintenance_units` and `unit_bodies` green. The catching test: the value differs at even records and equals the frozen side at odd ones, frozen side checked after one pass. Probe source below (probe 2). Every test now writes the same value at every record.

4. `scheduler/reach_rule.rs:141-145` (drift): `HOLDS` folds masks at `DefaultPlanDims`, not at the run's own `Cfg` dimensions. A schedule wider than the default could slip past the rule. Not reproduced.

5. `resource/bindings/column.rs:264-317` (unsafe): `Flip::turned` is a `Cell` read by workers and written by the main thread at `bring_in`. Sound today by `frame_publish` / `frame_await_done` order and per-frame rebuilt per-core copies (`worker.rs:199`), but no `// SAFETY:` comment names that order at the type.

## Verified by the reviewer

Buffered swap order, the read-after-write wait leaving buffered columns out, the phase cut on the phase-loop path, the build refusal of a unit writing the column it reaches, the two mutants you claimed, trybuild refusals of positional read and write of a `Buffered`, commits and readme.

## Probe 1: probe_reach_unit_outer.rs

```rust
//! Reviewer probe: a reach read of a plain column behind its writer, in a
//! carrier that also has an accumulator (so `run_parallel` takes the
//! unit-outer per-core path).
mod common;
mod stable_provider;

use arvo::{Cap, USize};
use common::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, ColumnReaderApi, EachApi, HasAccumWriter, HasColumnReader, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Reach, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::{Maybe, Outcome};
use stable_provider::StableProvider;

const ONE: Cap = arvo_tensor::cap(1);
const RECORDS: usize = 20_000;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Height(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Seen(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Av(u32);

type Col = Column<Height>;
type Cols = ColPtrCons<Height, ColPtrNil>;
type Bare = Cons<Col, Empty>;
type Out = Cons<Column<Seen>, Empty>;
type OutCols = ColPtrCons<Seen, ColPtrNil>;
type Around = Cons<Reach<Col, ONE>, Empty>;

macro_rules! unit {
    ($name:ident, read $read:ty as $rcols:ty, write $write:ty as $wcols:ty, | $r:ident | $body:expr) => {
        #[derive(Copy, Clone)]
        struct $name;
        impl BuilderInput for $name { type Dispatch = UnitDispatch<Self>; type Init = Self; }
        impl WorkUnit<Always> for $name {
            type Ctx<'frame> = EngineCtx<'frame, $read, $write, SnapNil, $rcols, $wcols, AccPtrNil>;
            type Hint = (Immediate, Atomic, Normal);
            type Read = $read;
            type Write = $write;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) { ctx.each().map(|$r| $body); }
        }
    };
}

fn part(side: Maybe<Height>) -> u32 { match side { Maybe::Is(h) => h.0, Maybe::Isnt => 0 } }

unit!(Raise, read Bare as Cols, write Bare as Cols, |read| {
    let v = read.get(Col::new()).0;
    if v == 4999 { std::thread::sleep(std::time::Duration::from_millis(200)); }
    rec![Height(v + 1)]
});
unit!(Spread, read Around as Cols, write Out as OutCols, |read| {
    let w = read.get(Reach::<Col, ONE>::new());
    rec![Seen(w.here().0.wrapping_add(part(w.before::<ONE>())).wrapping_add(part(w.after::<ONE>())))]
});

// Appends nothing; its only role is to make the carrier accumulator-bearing.
#[derive(Copy, Clone)]
struct Note;
impl BuilderInput for Note { type Dispatch = UnitDispatch<Self>; type Init = Self; }
type SeenIn = Cons<Column<Seen>, Empty>;
type AccW = Cons<Accum<Av>, Empty>;
impl WorkUnit<Always> for Note {
    type Ctx<'frame> = EngineCtx<'frame, SeenIn, AccW, SnapNil, OutCols, ColPtrNil, AccPtrCons<'frame, Av, AccPtrNil>>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = SeenIn;
    type Write = AccW;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<Seen, _>(i) };
            if v.0 == u32::MAX { ctx.accums().append::<Av, _>(Av(v.0)); }
        });
    }
}

#[test]
fn reach_behind_writer_on_the_unit_outer_parallel_path() {
    let scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(Col::new())
        .with(Column::<Seen>::new())
        .with(Raise)
        .with(Spread)
        .with(Note)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let pool = FixedWorkers::<4>;
    let mut scheduler = core::pin::pin!(scheduler);
    let b = scheduler.as_ref().get_ref().__bindings().__tail();
    let col = b.__ptr().as_ptr();
    let seen = scheduler.as_ref().get_ref().__bindings().__ptr().as_ptr();
    for i in 0..RECORDS { unsafe { col.add(i).write(Height(i as u32)) }; }
    assert!(matches!(scheduler.as_mut().run_parallel(&pool), Outcome::Ok(())));
    let raised: Vec<u32> = (0..RECORDS as u32).map(|i| i + 1).collect();
    let mut bad = Vec::new();
    for i in 0..RECORDS {
        let before = if i > 0 { raised[i - 1] } else { 0 };
        let after = if i + 1 < RECORDS { raised[i + 1] } else { 0 };
        let want = raised[i] + before + after;
        let got = unsafe { seen.add(i).read() }.0;
        if got != want { bad.push((i, got, want)); }
    }
    assert!(bad.is_empty(), "stale neighbour reads: {:?}", &bad[..bad.len().min(8)]);
}
fn store() -> ArenaColumnStorage<StableProvider> { ArenaColumnStorage::new(StableProvider::new(1 << 19)) }
```

## Probe 2: probe_partial_equal.rs

```rust
//! Reviewer probe: a writer whose values differ from the frozen side at some
//! records and equal it at others.
mod common;
mod stable_provider;
use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Buffered, Exchange, Snapshot};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::rec;
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;
const RECORDS: usize = 20_000;
#[derive(Copy, Clone, PartialEq, Debug)]
struct Height(u32);
fn store() -> ArenaColumnStorage<StableProvider> { ArenaColumnStorage::new(StableProvider::new(1 << 19)) }
macro_rules! suite { ($m:ident, $mode:ty) => { mod $m { use super::*;
    type Col = Buffered<Height, $mode>;
    type Cols = ColPtrCons<Height, ColPtrNil, $mode>;
    type Bare = Cons<Col, Empty>;
    #[derive(Copy, Clone)] struct Half;
    impl BuilderInput for Half { type Dispatch = UnitDispatch<Self>; type Init = Self; }
    impl WorkUnit<Always> for Half {
        type Ctx<'frame> = EngineCtx<'frame, Bare, Bare, SnapNil, Cols, Cols, AccPtrNil>;
        type Hint = (Immediate, Atomic, Normal);
        type Read = Bare; type Write = Bare;
        fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
            ctx.each().map(|r| { let v = r.get(Col::new()).0; rec![Height(if v % 2 == 0 { v + 1 } else { v })] });
        }
    }
    #[test] fn equal_records_still_land_on_the_live_side() {
        let mut s = Scheduler::builder().with(Col::new()).with(Half).build(store(), USize(RECORDS)).unwrap_or_else(|_| panic!("build"));
        let f = s.__bindings().__frozen().as_ptr(); let l = s.__bindings().__live().as_ptr();
        for i in 0..RECORDS { unsafe { f.add(i).write(Height(i as u32)); l.add(i).write(Height(0xDEAD)); } }
        assert!(matches!(s.run(), Outcome::Ok(())));
        let f = s.__bindings().__frozen().as_ptr();
        for i in 0..RECORDS { let want = if i % 2 == 0 { i as u32 + 1 } else { i as u32 }; let got = unsafe { f.add(i).read() }.0; assert_eq!(got, want, "record {i}"); }
    }
}}}
suite!(snapshot, Snapshot);
suite!(exchange, Exchange);
```
