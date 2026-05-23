# 2026-05-11 concurrency + cache + Rust-soundness audit of Pass 2

**Round:** 202605101036 (hilavitkutin runtime megaround)
**Phase:** SRC (post-Pass-2, pre-Pass-3)
**Reviewer angle:** concurrent / cache-coherent scheduler design + Rust soundness for hot-path code. Prior art reference: Halide scheduling, TaskFlow task graph, Tokio worker runtime.
**Branch:** `feat/runtime-megaround-202605101036`
**Reviewed tip:** `3302f1c`
**Status:** all High-impact findings addressed; deferral roadmap executed in full.

Two prior audit rounds reviewed Sessions 2A + 2B from algorithmic / substrate-discipline / chain-architecture angles; this is the fourth review with a deliberately different specialisation (concurrency + cache + Rust soundness + the road into Pass 3). The reviewer also produced a deferral-resolution roadmap (Part B), which is the load-bearing artefact below.

## Part A: review findings

### High-impact (all addressed)

1. **`ExecutionPlan` Send/Sync auto-impl was silent.** Today every field chains to types that auto-derive Send + Sync correctly. The risk is forward stability: if any future field introduces a raw pointer or interior mutability, the auto-impl silently breaks without a compile-time diagnostic at the ExecutionPlan level. *Resolution:* added `const _: fn() = || { fn assert_send_sync<T: Send + Sync>() {} assert_send_sync::<ExecutionPlan<1,1,1,1,1,1,1,1,1,1>>(); }` in `plan/mod.rs`. Compile-time gate against regressions.

2. **`PoolFrame::progress_slots` carried no type-level lifetime.** The `NonNull<AtomicUsize>` pointed into a plan-stage scratch arena. The contract ("arena outlives every worker") lived only in a doc comment; nothing in the type system enforced it. *Resolution:* added `'arena` lifetime parameter + `PhantomData<&'arena [AtomicUsize]>` field. `Executor::run` signature picks up `'arena` alongside `'frame`. Dropping the arena while a worker holds a `&PoolFrame<'arena, ...>` is now a compile-time error. Breaking change to `PoolFrame` and `Executor::run`; no bodies to migrate (Pass 4 has not shipped HybridExecutor yet).

3. **`CoreProgram::progress_slot_idx` had no bounds check vs `PoolFrame::progress_slot_count`.** A hand-crafted plan with `progress_slot_idx > MAX_FIBERS` would produce out-of-bounds pointer arithmetic at dispatch time. *Resolution:* `synthesise_core_programs` carries `debug_assert!(progress_slot_base + range_count <= MAX_FIBERS)` at construction. The longer-term typed-newtype approach (carrying a proof of validity relative to the pool) stays in BACKLOG for a future round; the debug-assert is sufficient for current scale.

4. **ExecutionPlan-to-dispatch borrow lifetime was implicit.** Currently `assign_cores` and `compute_execution_plan` take/produce `ExecutionPlan` by value or by `&`; nothing in the type system expresses that closures must not outlive the plan. *Resolution (partial):* the immediate type-level gate is the static Send + Sync assertion (finding 1). The lifetime parameter on the eventual Pass-3 `DispatchClosure` belongs in Pass 3 itself; pre-baking it now would prejudge the shape. Tracked as a Pass 3 entry concern, not a Pre-Pass-3 blocker.

### Latent risks (not blockers; deliberately deferred)

5. **`DirtyMasks::per_fiber` false-sharing potential.** 8 fibers × `USize` = one 64-byte cache line. If Pass 3 dispatches fibers to separate cores and any core updates a dirty mask at dispatch time, false sharing happens. *Decision:* the plan is structurally immutable post-construction (dirty propagation runs in plan-stage). Pass 3 reads dirty masks; doesn't write. The skeleton's single-USize backing also caps at 64 stores. No padding needed yet; revisit when the dirty-mask substrate upgrades to multi-container (arvo-bitmask `Mask<W>` follow-up).

6. **Monomorphisation explosion across 10 const generics.** Multiple consumer call sites with distinct cap tuples will produce one full instantiation per tuple. *Decision:* mitigation is type aliases in a future test-utils module + consumer guidance ("use one Cfg shape per scheduler config"). Not actionable as a Pre-Pass-3 structural change.

7. **`DirtyMask::union_with` has no synchronisation.** Two cores calling it concurrently would race. *Decision:* the plan is immutable at dispatch time. `union_with` is only called during plan construction, single-threaded. If future cross-fiber dirty propagation lands, a synchronisation contract gets added then. Documented as a single-thread mutation contract.

