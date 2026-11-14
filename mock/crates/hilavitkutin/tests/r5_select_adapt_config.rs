//! E8 adapt R5: `select_adapt_config` phase-imbalance decision.
//!
//! The decision is pure logic over the engine-internal `phase_ema`: it sets the
//! `adapt_reconfigure` trigger when there are at least two active phases (nonzero
//! EMA) and the max active EMA exceeds BALANCE_FACTOR (2) times the min. These
//! tests drive it deterministically through the white-box accessors
//! (`__set_phase_ema` + `__select_adapt_config` + `__adapt_reconfigure`), so they
//! do not depend on wall-clock timing. The end-to-end "adaptation improves an
//! imbalanced workload" property is the catalogued contract in
//! `adapt_perf_contracts.rs`, red until the actuation lands; this round ships the
//! decision that sets the trigger.
//!
//! Lives under `tests/` so the bare numeric EMA values do not trip the src-tree
//! primitive lints.

mod stable_provider;

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
use hilavitkutin_api::platform::{MemoryProviderApi, Nanos};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use stable_provider::stable_provider;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

#[derive(Copy, Clone, Default)]
struct Inv(u32);
#[derive(Copy, Clone, Default)]
struct Outv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColOut = Cons<Column<Outv>, Empty>;

// A trivial one-WU carrier. select_adapt_config reads phase_ema directly (set via
// the white-box accessor), so the carrier shape is irrelevant; it just gives a
// built scheduler to call the accessors on.
struct Copyer;
impl BuilderInput for Copyer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Copyer {
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
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Outv reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Outv, _>(i, Outv(inp.0)) };
        });
    }
}

macro_rules! build {
    () => {{
        let provider = stable_provider(16384, 0);
        Scheduler::builder()
            .with(Column::<Inv>::start())
            .with(Column::<Outv>::start())
            .with(Copyer)
            .build(store(provider), USize(4))
            .unwrap_or_else(|_| panic!("build should succeed"))
    }};
}

#[test]
fn imbalanced_sets_reconfigure() {
    let s = build!();
    s.__set_phase_ema(USize(0), Nanos::from_raw(1000));
    s.__set_phase_ema(USize(1), Nanos::from_raw(100));
    s.__select_adapt_config();
    assert!(
        s.__adapt_reconfigure().0,
        "1000 > 2 * 100: phases imbalanced, reconfigure set"
    );
}

#[test]
fn balanced_clears_reconfigure() {
    let s = build!();
    s.__set_phase_ema(USize(0), Nanos::from_raw(100));
    s.__set_phase_ema(USize(1), Nanos::from_raw(100));
    s.__select_adapt_config();
    assert!(
        !s.__adapt_reconfigure().0,
        "100 not > 2 * 100: balanced, reconfigure clear"
    );
}

#[test]
fn single_active_phase_clears() {
    let s = build!();
    s.__set_phase_ema(USize(0), Nanos::from_raw(1000));
    // phase 1 left at zero: only one active phase, no imbalance possible.
    s.__select_adapt_config();
    assert!(
        !s.__adapt_reconfigure().0,
        "one active phase: no imbalance, reconfigure clear"
    );
}

#[test]
fn no_active_phases_clears() {
    let s = build!();
    // all phase_ema slots zero (a fresh scheduler / parallel-or-fused path).
    s.__select_adapt_config();
    assert!(
        !s.__adapt_reconfigure().0,
        "no active phases: reconfigure clear"
    );
}
