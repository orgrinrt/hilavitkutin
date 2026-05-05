//! S4 candidate B: opt-out Replaceable.
//!
//! Default-impl-on-everything via `auto trait Replaceable {}`. Authors
//! opt out by writing `impl !Replaceable for Foo {}` per type they
//! want to lock down. Requires `feature(auto_traits)` and
//! `feature(negative_impls)`.
//!
//! Round-3 evidence flagged `feature(negative_impls)` as having
//! coherence issues for type-list disequality. This case is simpler:
//! `Replaceable` is a marker on a single type, not a relation across
//! a heterogeneous list. The negative impl applies to one type per
//! line. Whether that triggers coherence overlap depends on whether
//! any positive impl matches the same type.
//!
//! Build: `rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata`

#![allow(unused, incomplete_features)]
#![no_std]
#![feature(auto_traits)]
#![feature(negative_impls)]

pub auto trait Replaceable {}

// Three kits with mixed Replaceable / non-Replaceable owned state.
//
// Default: every type below is Replaceable unless explicitly excluded.

// MockspaceKit: RoundState + DesignRound are private (lock down);
// LintConfig is replaceable (no opt-out).
pub struct RoundState;
impl !Replaceable for RoundState {}

pub struct DesignRound;
impl !Replaceable for DesignRound {}

pub struct LintConfig;
// no impl !Replaceable; defaults to Replaceable.

// BenchTracingKit: Tracer is replaceable; TraceSample is private.
pub struct Tracer;
// no impl !Replaceable; defaults to Replaceable.

pub struct TraceSample;
impl !Replaceable for TraceSample {}

// LintPackKit: Diagnostic is replaceable; Statistics is private.
pub struct Diagnostic;
// no impl !Replaceable; defaults to Replaceable.

pub struct Statistics;
impl !Replaceable for Statistics {}

// Replace API stub.
pub fn replace_resource<T: Replaceable>(_new: T) {}

pub fn demo_success() {
    replace_resource::<LintConfig>(LintConfig);
    replace_resource::<Tracer>(Tracer);
    replace_resource::<Diagnostic>(Diagnostic);
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    replace_resource::<RoundState>(RoundState);
}

// Author cost summary:
// 6 Owned types declared. 3 opted out (impl !Replaceable for ...).
// 3 left as default-replaceable.
// Author writes 3 impl !Replaceable lines to lock down.
