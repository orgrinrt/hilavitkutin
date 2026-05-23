//! Adapt-metrics arena: hot/cold split sidecar storage.
//!
//! Topic 5 axis D. Per-fiber 64-byte-aligned hot lines co-locate
//! the progress counter (Topic 4 axis E) with inline metrics; per-
//! core 64-byte-aligned park slots; cold SoA region for end-of-pass
//! derived metrics. The arena lives in the plan-stage scratch buffer
//! the scheduler owns.
//!
//! The fiber-formation feasibility check (per Topic 3 M10 +
//! audit-2 M6) subtracts the adapt sidecar footprint from the L1
//! write budget before `compute_morsel_size`. Worst-case-all-active:
//! every axis is assumed enabled when sizing.
//!
//! This module ships the type carrier today; the field layout +
//! runtime population land alongside `Scheduler::run()` (Pass 6) and
//! the bench-validated EMA path (Pass 7). The const-generic shape +
//! arena pointer surface here is enough for the scheduler typestate
//! to thread.

use core::marker::PhantomData;

/// Per-pass adapt-metrics arena. Const-generic over the
/// configuration's worst-case widths. The fields land alongside the
/// `Scheduler::run` body in Pass 6.
///
/// `'arena` ties the storage borrow to the plan-stage scratch buffer
/// the scheduler owns; dropping the scheduler ahead of any reader
/// becomes a borrow-check error rather than a use-after-free.
pub(crate) struct AdaptArena<'arena, const MAX_FIBERS: usize, const MAX_CORES: usize, const MAX_PHASES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; rust grammar requires usize; tracked: #121
    _arena: PhantomData<&'arena ()>,
}

impl<'arena, const F: usize, const C: usize, const P: usize> AdaptArena<'arena, F, C, P> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array sizes; rust grammar requires usize; tracked: #121
    /// Allocate a fresh arena. Implementation lands in Pass 6 alongside
    /// the scheduler's plan-stage scratch buffer.
    pub(crate) const fn new() -> Self {
        Self { _arena: PhantomData }
    }
}
