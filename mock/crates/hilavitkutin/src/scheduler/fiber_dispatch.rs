//! Per-fiber dispatch descriptor.
//!
//! Split out of `scheduler/mod.rs` (file-size lint). No behaviour change.

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};

/// One fiber's slice of the flat dispatch order plus its morsel-locality
/// bit: the compact per-fiber dispatch program `run` walks.
///
/// `run` dispatches a `morsel_local` fiber morsel-outer (one morsel runs the
/// fiber's whole unit sequence before the next, keeping its intermediate
/// columns cache-resident) and an accumulator-bearing fiber unit-outer (a
/// unit completes its record range before the next, the cross-record-safe
/// form).
#[derive(Copy, Clone)]
pub struct FiberDispatch {
    /// Start index of this fiber's units in `topo_order`.
    pub start:          USize,
    /// Number of this fiber's units (its slice length in `topo_order`).
    pub len:            USize,
    /// True when the fiber writes no accumulator, so it dispatches
    /// morsel-outer.
    pub morsel_local:   Bool,
    /// This fiber's per-fiber morsel window size (records per morsel chunk),
    /// copied from `plan.morsel_windows[fiber_plan_idx]` at dispatch-order
    /// derivation. `run` windows a `morsel_local` fiber's record range by
    /// this (fiber-outer/morsel-inner, A2b). A zero value means fall back to
    /// `Cfg::MORSEL_SIZE`.
    pub morsel_size:    USize,
    /// The plan CSR fiber index this descriptor came from. The `fiber_dispatch`
    /// array is in phase-sequential dispatch order, a different index space from
    /// `plan.morsel_windows` (CSR fiber order); this records the mapping so the
    /// per-fiber size is read from the right CSR slot.
    pub fiber_plan_idx: USize,
}

impl Default for FiberDispatch {
    #[inline]
    fn default() -> Self {
        Self {
            start:          <USize as Identity<Additive>>::IDENTITY,
            len:            <USize as Identity<Additive>>::IDENTITY,
            morsel_local:   Bool::TRUE,
            morsel_size:    <USize as Identity<Additive>>::IDENTITY,
            fiber_plan_idx: <USize as Identity<Additive>>::IDENTITY,
        }
    }
}
