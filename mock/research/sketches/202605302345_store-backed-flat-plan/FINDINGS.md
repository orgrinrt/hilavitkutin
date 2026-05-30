# Findings: store-backed flat-CSR execution plan

**Date:** 2026-05-30
**Toolchain:** nightly-2026-05-28
**Context:** unified-columnar-storage arc, round 202605302330 (store-backed
plan). The DOC locked off an architect review alone; this sketch is the
de-risk the topic's own process called for ("a sketch under
`mock/research/sketches/` de-risks the representation ... before the DOC CL
locks") and that got skipped. Written before the SRC cut because re-reading
the plan module surfaced that `project_fiber_components`
(`plan/steps.rs:510`) itself materializes the ~8.4 MB nested
`[Phase;32]->[Trunk;32]->[TrunkComponent;32]xFiber` tree on the stack and
returns it by value, so the architect's "step functions unchanged, only
final assembly changes" model was incomplete: the flatten must reach into
the step function's output shape, not just the runner's final assembly.
**Outcome:** WORKS. Full store-backed cut is feasible in one coherent SRC.

## Hypothesis

The execution plan can be reshaped from the 11 MB nested const-array tree
into a flat-CSR representation that is (a) tiny on the stack (~10 KB scratch),
(b) store-backed as a small `Copy` handle (meta-`StoreId`s + live counts)
over a `ColumnStorage`, and (c) owned by a `Scheduler` that is `Send`/`Sync`
despite the store being `!Send`/`!Sync`, via a documented
frozen-between-commit-and-replan invariant.

## Result: WORKS

`cargo +nightly-2026-05-28 run` prints (size figures are deterministic):

```
=== PART 1: dissolve ===
nested ExecutionPlan (dominant field): 9191944 bytes (8.77 MB)
flat CSR scratch (stack, all plan-wide caps): 10792 bytes (10.5 KB)
store-backed PlanHandle: 20 bytes
=== PART 2+3: store-backed round trip ===
plan handle: phases=1 trunks=1 fibers=3 fiber_units=20
total fiber units read back from store: 20
empty plan: fibers=0 ok
replan on same store: 40 units -> 5 fibers, recovered 40
=== PART 4: Send/Sync over a !Send store ===
Scheduler<ArenaStore>: Send + Sync (asserted at compile time)
```

The four open questions from the topic's "Open questions" section, each
resolved:

1. **Handle shape.** `PlanHandle` is a 20-byte `Copy` struct: the live
   counts (phase/trunk/fiber/fiber-unit/fiber-column). It carries no data and
   no lifetime. The meta-`StoreId`s are a FIXED enumeration (`plan_col::*`
   constants), not threaded through the handle, because the plan columns are
   a closed set. A dynamic-id handle would carry the ids; the closed set
   makes the const namespace strictly simpler.

2. **Reserve timing.** Two-pass confirmed. The step chain builds flat scratch
   that ALREADY holds the live counts (they emerge as the chain runs). Pass 1
   reserves each column by its live count; pass 2 copies the scratch prefix
   in. No grow-on-demand needed, no re-reserve churn. The scratch is the
   single source of counts.

3. **How the chain writes columns.** It writes flat scratch arrays
   (`fiber_units[cursor] = ...`, `fibers[fid] = FlatFiber { unit_offset, ... }`),
   CSR-style: fibers hold `(unit_offset, unit_count)` into a flat
   `fiber_units` column; trunks hold `(fiber_offset, fiber_count)`; phases
   hold `(trunk_offset, trunk_count)`. The store columns are then a mechanical
   copy of the scratch prefixes. No typed write-cursor over the store is
   needed; the scratch IS the cursor.

4. **`Send`/`Sync`.** `unsafe impl<CS: ColumnStorage> Send/Sync for
   Scheduler<CS>` compiles and the assertion `assert_send_sync::<Scheduler<
   ArenaStore>>()` passes, even though `ArenaStore` holds `*mut u8` and is
   `!Send`/`!Sync`. The SAFETY argument: plan columns are written only under
   `&mut self` (commit/replan, exclusive) and frozen for the frame; dispatch
   reads through `&self` and never mutates; the raw provider pointers are
   never exposed for cross-thread mutation. A `&Scheduler` handed to per-core
   dispatch closures observes only immutable, fully-initialised columns.

