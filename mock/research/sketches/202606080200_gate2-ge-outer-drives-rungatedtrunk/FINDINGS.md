# Sketch findings: GATE-2 G-e outer dispatcher driving the SHIPPED RunGatedTrunk

**Date:** 2026-06-08
**Round:** 202606080100 (GATE-2 G-e const-gated per-trunk dispatch)
**Sketch:** `dispatcher.rs` (`rustc +nightly-2026-05-28 --edition 2021 -O`, runnable)
**Outcome:** WORKS

## Why this sketch exists (corrects 080130)

Sketch 202606080130 proved an outer dispatcher, but it REINVENTED the inner
per-trunk walk (`run_one_trunk` / `RunTrunkSel` keyed by a single `const POS`).
The engine already ships that walk: `dispatch/trunk_gate.rs::RunGatedTrunk`
(round 2a), keyed by `const PHASE` + `const TRUNK` over a Peano position witness
(`Here` / `There<..>`), whose doc states the outer "(phase, trunk) dispatcher in
phase order, and the `Scheduler::run` re-point, are round 2b" (= G-e).
use-the-stack-not-reinvent: G-e must DRIVE `RunGatedTrunk`, not a parallel walk.

Reusing the shipped walk surfaced the real un-proven seam, which 080130 did not
test: enumerating (phase, trunk) and calling a per-trunk mono forces
materialising a trunk/phase id into a CONST-GENERIC ARGUMENT, and `trunk_gate.rs`
itself records a generic-const-expression overflowing the trait solver when
normalised through this recursion.

## The fork resolved

Two coupled questions:

1. Inner keying. `RunGatedTrunk` carries `const PHASE` + `const TRUNK`. Supplying
   `PHASE = phase_of(POS)` to the per-trunk mono is always a GCE in const-arg
   position. A trunk lies wholly in one phase, so TRUNK-ONLY keying (gate on
   `trunk_of(pos) == TRUNK` only) drops the redundant `PHASE` const and removes
   that GCE. Phase ORDER is the runtime outer loop (proven 080130), not a const
   on the walk.
2. Outer enumeration. A `const POS` outer walk threading `{POS + 1}` passes
   `TRUNK = POS` by IDENTITY (no GCE), but re-introduces the `{POS + 1}`
   recursion `trunk_gate.rs` reports overflowing under a heavy gate. A Peano
   outer walk would avoid `{POS + 1}` but force `TRUNK = {Pos::INDEX}` (a GCE
   const-arg).

This sketch tested the cleanest candidate: TRUNK-ONLY Peano inner walk (shipped
RunGatedTrunk shape minus the redundant PHASE const) driven by a `const POS`
outer walk, under a representative-weight const member gate (a const fn running a
union-find over fixed read/write column-mask arrays, approximating
`is_member` -> `compute_trunks`). It COMPILES and orders correctly.

## The working shape (port this)

- Inner: `RunGatedTrunk<const TRUNK, Pos>` over `WuCons` / `WuNil`, recursion
  threads `There<Pos>` (a type, no const arithmetic; the shipped Peano idiom).
  Per cell, gate `Member::<Pos, TRUNK>::IS` (an associated const = `trunk_of(
  Pos::INDEX) == TRUNK`), run the head through `RunFiber::run_head` when true,
  always recurse the tail. Identical to the shipped impl with the `const PHASE`
  parameter and its half of the membership test removed.
- Outer: `Discover<Full, const POS>` over the carrier, recursion threads
  `{ POS + 1 }`, bound `Full: RunGatedTrunk<POS, Here>`. Per cell, const-gate
  `const { trunk_of(POS) == POS }` (POS is a trunk-root) and, inside, the runtime
  `phase_of(POS) == p` match (fire this trunk only in its phase's pass); dispatch
  `full.run_trunk(..)` (= `RunGatedTrunk::<TRUNK = POS, Here>`) on the FULL
  carrier. POS reaches the inner `TRUNK` by identity: NO GCE const-arg.
- Top: runtime phase loop `for p in 0..nphases { carrier.run(carrier, p, ..);
  waist }`. `nphases` = the shipped `phase_count::<..>()`. Single-core waist is a
  no-op; G2-N inserts the real `phase_barrier_arrive`.

Verified order for the 4-unit fixture (trunk grouping computed by a same-phase
union-find over the mask arrays, NOT hardcoded): `[0, 2, 1, 3]` = phase 0 trunk0
(members 0, 2) then trunk1 (1), phase 1 trunk3 (3). Phase-ordered, each trunk's
members contiguous, each trunk once.

## Fixture bug found (and what it confirms)

First run produced `[0, 2, 3, 1]`: the const `trunk_of` had dropped the
same-phase guard the real `compute_trunks` has (`if same_phase && conflict`), so
it merged pos2 (phase 0) and pos3 (phase 1) into one trunk via their shared
column. The dispatcher then correctly ran that (inconsistent) grouping. Adding
the same-phase guard to the fixture fixed it. The lesson for the src port: the
shipped grouping already carries the same-phase guard
(`grouping.rs::compute_trunks`), so trunk-only keying is sound. it does not
re-merge across phases, because trunks are computed per-phase upstream.

## Carry into the src CL

- Refactor `dispatch/trunk_gate.rs::RunGatedTrunk` to TRUNK-only keying: drop the
  `const PHASE` parameter and the `phase_of` half of `is_member`. The gate
  becomes `trunk_of::<..>(Pos::INDEX) == TRUNK`. Add a TRUNK-only `is_member`
  companion (or a `trunk_of(Pos::INDEX) == TRUNK` const) in `grouping.rs`.
  Pre-1.0 churn: delete the PHASE-keyed form, do not alias it.
- New outer dispatcher (e.g. `dispatch/trunk_dispatch.rs`): the `Discover<Full,
  const POS>` walk + runtime phase loop, parameterised
  `<Wus, Stores, Witnesses, CU, CS, Adj>` so the gates read the shipped const
  `trunk_of` / `phase_of` / `phase_count`. The `Full: RunGatedTrunk<POS, Here>`
  bound holds for every POS over the real carrier.
- Re-point `Scheduler::run` to the outer dispatcher single-core; keep
  `run_parallel` on its runtime-mask path (G2-Na re-points it onto these monos).
- VERIFY: objdump each per-trunk mono zero `blr` + member-only; output
  bit-identical to the flat `RunFiber` walk across the engine suite; #664
  element_wise GREEN no-regress (branching/accumulator stay RED, G-e is
  single-core).

## Conclusion

G-e is fully proven end to end. The clean shape is trunk-only keyed Peano inner
(shipped RunGatedTrunk minus the redundant PHASE const) + const-POS outer walk +
runtime phase loop, with zero GCE const-args and the `{POS + 1}` outer recursion
surviving the weighted inner instantiation. Proceed to the doc CL + src CL.
