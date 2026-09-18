# Sketch A4-b: is the FiberCons nest wireable, making the A4 mask walk a shortcut?

**Date:** 2026-07-19
**Premise (hypothesis):** the unwired `RunPipeline -> RunPhase -> RunTrunk -> RunFiber`
nest over `PhaseCons`/`TrunkCons`/`FiberCons` can be wired into dispatch, giving a
per-fiber morsel loop INSIDE the walk. If so, A4's outer per-fiber loop over the flat
carrier is an avoidable F-fold re-walk and both A4 and A2b should be redone on the nest.
Leeway: exact (the question is binary).

**Outcome: FAILS. The nest is unwireable from flat registration, and the flat walk is
the canonical mechanism rather than a substitute for it.** A4's shape stands. The cost
finding below is real and stays.

## Why the question was worth asking

The pattern that motivated it is real and has now bitten this arc three times: a
canonical mechanism gets built, left unwired, and a flat or mask-based substitute
occupies its role while later work builds on the substitute and inherits its cost. Both
head+tail (`thread::Convergence`) and the `Fiber.head_tail` plan record went that way.
`FiberCons` looks identical from the outside: full machinery, task #670 titled "restore
per-fiber morsel locality" (precisely A4's concern), and zero references in
`scheduler/mod.rs` or `dispatch/trunk_gate.rs`.

The check that distinguishes the two cases is whether the thing is unwired because it
was forgotten, or unwired because it cannot be reached.

## The answer, from the recorded fork

`mock/research/202606071200_gate2-carrier-mechanism-fork.md` settles it. Building the
nested carrier type from the plan requires deciding, at the type level, which WU belongs
to which trunk. That is partition-by-key on column-disjointness, and it is inherently
negative at the boundary ("this WU does NOT share a write column, so close the current
trunk"). Sketch `202606061400` recorded the outcome: "partition-by-key at the type level
is inherently negative at the boundary, whether the key is a supplied tag or a derived
predicate. Both positive-only formulations wall (E0119) ... The door is closed."
`min_specialization` does not lift it; full `specialization` is forbidden by
`unstable-features.md`.

The decisive distinction: Sketch A (`202606070300`) proved the walk machinery devirts
when the nest is **hand-built**. It never proved the engine can **derive** the nest from
flat registration, and its own outcome admits that gap. So `FiberCons` is not a forgotten
mechanism. It is a proof-of-concept for an approach the carrier-materialisation mandate
replaced, retained because it proved the walk shape.

## And the flat walk is what canon asks for

Spec domain 17 (`:1564-1617`) puts the grouping in the plan and the dispatch structure in
the emitted per-core program: "the flattener emits a monomorphised function" per fiber,
with the grouping a runtime plan output. The design separates them cleanly. Grouping is
`block_diagonalise` / `group_fibers` / `compute_waists`, already shipped. The dispatch
carrier is the flat type-level walk. The grouping never becomes a carrier type.

So A4's outer per-fiber loop is not a mask-based emulation of a better available shape.
It is the only shape in which per-fiber windows are expressible on the mandated carrier,
and it serves canon's per-fiber locality intent (domain 12/14) through the structure
canon's domain 17 specifies.

## The cost finding survives, and is now a measurement rather than a suspicion

Serving per-fiber locality on a flat carrier costs re-walks, and that cost is inherent
rather than incidental. For a trunk holding F fibers, dispatch is called once per
(fiber, morsel). Each call walks that trunk's whole unit list, running only the current
fiber's units; the other fibers' units do not fold away, because `Member::IS` is const
per **trunk** membership, not per fiber, so they are rejected by a **runtime**
`dirty.bit(Pos::INDEX)` test (`dispatch/trunk_gate.rs:114-119`). With F fibers of m
morsels each over U trunk units, that is F x m x U runtime bit tests where the old
shared-window shape did m x U, for identical executed work.

It cannot be folded away inside this mechanism: fiber membership is a plan output, so no
const gate can express it. The FIXME at #340 (plan-baked member masks plus a
fiber-to-phase map) reduces the walks it is possible to skip, not this term.

Grouping fibers by shared window to collapse the passes was considered and is **rejected
on canon**: fibers sharing a window would then be dispatched together per morsel, putting
both fibers' columns live in the same window, which is exactly the L1 pressure the
per-fiber window was computed to avoid (domain 12/14). Strict fiber-at-a-time is the
point.

So the honest statement is that A4 buys per-fiber L1 locality and pays F-fold walk
overhead, and which dominates is a measurement, not an argument. **G2C-M must carry an
arm for it**: per-fiber windowed dispatch against the old shared-window shape, at fiber
counts of 1, 2, 4 and 8, on both a bandwidth-heavy and a compute-heavy fiber. If the
locality win does not cover the walk cost at realistic fiber counts, that is a finding
about the mechanism worth surfacing to op, not something to quietly absorb.

## Consequences

A4 and A2b stand as shipped; no redo. The `FiberCons` nest, `RunTrunk`, `RunPhase` and
`RunPipeline` are dead machinery under `no-legacy-shims-pre-1.0` on the same footing as
the head+tail types, and belong in G2C-0's deletion audit rather than left to read as an
unfinished feature for the next person who greps for per-fiber dispatch. G2C-M gains the
walk-overhead arm above.
