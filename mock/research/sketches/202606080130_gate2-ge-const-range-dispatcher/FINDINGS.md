# Sketch findings: GATE-2 G-e const-range trunk-root dispatcher

**Date:** 2026-06-08
**Round:** 202606080100 (GATE-2 G-e const-gated per-trunk dispatch)
**Sketch:** `dispatcher.rs` (`rustc +nightly-2026-05-28 --edition 2021 -O`, runnable)
**Outcome:** WORKS

## Hypothesis

G-e's two halves are already proven (sketch 071230: the per-trunk
`const { trunk_of(POS)==TRUNK }`-gated walk DCEing to a member-only mono; 071330:
the grouping computed from access-set types). The remaining un-proven-as-a-whole
piece is the OUTER dispatcher: drive `run_one_trunk::<TRUNK>` for every trunk
across every phase, in phase order, single-core, output-equivalent to the flat
walk. This sketch proves that dispatcher compiles and orders correctly.

## Key finding: numeric const-recursion-to-a-bound does NOT terminate at the type level

The first design recursed numerically: `DispatchPosCg<POS>` recursing to
`DispatchPosCg<{POS+1}>` (stop at `N`) and `DispatchPhasesCg<PHASE>` to
`{PHASE+1}` (stop at `NPHASES`), each recursive call guarded by an
`if const { POS+1 < N }`. It FAILED to compile: "unconstrained generic constant"
/ unbounded type recursion. The `if const` guard gates RUNTIME execution but the
recursive call's trait bound (`C: DispatchPosCg<{POS+1}>`) is still required at
the type level regardless, so the bound chain `POS -> POS+1 -> POS+2 -> ...` never
bottoms out. The `[(); N - POS]:` witnesses did not rescue it (they constrain the
value, not the recursion depth).

This is exactly the engine's established pattern lesson: the shipped
`RunFiber` / `RunTrunkSel` / `BundleMasks` walks recurse STRUCTURALLY on the
carrier cons-list (`WuCons -> ... -> WuNil`), threading the numeric position as
`{ POS + 1 }` ALONGSIDE the structural recursion. The cons-list terminates at
`WuNil`, so the numeric thread terminates with it. Numeric recursion to a bound is
not the engine idiom and does not type-terminate.

## The working shape

Two changes from the failed design, both eliminating numeric-bound const
recursion:

1. **Trunk-root discovery recurses STRUCTURALLY on the carrier.** A `Discover<Full,
   const POS>` trait with `impl for WuNil` (structural base) and `impl for
   WuCons<H, T> where T: Discover<Full, { POS + 1 }>` threads `POS` exactly like
   `RunTrunkSel`. At each position it const-gates the trunk-root test
   (`const { trunk_of(POS) == POS }`) and, when the position is a trunk-root,
   dispatches `run_one_trunk::<Full, POS>` on the FULL carrier (passed alongside
   as `Full`, since `run_one_trunk` walks from position 0; the structural receiver
   is the tail being scanned). `POS` is the structurally-threaded const generic, so
   `run_one_trunk::<POS>` still monomorphises per trunk-root (the proven mono).

2. **Phase ordering is a RUNTIME loop, not const recursion.** `for p in
   0..nphases` (nphases known from the const grouping) calls the structural
   discovery once per phase; the per-position phase test `phase_of(POS) == p` is a
   RUNTIME compare (p is the loop variable), not a `const {}` gate. Each trunk-root
   dispatches only in its own phase pass, so trunks run phase-ordered (correct
   cross-phase dependency order; trunks within a phase are column-disjoint, so
   intra-phase order is free). No const recursion on PHASE, so no wall.

Verified output for a 4-unit carrier (pos0 phase0 trunk0-root with member pos2;
pos1 phase0 trunk1-root; pos3 phase1 trunk3-root): dispatch order `[0, 2, 1, 3]`
= phase 0 trunks (trunk0 members 0,2 then trunk1 member 1) then phase 1 (trunk3
member 3). Phase-ordered, each trunk's members contiguous, each trunk once.

## Why phase order is required (not position order)

A position-order dispatch of trunk-roots is NOT dependency-safe: trunk A may
depend on trunk B (a B output read by an A member) while A's root position
(component-min) is lower than B's, so root-position order could run A before B.
Phases are the dependency layers; dispatching phase-outer guarantees every
producing trunk runs before any consuming trunk. The runtime phase loop is the
cheap correct ordering.

## Carry into the src CL

- Port `Discover<Full, const POS>` + the runtime phase loop onto the real
  `WuVals` carrier; `trunk_of` / `phase_of` are the shipped const fns
  (`plan/grouping.rs`), parameterised `<Wus, Stores, Witnesses, CU, CS, Adj>`, so
  the gates read `const { trunk_of::<..>(POS) == POS }` and the runtime
  `phase_of::<..>(POS) == p`. `nphases` = the shipped `phase_count::<..>()`.
- `run_one_trunk` delegates to the shipped `RunFiber` per member position (sketch
  071230 shape on the real carrier).
- The `Full: RunTrunkSel<0, POS>` bound holds for every POS over the real carrier
  (same as the per-fiber witness threading the existing walks use).
- Re-point `Scheduler::run` to `dispatch_all(&self.wu_values, phase_count, ..)`
  single-core; keep `run_parallel` on its current runtime-mask path (G2-Na
  re-points it onto these monos for real parallelism).
- The waist between phases is a no-op single-core; G2-N inserts the real
  `phase_barrier_arrive`.

## Conclusion

The const-range dispatcher is feasible and proven. No Step-11 wall: the mechanism
is structural recursion + a runtime phase loop, no numeric-bound const recursion.
Proceed to the doc CL + src CL.
