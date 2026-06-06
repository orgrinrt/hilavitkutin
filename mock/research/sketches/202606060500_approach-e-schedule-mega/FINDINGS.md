# Findings: Approach-E schedule-mega single-core body (Phase D / #340)

**Status: WORKS.** nightly-2026-05-28, release fat-LTO cgu=1.

This sketch follows `202606052130_rcm-order-into-static-body`, which proved the
column-capable inline `RunFiberCol` walk devirtualizes for a branching DAG with
a multi-column join and is order-agnostic, in one whole-range pass. The chosen
single-core direction there was the consolidation spec's Approach E
(schedule-mega, one body for the whole schedule). This sketch builds that body
and answers the two structural questions the body probe left open, plus the
within-level RCM bench.

## What the probe ran

A two-phase schedule against the real engine crates: phase 0 is the diamond
(BranchX: In->Xv, BranchY: In->Yv, JoinZ: {Xv,Yv}->Zv); phase 1 is NormW
(Zv->Wv). Both phases run morsel-outer inside one `#[inline(never)]`
`run_schedule_mega<const MORSEL, A, P0, W0, P1, W1>` whose only bounds are the two
`RunFiberCol` bounds. The phase boundary is the sequence point between the two
per-phase morsel loops. N = 131072, MORSEL = 1024 (a const generic).

Phase 1 is element-wise here and step-8 grouping would fuse it into phase 0; it
sits in its own phase only to exercise the multi-phase body structure (two
sequenced morsel loops), the codegen shape step-8 emits for a genuine barrier
(reduction / scan). The devirt result is independent of whether this phase 1
needs the barrier.

## What WORKS

1. Type-check with witnesses inferred at the call site. The named
   `run_schedule_mega` carries only `P0: RunFiberCol<A, W0>` and
   `P1: RunFiberCol<A, W1>`; the per-unit four-witness lists infer at the two
   call sites in `main`. This is the fix for the placeholder-witness problem the
   body probe hit when it tried a named helper with an `Empty` witness.

2. Morsel-outer dispatch. The walk runs inside a runtime morsel loop (call the
   same inline walk per morsel sub-range; `EngineCtx::project` takes the morsel
   and `each()` iterates exactly it). Correct for all 131072 records.

3. Multi-phase body. Two phases, each its own morsel loop wrapping its own inline
   walk, sequenced in one fn. Zv[i] == join(branch_x(i), branch_y(i)) and Wv[i]
   == norm(Zv[i]) for all records, in both the registration-order and the
   RCM-within-level-order phase 0.

4. Devirtualization end to end. `objdump` of `run_schedule_mega` (193
   instructions): zero `blr` (no indirect calls), zero `bl` (no helper calls at
   all), the const-generic MORSEL=1024 baked as `#0x400` (4 occurrences, also
   `Kj400_` in the mangled symbol), column access via register-offset indexed
   loads `ldr w21, [x11, x20]` / `str w21, [x10, x20]`, and the only `[sp`
   accesses are prologue/epilogue (SIMD body 0x8d0-0xb54 has none). The
   per-record body auto-vectorized (48 vector ops). Zero surviving
   run_fiber_col / RunFiberCol / fiber_shim / CollectFiber symbols in the binary.

   (The disasm_5check check-2 text pattern looks for the `lsl #scale` addressing
   form, which 4-byte w-register loads do not emit; the addressing is still
   fully indexed. Check-text gap, not a devirt failure. Worth noting for the SRC
   slice's perf-gate wiring: check 2 needs a w-register-aware pattern.)

## The within-level RCM bench

Min of 500 (warmup 50), reproducible across runs: registration-order phase 0
(BranchX first) ~45.8us (~0.349 ns/rec); RCM-order phase 0 (BranchY first,
i.e. the plan picking the heavier branch ahead of the lighter among the two
independent equal-depth units) ~44.9us (~0.343 ns/rec); rcm/reg ratio ~0.98.

So the within-level order of independent equal-depth units is NOT perf-neutral on
single core: a small (~2%) but reproducible delta, here favouring BranchY-first.
Not negligible, not large. The magnitude is workload-dependent (this is a tiny
per-record kernel that vectorizes; a memory-bound or heavier-kernel workload
could differ in sign or size). Two consequences for the SRC slice:

- Realising the plan's RCM-reordered order is worth doing: it is free to do (the
  cons-list is just built in that order) and the order is not perf-neutral. This
  is NOT the rejected "defer RCM" punt; RCM is consumed by building the
  cons-list in its order.
- The definitive per-workload number belongs in the #664 perf-gate suite once the
  real plan output drives the order, not in this sketch.

## What this settles for the single-core direction

The Approach-E schedule-mega single-core body is feasible and devirtualizes. No
type-level fiber partition is needed on one core. The body is: one
monomorphised fn, per-phase morsel loops sequenced at phase boundaries, the
inline walk per morsel, morsel size a compile-time constant. The single-core
assembly problem reduces to "build the flat cons-list(s) in the plan's
RCM-reordered topo order", with a build-time validation that the order is
topo-valid. The within-level RCM order is a small benched refinement, not a
structural blocker.

## What this does not settle

- The genuine cross-record phase boundary (reduction / scan) needs the non-nil
  `AccumProject` tie, modeled here only structurally. That tie is the deferred
  SRC-time residual (MEMORY LATEST-55): confirm an accum-writing unit dispatches
  through the walk when driven from a witnesses-pinned entry. Low risk
  (CollectFiber already resolves the same tie), but it is the one walk-level
  thing this sketch did not exercise.
- The GATE-2 parallel path needs the type-level per-fiber partition (trunks map
  to cores). Out of scope for single core; on one core the partition collapses
  to the morsel/phase boundaries this sketch already runs as in-body control
  flow.

## Next

Rewrite the #340 DOC CL on this confirmed mechanism: the schedule-mega
single-core body (per-phase morsel loops, inline walk, const morsel, RCM-ordered
flat cons-list, build-time topo-valid validation), deleting the
CollectFiber/FiberSlot/fiber_shim fn-pointer path (no-legacy-shims-pre-1.0).
Lock, then SRC CL TDD-red-first. Phase C prerequisite: consume the plan's
discarded RCM-reordered output into the dispatch order (memo S3). `#666`
(arvo `dev` HEAD bump + waist_detect 2-arg fix) lands first on its own branch as
a GATE-1 prerequisite. The accum-tie residual is confirmed during SRC TDD.
