**Date:** 2026-06-07
**Phase:** course-correction (chart-the-path step 11: a step cannot be proven as the roadmap assumed)
**Scope:** GATE-2 carrier-materialisation mechanism. How the isolated column-disjoint trunks become compile-time per-core dispatch programs.
**Source:** canonical spec domain 17 + 20; proven sketch 202606061400 (D1b type-level fiber partition); proven sketch 202606070300 (Sketch A, hand-built nest); FuseCarrier doc; an independent code-architect read (2026-06-07); GATE-2 rechart 202606070100 + roadmap r3 202606070200.

This document records a drift found while grounding GATE-2 stage G2-0c, and the design fork it opens. It does not change any source; it charts the path so the build does not proceed on a walled mechanism.

## What G2-0c assumed, and why it walls

Roadmap r3 (`202606070200_engine-roadmap-r3-gate2.md` line 25) names G2-0c: "`build()` emits `PhaseCons<TrunkCons<FiberCons<WuCons>>>` from `PhaseBoundaries` + `BlockPartition` + fiber grouping, replacing the flat `WuVals` walk." It marks G2-0c "UNPROVEN as a whole (sketch A gates it)." Line 57 pre-designates the failure mode: "trunk-disjointness the types cannot express ... is a roadmap-changing finding for op, not a thing to route around."

The mechanism G2-0c needs is: turn the flat, registration-order, heterogeneous cons-list `WuVals = WuCons<SX, WuCons<SY, WuCons<SZ, WuNil>>>` into a nested type `PhaseCons<TrunkCons<FiberCons<WuCons<...>>>>` whose shape encodes the plan's phase/trunk/fiber grouping.

That shape IS the grouping. For heterogeneous WU types, the nested type's structure (how many TrunkCons cells, which WU type in which leaf) is exactly the partition. So building it requires deciding, at the type level, which WU belongs to which trunk. That decision is partition-by-key (column-disjointness of the WU's write set against the running trunk's accumulated write set), which is inherently negative at the boundary ("this WU does NOT share a write column, so close the current trunk and open a new one").

Two independent first-party findings confirm this needs forbidden full `specialization`:

1. Sketch `202606061400_d1b-typelevel-fiber-partition` OUTCOME: "partition-by-key at the type level is inherently negative at the boundary, whether the key is a supplied tag or a derived predicate. Both positive-only formulations wall (E0119). ... the fiber grouping is a runtime graph algorithm, not any kind of type-level fold. The door is closed."
2. `mock/crates/hilavitkutin/src/dispatch/fusion.rs` `FuseCarrier` doc: folding a single chain compiles with no E0119 precisely *because* it does not partition; N-way partition does not have that property.

`min_specialization` (allowed) does not lift the wall: it permits default bodies on disjoint inherent impls, not the column-intersection-based dispatch the partition needs. Full `specialization` is forbidden (`unstable-features.md`).

The expert read floated a "hybrid: nested TYPE fixed at compile time, grouping of VALUES into nest slots at runtime." That is incoherent for heterogeneous WUs: runtime-determined split points cannot produce a statically-known heterogeneous nested type. It holds only for the degenerate single-trunk-per-phase wrap (the whole flat carrier as one fiber), which is exactly the single-fiber GATE-1 bench case and adds nothing over the flat walk.

Conclusion: `build()` cannot derive a genuine multi-trunk nested carrier *type* from flat registration. Only the degenerate 1-phase/1-trunk/1-fiber wrap is derivable. **G2-0c as specced is not buildable as a pure type-level / runtime mechanism.**

## What the canonical design actually says

The 061400 OUTCOME already states the canonical resolution, citing the spec: "the canonical mechanism is a codegen flattener that EMITS the per-core program, with the grouping COMPUTED by the plan (the shipped `group_fibers`, a runtime graph walk) ... The dispatch STRUCTURE is the flat per-core carrier, which is type-level and devirtualises."

Canonical spec domain 17 (`:1564-1617`) agrees: "the flattener emits a monomorphised function" per fiber; "compiled per-core dispatch: each physical core gets a monomorphised function encoding its entire pipeline" with "the WU sequence per fiber (devirtualised LOCAL slices)" and morsel bounds / record ranges baked in. `hilavitkutin-build`'s stated role (CLAUDE.md) is exactly this: "LLVM passes, MIR manipulation, cfg emission." The grouping is a *runtime plan computation*; a *codegen flattener* consumes it to emit per-core programs. The grouping never becomes a nested carrier *type*.