### Soundness clarifications (commented in place)

8. `transmute_copy` pattern documented once at the root (`dispatch_codegen.rs` size assertions); plan/steps.rs sites follow that root contract.

9. `CONSUMED` sentinel safety reasoning: `usize::MAX` is unreachable as a real in-degree count (no finite graph approaches it). Sound by construction; comment-closed.

## Part B: deferral-resolution roadmap (executed in full)

The reviewer ranked the 9 acknowledged deferrals by Pass-3 impact, effort, and order. Pre-Pass-3 work list executed in strict order:

1. **D6 — `size_morsels` remainder distribution** (S effort). One-line arithmetic in `plan/steps.rs`. Sum invariant restored: 10 records across 3 fibers now produces `[4, 3, 3]` instead of `[3, 3, 3]` with a silently-dropped record. Commit `4bb4104`.

2. **D1 — persist morsel_sizes + thread upward_rank** (S effort). `ExecutionPlan` gained `morsel_sizes: [USize; MAX_FIBERS]`. Runner stores the step-9 result onto it. `_ranks` from step 8 now threads into `unit_meta[u].upward_rank` via the same unit-id-index projection that `commutative` should use (which discharged a secondary correctness bug in passing). Commit `4bb4104`.

3. **A2 — `PoolFrame<'arena, ...>` lifetime** (S effort). PoolFrame gained `'arena` lifetime + `PhantomData<&'arena [AtomicUsize]>`. `Executor::run` signature updated to thread `'arena`. Commit `b269156`.

4. **A1 — `ExecutionPlan` Send + Sync static assertion** (S effort). Inline `const _: fn() = || { ... }` in `plan/mod.rs`. Commit `b269156`.

5. **D8 — `synthesise_core_programs` + `plan/core_program.rs`** (M effort). The largest single item. New file shipping the conservative initial per-core projection (round-robin fiber assignment + Full ranges + sequential progress-slot base offsets + sync-role pattern over phases). Added Copy/Clone/Debug derives + const `new()` + Default to `CoreProgram`, `PhaseEntry`, `SyncRole`, `RecordRange` so the array-init pattern works. 3 smoke tests cover empty plan / single-unit / multi-fiber distribution. Commit `43fbca3`.

### Deferrals deliberately NOT addressed (per reviewer's roadmap)

- **D2** (`block_diagonalise` always-TRUE stub) — Pass 3 doesn't consume the error variant. Honest stub; tracked HILA-RUNTIME-C1.
- **D3** (10-const-generic ergonomics) — type-alias mitigation lands when Pass 3 needs it; not a structural blocker.
- **D4** (`plan/analysis/` visibility reorg) — does not affect Pass 3 codegen correctness.
- **D5** (`has_edge` linear scan) — not called on Pass-3 hot paths.
- **D7** (`classify_columns` cross-fiber upgrade) — conservative `Internal`-only classification produces correct (sub-optimal) dispatch. Tracked HILA-RUNTIME-C2.
- **D9** (`rcm_reorder`/`block_diagonalise`/`spectral_partition` stubs) — pass-through stubs with correct fallback. Substrate dependency: arvo-graph + arvo-spectral.

## Acceptance criteria for Pass 3 entry

All met as of `43fbca3`:

- `cargo test --package hilavitkutin` green: 80 tests pass (76 → 80; +4 new from this roadmap session).
- `cargo check --workspace` clean.
- `compute_execution_plan` produces an `ExecutionPlan` whose every field is populated (no `_morsels` discard, no `_ranks` discard).
- `synthesise_core_programs` shipped as a callable free function with smoke-test coverage.
- `PoolFrame` carries an `'arena` lifetime that the type checker enforces.
- `ExecutionPlan` has a compile-time Send + Sync gate.
- Cycle detection works: `topo_sort` returns placed-count; runner returns `Err(PlanError::Cycle)` when `placed < unit_count` (smoke-tested).

## Cross-references

- Reviewer dispatched 2026-05-11 (this date).
- Branch tip `43fbca3` on `feat/runtime-megaround-202605101036`.
- Earlier same-day work: review-driven fix-pass commits `1b15094` (main) + `3302f1c` (DirtyMask assertion discharge).
- Pass-2 commits: `b334ec6` (engine-id unification) + `55c1518` (test rehab) + `9a8e47e` (ExecutionPlan shape) + `e1142c5` (CSR + 13-step chain).
- Next: Pass 3 (dispatch codegen) per src CL `mock/design_rounds/202605101036_changelist.src.md`.