## What this settles for the decomposition

The store-backing threads through cleanly and is cheap once the shape is
flat. The flatten is the load-bearing change (it touches the step function
output shape and dissolves the monolith); the store-backing is a thin layer
on top (reserve + copy + handle + a CS field on the scheduler). Because both
compile and round-trip in the sketch, the **full store-backed cut is feasible
as one coherent SRC**, matching the locked DOC. No need to split flatten from
store-back into separate rounds. (Fallback if a real-impl wall appears below:
land the flatten alone first, which on its own dissolves the overflow and
unblocks #647, then store-back in a follow-on round, reconciling the DOC.)

## What the stub does NOT settle (real-impl checkpoints for the SRC)

- **`compute_execution_plan` signature ripple.** Today it is a free fn
  `fn compute_execution_plan<D>(inputs) -> Outcome<ExecutionPlan<D>, PlanError>`.
  Store-backing needs `&mut CS` to reserve. Either it becomes a method on
  `Scheduler` or takes `&mut impl ColumnStorage`. Every caller (tests, the
  meta-pipeline entry) updates. Grep callers before reshaping the signature.

- **Meta-`StoreId` namespace.** The sketch uses `StoreId(0..5)`. The real
  `StoreId` namespace is shared with consumer-declared stores. Plan columns
  need ids that cannot collide with consumer store ids: either a reserved
  high base offset, or a SEPARATE meta `ColumnStorage` the scheduler owns
  distinct from the consumer data plane. Decide in the SRC; the separate-meta
  -store option is cleaner (no offset arithmetic, no collision surface).

- **Real `Scheduler` Send/Sync against its actual bounds.** The sketch's
  blanket `unsafe impl<CS> Send for Scheduler<CS>` is sound only because
  `ArenaStore` holds nothing but plan columns. The real `Scheduler` carries
  the WU tuple, `M: MemoryProvider`, and existing fields; adding a `!Send` CS
  field may break a prior auto-derive. Confirm whether the real scheduler
  already has an explicit Send/Sync impl or auto-derives, and scope the
  unsafe impl to the real bounds (the frozen-plan argument covers the plan
  columns; any OTHER concurrently-mutated state would need its own argument).

- **head_tail column.** The sketch flattens `head_tail` to a flag byte. The
  real `Maybe<HeadTailConvergence>` SoA-splits into a presence flag column +
  parallel convergence-field columns, OR stays a small inline `Copy` field on
  `FlatFiber` (it is ~56 bytes). Inline keeps `FlatFiber` a single column;
  decide by whether 56 bytes per fiber slot is acceptable (64 fibers * 56 =
  3.5 KB, trivial). Inline is simpler; lean inline.

- **`unit_meta` / `morsel_sizes` / `rcm_order` / `column_class` / `dirty`.**
  These are already-flat fields, a few KB combined. The topic leans
  flatten-all-into-store for uniformity. The sketch only modelled the
  phase/trunk/fiber tree (the dominant term). Moving the small flat fields to
  store columns is mechanical and uniform; do it in the same SRC.

## Verdict

The store-backed flat-CSR plan is feasible, dissolves the 8.77 MB dominant
field to a 10.5 KB scratch + 20-byte handle, round-trips through a
`ColumnStorage`, and the `Send`/`Sync` lift over a `!Send` store holds with a
stateable invariant. Proceed to the SRC as one coherent cut: define the flat
types + handle + meta-store, rewrite `project_fiber_components` /
`fiber_grouping_from_trunks` / the runner to the flat shape, wire the
`Scheduler` CS field + Send/Sync, TDD that `compute_execution_plan::<
DefaultPlanDims>` no longer overflows and round-trips. Watch the four
checkpoints above (compute signature, meta-id namespace, real Send/Sync
bounds, head_tail shape).
