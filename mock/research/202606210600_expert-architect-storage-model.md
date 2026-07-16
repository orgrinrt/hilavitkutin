# Expert architect analysis: resource storage model

**Date:** 2026-06-19
**Deliverable for:** round 202606210600 pressure-test (`mock/design_rounds/202606210600_topic.storage-model-pressure-test.md`).
**Note:** authored by the `feature-dev:code-architect` agent; the agent could not persist the file from its own context, so the main agent transcribed its analysis verbatim here (HTML entities unescaped). Content unchanged.
**Oracle sources:** consolidation spec R5 lines 535-566, 1682-1749; `bindings.rs`; `hilavitkutin-api/src/storage.rs`; `hilavitkutin-api/src/store.rs`; `resource/provenance.rs`; `hilavitkutin-providers/src/storage.rs`; loimu unified-storage discussion; canonical addendum `mock/research/202606210600_resource-storage-model-canonical-addendum.md`.

## Executive position

The handle + per-member + shape-bound + unified-store model is sound w.r.t. the engine's no_std/no_alloc/monomorphised-dispatch goals and is compatible with the `Decompose` trait the API already ships. It is not yet the best-evidenced design because its central performance claim is more limited than the addendum presents, and one of its three distinct mechanisms (shape-bound sharing specifically) remains unbenched with an unresolved locality cost. Adopt the model with shape-bound sharing treated as a tunable axis rather than a fixed commitment.

## Part 1. Soundness and optimality

**The benched win is separable from the decomposition question.** Spec 1695-1701 attributes the 1.28-1.40x to (a) resource pointers having distinct provenance from column pointers, and (b) the dispatcher snapshotting accessed resource pointers to the stack before the morsel loop. Neither requires per-member decomposition. The same win is achievable with the current one-record blob per `Resource<T>` if the blob's pointer has separate provenance. `ResourcePtr<T>` / `ColumnPtr<T>` (provenance.rs) already establish the type-level provenance distinction. The addendum conflates the measured win with decomposition by presenting them together.

Per-member decomposition has a separate motivation: partial-member access (a WU reading one field does not pull the full blob), column-level morsel windowing over member columns, and struct-of-arrays expansion for multi-field types. Real benefits, unmeasured against the blob baseline.

**Shape-bound sharing: unresolved locality cost.** A WU accessing resource A's `Field<u32>` member must touch the shared column at `resource_a_slot` within a column interleaved with all other resources' `Field<u32>` entries: scattered access (every 1-in-N record, or a slot-lookup table), worse locality than the blob's contiguous data. Also: two WUs in one fiber both accessing resources that share a `Field<u32>` column read/write through what appears to LLVM as the same base pointer (`column_ptr::<u32>(shared_field_id)`), complicating the noalias argument. Neither benched. Shape-bound has value only when type-unique columns would exhaust the `StoreId` space; for typical workloads (4-8 resources) it does not.

