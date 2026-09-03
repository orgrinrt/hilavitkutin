//! Morsel record range + morsel-loop body (domain 17).
//!
//! One morsel's slice of the global record index space, plus the
//! per-fiber morsel-iteration scaffolding per Topic 7 axes B-G.
//!
//! Two shapes:
//!
//! 1. **`MorselRange`** half-open record range value type that
//!    plan-stage attaches to per-fiber dispatch records.
//!
//! 2. **`iter_morsel`** the per-morsel iteration helper. Forward
//!    iteration default (head+tail variant lands when a fiber is
//!    eligible per Topic 7 axis E). Inline sync at micro-morsel
//!    boundary via `core::hint::spin_loop` budget; no futex inside
//!    a morsel. Phase barrier sync between phases uses
//!    `PoolFrame.phase_arrived` (Topic 6 axis I).
//!
//! Per-fiber `AtomicUsize` "max progress" counter (inline sync)
//! lives in the arena per CHANGE 5; peer-worker reads Acquire and,
//! if behind by `MICRO_MORSEL.MAX_DRIFT_RECORDS`, spins briefly via
//! `core::hint::spin_loop`. The default `MICRO_MORSEL_INTERVAL` is
//! `Cfg::MICRO_MORSEL_INTERVAL` (64-record cap, pow2); the default
//! `MAX_DRIFT_RECORDS` is `Cfg::MAX_DRIFT_RECORDS` (32, pow2).
//! Consumer-overridable per `RunCfg`.

use arvo::strategy::Identity;
use arvo::{Bool, USize};

/// Half-open `[start, start + len)` record range for one morsel.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MorselRange {
    /// First record index.
    pub start: USize,
    /// Count of records in this morsel.
    pub len: USize,
}

impl MorselRange {
    /// Construct a morsel range with `start` and `len`.
    pub const fn new(start: USize, len: USize) -> Self {
        Self { start, len }
    }

    /// One past the last record index (`start + len`).
    pub const fn end(&self) -> USize {
        USize(self.start.0 + self.len.0)
    }

    /// True iff `len == 0`.
    pub const fn is_empty(&self) -> Bool {
        Bool(self.len.0 == 0)
    }
}

impl Default for MorselRange {
    fn default() -> Self {
        Self {
            start: USize::ZERO,
            len: USize::ZERO,
        }
    }
}

/// Records between micro-morsel inner-loop sync points. Pow2 cap.
/// Defaults to `Cfg::MICRO_MORSEL_INTERVAL` (64) per Topic 7 axis C.
/// Codegen propagates the actual value at instantiation.
pub const MICRO_MORSEL_INTERVAL: USize = USize(64); // lint:allow(no-bare-numeric) reason: cookbook default; runtime value flows through Cfg::MICRO_MORSEL_INTERVAL; tracked: #121

/// Maximum inter-fiber misalignment in records before forced
/// realign via `core::hint::spin_loop`. Pow2 cap. Defaults to
/// `Cfg::MAX_DRIFT_RECORDS` (32) per Topic 7 axis C. Codegen
/// propagates the actual value at instantiation.
pub const MAX_DRIFT_RECORDS: USize = USize(32); // lint:allow(no-bare-numeric) reason: cookbook default; runtime value flows through Cfg::MAX_DRIFT_RECORDS; tracked: #121

/// Walk one morsel range forward, applying a per-record body and
/// inserting an inline sync probe every `MICRO_MORSEL_INTERVAL`
/// records. Topic 7 axes B + C + D.
///
/// `body` is the monomorphised per-WU sequence emitted by
/// `super::fiber_dispatch::run_fiber`. `sync_probe` is the inline
/// peer-drift check that walks the arena counters and, when a peer
/// is behind by more than `MAX_DRIFT_RECORDS`, spins briefly via
/// `core::hint::spin_loop`. Both close over the dispatch frame at
/// the call site; this fn carries the iteration shape only.
#[inline(never)]
pub fn iter_morsel<F, S>(range: MorselRange, mut body: F, mut sync_probe: S)
where
    F: FnMut(USize),
    S: FnMut(USize),
{
    let mut i = range.start;
    let end = range.end();
    while i.0 < end.0 {
        body(i);
        i = USize(i.0 + 1);
        if i.0 % MICRO_MORSEL_INTERVAL.0 == 0 {
            sync_probe(i);
        }
    }
}
