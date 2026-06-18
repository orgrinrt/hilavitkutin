//! Sketch: cap-lifting shapes (#121 / #345 / #649).
//!
//! Probes which shapes can lift the engine's three hardcoded caps
//! (GATE2_MAX_UNITS / GATE2_MAX_ACCUMS in plan/grouping.rs, the
//! plan_dirty [AtomicBool; 256] field in scheduler/mod.rs, MAX_CORES in
//! thread/class.rs) to consumer-tunable values on nightly-2026-05-28.
//! One feature per shape so a failing shape does not block the others;
//! build each with `cargo build --features sN`. Findings in FINDINGS.md.
//!
//! Feature gates are per-shape on purpose: s1 (Capacity associated
//! types) and s3 (macro instantiation) must compile WITHOUT
//! generic_const_exprs in this consumer crate; that absence is part of
//! the finding.

#![allow(incomplete_features, dead_code)]
#![cfg_attr(
    any(
        feature = "s0",
        feature = "s2",
        feature = "s2b",
        feature = "s2c",
        feature = "s4",
        feature = "s4b"
    ),
    feature(generic_const_exprs)
)]
#![cfg_attr(
    any(feature = "s1", feature = "s2b", feature = "s2c", feature = "s4b"),
    feature(const_trait_impl)
)]

#[cfg(feature = "s0")]
mod s0;
#[cfg(feature = "s1")]
mod s1;
#[cfg(feature = "s2")]
mod s2;
#[cfg(feature = "s2b")]
mod s2b;
#[cfg(feature = "s2c")]
mod s2c;
#[cfg(feature = "s3")]
mod s3;
#[cfg(feature = "s4")]
mod s4;
#[cfg(feature = "s4b")]
mod s4b;

fn main() {
    #[cfg(feature = "s0")]
    s0::run();
    #[cfg(feature = "s1")]
    s1::run();
    #[cfg(feature = "s2")]
    s2::run();
    #[cfg(feature = "s2b")]
    s2b::run();
    #[cfg(feature = "s2c")]
    s2c::run();
    #[cfg(feature = "s3")]
    s3::run();
    #[cfg(feature = "s4")]
    s4::run();
    #[cfg(feature = "s4b")]
    s4b::run();
    #[cfg(not(any(
        feature = "s0",
        feature = "s1",
        feature = "s2",
        feature = "s2b",
        feature = "s2c",
        feature = "s3",
        feature = "s4",
        feature = "s4b"
    )))]
    println!("no shape feature enabled; build with --features sN");
}
