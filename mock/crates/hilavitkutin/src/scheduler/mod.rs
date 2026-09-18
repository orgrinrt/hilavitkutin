//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals>` carries a
//! phantom-tuple type-state plus two real value fields: the
//! `store_values` list and the `wu_values` list. `Wus` accumulates
//! registered WU types (cons-list). `Stores` accumulates registered
//! `Resource<T>` / `Column<T>` / `Virtual<T>` markers (cons-list).
//! `Platform` accumulates platform-provider types. `Vals` retains the
//! registered store VALUES (the `Resource<T>` carrier, the `Column<T>`
//! / `Virtual<T>` markers) in `Stores`-aligned order so the bindings drain
//! can move them into scheduler-owned storage at `build()`. `WuVals`
//! retains the registered WorkUnit instances so `build()` can carry
//! them into the `Scheduler`, where `run()` walks them.
//!
//! `.build(memory_provider)` carries `Stores: ContainsAll<Wus::AccumRead>
//! + ContainsAll<Wus::AccumWrite>`, which proves at compile time that
//! every registered WU's `Read` and `Write` membership is satisfied by
//! the registered stores. It walks `Stores` and `store_values` in
//! lockstep, allocating each `Resource<T>`'s block via the supplied
//! `MemoryProviderApi` and recording its `ResourcePtr<T>` in the bindings.
//!
//! Round 4 reshape: dropped `MAX_UNITS` / `MAX_STORES` / `MAX_LANES`
//! const generics. `Scheduler::replace_resource::<T>` lands with a
//! `T: Replaceable` bound.
//!
//! Round 202605091700 reshape: the nine `.add_*` and `.with_*` methods
//! retire in favour of one unified verb, `.with(value)`. Every value
//! passed to `.with` impls the sealed `BuilderInput` trait from
//! `hilavitkutin-api`; the per-kind typestate update flows through
//! `BuilderInput::Dispatch`.
//!
//! Round 202605290018 (B2a): store values route onto a
//! `Stores`-aligned `StoreValues` list under the single `.with` verb
//! via the `RouterKind` tag plus the `Place<P>` view. `Scheduler`
//! gains `<Stores, M>` parameters, an owned resource bindings, and a
//! `Drop` that deallocates it. `build` takes the `MemoryProvider` as
//! an argument and returns `Outcome<_, BuildError>`.
//!
//! Round (file-size lint): split into a module directory. `mod.rs` keeps
//! the `Scheduler` struct itself, `WorkerCtx` / `SendCtxPtr` /
//! `empty_pool_frame` (needed across nearly every sibling file) and the
//! `Drop` impl; everything else moved into sibling files declared below,
//! each re-exported so every public path is unchanged. `Scheduler`'s
//! fields stay private (unchanged): a sibling module of `scheduler` is a
//! descendant of it, and `Scheduler` is defined directly in `scheduler`
//! (this file), so every sibling reaches its private fields without any
//! visibility widening. Where a moved free function or method is called
//! from a sibling file rather than only from its own, it is marked
//! `pub(super)` at the definition site, noted in that file's header
//! comment.

use core::cell::Cell;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize};

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::platform::{Nanos, PoolFrame};
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store_values::{StoreValues, SvEmpty};
use hilavitkutin_api::work_unit_values::WuNil;

use crate::meta::MetaBlock;
use crate::plan::grouping::GATE2_MAX_UNITS;
use crate::plan::{AccessMask, DefaultPlanDims, PlanDims};
use crate::thread::class::MAX_CORES;
use crate::thread::frame::{await_exit, request_shutdown};

pub mod plan;

/// The default empty store-value list, used as the `Vals` default for a
/// bare `Scheduler` type.
pub use hilavitkutin_api::store_values::SvEmpty as DefaultStoreValues;
pub use plan::PlanCache;

use crate::resource::bindings::BindingsFor;

mod build_error;
pub use build_error::{BuildError, RecommendedOrder};

