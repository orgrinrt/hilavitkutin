//! Minimal model of the transparent-dispatch witness question, no engine deps.
//!
//! Question: can a `min_specialization` specializing impl route a fusible carrier
//! to a fused dispatch that needs a SECOND projection witness (`W2`), when the
//! base impl carries the per-WU witness `W`? `RunFiber`'s witness is a per-call
//! inferred type param (engine_ctx's deliberate E0207 dodge in `Project`). The
//! fused `ChainWu` carrier needs its OWN witness, which in a specializing impl
//! would appear only in a where-clause => unconstrained impl param (E0207),
//! independent of `min_specialization`.
//!
//! Build: `rustc --edition 2021 src/min_model.rs -o /tmp/min_model` (or via the
//! [[bin]] below). Outcome recorded at the bottom.

#![feature(min_specialization)]
#![allow(dead_code)]

// A "walk" keyed by a per-call witness W (models RunFiber<A, W>).
trait Walk<W> {
    fn walk(&self) -> u32;
}

// The general carrier: walks with witness `u8`.
#[derive(Copy, Clone)]
struct Carrier;
impl Walk<u8> for Carrier {
    fn walk(&self) -> u32 {
        1
    }
}

// The fusible-ness marker (models FuseCarrier).
trait Fuse {
    fn fuse(self) -> Fused;
}
impl Fuse for Carrier {
    fn fuse(self) -> Fused {
        Fused
    }
}

// The fused single-element carrier: walks with a DIFFERENT witness `u32`.
#[derive(Copy, Clone)]
struct Fused;
impl Walk<u32> for Fused {
    fn walk(&self) -> u32 {
        2
    }
}

// The transparent dispatch trait. `W` is the per-call witness, in the header
// (so the base impl has no unconstrained param). run() would call this with W
// inferred at the call site.
trait Program<W> {
    fn run_program(&self) -> u32;
}

// Base: any carrier that can walk with witness W.
impl<W, C: Walk<W>> Program<W> for C {
    default fn run_program(&self) -> u32 {
        self.walk()
    }
}

// Specialized: a fusible carrier folds and dispatches the fused walk, which
// needs the fused witness `W2`. `W2` appears only in the where-clause
// `Fused: Walk<W2>` -> the question is whether this is E0207 (unconstrained
// impl parameter) even though exactly one `W2` (= u32) satisfies it.
impl<W, W2, C> Program<W> for C
where
    C: Walk<W> + Fuse + Copy,
    Fused: Walk<W2>,
{
    fn run_program(&self) -> u32 {
        (*self).fuse().walk()
    }
}

fn main() {
    let c = Carrier;
    // If this compiles, transparent dispatch threads both witnesses and the
    // fused path is selected for the fusible carrier.
    let r: u32 = Program::run_program(&c);
    println!("min-model run_program = {r}");
}

// ---------------------------------------------------------------------
// OUTCOME: FAILS (nightly-2026-05-28, `rustc +nightly-2026-05-28`). Transparent
// single-`run()` auto-fusion via `min_specialization` is NOT expressible, for two
// INDEPENDENT reasons:
//
//   1. E0207: "the type parameter `W2` is not constrained by the impl trait,
//      self type, or predicates". The fused carrier's projection witness `W2`
//      appears only in the where-clause (`Fused: Walk<W2>`), so it is an
//      unconstrained impl parameter, even though exactly one `W2` satisfies the
//      bound (rustc does not do that inference for impl params). This is the
//      `RunFiber<A, Witnesses>` design: the witness is a per-CALL inferred type
//      param (engine_ctx's deliberate E0207 dodge in `Project`). A specializing
//      impl cannot thread a second, different witness. ORTHOGONAL to any feature.
//   2. "cannot specialize on trait `Walk` / `Fuse` / `Copy` / `Clone`":
//      `min_specialization` only permits specialization on always-applicable,
//      lifetime-free predicates. A `FuseCarrier` / `RecordOp` (or here `Fuse` /
//      `Walk`) bound is NOT always-applicable, so min_spec rejects the
//      specializing impl outright. So even WITH the fused witness solved, the
//      min_spec route is closed.
//
// CONCLUSION: the dispatch choice (fuse vs per-WU walk) cannot be a transparent
// `run()` that auto-detects. This is NOT feature-avoidance: `min_specialization`
// is enabled and tried here, and it cannot express this. The correct shape is an
// explicit inherent method `Scheduler::run_fused<W2>()` whose witness `W2` is a
// METHOD generic inferred at the call site (the same mechanism today's
// `run<W>()` uses) -- no specialization, no E0207. The engine still performs the
// fold (`FuseCarrier::fuse` on the retained carrier); the consumer opts a
// fusible fiber in by calling `run_fused()`. The body is already proven by
// sketch 202606091400 (it walked `WuCons<ChainWu, WuNil>` via a witness-inferred
// `run_fiber_morsel_outer`). `run()` stays the general per-WU walk for
// non-fusible carriers (accumulators).
// ---------------------------------------------------------------------
