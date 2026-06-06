//! Sketch (D1b Tier 4, op-persona redirect 2026-06-06 / #340): the positive-only
//! type-level grouping formulations, tried before locking the hybrid.
//!
//! The op-persona review (simulated, safety-net) pushed back: Tier 3 proved ONE
//! formulation (a total `DependsBool` with a negative arm) walls, but did NOT
//! prove grouping is unexpressible at the type level. It named two positive-only
//! formulations to try first:
//!   (a) the plan supplies each WU's fiber-id as an associated TYPE tag, then a
//!       positive-only type-level partition fold by tag;
//!   (b) DAG edges (predecessors) as associated types, grouping as positive-only
//!       forward recursion.
//!
//! This file tries (a), the stronger candidate. Each WU carries a `Tag` (a peano
//! type-level nat, simulating the plan's `group_fibers` output). The goal: fold
//! the flat `WuCons` list into `FiberCons`-of-`WuCons` by grouping consecutive
//! same-tag WUs.
//!
//! Hypothesis: (a) still walls, because the fold's per-step boundary decision is
//! "tag of next == tag of current (extend) OR != (close fiber, open new)". The
//! "==" arm is a positive type-equality (`impl SameTag<T, T>`, reflexive, fine),
//! but the "!=" arm is a type INEQUALITY, which is the same negative trait
//! reasoning Tier 3 hit: there is no positive witness for "these two types
//! differ". Partition-by-key at the type level is inherently negative at the
//! boundary, regardless of whether the key is a supplied tag or a derived
//! predicate. The build is the test; outcome recorded at the bottom.
//!
//! Cross-check that informs this: the shipped `group_fibers`
//! (mock/crates/hilavitkutin/src/plan/steps.rs:445) rolls fibers on out-degree
//! over the topo order (plus block-diagonal / spectral variants), NOT on a
//! pairwise tag/predicate. So even a working tag-fold would not match the real
//! boundary rule; the grouping is a runtime graph algorithm. Tier 4 tests the
//! best-case type-level formulation anyway, to be sure the door is closed.

#![allow(dead_code)]
#![feature(marker_trait_attr)]

// Reuse the proven carrier shapes conceptually; here only the type-level fold
// machinery matters, so use local minimal stand-ins for WU identity.
struct WuNil;
struct WuCons<H, T>(H, T);

// Type-level nat tags (simulating the plan's per-WU fiber-id output).
struct Z;
struct S<N>(N);

// Each "WU" carries a fiber tag.
trait FiberTagged {
    type Tag;
}
struct U1; // fiber 0
struct U2; // fiber 0 (same fiber as U1)
struct U3; // fiber 1
impl FiberTagged for U1 {
    type Tag = Z;
}
impl FiberTagged for U2 {
    type Tag = Z;
}
impl FiberTagged for U3 {
    type Tag = S<Z>;
}

// Positive type equality: reflexive only. `SameTag<A, A>` holds; nothing proves
// `SameTag<A, B>` for distinct A, B (that is the point).
trait SameTag<Other> {}
impl<T> SameTag<T> for T {}

// THE FOLD. Group consecutive same-tag WUs into fibers. The recursion must, at
// each cons cell, decide: does Head's tag match the fiber currently being built?
//   - match  -> push Head onto the current fiber, recurse (POSITIVE: SameTag).
//   - differ -> close the current fiber, start a new one with Head (NEGATIVE:
//               requires proving the tags are NOT the same).
//
// Encode the carry as (CurrentFiberAccum, CurrentTag). A `GroupBy` trait folds
// the input list. The two recursion arms below overlap on every (Head, CurTag):
// the "extend" arm fires when Head::Tag == CurTag, the "boundary" arm when it
// does not. Without a negative witness for "!=", the boundary arm cannot be
// written disjointly from the extend arm. The attempt makes the wall explicit.
trait GroupBy<CurTag, CurFiber> {
    type Out;
}