mod fiber_dispatch;
pub use fiber_dispatch::FiberDispatch;

mod plan_handle;
pub use plan_handle::PlanHandle;

mod null_providers;
pub use null_providers::{DefaultClock, NullClock, NullColumnStorage, NullMemoryProvider};

mod defaults;
mod dirty;
mod run;
mod run_fused;
mod run_parallel;
mod run_trunks;
mod worker;

mod builder;
pub use builder::SchedulerBuilder;

#[cfg(test)]
mod tests;

/// Compile-time guard that a `PlanDims`'s `Units` capacity fits the
/// `GATE2_MAX_UNITS` ceiling that still sizes `run_parallel`'s
/// `gate2_phase` / `gate2_trunk` scratch (#690 lifts those onto `Units`).
/// Forcing `<D as UnitsFitGate2>::ASSERT_UNITS_FIT` evaluates the assertion at
/// monomorphisation, failing the build with a clear message rather than letting
/// an over-wide `Units` index past the fixed arrays at runtime. A named assoc
/// const is used rather than an inline `const {}` because the latter is an
/// anonymous generic constant the `generic_const_exprs` grammar rejects.
trait UnitsFitGate2 {
    const ASSERT_UNITS_FIT: ();
}
impl<D: PlanDims> UnitsFitGate2 for D {
    const ASSERT_UNITS_FIT: () = assert!(
        cap_size(<<D as PlanDims>::Units as Capacity>::CAP) <= GATE2_MAX_UNITS,
        "PlanDims::Units capacity exceeds GATE2_MAX_UNITS: run_parallel's gate2_phase/gate2_trunk scratch is sized by GATE2_MAX_UNITS; reduce Units or wait for the #690 lift onto Units",
    );
}

/// Convenience alias for a built scheduler over the default run-config.
pub type BuiltScheduler<WuVals, Vals, CS> = Scheduler<DefaultRunCfg, WuVals, Vals, CS>;

/// Top-level scheduler.
///
/// Generic over the consumer's `RunCfg`, the retained WorkUnit-value
/// list `WuVals`, the registered store-value list `Vals`, and the
/// `ColumnStorage` `CS` that backs the resource data plane. `Cfg::Out`
/// parameterises `run()`'s return shape. The scheduler owns the resource
/// bindings (`<Vals as BindingsFor>::Bindings`, raw pointers into store columns)
/// and the store itself; the store frees every resource block on its own
/// `Drop`, so the scheduler needs no `Drop` of its own. It also holds the
/// registered WorkUnit instances on `WuVals`, the value-carrying unit
/// list `run()` walks.
pub struct Scheduler<
    Cfg: RunCfg = DefaultRunCfg,
    WuVals = WuNil,
    Vals: StoreValues + BindingsFor = SvEmpty,
    CS: ColumnStorage = NullColumnStorage,
    D: PlanDims = DefaultPlanDims,
    Stores = hilavitkutin_api::access::Empty,
    Clk = DefaultClock,
