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

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::{MemoryProviderApi, Nanos};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Outv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColOut = Cons<Column<Outv>, Empty>;

// A trivial one-WU carrier. select_adapt_config reads phase_ema directly (set via
// the white-box accessor), so the carrier shape is irrelevant; it just gives a
// built scheduler to call the accessors on.
struct Copyer;
impl BuilderInput for Copyer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Copyer {
    type Read = OneIn;
    type Write = ColOut;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColOut, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Outv, ColPtrNil>>;
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
        let provider = BumpProvider::<16384>::new();
        Scheduler::builder()
            .with(Column::<Inv>::new())
            .with(Column::<Outv>::new())
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
    assert!(s.__adapt_reconfigure().0, "1000 > 2 * 100: phases imbalanced, reconfigure set");
}

#[test]
fn balanced_clears_reconfigure() {
    let s = build!();
    s.__set_phase_ema(USize(0), Nanos::from_raw(100));
    s.__set_phase_ema(USize(1), Nanos::from_raw(100));
    s.__select_adapt_config();
    assert!(!s.__adapt_reconfigure().0, "100 not > 2 * 100: balanced, reconfigure clear");
}

#[test]
fn single_active_phase_clears() {
    let s = build!();
    s.__set_phase_ema(USize(0), Nanos::from_raw(1000));
    // phase 1 left at zero: only one active phase, no imbalance possible.
    s.__select_adapt_config();
    assert!(!s.__adapt_reconfigure().0, "one active phase: no imbalance, reconfigure clear");
}

#[test]
fn no_active_phases_clears() {
    let s = build!();
    // all phase_ema slots zero (a fresh scheduler / parallel-or-fused path).
    s.__select_adapt_config();
    assert!(!s.__adapt_reconfigure().0, "no active phases: reconfigure clear");
}