// Base: empty input -> the current fiber is the last one.
impl<CurTag, CurFiber> GroupBy<CurTag, CurFiber> for WuNil {
    type Out = FiberCons<CurFiber, FiberNil>;
}

struct FiberCons<F, Rest>(F, Rest);
struct FiberNil;

// Extend arm: Head shares the current tag -> prepend to current fiber, recurse.
impl<Head, Tail, CurTag, CurFiber> GroupBy<CurTag, CurFiber> for WuCons<Head, Tail>
where
    Head: FiberTagged,
    Head::Tag: SameTag<CurTag>,
    Tail: GroupBy<CurTag, WuCons<Head, CurFiber>>,
{
    type Out = <Tail as GroupBy<CurTag, WuCons<Head, CurFiber>>>::Out;
}

// Boundary arm: Head does NOT share the current tag -> close CurFiber, open a new
// fiber tagged Head::Tag. THIS is the arm that needs the negative. Written here
// as a second impl that overlaps the extend arm; the compiler rejects it because
// nothing distinguishes "differ" from "same" without negative reasoning.
impl<Head, Tail, CurTag, CurFiber> GroupBy<CurTag, CurFiber> for WuCons<Head, Tail>
where
    Head: FiberTagged,
    Tail: GroupBy<Head::Tag, WuCons<Head, WuNil>>,
{
    type Out = FiberConsBoundary<CurFiber, <Tail as GroupBy<Head::Tag, WuCons<Head, WuNil>>>::Out>;
}

struct FiberConsBoundary<F, Rest>(F, Rest);

fn main() {
    println!("if this compiled, the positive-only tag-fold expresses grouping at the type level");
}

// ---------------------------------------------------------------------
// OUTCOME: FAILS WITH E0119 (conflicting implementations of `GroupBy<_, _>` for
// `WuCons<_, _>`). The positive-only tag-fold walls at the SAME boundary as the
// negative DependsBool (Tier 3): the "extend" arm (Head::Tag SameTag CurTag) and
// the "boundary" arm (Head::Tag differs) overlap on every cons cell, and without
// a negative witness for "tags differ" the boundary arm cannot be disjoint.
//
// CONCLUSION (op-persona redirect closed): partition-by-key at the type level is
// inherently negative at the boundary, whether the key is a supplied tag (this
// file) or a derived predicate (tier3.rs). Both positive-only formulations the
// review named wall. Combined with the cross-check of the shipped `group_fibers`
// (mock/crates/hilavitkutin/src/plan/steps.rs:445), whose boundary rule is the
// DAG out-degree over the topo order plus block-diagonal / spectral variants
// (Fiedler power iteration), NOT a pairwise tag/predicate: the fiber grouping is a
// runtime graph algorithm, not any kind of type-level fold. The door is closed.
//
// This is NOT a problem for the keystone. Per consolidation domain 17 (`:1566`,
// `:1732-1733`): "the flattener emits a monomorphised function ... derived from
// the WU declarations' resource Read/Write sets. The codegen (domain 17
// flattener) emits it." The canonical mechanism is a codegen flattener that
// EMITS the per-core program, with the grouping COMPUTED by the plan (the shipped
// `group_fibers`, a runtime graph walk) and feeding morsel sizing + trunk-to-core
// assignment (R6 adaptive/plan params). The dispatch STRUCTURE is the flat
// per-core carrier, which is type-level and devirtualises (Tier 1 here +
// schedule-mega 202606060500, the 0.97x bench winner). The synthesis 2.4 "build
// time = the type system, grouping is a type-level computation" framing was a
// hypothesis the synthesis itself flagged as "the single biggest unproven
// question"; these sketches answer it: the CARRIER is type-level, the GROUPING is
// not (and need not be). The remaining bridge (runtime plan order -> compile-time
// flat per-core carrier type) is D1a.
// ---------------------------------------------------------------------
