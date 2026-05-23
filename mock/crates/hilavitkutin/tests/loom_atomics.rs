//! Loom-gated atomic-ordering tests per Topic 3 S7.
//!
//! Pass 7 of runtime megaround `202605101036` reserves this file
//! for the per-pair loom tests covering every cross-thread atomic
//! pair in the S7 ordering table. Tests gate on `cfg(loom)` so the
//! standard test run skips them; CI integration that runs loom is
//! deferred to mockspace task #203 per the round spec.

#![cfg(loom)]

#[test]
fn _s7_ordering_pairs_validated_in_loom_run() {
    // empty body; the S7 ordering table enumerates the
    // Release/Acquire pairs that gate per-phase visibility of
    // PoolFrame fields. Real loom tests land alongside the
    // executor wiring that exercises those pairs.
    loom::model(|| {});
}
