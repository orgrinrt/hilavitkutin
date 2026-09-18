//! `Default` for the bare `Scheduler` (empty store, `NullColumnStorage`).
//!
//! Split out of `scheduler/mod.rs` (file-size lint). No behaviour change.
//! `super::Scheduler`'s fields, `WorkerCtx` and `empty_pool_frame` stay
//! defined directly in `scheduler` (`mod.rs`): this module is a descendant,
//! so it reaches them without any widening.

use core::cell::Cell;
use core::marker::{PhantomData, PhantomPinned};
use core::sync::atomic::{AtomicBool, AtomicUsize};

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_tensor::Capacity;
use hilavitkutin_api::platform::Nanos;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::work_unit_values::WuNil;

use super::{
    FiberDispatch,
    NullColumnStorage,
    PlanHandle,
    Scheduler,
    SvEmpty,
    WorkerCtx,
    empty_pool_frame,
};
use crate::plan::grouping::{GATE2_MAX_ACCUMS, GATE2_MAX_UNITS};
use crate::plan::{AccessMask, DefaultPlanDims, PlanDims};
use crate::thread::class::MAX_CORES;

/// Default-construct an empty scheduler over the null store.
///
/// Only available for the no-store (`SvEmpty`) shape with the
/// `NullColumnStorage`: the empty bindings (`BindingNil`) owns nothing and
/// the null store reserves nothing, so no real store is needed. A
/// scheduler that owns resources is built via `build(storage)`.
impl<Cfg: RunCfg> Default for Scheduler<Cfg, WuNil, SvEmpty, NullColumnStorage> {
    fn default() -> Self {
        Self {
            _cfg:                 PhantomData,
            _stores:              PhantomData,
            topo_order:           <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                <USize as Identity<Additive>>::IDENTITY,
            ),
            topo_count:           <USize as Identity<Additive>>::IDENTITY,
            plan_handle:          PlanHandle::empty(),
            record_count:         <USize as Identity<Additive>>::IDENTITY,
            // The empty bundle (`WuNil`) writes no accumulator.
            fiber_dispatch:       <<DefaultPlanDims as PlanDims>::Fibers as Capacity>::filled(
                FiberDispatch::default(),
            ),
            fiber_dispatch_count: <USize as Identity<Additive>>::IDENTITY,
            plan_dirty:
                <<DefaultPlanDims as PlanDims>::PlanAffecting as Capacity>::from_fn(|_| {
                    AtomicBool::new(false)
                }),
            plan_cache:           super::PlanCache::new(),
            predecessor_masks:    <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                <DefaultPlanDims as PlanDims>::AdjRow::default(),
            ),
            read_masks:           <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                AccessMask::empty(),
            ),
            store_dirty:          Cell::new(AccessMask::empty()),
            first_frame:          AtomicBool::new(true),
            virtual_epoch:        AtomicUsize::new(0),
            meta_block:           crate::meta::MetaBlock::default(),
            clock:                super::DefaultClock::new(),
            bindings:             crate::resource::bindings::BindingNil,
            storage:              NullColumnStorage,
            wu_values:            WuNil,
            pool:                 empty_pool_frame(),
            worker_ctxs:          [const {
                WorkerCtx {
                    sched:   core::ptr::null(),
                    core_id: 0,
                }
            }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
            spawned:              Bool::FALSE,
            gate2_phase:          [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_UNITS],
            gate2_trunk:          [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_UNITS],
            gate2_n:              <USize as Identity<Additive>>::IDENTITY,
            gate2_nphases:        <USize as Identity<Additive>>::IDENTITY,
            gate2_ncores:         <USize as Identity<Additive>>::IDENTITY,
            gate2_accum_live:     [const { core::sync::atomic::AtomicUsize::new(0) };
                MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
            phase_ema:            [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
            phase_accum:          [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase accumulator zero-init; tracked: #121
            adapt_reconfigure:    Cell::new(Bool::FALSE),
            _pin:                 PhantomPinned,
        }
    }
}
