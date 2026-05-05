//! S4 candidate A: opt-in Replaceable.
//!
//! `pub trait Replaceable {}` with no auto impl. Authors opt in by
//! writing `impl Replaceable for Foo {}` per type they want the app
//! to be able to override via `.replace_resource::<Foo>(...)`.
//!
//! Build: `rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata`

#![allow(unused)]
#![no_std]

pub trait Replaceable {}

// Three kits with mixed Replaceable / non-Replaceable owned state.

// MockspaceKit: owns RoundState (private, not replaceable),
// DesignRound (private, not replaceable), LintConfig (potentially
// app-overridable for tests).
pub struct RoundState;
pub struct DesignRound;
pub struct LintConfig;
impl Replaceable for LintConfig {}  // opt in for testability

// BenchTracingKit: owns Tracer (potentially app-overridable to swap
// implementations), TraceSample (private).
pub struct Tracer;
pub struct TraceSample;
impl Replaceable for Tracer {}  // opt in

// LintPackKit: owns Diagnostic (cooperative-public, app may swap with
// alternative diagnostic representation), Statistics (private).
pub struct Diagnostic;
pub struct Statistics;
impl Replaceable for Diagnostic {}  // opt in

// Replace API stub.
pub fn replace_resource<T: Replaceable>(_new: T) {}

// Demo: replace replaceable resources.
pub fn demo_success() {
    replace_resource::<LintConfig>(LintConfig);
    replace_resource::<Tracer>(Tracer);
    replace_resource::<Diagnostic>(Diagnostic);
}

// Demo: try to replace a non-Replaceable resource. Compile error expected.
#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    replace_resource::<RoundState>(RoundState);
}

// Author cost summary (manual count from the file above):
// 6 Owned types declared. 3 opted in (impl Replaceable). 3 omitted.
// Author writes 3 impl lines to enable replacement.