> {
    _cfg:                 PhantomData<Cfg>,
    /// The registered store access set, retained so `mark_dirty` /
    /// `replace_resource` / `replace_value` can resolve a store type to its
    /// Stores-list bit position (the space `read_masks` / `store_dirty`
    /// index) via the `Locate` witness. Carried as `PhantomData` because the
    /// access set is purely type-level; no runtime value.
    _stores:              PhantomData<Stores>,
    /// The plan's topological dispatch permutation, computed at `build`.
    /// `topo_order[step]` is the registration-list position of the unit
    /// dispatched at topological step `step`; `run` walks the live prefix
    /// `topo_order[0 .. topo_count]`. Sized by the unit-capacity dimension,
    /// so `D` is named by a real field and the scheduler needs no
    /// `PhantomData<D>`.
    topo_order:           <D::Units as Capacity>::Array<USize>,
    /// How many of `topo_order`'s entries are live: the flattened dispatch
    /// total (equals the registered unit count when the fiber partition is
    /// complete, which `derive_phase_dispatch_order` debug-asserts). The tail
    /// past it is the zero-fill the array carries.
    topo_count:           USize,
    /// Locator for the plan's store-backed flat CSR columns (phases, trunks,
    /// fibers, per-unit metadata, per-fiber morsel windows, the RCM renumber),
    /// reserved in `storage` at a `StoreId` range continued past the resource
    /// columns. `PlanHandle::empty()` when no plan is store-backed (the bare
    /// scheduler). The dispatch consumer reads the plan back through it.
    plan_handle:          PlanHandle,
    /// The frame record count fixed at `build`. Input columns are reserved
    /// to it, and `run` windows it into morsels of `RunCfg::MORSEL_SIZE`
    /// (one full-range walk for an accumulator-bearing or record-less
    /// frame).
    record_count:         USize,
    /// Per-fiber dispatch descriptor, computed at `build` alongside
    /// `topo_order`. Each live entry slices `topo_order` for one fiber (in
    /// plan dispatch order) and carries that fiber's `morsel_local` bit, so
    /// `run` dispatches a morsel-local fiber morsel-outer (its intermediate
    /// columns stay cache-resident across the morsel) and an
    /// accumulator-bearing fiber unit-outer (the cross-record-safe form). The
    /// per-fiber bit replaces the whole-pipeline accumulator-free guard.
    fiber_dispatch:       <D::Fibers as Capacity>::Array<FiberDispatch>,
    /// How many of `fiber_dispatch`'s entries are live.
    fiber_dispatch_count: USize,
    // The plan-affecting dirty bitset, sized by the `PlanDims::PlanAffecting`
    // capacity type (the GCE-free lift of the former hardcoded `[AtomicBool;
    // 256]`). `DefaultPlanDims::PlanAffecting = Dim<256>` keeps the default
    // width; a consumer tunes it via its `PlanDims` impl. The capacity is a
    // type, so no `cap_size` expression sits in array-length position and
    // `generic_const_exprs` never runs over it.
    plan_dirty:           <D::PlanAffecting as Capacity>::Array<AtomicBool>,
    plan_cache:           PlanCache,
    /// Per-unit predecessor masks (carrier-position space), copied off the
    /// plan at build. The runtime propagates the dirty seed forward over
    /// these and gates each unit by its position.
    predecessor_masks:    <D::Units as Capacity>::Array<D::AdjRow>,
    /// Per-unit read access masks, copied off the plan. A unit is seeded
    /// dirty when its reads intersect the changed-store mask.
    read_masks:           <D::Units as Capacity>::Array<AccessMask<D::Stores>>,
    /// Per-store change seed (Stores-list-position space). `mark_dirty`,
    /// `replace_resource`, and `replace_value` set bits here; `run` /
    /// `run_fused` consume and clear it each frame.
    /// `Cell` for interior mutability: `run_parallel` rewrites the seed between
    /// frames while parked workers hold a live shared reference to the
    /// scheduler, so the write must not go through a plain field.
    store_dirty:          Cell<AccessMask<D::Stores>>,
    /// Cold-start flag. Every unit is dirty on the first frame after build,
    /// so the first `run` / `run_fused` executes the whole carrier; set
    /// false afterward. `AtomicBool` (Relaxed, ordered by the frame barriers)
    /// so the between-frame write goes through a shared reference.
    first_frame:          AtomicBool,
    /// E4 slice 1: the virtual-fire epoch. Incremented once per pass before
    /// dispatch; a producer's `fire<V>` stamps its `Virtual<V>` cell with the
    /// current value, and an `On<V>` consumer's gate opens when the cell equals
    /// it. Per-pass increment is the domain-10 epoch-reset (spec :709-713): last
    /// pass's stamp no longer equals this pass's epoch, so a stale fire gates
    /// shut without an explicit clear. `AtomicUsize` (Relaxed, ordered by the
    /// frame barriers) wraps effectively-never.
    virtual_epoch:        AtomicUsize,
    /// E4 slice 3: engine-owned meta state (the self-hosting meta pipeline's
    /// mutable resources). Not a `Store` (consumer stores are `Copy` read-only),
    /// written directly by the engine each pass; an `OnMeta` work unit reads it
    /// through the `MetaAccess`-gated Ctx accessor. `SchedulerMetrics::pass_count`
    /// advances once per pass.
    meta_block:           MetaBlock,
    /// E8 adapt: the clock provider sampled at frame start and end for the
    /// pass-duration EMA. Carried from the builder's clock slot;
    /// `DefaultClock` unless overridden via `SchedulerBuilder::clock`.
    clock:                Clk,
    /// Scheduler-owned resource bindings, built from the registered store
    /// values at `build()`. Holds only `Copy` pointers into the store's
    /// reserved columns; no destructor walk on drop.
    bindings:             <Vals as BindingsFor>::Bindings,
    /// The `ColumnStorage` that backs the resource bindings. Owns the
    /// reserved column memory and frees it on its own `Drop`.
    storage:              CS,
    /// Registered WorkUnit instances, retained from the builder in
    /// registration order. `run()` walks this value-carrying unit list.
    wu_values:            WuVals,
    /// GATE-2 persistent-pool sync words (frame seq/done/exited + shutdown +
    /// phase barrier). `'static` with dangling progress_slots and `<1, 1>` arrays
    /// (the C/P-sized adapt arrays are unused until the adapt subsystem ships;
    /// the sync words are scalars). Pinned (see `_pin`), so the spawned workers'
    /// raw pointers into it stay valid for the scheduler's life.
    pool:                 PoolFrame<'static, MAX_CORES, 1>,
    /// Per-worker contexts the spawned-once workers read through a raw pointer.
    /// Populated at the first `run_parallel`; stable because the scheduler is
    /// pinned once threaded.
    worker_ctxs:          [WorkerCtx; MAX_CORES],
    /// Whether the persistent pool has been spawned (first `run_parallel`).
    spawned:              Bool,
    /// Const-grouping result, computed once at the first `run_parallel` and read
    /// by every worker: per-unit waist-bounded phase and within-phase trunk, the
    /// live unit count, the phase count, and the active core count.
    gate2_phase:          [USize; crate::plan::grouping::GATE2_MAX_UNITS],
    gate2_trunk:          [USize; crate::plan::grouping::GATE2_MAX_UNITS],
    gate2_n:              USize,
    gate2_nphases:        USize,
    gate2_ncores:         USize,
    /// Per-core accumulator live counts published by workers on the threaded
    /// unit-outer accumulator path (GATE-2 deviation 9). Flat `[core *
    /// GATE2_MAX_ACCUMS + accum]`; worker `c` stores its per-accumulator live
    /// length (Relaxed) before `frame_done_arrive`, the main thread loads them
    /// after `frame_await_done` (acquire via the done counter) and feeds the
    /// `merge_accums` compaction. Sized by `MAX_CORES * GATE2_MAX_ACCUMS`.
    gate2_accum_live:     [AtomicUsize; MAX_CORES * crate::plan::grouping::GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-(core,accum) atomic publish array; tracked: #121
    /// Engine-internal per-phase duration EMA (domain-22 adapt). The single-core
    /// `dispatch_trunks` loop folds each phase's wall-clock duration here with the
    /// 1/8 EMA. `Cell` interior mutability is sound because only the main thread
    /// writes it (workers never call `dispatch_trunks`), the same discipline as
    /// `store_dirty`. Phases are bounded by units, so `GATE2_MAX_UNITS` bounds it.
    /// Feeds the eventual `select_adapt_config`; not consumer-exposed.
    phase_ema:            [Cell<Nanos>; crate::plan::grouping::GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA store sized by the unit cap; tracked: #121
    /// Per-frame per-phase duration accumulator (raw nanos). `dispatch_trunks`
    /// runs once per morsel, so it SUMS each morsel's phase-slice duration here;
    /// `run` folds the per-frame total into `phase_ema` once at frame end (so the
    /// EMA is per-frame, not per-morsel) and zeroes it. Same single-writer
    /// discipline as `phase_ema` / `store_dirty`.
    phase_accum:          [Cell<Nanos>; crate::plan::grouping::GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame per-phase duration accumulator; tracked: #121
    /// Phase-imbalance reconfigure trigger (domain-22 adapt). `select_adapt_config`
    /// sets it each frame when one active phase's EMA dominates the least active
    /// phase's by more than `BALANCE_FACTOR`. The actuation that acts on it is a
    /// follow-up; this is the decision half. Single-writer (main thread in `run`).
    adapt_reconfigure:    Cell<Bool>,
    /// Marks the scheduler `!Unpin`: once a worker holds a raw pointer into it,
    /// moving it would dangle that pointer, so `run_parallel` takes `Pin`.
    _pin:                 PhantomPinned,
}

/// Per-worker context for the GATE-2 persistent pool. Holds a type-erased
/// back-pointer to the owning `Scheduler` (the monomorphic `worker_main` casts it
/// back to the concrete type) plus the worker's core id. Stored inline in the
/// scheduler at a pinned, stable address; the spawned worker closure captures one
/// `*const WorkerCtx`, so the closure is pointer-sized and a consumer executor
/// that hands it to an OS thread through one pointer-sized argument, with no
/// allocation, can carry it.
struct WorkerCtx {
    sched:   *const (),
    core_id: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: core index carried to the worker in its context; tracked: #121
}

/// Send wrapper so the one-pointer worker closure satisfies `F: Send`. SAFETY:
/// the pointee is pinned scheduler-owned storage that outlives every worker (Drop
/// joins via `await_exit` before teardown); workers touch disjoint write columns.
#[derive(Copy, Clone)]
struct SendCtxPtr(*const WorkerCtx);
// SAFETY: see above.
unsafe impl Send for SendCtxPtr {}

/// Build an empty `PoolFrame<'static, MAX_CORES, 1>` for the scheduler's pool:
/// all sync words zero, dangling progress_slots (the frame protocol never reads
/// them). The core dimension is `MAX_CORES` so the per-core `idle_accumulator` /
/// `park_count` arrays are genuinely per-core (the waist barrier fills
/// `idle_accumulator[core]` for the core-idle adapt axis). The phase dimension
/// stays 1: `predicted_wait_ns` is per-phase and not yet driven, so it needs no
/// real phase cap here.
fn empty_pool_frame() -> PoolFrame<'static, MAX_CORES, 1> {
    PoolFrame {
        shutdown:            AtomicBool::new(false),
        phase_arrived:       AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        barrier_sense:       AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        seq:                 AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        done:                AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        exited:              AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        predicted_wait_ns:   [AtomicU32::new(0)], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        idle_accumulator:    [const { AtomicU64::new(0) }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core atomic init; tracked: #121
        park_count:          [const { AtomicU64::new(0) }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core atomic init; tracked: #121
        progress_slots:      NonNull::dangling(),
        progress_slot_count: <USize as Identity<Additive>>::IDENTITY,
        _arena:              PhantomData,
    }
}

impl<
    Cfg: RunCfg,
    WuVals,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Stores,
    Clk,
> Drop for Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
{
    fn drop(&mut self) {
        if self.spawned.0 {
            // Signal shutdown and wait every spawned worker to leave its mainloop
            // before the inline pool (which the workers read) tears down. This is
            // the join with no thread-join: the exit-counter barrier.
            request_shutdown(&self.pool);
            await_exit(&self.pool, self.gate2_ncores);
        }
    }
}
