//! Engine-owned meta state (E4 slice 3, the engine-to-meta bridge).
//!
//! The self-hosting meta pipeline (T5 §Q9) needs mutable per-pass state the
//! scheduler maintains and an `OnMeta` work unit reads. That state cannot ride a
//! consumer `Resource<T>`: a consumer resource is `Copy` and read-only after
//! init (the arena drain requires `Copy`, and work units read it via `&T`), so
//! it cannot carry a `Cell`-bearing field the engine mutates each pass. So the
//! meta state is engine-owned: a `MetaBlock` field on the `Scheduler`, written
//! directly (a plain field write, no registration, no `Selector` witness, no
//! specialization), read by an `OnMeta` work unit through the `MetaAccess`-gated
//! Ctx accessor in `dispatch::engine_ctx`.
//!
//! `MetaField` is the type-keyed projection from the block to one meta resource,
//! the meta analogue of the store `Selector`. The Ctx accessor resolves
//! `meta::<T>()` through it. Only `SchedulerMetrics` is wired this round (its
//! `pass_count`); the other meta resources (`Dag`, `ExecutionPlan`,
//! `LaneAssignment`) carry their data in the plan structures and join the block
//! with their data sources.

use arvo::Bool;
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::platform::Nanos;

/// Engine-owned mutable meta state, held as a `Scheduler` field.
///
/// Not `Copy`, not a `Store`: it holds interior-mutable meta resources the
/// engine writes directly. The `MetaBlock` reference is wired into an `OnMeta`
/// work unit's Ctx (as a `MetaRef`) at dispatch; the Ctx accessor projects each
/// meta resource out via `MetaField`.
#[derive(Default)]
pub struct MetaBlock {
    /// Scheduler self-observation state, advanced per pass.
    pub metrics: SchedulerMetrics,
}

/// Type-keyed projection of one meta resource out of the `MetaBlock`.
///
/// The meta analogue of the store `Selector`: the Ctx `meta::<T>()` accessor
/// resolves `&T` through this. Implemented per meta resource as it gains a home
/// in the block.
pub trait MetaField {
    /// Borrow this meta resource out of the engine-owned block.
    fn project(block: &MetaBlock) -> &Self;
}

impl MetaField for SchedulerMetrics {
    #[inline]
    fn project(block: &MetaBlock) -> &Self {
        &block.metrics
    }
}

/// Fold one frame-duration sample into the pass-duration EMA (weight 1/8).
///
/// The seed frame (first frame after build) stores the raw sample; later
/// frames step one eighth of the gap toward the sample in integer nanos. The
/// type is unsigned, so the step branches on direction instead of going
/// through a signed intermediate. Shared by `run`, `run_parallel`, and
/// `run_fused`; each calls it at frame end, between frames, so the write
/// needs no synchronisation beyond the existing frame protocol.
#[inline]
pub(crate) fn fold_ema(prev: Nanos, sample: Nanos, seed: Bool) -> Nanos {
    if seed.0 {
        return sample;
    }
    let eighth_div = Nanos::from_raw(8); // lint:allow(no-bare-numeric) reason: canonical EMA weight 1/8, spec T5; tracked: #121
    if sample.to_raw() >= prev.to_raw() {
        prev + (sample - prev) / eighth_div
    } else {
        prev - (prev - sample) / eighth_div
    }
}

#[cfg(test)]
mod fold_ema_tests {
    use super::*;

    fn ns(v: u64) -> Nanos { // lint:allow(no-bare-numeric) reason: test fixture literal lift; tracked: #121
        Nanos::from_raw(v)
    }

    #[test]
    fn seed_frame_stores_raw_sample() {
        assert_eq!(fold_ema(ns(0), ns(300), Bool::TRUE).to_raw(), 300);
        // A nonzero stale value is overwritten on seed, not folded.
        assert_eq!(fold_ema(ns(999), ns(300), Bool::TRUE).to_raw(), 300);
    }

    #[test]
    fn upward_fold_steps_an_eighth() {
        assert_eq!(fold_ema(ns(300), ns(700), Bool::FALSE).to_raw(), 350);
        // Truncating division: 350 + (700 - 350) / 8 = 350 + 43.
        assert_eq!(fold_ema(ns(350), ns(700), Bool::FALSE).to_raw(), 393);
    }

    #[test]
    fn downward_fold_steps_an_eighth() {
        assert_eq!(fold_ema(ns(700), ns(300), Bool::FALSE).to_raw(), 650);
        // A gap under the weight truncates to a zero step.
        assert_eq!(fold_ema(ns(305), ns(300), Bool::FALSE).to_raw(), 305);
    }

    #[test]
    fn equal_sample_is_a_fixpoint() {
        assert_eq!(fold_ema(ns(420), ns(420), Bool::FALSE).to_raw(), 420);
        assert_eq!(fold_ema(ns(0), ns(0), Bool::FALSE).to_raw(), 0);
    }
}
