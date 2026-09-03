//! Swap-semantics tests (spec S6, round 202607200500).
//!
//! `replace_value` installs the whole value as one blob write through the
//! `Selector` witness (spec S1); `replace_resource` performs the identical
//! install plus the plan-dirty seed, and the next frame enters the leading
//! plan band (spec S2). A plain value swap never forces the band, which is
//! the intended cost asymmetry between a dirty cone and a plan recompute.
//!
//! Harness mirrors tests/incremental_skip.rs: a stack-backed bump provider
//! behind `ArenaColumnStorage`, one column work unit so `run` dispatches,
//! and engine internals reached through the hidden `__` accessors.
//!
//! Lives under `tests/` so the bare numeric payloads do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/incremental_skip.rs).
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

const RECORDS: usize = 4;

// Swappable scalar resource (Replaceable, cheap value swap).
#[derive(Copy, Clone, Debug, PartialEq)]
struct Val(u32);
impl hilavitkutin_api::store::Replaceable for Val {}
impl hilavitkutin_api::footprint::ResourceFootprint for Val {
    const L1_BYTES: USize = USize(0);
}

// Plan-affecting resource (routes through replace_resource).
#[derive(Copy, Clone, Debug, PartialEq)]
struct Cfgish(u32);
impl hilavitkutin_api::run_cfg::PlanAffecting for Cfgish {}
impl hilavitkutin_api::footprint::ResourceFootprint for Cfgish {
    const L1_BYTES: USize = USize(0);
}

// Zero-sized swappable resource: the install touches no memory but the
// dirty semantics are identical.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Zst;
impl hilavitkutin_api::store::Replaceable for Zst {}
impl hilavitkutin_api::footprint::ResourceFootprint for Zst {
    const L1_BYTES: USize = USize(0);
}

// One column unit so the scheduler has a dispatchable carrier; it reads
// nothing and writes its own column, unrelated to the swapped resources
// (the swap-of-an-unread-resource case rides every test below).
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Out(u32);

type WriteOut = Cons<Column<Out>, Empty>;

struct FillWu;

impl BuilderInput for FillWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for FillWu {
    type Read = Empty;
    type Write = WriteOut;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, WriteOut, SnapNil, ColPtrNil, ColPtrCons<Out, ColPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Out>` buffer for the
            // record count and the plan proved this unit the exclusive
            // writer.
            unsafe { ctx.writer().write::<Out, _>(i, Out(i.0 as u32)) };
        });
    }
}

/// S1: the swap is one blob write into the SAME slot the drain filled.
///
/// The binding pointer before and after the swap is the same address, and
/// the slot reads back the new value byte-exactly. A copy-elsewhere
/// install (or no install) fails the address or value assertion.
#[test]
fn swap_installs_in_place_at_the_drained_slot() {
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Val(11)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let before_ptr = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: written with Val(11) at build; the scheduler is alive.
    assert_eq!(unsafe { *before_ptr }, Val(11));

    scheduler.replace_value(Val(42));

    let after_ptr = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(
        before_ptr, after_ptr,
        "the swap must write through the drained slot, not relocate it"
    );
    // SAFETY: same pointer, still owned by the live scheduler.
    assert_eq!(
        unsafe { *after_ptr },
        Val(42),
        "the new value must be installed"
    );
}

/// S1: two swaps before a frame; the last value wins.
#[test]
fn double_swap_before_run_last_value_wins() {
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Val(1)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    scheduler.replace_value(Val(2));
    scheduler.replace_value(Val(3));

    // SAFETY: the binding names the live one-record column.
    let now = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(now, Val(3), "the last swap before the frame must win");
}

/// S1: a ZST swap is a no-op install that still goes through the path.
#[test]
fn zst_swap_touches_no_memory_and_succeeds() {
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Zst))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    scheduler.replace_value(Zst);

    // SAFETY: a ZST read through the dangling-but-aligned binding pointer
    // touches no memory (mirrors the drain's ZST arm).
    let now = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(now, Zst);
}

/// S2: a plan-affecting swap opens the plan band on a non-first frame; a
/// plain value swap (or an unmarked frame) does not.
///
/// The published band decision is observed through `__plan_band`: true
/// after the cold first frame, false after a clean frame, false after a
/// frame that only saw `replace_value`, true again after a frame that
/// consumed a `replace_resource` plan-dirty bit.
#[test]
fn replace_resource_opens_the_plan_band_replace_value_does_not() {
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Val(5)))
        .with(Resource::new(Cfgish(1)))
        .with(Column::<Out>::new())
        .with(FillWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Cold first frame: always plan-dirty.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        scheduler.__plan_band().0,
        "the first frame always enters the band"
    );

    // Clean frame: no swap, band closed.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        !scheduler.__plan_band().0,
        "a clean frame must not enter the band"
    );

    // Value swap only: dirty cone, no plan recompute.
    scheduler.replace_value(Val(6));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        !scheduler.__plan_band().0,
        "a plain Replaceable swap must not force the plan band"
    );

    // Plan-affecting swap: the consumed bit opens the band once.
    scheduler.replace_resource(Cfgish(2));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        scheduler.__plan_band().0,
        "a replace_resource swap must open the plan band on the next frame"
    );

    // The bit was consumed: the following clean frame closes the band again.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        !scheduler.__plan_band().0,
        "one swap buys one plan band; the consumed bit must not persist"
    );
}

/// S2: the plan-affecting swap installs the value, same as S1.
#[test]
fn replace_resource_installs_the_value() {
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Cfgish(7)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    scheduler.replace_resource(Cfgish(9));

    // SAFETY: the binding names the live one-record column.
    let now = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(
        now,
        Cfgish(9),
        "replace_resource must install, not only mark"
    );
}

/// Catalogue (#689, GATE-2 deviation 5): the intended driver pattern
/// `run_parallel(..); replace_value(..); run_parallel(..)` materialises
/// a `&mut Scheduler` between frames while a spawned worker holds a
/// live `&Scheduler` across its park, an aliasing-model violation the
/// 2026-07-19 audit confirmed (miscompilation-class, not a data race;
/// the frame protocol's Release/Acquire pairs are sound). Not
/// observable natively, so this entry documents the pattern and the
/// resolution instead of failing: the scheduler-plane relocation round
/// (deviations 1, 5, and 6 together) moves every worker-visible byte
/// into a provider allocation, after which this becomes a Miri-gated
/// check and unignores.
#[test]
#[ignore = "catalogue: between-frames &mut Scheduler aliases a parked worker's held &Scheduler; soundness fix is the plane relocation round; tracked #689"]
fn parallel_swap_pattern_is_aliasing_clean() {
    // Intended assertion once the plane relocation lands: this exact
    // pattern runs under Miri without an aliasing report. Natively it
    // runs regardless, which is why the entry stays ignored until the
    // Miri gate exists.
}