So the design separates cleanly:
- Grouping (phase/trunk/fiber membership): runtime plan output (`block_diagonalise`, `group_fibers`, `compute_waists`). Already shipped in `ExecutionPlan`.
- Dispatch carrier: type-level, devirts. In the canonical design it is the per-core program the *flattener emits*; in the GATE-1 realisation it is the flat type-level `RunFiber` walk.

## The drift

The GATE-2 rechart (`202606070100`, 2026-06-07) and r3 reframed the mechanism as "build the nested `PhaseCons` carrier type from the plan inside `build()`." That is the type-level-partition path 061400 already closed. The rechart's "hybrid: the carrier TYPE carries the plan's result" was never cashed out, and cannot be for heterogeneous trunks. Sketch A proved the *walk machinery* (`RunPipeline -> RunPhase -> RunTrunk -> RunFiber`) devirts when the nest is *hand-built*; it did not prove the engine can *derive* the nest. Its own outcome admits the gap.

Per `canonical-design-outranks-intermediate-rounds.md`: when an intermediate round (the rechart/r3) conflicts with the canonical design (spec domain 17 + the earlier proven 061400), the canonical design wins; the intermediate round is the thing to fix.

## What op's live 2026-06-07 correction did and did not settle

op corrected the model to "isolated column-disjoint TRUNKS, one per core, zero sync, joined by waists + bridges; trunks established before parallelising," and called the "worker walking the flat carrier gated by a record-range predicate" framing DRIFT.

That correction is about the parallelism MODEL (isolated trunks vs record/morsel partition), and it stands. It is mechanism-agnostic: a codegen flattener emitting isolated per-trunk programs realises the isolated-trunk model directly. What the correction did NOT settle is the MECHANISM that materialises those trunks as compile-time programs. The rechart picked "type-level nest in `build()`" for that mechanism, and that pick walls.

## The fork (op's call)

How does GATE-2 materialise the isolated-trunk per-core dispatch programs?

A. **Codegen flattener (canonical; recommended).** A `hilavitkutin-build` step (and/or a proc-macro) emits the per-core / per-trunk monomorphised programs from the runtime grouping. Matches spec domain 17 + 061400's resolved conclusion. The walk traits already shipped (`RunTrunk` / `RunPhase` / `RunPipeline`, devirt-proven by Sketch A/B) become the emitted programs' bodies. Cost: build out the flattener, currently skeleton (`dispatch/mod.rs` `codegen_*` stub to `todo!`-shaped; `hilavitkutin-build` PoC-only). Largest, but the design-truth path.

B. **Flat carrier + per-core trunk-ownership gate.** Keep one flat type-level carrier; each core walks it gated by a trunk-ownership predicate (the proven devirt-clean predicated-branch shape, zero blr, same as the E7 dirty gate / RCM guard). The grouping stays runtime params. Cheapest. Risk: a flat-carrier-gated variant was the framing op called drift, though op's objection was specifically the record-range partition, not trunk-ownership gating; needs op to confirm whether trunk-ownership gating is acceptable.

C. **Macro-declared nesting (`app!`, #295).** A proc-macro emits the nested carrier type from registration at expansion (a macro can partition; it is codegen). Consumer API gains a macro; the macro must run the grouping at expansion, duplicating plan logic, and sits in tension with auto-ordering (op call a).

D. **Degenerate-only G2-0c now, defer the mechanism.** Land G2-0c as the trivial single-trunk wrap (output-equiv, single-fiber benches pass), route `run` through `RunPipeline`, and defer the real multi-trunk mechanism to when it bites at G2-N. Lets "single-core sectioning" land but enables no real parallelism and risks entrenching the walked mechanism.

Recommendation: A. It is what the spec and the proven 061400 both name, realises op's isolated-trunk model directly, and keeps the shipped devirt-clean walk traits as the emitted bodies. B is the cheap fallback if op wants to avoid resurrecting the flattener now. The build should not proceed on the rechart's type-level-nest mechanism (C-without-a-macro / the "hybrid"), which walls.

## Immediately fork-independent work (proceeds regardless)

- E3 waist-barrier hardening (gen / sense-bit for multi-episode reuse): every mechanism has waists between phases.
- Thread-pool spawn-once parking lifecycle (the pool plumbing, not the worker body): the worker body is fork-dependent, the pool lifecycle is not.
- #636 (dep_graph bench reconcile), #345 E8 half (runtime plan-recompute-on-resource-swap, domain-22): orthogonal to the dispatch carrier.
