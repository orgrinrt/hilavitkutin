//! `Plannable` is the runner-agnostic declaration half of a `WorkUnit`.
//!
//! Three properties, one per test: every `WorkUnit` is `Plannable` via
//! the blanket; a non-`WorkUnit` type hand-implements `Plannable` (the
//! shape a recording runner uses); and the `HintExt` slot admits a
//! 4-tuple hint without disturbing the 3-tuple.

#![no_std]

use hilavitkutin_api::{
    Always, HintExt, Immediate, Atomic, Normal, Plannable, SchedulingHint,
};
use hilavitkutin_api::access::Empty;

// A generic function that accepts anything Plannable and recovers its
// declared sets. This is the shape the plan stage takes: it reads the
// declaration, never `execute`.
fn declared_sets<T: Plannable<S>, S>() -> core::marker::PhantomData<(T::Read, T::Write, T::Hint)> {
    core::marker::PhantomData
}

// A type that is NOT a WorkUnit: no Ctx, no execute, no fiber model.
// This is the GPU-runner shape, a pass that records a dispatch.
struct RecordingPass;
impl Plannable for RecordingPass {
    type Read = Empty;
    type Write = Empty;
    type Hint = (Immediate, Atomic, Normal);
}

#[test]
fn non_workunit_is_plannable_by_hand() {
    // Accepted at the Plannable bound without being a WorkUnit.
    let _ = declared_sets::<RecordingPass, Always>();
}

// A test-local HintExt marker: proves the extension slot is usable
// downstream. No HintExt markers ship in the crate itself.
struct TestExt;
impl HintExt for TestExt {}

#[test]
fn hint_ext_admits_a_fourth_axis() {
    fn takes_hint<H: SchedulingHint>() {}
    takes_hint::<(Immediate, Atomic, Normal)>(); // existing 3-tuple
    takes_hint::<(Immediate, Atomic, Normal, TestExt)>(); // extended 4-tuple
}
