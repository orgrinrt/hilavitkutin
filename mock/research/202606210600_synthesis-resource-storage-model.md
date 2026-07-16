# Synthesis: resource storage-model pressure-test

**Date:** 2026-06-19
**Round:** 202606210600. Synthesises five deliverables + the canonical addendum + the three topics.
**Deliverables synthesised:**
- `202606210600_resource-storage-model-canonical-addendum.md` (the model, R5 reading)
- `202606210600_analysis-self-storage-model.md` (main agent)
- `202606210600_expert-architect-storage-model.md` (architecture + impl path)
- `202606210600_expert-perf-storage-model.md` (perf, soundness, bench gaps)
- `202606210600_expert-alternatives-storage-model.md` (model survey)
- `202606210600_expert-loimu-heritage-storage-model.md` (loimu heritage, sonnet)

## Convergence (what all sources agree on)

1. **Resource = handle, NOT inline-value blob.** Canonical (R5:1689 "pointer indirection to
   external slab storage, NOT inline"). Corroborated by loimu (resources hold handles to
   storage; loimu emphatically forbids resources holding collection data inline) and by the
   fact that `hilavitkutin-api/src/storage.rs` already ships a `Decompose { type Leaves }`
   trait. The shipped `DrainStores` one-record-opaque-blob (`bindings.rs:342`,
   `cs.reserve::<T>(id, USize(1))`) is DRIFT: it ignores the `Decompose` trait that exists.
   This is the firm, lock-now conclusion.

2. **The noalias win is provenance-separation + stack-local snapshot, SEPARABLE from
   decomposition.** Spec 1695-1704 + 1714-1724. `ResourcePtr<T>` / `ColumnPtr<T>`
   (provenance.rs) already establish the provenance distinction; the win needs the dispatcher
   to SNAPSHOT the resource pointer(s) to the stack before the morsel loop. Two findings:
   (a) the architect + perf both show this win does NOT require per-member decomposition (a
   blob with separate-provenance + snapshot gets it too); (b) the perf expert found the
   snapshot is **not currently implemented** (`resolve_resource` reads through the ptr,
   `engine_ctx.rs:1258`, no snapshot), and the addendum's "pointers stack-local, values in
   unified store" is a *different* mechanism that may FORFEIT the win if read+write share the
   store's provenance. The canonical addendum over-claimed by bundling the measured win with
   decomposition. The noalias guarantee should be an explicit architectural INVARIANT
   ("handle store never aliases value columns" + "snapshot resource ptrs before the loop"),
   not a measured outcome (loimu blind-spot F).

3. **Shape-bound sharing is the contested, unmeasured part.** Its only benefit (suppressing a
   column-count explosion) is moot for the few-small-resources workload against a 256-slot
   store (alternatives + perf); it adds the most machinery, reintroduces the LLVM-aliasing
   ambiguity provenance-separation exists to kill, and risks cross-core false-sharing (a
   shared column written by resources owned by different cores). loimu supplies the missing
   WHY ("batch by stored shape") but for its many-node UI/game workload, which does not
   transfer to few-resource pipelines. Convergent lean: type-unique per-resource columns;
   shape-bound deferred. **Op decision (2026-06-19): resolve this by BENCH, not assertion.**

## Locked-now (not bench-gated)

- Resource is a handle; the `DrainStores` blob is drift; the fix wires `DrainStores` through
  `<T as Decompose>::Leaves`. (Direction firm; the exact per-member layout is what the bench
  picks.)
- The noalias win requires the stack-local snapshot, which must actually be implemented, and
  is stated as an architectural invariant.
- The canonical addendum is revised (see below).

## The bench (the resolution for the contested axis)

A research bench comparing the candidate storage models head-to-head, per op. Variants:

- **V0 baseline:** the current one-record-blob (no snapshot): the drifted status quo.
- **V1 blob + stack-snapshot** (architect Alternative A): blob value, separate `ResourcePtr`
  provenance, dispatcher snapshots to stack before the loop. Minimal change; the proven-win
  conservative end.
- **V2 type-unique decomposed columns** (the converged lean / architect Alternative B): each
  resource's `Field`/`Seq`/`Map` members in their own type-unique columns; handle = per-leaf
  `ResourcePtr<F>`; const-derived ids; stack-snapshot.
- **V3 shape-bound shared columns** (the addendum's contested model): members share a column
  by stride across resources; resource-slot indexing.
- **V4 loimu-style full type-erasure via shaping:** type-erased shaped stores (arbitrary bits
  at uniform stride per shape), backcast on access, the loimu "batch by stored shape" model
  adapted to static composition (no runtime views).
- (Optional **V5 runtime handle table:** a flat runtime `[StoreId; MAX_LEAVES]` per resource,
  resolved at runtime rather than const: the "more runtime-y" end op named.)

**Measured axes** (from the perf expert's four arms + the alternatives/false-sharing flags):
- A. register-residency / noalias: does each variant keep resource members in registers across
  the morsel loop? (objdump for reloads; the 1.28-1.40x is the signal to reproduce or refute.)
- B. intra-resource read locality: cost of "read this whole resource" (scattered shared-column
  vs contiguous blob vs decomposed).
- C. column-count / slot-table: store-id pressure + plan-stage cost as resource count scales.
- D. Seq/Map windowing + Replaceable swap: collection-member access under the morsel window,
  and swap cost per model.
- E. cross-core false-sharing: shared-column writes from resources owned by different cores
  (V3 specifically).

Deciding signal: per axis, the variant that wins without regressing the others. The bench is a
durable `benches/` artifact + a findings doc; it picks V1..V4(/V5) for the real impl. Until it
runs, the drift-fix lands the firm direction (handle, snapshot, Decompose seam) on whichever
layout the bench selects.

## Addendum revisions (apply to `202606210600_resource-storage-model-canonical-addendum.md`)

1. The benched win is provenance-separation + stack-local snapshot, achievable with a blob too;
   decomposition is additive (partial-member access, SoA for Seq/Map), not the win's source.
2. Shape-bound sharing is ONE option on the sharing axis, not a committed design; it is
   bench-gated against type-unique + blob + loimu-erasure.
3. Add the explicit noalias invariant rule (handle store never aliases value columns; snapshot
   resource ptrs before the morsel loop) and note the snapshot is currently unimplemented.
4. Add loimu's "batch by stored shape" as the rationale IF shape-bound is later chosen, and
   inherit loimu's signals-not-records boundary into the adapt/Virtual layer.

## Op decisions (2026-06-19, resolved)

1. **Bench all six V0-V5.** Full spread (baseline, blob+snapshot, type-unique decomposed,
   shape-bound shared, loimu type-erasure-via-shaping, runtime handle-table).
2. **Hold all storage impl until the bench decides.** Do NOT ship V1 interim; land nothing
   storage-side until the bench picks a winner, then implement that. (A3 morsel-sizing + the
   drift-fix wait on the bench result.)
3. **Bench lives in the engine `benches/`** as a permanent re-runnable comparison artifact.

So the next action is to BUILD the six-variant comparison bench in `benches/`, run it, write a
findings doc, pick the winner, and only then implement the drift-fix (handle + snapshot +
Decompose seam) on the winning layout. Nothing storage-side lands before that.