**Seq/Map + morsel windowing.** `Seq<T, const N: Cap>` / `Map` are const-sized: no runtime resize, so the "resize" worry dissolves; the budget uses `N*size_of::<T>()` per member column. The real case is `Replaceable` swap: under the handle model a swap is a handle update (point the resource id's handle entries at new column ids) plus a member-by-member copy, not an in-place blob memcpy. Swap semantics need explicit spec.

**Aliasing/soundness under the unified store.** Handle member pointers are `ResourcePtr<F>` per leaf; regular columns are `ColumnPtr<T>`; distinct newtypes over `NonNull` = distinct provenance. Holds as long as member columns reserve distinct `StoreId`s from regular columns. Soundness gap: `DrainStores` calls `cs.reserve::<T>(id, USize(1))` (bindings.rs:342) with `T=ConcreteResourceType`; under the handle model it must `cs.reserve::<F>(leaf_id, n)` per leaf `F` in `<T as Decompose>::Leaves`, with a distinct ID block for resource member columns.

## Part 2. Concrete impl path

1. **Implement `Decompose` for resource types.** `hilavitkutin-api/src/storage.rs` ALREADY ships `trait Decompose { type Leaves: StoreBundle }` (a cons-list of `Field<S>` scalars). Hand-write impls per resource type; a `#[derive(Decompose)]` handles ergonomics later.
2. **Replace the blob reservation in `DrainStores`** (bindings.rs:309-370). Walk `<T as Decompose>::Leaves` (existing `BundleProject`/`Locate` machinery or a dedicated trait); for each leaf `Field<F>` call `cs.reserve::<F>(leaf_id, USize(1))` at a distinct id; collect base pointers into a handle. Change `ResourceBinding<T,Tail>` from one `ResourcePtr<T>` to a `ResourceHandle<T>` mirroring `Leaves` (cons-list of `ResourcePtr<F>`). `next_id` advances by leaf count, not 1.
3. **Shape-bound sharing options.** Option A (type-unique per leaf, no sharing): each `Field<u32>` gets its own `StoreId`; good locality; column count grows with total leaf count (fits `Dim<256>` for typical workloads). Option B (shape-bound, shared): bounded by unique leaf types; scattered-access cost. **Recommendation: ship Option A first; bench-gate B.**
4. **Ctx accessors.** Singleton resources (common case): stack-local handle snapshot, direct deref per member, no morsel arithmetic. Expose members individually via `resource_field::<T, FieldSelector>()` (avoids a copy, enables partial access).
5. **Drift-fix sequencing.** Decompose impls (RunCfg, meta resources, tests) -> DrainStores per-leaf reserve (Option A) -> ResourceBinding->ResourceHandle -> drain return type -> ctx member access -> A3 morsel-size uses per-leaf column-stride sums for write resources. Steps 1-3 mechanical; step 4 holds the const-time leaf-list-walk complexity; step 5 is the API decision.

## Part 3. Open forks (recommendations)

- **A. member fetch:** stack-local cache (snapshot handle member ptrs before the loop), matching spec 1716-1723 + the measured approach. Cost: 2-8 pointer copies per morsel dispatch for a small singleton.
- **B. sharing:** type-unique columns (Option A) for the initial impl; ship shape-bound only after a bench shows the column-count problem is real.
- **C. handle keying:** const-derivable (column ids computed at compile time from the type-level store list + per-resource leaf counts), matching static composition; no runtime table.
- **D. morsel count:** singleton resources carry 1 record; their small `Field` members are negligible in the L1 budget. N-instance collections treat the member column as `N*morsel_size`.
- **E. Replaceable swap:** member-by-member copy of each leaf into its column slot (scalars: a write per leaf; Seq/Map: write all N elements consecutively). Same path as initial drain.

## Part 4. Alternatives

- **A. blob + separate-provenance stack-snapshot (minimal change).** The shipped blob is close. The issue is the blob's `ResourcePtr<T>` being obtained from the same struct as `ColumnPtr<T>`. Fix: dispatcher snapshots the `ResourcePtr<T>` to a stack slot before the loop (spec 1716-1723); LLVM proves the stack pointer doesn't alias column heap data and promotes to registers. **Delivers the measured 1.28-1.40x win with NO decomposition, ~5-10 dispatcher lines.** Costs: whole-blob access only, blob grows with largest resource, no SoA for Seq/Map. Viable + simpler for the current small-singleton workload (RunCfg, ClockState, meta). The right short path if the noalias win is wanted now and decomposition deferred; composes cleanly with later decomposition.
- **B. type-unique per-resource arena (the simpler decomposition).** Option A as a full alternative: each resource type gets its own member columns, type-unique, no sharing. Simpler than shape-bound, same decomposition benefits, no shared-column indexing. Fits `Dim<256>` for realistic apps. **This is the recommendation** (it IS the handle model with the simpler sharing policy).
- **C. inline blob in column-separate context struct.** Resource stays inline but in a `#[repr(C)]` subfield region distinct from column pointers, snapshotted to a separate stack struct. No reservation, no Decompose complexity. But no decomposition benefit, and Seq/Map with large N embedded inline blows up the frame. Does not generalize (R5 makes Seq/Map first-class); do not adopt.
- **loimu:** convergent on "resources hold handles to storage, not data." Difference: loimu adds dynamic view archetypes (engine-computed groupings from access-pattern analysis) which hilavitkutin forbids (static composition, no runtime registration). Convergence validates the direction; hilavitkutin solves co-location via morsel locality instead. Can't port directly.

## Summary judgement

1. The noalias win = pointer-provenance separation (already in `ResourcePtr`/`ColumnPtr`) + stack-local snapshot before the morsel loop. Proven mechanism; decomposition is additive, not the prerequisite. Revise the addendum to state this.
2. Per-member decomposition enables partial-member access, member-column windowing, SoA for multi-field resources. Real, but unbenched vs the baseline.
3. Shape-bound sharing is unbenched with scattered-access + re-aliasing costs. Defer + bench-gate separately.
4. `Decompose` already exists in `hilavitkutin-api/src/storage.rs`; the drift is `DrainStores` (bindings.rs:342) ignoring it and storing the blob. Fix = wire DrainStores through `<T as Decompose>::Leaves`.
5. Alternative A (blob + stack-snapshot) is a valid shorter path delivering the measured win immediately; composes with full decomposition.
6. All forks except shape-bound have clear recommendations. Only shape-bound needs a bench before choosing.
7. Revise the addendum to: (a) note the benched win is provenance + snapshot, achievable with the blob too; (b) label shape-bound as one option on the sharing axis, not committed; (c) identify Alternative A as the valid faster path.
